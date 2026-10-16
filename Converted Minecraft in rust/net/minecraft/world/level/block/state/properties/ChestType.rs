//! Port of: net/minecraft/world/level/block/state/properties/ChestType.java
//! Java class(es): net.minecraft.world.level.block.state.properties.ChestType
//! Status: PARTIAL
//!
//! Session 19. Three constants carrying a `String name`, plus `getOpposite()`. Ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `getOpposite()` swaps `LEFT`/`RIGHT` and maps `SINGLE` to itself. No `toString()` override:
//! `Display` is `name()` (`SINGLE`), not `single`.
//!
//! PORT-BLOCKED: `CODEC` (`StringRepresentable.fromEnum(ChestType::values)`) needs DFU's
//! `com.mojang.serialization.Codec`. Declared as a plain `Codec<ChestType>`, so `byName` is not
//! reachable through it from Java; no `by_name` is exposed here.

/// Port of `net.minecraft.world.level.block.state.properties.ChestType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChestType {
    /// `SINGLE` -- ordinal 0, `"single"`.
    Single = 0,
    /// `LEFT` -- ordinal 1, `"left"`.
    Left = 1,
    /// `RIGHT` -- ordinal 2, `"right"`.
    Right = 2,
}

/// Java: `ChestType.values()`, declaration order.
pub const VALUES: [ChestType; 3] = [ChestType::Single, ChestType::Left, ChestType::Right];

impl ChestType {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            ChestType::Single => "SINGLE",
            ChestType::Left => "LEFT",
            ChestType::Right => "RIGHT",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ChestType#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            ChestType::Single => "single",
            ChestType::Left => "left",
            ChestType::Right => "right",
        }
    }

    /// Port of `ChestType#getOpposite()`.
    #[inline]
    pub const fn get_opposite(&self) -> ChestType {
        match self {
            ChestType::Single => ChestType::Single,
            ChestType::Left => ChestType::Right,
            ChestType::Right => ChestType::Left,
        }
    }
}

impl std::fmt::Display for ChestType {
    /// Java's `Enum#toString()` (not overridden), i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for ChestType {
    fn get_serialized_name(&self) -> &str {
        ChestType::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<ChestType> CODEC = StringRepresentable.fromEnum(ChestType::values)`.
///
/// `com.mojang.serialization.Codec` is not ported. See DESIGN_DECISIONS.md (#dfu-proposal).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
