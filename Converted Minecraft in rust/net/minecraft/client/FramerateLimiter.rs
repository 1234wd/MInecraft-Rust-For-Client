//! Port of: net/minecraft/client/FramerateLimiter.java
//! Java class(es): net.minecraft.client.FramerateLimiter
//! Status: PORTED
//!
//! Session 19 batch h. Every member: the five private constants, the three static fields and
//! `limitDisplayFPS`. Nothing is PORT-BLOCKED; all constants are private, so the jar's dump has
//! no group for this class (hence `PORTED`).
//!
//! # Java-to-Rust mapping
//!
//! * `System.nanoTime()` -> [`crate::javacompat::entropy::nano_time`] (injectable for tests;
//!   only differences are meaningful, as in Java).
//! * The three `private static` fields live in one process-wide `Mutex`, initialised lazily on
//!   first use like Java's `<clinit>` (`lastFrameTime = System.nanoTime()`). The lock is held for
//!   the whole call; vanilla only calls this from the render thread.
//! * `LockSupport.parkNanos(n)` -> `std::thread::park_timeout(n ns)` (both may return early).
//! * `Thread.interrupted()` -> always `false`: Rust threads have no interrupt status, so the
//!   "skip the sleep when interrupted" branch can never be taken.
//! * `Thread.onSpinWait()` -> `std::hint::spin_loop()`.
//! * `1000000000L / framerateLimit` with `0` throws `ArithmeticException: / by zero`, reproduced
//!   as a panic.

use crate::javacompat::entropy;
use std::sync::{LazyLock, Mutex};

/// Port of the private `FramerateLimiter#OVERSHOOT_SMOOTHING`.
const OVERSHOOT_SMOOTHING: f64 = 0.1;
/// Port of the private `FramerateLimiter#MAX_CURRENT_OVERSHOOT_NS`.
const MAX_CURRENT_OVERSHOOT_NS: i64 = 25_000_000;
/// Port of the private `FramerateLimiter#MAX_AVERAGE_OVERSHOOT_NS`.
const MAX_AVERAGE_OVERSHOOT_NS: i64 = 2_000_000;
/// Port of the private `FramerateLimiter#ONE_SECOND_IN_NANOSECONDS`.
const ONE_SECOND_IN_NANOSECONDS: i64 = 1_000_000_000;
/// Port of the private `FramerateLimiter#SPIN_SAFETY_BUFFER_NS`.
const SPIN_SAFETY_BUFFER_NS: i64 = 500_000;

/// The three private static fields.
struct State {
    last_frame_time: i64,
    average_overshoot_ns: i64,
    last_framerate_limit: i32,
}

static STATE: LazyLock<Mutex<State>> = LazyLock::new(|| {
    Mutex::new(State {
        last_frame_time: entropy::nano_time(),
        average_overshoot_ns: 0,
        last_framerate_limit: 0,
    })
});

/// Port of `FramerateLimiter#limitDisplayFPS(int)`: sleeps (then spins for the last
/// `averageOvershoot + 0.5 ms`) until `1e9 / framerateLimit` ns have passed since the previous
/// call returned.
pub fn limit_display_fps(framerate_limit: i32) {
    let mut state = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if framerate_limit == 0 {
        panic!("java.lang.ArithmeticException: / by zero");
    }
    let target_time_per_frame = ONE_SECOND_IN_NANOSECONDS / framerate_limit as i64;
    let target_time_ns = state.last_frame_time.wrapping_add(target_time_per_frame);
    if framerate_limit != state.last_framerate_limit {
        state.average_overshoot_ns = 0;
        state.last_framerate_limit = framerate_limit;
    }

    loop {
        let remaining_time_ns = target_time_ns.wrapping_sub(entropy::nano_time());
        if remaining_time_ns <= 0 {
            break;
        }
        if remaining_time_ns > state.average_overshoot_ns + SPIN_SAFETY_BUFFER_NS {
            let sleep_start_time_ns = entropy::nano_time();
            let expected_sleep_time_ns = remaining_time_ns - state.average_overshoot_ns - SPIN_SAFETY_BUFFER_NS;
            // Java: `if (!Thread.interrupted())` -- Rust threads have no interrupt flag.
            let interrupted = false;
            if !interrupted {
                std::thread::park_timeout(std::time::Duration::from_nanos(expected_sleep_time_ns as u64));
                let sleep_duration_ns = entropy::nano_time().wrapping_sub(sleep_start_time_ns);
                let current_overshoot_ns = sleep_duration_ns.wrapping_sub(expected_sleep_time_ns);
                if current_overshoot_ns > 0 && current_overshoot_ns < MAX_CURRENT_OVERSHOOT_NS {
                    state.average_overshoot_ns = (OVERSHOOT_SMOOTHING * current_overshoot_ns as f64
                        + 0.9 * state.average_overshoot_ns as f64) as i64;
                    state.average_overshoot_ns = std::cmp::min(state.average_overshoot_ns, MAX_AVERAGE_OVERSHOOT_NS);
                }
            }
        } else {
            std::hint::spin_loop();
        }
    }

    state.last_frame_time = entropy::nano_time();
}

/// Snapshot of the private static state `(lastFrameTime, averageOvershootNs,
/// lastFramerateLimit)`, for tests. Not a Java member.
#[doc(hidden)]
pub fn debug_state() -> (i64, i64, i32) {
    let state = STATE.lock().unwrap_or_else(|e| e.into_inner());
    (state.last_frame_time, state.average_overshoot_ns, state.last_framerate_limit)
}
