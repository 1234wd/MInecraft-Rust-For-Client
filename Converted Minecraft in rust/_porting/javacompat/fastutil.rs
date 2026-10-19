//! The parts of fastutil 8.5.18 (`it.unimi.dsi.fastutil`) that vanilla code calls, method by
//! method. Session 24.
//!
//! Read from fastutil's own bytecode (`javap -c it.unimi.dsi.fastutil.HashCommon`), not from
//! memory: the constant below is the `ldc2_w` operand of `HashCommon.mix(long)`.
//!
//! | Java | here | used by |
//! |---|---|---|
//! | `HashCommon.mix(long)` | [`hash_common::mix_i64`] | `FeatureFlagSet.hashCode` |
//! | `DoubleList.getDouble(int)`, `size()` | [`DoubleList`] | `CubePointRange`, `OffsetDoubleList`, `IndexMerger` |

/// `it.unimi.dsi.fastutil.HashCommon`.
pub mod hash_common {
    /// `HashCommon.LONG_PHI`, `2^64 / phi`: the `ldc2_w -7046029254386353131L` in `mix(long)`.
    pub const LONG_PHI: i64 = -7046029254386353131;

    /// Port of `public static long HashCommon.mix(long x)`.
    ///
    /// ```java
    /// long h = x * LONG_PHI;  // lmul: wrapping
    /// h ^= h >>> 32;          // lushr: logical
    /// return h ^ (h >>> 16);
    /// ```
    pub const fn mix_i64(x: i64) -> i64 {
        let mut h = x.wrapping_mul(LONG_PHI);
        h ^= ((h as u64) >> 32) as i64;
        h ^ ((h as u64) >> 16) as i64
    }
}

/// The `it.unimi.dsi.fastutil.doubles.DoubleList` methods used by the mirrored classes.
///
/// This models the read surface those classes call. It does not claim to implement fastutil's
/// mutable list, iterator, comparison, or bulk-operation APIs.
pub trait DoubleList {
    /// `DoubleList.getDouble(int)`.
    fn get_double(&self, index: i32) -> f64;

    /// `DoubleList.size()`.
    fn size(&self) -> i32;
}

/// The `it.unimi.dsi.fastutil.ints.IntIterator` methods used by `Divisor`.
pub trait IntIterator {
    /// `IntIterator.hasNext()`.
    fn has_next(&self) -> bool;

    /// `IntIterator.nextInt()`, which throws when exhausted.
    fn next_int(&mut self) -> i32;

    /// `IntIterator.skip(int)`.
    fn skip(&mut self, count: i32) -> i32 {
        if count < 0 {
            panic!("java.lang.IllegalArgumentException: Argument must be nonnegative: {count}");
        }
        let mut skipped = 0;
        while skipped < count && self.has_next() {
            self.next_int();
            skipped += 1;
        }
        skipped
    }

    /// `PrimitiveIterator.OfInt.forEachRemaining(IntConsumer)`. Java validates the consumer
    /// before iterating, including when this iterator is empty.
    fn for_each_remaining(&mut self, mut action: Option<&mut dyn FnMut(i32)>) {
        if action.is_none() {
            panic!("java.lang.NullPointerException");
        }
        while self.has_next() {
            let value = self.next_int();
            action.as_deref_mut().expect("non-null action")(value);
        }
    }

    /// `IntIterator.next()`'s boxed result.
    fn next_boxed(&mut self) -> i32 {
        self.next_int()
    }

    /// `Iterator.remove()`'s default implementation.
    fn remove(&mut self) -> ! {
        panic!("java.lang.UnsupportedOperationException");
    }
}
