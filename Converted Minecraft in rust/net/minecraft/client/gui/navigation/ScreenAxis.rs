//! Port of: net/minecraft/client/gui/navigation/ScreenAxis.java
//! Java class(es): net.minecraft.client.gui.navigation.ScreenAxis
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`.

use crate::net::minecraft::client::gui::navigation::ScreenDirection::ScreenDirection;

/// Port of `net.minecraft.client.gui.navigation.ScreenAxis`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScreenAxis {
    /// `HORIZONTAL` -- ordinal 0.
    Horizontal = 0,
    /// `VERTICAL` -- ordinal 1.
    Vertical = 1,
}

/// Java: `ScreenAxis.values()`, declaration order.
pub const VALUES: [ScreenAxis; 2] = [
    ScreenAxis::Horizontal,
    ScreenAxis::Vertical,
];

impl ScreenAxis {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ScreenAxis::Horizontal => "HORIZONTAL",
            ScreenAxis::Vertical => "VERTICAL",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ScreenAxis#orthogonal()`.
    pub const fn orthogonal(&self) -> ScreenAxis {
        match self {
            ScreenAxis::Horizontal => ScreenAxis::Vertical,
            ScreenAxis::Vertical => ScreenAxis::Horizontal,
        }
    }

    /// Port of `ScreenAxis#getPositive()`: RIGHT and DOWN (screen y grows downward).
    pub const fn get_positive(&self) -> ScreenDirection {
        match self {
            ScreenAxis::Horizontal => ScreenDirection::Right,
            ScreenAxis::Vertical => ScreenDirection::Down,
        }
    }

    /// Port of `ScreenAxis#getNegative()`.
    pub const fn get_negative(&self) -> ScreenDirection {
        match self {
            ScreenAxis::Horizontal => ScreenDirection::Left,
            ScreenAxis::Vertical => ScreenDirection::Up,
        }
    }

    /// Port of `ScreenAxis#getDirection(boolean)`.
    pub const fn get_direction(&self, positive: bool) -> ScreenDirection {
        if positive {
            self.get_positive()
        } else {
            self.get_negative()
        }
    }
}

impl std::fmt::Display for ScreenAxis {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
