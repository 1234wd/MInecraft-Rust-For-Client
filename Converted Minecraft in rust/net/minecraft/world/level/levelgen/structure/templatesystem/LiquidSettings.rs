//! Port of: net/minecraft/world/level/levelgen/structure/templatesystem/LiquidSettings.java
//! Java class(es): net.minecraft.world.level.levelgen.structure.templatesystem.LiquidSettings
//! Status: PARTIAL
//!
//! Session 19. Both constants and `getSerializedName()` are ported; ordinals and the `name`
//! field are checked against the jar in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED: `CODEC` (`StringRepresentable.fromValues`, a bare DataFixerUpper `Codec`).
//! Note it is `fromValues`, not `fromEnum`: the Java field is a plain `Codec`, so there is no
//! `EnumCodec.byName` to expose.

/// Port of `net.minecraft.world.level.levelgen.structure.templatesystem.LiquidSettings`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LiquidSettings {
    /// Port of `LiquidSettings#IGNORE_WATERLOGGING` -- ordinal 0, name `"ignore_waterlogging"`.
    IgnoreWaterlogging = 0,
    /// Port of `LiquidSettings#APPLY_WATERLOGGING` -- ordinal 1, name `"apply_waterlogging"`.
    ApplyWaterlogging = 1,
}

/// Java: `LiquidSettings.values()` -- declaration order.
pub const VALUES: [LiquidSettings; 2] = [
    LiquidSettings::IgnoreWaterlogging,
    LiquidSettings::ApplyWaterlogging,
];

impl LiquidSettings {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            LiquidSettings::IgnoreWaterlogging => "IGNORE_WATERLOGGING",
            LiquidSettings::ApplyWaterlogging => "APPLY_WATERLOGGING",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `LiquidSettings#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            LiquidSettings::IgnoreWaterlogging => "ignore_waterlogging",
            LiquidSettings::ApplyWaterlogging => "apply_waterlogging",
        }
    }
}

impl std::fmt::Display for LiquidSettings {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for LiquidSettings {
    fn get_serialized_name(&self) -> &str {
        LiquidSettings::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<LiquidSettings> CODEC = StringRepresentable.fromValues(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
