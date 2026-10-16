//! Port of: net/minecraft/world/level/block/state/properties/RailShape.java
//! Java class(es): net.minecraft.world.level.block.state.properties.RailShape
//! Status: VERIFIED
//!
//! Session 19. Ten constants carrying a `String name`. Every member ported: `getName`,
//! `toString`, `isSlope`, `getSerializedName`. Ordinals and the `name` field of every constant
//! are checked against the jar (`session19_a.rs`).
//!
//! `getName()`, `toString()` and `getSerializedName()` all return the same `name` field.
//! `isSlope()` is an explicit four-way equality test on the `ASCENDING_*` constants (ordinals
//! 2..=5), transcribed as written rather than as an ordinal range.

/// Port of `net.minecraft.world.level.block.state.properties.RailShape`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RailShape {
    /// `NORTH_SOUTH` -- ordinal 0, `"north_south"`.
    NorthSouth = 0,
    /// `EAST_WEST` -- ordinal 1, `"east_west"`.
    EastWest = 1,
    /// `ASCENDING_EAST` -- ordinal 2, `"ascending_east"`.
    AscendingEast = 2,
    /// `ASCENDING_WEST` -- ordinal 3, `"ascending_west"`.
    AscendingWest = 3,
    /// `ASCENDING_NORTH` -- ordinal 4, `"ascending_north"`.
    AscendingNorth = 4,
    /// `ASCENDING_SOUTH` -- ordinal 5, `"ascending_south"`.
    AscendingSouth = 5,
    /// `SOUTH_EAST` -- ordinal 6, `"south_east"`.
    SouthEast = 6,
    /// `SOUTH_WEST` -- ordinal 7, `"south_west"`.
    SouthWest = 7,
    /// `NORTH_WEST` -- ordinal 8, `"north_west"`.
    NorthWest = 8,
    /// `NORTH_EAST` -- ordinal 9, `"north_east"`.
    NorthEast = 9,
}

/// Java: `RailShape.values()`, declaration order.
pub const VALUES: [RailShape; 10] = [
    RailShape::NorthSouth,
    RailShape::EastWest,
    RailShape::AscendingEast,
    RailShape::AscendingWest,
    RailShape::AscendingNorth,
    RailShape::AscendingSouth,
    RailShape::SouthEast,
    RailShape::SouthWest,
    RailShape::NorthWest,
    RailShape::NorthEast,
];

impl RailShape {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            RailShape::NorthSouth => "NORTH_SOUTH",
            RailShape::EastWest => "EAST_WEST",
            RailShape::AscendingEast => "ASCENDING_EAST",
            RailShape::AscendingWest => "ASCENDING_WEST",
            RailShape::AscendingNorth => "ASCENDING_NORTH",
            RailShape::AscendingSouth => "ASCENDING_SOUTH",
            RailShape::SouthEast => "SOUTH_EAST",
            RailShape::SouthWest => "SOUTH_WEST",
            RailShape::NorthWest => "NORTH_WEST",
            RailShape::NorthEast => "NORTH_EAST",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `RailShape#getName()` -- the private `name` field.
    #[inline]
    pub const fn get_name(&self) -> &'static str {
        match self {
            RailShape::NorthSouth => "north_south",
            RailShape::EastWest => "east_west",
            RailShape::AscendingEast => "ascending_east",
            RailShape::AscendingWest => "ascending_west",
            RailShape::AscendingNorth => "ascending_north",
            RailShape::AscendingSouth => "ascending_south",
            RailShape::SouthEast => "south_east",
            RailShape::SouthWest => "south_west",
            RailShape::NorthWest => "north_west",
            RailShape::NorthEast => "north_east",
        }
    }

    /// Port of `RailShape#isSlope()`:
    /// `this == ASCENDING_NORTH || this == ASCENDING_EAST || this == ASCENDING_SOUTH || this == ASCENDING_WEST`.
    #[inline]
    pub const fn is_slope(&self) -> bool {
        matches!(
            self,
            RailShape::AscendingNorth
                | RailShape::AscendingEast
                | RailShape::AscendingSouth
                | RailShape::AscendingWest
        )
    }

    /// Port of `RailShape#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        self.get_name()
    }
}

impl std::fmt::Display for RailShape {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for RailShape {
    fn get_serialized_name(&self) -> &str {
        RailShape::get_serialized_name(self)
    }
}
