//! Port of: net/minecraft/world/level/block/state/properties/AttachFace.java
//! Java class(es): net.minecraft.world.level.block.state.properties.AttachFace
//! Status: VERIFIED
//!
//! Session 19. Three constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `AttachFace` does NOT override `toString()`, so `Display` is `name()` (`FLOOR`), unlike most
//! of its sibling property enums, which print the serialized name.

/// Port of `net.minecraft.world.level.block.state.properties.AttachFace`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AttachFace {
    /// `FLOOR` -- ordinal 0, `"floor"`.
    Floor = 0,
    /// `WALL` -- ordinal 1, `"wall"`.
    Wall = 1,
    /// `CEILING` -- ordinal 2, `"ceiling"`.
    Ceiling = 2,
}

/// Java: `AttachFace.values()`, declaration order.
pub const VALUES: [AttachFace; 3] = [AttachFace::Floor, AttachFace::Wall, AttachFace::Ceiling];

impl AttachFace {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            AttachFace::Floor => "FLOOR",
            AttachFace::Wall => "WALL",
            AttachFace::Ceiling => "CEILING",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `AttachFace#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            AttachFace::Floor => "floor",
            AttachFace::Wall => "wall",
            AttachFace::Ceiling => "ceiling",
        }
    }
}

impl std::fmt::Display for AttachFace {
    /// Java's `Enum#toString()` (not overridden), i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for AttachFace {
    fn get_serialized_name(&self) -> &str {
        AttachFace::get_serialized_name(self)
    }
}
