//! Port of: net/minecraft/world/level/levelgen/ThreadSafeLegacyRandomSource.java
//! Java class(es): net.minecraft.world.level.levelgen.ThreadSafeLegacyRandomSource
//! Status: VERIFIED
//!
//! Deprecated in vanilla and only reachable through `RandomSource#createThreadSafe()`.
//! Kept because removing it would change the public surface, and because its
//! `next(int)` differs from `LegacyRandomSource#next(int)` in one small but
//! REAL way: Java uses the UNSIGNED shift `>>> 48 - bits` here, versus the signed
//! `>>` in `LegacyRandomSource`. The state is 48 bits and therefore never negative,
//! so the two agree in practice -- but the difference is deliberate in vanilla and
//! must not be "tidied up".
//!
//! Java loops a CAS to make the read-modify-write atomic. Rust's `&mut self` is
//! already exclusive, so the loop has no counterpart.

use crate::net::minecraft::util::RandomSource::RandomSource;
use crate::net::minecraft::world::level::levelgen::{
    BitRandomSource::{assert_bits_in_range, BitRandomSource},
    LegacyRandomSource::{LegacyRandomSource, INCREMENT, MODULUS_MASK, MULTIPLIER},
    MarsagliaPolarGaussian::MarsagliaPolarGaussian,
    PositionalRandomFactory::PositionalRandomFactory,
};

pub struct ThreadSafeLegacyRandomSource {
    seed: i64,
    gaussian_source: MarsagliaPolarGaussian,
}

impl ThreadSafeLegacyRandomSource {
    /// Port of `ThreadSafeLegacyRandomSource#ThreadSafeLegacyRandomSource(long)`.
    pub fn new(seed: i64) -> Self {
        let mut me = Self { seed: 0, gaussian_source: MarsagliaPolarGaussian::new() };
        me.set_seed(seed);
        me
    }
}

impl BitRandomSource for ThreadSafeLegacyRandomSource {
    /// Port of `ThreadSafeLegacyRandomSource#next(int)`.
    #[inline]
    fn next(&mut self, bits: i32) -> i32 {
        assert_bits_in_range(bits);
        self.seed = self.seed.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) & MODULUS_MASK;
        // Java: `(int)(nextSeed >>> 48 - bits)` -- unsigned shift, unlike
        // LegacyRandomSource's `>>`. See the module docs.
        ((self.seed as u64) >> (48 - bits)) as i32
    }
}

impl RandomSource for ThreadSafeLegacyRandomSource {
    fn fork(&mut self) -> Box<dyn RandomSource> {
        Box::new(ThreadSafeLegacyRandomSource::new(RandomSource::next_long(self)))
    }

    /// Port of `ThreadSafeLegacyRandomSource#forkPositional()`.
    fn fork_positional(&mut self) -> Box<dyn PositionalRandomFactory> {
        Box::new(LegacyRandomSource::legacy_positional_random_factory(RandomSource::next_long(self)))
    }

    /// Port of `ThreadSafeLegacyRandomSource#setSeed(long)`.
    ///
    /// NOTE: unlike `LegacyRandomSource`, this one does NOT reset the gaussian
    /// source. Preserved as-is.
    #[inline]
    fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULTIPLIER) & MODULUS_MASK;
    }

    fn next_int(&mut self) -> i32 {
        BitRandomSource::next_int(self)
    }

    fn next_int_bounded(&mut self, bound: i32) -> i32 {
        BitRandomSource::next_int_bounded(self, bound)
    }

    fn next_long(&mut self) -> i64 {
        BitRandomSource::next_long(self)
    }

    fn next_boolean(&mut self) -> bool {
        BitRandomSource::next_boolean(self)
    }

    fn next_float(&mut self) -> f32 {
        BitRandomSource::next_float(self)
    }

    fn next_double(&mut self) -> f64 {
        BitRandomSource::next_double(self)
    }

    /// Port of `ThreadSafeLegacyRandomSource#nextGaussian()`.
    fn next_gaussian(&mut self) -> f64 {
        let mut gaussian = self.gaussian_source;
        let value = gaussian.next_gaussian(self);
        self.gaussian_source = gaussian;
        value
    }

    fn as_bit_source(&mut self) -> Option<&mut dyn BitRandomSource> {
        Some(self)
    }
}