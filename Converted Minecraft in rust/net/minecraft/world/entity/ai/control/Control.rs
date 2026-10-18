//! Port of: net/minecraft/world/entity/ai/control/Control.java
//! Java class(es): net.minecraft.world.entity.ai.control.Control
//! Status: VERIFIED
//!
//! Session 22. `public interface Control`: the base of the mob look/move/body controls, with one
//! default method.
//!
//! `rotateTowards(from, to, maxRot)` turns `from` toward `to` by at most `maxRot` degrees the short
//! way round: `from + Mth.clamp(Mth.degreesDifference(from, to), -maxRot, maxRot)`. Nothing checks
//! that `maxRot` is non-negative: a negative one inverts the bounds, and a NaN one fails every
//! comparison, and either way `Mth.clamp`'s own rule (`value < min ? min : Math.min(value, max)`)
//! decides the result. Both cases are probed.
//!
//! Verified by the `ROTTOWARDS` group of `small_probe.txt` (`tests/session22.rs`): 17 x 17 angle
//! pairs (both zeros, the 180 boundary, wraps past 360, `1e9`, NaN, infinity) x 7 limits.

use crate::net::minecraft::util::Mth::Mth;

/// Port of `net.minecraft.world.entity.ai.control.Control`.
pub trait Control {
    /// Port of `default float rotateTowards(float fromAngle, float toAngle, float maxRot)`.
    fn rotate_towards(&self, from_angle: f32, to_angle: f32, max_rot: f32) -> f32 {
        let diff = Mth::degrees_difference(from_angle, to_angle);
        let diff_clamped = Mth::clamp_f32(diff, -max_rot, max_rot);
        from_angle + diff_clamped
    }
}
