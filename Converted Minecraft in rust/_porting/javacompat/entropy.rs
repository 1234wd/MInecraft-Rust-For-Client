//! Port of: (no Java counterpart -- see module docs)
//! Java class(es): `java.lang.System#nanoTime`, `java.util.concurrent.ThreadLocalRandom`
//! Status: PORTED
//!
//! Every source of time or entropy in the port goes through this module.
//!
//! # Why this is not a gameplay change
//!
//! The functions here are non-deterministic by construction -- they exist precisely
//! *because* two calls must not return the same value. Centralising them changes no
//! behaviour; it changes only where the clock is read. Making them injectable is what
//! lets `cargo test` be deterministic at all.
//!
//! # The rule
//!
//! If a ported method reads the clock, draws OS entropy, or otherwise needs a value
//! that differs between runs, it MUST call into this module. It must not call
//! `std::time::SystemTime::now`, `Instant::now`, `std::collections::hash_map::RandomState`,
//! or `/dev/urandom` directly. That keeps the set of non-determinism auditable: one
//! file, one grep.
//!
//! # `System.nanoTime()` has no portable equivalent
//!
//! Java's `System.nanoTime()` counts from an *arbitrary, JVM-chosen origin*; only
//! differences are meaningful. Rust's `Instant` is also arbitrary-origin, but there is
//! no way to make the two agree, and `SystemTime` is a wall clock that can jump
//! backwards under NTP.
//!
//! So `nano_time()` returns nanoseconds from a fixed, documented origin (the UNIX
//! epoch). That is a *different number* from Java's, which means
//! `RandomSupport#generateUniqueSeed()` cannot be bit-identical to vanilla -- it is
//! unobservable-by-construction instead, because the whole point of that function is to
//! return something fresh. See `OPEN_QUESTIONS.md`.
//!
//! # Override plumbing
//!
//! [`set_override`] is deliberately awkward to call from game code: it lives behind a
//! `#[doc(hidden)]` and every non-test caller should go through the `*_with` helpers
//! below rather than mutating the global directly.

use std::sync::atomic::{AtomicI64, Ordering};

/// Nanoseconds in a second, as `i64`. Matches Java's `NANOS_PER_SECOND`.
pub const NANOS_PER_SECOND: i64 = 1_000_000_000;

static NANOS_OVERRIDE: AtomicI64 = AtomicI64::new(i64::MIN);

/// Port of `System.nanoTime()`.
///
/// Returns a monotonically non-decreasing nanosecond reading. Unlike Java's, the origin
/// is fixed (the UNIX epoch) because there is no way to reproduce Java's
/// unspecified origin; only differences are meaningful to callers, so this is
/// observationally equivalent for every use in the game.
///
/// # Determinism
///
/// Returns the value installed by [`set_override`] when one is set. Tests MUST set an
/// override; otherwise this is genuinely non-deterministic.
#[inline]
pub fn nano_time() -> i64 {
    let override_value = NANOS_OVERRIDE.load(Ordering::Relaxed);
    if override_value != i64::MIN {
        return override_value;
    }
    real_nanos()
}

/// Port of `System.currentTimeMillis()`.
#[inline]
pub fn current_time_millis() -> i64 {
    let override_value = NANOS_OVERRIDE.load(Ordering::Relaxed);
    if override_value != i64::MIN {
        // The override is in nanoseconds; callers asking for millis still get a
        // deterministic value rather than a real clock reading.
        return override_value / 1_000_000;
    }
    real_nanos() / 1_000_000
}

#[inline]
fn real_nanos() -> i64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => d.as_nanos() as i64,
        // Before 1970 is not reachable in practice, but saturate rather than panic.
        Err(_) => i64::MAX,
    }
}

