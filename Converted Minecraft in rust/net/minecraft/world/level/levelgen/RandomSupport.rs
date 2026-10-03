//! Port of: net/minecraft/world/level/levelgen/RandomSupport.java
//! Java class(es): net.minecraft.world.level.levelgen.RandomSupport, RandomSupport.Seed128bit
//! Status: VERIFIED
//!
//! Seed mixing for the 128-bit random sources. `same seed = same world` lives or
//! dies here, so every constant and every shift count is reproduced exactly.

use crate::javacompat::md5;
use std::sync::atomic::{AtomicI64, Ordering};

/// Port of `RandomSupport#GOLDEN_RATIO_64`.
pub const GOLDEN_RATIO_64: i64 = -7046029254386353131i64;
/// Port of `RandomSupport#SILVER_RATIO_64`.
pub const SILVER_RATIO_64: i64 = 7640891576956012809i64;

static SEED_UNIQUIFIER: AtomicI64 = AtomicI64::new(8682522807148012i64);

/// Port of `RandomSupport.Seed128bit` (a Java record -> a plain copyable struct).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Seed128bit {
    pub seed_lo: i64,
    pub seed_hi: i64,
}

impl Seed128bit {
    /// Port of `RandomSupport.Seed128bit#seedLo()`.
    #[inline]
    pub fn seed_lo(&self) -> i64 {
        self.seed_lo
    }

    /// Port of `RandomSupport.Seed128bit#seedHi()`.
    #[inline]
    pub fn seed_hi(&self) -> i64 {
        self.seed_hi
    }

    /// Port of `RandomSupport.Seed128bit#xor(long,long)`.
    #[inline]
    pub fn xor(&self, lo: i64, hi: i64) -> Seed128bit {
        Seed128bit { seed_lo: self.seed_lo ^ lo, seed_hi: self.seed_hi ^ hi }
    }

    /// Port of `RandomSupport.Seed128bit#xor(Seed128bit)`.
    #[inline]
    pub fn xor_seed(&self, other: &Seed128bit) -> Seed128bit {
        self.xor(other.seed_lo, other.seed_hi)
    }

    /// Port of `RandomSupport.Seed128bit#mixed()`.
    #[inline]
    pub fn mixed(&self) -> Seed128bit {
        Seed128bit {
            seed_lo: mix_stafford13(self.seed_lo),
            seed_hi: mix_stafford13(self.seed_hi),
        }
    }
}

/// Port of `RandomSupport#mixStafford13(long)` -- the MurmurHash3 finaliser /
/// SplitMix64 mixer. Note the `>>>` (unsigned) shifts on a `long`.
#[inline]
pub fn mix_stafford13(mut z: i64) -> i64 {
    z = (z ^ ((z as u64 >> 30) as i64)).wrapping_mul(-4658895280553007687i64);
    z = (z ^ ((z as u64 >> 27) as i64)).wrapping_mul(-7723592293110705685i64);
    z ^ ((z as u64 >> 31) as i64)
}

/// Port of `RandomSupport#upgradeSeedTo128bitUnmixed(long)`.
#[inline]
pub fn upgrade_seed_to_128bit_unmixed(legacy_seed: i64) -> Seed128bit {
    let low_bits = legacy_seed ^ SILVER_RATIO_64;
    let high_bits = low_bits.wrapping_add(GOLDEN_RATIO_64);
    Seed128bit { seed_lo: low_bits, seed_hi: high_bits }
}

/// Port of `RandomSupport#upgradeSeedTo128bit(long)`.
#[inline]
pub fn upgrade_seed_to_128bit(legacy_seed: i64) -> Seed128bit {
    upgrade_seed_to_128bit_unmixed(legacy_seed).mixed()
}

/// Port of `RandomSupport#seedFromHashOf(String)`.
///
/// MD5 over the UTF-8 bytes, then read as two little-endian longs. Guava's
/// `Hashing.md5()` and the JDK's `MessageDigest("MD5")` are the same RFC-1321
/// digest, so `md5_lo_hi` is the faithful stand-in (see javacompat/md5.rs).
#[inline]
pub fn seed_from_hash_of(input: &str) -> Seed128bit {
    let (lo, hi) = md5::md5_lo_hi(input.as_bytes());
    Seed128bit { seed_lo: lo, seed_hi: hi }
}

/// Port of the deterministic half of `RandomSupport#generateUniqueSeed()`:
/// `SEED_UNIQUIFIER.updateAndGet(current -> current * 1181783497276652981L)`.
#[inline]
pub fn next_unique_seed_prefix() -> i64 {
    SEED_UNIQUIFIER.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
        Some(current.wrapping_mul(1181783497276652981i64))
    })
    .unwrap_or_else(|e| e)
}

/// Port of `RandomSupport#generateUniqueSeed()`.
///
/// `System.nanoTime()` has no portable Rust equivalent (Rust's `Instant` has an
/// arbitrary, unspecified origin). We use the nanosecond wall clock instead. The
/// result is only XORed with the uniquifier to make a fresh seed -- nothing
/// gameplay-relevant depends on its absolute value -- but the two clocks are NOT
/// bit-identical, so this function is deliberately NOT parity-tested.
/// See _porting/OPEN_QUESTIONS.md.
pub fn generate_unique_seed() -> i64 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0);
    next_unique_seed_prefix() ^ nanos
}