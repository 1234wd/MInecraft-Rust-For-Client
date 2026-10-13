//! Port of: net/minecraft/world/level/pathfinder/PathComputationType.java
//! Java class(es): net.minecraft.world.level.pathfinder.PathComputationType
//! Status: VERIFIED
//!
//! Three constants, no fields, no methods. Verified against the jar by `_porting/tests/consts.rs`.
//!
//! # THE NAMES ARE DOMAINS, AND `AIR` IS NOT A SUPERSE'T
//!
//! `LAND`, `WATER`, `AIR` name the three navigation domains the pathfinder distinguishes. They
//! are used as a switch over which movement rules apply, and `AIR` is a peer of the other two
//! rather than a modifier of them -- there is no `LAND_AIR`. A port that introduced one would be
//! adding a concept the game does not have.

/// Port of `net.minecraft.world.level.pathfinder.PathComputationType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PathComputationType {
    /// Port of `PathComputationType#LAND` -- ordinal 0.
    Land = 0,
    /// Port of `PathComputationType#WATER` -- ordinal 1.
    Water = 1,
    /// Port of `PathComputationType#AIR` -- ordinal 2.
    Air = 2,
}

/// Java: `PathComputationType.values()`.
pub const VALUES: [PathComputationType; 3] = [
    PathComputationType::Land,
    PathComputationType::Water,
    PathComputationType::Air,
];