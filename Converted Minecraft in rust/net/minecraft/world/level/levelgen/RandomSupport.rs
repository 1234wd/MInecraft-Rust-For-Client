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

/// Port of the initial value of `RandomSupport#SEED_UNIQUIFIER`.
const SEED_UNIQUIFIER_INITIAL: i64 = 8682522807148012i64;

/// Port of the multiplier in `RandomSupport#generateUniqueSeed()`.
///
/// Odd, so multiplication by it is a bijection mod 2^64: the sequence never collapses
/// and never reaches the `0` fixed point from a non-zero start.
const SEED_UNIQUIFIER_MULTIPLIER: i64 = 1181783497276652981i64;

static SEED_UNIQUIFIER: AtomicI64 = AtomicI64::new(SEED_UNIQUIFIER_INITIAL);

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
///
/// `updateAndGet` returns the **new** (post-multiply) value, not the previous one, and
/// that is why this is a hand-rolled CAS loop rather than the obvious `fetch_update`:
/// Rust's `fetch_update` returns `Result<previous, previous>`, so the tidy spelling
/// returned the value Java had *before* the multiply. Every `RandomSource` built through
/// `RandomSource::create()` then started one step behind vanilla's stream.
#[inline]
pub fn next_unique_seed_prefix() -> i64 {
    next_seed_prefix_of(&SEED_UNIQUIFIER)
}

/// The body of [`next_unique_seed_prefix`], over an explicit atomic.
///
/// Split out so the CAS loop is testable on a LOCAL `AtomicI64`. A test that wrote to the
/// real `SEED_UNIQUIFIER` would race every other test in the binary that builds a random
/// source -- `RandomSource::create()` calls `generate_unique_seed()` -- and the symptom
/// would be a wrong number in an unrelated test rather than a failure here.
#[inline]
fn next_seed_prefix_of(uniquifier: &AtomicI64) -> i64 {
    // `updateAndGet` is an unconditional RMW, so the initial read is only a starting
    // guess: a stale value fails the compare-exchange and the loop retries with the
    // actual one, which is also how a spurious `_weak` failure is absorbed. The
    // successful RMW is `SeqCst`, which is the ordering Java's `AtomicLong` gives.
    let mut current = uniquifier.load(Ordering::Relaxed);
    loop {
        let next = current.wrapping_mul(SEED_UNIQUIFIER_MULTIPLIER);
        match uniquifier.compare_exchange_weak(current, next, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => return next,
            Err(actual) => current = actual,
        }
    }
}

/// Port of `RandomSupport#generateUniqueSeed()`:
/// `SEED_UNIQUIFIER.updateAndGet(c -> c * 1181783497276652981L) ^ System.nanoTime()`.
///
/// Routed through [`crate::javacompat::entropy::nano_time`] (`DESIGN_DECISIONS.md`
/// `#entropy-injection`: no ported file may read a clock directly) so tests can pin the
/// clock with [`crate::javacompat::entropy::with_override`]. The result is deliberately
/// NOT parity-tested against vanilla: Java's `nanoTime` origin is JVM-chosen, so the two
/// clocks are not bit-identical, and the whole point of the function is to return
/// something fresh. See `_porting/OPEN_QUESTIONS.md` #1.
pub fn generate_unique_seed() -> i64 {
    generate_unique_seed_of(&SEED_UNIQUIFIER)
}

