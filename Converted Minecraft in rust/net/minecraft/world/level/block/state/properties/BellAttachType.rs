//! Port of: net/minecraft/world/level/block/state/properties/BellAttachType.java
//! Java class(es): net.minecraft.world.level.block.state.properties.BellAttachType
//! Status: VERIFIED
//!
//! Session 19. Four constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! No `toString()` override: `Display` is `name()` (`SINGLE_WALL`), not `single_wall`.

/// Port of `net.minecraft.world.level.block.state.properties.BellAttachType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BellAttachType {
    /// `FLOOR` -- ordinal 0, `"floor"`.
    Floor = 0,
    /// `CEILING` -- ordinal 1, `"ceiling"`.
    Ceiling = 1,
    /// `SINGLE_WALL` -- ordinal 2, `"single_wall"`.
    SingleWall = 2,
    /// `DOUBLE_WALL` -- ordinal 3, `"double_wall"`.
    DoubleWall = 3,
}

/// Java: `BellAttachType.values()`, declaration order.
pub const VALUES: [BellAttachType; 4] = [
    BellAttachType::Floor,
    BellAttachType::Ceiling,
    BellAttachType::SingleWall,
    BellAttachType::DoubleWall,
];

impl BellAttachType {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            BellAttachType::Floor => "FLOOR",
            BellAttachType::Ceiling => "CEILING",
            BellAttachType::SingleWall => "SINGLE_WALL",
            BellAttachType::DoubleWall => "DOUBLE_WALL",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `BellAttachType#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            BellAttachType::Floor => "floor",
            BellAttachType::Ceiling => "ceiling",
            BellAttachType::SingleWall => "single_wall",
            BellAttachType::DoubleWall => "double_wall",
        }
    }
}

impl std::fmt::Display for BellAttachType {
    /// Java's `Enum#toString()` (not overridden), i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for BellAttachType {
    fn get_serialized_name(&self) -> &str {
        BellAttachType::get_serialized_name(self)
    }
}
