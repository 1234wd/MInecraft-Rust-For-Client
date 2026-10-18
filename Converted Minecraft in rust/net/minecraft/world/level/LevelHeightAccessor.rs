//! Port of: net/minecraft/world/level/LevelHeightAccessor.java
//! Java class(es): net.minecraft.world.level.LevelHeightAccessor, LevelHeightAccessor::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 22. `public interface LevelHeightAccessor`: a world's vertical extent -- `getHeight()` and
//! `getMinY()` -- and eleven defaults derived from them, plus `create(minY, height)`.
//!
//! # All `int`, all wrapping
//!
//! `getMaxY()` is `getMinY() + getHeight() - 1` and `getSectionsCount()` is
//! `getMaxSectionY() - getMinSectionY() + 1`, in plain `int` arithmetic: an extreme `minY`/`height`
//! pair wraps rather than failing (the probe has `(MIN_VALUE, MAX_VALUE)` and `(MAX_VALUE, 1)`).
//! Section coordinates are `SectionPos.blockToSectionCoord` (`>> 4`), and the index helpers are
//! offsets from `getMinSectionY()` with no range check.
//!
//! `isInsideBuildHeight`/`isOutsideBuildHeight` are each overloaded on `BlockPos` and `int`; per
//! `DESIGN_DECISIONS.md#overload-naming` they are `*_block_pos` and `*_i32`.
//!
//! Verified by the `HEIGHTACC` group of `small_probe.txt` (`tests/session22.rs`): eight
//! `create(minY, height)` worlds (vanilla overworld, empty, negative height, both extremes), every
//! default against fourteen `y` values.

use crate::net::minecraft::core::BlockPos::BlockPos;
use crate::net::minecraft::core::SectionPos::SectionPos;

/// Port of `net.minecraft.world.level.LevelHeightAccessor`.
pub trait LevelHeightAccessor {
    /// Port of `int getHeight()`.
    fn get_height(&self) -> i32;

    /// Port of `int getMinY()`.
    fn get_min_y(&self) -> i32;

    /// Port of `default int getMaxY()`: `getMinY() + getHeight() - 1`.
    fn get_max_y(&self) -> i32 {
        self.get_min_y().wrapping_add(self.get_height()).wrapping_sub(1)
    }

    /// Port of `default int getSectionsCount()`.
    fn get_sections_count(&self) -> i32 {
        self.get_max_section_y().wrapping_sub(self.get_min_section_y()).wrapping_add(1)
    }

    /// Port of `default int getMinSectionY()`.
    fn get_min_section_y(&self) -> i32 {
        SectionPos::block_to_section_coord_i32(self.get_min_y())
    }

    /// Port of `default int getMaxSectionY()`.
    fn get_max_section_y(&self) -> i32 {
        SectionPos::block_to_section_coord_i32(self.get_max_y())
    }

    /// Port of `default boolean isInsideBuildHeight(BlockPos pos)`.
    fn is_inside_build_height_block_pos(&self, pos: &BlockPos) -> bool {
        self.is_inside_build_height_i32(pos.get_y())
    }

    /// Port of `default boolean isInsideBuildHeight(int blockY)`.
    fn is_inside_build_height_i32(&self, block_y: i32) -> bool {
        block_y >= self.get_min_y() && block_y <= self.get_max_y()
    }

    /// Port of `default boolean isOutsideBuildHeight(BlockPos pos)`.
    fn is_outside_build_height_block_pos(&self, pos: &BlockPos) -> bool {
        self.is_outside_build_height_i32(pos.get_y())
    }

    /// Port of `default boolean isOutsideBuildHeight(int blockY)`.
    fn is_outside_build_height_i32(&self, block_y: i32) -> bool {
        block_y < self.get_min_y() || block_y > self.get_max_y()
    }

    /// Port of `default int getSectionIndex(int blockY)`.
    fn get_section_index(&self, block_y: i32) -> i32 {
        self.get_section_index_from_section_y(SectionPos::block_to_section_coord_i32(block_y))
    }

    /// Port of `default int getSectionIndexFromSectionY(int sectionY)`.
    fn get_section_index_from_section_y(&self, section_y: i32) -> i32 {
        section_y.wrapping_sub(self.get_min_section_y())
    }

    /// Port of `default int getSectionYFromSectionIndex(int sectionIndex)`.
    fn get_section_y_from_section_index(&self, section_index: i32) -> i32 {
        section_index.wrapping_add(self.get_min_section_y())
    }
}

/// The anonymous `LevelHeightAccessor` that `LevelHeightAccessor.create(minY, height)` returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Created {
    min_y: i32,
    height: i32,
}

impl LevelHeightAccessor for Created {
    fn get_height(&self) -> i32 {
        self.height
    }

    fn get_min_y(&self) -> i32 {
        self.min_y
    }
}

/// Port of `static LevelHeightAccessor create(int minY, int height)`.
pub const fn create(min_y: i32, height: i32) -> Created {
    Created { min_y, height }
}
