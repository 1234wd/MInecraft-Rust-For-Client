//! Port of: net/minecraft/server/level/ColumnPos.java
//! Java class(es): net.minecraft.server.level.ColumnPos
//! Status: VERIFIED
//!
//! Session 22. `public record ColumnPos(int x, int z)`: a block column, packable into a `long`,
//! that OVERRIDES both `toString` and `hashCode`.
//!
//! # Packing
//!
//! `asLong(x, z)` is `x & 0xFFFFFFFFL | (z & 0xFFFFFFFFL) << 32` -- `x` in the LOW word, `z` in the
//! high one. The masks are what stop a negative `x` sign-extending over `z`.
//! `getZ` uses an UNSIGNED shift (`>>> 32`), so it is `(pos as u64) >> 32`, never `pos >> 32`.
//!
//! # The overrides
//!
//! * `toString` is `"[x, z]"`, not the record's `ColumnPos[x=.., z=..]`.
//! * `hashCode` is `ChunkPos.hash(x, z)`, not the record's `31 * x + z`. That matters wherever a
//!   `ColumnPos` keys a `HashMap`, because it decides iteration order.
//! * `equals` is NOT overridden, so it is the record's component-wise one (derived here).
//!
//! `toChunkPos()` divides each coordinate by 16 rounding down, via
//! `SectionPos.blockToSectionCoord` (`>> 4`).
//!
//! Verified by the `REC`, `RECEQ`, `COLPOS` and `COLPOSRAW` groups of `small_probe.txt`
//! (`tests/session22.rs`): an 11x11 grid of coordinates around the chunk boundaries and the `int`
//! extremes, plus `getX`/`getZ` on raw `long`s that no `asLong` produces.

use crate::net::minecraft::core::SectionPos::SectionPos;
use crate::net::minecraft::world::level::ChunkPos::ChunkPos;

/// Port of `net.minecraft.server.level.ColumnPos`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColumnPos {
    x: i32,
    z: i32,
}

impl ColumnPos {
    /// Port of `private static final long COORD_BITS = 32L`.
    const COORD_BITS: i64 = 32;
    /// Port of `private static final long COORD_MASK = 4294967295L`.
    const COORD_MASK: i64 = 4_294_967_295;

    /// Port of the canonical constructor.
    pub const fn new(x: i32, z: i32) -> Self {
        ColumnPos { x, z }
    }

    /// Port of the accessor `x()`.
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Port of the accessor `z()`.
    pub const fn z(&self) -> i32 {
        self.z
    }

    /// Port of `toChunkPos()`.
    pub fn to_chunk_pos(&self) -> ChunkPos {
        ChunkPos::new(
            SectionPos::block_to_section_coord_i32(self.x),
            SectionPos::block_to_section_coord_i32(self.z),
        )
    }

    /// Port of `toLong()`.
    pub const fn to_long(&self) -> i64 {
        Self::as_long(self.x, self.z)
    }

    /// Port of `asLong(int x, int z)`.
    pub const fn as_long(x: i32, z: i32) -> i64 {
        (x as i64 & Self::COORD_MASK) | ((z as i64 & Self::COORD_MASK) << Self::COORD_BITS)
    }

    /// Port of `getX(long pos)`: `(int) (pos & COORD_MASK)`.
    pub const fn get_x(pos: i64) -> i32 {
        (pos & Self::COORD_MASK) as i32
    }

    /// Port of `getZ(long pos)`: `(int) (pos >>> 32 & COORD_MASK)` -- an unsigned shift.
    pub const fn get_z(pos: i64) -> i32 {
        (((pos as u64) >> Self::COORD_BITS) as i64 & Self::COORD_MASK) as i32
    }

    /// Port of the `hashCode()` override: `ChunkPos.hash(x, z)`.
    pub fn java_hash_code(&self) -> i32 {
        ChunkPos::hash(self.x, self.z)
    }
}

impl std::fmt::Display for ColumnPos {
    /// Port of the `toString()` override: `"[" + x + ", " + z + "]"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.z)
    }
}
