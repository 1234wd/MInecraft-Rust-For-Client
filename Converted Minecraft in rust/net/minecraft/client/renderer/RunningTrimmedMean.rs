//! Port of: net/minecraft/client/renderer/RunningTrimmedMean.java
//! Java class(es): net.minecraft.client.renderer.RunningTrimmedMean
//! Status: PORTED
//!
//! Session 19 batch h. Every member: the constructor and `registerValueAndGetMean`. Nothing is
//! PORT-BLOCKED; no constants, so no golden data (hence `PORTED`).
//!
//! # Vanilla quirks preserved
//!
//! * With MORE than two samples it returns the mean with the single min and max dropped,
//!   using `long` division (truncation toward zero) and wrapping `long` sums.
//! * With ONE or TWO samples it returns `count / total` -- the INVERSE of a mean -- when
//!   `total > 0`, else `0`. So a first sample of `16_000_000` returns `0`, a first sample of `1`
//!   returns `1`, and `(1, 1)` returns `1`. That is what the bytecode does.
//! * `new long[maxCount]` with a negative size throws `NegativeArraySizeException`, and a zero
//!   size makes the first `registerValueAndGetMean` throw `ArrayIndexOutOfBoundsException`; both
//!   are reproduced as panics with Java's messages.

/// Port of `net.minecraft.client.renderer.RunningTrimmedMean`.
#[derive(Clone, Debug)]
pub struct RunningTrimmedMean {
    values: Vec<i64>,
    count: i32,
    cursor: i32,
}

impl RunningTrimmedMean {
    /// Port of `RunningTrimmedMean(int maxCount)`.
    pub fn new(max_count: i32) -> Self {
        if max_count < 0 {
            panic!("java.lang.NegativeArraySizeException: {max_count}");
        }
        Self {
            values: vec![0; max_count as usize],
            count: 0,
            cursor: 0,
        }
    }

    /// Port of `RunningTrimmedMean#registerValueAndGetMean(long)`.
    pub fn register_value_and_get_mean(&mut self, value: i64) -> i64 {
        let length = self.values.len() as i32;
        if self.count < length {
            self.count += 1;
        }

        if self.cursor >= length {
            panic!(
                "java.lang.ArrayIndexOutOfBoundsException: Index {} out of bounds for length {}",
                self.cursor, length
            );
        }
        self.values[self.cursor as usize] = value;
        self.cursor = (self.cursor + 1) % length;
        let mut min = i64::MAX;
        let mut max = i64::MIN;
        let mut total: i64 = 0;

        for i in 0..self.count {
            let current = self.values[i as usize];
            total = total.wrapping_add(current);
            min = std::cmp::min(min, current);
            max = std::cmp::max(max, current);
        }

        if self.count > 2 {
            total = total.wrapping_sub(min.wrapping_add(max));
            total / (self.count - 2) as i64
        } else if total > 0 {
            // `int / long`: the int is widened, so this is long division of count by total.
            self.count as i64 / total
        } else {
            0
        }
    }
}
