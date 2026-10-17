//! Port of: net/minecraft/world/entity/WalkAnimationState.java
//! Java class(es): net.minecraft.world.entity.WalkAnimationState
//! Status: VERIFIED
//!
//! Session 22. `public class WalkAnimationState`: the limb-swing state behind every walking
//! entity's leg animation -- a smoothed speed, its previous value, an accumulated position and a
//! position scale.
//!
//! # Arithmetic
//!
//! All `float`, one IEEE operation at a time, in Java's order:
//!
//! * `update`: `speed = speed + (targetSpeed - speed) * factor`, then `position += speed`.
//! * `speed(partial)`: `Math.min(Mth.lerp(partial, speedOld, speed), 1.0F)` -- Java's `Math.min`,
//!   which PROPAGATES NaN (Rust's `f32::min` would return `1.0`), so it is
//!   `javacompat::java_lang::min_f32`.
//! * `position(partial)`: `(position - speed * (1.0F - partial)) * positionScale`.
//! * `isMoving()`: `speed > 1.0E-5F`.
//!
//! `stop()` zeroes speed, previous speed and position but NOT the position scale.
//!
//! Verified by the `WALK` group of `small_probe.txt` (`tests/session22.rs`): three scripts, every
//! reader at five partial ticks after every step, with NaN and infinite inputs.

use crate::javacompat::java_lang;
use crate::net::minecraft::util::Mth::Mth;

/// Port of `net.minecraft.world.entity.WalkAnimationState`.
#[derive(Debug, Clone, Copy)]
pub struct WalkAnimationState {
    speed_old: f32,
    speed: f32,
    position: f32,
    position_scale: f32,
}

impl Default for WalkAnimationState {
    fn default() -> Self {
        WalkAnimationState::new()
    }
}

impl WalkAnimationState {
    /// Port of the implicit constructor: all zero, `positionScale = 1.0F`.
    pub const fn new() -> Self {
        WalkAnimationState {
            speed_old: 0.0,
            speed: 0.0,
            position: 0.0,
            position_scale: 1.0,
        }
    }

    /// Port of `setSpeed(float speed)`.
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    /// Port of `update(float targetSpeed, float factor, float positionScale)`.
    pub fn update(&mut self, target_speed: f32, factor: f32, position_scale: f32) {
        self.speed_old = self.speed;
        self.speed = self.speed + (target_speed - self.speed) * factor;
        self.position = self.position + self.speed;
        self.position_scale = position_scale;
    }

    /// Port of `stop()`: the position scale is kept.
    pub fn stop(&mut self) {
        self.speed_old = 0.0;
        self.speed = 0.0;
        self.position = 0.0;
    }

    /// Port of `speed()`.
    pub const fn speed(&self) -> f32 {
        self.speed
    }

    /// Port of `speed(float partialTicks)`.
    pub fn speed_f32(&self, partial_ticks: f32) -> f32 {
        java_lang::min_f32(Mth::lerp_f32(partial_ticks, self.speed_old, self.speed), 1.0)
    }

    /// Port of `position()`.
    pub fn position(&self) -> f32 {
        self.position * self.position_scale
    }

    /// Port of `position(float partialTicks)`.
    pub fn position_f32(&self, partial_ticks: f32) -> f32 {
        (self.position - self.speed * (1.0 - partial_ticks)) * self.position_scale
    }

    /// Port of `isMoving()`.
    pub fn is_moving(&self) -> bool {
        self.speed > 1.0e-5
    }
}
