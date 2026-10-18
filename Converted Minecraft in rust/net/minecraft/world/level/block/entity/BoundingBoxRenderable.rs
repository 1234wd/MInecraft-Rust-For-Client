//! Port of: net/minecraft/world/level/block/entity/BoundingBoxRenderable.java
//! Java class(es): net.minecraft.world.level.block.entity.BoundingBoxRenderable, net.minecraft.world.level.block.entity.Mode, net.minecraft.world.level.block.entity.RenderableBox
//! Status: VERIFIED
//!
//! Session 22. `public interface BoundingBoxRenderable` (structure and test blocks that draw their
//! bounds): two abstract methods, a nested `enum Mode` and a nested `record RenderableBox`.
//!
//! # `RenderableBox.fromCorners`
//!
//! The box's origin is the per-axis `Math.min` of the two corners and its size the per-axis
//! `Math.max(a, b) - min` -- in `int`, so two corners `MIN_VALUE` and `MAX_VALUE` apart give a size
//! that WRAPS to `-1` (probed). The size is NOT inclusive: equal corners give size `0`.
//!
//! # Record members
//!
//! `equals` is `BlockPos#equals` and `Vec3i#equals`; `hashCode` is the record fold over their
//! (value-based) `hashCode`s; `toString` prints them as `BlockPos{x=.., y=.., z=..}` and
//! `Vec3i{x=.., y=.., z=..}` -- Guava's `toStringHelper` uses the runtime class name.
//!
//! Verified by the `BBMODE`, `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`).

use crate::javacompat::java_lang;
use crate::javacompat::java_object;
use crate::net::minecraft::core::BlockPos::BlockPos;
use crate::net::minecraft::core::Vec3i::Vec3i;

/// Port of `net.minecraft.world.level.block.entity.BoundingBoxRenderable`.
pub trait BoundingBoxRenderable {
    /// Port of `BoundingBoxRenderable.Mode renderMode()`.
    fn render_mode(&self) -> Mode;

    /// Port of `BoundingBoxRenderable.RenderableBox getRenderableBox()`.
    fn get_renderable_box(&self) -> RenderableBox;
}

/// Port of `BoundingBoxRenderable.Mode`. Declaration order is the ordinal order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    /// `NONE` -- ordinal 0.
    None = 0,
    /// `BOX` -- ordinal 1.
    Box = 1,
    /// `BOX_AND_INVISIBLE_BLOCKS` -- ordinal 2.
    BoxAndInvisibleBlocks = 2,
}

impl Mode {
    /// Java: `Mode.values()`.
    pub const VALUES: [Mode; 3] = [Mode::None, Mode::Box, Mode::BoxAndInvisibleBlocks];

    /// Java: `ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Mode::None => "NONE",
            Mode::Box => "BOX",
            Mode::BoxAndInvisibleBlocks => "BOX_AND_INVISIBLE_BLOCKS",
        }
    }
}

/// Port of `BoundingBoxRenderable.RenderableBox`, `record RenderableBox(BlockPos localPos, Vec3i size)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderableBox {
    local_pos: BlockPos,
    size: Vec3i,
}

impl RenderableBox {
    /// Port of the canonical constructor.
    pub const fn new(local_pos: BlockPos, size: Vec3i) -> Self {
        RenderableBox { local_pos, size }
    }

    /// Port of the accessor `localPos()`.
    pub const fn local_pos(&self) -> BlockPos {
        self.local_pos
    }

    /// Port of the accessor `size()`.
    pub const fn size(&self) -> Vec3i {
        self.size
    }

    /// Port of `RenderableBox.fromCorners(int x1, int y1, int z1, int x2, int y2, int z2)`.
    pub fn from_corners(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> RenderableBox {
        let x = java_lang::min_i32(x1, x2);
        let y = java_lang::min_i32(y1, y2);
        let z = java_lang::min_i32(z1, z2);
        RenderableBox::new(
            BlockPos::new(x, y, z),
            Vec3i::new(
                java_lang::max_i32(x1, x2).wrapping_sub(x),
                java_lang::max_i32(y1, y2).wrapping_sub(y),
                java_lang::max_i32(z1, z2).wrapping_sub(z),
            ),
        )
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[self.local_pos.java_hash_code(), self.size.java_hash_code()])
    }
}

impl std::hash::Hash for RenderableBox {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.java_hash_code().hash(state)
    }
}

impl std::fmt::Display for RenderableBox {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RenderableBox[localPos={}, size={}]",
            self.local_pos, self.size
        )
    }
}