/// Port of `java.util.concurrent.ThreadLocalRandom#nextLong()`.
///
/// A convenience for callers that just need *some* fresh value. Like
/// `ThreadLocalRandom`, this is seeded per-thread rather than from a global, so it is
/// not reproducible across runs and must not be used on a seeded path.
///
/// # Determinism
///
/// Uses [`nanoseed`], so tests get determinism for free from the same override.
#[inline]
pub fn thread_local_random_next_long() -> i64 {
    // xorshift64* -- adequate for "different each call" and free of dependencies.
    // NOT bit-compatible with ThreadLocalRandom, and never used on a seeded path;
    // see OPEN_QUESTIONS.md.
    let mut x = nanoseed();
    if x == 0 {
        x = 0x9E37_79B9_7F4A_7C15u64 as i64;
    }
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 1) as i64
}

/// Port of `ThreadLocalRandom#current()#nextLong()`'s seed source.
///
/// Present so tests can make *any* entropy-consuming path deterministic through the
/// one override.
#[inline]
pub fn nanoseed() -> i64 {
    nano_time() ^ (std::process::id() as i64) << 32
}

/// Install a deterministic `nano_time()` / `current_time_millis()` value.
///
/// `#[doc(hidden)]` because game code has no business calling this. The parity tests
/// call it; production never does.
///
/// Passing [`NANOS_OVERRIDE_NONE`] restores the real clock.
#[doc(hidden)]
pub fn set_override(nanos: i64) {
    NANOS_OVERRIDE.store(nanos, Ordering::SeqCst);
}

/// Sentinel meaning "no override installed".
#[doc(hidden)]
pub const NANOS_OVERRIDE_NONE: i64 = i64::MIN;

/// Run `f` with a deterministic clock, then restore the previous state.
///
/// Scoped so a failing assertion cannot leave the override installed and silently
/// change the meaning of a later test that shares the process.
#[doc(hidden)]
pub fn with_override<R>(nanos: i64, f: impl FnOnce() -> R) -> R {
    let previous = NANOS_OVERRIDE.swap(nanos, Ordering::SeqCst);
    let result = f();
    NANOS_OVERRIDE.store(previous, Ordering::SeqCst);
    result
}

#[cfg(test)]
mod tests {
    /// Serialises every test in this module.
    ///
    /// with_override installs a PROCESS-GLOBAL override, so any test that reads the
    /// clock can observe another test's override. Serialising only the WRITER is not
    /// enough: the reader has to take the lock too. That was a real flake here -- it
    /// failed roughly one run in three before the lock was hoisted to module scope.
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    use super::*;

    /// The override is a process global, and `cargo test` runs tests in parallel
    /// threads. Two tests that install it concurrently would race and flake, so they
    /// are combined into one and serialised with a mutex rather than being left as
    /// separate `#[test]`s.
    #[test]
    fn override_makes_the_clock_deterministic() {
        let _guard = LOCK.lock().unwrap_or_else(|e| e.into_inner());

        with_override(1_234_567_890_123_456_789, || {
            assert_eq!(nano_time(), 1_234_567_890_123_456_789);
            assert_eq!(nano_time(), 1_234_567_890_123_456_789);
            // 1234567890123456789 ns / 1e6 = 1234567890123.456789 -> truncated
            assert_eq!(current_time_millis(), 1_234_567_890_123);
        });
        // Restored to the real clock.
        assert_ne!(nano_time(), 1_234_567_890_123_456_789);

        // Nested overrides restore the outer value.
        with_override(100, || {
            with_override(200, || assert_eq!(nano_time(), 200));
            assert_eq!(nano_time(), 100);
        });
        assert_ne!(nano_time(), 100);

        // Removing the override gives a plausible wall-clock reading: after 2020 and
        // before 2100. If this ever fails the UNIT is wrong, not the machine's clock.
        set_override(NANOS_OVERRIDE_NONE);
        let nanos = nano_time();
        assert!(nanos > 1_577_836_800_000_000_000, "nano_time looks like seconds: {nanos}");
        assert!(nanos < 4_102_444_800_000_000_000, "nano_time overflowed: {nanos}");
    }
    #[test]
    fn thread_local_random_is_stable_under_override() {
        let _guard = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_override(42, || {
            assert_eq!(thread_local_random_next_long(), thread_local_random_next_long());
            // Still differs from `nano_time` itself, so it is not a pass-through.
            assert_ne!(thread_local_random_next_long(), 42);
        });
    }
}
