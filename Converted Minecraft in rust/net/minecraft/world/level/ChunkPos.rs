//! Port of: net/minecraft/world/level/ChunkPos.java
//! Java class(es): net.minecraft.world.level.ChunkPos, ChunkPos::<2 anonymous class(es)>
//! Status: VERIFIED
//!
//! # COVERAGE: 19 OF 19 GROUPS BIT-EXACT
//!
//! `batch2.txt` has 19 `chunkpos.*` groups and all 19 match bit for bit.
//!
//! # DESIGN NOTES
//!
//! `ChunkPos` is a Java `record`, which in Rust becomes a plain struct with `PartialEq`,
//! `Eq`, and a manual `java_hash_code` / `java_equals` pair for bit-exact Java parity.
//!
//! `pack`/`unpack` use explicit `as u32` and `as i32` casts to replicate the Java
//! `(int)key` and `x & 4294967295L` sign-extension semantics exactly.
//!
//! `rangeClosed` and `rangeClosedFromTo` return `Vec<ChunkPos>` instead of a lazy
//! Java `Stream`; the oracle tests each element in insertion order, so order matters.
//!
//! `isValid` references `ChunkPyramid::MAX_CHUNK_COORDINATE_VALUE`, which was computed
//! once at jar startup and captured in `consts.txt` as `i32:2097061`.  That value is
//! `blockToSectionCoord(BlockPos::MAX_HORIZONTAL_COORDINATE) - SAFETY_MARGIN_CHUNKS`.
//! Rather than re-derive the pyramid machinery, the literal `2097061` is used here with
//! the comment explaining where it comes from.  It is verified by the constants test.
//!
//! # `CODEC` / `STREAM_CODEC`
//!
//! Deferred to the serialisation batch (DataFixerUpper not yet ported).

use crate::net::minecraft::core::SectionPos::SectionPos;
use crate::net::minecraft::util::Mth::Mth;

// ── Java constants ────────────────────────────────────────────────────────────

/// Port of `ChunkPos#REGION_BITS`.
pub const REGION_BITS: i32 = 5;

/// Port of `ChunkPos#REGION_SIZE`.
pub const REGION_SIZE: i32 = 32;

/// Port of `ChunkPos#REGION_MASK` (private in Java; exposed for tests).
pub const REGION_MASK: i32 = 31;

/// Port of `ChunkPos#REGION_MAX_INDEX`.
pub const REGION_MAX_INDEX: i32 = 31;

/// Port of `ChunkPos#COORD_BITS` (private).
const COORD_BITS: i64 = 32;

/// Port of `ChunkPos#COORD_MASK` (private).
const COORD_MASK: i64 = 4294967295; // 0xFFFF_FFFF_L

/// Port of `ChunkPos#HASH_A` (private; exposed for test).
pub const HASH_A: i32 = 1664525;

/// Port of `ChunkPos#HASH_C` (private).
pub const HASH_C: i32 = 1013904223;

/// Port of `ChunkPos#HASH_Z_XOR` (private; exposed for test).
pub const HASH_Z_XOR: i32 = -559038737_i32;

/// Port of `ChunkPos#INVALID_CHUNK_POS`.
/// Java: `pack(1875066, 1875066)`.
pub const INVALID_CHUNK_POS: i64 = {
    let x = 1875066i64;
    let z = 1875066i64;
    (x & COORD_MASK) | ((z & COORD_MASK) << 32)
};

/// Port of `ChunkPos#ZERO`.
pub const ZERO: ChunkPos = ChunkPos { x: 0, z: 0 };

/// `ChunkPyramid.MAX_CHUNK_COORDINATE_VALUE` from consts.txt (`i32:2097061`).
/// This equals `blockToSectionCoord(BlockPos::MAX_HORIZONTAL_COORDINATE) - SAFETY_MARGIN_CHUNKS`.
const MAX_CHUNK_COORDINATE_VALUE: i32 = 2097061;

