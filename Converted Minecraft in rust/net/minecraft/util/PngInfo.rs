//! Port of: net/minecraft/util/PngInfo.java
//! Java class(es): net.minecraft.util.PngInfo
//! Status: PARTIAL
//!
//! Session 22. `public record PngInfo(int width, int height)`: a PNG's size read from its first 24
//! bytes, plus a header check on a `ByteBuffer`.
//!
//! # `fromStream` / `fromBytes`
//!
//! Big-endian reads through a `DataInputStream`: the 8-byte signature, the IHDR length (must be 13)
//! and type (must be `IHDR`), then width and height -- read as SIGNED `int`s, so `0xffffffff` is a
//! width of `-1` (probed). Each check throws `IOException` with the offending value:
//! `"Bad PNG Signature: " + FORMAT.toHexDigits(magic)` and `"Bad type for IHDR chunk: " + ...`.
//!
//! `FORMAT` is `HexFormat.of().withUpperCase().withPrefix("0x")`, but `toHexDigits` ignores the
//! prefix: the message is sixteen (or eight) UPPER-CASE digits with NO `0x` (probed:
//! `Bad PNG Signature: FFD8FFE000104A46`). A short input ends in `EOFException` with a null message,
//! raised by the read that ran out. Errors are `io::Error`s built by `javacompat::java_io`, which
//! carry Java's class name and message.
//!
//! # `validateHeader` -- why this file is PARTIAL
//!
//! `validateHeader(ByteBuffer)` checks the same header at absolute offsets 0, 8 and 12 with its own,
//! shorter messages (`"PNG header missing"` under 16 bytes, `"Bad PNG Signature"`,
//! `"Bad length for IHDR chunk!"`, `"Bad type for IHDR chunk!"`). It switches the buffer to
//! big-endian first and restores the caller's byte order ONLY ON SUCCESS -- on every error path the
//! buffer is left big-endian (probed). The port takes the buffer's bytes as a slice, which has no
//! byte-order state, so that side effect on a caller's `ByteBuffer` is NOT reproduced. Every return
//! value and every exception is.
//!
//! # Record members
//!
//! `equals` is derived, `hashCode` is `31 * width + height`, `toString` is `PngInfo[width=.., height=..]`.
//!
//! Verified by the `PNGINFO` and `PNGHDR` groups of `small_probe.txt` (`tests/session22.rs`).

use std::io::{self, Read};

use crate::javacompat::java_io::{self, DataInput, DataInputStream};
use crate::javacompat::java_object;

/// Port of `private static final long PNG_HEADER = -8552249625308161526L` (`89 50 4E 47 0D 0A 1A 0A`).
const PNG_HEADER: i64 = -8_552_249_625_308_161_526;
/// Port of `private static final int IHDR_TYPE = 1229472850` (`IHDR`).
const IHDR_TYPE: i32 = 1_229_472_850;
/// Port of `private static final int IHDR_SIZE = 13`.
const IHDR_SIZE: i32 = 13;

/// Port of `net.minecraft.util.PngInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PngInfo {
    width: i32,
    height: i32,
}

impl PngInfo {
    /// Port of the canonical constructor.
    pub const fn new(width: i32, height: i32) -> Self {
        PngInfo { width, height }
    }

    /// Port of the accessor `width()`.
    pub const fn width(&self) -> i32 {
        self.width
    }

    /// Port of the accessor `height()`.
    pub const fn height(&self) -> i32 {
        self.height
    }

    /// Port of `fromStream(InputStream inputStream) throws IOException`.
    pub fn from_stream<R: Read>(input_stream: R) -> io::Result<PngInfo> {
        let mut stream = DataInputStream::new(input_stream);
        let magic = stream.read_long()?;
        if magic != PNG_HEADER {
            return Err(java_io::io_exception(format!("Bad PNG Signature: {:016X}", magic)));
        }
        let header_size = stream.read_int()?;
        if header_size != IHDR_SIZE {
            return Err(java_io::io_exception(format!("Bad length for IHDR chunk: {header_size}")));
        }
        let header_type = stream.read_int()?;
        if header_type != IHDR_TYPE {
            return Err(java_io::io_exception(format!("Bad type for IHDR chunk: {:08X}", header_type)));
        }
        let width = stream.read_int()?;
        let height = stream.read_int()?;
        Ok(PngInfo::new(width, height))
    }

    /// Port of `fromBytes(byte[] bytes) throws IOException`.
    pub fn from_bytes(bytes: &[u8]) -> io::Result<PngInfo> {
        PngInfo::from_stream(bytes)
    }

    /// Port of `validateHeader(ByteBuffer buffer) throws IOException`, over the buffer's bytes up to
    /// its limit. See the module docs for the byte-order side effect this does not reproduce.
    pub fn validate_header(buffer: &[u8]) -> io::Result<()> {
        if buffer.len() < 16 {
            return Err(java_io::io_exception("PNG header missing"));
        }
        let long_at = |i: usize| i64::from_be_bytes(buffer[i..i + 8].try_into().expect("8 bytes"));
        let int_at = |i: usize| i32::from_be_bytes(buffer[i..i + 4].try_into().expect("4 bytes"));
        if long_at(0) != PNG_HEADER {
            return Err(java_io::io_exception("Bad PNG Signature"));
        }
        if int_at(8) != IHDR_SIZE {
            return Err(java_io::io_exception("Bad length for IHDR chunk!"));
        }
        if int_at(12) != IHDR_TYPE {
            return Err(java_io::io_exception("Bad type for IHDR chunk!"));
        }
        Ok(())
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[self.width, self.height])
    }
}

impl std::fmt::Display for PngInfo {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PngInfo[width={}, height={}]", self.width, self.height)
    }
}
