//! Port of: net/minecraft/world/level/chunk/status/ChunkType.java
//! Java class(es): net.minecraft.world.level.chunk.status.ChunkType
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A two-constant enum pairing each chunk storage tier with the protochunk that produces it.
//! `PROTOCHUNK` is the generation-time representation and `LEVELCHUNK` the loaded one; the
//! declaration order is the ordinal and is used when the chunk-status registry is ordered.

/// Port of `net.minecraft.world.level.chunk.status.ChunkType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChunkType {
    /// Port of `ChunkType#PROTOCHUNK` -- ordinal 0.
    ProtoChunk = 0,
    /// Port of `ChunkType#LEVELCHUNK` -- ordinal 1.
    LevelChunk = 1,
}

/// Java: `ChunkType.values()`.
pub const VALUES: [ChunkType; 2] = [ChunkType::ProtoChunk, ChunkType::LevelChunk];