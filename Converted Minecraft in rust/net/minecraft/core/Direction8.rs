//! Port of: net/minecraft/core/Direction8.java
//! Java class(es): net.minecraft.core.Direction8
//! Status: VERIFIED
//!
//! Session 19. Eight compass constants, each built from one or two horizontal `Direction`s, with
//! the private `directions` set and `step` vector and the getters `getDirections`, `getStepX`,
//! `getStepZ`. Names, ordinals, the `directions` set and the `step` vector are checked against
//! the jar (via their Java `toString()` forms) in `_porting/tests/session19_b.rs`.
//!
//! # THE SET ITERATES IN `Direction` ORDINAL ORDER
//!
//! `Sets.immutableEnumSet` iterates in enum ordinal order, not constructor-argument order. For
//! all eight constants the two coincide (`NORTH`/`SOUTH` precede `WEST`/`EAST` in `Direction`),
//! so [`Direction8::get_directions`] returns the constructor order, which is also the set order.
//!
//! # `step` IS THE SUM OF THE UNIT STEPS
//!
//! The constructor mutates a fresh `Vec3i(0, 0, 0)` once per direction. The port computes the
//! same sum in a `const fn`. `y` is always `0`; Java exposes only `getStepX` and `getStepZ`.

use crate::net::minecraft::core::Direction::Direction;
use crate::net::minecraft::core::Vec3i::Vec3i;

/// Port of `net.minecraft.core.Direction8`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction8 {
    /// Port of `Direction8#NORTH` -- ordinal 0, `{NORTH}`, step `(0, 0, -1)`.
    North = 0,
    /// Port of `Direction8#NORTH_EAST` -- ordinal 1, `{NORTH, EAST}`, step `(1, 0, -1)`.
    NorthEast = 1,
    /// Port of `Direction8#EAST` -- ordinal 2, `{EAST}`, step `(1, 0, 0)`.
    East = 2,
    /// Port of `Direction8#SOUTH_EAST` -- ordinal 3, `{SOUTH, EAST}`, step `(1, 0, 1)`.
    SouthEast = 3,
    /// Port of `Direction8#SOUTH` -- ordinal 4, `{SOUTH}`, step `(0, 0, 1)`.
    South = 4,
    /// Port of `Direction8#SOUTH_WEST` -- ordinal 5, `{SOUTH, WEST}`, step `(-1, 0, 1)`.
    SouthWest = 5,
    /// Port of `Direction8#WEST` -- ordinal 6, `{WEST}`, step `(-1, 0, 0)`.
    West = 6,
    /// Port of `Direction8#NORTH_WEST` -- ordinal 7, `{NORTH, WEST}`, step `(-1, 0, -1)`.
    NorthWest = 7,
}

/// Java: `Direction8.values()` -- declaration order (clockwise from north).
pub const VALUES: [Direction8; 8] = [
    Direction8::North,
    Direction8::NorthEast,
    Direction8::East,
    Direction8::SouthEast,
    Direction8::South,
    Direction8::SouthWest,
    Direction8::West,
    Direction8::NorthWest,
];

impl Direction8 {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Direction8::North => "NORTH",
            Direction8::NorthEast => "NORTH_EAST",
            Direction8::East => "EAST",
            Direction8::SouthEast => "SOUTH_EAST",
            Direction8::South => "SOUTH",
            Direction8::SouthWest => "SOUTH_WEST",
            Direction8::West => "WEST",
            Direction8::NorthWest => "NORTH_WEST",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Direction8#getDirections()` -- the immutable enum set, in iteration order
    /// (ascending `Direction` ordinal).
    pub const fn get_directions(&self) -> &'static [Direction] {
        match self {
            Direction8::North => &[Direction::North],
            Direction8::NorthEast => &[Direction::North, Direction::East],
            Direction8::East => &[Direction::East],
            Direction8::SouthEast => &[Direction::South, Direction::East],
            Direction8::South => &[Direction::South],
            Direction8::SouthWest => &[Direction::South, Direction::West],
            Direction8::West => &[Direction::West],
            Direction8::NorthWest => &[Direction::North, Direction::West],
        }
    }

    /// The private `step` field: the sum of the directions' unit steps, as the constructor
    /// computes it. No Java getter returns the vector; exposed read-only for parity tests.
    pub const fn step(&self) -> Vec3i {
        let directions = self.get_directions();
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        let mut i = 0;
        while i < directions.len() {
            x += directions[i].get_step_x();
            y += directions[i].get_step_y();
            z += directions[i].get_step_z();
            i += 1;
        }
        Vec3i::new(x, y, z)
    }

    /// Port of `Direction8#getStepX()`.
    #[inline]
    pub const fn get_step_x(&self) -> i32 {
        self.step().get_x()
    }

    /// Port of `Direction8#getStepZ()`.
    #[inline]
    pub const fn get_step_z(&self) -> i32 {
        self.step().get_z()
    }
}

impl std::fmt::Display for Direction8 {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
