//! Port of: net/minecraft/util/ZeroBitStorage.java
//! Java class(es): net.minecraft.util.ZeroBitStorage
//! Status: PORTED
//!
//! The 0-bit storage used by single-value palettes: every index reads 0 and only 0 may be
//! written. Index/value checks use `Validate.inclusiveBetween`, so `set(0, 1)` panics with
//! `java.lang.IllegalArgumentException: The value 1 is not in the specified inclusive range of 0 to 0`
//! (message checked against the jar).
//!
//! `copy()` returns `this` in Java. The Rust port returns a new box holding the same size; the
//! object has no mutable state, so nothing can observe the difference.

use super::BitStorage::BitStorage;

/// Port of `ZeroBitStorage#RAW`: the shared empty raw array.
pub const RAW: [i64; 0] = [];

/// `org.apache.commons.lang3.Validate#inclusiveBetween(long, long, long)`.
fn validate_inclusive_between(start: i64, end: i64, value: i64) {
    if value < start || value > end {
        panic!(
            "java.lang.IllegalArgumentException: The value {value} is not in the specified inclusive range of {start} to {end}"
        );
    }
}

/// Port of `net.minecraft.util.ZeroBitStorage`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZeroBitStorage {
    size: i32,
}

impl ZeroBitStorage {
    /// Port of `ZeroBitStorage(int size)`.
    pub const fn new(size: i32) -> ZeroBitStorage {
        ZeroBitStorage { size }
    }
}

impl BitStorage for ZeroBitStorage {
    /// Port of `ZeroBitStorage#getAndSet(int, int)`: validates, always returns 0.
    fn get_and_set(&mut self, index: i32, value: i32) -> i32 {
        validate_inclusive_between(0, self.size.wrapping_sub(1) as i64, index as i64);
        validate_inclusive_between(0, 0, value as i64);
        0
    }

    /// Port of `ZeroBitStorage#set(int, int)`: validates only.
    fn set(&mut self, index: i32, value: i32) {
        validate_inclusive_between(0, self.size.wrapping_sub(1) as i64, index as i64);
        validate_inclusive_between(0, 0, value as i64);
    }

    /// Port of `ZeroBitStorage#get(int)`.
    fn get(&self, index: i32) -> i32 {
        validate_inclusive_between(0, self.size.wrapping_sub(1) as i64, index as i64);
        0
    }

    /// Port of `ZeroBitStorage#getRaw()`: always [`RAW`].
    fn get_raw(&self) -> &[i64] {
        &RAW
    }

    /// Port of `ZeroBitStorage#getSize()`.
    fn get_size(&self) -> i32 {
        self.size
    }

    /// Port of `ZeroBitStorage#getBits()`: always 0.
    fn get_bits(&self) -> i32 {
        0
    }

    /// Port of `ZeroBitStorage#getAll(IntConsumer)`: `size` zeros.
    fn get_all(&self, output: &mut dyn FnMut(i32)) {
        for _ in 0..self.size {
            output(0);
        }
    }

    /// Port of `ZeroBitStorage#unpack(int[])`: `Arrays.fill(output, 0, size, 0)`.
    ///
    /// `Arrays.fill` range-checks first; a `size` past the end panics with Java's
    /// `ArrayIndexOutOfBoundsException` text and writes nothing.
    fn unpack(&self, output: &mut [i32]) {
        if self.size < 0 {
            panic!("java.lang.IllegalArgumentException: fromIndex(0) > toIndex({})", self.size);
        }
        if self.size as usize > output.len() {
            panic!(
                "java.lang.ArrayIndexOutOfBoundsException: Array index out of range: {}",
                self.size
            );
        }
        for v in &mut output[..self.size as usize] {
            *v = 0;
        }
    }

    /// Port of `ZeroBitStorage#copy()`.
    fn copy(&self) -> Box<dyn BitStorage> {
        Box::new(*self)
    }
}