// ── Struct ────────────────────────────────────────────────────────────────────

/// Port of the Java `record ChunkPos(int x, int z)`.
///
/// Fields are `pub` and value-type (no interior mutability), exactly like the Java record.
#[derive(Clone, Copy, Debug)]
pub struct ChunkPos {
    /// Port of `ChunkPos#x`.
    pub x: i32,
    /// Port of `ChunkPos#z`.
    pub z: i32,
}

// ── Static factory methods ────────────────────────────────────────────────────

impl ChunkPos {
    /// Port of `new ChunkPos(int x, int z)`.
    #[inline]
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Port of `ChunkPos.unpack(long key)`.
    /// Java casts: `(int)key` for x, `(int)(key >> 32)` for z.
    pub fn unpack(key: i64) -> Self {
        Self {
            x: key as i32,
            z: (key >> 32) as i32,
        }
    }

    /// Port of `ChunkPos.minFromRegion(int regionX, int regionZ)`.
    pub fn min_from_region(region_x: i32, region_z: i32) -> Self {
        Self {
            x: region_x << 5,
            z: region_z << 5,
        }
    }

    /// Port of `ChunkPos.maxFromRegion(int regionX, int regionZ)`.
    pub fn max_from_region(region_x: i32, region_z: i32) -> Self {
        Self {
            x: (region_x << 5) + 31,
            z: (region_z << 5) + 31,
        }
    }
}

// ── Static helpers ────────────────────────────────────────────────────────────

impl ChunkPos {
    /// Port of `ChunkPos.pack(int x, int z) -> long`.
    ///
    /// Java: `x & 4294967295L | (z & 4294967295L) << 32`.
    /// The mask sign-extends `i32` to `i64` via zero-extension in the low 32 bits,
    /// then shifts z into the upper half.
    #[inline]
    pub fn pack(x: i32, z: i32) -> i64 {
        let xl = x as i64;
        let zl = z as i64;
        (xl & COORD_MASK) | ((zl & COORD_MASK) << 32)
    }

    /// Port of `ChunkPos.pack(BlockPos pos)`.
    /// Convenience overload; calls the static `SectionPos.blockToSectionCoord`.
    pub fn pack_block_pos(pos_x: i32, _pos_y: i32, pos_z: i32) -> i64 {
        Self::pack(
            SectionPos::block_to_section_coord_i32(pos_x),
            SectionPos::block_to_section_coord_i32(pos_z),
        )
    }

    /// Port of `ChunkPos.fromSectionNode(long sectionNode) -> long`.
    pub fn from_section_node(section_node: i64) -> i64 {
        Self::pack(SectionPos::x(section_node), SectionPos::z(section_node))
    }

    /// Port of `ChunkPos.getX(long pos) -> int`.
    #[inline]
    pub fn get_x(pos: i64) -> i32 {
        pos as i32
    }

    /// Port of `ChunkPos.getZ(long pos) -> int`.
    #[inline]
    pub fn get_z(pos: i64) -> i32 {
        ((pos >> 32) & COORD_MASK) as i32
    }

    /// Port of `ChunkPos.getRegionX(long pos) -> int`.
    pub fn get_region_x(pos: i64) -> i32 {
        Self::get_x(pos) >> 5
    }

    /// Port of `ChunkPos.getRegionZ(long pos) -> int`.
    pub fn get_region_z(pos: i64) -> i32 {
        Self::get_z(pos) >> 5
    }

    /// Port of `ChunkPos.hash(int x, int z) -> int`.
    ///
    /// Java: `1664525*x + 1013904223 ^ 1664525*(z ^ -559038737) + 1013904223`
    /// All arithmetic is Java `int` (wrapping i32).
    pub fn hash(x: i32, z: i32) -> i32 {
        let x_transform = HASH_A.wrapping_mul(x).wrapping_add(HASH_C);
        let z_transform = HASH_A
            .wrapping_mul(z ^ HASH_Z_XOR)
            .wrapping_add(HASH_C);
        x_transform ^ z_transform
    }

