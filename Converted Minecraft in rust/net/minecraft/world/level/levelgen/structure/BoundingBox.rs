//! Port of: net/minecraft/world/level/levelgen/structure/BoundingBox.java
//! Java class(es): net.minecraft.world.level.levelgen.structure.BoundingBox
//! Status: PARTIAL
//!
//! Session 19. Ported: both constructors (including the inverted-bounds repair), `fromCorners`,
//! `infinite`, `orientBox`, `intersectingChunks`, both `intersects`, `encapsulatingPositions`,
//! `encapsulatingBoxes`, both mutating `encapsulate`, `encapsulating`, both mutating `move`,
//! `moved`, both `inflatedBy`, both `isInside`, `getLength`, the three spans, `getCenter`,
//! `forAllCorners` (order preserved), `toString`, `equals`, `hashCode`, and the six accessors.
//! Tested in `_porting/tests/session19_consts.rs`.
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec.INT_STREAM`) and `STREAM_CODEC` (netty).
//!
//! # NOT REPRODUCED: THE LOG LINE FOR INVERTED BOUNDS
//!
//! When `max < min` on any axis the Java constructor calls
//! `Util.logAndPauseIfInIde("Invalid bounding box data, inverted bounds for: " + this)` and then
//! swaps the bounds. The swap is ported exactly. The log call is a side effect on the game log
//! only (it pauses only inside an IDE); the port has no logger, so the message text is available
//! from [`BoundingBox::inverted_bounds_message`] but nothing is printed.
//!
//! # `int` ARITHMETIC WRAPS
//!
//! `move`, `moved`, `inflatedBy`, `orientBox`, the spans and `getCenter` are plain Java `int`
//! arithmetic, so they wrap on overflow -- `infinite().getXSpan()` is `0`
//! (`MAX - MIN + 1` wraps twice). The port uses `wrapping_*` throughout to match.

use crate::net::minecraft::core::BlockPos::BlockPos;
use crate::net::minecraft::core::Direction::Direction;
use crate::net::minecraft::core::SectionPos::SectionPos;
use crate::net::minecraft::core::Vec3i::Vec3i;
use crate::net::minecraft::world::level::ChunkPos::ChunkPos;

/// Port of `net.minecraft.world.level.levelgen.structure.BoundingBox`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoundingBox {
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
}

impl BoundingBox {
    /// Port of `BoundingBox(int, int, int, int, int, int)`.
    pub fn new(min_x: i32, min_y: i32, min_z: i32, max_x: i32, max_y: i32, max_z: i32) -> BoundingBox {
        let raw = BoundingBox { min_x, min_y, min_z, max_x, max_y, max_z };
        if max_x < min_x || max_y < min_y || max_z < min_z {
            // Java logs `inverted_bounds_message()` here; see the module docs.
            BoundingBox {
                min_x: min_x.min(max_x),
                min_y: min_y.min(max_y),
                min_z: min_z.min(max_z),
                max_x: min_x.max(max_x),
                max_y: min_y.max(max_y),
                max_z: min_z.max(max_z),
            }
        } else {
            raw
        }
    }

    /// The text Java logs for inverted input, built from the RAW (pre-swap) bounds.
    pub fn inverted_bounds_message(min_x: i32, min_y: i32, min_z: i32, max_x: i32, max_y: i32, max_z: i32) -> String {
        let raw = BoundingBox { min_x, min_y, min_z, max_x, max_y, max_z };
        format!("Invalid bounding box data, inverted bounds for: {raw}")
    }

    /// Port of `BoundingBox(BlockPos)`: a single-block box.
    pub fn from_block_pos(content: &BlockPos) -> BoundingBox {
        BoundingBox::new(content.get_x(), content.get_y(), content.get_z(), content.get_x(), content.get_y(), content.get_z())
    }

    /// Port of `BoundingBox#fromCorners(Vec3i, Vec3i)`.
    pub fn from_corners(pos0: &Vec3i, pos1: &Vec3i) -> BoundingBox {
        BoundingBox::new(
            pos0.get_x().min(pos1.get_x()),
            pos0.get_y().min(pos1.get_y()),
            pos0.get_z().min(pos1.get_z()),
            pos0.get_x().max(pos1.get_x()),
            pos0.get_y().max(pos1.get_y()),
            pos0.get_z().max(pos1.get_z()),
        )
    }

    /// Port of `BoundingBox#infinite()`.
    pub fn infinite() -> BoundingBox {
        BoundingBox::new(i32::MIN, i32::MIN, i32::MIN, i32::MAX, i32::MAX, i32::MAX)
    }

    /// Port of `BoundingBox#orientBox(int x9, Direction)`.
    ///
    /// `SOUTH` and every non-horizontal direction share the `default` branch, as in Java.
    #[allow(clippy::too_many_arguments)]
    pub fn orient_box(
        foot_x: i32,
        foot_y: i32,
        foot_z: i32,
        off_x: i32,
        off_y: i32,
        off_z: i32,
        width: i32,
        height: i32,
        depth: i32,
        direction: Direction,
    ) -> BoundingBox {
        let a = |p: i32, q: i32| p.wrapping_add(q);
        let s = |p: i32, q: i32| p.wrapping_sub(q);
        match direction {
            Direction::North => BoundingBox::new(
                a(foot_x, off_x),
                a(foot_y, off_y),
                a(a(s(foot_z, depth), 1), off_z),
                a(s(a(foot_x, width), 1), off_x),
                a(s(a(foot_y, height), 1), off_y),
                a(foot_z, off_z),
            ),
            Direction::West => BoundingBox::new(
                a(a(s(foot_x, depth), 1), off_z),
                a(foot_y, off_y),
                a(foot_z, off_x),
                a(foot_x, off_z),
                a(s(a(foot_y, height), 1), off_y),
                a(s(a(foot_z, width), 1), off_x),
            ),
            Direction::East => BoundingBox::new(
                a(foot_x, off_z),
                a(foot_y, off_y),
                a(foot_z, off_x),
                a(s(a(foot_x, depth), 1), off_z),
                a(s(a(foot_y, height), 1), off_y),
                a(s(a(foot_z, width), 1), off_x),
            ),
            _ => BoundingBox::new(
                a(foot_x, off_x),
                a(foot_y, off_y),
                a(foot_z, off_z),
                a(s(a(foot_x, width), 1), off_x),
                a(s(a(foot_y, height), 1), off_y),
                a(s(a(foot_z, depth), 1), off_z),
            ),
        }
    }

    /// Port of `BoundingBox#intersectingChunks()`: `ChunkPos.rangeClosed` between the corner
    /// chunks, in that method's walk order.
    pub fn intersecting_chunks(&self) -> Vec<ChunkPos> {
        let min_chunk_x = SectionPos::block_to_section_coord_i32(self.min_x);
        let min_chunk_z = SectionPos::block_to_section_coord_i32(self.min_z);
        let max_chunk_x = SectionPos::block_to_section_coord_i32(self.max_x);
        let max_chunk_z = SectionPos::block_to_section_coord_i32(self.max_z);
        ChunkPos::range_closed_from_to(min_chunk_x, min_chunk_z, max_chunk_x, max_chunk_z)
    }

    /// Port of `BoundingBox#intersects(BoundingBox)` (inclusive on both ends).
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.max_x >= other.min_x
            && self.min_x <= other.max_x
            && self.max_z >= other.min_z
            && self.min_z <= other.max_z
            && self.max_y >= other.min_y
            && self.min_y <= other.max_y
    }

    /// Port of `BoundingBox#intersects(int, int, int, int)`: X and Z only.
    pub fn intersects_xz(&self, min_x: i32, min_z: i32, max_x: i32, max_z: i32) -> bool {
        self.max_x >= min_x && self.min_x <= max_x && self.max_z >= min_z && self.min_z <= max_z
    }

    /// Port of `BoundingBox#encapsulatingPositions(Iterable<BlockPos>)`. `None` for no positions.
    pub fn encapsulating_positions<'a>(iterable: impl IntoIterator<Item = &'a BlockPos>) -> Option<BoundingBox> {
        let mut iterator = iterable.into_iter();
        let first = iterator.next()?;
        let mut result = BoundingBox::from_block_pos(first);
        for pos in iterator {
            result.encapsulate_pos(pos);
        }
        Some(result)
    }

    /// Port of `BoundingBox#encapsulatingBoxes(Iterable<BoundingBox>)`. `None` for no boxes.
    pub fn encapsulating_boxes<'a>(iterable: impl IntoIterator<Item = &'a BoundingBox>) -> Option<BoundingBox> {
        let mut iterator = iterable.into_iter();
        let first = iterator.next()?;
        let mut result = BoundingBox::new(first.min_x, first.min_y, first.min_z, first.max_x, first.max_y, first.max_z);
        for b in iterator {
            result.encapsulate(b);
        }
        Some(result)
    }

    /// Port of the deprecated, MUTATING `BoundingBox#encapsulate(BoundingBox)`.
    pub fn encapsulate(&mut self, other: &BoundingBox) -> &mut BoundingBox {
        self.min_x = self.min_x.min(other.min_x);
        self.min_y = self.min_y.min(other.min_y);
        self.min_z = self.min_z.min(other.min_z);
        self.max_x = self.max_x.max(other.max_x);
        self.max_y = self.max_y.max(other.max_y);
        self.max_z = self.max_z.max(other.max_z);
        self
    }

    /// Port of `BoundingBox#encapsulating(BoundingBox, BoundingBox)` (non-mutating).
    pub fn encapsulating(a: &BoundingBox, b: &BoundingBox) -> BoundingBox {
        BoundingBox::new(
            a.min_x.min(b.min_x),
            a.min_y.min(b.min_y),
            a.min_z.min(b.min_z),
            a.max_x.max(b.max_x),
            a.max_y.max(b.max_y),
            a.max_z.max(b.max_z),
        )
    }

    /// Port of the deprecated, MUTATING `BoundingBox#encapsulate(BlockPos)`.
    pub fn encapsulate_pos(&mut self, pos: &BlockPos) -> &mut BoundingBox {
        self.min_x = self.min_x.min(pos.get_x());
        self.min_y = self.min_y.min(pos.get_y());
        self.min_z = self.min_z.min(pos.get_z());
        self.max_x = self.max_x.max(pos.get_x());
        self.max_y = self.max_y.max(pos.get_y());
        self.max_z = self.max_z.max(pos.get_z());
        self
    }

    /// Port of the deprecated, MUTATING `BoundingBox#move(int, int, int)`.
    pub fn move_by(&mut self, dx: i32, dy: i32, dz: i32) -> &mut BoundingBox {
        self.min_x = self.min_x.wrapping_add(dx);
        self.min_y = self.min_y.wrapping_add(dy);
        self.min_z = self.min_z.wrapping_add(dz);
        self.max_x = self.max_x.wrapping_add(dx);
        self.max_y = self.max_y.wrapping_add(dy);
        self.max_z = self.max_z.wrapping_add(dz);
        self
    }

    /// Port of the deprecated, MUTATING `BoundingBox#move(Vec3i)`.
    pub fn move_by_vec(&mut self, amount: &Vec3i) -> &mut BoundingBox {
        self.move_by(amount.get_x(), amount.get_y(), amount.get_z())
    }

    /// Port of `BoundingBox#moved(int, int, int)`: a new box (goes through the checking
    /// constructor, which a wrapped coordinate can trip).
    pub fn moved(&self, dx: i32, dy: i32, dz: i32) -> BoundingBox {
        BoundingBox::new(
            self.min_x.wrapping_add(dx),
            self.min_y.wrapping_add(dy),
            self.min_z.wrapping_add(dz),
            self.max_x.wrapping_add(dx),
            self.max_y.wrapping_add(dy),
            self.max_z.wrapping_add(dz),
        )
    }

    /// Port of `BoundingBox#inflatedBy(int)`.
    pub fn inflated_by(&self, amount: i32) -> BoundingBox {
        self.inflated_by_xyz(amount, amount, amount)
    }

    /// Port of `BoundingBox#inflatedBy(int, int, int)`. A negative amount can invert the box,
    /// which the constructor then repairs.
    pub fn inflated_by_xyz(&self, inflate_x: i32, inflate_y: i32, inflate_z: i32) -> BoundingBox {
        BoundingBox::new(
            self.min_x.wrapping_sub(inflate_x),
            self.min_y.wrapping_sub(inflate_y),
            self.min_z.wrapping_sub(inflate_z),
            self.max_x.wrapping_add(inflate_x),
            self.max_y.wrapping_add(inflate_y),
            self.max_z.wrapping_add(inflate_z),
        )
    }

    /// Port of `BoundingBox#isInside(Vec3i)`.
    pub fn is_inside_vec(&self, pos: &Vec3i) -> bool {
        self.is_inside(pos.get_x(), pos.get_y(), pos.get_z())
    }

    /// Port of `BoundingBox#isInside(int, int, int)` (inclusive).
    pub fn is_inside(&self, x: i32, y: i32, z: i32) -> bool {
        x >= self.min_x && x <= self.max_x && z >= self.min_z && z <= self.max_z && y >= self.min_y && y <= self.max_y
    }

    /// Port of `BoundingBox#getLength()`: `max - min` per axis (NOT the span, which adds 1).
    pub fn get_length(&self) -> Vec3i {
        Vec3i::new(
            self.max_x.wrapping_sub(self.min_x),
            self.max_y.wrapping_sub(self.min_y),
            self.max_z.wrapping_sub(self.min_z),
        )
    }

    /// Port of `BoundingBox#getXSpan()`.
    pub fn get_x_span(&self) -> i32 {
        self.max_x.wrapping_sub(self.min_x).wrapping_add(1)
    }

    /// Port of `BoundingBox#getYSpan()`.
    pub fn get_y_span(&self) -> i32 {
        self.max_y.wrapping_sub(self.min_y).wrapping_add(1)
    }

    /// Port of `BoundingBox#getZSpan()`.
    pub fn get_z_span(&self) -> i32 {
        self.max_z.wrapping_sub(self.min_z).wrapping_add(1)
    }

    /// Port of `BoundingBox#getCenter()`: `min + span / 2` per axis, Java `int` division
    /// (truncating toward zero).
    pub fn get_center(&self) -> BlockPos {
        BlockPos::new(
            self.min_x.wrapping_add(self.get_x_span() / 2),
            self.min_y.wrapping_add(self.get_y_span() / 2),
            self.min_z.wrapping_add(self.get_z_span() / 2),
        )
    }

    /// Port of `BoundingBox#forAllCorners(Consumer<BlockPos>)`.
    ///
    /// Order: max/min X alternates fastest, then Y, then Z -- starting from the MAX corner.
    pub fn for_all_corners(&self, mut consumer: impl FnMut(BlockPos)) {
        consumer(BlockPos::new(self.max_x, self.max_y, self.max_z));
        consumer(BlockPos::new(self.min_x, self.max_y, self.max_z));
        consumer(BlockPos::new(self.max_x, self.min_y, self.max_z));
        consumer(BlockPos::new(self.min_x, self.min_y, self.max_z));
        consumer(BlockPos::new(self.max_x, self.max_y, self.min_z));
        consumer(BlockPos::new(self.min_x, self.max_y, self.min_z));
        consumer(BlockPos::new(self.max_x, self.min_y, self.min_z));
        consumer(BlockPos::new(self.min_x, self.min_y, self.min_z));
    }

    /// Port of `BoundingBox#hashCode()`: `Objects.hash(minX, minY, minZ, maxX, maxY, maxZ)`,
    /// i.e. `Arrays.hashCode` -- start at 1, then `31 * h + v` per field, wrapping.
    pub fn java_hash_code(&self) -> i32 {
        [self.min_x, self.min_y, self.min_z, self.max_x, self.max_y, self.max_z]
            .iter()
            .fold(1i32, |h, v| h.wrapping_mul(31).wrapping_add(*v))
    }

    /// Port of `BoundingBox#minX()`.
    pub fn min_x(&self) -> i32 {
        self.min_x
    }
    /// Port of `BoundingBox#minY()`.
    pub fn min_y(&self) -> i32 {
        self.min_y
    }
    /// Port of `BoundingBox#minZ()`.
    pub fn min_z(&self) -> i32 {
        self.min_z
    }
    /// Port of `BoundingBox#maxX()`.
    pub fn max_x(&self) -> i32 {
        self.max_x
    }
    /// Port of `BoundingBox#maxY()`.
    pub fn max_y(&self) -> i32 {
        self.max_y
    }
    /// Port of `BoundingBox#maxZ()`.
    pub fn max_z(&self) -> i32 {
        self.max_z
    }
}

impl std::fmt::Display for BoundingBox {
    /// Port of `BoundingBox#toString()` (Guava `MoreObjects.toStringHelper`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BoundingBox{{minX={}, minY={}, minZ={}, maxX={}, maxY={}, maxZ={}}}",
            self.min_x, self.min_y, self.min_z, self.max_x, self.max_y, self.max_z
        )
    }
}

/// Java: `public static final Codec<BoundingBox> CODEC` (`Codec.INT_STREAM.comapFlatMap(...)`).
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, BoundingBox> STREAM_CODEC`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf` / `BlockPos.STREAM_CODEC`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
