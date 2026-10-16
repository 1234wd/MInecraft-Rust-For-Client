//! Port of: net/minecraft/util/datafix/PackedBitStorage.java
//! Java class(es): net.minecraft.util.datafix.PackedBitStorage
//! Status: PORTED
//!
//! The pre-1.16 chunk packing used by the data fixers: values are laid end to end and MAY
//! straddle two longs (contrast `SimpleBitStorage`, which never splits a value). Array length is
//! `Mth.roundToward(size * bits, 64) / 64`, with Java `int` overflow in `size * bits`.
//!
//! All shifts are Java `long` shifts (count masked to 6 bits), `>>>` is a logical shift.
//! Exceptions match the jar: `Validate.inclusiveBetween` text, and
//! `java.lang.IllegalArgumentException: Invalid length given for storage, got: N but expected: M`.
//! The two-argument constructor allocates its array BEFORE the bit-count check, exactly like the
//! Java `this(bits, size, new long[...])` call, so a negative length fails first with
//! `java.lang.NegativeArraySizeException`.

use crate::net::minecraft::util::Mth::Mth;

/// Port of `PackedBitStorage#BIT_TO_LONG_SHIFT` (private in Java).
pub const BIT_TO_LONG_SHIFT: i32 = 6;

/// `org.apache.commons.lang3.Validate#inclusiveBetween(long, long, long)`.
fn validate_inclusive_between(start: i64, end: i64, value: i64) {
    if value < start || value > end {
        panic!(
            "java.lang.IllegalArgumentException: The value {value} is not in the specified inclusive range of {start} to {end}"
        );
    }
}

/// Java `long >>> n` (count masked to 6 bits).
#[inline]
fn ushr(v: i64, n: i32) -> i64 {
    ((v as u64) >> ((n & 63) as u32)) as i64
}

/// Java `long << n` (count masked to 6 bits).
#[inline]
fn shl(v: i64, n: i32) -> i64 {
    v.wrapping_shl((n & 63) as u32)
}

/// Java `long >> n` (count masked to 6 bits).
#[inline]
fn shr(v: i64, n: i32) -> i64 {
    v.wrapping_shr((n & 63) as u32)
}

/// Port of `net.minecraft.util.datafix.PackedBitStorage`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackedBitStorage {
    data: Vec<i64>,
    bits: i32,
    mask: i64,
    size: i32,
}

impl PackedBitStorage {
    /// Port of `PackedBitStorage(int bits, int size)`: zero-filled storage.
    pub fn new(bits: i32, size: i32) -> PackedBitStorage {
        let length = Mth::round_toward_i32(size.wrapping_mul(bits), 64) / 64;
        if length < 0 {
            panic!("java.lang.NegativeArraySizeException: {length}");
        }
        PackedBitStorage::with_data(bits, size, vec![0i64; length as usize])
    }

    /// Port of `PackedBitStorage(int bits, int size, long[] data)`.
    pub fn with_data(bits: i32, size: i32, data: Vec<i64>) -> PackedBitStorage {
        validate_inclusive_between(1, 32, bits as i64);
        let mask = (1i64 << bits) - 1;
        let required_length = Mth::round_toward_i32(size.wrapping_mul(bits), 64) / 64;
        if data.len() as i64 != required_length as i64 {
            panic!(
                "java.lang.IllegalArgumentException: Invalid length given for storage, got: {} but expected: {}",
                data.len(),
                required_length
            );
        }
        PackedBitStorage { data, bits, mask, size }
    }

    /// Port of `PackedBitStorage#set(int, int)`.
    pub fn set(&mut self, index: i32, value: i32) {
        validate_inclusive_between(0, self.size.wrapping_sub(1) as i64, index as i64);
        validate_inclusive_between(0, self.mask, value as i64);
        let position = index.wrapping_mul(self.bits);
        let start_data = position >> 6;
        let end_data = index.wrapping_add(1).wrapping_mul(self.bits).wrapping_sub(1) >> 6;
        let start_bit = position ^ start_data.wrapping_shl(6);
        let masked = (value as i64) & self.mask;
        let s = start_data as usize;
        self.data[s] = (self.data[s] & !shl(self.mask, start_bit)) | shl(masked, start_bit);
        if start_data != end_data {
            let shift_bits = 64 - start_bit;
            let wanted_bits = self.bits - shift_bits;
            let e = end_data as usize;
            self.data[e] = shl(ushr(self.data[e], wanted_bits), wanted_bits) | shr(masked, shift_bits);
        }
    }

    /// Port of `PackedBitStorage#get(int)`.
    pub fn get(&self, index: i32) -> i32 {
        validate_inclusive_between(0, self.size.wrapping_sub(1) as i64, index as i64);
        let position = index.wrapping_mul(self.bits);
        let start_data = position >> 6;
        let end_data = index.wrapping_add(1).wrapping_mul(self.bits).wrapping_sub(1) >> 6;
        let start_bit = position ^ start_data.wrapping_shl(6);
        if start_data == end_data {
            return (ushr(self.data[start_data as usize], start_bit) & self.mask) as i32;
        }
        let shift_bits = 64 - start_bit;
        ((ushr(self.data[start_data as usize], start_bit) | shl(self.data[end_data as usize], shift_bits))
            & self.mask) as i32
    }

    /// Port of `PackedBitStorage#getRaw()`.
    pub fn get_raw(&self) -> &[i64] {
        &self.data
    }

    /// Mutable view of the backing array (Java `getRaw()` hands out the live `long[]`).
    pub fn get_raw_mut(&mut self) -> &mut [i64] {
        &mut self.data
    }

    /// Port of `PackedBitStorage#getBits()`.
    pub fn get_bits(&self) -> i32 {
        self.bits
    }
}
