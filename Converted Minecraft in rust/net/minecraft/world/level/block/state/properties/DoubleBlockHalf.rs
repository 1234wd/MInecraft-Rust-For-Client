//! Port of: net/minecraft/world/level/block/state/properties/DoubleBlockHalf.java
//! Java class(es): net.minecraft.world.level.block.state.properties.DoubleBlockHalf
//! Status: VERIFIED
//!
//! Session 19. Two constants carrying a `Direction directionToOther`. Every member ported:
//! `getDirectionToOther`, `getSerializedName`, `getOtherHalf`, the `toString` override.
//!
//! Ordinals and the `directionToOther` field of both constants are checked against the jar
//! (`session19_a.rs`): `UPPER` points `DOWN` to its partner, `LOWER` points `UP`. The serialized
//! names are computed by a ternary rather than stored, so the dump has no column for them;
//! `"upper"` / `"lower"` are transcribed from the source:
//!
//! ```java
//! return this == UPPER ? "upper" : "lower";
//! ```
//!
//! `toString()` is overridden to `getSerializedName()`, so `Display` prints `upper`.

use crate::net::minecraft::core::Direction::Direction;

/// Port of `net.minecraft.world.level.block.state.properties.DoubleBlockHalf`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DoubleBlockHalf {
    /// `UPPER(Direction.DOWN)` -- ordinal 0, serialized `"upper"`.
    Upper = 0,
    /// `LOWER(Direction.UP)` -- ordinal 1, serialized `"lower"`.
    Lower = 1,
}

/// Java: `DoubleBlockHalf.values()`, declaration order.
pub const VALUES: [DoubleBlockHalf; 2] = [DoubleBlockHalf::Upper, DoubleBlockHalf::Lower];

impl DoubleBlockHalf {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            DoubleBlockHalf::Upper => "UPPER",
            DoubleBlockHalf::Lower => "LOWER",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `DoubleBlockHalf#getDirectionToOther()` -- the private `directionToOther` field.
    #[inline]
    pub const fn get_direction_to_other(&self) -> Direction {
        match self {
            DoubleBlockHalf::Upper => Direction::Down,
            DoubleBlockHalf::Lower => Direction::Up,
        }
    }

    /// Port of `DoubleBlockHalf#getSerializedName()`: `this == UPPER ? "upper" : "lower"`.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            DoubleBlockHalf::Upper => "upper",
            DoubleBlockHalf::Lower => "lower",
        }
    }

    /// Port of `DoubleBlockHalf#getOtherHalf()`: `this == UPPER ? LOWER : UPPER`.
    #[inline]
    pub const fn get_other_half(&self) -> DoubleBlockHalf {
        match self {
            DoubleBlockHalf::Upper => DoubleBlockHalf::Lower,
            DoubleBlockHalf::Lower => DoubleBlockHalf::Upper,
        }
    }
}

impl std::fmt::Display for DoubleBlockHalf {
    /// Port of the `toString()` override: `getSerializedName()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for DoubleBlockHalf {
    fn get_serialized_name(&self) -> &str {
        DoubleBlockHalf::get_serialized_name(self)
    }
}
