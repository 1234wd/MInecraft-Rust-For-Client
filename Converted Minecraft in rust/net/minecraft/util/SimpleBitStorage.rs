//! Port of: net/minecraft/util/SimpleBitStorage.java
//! Java class(es): net.minecraft.util.SimpleBitStorage, net.minecraft.util.SimpleBitStorage.InitializationException
//! Status: VERIFIED
//!
//! Packed `long[]` storage behind chunk palettes; this is a SAVE FORMAT, so the packing must be
//! bit-identical. Values never straddle two longs: each long holds `64 / bits` values and the
//! leftover high bits stay zero (unlike the legacy `datafix.PackedBitStorage`).
//!
//! The cell index is computed with the `MAGIC` multiply-shift division table, ported exactly
//! (`(index * mul + add) >> 32 >> shift`, unsigned `mul`/`add`). The tests check the table against
//! the jar via reflection-printed values and against true division for every `valuesPerLong`.
//!
//! Exceptions (all checked against the jar's messages):
//! * `Validate.inclusiveBetween` -> `java.lang.IllegalArgumentException: The value V is not in the
//!   specified inclusive range of A to B` (commons-lang3 3.20.0 wording).
//! * wrong `data` length -> `net.minecraft.util.SimpleBitStorage$InitializationException: Invalid
//!   length given for storage, got: N but expected: M`. Java callers CATCH this one
//!   (`PalettedContainer`), so it is also available as a `Result` via [`SimpleBitStorage::try_with_data`].
//! * a negative computed length -> `java.lang.NegativeArraySizeException: -7`.
//!
//! Out-of-range indices into caller-supplied arrays (`values` shorter than `size`, `unpack`
//! output too short) panic with Rust's slice message, not Java's
//! `ArrayIndexOutOfBoundsException` text.

use super::BitStorage::BitStorage;

/// Port of `SimpleBitStorage#MAGIC` (private in Java; exposed for the tests).
///
/// Rows of `(divideMul, divideAdd, divideShift)` indexed by `3 * (valuesPerLong - 1)`.
pub const MAGIC: [i32; 192] = [
    -1, -1, 0,
    i32::MIN, 0, 0,
    1431655765, 1431655765, 0,
    i32::MIN, 0, 1,
    858993459, 858993459, 0,
    715827882, 715827882, 0,
    613566756, 613566756, 0,
    i32::MIN, 0, 2,
    477218588, 477218588, 0,
    429496729, 429496729, 0,
    390451572, 390451572, 0,
    357913941, 357913941, 0,
    330382099, 330382099, 0,
    306783378, 306783378, 0,
    286331153, 286331153, 0,
    i32::MIN, 0, 3,
    252645135, 252645135, 0,
    238609294, 238609294, 0,
    226050910, 226050910, 0,
    214748364, 214748364, 0,
    204522252, 204522252, 0,
    195225786, 195225786, 0,
    186737708, 186737708, 0,
    178956970, 178956970, 0,
    171798691, 171798691, 0,
    165191049, 165191049, 0,
    159072862, 159072862, 0,
    153391689, 153391689, 0,
    148102320, 148102320, 0,
    143165576, 143165576, 0,
    138547332, 138547332, 0,
    i32::MIN, 0, 4,
    130150524, 130150524, 0,
    126322567, 126322567, 0,
    122713351, 122713351, 0,
    119304647, 119304647, 0,
    116080197, 116080197, 0,
    113025455, 113025455, 0,
    110127366, 110127366, 0,
    107374182, 107374182, 0,
    104755299, 104755299, 0,
    102261126, 102261126, 0,
    99882960, 99882960, 0,
    97612893, 97612893, 0,
    95443717, 95443717, 0,
    93368854, 93368854, 0,
    91382282, 91382282, 0,
    89478485, 89478485, 0,
    87652393, 87652393, 0,
    85899345, 85899345, 0,
    84215045, 84215045, 0,
    82595524, 82595524, 0,
    81037118, 81037118, 0,
    79536431, 79536431, 0,
    78090314, 78090314, 0,
    76695844, 76695844, 0,
    75350303, 75350303, 0,
    74051160, 74051160, 0,
    72796055, 72796055, 0,
    71582788, 71582788, 0,
    70409299, 70409299, 0,
    69273666, 69273666, 0,
    68174084, 68174084, 0,
    i32::MIN, 0, 5,
];

