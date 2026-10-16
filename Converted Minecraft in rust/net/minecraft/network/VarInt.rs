//! Port of: net/minecraft/network/VarInt.java
//! Java class(es): net.minecraft.network.VarInt
//! Status: VERIFIED
//!
//! The protocol's variable-length `int`: little-endian groups of 7 bits, bit 7 of each byte set
//! while more bytes follow. This is the network wire format, so the exact bytes matter; they are
//! pinned in `_porting/tests/session19_e.rs` against a run of the 26.2 jar, and
//! `MAX_VARINT_SIZE` against the constant dump.
//!
//! # `read`/`write` over a byte slice instead of a netty `ByteBuf`
//!
//! netty is not ported. `read` and `write` keep their Java names and bit logic, but operate on
//! a `&[u8]` plus a reader index (read) or a `Vec<u8>` (write). The slice behaves exactly like
//! `Unpooled.wrappedBuffer(input)` with its `readerIndex` at `*reader_index`, **including the
//! exception text** when the buffer runs dry, which netty formats from that buffer's state:
//!
//! ```text
//! java.lang.IndexOutOfBoundsException: readerIndex(2) + length(1) exceeds writerIndex(2): UnpooledHeapByteBuf(ridx: 2, widx: 2, cap: 2/2)
//! ```
//!
//! An empty array is the one exception: `Unpooled.wrappedBuffer(new byte[0])` returns netty's
//! shared `EMPTY_BUFFER`, whose `readByte` throws an `IndexOutOfBoundsException` with a **null**
//! message. Real pipeline buffers are pooled/direct and name a different class in that text;
//! only the heap-wrapper form is reproduced here.
//!
//! # Java quirks preserved
//!
//! - **Negative values always take 5 bytes**: `write` shifts with `>>>`, so the sign bit walks
//!   down into the fifth byte (`-1` -> `ff ff ff ff 0f`).
//! - `read` ORs the byte in **before** checking the length, and Java masks an `int` shift
//!   count to 5 bits, so the sixth byte is shifted by `35 & 31 = 3` -- harmless only because the
//!   `"VarInt too big"` throw follows immediately. A fifth byte's bits above bit 31 are dropped
//!   silently (`ff ff ff ff 7f` reads as `-1`, not an error).

/// Java: `VarInt.MAX_VARINT_SIZE` = `5`.
pub const MAX_VARINT_SIZE: i32 = 5;
/// Java: `private static final int DATA_BITS_MASK` = `127`.
const DATA_BITS_MASK: i32 = 127;
/// Java: `private static final int CONTINUATION_BIT_MASK` = `128`.
const CONTINUATION_BIT_MASK: i32 = 128;
/// Java: `private static final int DATA_BITS_PER_BYTE` = `7`.
const DATA_BITS_PER_BYTE: i32 = 7;

/// Port of `VarInt#getByteSize(int)`: how many bytes `write` emits for `value` (1..=5).
pub fn get_byte_size(value: i32) -> i32 {
    for i in 1..MAX_VARINT_SIZE {
        // Java: `(value & -1 << i * 7) == 0`; `*` binds tighter than `<<`.
        if (value & (-1_i32).wrapping_shl((i * DATA_BITS_PER_BYTE) as u32)) == 0 {
            return i;
        }
    }
    MAX_VARINT_SIZE
}

/// Port of `VarInt#hasContinuationBit(byte)`: bit 7 of the (signed) byte.
#[inline]
pub fn has_continuation_bit(in_: i8) -> bool {
    (in_ as i32 & CONTINUATION_BIT_MASK) == CONTINUATION_BIT_MASK
}

/// Port of `VarInt#read(ByteBuf)`, over `input` read from `*reader_index` (advanced past the
/// bytes consumed, as `ByteBuf#readByte` advances `readerIndex`).
///
/// Panics with `java.lang.RuntimeException: VarInt too big` on a sixth byte, and with netty's
/// `IndexOutOfBoundsException` text (see the module docs) when `input` runs out first.
pub fn read(input: &[u8], reader_index: &mut usize) -> i32 {
    let mut out: i32 = 0;
    let mut bytes: i32 = 0;
    loop {
        let in_ = wrapped_buffer_read_byte(input, reader_index);
        // Java: `out |= (in & 127) << bytes++ * 7;` -- an int shift, count masked to 5 bits.
        out |= (in_ as i32 & DATA_BITS_MASK).wrapping_shl((bytes * DATA_BITS_PER_BYTE) as u32);
        bytes += 1;
        if bytes > MAX_VARINT_SIZE {
            panic!("java.lang.RuntimeException: VarInt too big");
        }
        if !has_continuation_bit(in_) {
            return out;
        }
    }
}

/// Port of `VarInt#write(ByteBuf, int)`: appends the encoding to `output` (the writer index of
/// a growable buffer) and returns it, as Java returns the `ByteBuf` for chaining.
pub fn write(output: &mut Vec<u8>, value: i32) -> &mut Vec<u8> {
    let mut value = value;
    while (value & -128) != 0 {
        // `writeByte(int)` keeps the low 8 bits.
        output.push((value & DATA_BITS_MASK | CONTINUATION_BIT_MASK) as u8);
        // Java: `value >>>= 7` (unsigned shift).
        value = ((value as u32) >> DATA_BITS_PER_BYTE) as i32;
    }
    output.push(value as u8);
    output
}

/// `ByteBuf#readByte()` on `Unpooled.wrappedBuffer(input)` positioned at `*reader_index`.
///
/// Not a member of `VarInt`; shared with `VarLong`'s slice-based `read`. Reproduces netty
/// 4.2.15's `AbstractByteBuf#checkReadableBytes0` message for a heap wrapper (whose capacity and
/// max capacity are both the array length), and `EmptyByteBuf`'s null-message throw for an
/// empty array.
pub(crate) fn wrapped_buffer_read_byte(input: &[u8], reader_index: &mut usize) -> i8 {
    let ridx = *reader_index;
    let widx = input.len();
    if widx == 0 {
        // `Unpooled.wrappedBuffer(new byte[0])` is `Unpooled.EMPTY_BUFFER`; Throwable#toString
        // of a null-message exception is just the class name.
        panic!("java.lang.IndexOutOfBoundsException");
    }
    if ridx >= widx {
        panic!(
            "java.lang.IndexOutOfBoundsException: readerIndex({ridx}) + length(1) exceeds \
             writerIndex({widx}): UnpooledHeapByteBuf(ridx: {ridx}, widx: {widx}, cap: {widx}/{widx})"
        );
    }
    *reader_index = ridx + 1;
    input[ridx] as i8
}
