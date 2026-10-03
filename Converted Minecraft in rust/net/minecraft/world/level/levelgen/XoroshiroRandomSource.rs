//! Port of: net/minecraft/world/level/levelgen/XoroshiroRandomSource.java
//! Java class(es): net.minecraft.world.level.levelgen.XoroshiroRandomSource,
//!                  XoroshiroRandomSource.XoroshiroPositionalRandomFactory
//! Status: VERIFIED
//!
//! The default algorithm for modern worlds. Unlike the legacy LCGs, this one does
//! NOT implement `BitRandomSource`: it consumes a fresh 64-bit word for every
//! operation and slices bits out of it, so `nextInt(bound)` needs its own
//! Lemire-style multiply-shift with a rejection loop. That difference is the whole
//! reason this class exists separately in vanilla.

use crate::net::minecraft::util::{Mth::Mth, RandomSource::RandomSource};
use crate::net::minecraft::world::level::levelgen::{
    MarsagliaPolarGaussian::MarsagliaPolarGaussian,
    PositionalRandomFactory::PositionalRandomFactory,
    RandomSupport::{self, Seed128bit},
    Xoroshiro128PlusPlus::Xoroshiro128PlusPlus,
};

/// Port of `XoroshiroRandomSource#FLOAT_UNIT`.
pub const FLOAT_UNIT: f32 = 5.9604645E-8f32;
/// Port of `XoroshiroRandomSource#DOUBLE_UNIT` (`1.110223E-16F` widened to double).
pub const DOUBLE_UNIT: f64 = 1.110223E-16f32 as f64;

pub struct XoroshiroRandomSource {
    random_number_generator: Xoroshiro128PlusPlus,
    gaussian_source: MarsagliaPolarGaussian,
}

impl XoroshiroRandomSource {
    /// Port of `XoroshiroRandomSource#XoroshiroRandomSource(long)`.
    pub fn new(seed: i64) -> Self {
        Self::from_seed(RandomSupport::upgrade_seed_to_128bit(seed))
    }

    /// Port of `XoroshiroRandomSource#XoroshiroRandomSource(RandomSupport.Seed128bit)`.
    pub fn from_seed(seed: Seed128bit) -> Self {
        Self { random_number_generator: Xoroshiro128PlusPlus::from_seed(seed), gaussian_source: MarsagliaPolarGaussian::new() }
    }

    /// Port of `XoroshiroRandomSource#XoroshiroRandomSource(long,long)`.
    pub fn new_from_lo_hi(seed_lo: i64, seed_hi: i64) -> Self {
        Self {
            random_number_generator: Xoroshiro128PlusPlus::new(seed_lo, seed_hi),
            gaussian_source: MarsagliaPolarGaussian::new(),
        }
    }

    /// Port of `XoroshiroRandomSource#XoroshiroPositionalRandomFactory`.
    pub fn xoroshiro_positional_random_factory(seed_lo: i64, seed_hi: i64) -> XoroshiroPositionalRandomFactory {
        XoroshiroPositionalRandomFactory { seed_lo, seed_hi }
    }

    /// Port of `XoroshiroRandomSource#nextBits(int)`.
    #[inline]
    fn next_bits(&mut self, bits: i32) -> u64 {
        (self.random_number_generator.next_long() as u64) >> (64 - bits)
    }
}

impl RandomSource for XoroshiroRandomSource {
    /// Port of `XoroshiroRandomSource#fork()`.
    fn fork(&mut self) -> Box<dyn RandomSource> {
        let lo = self.random_number_generator.next_long();
        let hi = self.random_number_generator.next_long();
        Box::new(XoroshiroRandomSource::new_from_lo_hi(lo, hi))
    }

    /// Port of `XoroshiroRandomSource#forkPositional()`.
    ///
    /// Java evaluates the two `nextLong()` calls left to right; keep that order.
    fn fork_positional(&mut self) -> Box<dyn PositionalRandomFactory> {
        let lo = self.random_number_generator.next_long();
        let hi = self.random_number_generator.next_long();
        Box::new(XoroshiroRandomSource::xoroshiro_positional_random_factory(lo, hi))
    }

    /// Port of `XoroshiroRandomSource#setSeed(long)`.
    fn set_seed(&mut self, seed: i64) {
        self.random_number_generator = Xoroshiro128PlusPlus::from_seed(RandomSupport::upgrade_seed_to_128bit(seed));
        self.gaussian_source.reset();
    }

    /// Port of `XoroshiroRandomSource#nextInt()`.
    #[inline]
    fn next_int(&mut self) -> i32 {
        self.random_number_generator.next_long() as i32
    }

