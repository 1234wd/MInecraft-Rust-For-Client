//! Port of: net/minecraft/core/SectionPos.java
//! Java class(es): net.minecraft.core.SectionPos, SectionPos::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! # COVERAGE: 25 OF 25 GROUPS BIT-EXACT
//!
//! `batch2.txt` has 25 `sectionpos.*` groups and all 25 match bit for bit.
//!
//! # DESIGN NOTES
//!
//! `SectionPos extends Vec3i` in Java. Here we use a plain struct with `x, y, z: i32`
//! fields instead of inheriting from Vec3i, since we only need the coordinates.
//!
//! Java overloading is resolved by name:
//! - Static `int x(long)` / `int y(long)` / `int z(long)` → `SectionPos::x(i64)` etc.
//!   (associated functions, no `&self`)
//! - Instance `int x()` / `int y()` / `int z()` → `self.get_x()` / `self.get_y()` /
//!   `self.get_z()` (field access via methods to avoid name conflict with the statics).
//!
//! `betweenClosedStream`, `cube`, and `aroundChunk` return `Vec<SectionPos>` instead of
//! lazy Java Streams; the oracle tests index into the resulting list, so order matters.
//! The iteration order follows `Cursor3D`: x varies fastest, then y, then z.
//!
//! `aroundAndAtBlockPos` collects section longs into a `Vec<i64>` via a callback closure.
//!
//! `posToSectionCoord` and `blockToSectionCoord(double)` both call
//! `Mth::floor_f64(coord) >> 4` — identical bodies, same arithmetic.
//!
//! # `STREAM_CODEC`
//!
//! Deferred to the serialisation batch (codec machinery not yet ported).

use crate::net::minecraft::core::BlockPos;
use crate::net::minecraft::core::Direction::Direction;
use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::world::level::ChunkPos::ChunkPos;

// ── Public Java constants ─────────────────────────────────────────────────────

/// Port of `SectionPos#SECTION_BITS`.
pub const SECTION_BITS: i32 = 4;

/// Port of `SectionPos#SECTION_SIZE`.
pub const SECTION_SIZE: i32 = 16;

/// Port of `SectionPos#SECTION_BLOCK_COUNT`.
pub const SECTION_BLOCK_COUNT: i32 = 4096;

/// Port of `SectionPos#SECTION_MASK`.
pub const SECTION_MASK: i32 = 15;

/// Port of `SectionPos#SECTION_HALF_SIZE`.
pub const SECTION_HALF_SIZE: i32 = 8;

/// Port of `SectionPos#SECTION_MAX_INDEX`.
pub const SECTION_MAX_INDEX: i32 = 15;

// ── Private layout constants (recovered from behaviour, not decompiled) ───────

const PACKED_X_LENGTH: i32 = 22;
const PACKED_Y_LENGTH: i32 = 20;
const PACKED_Z_LENGTH: i32 = 22;

const PACKED_X_MASK: i64 = 4_194_303; // (1 << 22) - 1
const PACKED_Y_MASK: i64 = 1_048_575; // (1 << 20) - 1
const PACKED_Z_MASK: i64 = 4_194_303; // (1 << 22) - 1

const Y_OFFSET: i32 = 0;
const Z_OFFSET: i32 = 20;
const X_OFFSET: i32 = 42;

const RELATIVE_X_SHIFT: i32 = 8;
const RELATIVE_Y_SHIFT: i32 = 0;
const RELATIVE_Z_SHIFT: i32 = 4;

// ── Struct ────────────────────────────────────────────────────────────────────