/// `org.apache.commons.lang3.Validate#inclusiveBetween(long, long, long)`.
fn validate_inclusive_between(start: i64, end: i64, value: i64) {
    if value < start || value > end {
        panic!(
            "java.lang.IllegalArgumentException: The value {value} is not in the specified inclusive range of {start} to {end}"
        );
    }
}

/// Port of `SimpleBitStorage.InitializationException` (a `RuntimeException`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitializationException {
    message: String,
}

impl InitializationException {
    /// Java `getMessage()`.
    pub fn get_message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for InitializationException {
    /// Java `Throwable#toString()`: binary class name, `": "`, message.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "net.minecraft.util.SimpleBitStorage$InitializationException: {}", self.message)
    }
}

/// Port of `net.minecraft.util.SimpleBitStorage`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleBitStorage {
    data: Vec<i64>,
    bits: i32,
    mask: i64,
    size: i32,
    values_per_long: i32,
    divide_mul: i32,
    divide_add: i32,
    divide_shift: i32,
}

impl SimpleBitStorage {
    /// Port of `SimpleBitStorage(int bits, int size, int[] values)`: packs `values[0..size]`.
    pub fn from_values(bits: i32, size: i32, values: &[i32]) -> SimpleBitStorage {
        let mut s = SimpleBitStorage::new(bits, size);
        let vpl = s.values_per_long;
        let mut output_index: usize = 0;
        let mut input_offset: i32 = 0;
        while input_offset <= size.wrapping_sub(vpl) {
            let mut packed_value: i64 = 0;
            let mut index_in_long = vpl - 1;
            while index_in_long >= 0 {
                packed_value = packed_value.wrapping_shl(bits as u32);
                packed_value |= (values[(input_offset + index_in_long) as usize] as i64) & s.mask;
                index_in_long -= 1;
            }
            s.data[output_index] = packed_value;
            output_index += 1;
            input_offset += vpl;
        }

        let remainder_count = size - input_offset;
        if remainder_count > 0 {
            let mut last_packed_value: i64 = 0;
            let mut index_in_long = remainder_count - 1;
            while index_in_long >= 0 {
                last_packed_value = last_packed_value.wrapping_shl(bits as u32);
                last_packed_value |= (values[(input_offset + index_in_long) as usize] as i64) & s.mask;
                index_in_long -= 1;
            }
            s.data[output_index] = last_packed_value;
        }
        s
    }

    /// Port of `SimpleBitStorage(int bits, int size)`: zero-filled storage.
    pub fn new(bits: i32, size: i32) -> SimpleBitStorage {
        SimpleBitStorage::with_data(bits, size, None)
    }

    /// Port of `SimpleBitStorage(int bits, int size, @Nullable long[] data)`.
    ///
    /// Panics with the `InitializationException` text when `data` has the wrong length; use
    /// [`Self::try_with_data`] where Java code catches it.
    pub fn with_data(bits: i32, size: i32, data: Option<Vec<i64>>) -> SimpleBitStorage {
        match SimpleBitStorage::try_with_data(bits, size, data) {
            Ok(s) => s,
            Err(e) => panic!("{e}"),
        }
    }

    /// The `SimpleBitStorage(int, int, long[])` constructor with its checked
    /// `InitializationException` surfaced as an `Err`. Every other Java exception still panics.
    pub fn try_with_data(
        bits: i32,
        size: i32,
        data: Option<Vec<i64>>,
    ) -> Result<SimpleBitStorage, InitializationException> {
        validate_inclusive_between(1, 32, bits as i64);
        let mask = (1i64 << bits) - 1;
        // `(char)(64 / bits)`: always 2..=64 here, so the char narrowing is the identity.
        let values_per_long = ((64 / bits) as u16) as i32;
        let row = (3 * (values_per_long - 1)) as usize;
        let divide_mul = MAGIC[row];
        let divide_add = MAGIC[row + 1];
        let divide_shift = MAGIC[row + 2];
        let required_length = size.wrapping_add(values_per_long).wrapping_sub(1) / values_per_long;
        let data = match data {
            Some(d) => {
                if d.len() as i64 != required_length as i64 {
                    return Err(InitializationException {
                        message: format!(
                            "Invalid length given for storage, got: {} but expected: {}",
                            d.len(),
                            required_length
                        ),
                    });
                }
                d
            }
            None => {
                if required_length < 0 {
                    panic!("java.lang.NegativeArraySizeException: {required_length}");
                }
                vec![0i64; required_length as usize]
            }
        };
        Ok(SimpleBitStorage {
            data,
            bits,
            mask,
            size,
            values_per_long,
            divide_mul,
            divide_add,
            divide_shift,
        })
    }

