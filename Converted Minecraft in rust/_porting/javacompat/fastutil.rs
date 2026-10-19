//! The parts of fastutil 8.5.18 (`it.unimi.dsi.fastutil`) that vanilla code calls, method by
//! method. Session 24.
//!
//! Read from fastutil's own bytecode (`javap -c it.unimi.dsi.fastutil.HashCommon`), not from
//! memory: the constant below is the `ldc2_w` operand of `HashCommon.mix(long)`.
//!
//! | Java | here | used by |
//! |---|---|---|
//! | `HashCommon.mix(long)` | [`hash_common::mix_i64`] | `FeatureFlagSet.hashCode` |

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
