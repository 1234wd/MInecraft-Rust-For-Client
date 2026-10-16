//! Port of: net/minecraft/world/level/levelgen/structure/TerrainAdjustment.java
//! Java class(es): net.minecraft.world.level.levelgen.structure.TerrainAdjustment
//! Status: PARTIAL
//!
//! Session 19. The five constants and `getSerializedName()` are ported; ordinals and the `id`
//! field are checked against the jar in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec`).

/// Port of `net.minecraft.world.level.levelgen.structure.TerrainAdjustment`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainAdjustment {
    /// Port of `TerrainAdjustment#NONE` -- ordinal 0, id `"none"`.
    None = 0,
    /// Port of `TerrainAdjustment#BURY` -- ordinal 1, id `"bury"`.
    Bury = 1,
    /// Port of `TerrainAdjustment#BEARD_THIN` -- ordinal 2, id `"beard_thin"`.
    BeardThin = 2,
    /// Port of `TerrainAdjustment#BEARD_BOX` -- ordinal 3, id `"beard_box"`.
    BeardBox = 3,
    /// Port of `TerrainAdjustment#ENCAPSULATE` -- ordinal 4, id `"encapsulate"`.
    Encapsulate = 4,
}

/// Java: `TerrainAdjustment.values()` -- declaration order.
pub const VALUES: [TerrainAdjustment; 5] = [
    TerrainAdjustment::None,
    TerrainAdjustment::Bury,
    TerrainAdjustment::BeardThin,
    TerrainAdjustment::BeardBox,
    TerrainAdjustment::Encapsulate,
];

impl TerrainAdjustment {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            TerrainAdjustment::None => "NONE",
            TerrainAdjustment::Bury => "BURY",
            TerrainAdjustment::BeardThin => "BEARD_THIN",
            TerrainAdjustment::BeardBox => "BEARD_BOX",
            TerrainAdjustment::Encapsulate => "ENCAPSULATE",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `TerrainAdjustment#getSerializedName()` -- the private `id` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            TerrainAdjustment::None => "none",
            TerrainAdjustment::Bury => "bury",
            TerrainAdjustment::BeardThin => "beard_thin",
            TerrainAdjustment::BeardBox => "beard_box",
            TerrainAdjustment::Encapsulate => "encapsulate",
        }
    }
}

impl std::fmt::Display for TerrainAdjustment {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for TerrainAdjustment {
    fn get_serialized_name(&self) -> &str {
        TerrainAdjustment::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<TerrainAdjustment> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
