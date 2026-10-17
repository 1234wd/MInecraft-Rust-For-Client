//! Port of: net/minecraft/client/gui/navigation/ScreenPosition.java
//! Java class(es): net.minecraft.client.gui.navigation.ScreenPosition
//! Status: VERIFIED
//!
//! Session 22. `public record ScreenPosition(int x, int y)` with a factory and two readers keyed
//! on [`ScreenAxis`] / [`ScreenDirection`].
//!
//! * `of(axis, primary, secondary)`: `HORIZONTAL` puts the primary value in `x`, `VERTICAL` in `y`.
//! * `step(direction)`: one unit, with `DOWN` as `y + 1` (screen coordinates grow downwards).
//!   Plain `int` arithmetic, so a step past `MAX_VALUE` wraps -- `wrapping_*` here.
//! * `getCoordinate(axis)`: `x` for `HORIZONTAL`, `y` for `VERTICAL`.
//!
//! # Record members
//!
//! `equals` is derived, `hashCode` is [`ScreenPosition::java_hash_code`] (`31 * x + y`),
//! `toString` is `ScreenPosition[x=<n>, y=<n>]`.
//!
//! Verified by the `REC`, `RECEQ`, `SPOF`, `SPSTEP` and `SPCOORD` groups of `small_probe.txt`
//! (`tests/session22.rs`) over a 5x5 grid of coordinates including `MIN_VALUE` and `MAX_VALUE`.

use crate::javacompat::java_object;
use crate::net::minecraft::client::gui::navigation::ScreenAxis::ScreenAxis;
use crate::net::minecraft::client::gui::navigation::ScreenDirection::ScreenDirection;

/// Port of `net.minecraft.client.gui.navigation.ScreenPosition`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScreenPosition {
    x: i32,
    y: i32,
}

impl ScreenPosition {
    /// Port of the canonical constructor.
    pub const fn new(x: i32, y: i32) -> Self {
        ScreenPosition { x, y }
    }

    /// Port of the accessor `x()`.
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Port of the accessor `y()`.
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Port of `of(ScreenAxis axis, int primaryValue, int secondaryValue)`.
    pub const fn of(axis: ScreenAxis, primary_value: i32, secondary_value: i32) -> ScreenPosition {
        match axis {
            ScreenAxis::Horizontal => ScreenPosition::new(primary_value, secondary_value),
            ScreenAxis::Vertical => ScreenPosition::new(secondary_value, primary_value),
        }
    }

    /// Port of `step(ScreenDirection direction)`.
    pub const fn step(&self, direction: ScreenDirection) -> ScreenPosition {
        match direction {
            ScreenDirection::Down => ScreenPosition::new(self.x, self.y.wrapping_add(1)),
            ScreenDirection::Up => ScreenPosition::new(self.x, self.y.wrapping_sub(1)),
            ScreenDirection::Left => ScreenPosition::new(self.x.wrapping_sub(1), self.y),
            ScreenDirection::Right => ScreenPosition::new(self.x.wrapping_add(1), self.y),
        }
    }

    /// Port of `getCoordinate(ScreenAxis axis)`.
    pub const fn get_coordinate(&self, axis: ScreenAxis) -> i32 {
        match axis {
            ScreenAxis::Horizontal => self.x,
            ScreenAxis::Vertical => self.y,
        }
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[self.x, self.y])
    }
}

impl std::fmt::Display for ScreenPosition {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScreenPosition[x={}, y={}]", self.x, self.y)
    }
}