/// Port of `SectionPos`.
///
/// Represents a 3-D section coordinate (chunk-column section, 16 blocks on each side).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SectionPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl SectionPos {
    // ── Constructors / factories ──────────────────────────────────────────────

    /// Port of `SectionPos.of(int, int, int)`.
    #[inline]
    pub fn of_xyz(x: i32, y: i32, z: i32) -> Self {
        SectionPos { x, y, z }
    }

    /// Port of `SectionPos.of(long sectionNode)`.
    #[inline]
    pub fn of_long(section_node: i64) -> Self {
        SectionPos {
            x: Self::x(section_node),
            y: Self::y(section_node),
            z: Self::z(section_node),
        }
    }

    /// Port of `SectionPos.of(BlockPos pos)`.
    pub fn of_block_pos(pos: &BlockPos::BlockPos) -> Self {
        SectionPos {
            x: Self::block_to_section_coord_i32(pos.x),
            y: Self::block_to_section_coord_i32(pos.y),
            z: Self::block_to_section_coord_i32(pos.z),
        }
    }

    /// Port of `SectionPos.of(BlockPos pos)` — from raw block coordinates.
    pub fn of_block_xyz(block_x: i32, block_y: i32, block_z: i32) -> Self {
        SectionPos {
            x: Self::block_to_section_coord_i32(block_x),
            y: Self::block_to_section_coord_i32(block_y),
            z: Self::block_to_section_coord_i32(block_z),
        }
    }

    /// Port of `SectionPos.of(Vec3 pos)` — uses `posToSectionCoord` (floor then >> 4).
    pub fn of_vec3(pos_x: f64, pos_y: f64, pos_z: f64) -> Self {
        SectionPos {
            x: Self::pos_to_section_coord(pos_x),
            y: Self::pos_to_section_coord(pos_y),
            z: Self::pos_to_section_coord(pos_z),
        }
    }

    /// Port of `SectionPos.of(ChunkPos pos, int sectionY)`.
    #[inline]
    pub fn of_chunk_pos(pos: &ChunkPos, section_y: i32) -> Self {
        SectionPos {
            x: pos.x,
            y: section_y,
            z: pos.z,
        }
    }

    // ── Static packed-long helpers ────────────────────────────────────────────

    /// Port of `SectionPos.asLong(int x, int y, int z) -> long`.
    ///
    /// Packs section coordinates into a 64-bit long:
    /// `(x & MASK_22) << 42 | (y & MASK_20) << 0 | (z & MASK_22) << 20`.
    #[inline]
    pub fn as_long_xyz(x: i32, y: i32, z: i32) -> i64 {
        let mut node: i64 = 0;
        node |= ((x as i64) & PACKED_X_MASK) << X_OFFSET;
        node |= ((y as i64) & PACKED_Y_MASK) << Y_OFFSET;
        node | (((z as i64) & PACKED_Z_MASK) << Z_OFFSET)
    }

    /// Port of `SectionPos.asLong(BlockPos pos) -> long`.
    pub fn as_long_block_pos(block_x: i32, block_y: i32, block_z: i32) -> i64 {
        Self::as_long_xyz(
            Self::block_to_section_coord_i32(block_x),
            Self::block_to_section_coord_i32(block_y),
            Self::block_to_section_coord_i32(block_z),
        )
    }

    /// Port of static `SectionPos.x(long sectionNode) -> int`.
    ///
    /// Extracts the X section coordinate from a packed long by arithmetic shift:
    /// `(int)(sectionNode << 0 >> 42)`.
    #[inline]
    pub fn x(section_node: i64) -> i32 {
        (section_node << 0 >> 42) as i32
    }

    /// Port of static `SectionPos.y(long sectionNode) -> int`.
    ///
    /// `(int)(sectionNode << 44 >> 44)`.
    #[inline]
    pub fn y(section_node: i64) -> i32 {
        (section_node << 44 >> 44) as i32
    }

    /// Port of static `SectionPos.z(long sectionNode) -> int`.
    ///
    /// `(int)(sectionNode << 22 >> 42)`.
    #[inline]
    pub fn z(section_node: i64) -> i32 {
        (section_node << 22 >> 42) as i32
    }

    /// Port of `SectionPos.offset(long sectionNode, int stepX, int stepY, int stepZ) -> long`.
    #[inline]
    pub fn offset_long(section_node: i64, step_x: i32, step_y: i32, step_z: i32) -> i64 {
        Self::as_long_xyz(
            Self::x(section_node).wrapping_add(step_x),
            Self::y(section_node).wrapping_add(step_y),
            Self::z(section_node).wrapping_add(step_z),
        )
    }

    /// Port of `SectionPos.offset(long sectionNode, Direction offset) -> long`.
    #[inline]
    pub fn offset_long_dir(section_node: i64, dir: &Direction) -> i64 {
        Self::offset_long(
            section_node,
            dir.get_step_x(),
            dir.get_step_y(),
            dir.get_step_z(),
        )
    }

    /// Port of `SectionPos.blockToSection(long blockNode) -> long`.
    ///
    /// Converts a packed block position long to a packed section position long.
    pub fn block_to_section(block_node: i64) -> i64 {
        Self::as_long_xyz(
            Self::block_to_section_coord_i32(BlockPos::get_x(block_node)),
            Self::block_to_section_coord_i32(BlockPos::get_y(block_node)),
            Self::block_to_section_coord_i32(BlockPos::get_z(block_node)),
        )
    }

    /// Port of `SectionPos.getZeroNode(long sectionNode) -> long`.
    ///
    /// Clears the Y bits (bits 0..19): `sectionNode & -1048576L`.
    #[inline]
    pub fn get_zero_node_long(section_node: i64) -> i64 {
        section_node & (-1_048_576_i64)
    }

    /// Port of `SectionPos.getZeroNode(int x, int z) -> long`.
    ///
    /// Packs x/0/z then clears Y bits.
    pub fn get_zero_node_xz(x: i32, z: i32) -> i64 {
        Self::get_zero_node_long(Self::as_long_xyz(x, 0, z))
    }

    /// Port of `SectionPos.sectionToChunk(long sectionNode) -> long`.
    pub fn section_to_chunk(section_node: i64) -> i64 {
        ChunkPos::pack(Self::x(section_node), Self::z(section_node))
    }

    // ── Coordinate conversions (static) ──────────────────────────────────────

    /// Port of `SectionPos.blockToSectionCoord(int blockCoord) -> int`.
    ///
    /// Arithmetic right-shift by 4 (floor-divide by 16).
    #[inline]
    pub fn block_to_section_coord_i32(block_coord: i32) -> i32 {
        block_coord >> 4
    }

    /// Port of `SectionPos.blockToSectionCoord(double coord) -> int`.
    ///
    /// `Mth.floor(coord) >> 4`.
    #[inline]
    pub fn block_to_section_coord_f64(coord: f64) -> i32 {
        Mth::floor_f64(coord) >> 4
    }

    /// Port of `SectionPos.posToSectionCoord(double pos) -> int`.
    ///
    /// Identical to `blockToSectionCoord(double)`.
    #[inline]
    pub fn pos_to_section_coord(pos: f64) -> i32 {
        Mth::floor_f64(pos) >> 4
    }

    /// Port of `SectionPos.sectionRelative(int blockCoord) -> int`.
    ///
    /// `blockCoord & 15` — the position within the section (0..=15).
    #[inline]
    pub fn section_relative(block_coord: i32) -> i32 {
        block_coord & 15
    }

    /// Port of `SectionPos.sectionToBlockCoord(int sectionCoord) -> int`.
    #[inline]
    pub fn section_to_block_coord(section_coord: i32) -> i32 {
        section_coord << 4
    }

    /// Port of `SectionPos.sectionToBlockCoord(int sectionCoord, int offset) -> int`.
    #[inline]
    pub fn section_to_block_coord_with_offset(section_coord: i32, offset: i32) -> i32 {
        (section_coord << 4).wrapping_add(offset)
    }

    /// Port of `SectionPos.sectionRelativePos(BlockPos pos) -> short`.
    ///
    /// Packs section-relative x/y/z into a 16-bit value: `x<<8 | z<<4 | y`.
    /// Returned as `i32` (Java short sign-extends to int).
    pub fn section_relative_pos(block_x: i32, block_y: i32, block_z: i32) -> i32 {
        let rx = Self::section_relative(block_x);
        let ry = Self::section_relative(block_y);
        let rz = Self::section_relative(block_z);
        let raw = (rx << RELATIVE_X_SHIFT) | (rz << RELATIVE_Z_SHIFT) | (ry << RELATIVE_Y_SHIFT);
        // Java stores as `short` then returns it; sign-extend 16 → 32 bits.
        (raw as i16) as i32
    }

    /// Port of `SectionPos.sectionRelativeX(short relative) -> int`.
    ///
    /// `relative >>> 8 & 15` — unsigned right shift.
    #[inline]
    pub fn section_relative_x(relative: i32) -> i32 {
        (relative as u16 as u32 >> 8) as i32 & 15
    }

    /// Port of `SectionPos.sectionRelativeY(short relative) -> int`.
    ///
    /// `relative >>> 0 & 15`.
    #[inline]
    pub fn section_relative_y(relative: i32) -> i32 {
        (relative as u16 as u32) as i32 & 15
    }

    /// Port of `SectionPos.sectionRelativeZ(short relative) -> int`.
    ///
    /// `relative >>> 4 & 15`.
    #[inline]
    pub fn section_relative_z(relative: i32) -> i32 {
        (relative as u16 as u32 >> 4) as i32 & 15
    }

    // ── Instance methods ──────────────────────────────────────────────────────

    /// Port of instance `x()`.
    #[inline]
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Port of instance `y()`.
    #[inline]
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// Port of instance `z()`.
    #[inline]
    pub fn get_z(&self) -> i32 {
        self.z
    }

    /// Port of `SectionPos#minBlockX()`.
    #[inline]
    pub fn min_block_x(&self) -> i32 {
        Self::section_to_block_coord(self.x)
    }

    /// Port of `SectionPos#minBlockY()`.
    #[inline]
    pub fn min_block_y(&self) -> i32 {
        Self::section_to_block_coord(self.y)
    }

    /// Port of `SectionPos#minBlockZ()`.
    #[inline]
    pub fn min_block_z(&self) -> i32 {
        Self::section_to_block_coord(self.z)
    }

    /// Port of `SectionPos#maxBlockX()`.
    #[inline]
    pub fn max_block_x(&self) -> i32 {
        Self::section_to_block_coord_with_offset(self.x, 15)
    }

    /// Port of `SectionPos#maxBlockY()`.
    #[inline]
    pub fn max_block_y(&self) -> i32 {
        Self::section_to_block_coord_with_offset(self.y, 15)
    }

    /// Port of `SectionPos#maxBlockZ()`.
    #[inline]
    pub fn max_block_z(&self) -> i32 {
        Self::section_to_block_coord_with_offset(self.z, 15)
    }

    /// Port of `SectionPos#origin()` → returns (x,y,z) as tuple.
    ///
    /// Java returns `new BlockPos(minBlockX, minBlockY, minBlockZ)`.
    #[inline]
    pub fn origin(&self) -> (i32, i32, i32) {
        (self.min_block_x(), self.min_block_y(), self.min_block_z())
    }

    /// Port of `SectionPos#center()`.
    ///
    /// `origin().offset(8, 8, 8)` = (minBlockX+8, minBlockY+8, minBlockZ+8).
    #[inline]
    pub fn center(&self) -> (i32, i32, i32) {
        (
            self.min_block_x().wrapping_add(8),
            self.min_block_y().wrapping_add(8),
            self.min_block_z().wrapping_add(8),
        )
    }

    /// Port of `SectionPos#chunk()`.
    ///
    /// Returns a `ChunkPos` for the chunk that contains this section.
    #[inline]
    pub fn chunk(&self) -> ChunkPos {
        ChunkPos::new(self.x, self.z)
    }

    /// Port of instance `asLong()`.
    #[inline]
    pub fn as_long(&self) -> i64 {
        Self::as_long_xyz(self.x, self.y, self.z)
    }

    /// Port of `SectionPos.offset(int x, int y, int z)`.
    ///
    /// Returns `self` if all deltas are zero (Java optimisation preserved).
    pub fn offset(&self, dx: i32, dy: i32, dz: i32) -> Self {
        if dx == 0 && dy == 0 && dz == 0 {
            *self
        } else {
            SectionPos {
                x: self.x.wrapping_add(dx),
                y: self.y.wrapping_add(dy),
                z: self.z.wrapping_add(dz),
            }
        }
    }

    /// Port of instance `relativeToBlockX(short relative) -> int`.
    pub fn relative_to_block_x(&self, relative: i32) -> i32 {
        self.min_block_x() + Self::section_relative_x(relative)
    }

    /// Port of instance `relativeToBlockY(short relative) -> int`.
    pub fn relative_to_block_y(&self, relative: i32) -> i32 {
        self.min_block_y() + Self::section_relative_y(relative)
    }

    /// Port of instance `relativeToBlockZ(short relative) -> int`.
    pub fn relative_to_block_z(&self, relative: i32) -> i32 {
        self.min_block_z() + Self::section_relative_z(relative)
    }

    /// Port of instance `relativeToBlockPos(short relative) -> BlockPos`.
    ///
    /// Returns (x, y, z) of the world block position.
    pub fn relative_to_block_pos(&self, relative: i32) -> (i32, i32, i32) {
        (
            self.relative_to_block_x(relative),
            self.relative_to_block_y(relative),
            self.relative_to_block_z(relative),
        )
    }

    // ── Stream-like iteration methods ─────────────────────────────────────────

    /// Port of `SectionPos.blocksInside()`.
    ///
    /// Returns all block positions inside this section, in `betweenClosed` order
    /// (x fastest, then y, then z — Cursor3D order).
    pub fn blocks_inside(&self) -> Vec<(i32, i32, i32)> {
        let mut result = Vec::with_capacity(4096);
        let (min_x, min_y, min_z) = self.origin();
        let (max_x, max_y, max_z) = (self.max_block_x(), self.max_block_y(), self.max_block_z());
        // Cursor3D order: x varies fastest, then y, then z
        let mut x = min_x;
        let mut y = min_y;
        let mut z = min_z;
        loop {
            result.push((x, y, z));
            if x < max_x {
                x += 1;
            } else if y < max_y {
                x = min_x;
                y += 1;
            } else if z < max_z {
                x = min_x;
                y = min_y;
                z += 1;
            } else {
                break;
            }
        }
        result
    }

    /// Port of `SectionPos.betweenClosedStream(int, int, int, int, int, int)`.
    ///
    /// Returns all section positions in the axis-aligned box [min..max], inclusive,
    /// in Cursor3D order (x fastest, then y, then z).
    pub fn between_closed_stream(
        min_x: i32, min_y: i32, min_z: i32,
        max_x: i32, max_y: i32, max_z: i32,
    ) -> Vec<SectionPos> {
        let mut result = Vec::new();
        // Cursor3D: x varies fastest, then y, then z
        let mut x = min_x;
        let mut y = min_y;
        let mut z = min_z;
        loop {
            result.push(SectionPos::of_xyz(x, y, z));
            if x < max_x {
                x += 1;
            } else if y < max_y {
                x = min_x;
                y += 1;
            } else if z < max_z {
                x = min_x;
                y = min_y;
                z += 1;
            } else {
                break;
            }
        }
        result
    }

    /// Port of `SectionPos.cube(SectionPos center, int radius)`.
    pub fn cube(center_x: i32, center_y: i32, center_z: i32, radius: i32) -> Vec<SectionPos> {
        Self::between_closed_stream(
            center_x - radius, center_y - radius, center_z - radius,
            center_x + radius, center_y + radius, center_z + radius,
        )
    }

    /// Port of `SectionPos.aroundChunk(ChunkPos center, int radius, int minSection, int maxSection)`.
    pub fn around_chunk(
        chunk_x: i32, chunk_z: i32, radius: i32, min_section: i32, max_section: i32,
    ) -> Vec<SectionPos> {
        Self::between_closed_stream(
            chunk_x - radius, min_section, chunk_z - radius,
            chunk_x + radius, max_section, chunk_z + radius,
        )
    }

    /// Port of `SectionPos.aroundAndAtBlockPos(int blockX, int blockY, int blockZ, LongConsumer)`.
    ///
    /// Calls `consumer` with each section long that overlaps the block position
    /// (at most 8 sections if the block is at a section boundary, otherwise 1).
    pub fn around_and_at_block_pos(
        block_x: i32, block_y: i32, block_z: i32, consumer: &mut Vec<i64>,
    ) {
        let min_sx = Self::block_to_section_coord_i32(block_x.wrapping_sub(1));
        let max_sx = Self::block_to_section_coord_i32(block_x.wrapping_add(1));
        let min_sy = Self::block_to_section_coord_i32(block_y.wrapping_sub(1));
        let max_sy = Self::block_to_section_coord_i32(block_y.wrapping_add(1));
        let min_sz = Self::block_to_section_coord_i32(block_z.wrapping_sub(1));
        let max_sz = Self::block_to_section_coord_i32(block_z.wrapping_add(1));
        if min_sx == max_sx && min_sy == max_sy && min_sz == max_sz {
            consumer.push(Self::as_long_xyz(min_sx, min_sy, min_sz));
        } else {
            // x-outer, y-middle, z-inner (matching Java triple loop order)
            for sx in min_sx..=max_sx {
                for sy in min_sy..=max_sy {
                    for sz in min_sz..=max_sz {
                        consumer.push(Self::as_long_xyz(sx, sy, sz));
                    }
                }
            }
        }
    }
}
