//! Port of: net/minecraft/util/TickThrottler.java
//! Java class(es): net.minecraft.util.TickThrottler
//! Status: VERIFIED
//!
//! Session 22. `public class TickThrottler`: a leaky bucket. `increment()` adds `incrementStep`,
//! `tick()` drains one per call down to zero, and `isUnderThreshold()` says whether the bucket is
//! below `threshold`.
//!
//! # Edges
//!
//! * `increment()` is `count = count + incrementStep` in `int`, so it WRAPS: a large step can turn
//!   the count negative, which then reads as under the threshold.
//! * `tick()` only decrements a POSITIVE count, so a negative step or a wrapped count stays put.
//! * `isUnderThreshold()` is `threshold <= 0 || count < threshold` -- a non-positive threshold
//!   disables throttling entirely.
//!
//! `count` is private in Java and has no accessor here either. The probe reads it by reflection
//! after every step; the test reads it from this struct's `Debug` output, so no non-vanilla API is
//! added to observe it.
//!
//! Verified by the `THROTTLE` group of `small_probe.txt` (`tests/session22.rs`): eight
//! step/threshold pairs (zero, negative, `MAX_VALUE`) through a twenty-step script.

/// Port of `net.minecraft.util.TickThrottler`.
#[derive(Debug, Clone, Copy)]
pub struct TickThrottler {
    increment_step: i32,
    threshold: i32,
    count: i32,
}

impl TickThrottler {
    /// Port of `new TickThrottler(int incrementStep, int threshold)`: an empty bucket.
    pub const fn new(increment_step: i32, threshold: i32) -> Self {
        TickThrottler {
            increment_step,
            threshold,
            count: 0,
        }
    }

    /// Port of `increment()`: wrapping `int` addition.
    pub fn increment(&mut self) {
        self.count = self.count.wrapping_add(self.increment_step);
    }

    /// Port of `tick()`: drains one, only while the count is positive.
    pub fn tick(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }

    /// Port of `isUnderThreshold()`.
    pub const fn is_under_threshold(&self) -> bool {
        self.threshold <= 0 || self.count < self.threshold
    }
}
