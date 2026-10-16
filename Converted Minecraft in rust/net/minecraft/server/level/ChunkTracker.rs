//! Port of: net/minecraft/server/level/ChunkTracker.java
//! Java class(es): net.minecraft.server.level.ChunkTracker
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Verified against the real 26.2 jar: a concrete subclass
//! in `_porting/java-oracle/src/p/GraphProbe.java` and its twin in
//! `_porting/tests/session19_consts.rs` must produce identical levels, queue sizes and
//! `runUpdates` budgets for the scenarios in `_porting/test-data/graph_probe.txt`.
//!
//! # HOW A CONCRETE TRACKER USES THIS
//!
//! Java's `ChunkTracker` is an abstract subclass that fills in four of
//! `DynamicGraphMinFixedPoint`'s callbacks and leaves `getLevelFromSource`, `getLevel` and
//! `setLevel` to ITS subclasses. Rust traits cannot supply a supertrait's methods, so the four
//! fills are provided here as `chunk_*` methods, and a concrete tracker's
//! `DynamicGraphMinFixedPoint` impl delegates to them (see the test twin for the exact wiring).
//!
//! # THE 3x3 NEIGHBOURHOOD, AND THE SOURCE STANDING IN FOR "SELF"
//!
//! Levels propagate to the 8 surrounding chunks (+1 per step). In `getComputedLevel` the loop
//! visits all 9 cells and substitutes `INVALID_CHUNK_POS` -- the source node -- for the cell's
//! own position, so a chunk's own ticket level is read through `getLevelFromSource` in the same
//! pass. The early `return` at level 0 is part of the contract (no lower level exists).

use crate::net::minecraft::world::level::lighting::DynamicGraphMinFixedPoint::DynamicGraphMinFixedPoint;
use crate::net::minecraft::world::level::ChunkPos::{ChunkPos, INVALID_CHUNK_POS};

/// Port of the abstract `net.minecraft.server.level.ChunkTracker`.
pub trait ChunkTracker: DynamicGraphMinFixedPoint {
    /// Java: `protected abstract int getLevelFromSource(long to)`.
    fn get_level_from_source(&mut self, to: i64) -> i32;

    /// Port of `ChunkTracker#isSource(long)`: the source is `ChunkPos.INVALID_CHUNK_POS`.
    fn chunk_is_source(&self, node: i64) -> bool {
        node == INVALID_CHUNK_POS
    }

    /// Port of `ChunkTracker#checkNeighborsAfterUpdate(long, int, boolean)`.
    fn chunk_check_neighbors_after_update(&mut self, node: i64, level: i32, only_decrease: bool) {
        let level_count = self.graph_ref().level_count();
        if !only_decrease || level < level_count - 2 {
            let pos = ChunkPos::unpack(node);
            let (x, z) = (pos.x, pos.z);
            for offset_x in -1..=1 {
                for offset_z in -1..=1 {
                    let neighbor = ChunkPos::pack(x.wrapping_add(offset_x), z.wrapping_add(offset_z));
                    if neighbor != node {
                        self.check_neighbor(node, neighbor, level, only_decrease);
                    }
                }
            }
        }
    }

    /// Port of `ChunkTracker#getComputedLevel(long, long, int)`.
    fn chunk_get_computed_level(&mut self, node: i64, known_parent: i64, known_level_from_parent: i32) -> i32 {
        let mut computed_level = known_level_from_parent;
        let pos = ChunkPos::unpack(node);
        let (x, z) = (pos.x, pos.z);
        for offset_x in -1..=1 {
            for offset_z in -1..=1 {
                let mut neighbor = ChunkPos::pack(x.wrapping_add(offset_x), z.wrapping_add(offset_z));
                if neighbor == node {
                    neighbor = INVALID_CHUNK_POS;
                }
                if neighbor != known_parent {
                    let neighbor_level = self.get_level(neighbor);
                    let cost_from_neighbor = self.compute_level_from_neighbor(neighbor, node, neighbor_level);
                    if computed_level > cost_from_neighbor {
                        computed_level = cost_from_neighbor;
                    }
                    if computed_level == 0 {
                        return computed_level;
                    }
                }
            }
        }
        computed_level
    }

    /// Port of `ChunkTracker#computeLevelFromNeighbor(long, long, int)`.
    fn chunk_compute_level_from_neighbor(&mut self, from: i64, to: i64, from_level: i32) -> i32 {
        if from == INVALID_CHUNK_POS {
            self.get_level_from_source(to)
        } else {
            from_level.wrapping_add(1)
        }
    }

    /// Port of `ChunkTracker#update(long, int, boolean)`: an edge from the source to `node`.
    fn update(&mut self, node: i64, new_level_from: i32, only_decreased: bool) {
        self.check_edge(INVALID_CHUNK_POS, node, new_level_from, only_decreased);
    }
}
