//! Port of: net/minecraft/core/FrontAndTop.java
//! Java class(es): net.minecraft.core.FrontAndTop
//! Status: VERIFIED
//!
//! Session 19. All 12 constants with `name`, `front` and `top`, `getSerializedName()`,
//! `front()`, `top()`, `fromFrontAndTop(Direction, Direction)` and the private `NUM_DIRECTIONS`,
//! `BY_TOP_FRONT` and `lookupKey`. Nothing is PORT-BLOCKED (the class has no codec). Ordinals
//! and all three fields are checked against the jar, and the lookup table is tested over all
//! 36 direction pairs, in `_porting/tests/session19_c.rs`.
//!
//! # `Util.make` IS NOT A DEPENDENCY HERE
//!
//! `BY_TOP_FRONT = Util.make(new FrontAndTop[36], map -> { for (v : values()) map[lookupKey(v.front, v.top)] = v; })`.
//! `Util.make` just runs the lambda on the array and returns it, so the table is built directly
//! below (a `const` loop over `VALUES`, later entries overwriting earlier ones as in Java --
//! though no two constants share a key).
//!
//! # `fromFrontAndTop` RETURNS `null` FOR 24 OF 36 PAIRS
//!
//! Only the 12 combinations that exist as constants are filled: `DOWN`/`UP` fronts with a
//! horizontal top, and horizontal fronts with `UP` top. Anything else (`NORTH`/`DOWN`,
//! `NORTH`/`EAST`, `UP`/`UP`, ...) reads an empty slot and Java returns `null`; here `None`.
//! The key is `front.ordinal() * 6 + top.ordinal()` -- despite the field being named
//! `BY_TOP_FRONT`, front is the major index.

use crate::net::minecraft::core::Direction::{Direction, VALUES as DIRECTION_VALUES};

/// Port of `net.minecraft.core.FrontAndTop`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FrontAndTop {
    /// Port of `FrontAndTop#DOWN_EAST` -- ordinal 0, front `DOWN`, top `EAST`.
    DownEast = 0,
    /// Port of `FrontAndTop#DOWN_NORTH` -- ordinal 1, front `DOWN`, top `NORTH`.
    DownNorth = 1,
    /// Port of `FrontAndTop#DOWN_SOUTH` -- ordinal 2, front `DOWN`, top `SOUTH`.
    DownSouth = 2,
    /// Port of `FrontAndTop#DOWN_WEST` -- ordinal 3, front `DOWN`, top `WEST`.
    DownWest = 3,
    /// Port of `FrontAndTop#UP_EAST` -- ordinal 4, front `UP`, top `EAST`.
    UpEast = 4,
    /// Port of `FrontAndTop#UP_NORTH` -- ordinal 5, front `UP`, top `NORTH`.
    UpNorth = 5,
    /// Port of `FrontAndTop#UP_SOUTH` -- ordinal 6, front `UP`, top `SOUTH`.
    UpSouth = 6,
    /// Port of `FrontAndTop#UP_WEST` -- ordinal 7, front `UP`, top `WEST`.
    UpWest = 7,
    /// Port of `FrontAndTop#WEST_UP` -- ordinal 8, front `WEST`, top `UP`.
    WestUp = 8,
    /// Port of `FrontAndTop#EAST_UP` -- ordinal 9, front `EAST`, top `UP`.
    EastUp = 9,
    /// Port of `FrontAndTop#NORTH_UP` -- ordinal 10, front `NORTH`, top `UP`.
    NorthUp = 10,
    /// Port of `FrontAndTop#SOUTH_UP` -- ordinal 11, front `SOUTH`, top `UP`.
    SouthUp = 11,
}

/// Java: `FrontAndTop.values()` -- declaration order.
pub const VALUES: [FrontAndTop; 12] = [
    FrontAndTop::DownEast,
    FrontAndTop::DownNorth,
    FrontAndTop::DownSouth,
    FrontAndTop::DownWest,
    FrontAndTop::UpEast,
    FrontAndTop::UpNorth,
    FrontAndTop::UpSouth,
    FrontAndTop::UpWest,
    FrontAndTop::WestUp,
    FrontAndTop::EastUp,
    FrontAndTop::NorthUp,
    FrontAndTop::SouthUp,
];

/// Java: `private static final int NUM_DIRECTIONS = Direction.values().length` (6).
const NUM_DIRECTIONS: i32 = DIRECTION_VALUES.len() as i32;

