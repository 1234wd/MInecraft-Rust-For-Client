//! Port of: net/minecraft/world/entity/PlayerRideableJumping.java
//! Java class(es): net.minecraft.world.entity.PlayerRideableJumping
//! Status: VERIFIED
//!
//! Session 22. `public interface PlayerRideableJumping extends PlayerRideable`: a mount that can
//! jump under its rider. Four abstract methods and two defaults.
//!
//! `getPlayerJumpPendingScale(jumpAmount)` is `jumpAmount >= 90 ? 1.0F : 0.4F + 0.4F * jumpAmount /
//! 90.0F` -- evaluated left to right in `float`: `0.4F * jumpAmount` widens the `int` to `float`
//! first, then `/ 90.0F`, then `0.4F +`. Nothing clamps a negative amount, so `MIN_VALUE` gives a
//! large negative scale (probed). `getJumpCooldown()` defaults to `0`.
//!
//! Verified by the `RIDEJUMP` group of `small_probe.txt` (`tests/session22.rs`): eleven amounts
//! around 90 and at both `int` extremes, as raw bits.

use crate::net::minecraft::world::entity::PlayerRideable::PlayerRideable;

/// Port of `net.minecraft.world.entity.PlayerRideableJumping`.
pub trait PlayerRideableJumping: PlayerRideable {
    /// Port of `void onPlayerJump(int jumpAmount)`.
    fn on_player_jump(&mut self, jump_amount: i32);

    /// Port of `boolean canJump()`.
    fn can_jump(&self) -> bool;

    /// Port of `void handleStartJump(int jumpScale)`.
    fn handle_start_jump(&mut self, jump_scale: i32);

    /// Port of `void handleStopJump()`.
    fn handle_stop_jump(&mut self);

    /// Port of `default int getJumpCooldown()`: `0`.
    fn get_jump_cooldown(&self) -> i32 {
        0
    }

    /// Port of `default float getPlayerJumpPendingScale(int jumpAmount)`.
    fn get_player_jump_pending_scale(&self, jump_amount: i32) -> f32 {
        if jump_amount >= 90 {
            1.0
        } else {
            0.4 + 0.4 * jump_amount as f32 / 90.0
        }
    }
}
