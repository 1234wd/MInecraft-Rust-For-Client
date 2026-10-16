//! Port of: net/minecraft/world/entity/MobCategory.java
//! Java class(es): net.minecraft.world.entity.MobCategory
//! Status: PARTIAL
//!
//! Session 19. The eight constants with every field (`max`, `isFriendly`, `isPersistent`,
//! `name`, `debugAbbreviation`, `noDespawnDistance`, `despawnDistance`) and every getter are
//! ported and checked against the jar in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec`).
//!
//! # VALUES WORTH NOT "FIXING"
//!
//! * `MISC` has `max = -1` (it is never spawned by the natural spawner).
//! * `WATER_AMBIENT` is the only category with `despawnDistance = 64`; all others are 128.
//! * `noDespawnDistance` is a per-instance field initialised to `32`, but
//!   `getNoDespawnDistance()` returns the literal `32` rather than reading it. Same value.
//! * `CREATURE` and `MISC` are the only persistent categories.

/// Port of `net.minecraft.world.entity.MobCategory`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MobCategory {
    /// Port of `MobCategory#MONSTER` -- ordinal 0, `("monster", "MO", 70, false, false, 128)`.
    Monster = 0,
    /// Port of `MobCategory#CREATURE` -- ordinal 1, `("creature", "C", 10, true, true, 128)`.
    Creature = 1,
    /// Port of `MobCategory#AMBIENT` -- ordinal 2, `("ambient", "AM", 15, true, false, 128)`.
    Ambient = 2,
    /// Port of `MobCategory#AXOLOTLS` -- ordinal 3, `("axolotls", "AX", 5, true, false, 128)`.
    Axolotls = 3,
    /// Port of `MobCategory#UNDERGROUND_WATER_CREATURE` -- ordinal 4,
    /// `("underground_water_creature", "UWC", 5, true, false, 128)`.
    UndergroundWaterCreature = 4,
    /// Port of `MobCategory#WATER_CREATURE` -- ordinal 5,
    /// `("water_creature", "WC", 5, true, false, 128)`.
    WaterCreature = 5,
    /// Port of `MobCategory#WATER_AMBIENT` -- ordinal 6,
    /// `("water_ambient", "WA", 20, true, false, 64)`.
    WaterAmbient = 6,
    /// Port of `MobCategory#MISC` -- ordinal 7, `("misc", "MI", -1, true, true, 128)`.
    Misc = 7,
}

/// Java: `MobCategory.values()` -- declaration order.
pub const VALUES: [MobCategory; 8] = [
    MobCategory::Monster,
    MobCategory::Creature,
    MobCategory::Ambient,
    MobCategory::Axolotls,
    MobCategory::UndergroundWaterCreature,
    MobCategory::WaterCreature,
    MobCategory::WaterAmbient,
    MobCategory::Misc,
];

impl MobCategory {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            MobCategory::Monster => "MONSTER",
            MobCategory::Creature => "CREATURE",
            MobCategory::Ambient => "AMBIENT",
            MobCategory::Axolotls => "AXOLOTLS",
            MobCategory::UndergroundWaterCreature => "UNDERGROUND_WATER_CREATURE",
            MobCategory::WaterCreature => "WATER_CREATURE",
            MobCategory::WaterAmbient => "WATER_AMBIENT",
            MobCategory::Misc => "MISC",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `MobCategory#getName()` -- the private `name` field.
    #[inline]
    pub const fn get_name(&self) -> &'static str {
        match self {
            MobCategory::Monster => "monster",
            MobCategory::Creature => "creature",
            MobCategory::Ambient => "ambient",
            MobCategory::Axolotls => "axolotls",
            MobCategory::UndergroundWaterCreature => "underground_water_creature",
            MobCategory::WaterCreature => "water_creature",
            MobCategory::WaterAmbient => "water_ambient",
            MobCategory::Misc => "misc",
        }
    }

    /// Port of `MobCategory#getDebugAbbreviation()`.
    #[inline]
    pub const fn get_debug_abbreviation(&self) -> &'static str {
        match self {
            MobCategory::Monster => "MO",
            MobCategory::Creature => "C",
            MobCategory::Ambient => "AM",
            MobCategory::Axolotls => "AX",
            MobCategory::UndergroundWaterCreature => "UWC",
            MobCategory::WaterCreature => "WC",
            MobCategory::WaterAmbient => "WA",
            MobCategory::Misc => "MI",
        }
    }

    /// Port of `MobCategory#getSerializedName()` -- the same `name` field as `getName()`.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        self.get_name()
    }

    /// Port of `MobCategory#getMaxInstancesPerChunk()` -- the private `max` field.
    #[inline]
    pub const fn get_max_instances_per_chunk(&self) -> i32 {
        match self {
            MobCategory::Monster => 70,
            MobCategory::Creature => 10,
            MobCategory::Ambient => 15,
            MobCategory::Axolotls => 5,
            MobCategory::UndergroundWaterCreature => 5,
            MobCategory::WaterCreature => 5,
            MobCategory::WaterAmbient => 20,
            MobCategory::Misc => -1,
        }
    }

    /// Port of `MobCategory#isFriendly()`. Only `MONSTER` is unfriendly.
    #[inline]
    pub const fn is_friendly(&self) -> bool {
        !matches!(self, MobCategory::Monster)
    }

    /// Port of `MobCategory#isPersistent()`. `CREATURE` and `MISC` only.
    #[inline]
    pub const fn is_persistent(&self) -> bool {
        matches!(self, MobCategory::Creature | MobCategory::Misc)
    }

    /// Port of `MobCategory#getDespawnDistance()`.
    #[inline]
    pub const fn get_despawn_distance(&self) -> i32 {
        match self {
            MobCategory::WaterAmbient => 64,
            _ => 128,
        }
    }

    /// Port of `MobCategory#getNoDespawnDistance()` -- Java returns the literal `32` (the
    /// `noDespawnDistance` field holds the same value for every constant).
    #[inline]
    pub const fn get_no_despawn_distance(&self) -> i32 {
        32
    }
}

impl std::fmt::Display for MobCategory {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for MobCategory {
    fn get_serialized_name(&self) -> &str {
        MobCategory::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<MobCategory> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
