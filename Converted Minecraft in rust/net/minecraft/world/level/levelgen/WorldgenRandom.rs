//! Port of: net/minecraft/world/level/levelgen/WorldgenRandom.java
//! Java class(es): net.minecraft.world.level.levelgen.WorldgenRandom,
//!                  WorldgenRandom.Algorithm
//! Status: VERIFIED
//!
//! `WorldgenRandom` extends `LegacyRandomSource` in Java but throws away the
//! inherited LCG state: `next(int)` delegates to a wrapped source. So it is NOT
//! inheritance in Rust -- it is a struct holding a `Box<dyn RandomSource>` and a
//! call counter. The counter matters: it is read back by feature code to decide
//! how much randomness a feature has burned, so it must be incremented on every
//! `next(bits)` call, in the same place as Java.
//!
//! The `set*Seed` family is what makes worldgen reproducible: given the world seed
//! and a chunk/feature coordinate, every feature gets an independent, order-
//! independent stream. Preserve the exact arithmetic and the exact call ORDER --
//! `setDecorationSeed` draws two longs *before* xoring, and so on.


use crate::net::minecraft::util::RandomSource::{create_thread_local_instance, RandomSource};
use crate::net::minecraft::world::level::levelgen::{
    BitRandomSource::{assert_bits_in_range, BitRandomSource},
    LegacyRandomSource::LegacyRandomSource,
    PositionalRandomFactory::PositionalRandomFactory,
    XoroshiroRandomSource::XoroshiroRandomSource,
};

pub struct WorldgenRandom {
    random_source: Box<dyn RandomSource>,
    count: i32,
}

impl WorldgenRandom {
    /// Port of `WorldgenRandom#WorldgenRandom(RandomSource)`.
    ///
    /// Java calls `super(0L)`, seeding the (unused) inherited LCG. That state is
    /// never read, so it has no counterpart here.
    pub fn new(random_source: Box<dyn RandomSource>) -> Self {
        Self { random_source, count: 0 }
    }

    /// Port of `WorldgenRandom#getCount()`.
    #[inline]
    pub fn get_count(&self) -> i32 {
        self.count
    }

    /// Take the wrapped source back out, leaving a fresh placeholder behind.
    ///
    /// Exists for the parity tests, which need to build a `WorldgenRandom` per seed
    /// while reusing one long-lived source stream -- the oracle nests its loops that
    /// way. Not part of vanilla's API.
    #[doc(hidden)]
    pub fn take_random_source_for_test(&mut self) -> Box<dyn RandomSource> {
        std::mem::replace(&mut self.random_source, Box::new(LegacyRandomSource::new(0)))
    }

    /// Port of `WorldgenRandom#setDecorationSeed(long,int,int)`.
    ///
    /// Draws two longs with `| 1L` (forcing odd multipliers) BEFORE the xor.
    ///
    /// Operator precedence is load-bearing here:
    /// ```java
    /// long result = chunkX * xScale + chunkZ * zScale ^ seed;
    /// ```
    /// `*` and `+` bind tighter than `^`, so this is
    /// `((chunkX * xScale) + (chunkZ * zScale)) ^ seed` -- an ADD then an XOR --
    /// NOT a three-way xor. Contrast `setLargeFeatureSeed` just below, which really
    /// is `chunkX * xScale ^ chunkZ * zScale ^ seed`. Confusing the two desyncs
    /// decoration placement (flowers, grass, snow layers) against everything else.
    pub fn set_decoration_seed(&mut self, seed: i64, chunk_x: i32, chunk_z: i32) -> i64 {
        self.set_seed(seed);
        // `this.nextLong()` in Java resolves to WorldgenRandom's OWN inherited
        // BitRandomSource#nextLong, which goes through this class's `next(int)`
        // override -- so it advances `count` by 2 per call (nextLong = two
        // next(32) calls). Calling the wrapped source directly would skip the
        // counter and desync `getCount()`, which feature code reads to decide how
        // much randomness it has burned.
        let x_scale = BitRandomSource::next_long(self) | 1;
        let z_scale = BitRandomSource::next_long(self) | 1;
        let result = (chunk_x as i64)
            .wrapping_mul(x_scale)
            .wrapping_add((chunk_z as i64).wrapping_mul(z_scale))
            ^ seed;
        self.set_seed(result);
        result
    }

    /// Port of `WorldgenRandom#setFeatureSeed(long,int,int)`.
    pub fn set_feature_seed(&mut self, seed: i64, index: i32, step: i32) {
        let result = seed.wrapping_add(index as i64).wrapping_add(10000i64.wrapping_mul(step as i64));
        self.set_seed(result);
    }

    /// Port of `WorldgenRandom#setLargeFeatureSeed(long,int,int)`.
    /// Port of `WorldgenRandom#setLargeFeatureSeed(long,int,int)`.
    ///
    /// This one really IS a three-way xor -- contrast `set_decoration_seed`, which
    /// is an add followed by an xor. Same `this.nextLong()` routing, so `count`
    /// advances here too.
    pub fn set_large_feature_seed(&mut self, seed: i64, chunk_x: i32, chunk_z: i32) {
        self.set_seed(seed);
        let x_scale = BitRandomSource::next_long(self);
        let z_scale = BitRandomSource::next_long(self);
        let result = (chunk_x as i64).wrapping_mul(x_scale) ^ (chunk_z as i64).wrapping_mul(z_scale) ^ seed;
        self.set_seed(result);
    }

