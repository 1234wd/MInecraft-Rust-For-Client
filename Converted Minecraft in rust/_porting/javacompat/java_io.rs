//! Port of: (no `.java` counterpart -- JDK internals)
//! Java class(es): `java.io.DataInput`, `java.io.DataOutput`, `java.io.DataInputStream`, `java.io.DataOutputStream`, `java.io.EOFException`, `java.io.UTFDataFormatException`
//! Status: PORTED
//!
//! Big-endian primitive I/O and Java's *modified* UTF-8, as used by NBT (`NbtIo`, every
//! `Tag.write`, every `TagType.load`). Checked against JDK 25 by `nbt_probe.txt`
//! (`UTFW`/`UTFR`/`F32`/`F64`/`SKIP`/`PRIMIO`/`OUT` rows, `tests/session19_g.rs`).
//!
//! # Exceptions
//!
//! Java's checked `IOException`s become `std::io::Error`s whose payload is a
//! [`JavaIoException`] carrying the exact Java class name and message (`EOFException` has a
//! `null` message, i.e. `None`). [`java_class_name`] / [`java_message`] / [`java_to_string`]
//! read them back; an `io::Error` that did not come from this module (a real OS error from the
//! underlying stream) reports as `java.io.IOException` with Rust's text.
//!
//! # Modified UTF-8 (`writeUTF` / `readUTF`)
//!
//! * `U+0000` is written as `C0 80`, never as a raw `00`.
//! * Every UTF-16 code unit is encoded separately, so a supplementary character becomes two
//!   3-byte surrogate encodings (`U+1F600` -> `ED A0 BD ED B8 80`), never a 4-byte sequence.
//! * The length prefix is an unsigned 16-bit BYTE count. Above 65535 bytes `writeUTF` throws
//!   `UTFDataFormatException("encoded string (<first 8 units>...<last 8 units>) too long: <n> bytes")`
//!   before writing anything.
//! * `readUTF` accepts what the JDK accepts: overlong 2- and 3-byte forms (`C0 80`, `C1 BF`,
//!   `E0 80 80`) and a raw `00` byte; it rejects `80..BF`/`F0..FF` lead bytes with
//!   `"malformed input around byte N"` and a truncated final character with
//!   `"malformed input: partial character at end"`, with the JDK's exact byte offsets.
//!
//! # The lone-surrogate limitation
//!
//! Java strings are UTF-16 and may hold unpaired surrogates (`"\uD800"`); Rust `String` cannot.
//! The exact decoder/encoder therefore work on `u16` code units ([`read_utf_units`],
//! [`write_utf_units`]), and only the `String` conveniences lose information:
//!
//! * [`DataInput::read_utf`] decodes exactly, then converts with `String::from_utf16_lossy`, so
//!   each unpaired surrogate becomes `U+FFFD`. Java would keep it, and writing the string back
//!   would give `ED A0 80` where the port gives `EF BF BD`. Chosen over an error because a world
//!   file holding such a string (vanilla can create one, e.g. by `substring` through a pair) must
//!   still load; it is the only lossy step and it is pinned by a test.
//! * The `too long` message quotes 8 UTF-16 units from each end; if that cut splits a pair, the
//!   port's message holds `U+FFFD` where Java's holds the lone half.
//!
//! # NaN on the wire
//!
//! `writeFloat`/`writeDouble` go through `floatToIntBits`/`doubleToLongBits`, which CANONICALISE
//! NaN (`7fc00000` / `7ff8000000000000`, sign dropped); `readFloat`/`readDouble` keep the raw
//! bits. So a NaN payload survives a read but not a write -- the probe pins both directions.

use std::fmt;
use std::io::{self, ErrorKind, Read, Write};

/// `java.io.EOFException`.
pub const EOF_EXCEPTION: &str = "java.io.EOFException";
/// `java.io.IOException`.
pub const IO_EXCEPTION: &str = "java.io.IOException";
/// `java.io.UTFDataFormatException`.
pub const UTF_DATA_FORMAT_EXCEPTION: &str = "java.io.UTFDataFormatException";

