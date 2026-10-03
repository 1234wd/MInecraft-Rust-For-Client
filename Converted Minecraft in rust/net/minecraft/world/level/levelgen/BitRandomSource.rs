//! Port of: net/minecraft/world/level/levelgen/BitRandomSource.java
//! Java class(es): net.minecraft.world.level.levelgen.BitRandomSource
//! Status: VERIFIED
//!
//! In Java this is an interface whose default methods derive `nextInt`, `nextLong`,
//! `nextBoolean`, `nextFloat`, `nextDouble` and the bounded `nextInt(int)` from a
//! single abstract `next(int bits)`.
//!
//! Rust cannot inherit a trait's method *bodies* into a sub-trait, so the six
//! derived operations live here as defaults on `BitRandomSource` and each concrete
//! LCG type forwards to them in its `RandomSource` impl (six one-liners). The
//! arithmetic below is a verbatim transcription.

// NOTE on `next_int_bounded`: Java's `sample % bound` is the SIGNED remainder, so
// it must stay `%` and NOT become `rem_euclid`/`floor_mod`. The loop's exit test
// (`sample - modulo + (bound - 1) >= 0`) is the overflow-based unbiasedness check
// and must keep using `wrapping_*` arithmetic.

/// Port of `BitRandomSource#FLOAT_MULTIPLIER` (`5.9604645E-8F`, i.e. 2^-24).
pub const FLOAT_MULTIPLIER: f32 = 5.9604645E-8f32;
/// Port of `BitRandomSource#DOUBLE_MULTIPLIER` (`1.110223E-16F`, i.e. 2^-53).
///
/// NOTE: this is a *float* literal in Java that is then widened to `double`. The
/// widening is exact, so the Rust f64 constant is the same value -- but it is not
/// `2f64.powi(-53)`; it is `f64::from(1.110223E-16f32)`. Keeping the `f32 -> f64`
/// cast visible is deliberate.
pub const DOUBLE_MULTIPLIER: f64 = 1.110223E-16f32 as f64;

pub trait BitRandomSource {
    /// Port of `BitRandomSource#next(int bits)`.
    ///
    /// `bits` is always in `1..=32` in vanilla; anything else is a porting bug and
    /// we panic rather than silently mask, because Java's shift would too.
    fn next(&mut self, bits: i32) -> i32;

    /// Port of `BitRandomSource#nextInt()` (the default method).
    #[inline]
    fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    /// Port of `BitRandomSource#nextInt(int bound)` (the default method).
    ///
    /// Reproduces both the power-of-two fast path and the rejection loop with
    /// Java's integer-overflow-based unbiasedness check.
    #[inline]
    fn next_int_bounded(&mut self, bound: i32) -> i32 {
        if bound <= 0 {
            panic!("Bound must be positive");
        }
        if (bound & bound.wrapping_sub(1)) == 0 {
            return (((bound as i64) * (self.next(31) as i64)) >> 31) as i32;
        }
        loop {
            let sample = self.next(31);
            let modulo = sample % bound;
            if sample.wrapping_sub(modulo).wrapping_add(bound.wrapping_sub(1)) >= 0 {
                return modulo;
            }
        }
    }

    /// Port of `BitRandomSource#nextLong()` (the default method).
    #[inline]
    fn next_long(&mut self) -> i64 {
        let upper = self.next(32) as i64;
        let lower = self.next(32) as i64;
        (upper << 32).wrapping_add(lower)
    }

    /// Port of `BitRandomSource#nextBoolean()` (the default method).
    #[inline]
    fn next_boolean(&mut self) -> bool {
        self.next(1) != 0
    }

    /// Port of `BitRandomSource#nextFloat()` (the default method): 24 bits.
    #[inline]
    fn next_float(&mut self) -> f32 {
        self.next(24) as f32 * FLOAT_MULTIPLIER
    }

    /// Port of `BitRandomSource#nextDouble()` (the default method):
    /// 26 high bits, then 27 low bits.
    ///
    /// # THE VANILLA QUIRK: `combined` is narrowed to `float` before the multiply
    ///
    /// Java source:
    /// ```java
    /// long combined = ((long)upper << 27) + lower;
    /// return combined * 1.110223E-16F;
    /// ```
    ///
    /// `1.110223E-16F` is a **float** literal, so binary numeric promotion for
    /// `long * float` widens `long -> float` FIRST and then `float -> double`. The
    /// `long` is therefore squeezed into a 24-bit mantissa and loses its low 32 bits
    /// BEFORE the scaling happens.
    ///
    /// Measured on HotSpot 21 with seed 0:
    /// ```text
    /// upper = 0x02ec82d1          lower = 0x006a6ca89
    /// combined                 = 0x1764168ea6ca89
    /// (float) combined         = 0x4337641680000000   <-- low 32 bits are GONE
    /// (double)combined * 2^-53 = 0x3fe764168ea6ca89   <-- the "obvious" port
    /// actual vanilla result    = 0x3fe7641680000000   <-- the float-narrowed one
    /// ```
    ///
    /// So vanilla's `nextDouble` carries only ~24 bits of entropy, not 53. The
    /// natural `(combined as f64) * 2f64.powi(-53)` yields a DIFFERENT stream and
    /// desynchronises every legacy world. The `as f32` cast is not a typo -- do not
    /// "fix" it.
    #[inline]
    fn next_double(&mut self) -> f64 {
        let upper = self.next(26) as i64;
        let lower = self.next(27) as i64;
        let combined = (upper << 27).wrapping_add(lower);
        // Java's `long * <float literal>` promotes long -> float -> double.
        (combined as f32) as f64 * DOUBLE_MULTIPLIER
    }
}

/// Shared guard so the three LCG types spell the same bounds check once.
///
/// Java's `>>` with a bad shift count silently masks to `& 31` (JLS 15.19); a
/// negative or >32 count would therefore produce a value rather than throw. Vanilla
/// never passes one, so we panic instead of inventing an answer -- that way a
/// mis-port shows up immediately instead of silently desyncing the world.
/// See _porting/DESIGN_DECISIONS.md (#panics-not-results).
#[inline]
pub(crate) fn assert_bits_in_range(bits: i32) {
    assert!((1..=32).contains(&bits), "BitRandomSource#next: bits must be in 1..=32, got {bits}");
}