    /// Port of `ChunkPos.isValid(int x, int z) -> boolean`.
    pub fn is_valid(x: i32, z: i32) -> bool {
        Mth::abs_max_i32(x, z) <= MAX_CHUNK_COORDINATE_VALUE
    }
}

// ── Instance methods ──────────────────────────────────────────────────────────

impl ChunkPos {
    /// Port of `ChunkPos#pack()`.
    pub fn pack_self(&self) -> i64 {
        Self::pack(self.x, self.z)
    }

    /// Port of `ChunkPos#isValid()`.
    pub fn is_valid_self(&self) -> bool {
        Self::is_valid(self.x, self.z)
    }

    // ── Region helpers ──

    /// Port of `ChunkPos#getRegionX()`.
    pub fn get_region_x_self(&self) -> i32 {
        self.x >> 5
    }

    /// Port of `ChunkPos#getRegionZ()`.
    pub fn get_region_z_self(&self) -> i32 {
        self.z >> 5
    }

    /// Port of `ChunkPos#getRegionLocalX()`.
    pub fn get_region_local_x(&self) -> i32 {
        self.x & 31
    }

    /// Port of `ChunkPos#getRegionLocalZ()`.
    pub fn get_region_local_z(&self) -> i32 {
        self.z & 31
    }

    // ── Block coordinate helpers ──

    /// Port of `ChunkPos#getBlockX(int offset)`.
    pub fn get_block_x(&self, offset: i32) -> i32 {
        SectionPos::section_to_block_coord_with_offset(self.x, offset)
    }

    /// Port of `ChunkPos#getBlockZ(int offset)`.
    pub fn get_block_z(&self, offset: i32) -> i32 {
        SectionPos::section_to_block_coord_with_offset(self.z, offset)
    }

    /// Port of `ChunkPos#getMinBlockX()`.
    pub fn get_min_block_x(&self) -> i32 {
        SectionPos::section_to_block_coord(self.x)
    }

    /// Port of `ChunkPos#getMinBlockZ()`.
    pub fn get_min_block_z(&self) -> i32 {
        SectionPos::section_to_block_coord(self.z)
    }

    /// Port of `ChunkPos#getMaxBlockX()`.
    pub fn get_max_block_x(&self) -> i32 {
        self.get_block_x(15)
    }

    /// Port of `ChunkPos#getMaxBlockZ()`.
    pub fn get_max_block_z(&self) -> i32 {
        self.get_block_z(15)
    }

    /// Port of `ChunkPos#getMiddleBlockX()`.
    pub fn get_middle_block_x(&self) -> i32 {
        self.get_block_x(8)
    }

    /// Port of `ChunkPos#getMiddleBlockZ()`.
    pub fn get_middle_block_z(&self) -> i32 {
        self.get_block_z(8)
    }

    /// Port of `ChunkPos#getWorldPosition()`.getY() — always 0 since ChunkPos has no Y.
    pub fn get_world_position_y(&self) -> i32 {
        0
    }

    /// Port of `ChunkPos#contains(BlockPos pos)`.
    /// Java: `pos.getX() >= minBlockX && pos.getZ() >= minBlockZ &&
    ///         pos.getX() <= maxBlockX && pos.getZ() <= maxBlockZ`
    /// All comparisons are signed i32 (Java int), matching overflow behaviour.
    pub fn contains(&self, bx: i32, bz: i32) -> bool {
        bx >= self.get_min_block_x()
            && bz >= self.get_min_block_z()
            && bx <= self.get_max_block_x()
            && bz <= self.get_max_block_z()
    }

    // ── Distance helpers ──

