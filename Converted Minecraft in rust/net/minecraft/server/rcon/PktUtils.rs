//! Port of: net/minecraft/server/rcon/PktUtils.java
//! Java class(es): net.minecraft.server.rcon.PktUtils
//! Status: VERIFIED
//!
//! Byte helpers for the RCON and query servers. Every member is ported; `MAX_PACKET_SIZE` is
//! checked against the constant dump, and every method -- including its exceptions -- against
//! a run of the 26.2 jar, by `_porting/tests/session19_e.rs`.
//!
//! Java `byte[]` parameters are `&[u8]` here (same bits; the code masks with `& 0xFF` wherever
//! the sign would matter, and the one unmasked byte is shifted left by 24, which discards the
//! sign extension). A single Java `byte` parameter stays `i8`.
//!
//! # Quirks pinned by the tests
//!
//! - `stringFromByteArray(b, offset, length)` treats `length` as an **end index**, scans for a
//!   NUL starting at `offset`, but never includes `b[length - 1]`: `"ab"` with length 2 gives
//!   `"a"`. It reads `b[i]` *before* testing `i < max`, so `length == 0` reads `b[-1]` and
//!   throws `ArrayIndexOutOfBoundsException: Index -1 ...`, and `offset > length - 1` reaches
//!   `new String` with a negative count, which throws `StringIndexOutOfBoundsException`.
//! - The `int` readers return **0** (not an exception) when fewer than 4 bytes fit before
//!   `length`, but still throw when `length` lies about the array (or `offset` is negative),
//!   reporting the first index Java evaluates: `offset + 3` first for little-endian,
//!   `offset` first for network order.
//! - `new String(bytes, UTF_8)` is the JDK's own replacing decoder, which is **not** what
//!   `String::from_utf8_lossy` does: the JDK swallows a whole encoded surrogate (`ED A0 80`)
//!   into one U+FFFD where Rust emits three, and stops after a truncated multi-byte tail at the
//!   end of input. `jdk_decode_utf8` below is a line-by-line port of JDK 25's
//!   `String#decodeUTF8_UTF16` with `doReplace = true`. (It is Java stdlib behaviour and would
//!   belong in `javacompat`; it lives here because this batch may not edit that module.)

/// Java: `PktUtils.MAX_PACKET_SIZE` = `1460`.
pub const MAX_PACKET_SIZE: i32 = 1460;

/// Java: `PktUtils.HEX_CHAR`, a `public static final char[]` (mutable in Java; nothing writes it).
pub const HEX_CHAR: [char; 16] =
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

/// Port of `PktUtils#stringFromByteArray(byte[], int, int)`.
///
/// Decodes `b[offset..i)` as UTF-8, where `i` is the first NUL at or after `offset`, capped at
/// `length - 1` (exclusive of that byte). See the module docs for the exceptions.
pub fn string_from_byte_array(b: &[u8], offset: i32, length: i32) -> String {
    let max = length.wrapping_sub(1);
    let mut i = if offset > max { max } else { offset };
    // Java: `while (0 != b[i] && i < max) i++;` -- the array read comes first.
    while 0 != array_get(b, i) && i < max {
        i += 1;
    }
    new_string_utf8(b, offset, i.wrapping_sub(offset))
}

/// Port of `PktUtils#intFromByteArray(byte[], int)`: little-endian `int` at `offset`, with
/// `length = b.length`.
pub fn int_from_byte_array_2(b: &[u8], offset: i32) -> i32 {
    int_from_byte_array_3(b, offset, b.len() as i32)
}

/// Port of `PktUtils#intFromByteArray(byte[], int, int)`: little-endian `int` at `offset`, or
/// `0` if `length - offset - 4 < 0` (Java `int` arithmetic, wrapping).
pub fn int_from_byte_array_3(b: &[u8], offset: i32, length: i32) -> i32 {
    if 0 > length.wrapping_sub(offset).wrapping_sub(4) {
        return 0;
    }
    // Java evaluates the `|` chain left to right: offset + 3 is read (and bounds-checked) first.
    let b3 = array_get(b, offset.wrapping_add(3)) as i8 as i32;
    let b2 = array_get(b, offset.wrapping_add(2)) as i32;
    let b1 = array_get(b, offset.wrapping_add(1)) as i32;
    let b0 = array_get(b, offset) as i32;
    b3 << 24 | (b2 & 0xFF) << 16 | (b1 & 0xFF) << 8 | b0 & 0xFF
}