    /// Port of `WorldgenRandom#setLargeFeatureWithSalt(long,int,int,int)`.
    pub fn set_large_feature_with_salt(&mut self, seed: i64, x: i32, z: i32, blend: i32) {
        let result = (x as i64)
            .wrapping_mul(341873128712i64)
            .wrapping_add((z as i64).wrapping_mul(132897987541i64))
            .wrapping_add(seed)
            .wrapping_add(blend as i64);
        self.set_seed(result);
    }

    /// Port of `WorldgenRandom#seedSlimeChunk(int,int,long,long)`.
    ///
    /// Note Java's `*` here is `int * int` on the left-hand terms, promoted to
    /// `long` only when added to the `long` seed. `x * x * 4987142` therefore
    /// overflows as an `int` first for large `x`. Reproduced with `wrapping_mul`
    /// on `i32` and one widening step at the end, exactly as the JVM promotes.
    pub fn seed_slime_chunk(x: i32, z: i32, seed: i64, salt: i64) -> Box<dyn RandomSource> {
        // Java:
        //   RandomSource.createThreadLocalInstance(
        //       seed + x * x * 4987142 + x * 5947611 + z * z * 4392871L + z * 389711 ^ salt);
        //
        // Two things are load-bearing and easy to get wrong:
        //
        // 1. PRECEDENCE. `*` and `+` bind tighter than `^`, so the `^ salt` applies
        //    to the whole sum, not just the last term.
        //
        // 2. MIXED WIDTHS. `x * x * 4987142`, `x * 5947611` and `z * 389711` are all
        //    `int` arithmetic and wrap at 32 bits. `z * z * 4392871L` is the odd one
        //    out: the trailing `L` makes it a `long` multiply. Java then sums
        //    left-to-right with a promotion at each step. Doing it all in i32 (or
        //    all in i64) gives a different slime-chunk decision near chunk borders.
        let step1 = seed.wrapping_add(x.wrapping_mul(x).wrapping_mul(4987142i32) as i64);
        let step2 = step1.wrapping_add(x.wrapping_mul(5947611i32) as i64);
        let step3 = step2.wrapping_add((z as i64).wrapping_mul(z as i64).wrapping_mul(4392871i64));
        let step4 = step3.wrapping_add(z.wrapping_mul(389711i32) as i64);
        create_thread_local_instance(step4 ^ salt)
    }
}

impl BitRandomSource for WorldgenRandom {
    /// Port of `WorldgenRandom#next(int)`.
    ///
    /// ```java
    /// this.count++;
    /// return this.randomSource instanceof LegacyRandomSource l ? l.next(bits)
    ///      : (int)(this.randomSource.nextLong() >>> 64 - bits);
    /// ```
    /// The borrow of `as_bit_source()` must end before the fallback branch touches
    /// `self.random_source`, hence the intermediate binding.
    #[inline]
    fn next(&mut self, bits: i32) -> i32 {
        self.count += 1;
        let via_bits = self.random_source.as_bit_source().map(|b| b.next(bits));
        match via_bits {
            Some(value) => value,
            None => ((self.random_source.next_long() as u64) >> (64 - bits)) as i32,
        }
    }
}

impl RandomSource for WorldgenRandom {
    /// Port of `WorldgenRandom#fork()`.
    fn fork(&mut self) -> Box<dyn RandomSource> {
        self.random_source.fork()
    }

    /// Port of `WorldgenRandom#forkPositional()`.
    fn fork_positional(&mut self) -> Box<dyn PositionalRandomFactory> {
        self.random_source.fork_positional()
    }

    /// Port of `WorldgenRandom#setSeed(long)`.
    #[inline]
    fn set_seed(&mut self, seed: i64) {
        self.random_source.set_seed(seed);
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

    fn next_gaussian(&mut self) -> f64 {
        self.random_source.next_gaussian()
    }
}

/// Port of `WorldgenRandom.Algorithm` -- a Java enum holding a constructor
/// reference. Rust gets the same closed set plus an explicit `new_instance`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    /// Port of `Algorithm#LEGACY`.
    Legacy,
    /// Port of `Algorithm#XOROSHIRO`.
    Xoroshiro,
}

impl Algorithm {
    /// Port of `Algorithm#values()`: the declaration order is load-bearing.
    pub const VALUES: [Algorithm; 2] = [Algorithm::Legacy, Algorithm::Xoroshiro];

    /// Port of `Algorithm#ordinal()`.
    pub fn ordinal(self) -> i32 {
        match self {
            Algorithm::Legacy => 0,
            Algorithm::Xoroshiro => 1,
        }
    }

    /// Port of `Algorithm#newInstance(long)`.
    pub fn new_instance(self, seed: i64) -> Box<dyn RandomSource> {
        match self {
            Algorithm::Legacy => Box::new(LegacyRandomSource::new(seed)),
            Algorithm::Xoroshiro => Box::new(XoroshiroRandomSource::new(seed)),
        }
    }
}
