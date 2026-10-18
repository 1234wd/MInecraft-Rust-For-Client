//! Port of: net/minecraft/util/SmoothDouble.java
//! Java class(es): net.minecraft.util.SmoothDouble
//! Status: VERIFIED
//!
//! Session 22. `public class SmoothDouble`: the smoothing behind the cinematic camera. Three
//! `double`s, all starting at `0.0`.
//!
//! # `getNewDeltaValue`
//!
//! ```java
//! this.targetValue += targetDelta;
//! double delta = this.targetValue - this.remainingValue;
//! double newLastAmount = Mth.lerp(0.5, this.lastAmount, delta);
//! double deltaSign = Math.signum(delta);
//! if (deltaSign * delta > deltaSign * this.lastAmount) {
//!     delta = newLastAmount;
//! }
//! this.lastAmount = newLastAmount;
//! this.remainingValue += delta * time;
//! return delta * time;
//! ```
//!
//! `Math.signum` returns a zero (of either sign) or NaN UNCHANGED, which Rust's `f64::signum`
//! does not -- it returns `1.0` for `+0.0`. With a zero delta Java's comparison is `0 > 0`, false;
//! with Rust's signum it could be true. So this is `javacompat::java_lang::signum_f64`.
//!
//! Verified by the `SMOOTH` group of `small_probe.txt` (`tests/session22.rs`): three scripts with
//! both zeros, NaN, `-Infinity`, `1.0E300` and subnormals, every return value as raw bits
//! (any NaN equal to any NaN: the sign of a computed NaN is not portable -- see `nan_canon`).

use crate::javacompat::java_lang;
use crate::net::minecraft::util::Mth::Mth;

/// Port of `net.minecraft.util.SmoothDouble`.
#[derive(Debug, Clone, Copy, Default)]
pub struct SmoothDouble {
    target_value: f64,
    remaining_value: f64,
    last_amount: f64,
}

impl SmoothDouble {
    /// Port of the implicit constructor: all zero.
    pub const fn new() -> Self {
        SmoothDouble {
            target_value: 0.0,
            remaining_value: 0.0,
            last_amount: 0.0,
        }
    }

    /// Port of `getNewDeltaValue(double targetDelta, double time)`.
    pub fn get_new_delta_value(&mut self, target_delta: f64, time: f64) -> f64 {
        self.target_value += target_delta;
        let mut delta = self.target_value - self.remaining_value;
        let new_last_amount = Mth::lerp_f64(0.5, self.last_amount, delta);
        let delta_sign = java_lang::signum_f64(delta);
        if delta_sign * delta > delta_sign * self.last_amount {
            delta = new_last_amount;
        }
        self.last_amount = new_last_amount;
        self.remaining_value += delta * time;
        delta * time
    }

    /// Port of `reset()`.
    pub fn reset(&mut self) {
        self.target_value = 0.0;
        self.remaining_value = 0.0;
        self.last_amount = 0.0;
    }
}