/// Java: `private static final FrontAndTop[] BY_TOP_FRONT` -- `null` slots are `None`.
static BY_TOP_FRONT: [Option<FrontAndTop>; (NUM_DIRECTIONS * NUM_DIRECTIONS) as usize] = {
    let mut map: [Option<FrontAndTop>; (NUM_DIRECTIONS * NUM_DIRECTIONS) as usize] =
        [None; (NUM_DIRECTIONS * NUM_DIRECTIONS) as usize];
    let mut i = 0;
    while i < VALUES.len() {
        let value = VALUES[i];
        map[lookup_key(value.front(), value.top()) as usize] = Some(value);
        i += 1;
    }
    map
};

/// Port of the private `FrontAndTop#lookupKey(Direction, Direction)`:
/// `front.ordinal() * NUM_DIRECTIONS + top.ordinal()`.
#[inline]
const fn lookup_key(front: Direction, top: Direction) -> i32 {
    front.ordinal() * NUM_DIRECTIONS + top.ordinal()
}

impl FrontAndTop {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            FrontAndTop::DownEast => "DOWN_EAST",
            FrontAndTop::DownNorth => "DOWN_NORTH",
            FrontAndTop::DownSouth => "DOWN_SOUTH",
            FrontAndTop::DownWest => "DOWN_WEST",
            FrontAndTop::UpEast => "UP_EAST",
            FrontAndTop::UpNorth => "UP_NORTH",
            FrontAndTop::UpSouth => "UP_SOUTH",
            FrontAndTop::UpWest => "UP_WEST",
            FrontAndTop::WestUp => "WEST_UP",
            FrontAndTop::EastUp => "EAST_UP",
            FrontAndTop::NorthUp => "NORTH_UP",
            FrontAndTop::SouthUp => "SOUTH_UP",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `FrontAndTop#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            FrontAndTop::DownEast => "down_east",
            FrontAndTop::DownNorth => "down_north",
            FrontAndTop::DownSouth => "down_south",
            FrontAndTop::DownWest => "down_west",
            FrontAndTop::UpEast => "up_east",
            FrontAndTop::UpNorth => "up_north",
            FrontAndTop::UpSouth => "up_south",
            FrontAndTop::UpWest => "up_west",
            FrontAndTop::WestUp => "west_up",
            FrontAndTop::EastUp => "east_up",
            FrontAndTop::NorthUp => "north_up",
            FrontAndTop::SouthUp => "south_up",
        }
    }

    /// Port of `FrontAndTop#front()`.
    #[inline]
    pub const fn front(&self) -> Direction {
        match self {
            FrontAndTop::DownEast
            | FrontAndTop::DownNorth
            | FrontAndTop::DownSouth
            | FrontAndTop::DownWest => Direction::Down,
            FrontAndTop::UpEast | FrontAndTop::UpNorth | FrontAndTop::UpSouth | FrontAndTop::UpWest => {
                Direction::Up
            }
            FrontAndTop::WestUp => Direction::West,
            FrontAndTop::EastUp => Direction::East,
            FrontAndTop::NorthUp => Direction::North,
            FrontAndTop::SouthUp => Direction::South,
        }
    }

    /// Port of `FrontAndTop#top()`.
    #[inline]
    pub const fn top(&self) -> Direction {
        match self {
            FrontAndTop::DownEast | FrontAndTop::UpEast => Direction::East,
            FrontAndTop::DownNorth | FrontAndTop::UpNorth => Direction::North,
            FrontAndTop::DownSouth | FrontAndTop::UpSouth => Direction::South,
            FrontAndTop::DownWest | FrontAndTop::UpWest => Direction::West,
            FrontAndTop::WestUp | FrontAndTop::EastUp | FrontAndTop::NorthUp | FrontAndTop::SouthUp => {
                Direction::Up
            }
        }
    }
}

impl std::fmt::Display for FrontAndTop {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for FrontAndTop {
    fn get_serialized_name(&self) -> &str {
        FrontAndTop::get_serialized_name(self)
    }
}

/// Port of `FrontAndTop#fromFrontAndTop(Direction, Direction)`.
///
/// `None` is Java's `null` for a pair that is not one of the 12 constants (see the module docs).
#[inline]
pub fn from_front_and_top(front: Direction, top: Direction) -> Option<FrontAndTop> {
    BY_TOP_FRONT[lookup_key(front, top) as usize]
}
