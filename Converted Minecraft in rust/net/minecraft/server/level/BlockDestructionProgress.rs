//! Port of: net/minecraft/server/level/BlockDestructionProgress.java
//! Java class(es): net.minecraft.server.level.BlockDestructionProgress
//! Status: VERIFIED
//!
//! Session 22. `public class BlockDestructionProgress implements Comparable<...>`: one player's
//! block-breaking crack overlay -- breaker id, position, a 0..10 progress and the render tick it was
//! last updated.
//!
//! # Three behaviours that are easy to "tidy"
//!
//! * `setProgress` clamps only ABOVE 10. A negative progress is stored as given (the probe stores
//!   `-1` and `MIN_VALUE`).
//! * `equals` is by `id` alone (after a same-class check), and `hashCode` is `Integer.hashCode(id)`
//!   -- position and progress take no part. Two progress records for one breaker are equal.
//! * `compareTo` orders by PROGRESS first, then id, each via `Integer.compare` (`-1`/`0`/`1`). It is
//!   therefore inconsistent with `equals` (same id, different progress: equal but not `0`), which
//!   is why the port exposes [`BlockDestructionProgress::compare_to`] and implements no `Ord`.
//!
//! Verified by the `BDPSET` and `BDP` groups of `small_probe.txt` (`tests/session22.rs`): the clamp
//! over seven values, and for six records (including the `int` extremes as ids) the accessors, the
//! hash, and the full `equals` and `compareTo` matrices.

use crate::net::minecraft::core::BlockPos::BlockPos;

/// Port of `net.minecraft.server.level.BlockDestructionProgress`.
#[derive(Debug, Clone, Copy)]
pub struct BlockDestructionProgress {
    id: i32,
    pos: BlockPos,
    progress: i32,
    updated_render_tick: i64,
}

impl BlockDestructionProgress {
    /// Port of `new BlockDestructionProgress(int id, BlockPos pos)`: progress 0, tick 0.
    pub const fn new(id: i32, pos: BlockPos) -> Self {
        BlockDestructionProgress {
            id,
            pos,
            progress: 0,
            updated_render_tick: 0,
        }
    }

    /// Port of `getId()`.
    pub const fn get_id(&self) -> i32 {
        self.id
    }

    /// Port of `getPos()`.
    pub const fn get_pos(&self) -> BlockPos {
        self.pos
    }

    /// Port of `setProgress(int progress)`: values above 10 become 10; nothing is clamped below.
    pub fn set_progress(&mut self, progress: i32) {
        self.progress = if progress > 10 { 10 } else { progress };
    }

    /// Port of `getProgress()`.
    pub const fn get_progress(&self) -> i32 {
        self.progress
    }

    /// Port of `updateTick(long tick)`.
    pub fn update_tick(&mut self, tick: i64) {
        self.updated_render_tick = tick;
    }

    /// Port of `getUpdatedRenderTick()`.
    pub const fn get_updated_render_tick(&self) -> i64 {
        self.updated_render_tick
    }

    /// Port of `hashCode()`: `Integer.hashCode(id)`.
    pub const fn java_hash_code(&self) -> i32 {
        self.id
    }

    /// Port of `compareTo(BlockDestructionProgress o)`: progress, then id, via `Integer.compare`.
    pub fn compare_to(&self, o: &BlockDestructionProgress) -> i32 {
        let ord = if self.progress != o.progress {
            self.progress.cmp(&o.progress)
        } else {
            self.id.cmp(&o.id)
        };
        ord as i32
    }
}

impl PartialEq for BlockDestructionProgress {
    /// Port of `equals(Object o)`: same class and same `id`.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for BlockDestructionProgress {}

impl std::hash::Hash for BlockDestructionProgress {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
