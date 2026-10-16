//! Port of: net/minecraft/client/renderer/texture/MipmapStrategy.java
//! Java class(es): net.minecraft.client.renderer.texture.MipmapStrategy
//! Status: PARTIAL
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`. The `name` field is checked too.
//!
//! `CODEC` is PORT-BLOCKED on DataFixerUpper (`com.mojang.serialization.Codec`); that is the only
//! member not ported, hence `PARTIAL`.

/// Port of `net.minecraft.client.renderer.texture.MipmapStrategy`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MipmapStrategy {
    /// `AUTO` -- ordinal 0.
    Auto = 0,
    /// `MEAN` -- ordinal 1.
    Mean = 1,
    /// `CUTOUT` -- ordinal 2.
    Cutout = 2,
    /// `STRICT_CUTOUT` -- ordinal 3.
    StrictCutout = 3,
    /// `DARK_CUTOUT` -- ordinal 4.
    DarkCutout = 4,
}

/// Java: `MipmapStrategy.values()`, declaration order.
pub const VALUES: [MipmapStrategy; 5] = [
    MipmapStrategy::Auto,
    MipmapStrategy::Mean,
    MipmapStrategy::Cutout,
    MipmapStrategy::StrictCutout,
    MipmapStrategy::DarkCutout,
];

impl MipmapStrategy {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            MipmapStrategy::Auto => "AUTO",
            MipmapStrategy::Mean => "MEAN",
            MipmapStrategy::Cutout => "CUTOUT",
            MipmapStrategy::StrictCutout => "STRICT_CUTOUT",
            MipmapStrategy::DarkCutout => "DARK_CUTOUT",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `MipmapStrategy#getSerializedName()` (the `name` field).
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            MipmapStrategy::Auto => "auto",
            MipmapStrategy::Mean => "mean",
            MipmapStrategy::Cutout => "cutout",
            MipmapStrategy::StrictCutout => "strict_cutout",
            MipmapStrategy::DarkCutout => "dark_cutout",
        }
    }
}

impl std::fmt::Display for MipmapStrategy {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for MipmapStrategy {
    fn get_serialized_name(&self) -> &str {
        MipmapStrategy::get_serialized_name(self)
    }
}

/// Java: `public static final Codec<MipmapStrategy> CODEC = StringRepresentable.fromValues(...)`.
///
/// PORT-BLOCKED: `fromValues` returns a bare DataFixerUpper `Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
