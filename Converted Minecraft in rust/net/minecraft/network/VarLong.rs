//! Port of: net/minecraft/network/VarLong.java
//! Java class(es): net.minecraft.network.VarLong
//! Status: PORTED
//!
//! The 64-bit sibling of `VarInt`: little-endian 7-bit groups, bit 7 = "more follows", at most
//! 10 bytes. Every field is private, so the class has no `consts.txt` group; the encodings in
//! `_porting/tests/session19_e.rs` were instead cross-checked against a run of the 26.2 jar.
//!
//! `read`/`write` work on a byte slice + reader index / a `Vec<u8>` instead of a netty
//! `ByteBuf`, with the same `Unpooled.wrappedBuffer` exception text as `VarInt` (see that
//! module's docs; the byte reader is shared from there).
//!
//! Java quirks preserved: negative values always take **10** bytes (`>>>=`); `read` ORs each
//! byte in before the length check, with the `long` shift count masked to 6 bits, so the tenth
//! byte keeps only its lowest bit (`ff x9 7f` reads as `-1`) and an eleventh byte throws
//! `"VarLong too big"`. Note the data byte is masked as an `int` and only then widened to
//! `long` (`(long)(in & 127)`), so no sign extension leaks in.

use crate::net::minecraft::network::VarInt::wrapped_buffer_read_byte;

/// Java: `private static final int MAX_VARLONG_SIZE` = `10`.
const MAX_VARLONG_SIZE: i32 = 10;
/// Java: `private static final int DATA_BITS_MASK` = `127`.
const DATA_BITS_MASK: i32 = 127;
/// Java: `private static final int CONTINUATION_BIT_MASK` = `128`.
const CONTINUATION_BIT_MASK: i32 = 128;
/// Java: `private static final int DATA_BITS_PER_BYTE` = `7`.
const DATA_BITS_PER_BYTE: i32 = 7;

/// Port of `VarLong#getByteSize(long)`: how many bytes `write` emits for `value` (1..=10).
pub fn get_byte_size(value: i64) -> i32 {
    for i in 1..MAX_VARLONG_SIZE {
        // Java: `(value & -1L << i * 7) == 0L`.
        if (value & (-1_i64).wrapping_shl((i * DATA_BITS_PER_BYTE) as u32)) == 0 {
            return i;
        }
    }
    MAX_VARLONG_SIZE
}

/// Port of `VarLong#hasContinuationBit(byte)`: bit 7 of the (signed) byte.
#[inline]
pub fn has_continuation_bit(in_: i8) -> bool {
    (in_ as i32 & CONTINUATION_BIT_MASK) == CONTINUATION_BIT_MASK
}

/// Port of `VarLong#read(ByteBuf)`, over `input` read from `*reader_index` (advanced past the
/// bytes consumed).
///
/// Panics with `java.lang.RuntimeException: VarLong too big` on an eleventh byte, and with
/// netty's `IndexOutOfBoundsException` text when `input` runs out first.
pub fn read(input: &[u8], reader_index: &mut usize) -> i64 {
    let mut out: i64 = 0;
    let mut bytes: i32 = 0;
    loop {
        let in_ = wrapped_buffer_read_byte(input, reader_index);
        // Java: `out |= (long)(in & 127) << bytes++ * 7;` -- a long shift, count masked to 6 bits.
        out |= ((in_ as i32 & DATA_BITS_MASK) as i64).wrapping_shl((bytes * DATA_BITS_PER_BYTE) as u32);
        bytes += 1;
        if bytes > MAX_VARLONG_SIZE {
            panic!("java.lang.RuntimeException: VarLong too big");
        }
        if !has_continuation_bit(in_) {
            return out;
        }
    }
}

/// Port of `VarLong#write(ByteBuf, long)`: appends the encoding to `output` and returns it.
pub fn write(output: &mut Vec<u8>, value: i64) -> &mut Vec<u8> {
    let mut value = value;
    while (value & -128_i64) != 0 {
        // Java: `writeByte((int)(value & 127L) | 128)`; `writeByte` keeps the low 8 bits.
        output.push(((value & DATA_BITS_MASK as i64) as i32 | CONTINUATION_BIT_MASK) as u8);
        // Java: `value >>>= 7` (unsigned shift).
        value = ((value as u64) >> DATA_BITS_PER_BYTE) as i64;
    }
    // Java: `writeByte((int)value)`.
    output.push(value as i32 as u8);
    output
}
