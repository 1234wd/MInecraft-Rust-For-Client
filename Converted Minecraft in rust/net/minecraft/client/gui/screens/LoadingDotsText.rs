//! Port of: net/minecraft/client/gui/screens/LoadingDotsText.java
//! Java class(es): net.minecraft.client.gui.screens.LoadingDotsText
//! Status: VERIFIED
//!
//! Session 22. `public class LoadingDotsText`: a four-frame "O o o" spinner picked from a clock.
//!
//! # A negative time can index the array with a NEGATIVE number
//!
//! ```java
//! int index = (int)(timeMs / 300L % FRAMES.length);
//! return FRAMES[index];
//! ```
//!
//! Both `/` and `%` are on `long` and truncate toward zero, so for `timeMs` in `-599..=-300` the
//! index is `-1`, and Java throws `ArrayIndexOutOfBoundsException: Index -1 out of bounds for
//! length 4`. Times that are whole negative multiples of 1200 come back round to frame 0. The port
//! panics with Java's exact text (`DESIGN_DECISIONS.md#runtime-exceptions`) rather than wrapping
//! the index, because a wrapped index would quietly show a frame the game never shows.
//!
//! Verified by the `DOTS` group of `small_probe.txt` (`tests/session22.rs`): nineteen times,
//! including every frame boundary, the AIOOBE band and both `long` extremes.

/// Port of `private static final String[] FRAMES`.
const FRAMES: [&str; 4] = ["O o o", "o O o", "o o O", "o O o"];

/// Port of `private static final long INTERVAL_MS = 300L`.
const INTERVAL_MS: i64 = 300;

/// Port of `LoadingDotsText.get(long timeMs)`.
///
/// # Panics
///
/// With Java's `ArrayIndexOutOfBoundsException` text when the index is negative.
pub fn get(time_ms: i64) -> &'static str {
    let index = (time_ms / INTERVAL_MS % FRAMES.len() as i64) as i32;
    if index < 0 {
        panic!(
            "java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {}",
            FRAMES.len()
        );
    }
    FRAMES[index as usize]
}