/// The body of [`generate_unique_seed`], over an explicit atomic.
///
/// Same reason as [`next_seed_prefix_of`]: it makes the XOR composition checkable without
/// mutating crate-global state. Java evaluates `updateAndGet(..) ^ System.nanoTime()`
/// left to right, and so does Rust, so the prefix is drawn before the clock is read.
#[inline]
fn generate_unique_seed_of(uniquifier: &AtomicI64) -> i64 {
    next_seed_prefix_of(uniquifier) ^ crate::javacompat::entropy::nano_time()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::javacompat::entropy::{self, TEST_CLOCK_LOCK};

    /// Oracle: `AtomicLong(8682522807148012L).updateAndGet(c -> c * 1181783497276652981L)`
    /// on JDK 21, four successive calls. Checked on a local atomic, so this test is
    /// order-independent and cannot disturb the real `SEED_UNIQUIFIER`.
    #[test]
    fn unique_seed_prefix_returns_the_updated_value_like_update_and_get() {
        const ORACLE: [i64; 4] = [
            3447679086515839964,
            -2942033378085796212,
            9105146733113736444,
            -5237447342009718228,
        ];

        let uniquifier = AtomicI64::new(SEED_UNIQUIFIER_INITIAL);
        for (step, want) in ORACLE.iter().enumerate() {
            let got = next_seed_prefix_of(&uniquifier);
            assert_eq!(got, *want, "step {}", step + 1);
            // `updateAndGet` returns exactly what it stored -- never the previous value,
            // which is what `fetch_update` would have handed back.
            let stored = uniquifier.load(Ordering::SeqCst);
            assert_eq!(stored, got, "step {} store", step + 1);
        }
        assert_ne!(
            ORACLE[0], SEED_UNIQUIFIER_INITIAL,
            "updateAndGet must return the updated value, not the previous one"
        );
    }

    /// The wrapping edge cases, straight off the JVM. `long` multiplication wraps; it does
    /// not saturate, and it must not panic in a Rust debug build, which is why every
    /// multiply here is `wrapping_mul`.
    #[test]
    fn unique_seed_prefix_wraps_like_a_java_long() {
        for (start, want) in [
            // Java: 0L * M == 0. The one fixed point, unreachable from a non-zero start
            // because the multiplier is odd.
            (0i64, 0i64),
            // Java: Long.MIN_VALUE * M == Long.MIN_VALUE (2^63 * odd == 2^63 mod 2^64).
            (i64::MIN, i64::MIN),
            (i64::MAX, 8041588539578122827),
            (-1, -1181783497276652981),
        ] {
            let uniquifier = AtomicI64::new(start);
            assert_eq!(next_seed_prefix_of(&uniquifier), want, "start {start}");
        }
    }

    /// `updateAndGet` is atomic, so N threads x M calls must yield exactly the N*M
    /// successive multiples -- none lost to a read-modify-write race, none returned twice.
    ///
    /// Kept because the replacement is a hand-rolled loop, where a mistake in the retry
    /// path (reusing `current` instead of the observed value, say) shows up only under
    /// contention and would otherwise reach the game as duplicate world seeds.
    #[test]
    fn unique_seed_prefix_loses_no_update_under_contention() {
        const THREADS: usize = 8;
        const PER_THREAD: usize = 500;

        let uniquifier = std::sync::Arc::new(AtomicI64::new(SEED_UNIQUIFIER_INITIAL));
        let handles: Vec<_> = (0..THREADS)
            .map(|_| {
                let u = std::sync::Arc::clone(&uniquifier);
                std::thread::spawn(move || {
                    let mut drawn = Vec::with_capacity(PER_THREAD);
                    for _ in 0..PER_THREAD {
                        drawn.push(next_seed_prefix_of(&u));
                    }
                    drawn
                })
            })
            .collect();
        let mut got: Vec<i64> =
            handles.into_iter().flat_map(|h| h.join().expect("worker panicked")).collect();

        // The sequential oracle for the same number of steps.
        let mut want = Vec::with_capacity(THREADS * PER_THREAD);
        let mut v = SEED_UNIQUIFIER_INITIAL;
        for _ in 0..THREADS * PER_THREAD {
            v = v.wrapping_mul(SEED_UNIQUIFIER_MULTIPLIER);
            want.push(v);
        }

        assert_eq!(
            uniquifier.load(Ordering::SeqCst),
            v,
            "final state skipped or repeated a step"
        );
        got.sort_unstable();
        want.sort_unstable();
        assert_eq!(got, want, "a returned value was duplicated or lost");
    }

    /// `generateUniqueSeed()` is `prefix ^ nanoTime`, in that order.
    #[test]
    fn generate_unique_seed_xors_the_prefix_with_nano_time() {
        // The clock override is process-global; see `entropy::TEST_CLOCK_LOCK`.
        let _clock = TEST_CLOCK_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        const NANOS: i64 = 0x0123_4567_89ab_cdef;

        let uniquifier = AtomicI64::new(SEED_UNIQUIFIER_INITIAL);
        let seed = entropy::with_override(NANOS, || generate_unique_seed_of(&uniquifier));
        assert_eq!(seed, 3447679086515839964i64 ^ NANOS);

        // A second draw advances the prefix, so the same clock gives a different seed --
        // i.e. the uniquifier really is in the result and is not being re-read.
        let again = entropy::with_override(NANOS, || generate_unique_seed_of(&uniquifier));
        assert_eq!(again, -2942033378085796212i64 ^ NANOS);
        assert_ne!(seed, again);
    }

    /// The public, crate-global entry point advances `SEED_UNIQUIFIER` rather than
    /// returning the clock alone. Deliberately asserts only the inequality: this test
    /// shares the static with anything else in the binary that creates a random source,
    /// so its absolute position in the sequence is not its business.
    #[test]
    fn generate_unique_seed_advances_the_global_uniquifier() {
        let _clock = TEST_CLOCK_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        const NANOS: i64 = 0x7fff_ffff_ffff_fff0;

        let (first, second) =
            entropy::with_override(NANOS, || (generate_unique_seed(), generate_unique_seed()));
        assert_ne!(
            first, second,
            "same pinned clock twice gave the same seed, so the uniquifier did not advance"
        );
        assert_ne!(first, NANOS, "seed is the bare clock reading");
    }
}