    /// Port of `SimpleBitStorage#cellIndex(int)` (private in Java; exposed for the tests).
    ///
    /// `(int)(bitIndex * toUnsignedLong(mul) + toUnsignedLong(add) >> 32 >> shift)`.
    pub fn cell_index(&self, bit_index: i32) -> i32 {
        let mul = (self.divide_mul as u32) as i64;
        let add = (self.divide_add as u32) as i64;
        (((bit_index as i64).wrapping_mul(mul).wrapping_add(add) >> 32) >> self.divide_shift) as i32
    }

    /// Mutable view of the backing array (Java `getRaw()` hands out the live `long[]`).
    pub fn get_raw_mut(&mut self) -> &mut [i64] {
        &mut self.data
    }
}

impl BitStorage for SimpleBitStorage {
    /// Port of `SimpleBitStorage#getAndSet(int, int)`.
    fn get_and_set(&mut self, index: i32, value: i32) -> i32 {
        validate_inclusive_between(0, self.size.wrapping_sub(1) as i64, index as i64);
        validate_inclusive_between(0, self.mask, value as i64);
        let cell_index = self.cell_index(index);
        let cell_value = self.data[cell_index as usize];
        let bit_index = ((index - cell_index * self.values_per_long) * self.bits) as u32;
        let old_value = ((cell_value >> bit_index) & self.mask) as i32;
        self.data[cell_index as usize] =
            (cell_value & !(self.mask << bit_index)) | (((value as i64) & self.mask) << bit_index);
        old_value
    }

    /// Port of `SimpleBitStorage#set(int, int)`.
    fn set(&mut self, index: i32, value: i32) {
        validate_inclusive_between(0, self.size.wrapping_sub(1) as i64, index as i64);
        validate_inclusive_between(0, self.mask, value as i64);
        let cell_index = self.cell_index(index);
        let cell_value = self.data[cell_index as usize];
        let bit_index = ((index - cell_index * self.values_per_long) * self.bits) as u32;
        self.data[cell_index as usize] =
            (cell_value & !(self.mask << bit_index)) | (((value as i64) & self.mask) << bit_index);
    }

    /// Port of `SimpleBitStorage#get(int)`.
    fn get(&self, index: i32) -> i32 {
        validate_inclusive_between(0, self.size.wrapping_sub(1) as i64, index as i64);
        let cell_index = self.cell_index(index);
        let cell_value = self.data[cell_index as usize];
        let bit_index = ((index - cell_index * self.values_per_long) * self.bits) as u32;
        ((cell_value >> bit_index) & self.mask) as i32
    }

    /// Port of `SimpleBitStorage#getRaw()`.
    fn get_raw(&self) -> &[i64] {
        &self.data
    }

    /// Port of `SimpleBitStorage#getSize()`.
    fn get_size(&self) -> i32 {
        self.size
    }

    /// Port of `SimpleBitStorage#getBits()`.
    fn get_bits(&self) -> i32 {
        self.bits
    }

    /// Port of `SimpleBitStorage#getAll(IntConsumer)`.
    fn get_all(&self, output: &mut dyn FnMut(i32)) {
        let mut count: i32 = 0;
        for &cell in &self.data {
            let mut cell_value = cell;
            for _ in 0..self.values_per_long {
                output((cell_value & self.mask) as i32);
                cell_value >>= self.bits;
                count += 1;
                if count >= self.size {
                    return;
                }
            }
        }
    }

    /// Port of `SimpleBitStorage#unpack(int[])`.
    fn unpack(&self, output: &mut [i32]) {
        let data_length = self.data.len() as i32;
        let mut output_offset: i32 = 0;

        for i in 0..(data_length - 1).max(0) {
            let mut cell_value = self.data[i as usize];
            for index_in_long in 0..self.values_per_long {
                output[(output_offset + index_in_long) as usize] = (cell_value & self.mask) as i32;
                cell_value >>= self.bits;
            }
            output_offset += self.values_per_long;
        }

        let remainder = self.size - output_offset;
        if remainder > 0 {
            let mut cell_value = self.data[(data_length - 1) as usize];
            for index_in_long in 0..remainder {
                output[(output_offset + index_in_long) as usize] = (cell_value & self.mask) as i32;
                cell_value >>= self.bits;
            }
        }
    }

    /// Port of `SimpleBitStorage#copy()`: a deep copy of the backing array.
    fn copy(&self) -> Box<dyn BitStorage> {
        Box::new(SimpleBitStorage::with_data(self.bits, self.size, Some(self.data.clone())))
    }
}
