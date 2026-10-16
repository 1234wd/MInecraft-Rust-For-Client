//! Port of: net/minecraft/world/level/border/BorderStatus.java
//! Java class(es): net.minecraft.world.level.border.BorderStatus
//! Status: VERIFIED
//!
//! Session 19. Three constants, each with the world-border render `color` (packed `0xRRGGBB`,
//! no alpha). Names, ordinals and `color` are checked against the jar in
//! `_porting/tests/session19_b.rs`.

/// Port of `net.minecraft.world.level.border.BorderStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BorderStatus {
    /// Port of `BorderStatus#GROWING` -- ordinal 0, color `4259712` (`0x40FF80`).
    Growing = 0,
    /// Port of `BorderStatus#SHRINKING` -- ordinal 1, color `16724016` (`0xFF3030`).
    Shrinking = 1,
    /// Port of `BorderStatus#STATIONARY` -- ordinal 2, color `2138367` (`0x20A0FF`).
    Stationary = 2,
}

/// Java: `BorderStatus.values()` -- declaration order.
pub const VALUES: [BorderStatus; 3] = [
    BorderStatus::Growing,
    BorderStatus::Shrinking,
    BorderStatus::Stationary,
];

impl BorderStatus {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            BorderStatus::Growing => "GROWING",
            BorderStatus::Shrinking => "SHRINKING",
            BorderStatus::Stationary => "STATIONARY",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `BorderStatus#getColor()`.
    #[inline]
    pub const fn get_color(&self) -> i32 {
        match self {
            BorderStatus::Growing => 4259712,
            BorderStatus::Shrinking => 16724016,
            BorderStatus::Stationary => 2138367,
        }
    }
}

impl std::fmt::Display for BorderStatus {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
