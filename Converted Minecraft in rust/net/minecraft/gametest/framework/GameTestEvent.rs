//! Port of: net/minecraft/gametest/framework/GameTestEvent.java
//! Java class(es): net.minecraft.gametest.framework.GameTestEvent
//! Status: VERIFIED
//!
//! Session 22. `public class GameTestEvent`: one step of a game-test sequence -- an assertion to
//! run, optionally pinned to an expected tick or held back by a minimum delay. Three public final
//! fields, a private constructor and three factories.
//!
//! The two delays are BOXED `Long`s and nullable; each factory sets at most one of them:
//!
//! | factory | expectedDelay | minimumDelay |
//! |---|---|---|
//! | `create(Runnable)` | null | null |
//! | `create(long, Runnable)` | the tick | null |
//! | `createWithMinimumDelay(long, Runnable)` | null | the delay |
//!
//! `create` is overloaded, so per `DESIGN_DECISIONS.md#overload-naming` the two are
//! [`GameTestEvent::create`] and [`GameTestEvent::create_i64`]. The `Runnable` is a boxed
//! `FnMut() + Send`.
//!
//! Verified by the `GAMETEST` group of `small_probe.txt` (`tests/session22.rs`), including that
//! each factory stores the very runnable it was given.

/// Port of `net.minecraft.gametest.framework.GameTestEvent`.
pub struct GameTestEvent {
    /// Port of `public final @Nullable Long expectedDelay`.
    pub expected_delay: Option<i64>,
    /// Port of `public final @Nullable Long minimumDelay`.
    pub minimum_delay: Option<i64>,
    /// Port of `public final Runnable assertion`.
    pub assertion: Box<dyn FnMut() + Send>,
}

impl GameTestEvent {
    /// Port of the private constructor.
    fn new(expected_delay: Option<i64>, minimum_delay: Option<i64>, assertion: Box<dyn FnMut() + Send>) -> Self {
        GameTestEvent {
            expected_delay,
            minimum_delay,
            assertion,
        }
    }

    /// Port of `create(Runnable runnable)`: no delay of either kind.
    pub fn create(runnable: impl FnMut() + Send + 'static) -> GameTestEvent {
        GameTestEvent::new(None, None, Box::new(runnable))
    }

    /// Port of `create(long expectedTick, Runnable runnable)`.
    pub fn create_i64(expected_tick: i64, runnable: impl FnMut() + Send + 'static) -> GameTestEvent {
        GameTestEvent::new(Some(expected_tick), None, Box::new(runnable))
    }

    /// Port of `createWithMinimumDelay(long minimumDelay, Runnable runnable)`.
    pub fn create_with_minimum_delay(minimum_delay: i64, runnable: impl FnMut() + Send + 'static) -> GameTestEvent {
        GameTestEvent::new(None, Some(minimum_delay), Box::new(runnable))
    }
}

impl std::fmt::Debug for GameTestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameTestEvent")
            .field("expected_delay", &self.expected_delay)
            .field("minimum_delay", &self.minimum_delay)
            .finish_non_exhaustive()
    }
}