/// A Java checked exception, carried as the payload of a `std::io::Error`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaIoException {
    class_name: &'static str,
    message: Option<String>,
}

impl JavaIoException {
    /// An exception of class `class_name` (fully qualified) with `getMessage()` = `message`.
    pub fn new(class_name: &'static str, message: Option<String>) -> Self {
        Self { class_name, message }
    }

    /// `getClass().getName()`.
    pub fn class_name(&self) -> &'static str {
        self.class_name
    }

    /// `getMessage()` (`None` for Java `null`).
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl fmt::Display for JavaIoException {
    /// `Throwable.toString()`: the class name, then `": " + message` when the message is non-null.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(m) => write!(f, "{}: {}", self.class_name, m),
            None => f.write_str(self.class_name),
        }
    }
}

impl std::error::Error for JavaIoException {}

/// `new EOFException()` (null message).
pub fn eof_exception() -> io::Error {
    io::Error::new(ErrorKind::UnexpectedEof, JavaIoException::new(EOF_EXCEPTION, None))
}

/// `new IOException(message)`.
pub fn io_exception(message: impl Into<String>) -> io::Error {
    io::Error::new(ErrorKind::Other, JavaIoException::new(IO_EXCEPTION, Some(message.into())))
}

/// `new UTFDataFormatException(message)`.
pub fn utf_data_format_exception(message: impl Into<String>) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, JavaIoException::new(UTF_DATA_FORMAT_EXCEPTION, Some(message.into())))
}

/// The Java exception inside `e`, if `e` was raised by this module.
pub fn java_exception(e: &io::Error) -> Option<&JavaIoException> {
    e.get_ref().and_then(|inner| inner.downcast_ref::<JavaIoException>())
}

/// The Java class name of `e`: exact for errors raised here, `java.io.IOException` (or
/// `java.io.EOFException` for `UnexpectedEof`) for foreign `io::Error`s.
pub fn java_class_name(e: &io::Error) -> &'static str {
    match java_exception(e) {
        Some(j) => j.class_name(),
        None if e.kind() == ErrorKind::UnexpectedEof => EOF_EXCEPTION,
        None => IO_EXCEPTION,
    }
}

/// `getMessage()` of `e` (`None` for Java `null`).
pub fn java_message(e: &io::Error) -> Option<String> {
    match java_exception(e) {
        Some(j) => j.message.clone(),
        None => Some(e.to_string()),
    }
}

/// `Throwable.toString()` of `e`.
pub fn java_to_string(e: &io::Error) -> String {
    match java_message(e) {
        Some(m) => format!("{}: {}", java_class_name(e), m),
        None => java_class_name(e).to_string(),
    }
}

/// `Float.floatToIntBits` (NaN canonicalised to `0x7fc00000`).
#[inline]
pub fn float_to_int_bits(v: f32) -> i32 {
    if v.is_nan() {
        0x7fc0_0000
    } else {
        v.to_bits() as i32
    }
}

/// `Double.doubleToLongBits` (NaN canonicalised to `0x7ff8000000000000`).
#[inline]
pub fn double_to_long_bits(v: f64) -> i64 {
    if v.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        v.to_bits() as i64
    }
}

// ============================================================================
// DataInput
// ============================================================================

/// Port of `java.io.DataInput`, with `DataInputStream`'s semantics for the default methods.
///
/// Implementors provide [`read_fully`](DataInput::read_fully) and
/// [`skip_bytes`](DataInput::skip_bytes); everything else is derived exactly as
/// `DataInputStream` does it.
pub trait DataInput {
    /// `readFully(byte[])`: fill `b` completely or fail with `EOFException`.
    fn read_fully(&mut self, b: &mut [u8]) -> io::Result<()>;

    /// `skipBytes(int)`: skip up to `n` bytes, stopping silently at end of stream; returns the
    /// number skipped. `n <= 0` skips nothing (Java callers pass unchecked, possibly negative,
    /// counts -- e.g. `ListTag`'s skip of a negative length).
    fn skip_bytes(&mut self, n: i32) -> io::Result<i32>;

    /// `readBoolean()`.
    fn read_boolean(&mut self) -> io::Result<bool> {
        Ok(self.read_unsigned_byte()? != 0)
    }