/// Port of `PktUtils#intFromNetworkByteArray(byte[], int, int)`: big-endian `int` at `offset`,
/// or `0` if `length - offset - 4 < 0`.
pub fn int_from_network_byte_array(b: &[u8], offset: i32, length: i32) -> i32 {
    if 0 > length.wrapping_sub(offset).wrapping_sub(4) {
        return 0;
    }
    // Left to right: offset is read (and bounds-checked) first.
    let b0 = array_get(b, offset) as i8 as i32;
    let b1 = array_get(b, offset.wrapping_add(1)) as i32;
    let b2 = array_get(b, offset.wrapping_add(2)) as i32;
    let b3 = array_get(b, offset.wrapping_add(3)) as i32;
    b0 << 24 | (b1 & 0xFF) << 16 | (b2 & 0xFF) << 8 | b3 & 0xFF
}

/// Port of `PktUtils#toHexString(byte)`: two lowercase hex digits, high nibble first.
pub fn to_hex_string(b: i8) -> String {
    let b = b as i32;
    let mut s = String::with_capacity(2);
    // Java: `HEX_CHAR[(b & 240) >>> 4] + HEX_CHAR[b & 15]`.
    s.push(HEX_CHAR[((b & 240) as u32 >> 4) as usize]);
    s.push(HEX_CHAR[(b & 15) as usize]);
    s
}

// ---------------------------------------------------------------------------------------------
// Java array / String semantics used above
// ---------------------------------------------------------------------------------------------

/// Java `b[index]` with the JVM's bounds check (checked before any `usize` cast).
fn array_get(b: &[u8], index: i32) -> u8 {
    if index < 0 || index as i64 >= b.len() as i64 {
        panic!(
            "java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {}",
            b.len()
        );
    }
    b[index as usize]
}

/// Java `new String(bytes, offset, length, StandardCharsets.UTF_8)`: JDK 25's
/// `String#checkBoundsOffCount` (via `Preconditions#checkFromIndexSize`), then the decoder.
fn new_string_utf8(bytes: &[u8], offset: i32, length: i32) -> String {
    let n = bytes.len() as i32;
    if (n | offset | length) < 0 || length > n - offset {
        panic!(
            "java.lang.StringIndexOutOfBoundsException: Range [{offset}, {offset} + {length}) out of \
             bounds for length {n}"
        );
    }
    jdk_decode_utf8(&bytes[offset as usize..(offset + length) as usize])
}

/// U+FFFD, JDK `String.REPL`.
const REPL: u16 = 0xFFFD;

/// The `int` a Java `byte` widens to.
#[inline]
fn sb(src: &[u8], i: usize) -> i32 {
    src[i] as i8 as i32
}

#[inline]
fn is_not_continuation(b: i32) -> bool {
    (b & 0xc0) != 0x80
}

#[inline]
fn is_malformed3(b1: i32, b2: i32, b3: i32) -> bool {
    (b1 == 0xe0_u8 as i8 as i32 && (b2 & 0xe0) == 0x80) || (b2 & 0xc0) != 0x80 || (b3 & 0xc0) != 0x80
}

#[inline]
fn is_malformed3_2(b1: i32, b2: i32) -> bool {
    (b1 == 0xe0_u8 as i8 as i32 && (b2 & 0xe0) == 0x80) || (b2 & 0xc0) != 0x80
}

#[inline]
fn is_malformed4(b2: i32, b3: i32, b4: i32) -> bool {
    (b2 & 0xc0) != 0x80 || (b3 & 0xc0) != 0x80 || (b4 & 0xc0) != 0x80
}

/// Both arguments already masked to `0..=255` by the caller, as in the JDK.
#[inline]
fn is_malformed4_2(b1: i32, b2: i32) -> bool {
    (b1 == 0xf0 && (b2 < 0x90 || b2 > 0xbf)) || (b1 == 0xf4 && (b2 & 0xf0) != 0x80) || (b2 & 0xc0) != 0x80
}

#[inline]
fn is_malformed4_3(b3: i32) -> bool {
    (b3 & 0xc0) != 0x80
}

#[inline]
fn decode2(b1: i32, b2: i32) -> u16 {
    (((b1 << 6) ^ b2) ^ (((0xC0_u8 as i8 as i32) << 6) ^ (0x80_u8 as i8 as i32))) as u16
}

#[inline]
fn decode3(b1: i32, b2: i32, b3: i32) -> u16 {
    let k = ((0xE0_u8 as i8 as i32) << 12) ^ ((0x80_u8 as i8 as i32) << 6) ^ (0x80_u8 as i8 as i32);
    ((b1 << 12) ^ (b2 << 6) ^ (b3 ^ k)) as u16
}

