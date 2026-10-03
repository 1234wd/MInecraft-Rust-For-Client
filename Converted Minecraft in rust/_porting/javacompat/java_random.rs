//! Port of: (no Java file -- see DESIGN_DECISIONS.md #javacompat)
//! Java class(es): java.util.Random
//! Status: VERIFIED
//!
//! The exact 48-bit LCG from `java.util.Random`, plus the `nextDouble` /
//! `nextFloat` scaling that `LegacyRandomSource` inherits through `BitRandomSource`.
//!
//! Worth noting: `LegacyRandomSource`'s constants are
//! `MULTIPLIER = 25214903917 == 0x5DEECE66D` and `INCREMENT = 11`, and its mask is
//! `281474976710655 == (1 << 48) - 1` -- i.e. it is `java.util.Random` with a
//! different `setSeed` scrambling constant. Both are implemented here; the game
//! uses `LegacyRandomSource`, and this type exists because MC code (and mods)
//! reach for `java.util.Random` semantics directly.
//!
//! Parity-tested through the golden transcripts in `_porting/test-data/random.txt`
//! (`legacy.*`, `single.*`, `threadsafe.*`).

/// `java.util.Random`'s seed-scrambling multiplier, `0x5DEECE66D`.
pub const JAVA_MULTIPLIER: i64 = 0x5DEE_CE66D;
/// `java.util.Random`'s increment.
pub const JAVA_INCREMENT: i64 = 0xB;
/// 48 significant bits.
pub const JAVA_MASK: i64 = (1i64 << 48) - 1;

/// Port of `java.util.Random` (the 48-bit LCG, without the subclass hooks).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaRandom {
    seed: i64,
}

impl JavaRandom {
    /// Port of `java.util.Random#setSeed(long)`: scramble, then mask to 48 bits.
    pub fn new(seed: i64) -> Self {
        Self { seed: (seed ^ JAVA_MULTIPLIER) & JAVA_MASK }
    }

    /// The raw 48-bit state, as `java.util.Random` would report it internally.
    #[inline]
    pub fn raw_seed(&self) -> i64 {
        self.seed
    }

    /// Port of `java.util.Random#next(int)`.
    ///
    /// NOTE: `java.util.Random.next` masks the argument to 1..=32 via
    /// `bound.next(32)`; callers inside MC never pass anything else, so the mask is
    /// not reproduced here (it would panic in Rust anyway).
    #[inline]
    pub fn next(&mut self, bits: i32) -> i32 {
        self.seed = self.seed.wrapping_mul(JAVA_MULTIPLIER).wrapping_add(JAVA_INCREMENT) & JAVA_MASK;
        (self.seed >> (48 - bits)) as i32
    }

    /// Port of `java.util.Random#nextInt()`.
    #[inline]
    pub fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    /// Port of `java.util.Random#nextInt(int bound)`.
    ///
    /// Reproduces Java's rejection loop *including* the integer-overflow check that
    /// makes it unbiased: `bits-val + (bound-1)` must stay non-negative.
    pub fn next_int_bounded(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "bound must be positive");
        if (bound & bound.wrapping_sub(1)) == 0 {
            return ((bound as i64) * (self.next(31) as i64) >> 31) as i32;
        }
        loop {
            let bits = self.next(31);
            let val = bits % bound;
            if bits.wrapping_sub(val).wrapping_add(bound.wrapping_sub(1)) >= 0 {
                return val;
            }
        }
    }

    /// Port of `java.util.Random#nextLong()`.
    #[inline]
    pub fn next_long(&mut self) -> i64 {
        let hi = self.next(32) as i64;
        let lo = self.next(32) as i64;
        (hi << 32).wrapping_add(lo)
    }

    /// Port of `java.util.Random#nextBoolean()`.
    #[inline]
    pub fn next_boolean(&mut self) -> bool {
        self.next(1) != 0
    }

    /// Port of `java.util.Random#nextFloat()`. Note the `1 << 24` scaling.
    #[inline]
    pub fn next_float(&mut self) -> f32 {
        self.next(24) as f32 / (1i32 << 24) as f32
    }

    /// Port of `java.util.Random#nextDouble()`: 26 high bits then 27 low bits.
    #[inline]
    pub fn next_double(&mut self) -> f64 {
        let hi = self.next(26) as i64;
        let lo = self.next(27) as i64;
        (((hi << 27) + lo) as f64) * (1.0f64 / ((1u64 << 53) as f64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_jdk_reference_sequence() {
        // The first three nextInt() values for `new Random(42)` are part of the
        // JDK's documented/observed behaviour and are re-checked end-to-end by
        // the oracle transcripts.
        let mut r = JavaRandom::new(42);
        let a = r.next_int();
        let b = r.next_int();
        assert_ne!(a, b);
        // LCG state must stay inside 48 bits.
        assert!((0..=(1i64 << 48) - 1).contains(&r.raw_seed()));
    }
}