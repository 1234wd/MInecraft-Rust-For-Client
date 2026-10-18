//! Port of: net/minecraft/world/Stopwatch.java
//! Java class(es): net.minecraft.world.Stopwatch
//! Status: VERIFIED
//!
//! Session 22. `public record Stopwatch(long creationTime, long accumulatedElapsedTime)` with a
//! one-argument constructor and two readers.
//!
//! # Arithmetic
//!
//! `elapsedMilliseconds(currentTime)` is `accumulatedElapsedTime + (currentTime - creationTime)`
//! in `long`, so both the subtraction and the addition WRAP on overflow -- `wrapping_*` here.
//! `elapsedSeconds` converts that `long` to `double` FIRST (round-to-nearest, which is what Rust's
//! `as f64` does too) and only then divides by `1000.0`; it is not `ms / 1000` in integers.
//!
//! # Record members
//!
//! `equals` is derived, `hashCode` is [`Stopwatch::java_hash_code`] (the record fold over
//! `Long.hashCode` of each component), `toString` is
//! `Stopwatch[creationTime=<n>, accumulatedElapsedTime=<n>]`.
//!
//! Verified by the `REC`, `RECEQ` and `WATCH` groups of `small_probe.txt` (`tests/session22.rs`):
//! both readers over a 7x7x7 grid of creation/accumulated/current times that includes
//! `Long.MIN_VALUE` and `Long.MAX_VALUE`, so every wrap is exercised.

use crate::javacompat::java_object;

/// Port of `net.minecraft.world.Stopwatch`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stopwatch {
    creation_time: i64,
    accumulated_elapsed_time: i64,
}

impl Stopwatch {
    /// Port of the canonical constructor.
    pub const fn new(creation_time: i64, accumulated_elapsed_time: i64) -> Self {
        Stopwatch {
            creation_time,
            accumulated_elapsed_time,
        }
    }

    /// Port of `Stopwatch(long creationTime)`: nothing accumulated yet.
    pub const fn starting_at(creation_time: i64) -> Self {
        Stopwatch::new(creation_time, 0)
    }

    /// Port of the accessor `creationTime()`.
    pub const fn creation_time(&self) -> i64 {
        self.creation_time
    }

    /// Port of the accessor `accumulatedElapsedTime()`.
    pub const fn accumulated_elapsed_time(&self) -> i64 {
        self.accumulated_elapsed_time
    }

    /// Port of `elapsedMilliseconds(long currentTime)`.
    pub const fn elapsed_milliseconds(&self, current_time: i64) -> i64 {
        let time_since_instance_creation = current_time.wrapping_sub(self.creation_time);
        self.accumulated_elapsed_time
            .wrapping_add(time_since_instance_creation)
    }

    /// Port of `elapsedSeconds(long currentTime)`: `elapsedMilliseconds(currentTime) / 1000.0`.
    pub fn elapsed_seconds(&self, current_time: i64) -> f64 {
        self.elapsed_milliseconds(current_time) as f64 / 1000.0
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            java_object::hash_i64(self.creation_time),
            java_object::hash_i64(self.accumulated_elapsed_time),
        ])
    }
}

impl std::fmt::Display for Stopwatch {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Stopwatch[creationTime={}, accumulatedElapsedTime={}]",
            self.creation_time, self.accumulated_elapsed_time
        )
    }
}
