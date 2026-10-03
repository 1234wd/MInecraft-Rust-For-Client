//! Port of: net/minecraft/world/level/levelgen/Xoroshiro128PlusPlus.java
//! Java class(es): net.minecraft.world.level.levelgen.Xoroshiro128PlusPlus
//! Status: VERIFIED
//!
//! The raw xoroshiro128++ generator. Modern worlds (and every `RandomState`) draw
//! from this, so it is the most safety-critical arithmetic in the port.
//!
//! Java exposes `CODEC`, a DataFixerUpper codec. It is a static field built from
//! the DFU API, is never read by the simulation, and depends on a codec framework
//! that is not ported yet -- so it is deliberately absent. See
//! _porting/OPEN_QUESTIONS.md and MANIFEST.csv (notes = `codec-not-ported`).

use crate::net::minecraft::world::level::levelgen::RandomSupport::Seed128bit;

pub struct Xoroshiro128PlusPlus {
    seed_lo: i64,
    seed_hi: i64,
}

impl Xoroshiro128PlusPlus {
    /// Port of `Xoroshiro128PlusPlus#Xoroshiro128PlusPlus(RandomSupport.Seed128bit)`.
    pub fn from_seed(seed: Seed128bit) -> Self {
        Self::new(seed.seed_lo, seed.seed_hi)
    }

    /// Port of `Xoroshiro128PlusPlus#Xoroshiro128PlusPlus(long,long)`.
    ///
    /// The all-zero state is a fixed point of xoroshiro (it would emit zeros
    /// forever), so vanilla substitutes the golden/silver ratio pair. This is a
    /// gameplay mechanic, not a safety net.
    pub fn new(seed_lo: i64, seed_hi: i64) -> Self {
        let (mut lo, mut hi) = (seed_lo, seed_hi);
        if (lo | hi) == 0 {
            lo = -7046029254386353131i64; // GOLDEN_RATIO_64
            hi = 7640891576956012809i64; // SILVER_RATIO_64
        }
        Self { seed_lo: lo, seed_hi: hi }
    }

    /// Port of `Xoroshiro128PlusPlus#nextLong()`.
    ///
    /// `Long.rotateLeft(x, k)` is `x.rotate_left(k)` in Rust -- both mask the
    /// distance to 6 bits, so no extra work is needed.
    pub fn next_long(&mut self) -> i64 {
        let s0 = self.seed_lo;
        let s1 = self.seed_hi;
        let result = java_lang_rotate_left(s0.wrapping_add(s1), 17).wrapping_add(s0);
        let s1 = s1 ^ s0;
        self.seed_lo = java_lang_rotate_left(s0, 49) ^ s1 ^ s1.wrapping_shl(21);
        self.seed_hi = java_lang_rotate_left(s1, 28);
        result
    }

    /// Port of the (absent) `CODEC` state accessor, exposed for world serialization.
    #[inline]
    pub fn seed_lo(&self) -> i64 {
        self.seed_lo
    }

    /// Port of the (absent) `CODEC` state accessor, exposed for world serialization.
    #[inline]
    pub fn seed_hi(&self) -> i64 {
        self.seed_hi
    }
}

#[inline]
fn java_lang_rotate_left(v: i64, distance: u32) -> i64 {
    v.rotate_left(distance)
}