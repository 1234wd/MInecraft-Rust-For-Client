//! Port of: net/minecraft/client/entity/ClientAvatarState.java
//! Java class(es): net.minecraft.client.entity.ClientAvatarState
//! Status: VERIFIED
//!
//! Session 22. `public class ClientAvatarState`: the client-side animation state of a player
//! avatar -- the cape ("cloak") position that trails the body, the view bob and the walk distance,
//! each with its previous-tick value for interpolation.
//!
//! # `moveCloak`
//!
//! Per axis, `d = pos - cloak`; if `!(d > 10.0) && !(d < -10.0)` the cloak eases a quarter of the
//! way (`cloak += d * 0.25`), otherwise it SNAPS to the position and its previous value is reset
//! too. The test is written with negations, so a NaN difference takes the EASE branch and poisons
//! the cloak with NaN rather than snapping -- kept, and probed.
//!
//! # Interpolation widths
//!
//! The cloak readers pass a `float` partial tick to `Mth.lerp(double, double, double)`, so the
//! partial is WIDENED to `double`; the bob and walk-distance readers stay in `float`.
//! `getBackwardsInterpolatedWalkDistance(p)` is `-(walkDist + (walkDist - walkDistO) * p)`.
//!
//! Verified by the `AVATAR` group of `small_probe.txt` (`tests/session22.rs`): a fifteen-step
//! script (a snap, a NaN axis, a `1.0E9` teleport) with every reader at three partial ticks after
//! every step, as raw bits (any NaN equal to any NaN: the sign of a computed NaN is not
//! portable -- see `nan_canon` in `tests/support/small_probe.rs`).

use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::world::phys::Vec3::Vec3;

/// Port of `net.minecraft.client.entity.ClientAvatarState`.
#[derive(Debug, Clone, Copy)]
pub struct ClientAvatarState {
    delta_movement_on_previous_tick: Vec3,
    walk_dist: f32,
    walk_dist_o: f32,
    x_cloak: f64,
    y_cloak: f64,
    z_cloak: f64,
    x_cloak_o: f64,
    y_cloak_o: f64,
    z_cloak_o: f64,
    bob: f32,
    bob_o: f32,
}

impl Default for ClientAvatarState {
    fn default() -> Self {
        ClientAvatarState::new()
    }
}

impl ClientAvatarState {
    /// Port of the implicit constructor: everything zero, `deltaMovementOnPreviousTick = Vec3.ZERO`.
    pub const fn new() -> Self {
        ClientAvatarState {
            delta_movement_on_previous_tick: Vec3::ZERO,
            walk_dist: 0.0,
            walk_dist_o: 0.0,
            x_cloak: 0.0,
            y_cloak: 0.0,
            z_cloak: 0.0,
            x_cloak_o: 0.0,
            y_cloak_o: 0.0,
            z_cloak_o: 0.0,
            bob: 0.0,
            bob_o: 0.0,
        }
    }

    /// Port of `tick(Vec3 position, Vec3 deltaMovement)`.
    pub fn tick(&mut self, position: &Vec3, delta_movement: &Vec3) {
        self.walk_dist_o = self.walk_dist;
        self.delta_movement_on_previous_tick = *delta_movement;
        self.move_cloak(position);
    }

    /// Port of `addWalkDistance(float added)`.
    pub fn add_walk_distance(&mut self, added: f32) {
        self.walk_dist += added;
    }

    /// Port of `deltaMovementOnPreviousTick()`.
    pub const fn delta_movement_on_previous_tick(&self) -> Vec3 {
        self.delta_movement_on_previous_tick
    }

    /// Port of `private void moveCloak(Vec3 pos)`.
    fn move_cloak(&mut self, pos: &Vec3) {
        self.x_cloak_o = self.x_cloak;
        self.y_cloak_o = self.y_cloak;
        self.z_cloak_o = self.z_cloak;
        let x = pos.x - self.x_cloak;
        let y = pos.y - self.y_cloak;
        let z = pos.z - self.z_cloak;
        if !(x > 10.0) && !(x < -10.0) {
            self.x_cloak += x * 0.25;
        } else {
            self.x_cloak = pos.x;
            self.x_cloak_o = self.x_cloak;
        }
        if !(y > 10.0) && !(y < -10.0) {
            self.y_cloak += y * 0.25;
        } else {
            self.y_cloak = pos.y;
            self.y_cloak_o = self.y_cloak;
        }
        if !(z > 10.0) && !(z < -10.0) {
            self.z_cloak += z * 0.25;
        } else {
            self.z_cloak = pos.z;
            self.z_cloak_o = self.z_cloak;
        }
    }

    /// Port of `getInterpolatedCloakX(float partialTicks)`: the partial widened to `double`.
    pub fn get_interpolated_cloak_x(&self, partial_ticks: f32) -> f64 {
        Mth::lerp_f64(partial_ticks as f64, self.x_cloak_o, self.x_cloak)
    }

    /// Port of `getInterpolatedCloakY(float partialTicks)`.
    pub fn get_interpolated_cloak_y(&self, partial_ticks: f32) -> f64 {
        Mth::lerp_f64(partial_ticks as f64, self.y_cloak_o, self.y_cloak)
    }

    /// Port of `getInterpolatedCloakZ(float partialTicks)`.
    pub fn get_interpolated_cloak_z(&self, partial_ticks: f32) -> f64 {
        Mth::lerp_f64(partial_ticks as f64, self.z_cloak_o, self.z_cloak)
    }

    /// Port of `updateBob(float tBob)`.
    pub fn update_bob(&mut self, t_bob: f32) {
        self.bob_o = self.bob;
        self.bob = self.bob + (t_bob - self.bob) * 0.4;
    }

    /// Port of `resetBob()`.
    pub fn reset_bob(&mut self) {
        self.bob_o = self.bob;
        self.bob = 0.0;
    }

    /// Port of `getInterpolatedBob(float partialTicks)`.
    pub fn get_interpolated_bob(&self, partial_ticks: f32) -> f32 {
        Mth::lerp_f32(partial_ticks, self.bob_o, self.bob)
    }

    /// Port of `getBackwardsInterpolatedWalkDistance(float partialTicks)`.
    pub fn get_backwards_interpolated_walk_distance(&self, partial_ticks: f32) -> f32 {
        let wda = self.walk_dist - self.walk_dist_o;
        -(self.walk_dist + wda * partial_ticks)
    }

    /// Port of `getInterpolatedWalkDistance(float partialTicks)`.
    pub fn get_interpolated_walk_distance(&self, partial_ticks: f32) -> f32 {
        Mth::lerp_f32(partial_ticks, self.walk_dist_o, self.walk_dist)
    }
}