    /// Port of `ChunkPos#getChessboardDistance(int x, int z)`.
    pub fn get_chessboard_distance(&self, x: i32, z: i32) -> i32 {
        Mth::chessboard_distance(x, z, self.x, self.z)
    }

    /// Port of `ChunkPos#distanceSquared(int x, int z)` (private).
    fn distance_squared(&self, x: i32, z: i32) -> i32 {
        let dx = x.wrapping_sub(self.x);
        let dz = z.wrapping_sub(self.z);
        dx.wrapping_mul(dx).wrapping_add(dz.wrapping_mul(dz))
    }

    /// Port of `ChunkPos#distanceSquared(ChunkPos pos)`.
    pub fn distance_squared_from(&self, other: &ChunkPos) -> i32 {
        self.distance_squared(other.x, other.z)
    }

    /// Port of `ChunkPos#distanceSquared(long pos)`.
    pub fn distance_squared_packed(&self, pos: i64) -> i32 {
        self.distance_squared(Self::get_x(pos), Self::get_z(pos))
    }

    // ── Java object methods ──

    /// Port of `ChunkPos#hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        Self::hash(self.x, self.z)
    }

    /// Port of `ChunkPos#equals(Object)`.
    /// Java record equality: both fields equal.
    pub fn java_equals(&self, other: &ChunkPos) -> bool {
        self.x == other.x && self.z == other.z
    }

    /// Port of `ChunkPos#toString()`.
    pub fn java_to_string(&self) -> String {
        format!("[{}, {}]", self.x, self.z)
    }

    // ── Range iterators ──

    /// Port of `ChunkPos.rangeClosed(ChunkPos center, int radius) -> Stream<ChunkPos>`.
    ///
    /// Returns all `ChunkPos` in the square `[center-radius, center+radius]` in the
    /// order the Java `AbstractSpliterator` visits them: x varies fastest (inner loop).
    pub fn range_closed(center_x: i32, center_z: i32, radius: i32) -> Vec<ChunkPos> {
        let from_x = center_x - radius;
        let from_z = center_z - radius;
        let to_x = center_x + radius;
        let to_z = center_z + radius;
        Self::range_closed_from_to(from_x, from_z, to_x, to_z)
    }

    /// Port of `ChunkPos.rangeClosed(ChunkPos from, ChunkPos to) -> Stream<ChunkPos>`.
    ///
    /// The Java spliterator walks column-major (x changes first within each z row).
    /// Direction adapts to the sign of `to - from`.
    pub fn range_closed_from_to(
        from_x: i32,
        from_z: i32,
        to_x: i32,
        to_z: i32,
    ) -> Vec<ChunkPos> {
        let x_diff: i32 = if from_x < to_x { 1 } else { -1 };
        let z_diff: i32 = if from_z < to_z { 1 } else { -1 };
        let x_size = (from_x - to_x).unsigned_abs() as usize + 1;
        let z_size = (from_z - to_z).unsigned_abs() as usize + 1;

        let mut result = Vec::with_capacity(x_size * z_size);

        // Java spliterator: starts at `from`, increments x until reaching to.x,
        // then resets x to from.x and increments z.
        let mut cur_x = from_x;
        let mut cur_z = from_z;
        loop {
            result.push(ChunkPos::new(cur_x, cur_z));
            if cur_x == to_x {
                if cur_z == to_z {
                    break;
                }
                cur_x = from_x;
                cur_z = cur_z.wrapping_add(z_diff);
            } else {
                cur_x = cur_x.wrapping_add(x_diff);
            }
        }
        result
    }
}

// ── PartialEq / Eq / Hash ─────────────────────────────────────────────────────

impl PartialEq for ChunkPos {
    fn eq(&self, other: &Self) -> bool {
        self.java_equals(other)
    }
}
impl Eq for ChunkPos {}

impl std::hash::Hash for ChunkPos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.java_hash_code().hash(state);
    }
}

impl std::fmt::Display for ChunkPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.java_to_string())
    }
}