#[inline]
fn decode4(b1: i32, b2: i32, b3: i32, b4: i32) -> i32 {
    let k = ((0xF0_u8 as i8 as i32) << 18)
        ^ ((0x80_u8 as i8 as i32) << 12)
        ^ ((0x80_u8 as i8 as i32) << 6)
        ^ (0x80_u8 as i8 as i32);
    (b1 << 18) ^ (b2 << 12) ^ (b3 << 6) ^ (b4 ^ k)
}

/// JDK `String#malformed3`: how many bytes a malformed 3-byte lead swallows (1 or 2).
fn malformed3(src: &[u8], sp: usize) -> usize {
    let b1 = sb(src, sp);
    let b2 = sb(src, sp + 1);
    if (b1 == 0xe0_u8 as i8 as i32 && (b2 & 0xe0) == 0x80) || is_not_continuation(b2) {
        1
    } else {
        2
    }
}

/// JDK `String#malformed4`: how many bytes a malformed 4-byte lead swallows (1, 2 or 3).
fn malformed4(src: &[u8], sp: usize) -> usize {
    let b1 = sb(src, sp) & 0xff;
    let b2 = sb(src, sp + 1) & 0xff;
    if b1 > 0xf4
        || (b1 == 0xf0 && (b2 < 0x90 || b2 > 0xbf))
        || (b1 == 0xf4 && (b2 & 0xf0) != 0x80)
        || is_not_continuation(b2)
    {
        return 1;
    }
    if is_not_continuation(sb(src, sp + 2)) {
        return 2;
    }
    3
}

/// Port of JDK 25 `String#decodeUTF8_UTF16(src, 0, src.length, dst, 0, doReplace = true)`.
///
/// The JDK's Latin-1 fast path in the `String(byte[], int, int, Charset)` constructor only
/// handles input this loop decodes identically (ASCII and `C2`/`C3` + continuation), so this
/// one loop is the whole decoder.
fn jdk_decode_utf8(src: &[u8]) -> String {
    let sl = src.len();
    let mut sp = 0usize;
    let mut dst: Vec<u16> = Vec::with_capacity(sl);
    while sp < sl {
        let mut b1 = sb(src, sp);
        sp += 1;
        if b1 >= 0 {
            dst.push(b1 as u16);
        } else if (b1 >> 5) == -2 && (b1 & 0x1e) != 0 {
            if sp < sl {
                let b2 = sb(src, sp);
                sp += 1;
                if is_not_continuation(b2) {
                    dst.push(REPL);
                    sp -= 1;
                } else {
                    dst.push(decode2(b1, b2));
                }
                continue;
            }
            dst.push(REPL);
            break;
        } else if (b1 >> 4) == -2 {
            if sp + 1 < sl {
                let b2 = sb(src, sp);
                let b3 = sb(src, sp + 1);
                sp += 2;
                if is_malformed3(b1, b2, b3) {
                    dst.push(REPL);
                    sp -= 3;
                    sp += malformed3(src, sp);
                } else {
                    let c = decode3(b1, b2, b3);
                    // Character.isSurrogate
                    if (0xD800..=0xDFFF).contains(&c) {
                        dst.push(REPL);
                    } else {
                        dst.push(c);
                    }
                }
                continue;
            }
            if sp < sl && is_malformed3_2(b1, sb(src, sp)) {
                dst.push(REPL);
                continue;
            }
            dst.push(REPL);
            break;
        } else if (b1 >> 3) == -2 {
            if sp + 2 < sl {
                let b2 = sb(src, sp);
                let b3 = sb(src, sp + 1);
                let b4 = sb(src, sp + 2);
                sp += 3;
                let uc = decode4(b1, b2, b3, b4);
                // Character.isSupplementaryCodePoint is the shortest-form / range check.
                if is_malformed4(b2, b3, b4) || !(0x10000..0x110000).contains(&uc) {
                    dst.push(REPL);
                    sp -= 4;
                    sp += malformed4(src, sp);
                } else {
                    // Character.highSurrogate / lowSurrogate
                    dst.push(((uc >> 10) + (0xD800 - (0x10000 >> 10))) as u16);
                    dst.push(((uc & 0x3ff) + 0xDC00) as u16);
                }
                continue;
            }
            b1 &= 0xff;
            if b1 > 0xf4 || (sp < sl && is_malformed4_2(b1, sb(src, sp) & 0xff)) {
                dst.push(REPL);
                continue;
            }
            sp += 1;
            dst.push(REPL);
            if sp < sl && is_malformed4_3(sb(src, sp)) {
                continue;
            }
            break;
        } else {
            dst.push(REPL);
        }
    }
    // Lone surrogates are always replaced above, so the units are well-formed UTF-16.
    String::from_utf16(&dst).expect("the JDK UTF-8 decoder never emits a lone surrogate")
}