    /// `readByte()`.
    fn read_byte(&mut self) -> io::Result<i8> {
        let mut b = [0u8; 1];
        self.read_fully(&mut b)?;
        Ok(b[0] as i8)
    }

    /// `readUnsignedByte()`.
    fn read_unsigned_byte(&mut self) -> io::Result<i32> {
        let mut b = [0u8; 1];
        self.read_fully(&mut b)?;
        Ok(b[0] as i32)
    }

    /// `readShort()`.
    fn read_short(&mut self) -> io::Result<i16> {
        let mut b = [0u8; 2];
        self.read_fully(&mut b)?;
        Ok(i16::from_be_bytes(b))
    }

    /// `readUnsignedShort()`.
    fn read_unsigned_short(&mut self) -> io::Result<i32> {
        let mut b = [0u8; 2];
        self.read_fully(&mut b)?;
        Ok(u16::from_be_bytes(b) as i32)
    }

    /// `readChar()` (one UTF-16 code unit).
    fn read_char(&mut self) -> io::Result<u16> {
        let mut b = [0u8; 2];
        self.read_fully(&mut b)?;
        Ok(u16::from_be_bytes(b))
    }

    /// `readInt()`.
    fn read_int(&mut self) -> io::Result<i32> {
        let mut b = [0u8; 4];
        self.read_fully(&mut b)?;
        Ok(i32::from_be_bytes(b))
    }

    /// `readLong()`.
    fn read_long(&mut self) -> io::Result<i64> {
        let mut b = [0u8; 8];
        self.read_fully(&mut b)?;
        Ok(i64::from_be_bytes(b))
    }

    /// `readFloat()`: `Float.intBitsToFloat(readInt())`, raw bits kept (NaN payloads survive).
    fn read_float(&mut self) -> io::Result<f32> {
        Ok(f32::from_bits(self.read_int()? as u32))
    }

    /// `readDouble()`: `Double.longBitsToDouble(readLong())`, raw bits kept.
    fn read_double(&mut self) -> io::Result<f64> {
        Ok(f64::from_bits(self.read_long()? as u64))
    }

    /// `readUTF()`, converted to a Rust `String`. Unpaired surrogates become `U+FFFD` (see the
    /// module docs); use [`read_utf_units`] for the exact UTF-16.
    fn read_utf(&mut self) -> io::Result<String> {
        Ok(String::from_utf16_lossy(&read_utf_units(self)?))
    }
}

/// `DataInputStream.readUTF(DataInput)`, exactly, as UTF-16 code units.
pub fn read_utf_units<I: DataInput + ?Sized>(input: &mut I) -> io::Result<Vec<u16>> {
    let utflen = input.read_unsigned_short()? as usize;
    let mut bytes = vec![0u8; utflen];
    input.read_fully(&mut bytes)?;
    decode_modified_utf8(&bytes)
}

/// The decoding loop of `DataInputStream.readUTF`, over the `utflen` payload bytes.
pub fn decode_modified_utf8(bytes: &[u8]) -> io::Result<Vec<u16>> {
    let utflen = bytes.len();
    let mut chars: Vec<u16> = Vec::with_capacity(utflen);
    let mut count = 0usize;
    while count < utflen {
        let c = bytes[count] as i32;
        match c >> 4 {
            0..=7 => {
                // 0xxxxxxx
                count += 1;
                chars.push(c as u16);
            }
            12 | 13 => {
                // 110x xxxx   10xx xxxx
                count += 2;
                if count > utflen {
                    return Err(utf_data_format_exception("malformed input: partial character at end"));
                }
                let char2 = bytes[count - 1] as i32;
                if (char2 & 0xC0) != 0x80 {
                    return Err(utf_data_format_exception(format!("malformed input around byte {count}")));
                }
                chars.push((((c & 0x1F) << 6) | (char2 & 0x3F)) as u16);
            }
            14 => {
                // 1110 xxxx  10xx xxxx  10xx xxxx
                count += 3;
                if count > utflen {
                    return Err(utf_data_format_exception("malformed input: partial character at end"));
                }
                let char2 = bytes[count - 2] as i32;
                let char3 = bytes[count - 1] as i32;
                if (char2 & 0xC0) != 0x80 || (char3 & 0xC0) != 0x80 {
                    return Err(utf_data_format_exception(format!("malformed input around byte {}", count - 1)));
                }
                chars.push((((c & 0x0F) << 12) | ((char2 & 0x3F) << 6) | (char3 & 0x3F)) as u16);
            }
            _ => {
                // 10xx xxxx,  1111 xxxx
                return Err(utf_data_format_exception(format!("malformed input around byte {count}")));
            }
        }
    }
    Ok(chars)
}

