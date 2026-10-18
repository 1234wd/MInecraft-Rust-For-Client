//! Port of: net/minecraft/world/entity/AnimationState.java
//! Java class(es): net.minecraft.world.entity.AnimationState
//! Status: VERIFIED
//!
//! Session 22. `public class AnimationState`: when an entity animation started, as a tick count,
//! with `Integer.MIN_VALUE` meaning "stopped".
//!
//! # The sentinel is an ordinary `int`
//!
//! `STOPPED` is `Integer.MIN_VALUE`, and nothing stops a caller passing it: `start(MIN_VALUE)`
//! leaves the state reading as NOT started. `fastForward` subtracts in plain `int` arithmetic, so
//! fast-forwarding a start of `MAX_VALUE` by `-1` wraps to `MIN_VALUE` and silently STOPS the
//! animation. Both are in the probe and both are kept.
//!
//! # Conversions
//!
//! * `fastForward(ticks, timeScale)`: `(int) (ticks * timeScale)` -- the `int` widened to `float`,
//!   a `float` multiply, then Java's saturating, NaN-to-zero narrowing (which Rust's `as` matches).
//! * `getTimeInMillis(ageInTicks)`: `startTick` widened to `float` (rounding at large magnitudes),
//!   a `float` subtraction, `* 50.0F`, then a saturating narrowing to `long`. A stopped state
//!   therefore reports a huge time rather than an error (`MIN_VALUE` as a float is `-2^31`).
//!
//! `ifStarted(consumer)` hands `this` to the consumer only when started; here the consumer gets
//! `&mut Self`, which is what a Java `Consumer<AnimationState>` can do with it.
//!
//! `startTick` is private in Java and has no accessor here; the probe reads it by reflection and
//! the test reads it from the derived `Debug` output.
//!
//! Verified by the `ANIM` group of `small_probe.txt` (`tests/session22.rs`): five scripts covering
//! every method, the sentinel, the wrap, NaN and infinite scales and ages.

/// Port of `private static final int STOPPED = Integer.MIN_VALUE`.
const STOPPED: i32 = i32::MIN;

/// Port of `net.minecraft.world.entity.AnimationState`.
#[derive(Debug, Clone, Copy)]
pub struct AnimationState {
    start_tick: i32,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::new()
    }
}

impl AnimationState {
    /// Port of the implicit constructor: stopped.
    pub const fn new() -> Self {
        AnimationState {
            start_tick: STOPPED,
        }
    }

    /// Port of `start(int tickCount)`.
    pub fn start(&mut self, tick_count: i32) {
        self.start_tick = tick_count;
    }

    /// Port of `startIfStopped(int tickCount)`.
    pub fn start_if_stopped(&mut self, tick_count: i32) {
        if !self.is_started() {
            self.start(tick_count);
        }
    }

    /// Port of `animateWhen(boolean condition, int tickCount)`.
    pub fn animate_when(&mut self, condition: bool, tick_count: i32) {
        if condition {
            self.start_if_stopped(tick_count);
        } else {
            self.stop();
        }
    }

    /// Port of `stop()`.
    pub fn stop(&mut self) {
        self.start_tick = STOPPED;
    }

    /// Port of `ifStarted(Consumer<AnimationState> timer)`.
    pub fn if_started(&mut self, timer: impl FnOnce(&mut AnimationState)) {
        if self.is_started() {
            timer(self);
        }
    }

    /// Port of `fastForward(int ticks, float timeScale)`: `startTick -= (int) (ticks * timeScale)`.
    pub fn fast_forward(&mut self, ticks: i32, time_scale: f32) {
        if self.is_started() {
            self.start_tick = self
                .start_tick
                .wrapping_sub((ticks as f32 * time_scale) as i32);
        }
    }

    /// Port of `getTimeInMillis(float ageInTicks)`.
    pub fn get_time_in_millis(&self, age_in_ticks: f32) -> i64 {
        let time_in_ticks = age_in_ticks - self.start_tick as f32;
        (time_in_ticks * 50.0) as i64
    }

    /// Port of `isStarted()`.
    pub const fn is_started(&self) -> bool {
        self.start_tick != STOPPED
    }

    /// Port of `copyFrom(AnimationState state)`.
    pub fn copy_from(&mut self, state: &AnimationState) {
        self.start_tick = state.start_tick;
    }
}
