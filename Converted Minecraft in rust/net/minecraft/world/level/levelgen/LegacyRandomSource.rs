//! Port of: net/minecraft/world/level/levelgen/LegacyRandomSource.java
//! Java class(es): net.minecraft.world.level.levelgen.LegacyRandomSource,
//!                  LegacyRandomSource.LegacyPositionalRandomFactory
//! Status: VERIFIED
//!
//! The 48-bit LCG every legacy world seed flows through. "Same seed = same world"
//! is literally this file, so the mask, the increment and the shift count are all
//! reproduced verbatim.
//!
//! Java guards the state with an `AtomicLong` and throws if two threads touch it.
//! Rust's `&mut self` gives the same guarantee for free, so the CAS and
//! `ThreadingDetector` machinery have no counterpart -- see
//! _porting/DESIGN_DECISIONS.md (#threading-detector).

use crate::javacompat::java_lang;
use crate::net::minecraft::util::{Mth::Mth, RandomSource::RandomSource};
use crate::net::minecraft::world::level::levelgen::{
    BitRandomSource::BitRandomSource, MarsagliaPolarGaussian::MarsagliaPolarGaussian,
    PositionalRandomFactory::PositionalRandomFactory,
};

/// Port of `LegacyRandomSource#MODULUS_BITS`.
pub const MODULUS_BITS: i32 = 48;
/// Port of `LegacyRandomSource#MODULUS_MASK` (`281474976710655L`).
pub const MODULUS_MASK: i64 = 281474976710655i64;
/// Port of `LegacyRandomSource#MULTIPLIER` (`25214903917L` == `0x5DEECE66D`).
pub const MULTIPLIER: i64 = 25214903917i64;
/// Port of `LegacyRandomSource#INCREMENT` (`11L`).
pub const INCREMENT: i64 = 11i64;

pub struct LegacyRandomSource {
    seed: i64,
    gaussian_source: MarsagliaPolarGaussian,
}

impl LegacyRandomSource {
    /// Port of `LegacyRandomSource#LegacyRandomSource(long)`.
    pub fn new(seed: i64) -> Self {
        Self { seed: 0, gaussian_source: MarsagliaPolarGaussian::new() }.with_seed(seed)
    }

    /// Shared body of `setSeed` for the constructor and `RandomSource#setSeed`.
    #[inline]
    fn with_seed(mut self, seed: i64) -> Self {
        self.set_seed(seed);
        self
    }

    /// Port of `LegacyRandomSource#LegacyPositionalRandomFactory`.
    pub fn legacy_positional_random_factory(seed: i64) -> LegacyPositionalRandomFactory {
        LegacyPositionalRandomFactory { seed }
    }
}

impl BitRandomSource for LegacyRandomSource {
    /// Port of `LegacyRandomSource#next(int)`.
    ///
    /// Java writes `newSeed >> 48 - bits` (a SIGNED shift). `newSeed` is masked to
    /// 48 bits so it is never negative and the signed/unsigned shifts agree; the
    /// mask has to be applied BEFORE the shift or results differ.
    #[inline]
    fn next(&mut self, bits: i32) -> i32 {
        crate::net::minecraft::world::level::levelgen::BitRandomSource::assert_bits_in_range(bits);
        self.seed = self.seed.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) & MODULUS_MASK;
        (self.seed >> (MODULUS_BITS - bits)) as i32
    }
}

impl RandomSource for LegacyRandomSource {
    fn fork(&mut self) -> Box<dyn RandomSource> {
        Box::new(LegacyRandomSource::new(RandomSource::next_long(self)))
    }

    fn fork_positional(&mut self) -> Box<dyn PositionalRandomFactory> {
        Box::new(LegacyRandomSource::legacy_positional_random_factory(RandomSource::next_long(self)))
    }

    /// Port of `LegacyRandomSource#setSeed(long)`.
    #[inline]
    fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULTIPLIER) & MODULUS_MASK;
        self.gaussian_source.reset();
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

    /// Port of `LegacyRandomSource#nextGaussian()`.
    ///
    /// The cached spare value is moved into a local so that `self` can be borrowed
    /// mutably at the same time (the Java object graph was self-referential).
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

/// Port of `LegacyRandomSource.LegacyPositionalRandomFactory`.
pub struct LegacyPositionalRandomFactory {
    seed: i64,
}

impl LegacyPositionalRandomFactory {
    /// Port of `LegacyPositionalRandomFactory#LegacyPositionalRandomFactory(long)`.
    pub fn new(seed: i64) -> Self {
        Self { seed }
    }
}

impl PositionalRandomFactory for LegacyPositionalRandomFactory {
    /// Port of `LegacyPositionalRandomFactory#at(int,int,int)`.
    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource> {
        let positional_seed = Mth::get_seed(x, y, z);
        Box::new(LegacyRandomSource::new(positional_seed ^ self.seed))
    }

    /// Port of `LegacyPositionalRandomFactory#fromHashOf(String)`.
    ///
    /// Java: `int positionalSeed = name.hashCode(); ... positionalSeed ^ this.seed`.
    /// The `^` promotes the `int` to `long` by SIGN EXTENSION -- Rust's `as i64`
    /// does the same, so the two agree.
    fn from_hash_of(&self, name: &str) -> Box<dyn RandomSource> {
        let positional_seed = java_lang::string_hash_code(name);
        Box::new(LegacyRandomSource::new((positional_seed as i64) ^ self.seed))
    }

    /// Port of `LegacyPositionalRandomFactory#fromSeed(long)`.
    fn from_seed(&self, seed: i64) -> Box<dyn RandomSource> {
        Box::new(LegacyRandomSource::new(seed))
    }

    /// Port of `LegacyPositionalRandomFactory#parityConfigString(StringBuilder)`.
    fn parity_config_string(&self) -> String {
        format!("LegacyPositionalRandomFactory{{{}}}", self.seed)
    }
}