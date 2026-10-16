//! Port of: net/minecraft/world/ticks/TickPriority.java
//! Java class(es): net.minecraft.world.ticks.TickPriority
//! Status: PARTIAL
//!
//! Session 19. Seven constants with their `value` (`-3..=3`), `getValue` and `byValue` are
//! ported; names, ordinals and `value` are checked against the jar in
//! `_porting/tests/session19_b.rs`. PORT-BLOCKED: `CODEC` (`Codec.INT.xmap(byValue, getValue)`,
//! DataFixerUpper).
//!
//! # `byValue` CLAMPS, IT NEVER FAILS
//!
//! An exact match is returned if there is one. Otherwise anything below `EXTREMELY_HIGH.value`
//! (`-3`) is `EXTREMELY_HIGH` and anything else (necessarily above `3`) is `EXTREMELY_LOW`. A
//! saved tick with priority `-100` therefore loads as `EXTREMELY_HIGH` (the CODEC decodes
//! through `byValue`). Lower value = higher priority.

/// Port of `net.minecraft.world.ticks.TickPriority`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TickPriority {
    /// Port of `TickPriority#EXTREMELY_HIGH` -- ordinal 0, value `-3`.
    ExtremelyHigh = 0,
    /// Port of `TickPriority#VERY_HIGH` -- ordinal 1, value `-2`.
    VeryHigh = 1,
    /// Port of `TickPriority#HIGH` -- ordinal 2, value `-1`.
    High = 2,
    /// Port of `TickPriority#NORMAL` -- ordinal 3, value `0`.
    Normal = 3,
    /// Port of `TickPriority#LOW` -- ordinal 4, value `1`.
    Low = 4,
    /// Port of `TickPriority#VERY_LOW` -- ordinal 5, value `2`.
    VeryLow = 5,
    /// Port of `TickPriority#EXTREMELY_LOW` -- ordinal 6, value `3`.
    ExtremelyLow = 6,
}

/// Java: `TickPriority.values()` -- declaration order.
pub const VALUES: [TickPriority; 7] = [
    TickPriority::ExtremelyHigh,
    TickPriority::VeryHigh,
    TickPriority::High,
    TickPriority::Normal,
    TickPriority::Low,
    TickPriority::VeryLow,
    TickPriority::ExtremelyLow,
];

impl TickPriority {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            TickPriority::ExtremelyHigh => "EXTREMELY_HIGH",
            TickPriority::VeryHigh => "VERY_HIGH",
            TickPriority::High => "HIGH",
            TickPriority::Normal => "NORMAL",
            TickPriority::Low => "LOW",
            TickPriority::VeryLow => "VERY_LOW",
            TickPriority::ExtremelyLow => "EXTREMELY_LOW",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `TickPriority#getValue()`.
    #[inline]
    pub const fn get_value(&self) -> i32 {
        match self {
            TickPriority::ExtremelyHigh => -3,
            TickPriority::VeryHigh => -2,
            TickPriority::High => -1,
            TickPriority::Normal => 0,
            TickPriority::Low => 1,
            TickPriority::VeryLow => 2,
            TickPriority::ExtremelyLow => 3,
        }
    }
}

/// Port of `TickPriority#byValue(int)`: the first constant (declaration order) with that value,
/// else `EXTREMELY_HIGH` below `-3` and `EXTREMELY_LOW` otherwise.
pub fn by_value(value: i32) -> TickPriority {
    for priority in VALUES {
        if priority.get_value() == value {
            return priority;
        }
    }
    if value < TickPriority::ExtremelyHigh.get_value() {
        TickPriority::ExtremelyHigh
    } else {
        TickPriority::ExtremelyLow
    }
}

impl std::fmt::Display for TickPriority {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<TickPriority> CODEC = Codec.INT.xmap(TickPriority::byValue,
/// TickPriority::getValue)`.
///
/// `com.mojang.serialization.Codec` is not ported. The mapping it would apply is [`by_value`] /
/// [`TickPriority::get_value`].
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
