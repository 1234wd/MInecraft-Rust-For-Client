//! Port of: net/minecraft/client/gui/navigation/ScreenDirection.java
//! Java class(es): net.minecraft.client.gui.navigation.ScreenDirection
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`.
//!
//! `coordinateValueComparator()` is a per-constant `IntComparator` returning exactly `-1`, `0` or `1`
//! (not an arbitrary sign): equal coordinates are `0`, otherwise `-1` when `isBefore(a, b)`. For the
//! negative directions (UP, LEFT) "before" means numerically GREATER.

use crate::net::minecraft::client::gui::navigation::ScreenAxis::ScreenAxis;

/// Port of `net.minecraft.client.gui.navigation.ScreenDirection`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScreenDirection {
    /// `UP` -- ordinal 0.
    Up = 0,
    /// `DOWN` -- ordinal 1.
    Down = 1,
    /// `LEFT` -- ordinal 2.
    Left = 2,
    /// `RIGHT` -- ordinal 3.
    Right = 3,
}

/// Java: `ScreenDirection.values()`, declaration order.
pub const VALUES: [ScreenDirection; 4] = [
    ScreenDirection::Up,
    ScreenDirection::Down,
    ScreenDirection::Left,
    ScreenDirection::Right,
];

impl ScreenDirection {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ScreenDirection::Up => "UP",
            ScreenDirection::Down => "DOWN",
            ScreenDirection::Left => "LEFT",
            ScreenDirection::Right => "RIGHT",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ScreenDirection#getAxis()`.
    pub const fn get_axis(&self) -> ScreenAxis {
        match self {
            ScreenDirection::Up | ScreenDirection::Down => ScreenAxis::Vertical,
            ScreenDirection::Left | ScreenDirection::Right => ScreenAxis::Horizontal,
        }
    }

    /// Port of `ScreenDirection#getOpposite()`.
    pub const fn get_opposite(&self) -> ScreenDirection {
        match self {
            ScreenDirection::Up => ScreenDirection::Down,
            ScreenDirection::Down => ScreenDirection::Up,
            ScreenDirection::Left => ScreenDirection::Right,
            ScreenDirection::Right => ScreenDirection::Left,
        }
    }

    /// Port of `ScreenDirection#isPositive()`.
    pub const fn is_positive(&self) -> bool {
        match self {
            ScreenDirection::Up | ScreenDirection::Left => false,
            ScreenDirection::Down | ScreenDirection::Right => true,
        }
    }

    /// Port of `ScreenDirection#isAfter(int, int)`.
    pub const fn is_after(&self, a: i32, b: i32) -> bool {
        if self.is_positive() {
            a > b
        } else {
            b > a
        }
    }

    /// Port of `ScreenDirection#isBefore(int, int)`.
    pub const fn is_before(&self, a: i32, b: i32) -> bool {
        if self.is_positive() {
            a < b
        } else {
            b < a
        }
    }

    /// Port of `ScreenDirection#coordinateValueComparator()`, applied.
    pub const fn compare_coordinates(&self, k1: i32, k2: i32) -> i32 {
        if k1 == k2 {
            0
        } else if self.is_before(k1, k2) {
            -1
        } else {
            1
        }
    }

    /// Port of `ScreenDirection#coordinateValueComparator()` as a value (the Java `IntComparator`).
    pub fn coordinate_value_comparator(&self) -> impl Fn(i32, i32) -> i32 + Copy {
        let d = *self;
        move |k1, k2| d.compare_coordinates(k1, k2)
    }
}

impl std::fmt::Display for ScreenDirection {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