    /// Port of `XoroshiroRandomSource#nextInt(int bound)`.
    ///
    /// Lemire's multiply-shift with the "unbiased buckets" rejection loop. The
    /// `Integer.remainderUnsigned(~bound + 1, bound)` start index and the 32-bit
    /// mask on the low half must be reproduced exactly.
    fn next_int_bounded(&mut self, bound: i32) -> i32 {
        if bound <= 0 {
            panic!("Bound must be positive");
        }

        let mut random_bits = self.next_int() as u32 as i64;
        let mut multiplied_random_bits = random_bits.wrapping_mul(bound as i64);
        let mut fractional_part = multiplied_random_bits & 4294967295i64;
        if fractional_part < bound as i64 {
            // Java: `Integer.remainderUnsigned(~bound + 1, bound)` and `~bound + 1`
            // is exactly `-bound`; `remainderUnsigned` is an unsigned `%`.
            let unbiased_buckets_start_index = remainder_unsigned(bound.wrapping_neg(), bound as u32) as i64;
            while fractional_part < unbiased_buckets_start_index {
                random_bits = self.next_int() as u32 as i64;
                multiplied_random_bits = random_bits.wrapping_mul(bound as i64);
                fractional_part = multiplied_random_bits & 4294967295i64;
            }
        }

        let integer_part = multiplied_random_bits >> 32;
        integer_part as i32
    }

    /// Port of `XoroshiroRandomSource#nextLong()`.
    #[inline]
    fn next_long(&mut self) -> i64 {
        self.random_number_generator.next_long()
    }

    /// Port of `XoroshiroRandomSource#nextBoolean()`.
    #[inline]
    fn next_boolean(&mut self) -> bool {
        (self.random_number_generator.next_long() & 1i64) != 0
    }

    /// Port of `XoroshiroRandomSource#nextFloat()`.
    #[inline]
    fn next_float(&mut self) -> f32 {
        self.next_bits(24) as f32 * FLOAT_UNIT
    }

    /// Port of `XoroshiroRandomSource#nextDouble()`.
    ///
    /// SAME float-narrowing quirk as `BitRandomSource#nextDouble`:
    /// `nextBits(53) * 1.110223E-16F` is `long * <float literal>`, so Java widens
    /// `long -> float` first (keeping only 24 mantissa bits) and then
    /// `float -> double`. Writing `next_bits(53) as f64 * DOUBLE_UNIT` produces a
    /// DIFFERENT stream and desynchronises modern worlds.
    #[inline]
    fn next_double(&mut self) -> f64 {
        (self.next_bits(53) as f32) as f64 * DOUBLE_UNIT
    }

    /// Port of `XoroshiroRandomSource#nextGaussian()`.
    fn next_gaussian(&mut self) -> f64 {
        let mut gaussian = self.gaussian_source;
        let value = gaussian.next_gaussian(self);
        self.gaussian_source = gaussian;
        value
    }

    /// Port of the `XoroshiroRandomSource#consumeCount(int)` OVERRIDE.
    ///
    /// Xoroshiro overrides the interface default so that it burns one 64-bit word
    /// per round instead of one 32-bit `nextInt()`.
    fn consume_count(&mut self, rounds: i32) {
        for _ in 0..rounds {
            self.random_number_generator.next_long();
        }
    }
}

/// Port of `Integer.remainderUnsigned(int,int)`.
#[inline]
fn remainder_unsigned(dividend: i32, divisor: u32) -> u32 {
    // `(a as u32) % (b as u32)` is exactly Integer.remainderUnsigned.
    (dividend as u32) % divisor
}

/// Port of `XoroshiroRandomSource.XoroshiroPositionalRandomFactory`.
pub struct XoroshiroPositionalRandomFactory {
    seed_lo: i64,
    seed_hi: i64,
}

impl XoroshiroPositionalRandomFactory {
    /// Port of `XoroshiroPositionalRandomFactory#XoroshiroPositionalRandomFactory(long,long)`.
    pub fn new(seed_lo: i64, seed_hi: i64) -> Self {
        Self { seed_lo, seed_hi }
    }
}

impl PositionalRandomFactory for XoroshiroPositionalRandomFactory {
    /// Port of `XoroshiroPositionalRandomFactory#at(int,int,int)`.
    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource> {
        let positional_seed = Mth::get_seed(x, y, z);
        Box::new(XoroshiroRandomSource::new_from_lo_hi(positional_seed ^ self.seed_lo, self.seed_hi))
    }

    /// Port of `XoroshiroPositionalRandomFactory#fromHashOf(String)`.
    fn from_hash_of(&self, name: &str) -> Box<dyn RandomSource> {
        let seed = RandomSupport::seed_from_hash_of(name);
        Box::new(XoroshiroRandomSource::from_seed(seed.xor(self.seed_lo, self.seed_hi)))
    }

    /// Port of `XoroshiroPositionalRandomFactory#fromSeed(long)`.
    fn from_seed(&self, seed: i64) -> Box<dyn RandomSource> {
        Box::new(XoroshiroRandomSource::new_from_lo_hi(seed ^ self.seed_lo, seed ^ self.seed_hi))
    }

    /// Port of `XoroshiroPositionalRandomFactory#parityConfigString(StringBuilder)`.
    fn parity_config_string(&self) -> String {
        format!("seedLo: {}, seedHi: {}", self.seed_lo, self.seed_hi)
    }
}
