//! Port of: net/minecraft/util/RandomSource.java
//! Java class(es): net.minecraft.util.RandomSource
//! Status: VERIFIED
//!
//! Rust mapping notes (see _porting/DESIGN_DECISIONS.md):
//! * Java `interface RandomSource` with `default` methods -> a Rust trait where the
//!   four defaults (`nextIntBetweenInclusive`, the two `triangle`s, `consumeCount`
//!   and `nextInt(origin,bound)`) keep their default bodies verbatim.
//! * Java's overload `nextInt()` / `nextInt(int bound)` cannot both exist in Rust,
//!   so the bounded one is `next_int_bounded`.
//! * `nextInt(bound <= 0)` throws in Java. Nothing in vanilla catches it -- it is a
//!   crash in Java too -- so we panic. Where Java *does* catch-and-continue, the Rust
//!   port must use `Result`; see DESIGN_DECISIONS.md (#exceptions).
//! * `System.nanoTime()` has no portable equivalent; `generate_unique_seed()` is the
//!   only affected entry point and is not parity-tested.

// Each Java class lives in its own module in the mirror, so the Rust items sit one
// level deeper (`.../RandomSource::RandomSource`). The `as` imports restore the Java
// spellings so the bodies below can be lined up with the original file.
use crate::net::minecraft::world::level::levelgen::{
    BitRandomSource::BitRandomSource, LegacyRandomSource::LegacyRandomSource,
    PositionalRandomFactory::PositionalRandomFactory, RandomSupport, SingleThreadedRandomSource::SingleThreadedRandomSource,
    ThreadSafeLegacyRandomSource::ThreadSafeLegacyRandomSource,
};

/// Port of `RandomSource#GAUSSIAN_SPREAD_FACTOR` (deprecated in Java, kept for parity).
pub const GAUSSIAN_SPREAD_FACTOR: f64 = 2.297;

pub trait RandomSource {
    /// Port of `RandomSource#fork()`.
    fn fork(&mut self) -> Box<dyn RandomSource>;

    /// Port of `RandomSource#forkPositional()`.
    fn fork_positional(&mut self) -> Box<dyn PositionalRandomFactory>;

    /// Port of `RandomSource#setSeed(long)`.
    fn set_seed(&mut self, seed: i64);

    /// Port of `RandomSource#nextInt()`.
    fn next_int(&mut self) -> i32;

    /// Port of `RandomSource#nextInt(int bound)`.
    fn next_int_bounded(&mut self, bound: i32) -> i32;

    /// Port of `RandomSource#nextLong()`.
    fn next_long(&mut self) -> i64;

    /// Port of `RandomSource#nextBoolean()`.
    fn next_boolean(&mut self) -> bool;

    /// Port of `RandomSource#nextFloat()`.
    fn next_float(&mut self) -> f32;

    /// Port of `RandomSource#nextDouble()`.
    fn next_double(&mut self) -> f64;

    /// Port of `RandomSource#nextGaussian()`.
    fn next_gaussian(&mut self) -> f64;

    /// Port of the `instanceof BitRandomSource` test in `WorldgenRandom#next(int)`.
    ///
    /// Java: `this.randomSource instanceof LegacyRandomSource l ? l.next(bits)
    /// : (int)(this.randomSource.nextLong() >>> 64 - bits)`. Rust has no
    /// `instanceof`, so bit-based sources answer `Some(self)` here and everything
    /// else takes the default `None`. Do not "simplify" this away.
    fn as_bit_source(&mut self) -> Option<&mut dyn BitRandomSource> {
        None
    }

    // ---- Java `default` methods, bodies unchanged ----------------------

    /// Port of `RandomSource#nextIntBetweenInclusive(int,int)` (default).
    #[inline]
    fn next_int_between_inclusive(&mut self, min: i32, max_inclusive: i32) -> i32 {
        self.next_int_bounded(max_inclusive - min + 1) + min
    }

    /// Port of `RandomSource#triangle(double,double)` (default).
    #[inline]
    fn triangle_f64(&mut self, mean: f64, spread: f64) -> f64 {
        mean + spread * (self.next_double() - self.next_double())
    }

    /// Port of `RandomSource#triangle(float,float)` (default).
    #[inline]
    fn triangle_f32(&mut self, mean: f32, spread: f32) -> f32 {
        mean + spread * (self.next_float() - self.next_float())
    }

    /// Port of `RandomSource#consumeCount(int)` (default).
    #[inline]
    fn consume_count(&mut self, rounds: i32) {
        for _ in 0..rounds {
            self.next_int();
        }
    }

    /// Port of `RandomSource#nextInt(int origin,int bound)` (default).
    #[inline]
    fn next_int_origin_bound(&mut self, origin: i32, bound: i32) -> i32 {
        if origin >= bound {
            panic!("bound - origin is non positive");
        }
        origin + self.next_int_bounded(bound - origin)
    }
}

/// Port of `RandomSource#create()` -- unseeded. Non-deterministic, not parity-tested.
pub fn create_unseeded() -> Box<dyn RandomSource> {
    create_with_seed(RandomSupport::generate_unique_seed())
}

/// Port of `RandomSource#create(long)` -> `new LegacyRandomSource(seed)`.
pub fn create_with_seed(seed: i64) -> Box<dyn RandomSource> {
    Box::new(LegacyRandomSource::new(seed))
}

/// Port of `RandomSource#createThreadSafe()` (deprecated in Java).
pub fn create_thread_safe() -> Box<dyn RandomSource> {
    Box::new(ThreadSafeLegacyRandomSource::new(RandomSupport::generate_unique_seed()))
}

/// Port of `RandomSource#createThreadLocalInstance()` -- unseeded, non-deterministic.
pub fn create_thread_local_instance_unseeded() -> Box<dyn RandomSource> {
    Box::new(SingleThreadedRandomSource::new(nano_seed()))
}

/// Port of `RandomSource#createThreadLocalInstance(long)` -> `new SingleThreadedRandomSource(seed)`.
pub fn create_thread_local_instance(seed: i64) -> Box<dyn RandomSource> {
    Box::new(SingleThreadedRandomSource::new(seed))
}

fn nano_seed() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}