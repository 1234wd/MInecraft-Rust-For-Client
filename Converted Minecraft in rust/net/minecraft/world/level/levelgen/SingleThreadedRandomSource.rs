//! Port of: net/minecraft/world/level/levelgen/SingleThreadedRandomSource.java
//! Java class(es): net.minecraft.world.level.levelgen.SingleThreadedRandomSource
//! Status: VERIFIED
//!
//! Same 48-bit LCG as `LegacyRandomSource` without the `AtomicLong`. It is what
//! `RandomSource#createThreadLocalInstance(long)` returns, and therefore what
//! `Mth#wobble` uses -- so wobbled block positions are world-stable.

use crate::net::minecraft::util::RandomSource::RandomSource;
use crate::net::minecraft::world::level::levelgen::{
    BitRandomSource::{assert_bits_in_range, BitRandomSource},
    LegacyRandomSource::{LegacyRandomSource, INCREMENT, MODULUS_MASK, MULTIPLIER},
    MarsagliaPolarGaussian::MarsagliaPolarGaussian,
    PositionalRandomFactory::PositionalRandomFactory,
};

pub struct SingleThreadedRandomSource {
    seed: i64,
    /// Port of the `@Nullable MarsagliaPolarGaussian gaussianSource` field.
    ///
    /// Java allocates it lazily on the first `nextGaussian()`. We keep the `Option`
    /// so that the *allocation* behaviour matches, even though nothing observable
    /// depends on it.
    gaussian_source: Option<MarsagliaPolarGaussian>,
}

impl SingleThreadedRandomSource {
    /// Port of `SingleThreadedRandomSource#SingleThreadedRandomSource(long)`.
    pub fn new(seed: i64) -> Self {
        let mut me = Self { seed: 0, gaussian_source: None };
        me.set_seed(seed);
        me
    }
}

impl BitRandomSource for SingleThreadedRandomSource {
    /// Port of `SingleThreadedRandomSource#next(int)`.
    #[inline]
    fn next(&mut self, bits: i32) -> i32 {
        assert_bits_in_range(bits);
        self.seed = self.seed.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) & MODULUS_MASK;
        (self.seed >> (48 - bits)) as i32
    }
}

impl RandomSource for SingleThreadedRandomSource {
    fn fork(&mut self) -> Box<dyn RandomSource> {
        Box::new(SingleThreadedRandomSource::new(RandomSource::next_long(self)))
    }

    /// Port of `SingleThreadedRandomSource#forkPositional()`.
    ///
    /// NOTE the Java quirk: this one returns a LEGACY factory even though the
    /// receiver is a `SingleThreadedRandomSource`. Preserved as-is.
    fn fork_positional(&mut self) -> Box<dyn PositionalRandomFactory> {
        Box::new(LegacyRandomSource::legacy_positional_random_factory(RandomSource::next_long(self)))
    }

    /// Port of `SingleThreadedRandomSource#setSeed(long)`.
    ///
    /// Note there is no `gaussianSource.reset()` here when the gaussian is absent,
    /// and a plain `reset()` when it is present.
    #[inline]
    fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULTIPLIER) & MODULUS_MASK;
        if let Some(gaussian) = self.gaussian_source.as_mut() {
            gaussian.reset();
        }
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

    /// Port of `SingleThreadedRandomSource#nextGaussian()`, lazy allocation included.
    fn next_gaussian(&mut self) -> f64 {
        if self.gaussian_source.is_none() {
            self.gaussian_source = Some(MarsagliaPolarGaussian::new());
        }
        let mut gaussian = self.gaussian_source.take().expect("just created");
        let value = gaussian.next_gaussian(self);
        self.gaussian_source = Some(gaussian);
        value
    }

    fn as_bit_source(&mut self) -> Option<&mut dyn BitRandomSource> {
        Some(self)
    }
}