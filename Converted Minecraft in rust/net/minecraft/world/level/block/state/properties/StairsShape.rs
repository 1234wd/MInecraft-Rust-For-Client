//! Port of: net/minecraft/world/level/block/state/properties/StairsShape.java
//! Java class(es): net.minecraft.world.level.block.state.properties.StairsShape
//! Status: VERIFIED
//!
//! Session 19. Five constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to return the `name` field, so `Display` prints `inner_left`.

/// Port of `net.minecraft.world.level.block.state.properties.StairsShape`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StairsShape {
    /// `STRAIGHT` -- ordinal 0, `"straight"`.
    Straight = 0,
    /// `INNER_LEFT` -- ordinal 1, `"inner_left"`.
    InnerLeft = 1,
    /// `INNER_RIGHT` -- ordinal 2, `"inner_right"`.
    InnerRight = 2,
    /// `OUTER_LEFT` -- ordinal 3, `"outer_left"`.
    OuterLeft = 3,
    /// `OUTER_RIGHT` -- ordinal 4, `"outer_right"`.
    OuterRight = 4,
}

/// Java: `StairsShape.values()`, declaration order.
pub const VALUES: [StairsShape; 5] = [
    StairsShape::Straight,
    StairsShape::InnerLeft,
    StairsShape::InnerRight,
    StairsShape::OuterLeft,
    StairsShape::OuterRight,
];

impl StairsShape {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            StairsShape::Straight => "STRAIGHT",
            StairsShape::InnerLeft => "INNER_LEFT",
            StairsShape::InnerRight => "INNER_RIGHT",
            StairsShape::OuterLeft => "OUTER_LEFT",
            StairsShape::OuterRight => "OUTER_RIGHT",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `StairsShape#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            StairsShape::Straight => "straight",
            StairsShape::InnerLeft => "inner_left",
            StairsShape::InnerRight => "inner_right",
            StairsShape::OuterLeft => "outer_left",
            StairsShape::OuterRight => "outer_right",
        }
    }
}

impl std::fmt::Display for StairsShape {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for StairsShape {
    fn get_serialized_name(&self) -> &str {
        StairsShape::get_serialized_name(self)
    }
}