/// Port of `java.io.DataInputStream` over any `std::io::Read`.
pub struct DataInputStream<R: Read> {
    inner: R,
}

impl<R: Read> DataInputStream<R> {
    /// `new DataInputStream(in)`.
    pub fn new(inner: R) -> Self {
        Self { inner }
    }

    /// The wrapped reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Unwrap.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> DataInput for DataInputStream<R> {
    fn read_fully(&mut self, b: &mut [u8]) -> io::Result<()> {
        let mut n = 0;
        while n < b.len() {
            match self.inner.read(&mut b[n..]) {
                Ok(0) => return Err(eof_exception()),
                Ok(k) => n += k,
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn skip_bytes(&mut self, n: i32) -> io::Result<i32> {
        // `while (total < n && (cur = in.skip(n - total)) > 0) total += cur;` -- InputStream.skip
        // discards what it can and reports 0 at end of stream.
        let mut total: i32 = 0;
        let mut scratch = [0u8; 4096];
        while total < n {
            let want = ((n - total) as usize).min(scratch.len());
            match self.inner.read(&mut scratch[..want]) {
                Ok(0) => break,
                Ok(k) => total += k as i32,
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(total)
    }
}

// ============================================================================
// DataOutput
// ============================================================================

/// Port of `java.io.DataOutput`, with `DataOutputStream`'s semantics for the default methods.
///
/// Implementors provide [`write`](DataOutput::write); every other method is derived exactly as
/// `DataOutputStream` encodes it. Wrappers (e.g. `NbtIo.StringFallbackDataOutput`) override
/// [`write_utf`](DataOutput::write_utf).
pub trait DataOutput {
    /// `write(byte[])`.
    fn write(&mut self, b: &[u8]) -> io::Result<()>;

    /// `write(int)`: the low 8 bits.
    fn write_one(&mut self, b: i32) -> io::Result<()> {
        self.write(&[b as u8])
    }

    /// `writeBoolean(boolean)`.
    fn write_boolean(&mut self, v: bool) -> io::Result<()> {
        self.write(&[v as u8])
    }

    /// `writeByte(int)`: the low 8 bits.
    fn write_byte(&mut self, v: i32) -> io::Result<()> {
        self.write(&[v as u8])
    }

    /// `writeShort(int)`: the low 16 bits, big-endian.
    fn write_short(&mut self, v: i32) -> io::Result<()> {
        self.write(&(v as u16).to_be_bytes())
    }

    /// `writeChar(int)`: the low 16 bits, big-endian.
    fn write_char(&mut self, v: i32) -> io::Result<()> {
        self.write(&(v as u16).to_be_bytes())
    }

    /// `writeInt(int)`.
    fn write_int(&mut self, v: i32) -> io::Result<()> {
        self.write(&v.to_be_bytes())
    }

    /// `writeLong(long)`.
    fn write_long(&mut self, v: i64) -> io::Result<()> {
        self.write(&v.to_be_bytes())
    }

    /// `writeFloat(float)`: `writeInt(Float.floatToIntBits(v))` -- NaN is canonicalised.
    fn write_float(&mut self, v: f32) -> io::Result<()> {
        self.write_int(float_to_int_bits(v))
    }

    /// `writeDouble(double)`: `writeLong(Double.doubleToLongBits(v))` -- NaN is canonicalised.
    fn write_double(&mut self, v: f64) -> io::Result<()> {
        self.write_long(double_to_long_bits(v))
    }

    /// `writeBytes(String)`: the low byte of every UTF-16 unit.
    fn write_bytes(&mut self, s: &str) -> io::Result<()> {
        let b: Vec<u8> = s.encode_utf16().map(|u| u as u8).collect();
        self.write(&b)
    }

    /// `writeChars(String)`: every UTF-16 unit as two bytes.
    fn write_chars(&mut self, s: &str) -> io::Result<()> {
        let mut b = Vec::with_capacity(s.len() * 2);
        for u in s.encode_utf16() {
            b.extend_from_slice(&u.to_be_bytes());
        }
        self.write(&b)
    }

    /// `writeUTF(String)`. See [`write_utf_units`].
    fn write_utf(&mut self, s: &str) -> io::Result<()> {
        let units: Vec<u16> = s.encode_utf16().collect();
        write_utf_units(self, &units)
    }
}

/// `DataOutputStream.writeUTF(String, DataOutput)` over UTF-16 units: the 2-byte length plus
/// the modified UTF-8, written with one `write` call; or `UTFDataFormatException` (nothing
/// written) when the encoding exceeds 65535 bytes.
pub fn write_utf_units<O: DataOutput + ?Sized>(out: &mut O, units: &[u16]) -> io::Result<()> {
    let bytes = encode_modified_utf8(units)?;
    out.write(&bytes)
}

/// The encoding half of `writeUTF`: length prefix + modified UTF-8, or the JDK's
/// `UTFDataFormatException` for an over-long string.
pub fn encode_modified_utf8(units: &[u16]) -> io::Result<Vec<u8>> {
    let utflen: u64 = units
        .iter()
        .map(|&c| if c >= 0x800 { 3u64 } else if c >= 0x80 || c == 0 { 2 } else { 1 })
        .sum();
    if utflen > 65535 {
        return Err(utf_data_format_exception(too_long_msg(units, utflen)));
    }
    let mut out = Vec::with_capacity(utflen as usize + 2);
    out.extend_from_slice(&(utflen as u16).to_be_bytes());
    for &c in units {
        let c = c as u32;
        if c < 0x80 && c != 0 {
            out.push(c as u8);
        } else if c >= 0x800 {
            out.push((0xE0 | ((c >> 12) & 0x0F)) as u8);
            out.push((0x80 | ((c >> 6) & 0x3F)) as u8);
            out.push((0x80 | (c & 0x3F)) as u8);
        } else {
            out.push((0xC0 | ((c >> 6) & 0x1F)) as u8);
            out.push((0x80 | (c & 0x3F)) as u8);
        }
    }
    Ok(out)
}

/// `DataOutputStream.tooLongMsg`: `"encoded string (" + s.substring(0, 8) + "..." +
/// s.substring(len - 8) + ") too long: " + actualLength + " bytes"`. Only reachable for strings
/// longer than 21845 units, so both substrings exist.
fn too_long_msg(units: &[u16], utflen: u64) -> String {
    let n = units.len();
    let head = String::from_utf16_lossy(&units[..8]);
    let tail = String::from_utf16_lossy(&units[n - 8..]);
    format!("encoded string ({head}...{tail}) too long: {utflen} bytes")
}

/// Port of `java.io.DataOutputStream` over any `std::io::Write`.
pub struct DataOutputStream<W: Write> {
    inner: W,
    written: i32,
}

impl<W: Write> DataOutputStream<W> {
    /// `new DataOutputStream(out)`.
    pub fn new(inner: W) -> Self {
        Self { inner, written: 0 }
    }

    /// `size()`: bytes written so far, saturating at `Integer.MAX_VALUE` like `incCount`.
    pub fn size(&self) -> i32 {
        self.written
    }

    /// `flush()`.
    pub fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    /// The wrapped writer.
    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    /// Unwrap.
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> DataOutput for DataOutputStream<W> {
    fn write(&mut self, b: &[u8]) -> io::Result<()> {
        self.inner.write_all(b)?;
        let next = (self.written as i64) + b.len() as i64;
        self.written = if next > i32::MAX as i64 { i32::MAX } else { next as i32 };
        Ok(())
    }
}
