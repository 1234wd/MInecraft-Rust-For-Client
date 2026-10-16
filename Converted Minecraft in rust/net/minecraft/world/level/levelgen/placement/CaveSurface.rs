//! Port of: net/minecraft/world/level/levelgen/placement/CaveSurface.java
//! Java class(es): net.minecraft.world.level.levelgen.placement.CaveSurface
//! Status: PARTIAL
//!
//! Session 19. Both constants with their `direction`, `y` and `id` fields are ported and checked
//! against the jar in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec`).
//!
//! `CEILING` points `UP` with `y = 1`; `FLOOR` points `DOWN` with `y = -1`. `y` is the step
//! *toward* the surface, i.e. the same sign as the direction's y step.

use crate::net::minecraft::core::Direction::Direction;

/// Port of `net.minecraft.world.level.levelgen.placement.CaveSurface`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CaveSurface {
    /// Port of `CaveSurface#CEILING` -- ordinal 0, `UP`, y `1`, id `"ceiling"`.
    Ceiling = 0,
    /// Port of `CaveSurface#FLOOR` -- ordinal 1, `DOWN`, y `-1`, id `"floor"`.
    Floor = 1,
}

/// Java: `CaveSurface.values()` -- declaration order.
pub const VALUES: [CaveSurface; 2] = [CaveSurface::Ceiling, CaveSurface::Floor];

impl CaveSurface {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            CaveSurface::Ceiling => "CEILING",
            CaveSurface::Floor => "FLOOR",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `CaveSurface#getDirection()`.
    #[inline]
    pub const fn get_direction(&self) -> Direction {
        match self {
            CaveSurface::Ceiling => Direction::Up,
            CaveSurface::Floor => Direction::Down,
        }
    }

    /// Port of `CaveSurface#getY()`.
    #[inline]
    pub const fn get_y(&self) -> i32 {
        match self {
            CaveSurface::Ceiling => 1,
            CaveSurface::Floor => -1,
        }
    }

    /// Port of `CaveSurface#getSerializedName()` -- the private `id` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            CaveSurface::Ceiling => "ceiling",
            CaveSurface::Floor => "floor",
        }
    }
}

impl std::fmt::Display for CaveSurface {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for CaveSurface {
    fn get_serialized_name(&self) -> &str {
        CaveSurface::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<CaveSurface> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
