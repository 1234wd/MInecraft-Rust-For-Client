//! Port of: net/minecraft/world/level/block/state/properties/StructureMode.java
//! Java class(es): net.minecraft.world.level.block.state.properties.StructureMode
//! Status: PARTIAL
//!
//! Session 19. Four constants carrying a `String name` and a `Component displayName`. Ordinals
//! and the `name` field of every constant are checked against the jar (`session19_a.rs`).
//! No `toString()` override: `Display` is `name()` (`SAVE`), not `save`.
//!
//! PORT-BLOCKED:
//! * `getDisplayName()` / the `displayName` field: `Component.translatable(
//!   "structure_block.mode_info." + name)`, needs `net.minecraft.network.chat.Component`.
//! * `LEGACY_CODEC` (`@Deprecated`): `ExtraCodecs.legacyEnum(StructureMode::valueOf)`, needs
//!   `net.minecraft.util.ExtraCodecs` and DFU `Codec`.
//!
//! # `LEGACY_CODEC` DECODES BY CONSTANT NAME
//!
//! It is built on `valueOf`, i.e. the UPPERCASE Java constant name (`"SAVE"`), not on the
//! lowercase serialized name (`"save"`) that block states use. Whoever unblocks it must not
//! route it through `get_serialized_name`.

/// Port of `net.minecraft.world.level.block.state.properties.StructureMode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StructureMode {
    /// `SAVE` -- ordinal 0, `"save"`.
    Save = 0,
    /// `LOAD` -- ordinal 1, `"load"`.
    Load = 1,
    /// `CORNER` -- ordinal 2, `"corner"`.
    Corner = 2,
    /// `DATA` -- ordinal 3, `"data"`.
    Data = 3,
}

/// Java: `StructureMode.values()`, declaration order.
pub const VALUES: [StructureMode; 4] = [
    StructureMode::Save,
    StructureMode::Load,
    StructureMode::Corner,
    StructureMode::Data,
];

impl StructureMode {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            StructureMode::Save => "SAVE",
            StructureMode::Load => "LOAD",
            StructureMode::Corner => "CORNER",
            StructureMode::Data => "DATA",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `StructureMode#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            StructureMode::Save => "save",
            StructureMode::Load => "load",
            StructureMode::Corner => "corner",
            StructureMode::Data => "data",
        }
    }

    /// Java: `public Component getDisplayName()` -- the `displayName` field,
    /// `Component.translatable("structure_block.mode_info." + name)`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_display_name(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }
}

impl std::fmt::Display for StructureMode {
    /// Java's `Enum#toString()` (not overridden), i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for StructureMode {
    fn get_serialized_name(&self) -> &str {
        StructureMode::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `@Deprecated public static final Codec<StructureMode> LEGACY_CODEC =
/// ExtraCodecs.legacyEnum(StructureMode::valueOf)`.
///
/// `net.minecraft.util.ExtraCodecs` (and DFU `Codec`) are not ported.
pub fn LEGACY_CODEC() {
    todo!("PORT-BLOCKED: net.minecraft.util.ExtraCodecs is not ported")
}
