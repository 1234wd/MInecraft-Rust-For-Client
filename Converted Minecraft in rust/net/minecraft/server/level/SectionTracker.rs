//! Port of: net/minecraft/server/level/SectionTracker.java
//! Java class(es): net.minecraft.server.level.SectionTracker
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Verified against the real 26.2 jar the same way as
//! `ChunkTracker` (see that file): concrete twins in `GraphProbe.java` and
//! `_porting/tests/session19_consts.rs` must agree on every row of
//! `_porting/test-data/graph_probe.txt`.
//!
//! Same structure as `ChunkTracker`, over the 3x3x3 neighbourhood of a packed `SectionPos`
//! (`SectionPos.offset(long, dx, dy, dz)`), with `Long.MAX_VALUE` as the source and the base
//! class's `isSource`. The `chunk`-style delegation is described in `ChunkTracker.rs`.

use crate::net::minecraft::core::SectionPos::SectionPos;
use crate::net::minecraft::world::level::lighting::DynamicGraphMinFixedPoint::DynamicGraphMinFixedPoint;

/// Port of the abstract `net.minecraft.server.level.SectionTracker`.
pub trait SectionTracker: DynamicGraphMinFixedPoint {
    /// Java: `protected abstract int getLevelFromSource(long to)`.
    fn get_level_from_source(&mut self, to: i64) -> i32;

    /// Port of `SectionTracker#checkNeighborsAfterUpdate(long, int, boolean)`.
    fn section_check_neighbors_after_update(&mut self, node: i64, level: i32, only_decrease: bool) {
        let level_count = self.graph_ref().level_count();
        if !only_decrease || level < level_count - 2 {
            for offset_x in -1..=1 {
                for offset_y in -1..=1 {
                    for offset_z in -1..=1 {
                        let neighbor = SectionPos::offset_long(node, offset_x, offset_y, offset_z);
                        if neighbor != node {
                            self.check_neighbor(node, neighbor, level, only_decrease);
                        }
                    }
                }
            }
        }
    }

    /// Port of `SectionTracker#getComputedLevel(long, long, int)`.
    fn section_get_computed_level(&mut self, node: i64, known_parent: i64, known_level_from_parent: i32) -> i32 {
        let mut computed_level = known_level_from_parent;
        for offset_x in -1..=1 {
            for offset_y in -1..=1 {
                for offset_z in -1..=1 {
                    let mut neighbor = SectionPos::offset_long(node, offset_x, offset_y, offset_z);
                    if neighbor == node {
                        neighbor = i64::MAX;
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
        }
        computed_level
    }

    /// Port of `SectionTracker#computeLevelFromNeighbor(long, long, int)`.
    fn section_compute_level_from_neighbor(&mut self, from: i64, to: i64, from_level: i32) -> i32 {
        if self.is_source(from) {
            self.get_level_from_source(to)
        } else {
            from_level.wrapping_add(1)
        }
    }

    /// Port of `SectionTracker#update(long, int, boolean)`.
    fn update(&mut self, node: i64, new_level_from: i32, only_decreased: bool) {
        self.check_edge(i64::MAX, node, new_level_from, only_decreased);
    }
}
