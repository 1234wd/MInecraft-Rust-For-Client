//! Port of: net/minecraft/world/level/levelgen/structure/StructureCheckResult.java
//! Java class(es): net.minecraft.world.level.levelgen.structure.StructureCheckResult
//! Status: VERIFIED
//!
//! Three constants, no fields, no methods. Verified against the jar by `_porting/tests/consts.rs`.
//!
//! # `START_PRESENT` AND `START_NOT_PRESENT` ARE ABOUT **PIECES**, NOT THE STRUCTURE
//!
//! These come out of `StructureCheck#checkStart`, which answers "is the structure's start piece
//! already generated here?" -- and the third value, `CHUNK_LOAD_NEEDED`, is the one that makes
//! the set worth having: it is neither present nor absent, it is *not yet knowable*. A port that
//! collapsed it into `START_NOT_PRESENT` would make the structure generator place a second copy.

/// Port of `net.minecraft.world.level.levelgen.structure.StructureCheckResult`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StructureCheckResult {
    /// Port of `StructureCheckResult#START_PRESENT` -- ordinal 0.
    StartPresent = 0,
    /// Port of `StructureCheckResult#START_NOT_PRESENT` -- ordinal 1.
    StartNotPresent = 1,
    /// Port of `StructureCheckResult#CHUNK_LOAD_NEEDED` -- ordinal 2.
    ChunkLoadNeeded = 2,
}

/// Java: `StructureCheckResult.values()`.
pub const VALUES: [StructureCheckResult; 3] = [
    StructureCheckResult::StartPresent,
    StructureCheckResult::StartNotPresent,
    StructureCheckResult::ChunkLoadNeeded,
];