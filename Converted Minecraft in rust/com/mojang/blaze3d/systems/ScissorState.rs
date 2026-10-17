//! Port of: com/mojang/blaze3d/systems/ScissorState.java
//! Java class(es): com.mojang.blaze3d.systems.ScissorState
//! Status: VERIFIED
//!
//! Session 22. `public class ScissorState`: a mutable scissor rectangle plus an enabled flag, with
//! a no-arg constructor (disabled, all zero), a copy constructor and `setFrom`.
//!
//! `disable()` clears ONLY the flag -- the rectangle is kept, so a later read of `x()`/`width()`
//! still returns the last enabled values (the probe checks this). The copy constructor and
//! `setFrom` copy all five fields; a copy is independent of its source afterwards.
//!
//! No `equals` override in Java (identity), so no `PartialEq` here.
//!
//! Verified by the `SCISSOR` group of `small_probe.txt` (`tests/session22.rs`).

/// Port of `com.mojang.blaze3d.systems.ScissorState`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScissorState {
    enabled: bool,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl ScissorState {
    /// Port of `new ScissorState()`: disabled, all zero.
    pub const fn new() -> Self {
        ScissorState {
            enabled: false,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    /// Port of `new ScissorState(ScissorState state)`: a copy of all five fields.
    pub const fn copy_of(state: &ScissorState) -> Self {
        ScissorState {
            enabled: state.enabled,
            x: state.x,
            y: state.y,
            width: state.width,
            height: state.height,
        }
    }

    /// Port of `enable(int x, int y, int width, int height)`.
    pub fn enable(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.enabled = true;
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    /// Port of `disable()`: clears the flag only; the rectangle is kept.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Port of `enabled()`.
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    /// Port of `x()`.
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Port of `y()`.
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Port of `width()`.
    pub const fn width(&self) -> i32 {
        self.width
    }

    /// Port of `height()`.
    pub const fn height(&self) -> i32 {
        self.height
    }

    /// Port of `setFrom(ScissorState state)`.
    pub fn set_from(&mut self, state: &ScissorState) {
        self.enabled = state.enabled;
        self.x = state.x;
        self.y = state.y;
        self.width = state.width;
        self.height = state.height;
    }
}
