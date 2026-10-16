//! Port of: net/minecraft/world/level/material/FogType.java
//! Java class(es): net.minecraft.world.level.material.FogType
//! Status: VERIFIED
//!
//! Session 19. A plain five-constant enum (the fog the camera is in). Names and ordinals are
//! checked against the jar in `_porting/tests/session19_b.rs`.

/// Port of `net.minecraft.world.level.material.FogType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FogType {
    /// Port of `FogType#LAVA` -- ordinal 0.
    Lava = 0,
    /// Port of `FogType#WATER` -- ordinal 1.
    Water = 1,
    /// Port of `FogType#POWDER_SNOW` -- ordinal 2.
    PowderSnow = 2,
    /// Port of `FogType#ATMOSPHERIC` -- ordinal 3.
    Atmospheric = 3,
    /// Port of `FogType#NONE` -- ordinal 4.
    None = 4,
}

/// Java: `FogType.values()` -- declaration order.
pub const VALUES: [FogType; 5] = [
    FogType::Lava,
    FogType::Water,
    FogType::PowderSnow,
    FogType::Atmospheric,
    FogType::None,
];

impl FogType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            FogType::Lava => "LAVA",
            FogType::Water => "WATER",
            FogType::PowderSnow => "POWDER_SNOW",
            FogType::Atmospheric => "ATMOSPHERIC",
            FogType::None => "NONE",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for FogType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
