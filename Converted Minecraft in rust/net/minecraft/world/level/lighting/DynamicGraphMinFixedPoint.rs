//! Port of: net/minecraft/world/level/lighting/DynamicGraphMinFixedPoint.java
//! Java class(es): net.minecraft.world.level.lighting.DynamicGraphMinFixedPoint, DynamicGraphMinFixedPoint::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Verified against the real 26.2 jar: concrete
//! `ChunkTracker` and `SectionTracker` subclasses in `_porting/java-oracle/src/p/GraphProbe.java`
//! and in `_porting/tests/session19_consts.rs` run the same scenarios -- full and PARTIAL
//! `runUpdates`, source raises and lowers, `removeIf` -- and every level, every queue size and
//! every returned budget must match `_porting/test-data/graph_probe.txt`.
//!
//! # SHAPE OF THE PORT
//!
//! The Java class is abstract: subclasses supply five callbacks (`getLevel`, `setLevel`,
//! `getComputedLevel`, `computeLevelFromNeighbor`, `checkNeighborsAfterUpdate`) and may override
//! `isSource`, while the base class owns the queue and the "computed level" map. In Rust the
//! state is [`GraphState`] and the algorithm is a trait whose provided methods are the Java
//! base-class methods; an implementor returns its `GraphState` from [`DynamicGraphMinFixedPoint::graph`]
//! and implements the callbacks.
//!
//! # THE MAP'S ITERATION ORDER DOES NOT MATTER HERE
//!
//! `computedLevels` is a fastutil `Long2ByteOpenHashMap`. It is only iterated in `removeIf`,
//! which first COLLECTS matching nodes and then removes each from the queue; the queue's final
//! contents and its `firstQueuedLevel` (always the lowest non-empty level) do not depend on the
//! removal order. So a `std` `HashMap` is equivalent, and the map's `minMapSize`/load-factor
//! tuning (capacity only) is ignored.
//!
//! `hasWork` is `volatile` in Java (read from another thread by the chunk system); this port is
//! single-threaded and keeps it as a plain field.

use std::collections::HashMap;

use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::world::level::lighting::LeveledPriorityQueue::LeveledPriorityQueue;

/// Java: `public static final long SOURCE = Long.MAX_VALUE`.
pub const SOURCE: i64 = i64::MAX;
/// Java: `private static final int NO_COMPUTED_LEVEL = 255`.
pub const NO_COMPUTED_LEVEL: i32 = 255;

/// The fields of the abstract Java class.
#[derive(Clone, Debug, PartialEq)]
pub struct GraphState {
    level_count: i32,
    priority_queue: LeveledPriorityQueue,
    computed_levels: HashMap<i64, u8>,
    has_work: bool,
}

impl GraphState {
    /// Port of the protected constructor `DynamicGraphMinFixedPoint(int, int, int)`.
    pub fn new(level_count: i32, min_queue_size: i32, min_map_size: i32) -> GraphState {
        let _ = min_map_size;
        if level_count >= 254 {
            panic!("java.lang.IllegalArgumentException: Level count must be < 254.");
        }
        GraphState {
            level_count,
            priority_queue: LeveledPriorityQueue::new(level_count, min_queue_size),
            computed_levels: HashMap::new(),
            has_work: false,
        }
    }

    /// The protected `levelCount` field.
    pub fn level_count(&self) -> i32 {
        self.level_count
    }

    /// `computedLevels.get(node) & 255`: `255` when absent (the map's default return is `-1`).
    fn computed(&self, node: i64) -> i32 {
        self.computed_levels.get(&node).map(|b| *b as i32).unwrap_or(NO_COMPUTED_LEVEL)
    }

    /// `computedLevels.remove(node) & 255`.
    fn take_computed(&mut self, node: i64) -> i32 {
        self.computed_levels.remove(&node).map(|b| b as i32).unwrap_or(NO_COMPUTED_LEVEL)
    }

    /// The priority queue (read-only, for tests and diagnostics).
    pub fn priority_queue(&self) -> &LeveledPriorityQueue {
        &self.priority_queue
    }
}

/// Port of `net.minecraft.world.level.lighting.DynamicGraphMinFixedPoint`.
pub trait DynamicGraphMinFixedPoint {
    /// The base-class state.
    fn graph(&mut self) -> &mut GraphState;
    /// Read-only access to the base-class state.
    fn graph_ref(&self) -> &GraphState;

    /// Java: `protected abstract int getComputedLevel(long node, long knownParent, int knownLevelFromParent)`.
    fn get_computed_level(&mut self, node: i64, known_parent: i64, known_level_from_parent: i32) -> i32;
    /// Java: `protected abstract void checkNeighborsAfterUpdate(long node, int level, boolean onlyDecrease)`.
    fn check_neighbors_after_update(&mut self, node: i64, level: i32, only_decrease: bool);
    /// Java: `protected abstract int getLevel(long node)`.
    fn get_level(&mut self, node: i64) -> i32;
    /// Java: `protected abstract void setLevel(long node, int level)`.
    fn set_level(&mut self, node: i64, level: i32);
    /// Java: `protected abstract int computeLevelFromNeighbor(long from, long to, int fromLevel)`.
    fn compute_level_from_neighbor(&mut self, from: i64, to: i64, from_level: i32) -> i32;

    /// Port of `DynamicGraphMinFixedPoint#isSource(long)`; `ChunkTracker` overrides it.
    fn is_source(&self, node: i64) -> bool {
        node == i64::MAX
    }

    /// Port of `DynamicGraphMinFixedPoint#removeFromQueue(long)`.
    fn remove_from_queue(&mut self, node: i64) {
        let computed_level = self.graph().take_computed(node);
        if computed_level != NO_COMPUTED_LEVEL {
            let level = self.get_level(node);
            let priority = calculate_priority(self.graph_ref().level_count, level, computed_level);
            let g = self.graph();
            let level_count = g.level_count;
            g.priority_queue.dequeue(node, priority, level_count);
            g.has_work = !g.priority_queue.is_empty();
        }
    }

    /// Port of `DynamicGraphMinFixedPoint#removeIf(LongPredicate)`: collect first, then remove.
    fn remove_if(&mut self, mut pred: impl FnMut(i64) -> bool)
    where
        Self: Sized,
    {
        let nodes: Vec<i64> = self.graph_ref().computed_levels.keys().copied().filter(|n| pred(*n)).collect();
        for node in nodes {
            self.remove_from_queue(node);
        }
    }

    /// Port of `DynamicGraphMinFixedPoint#checkNode(long)`.
    fn check_node(&mut self, node: i64) {
        let level_count = self.graph_ref().level_count;
        self.check_edge(node, node, level_count - 1, false);
    }

    /// Port of the protected 4-argument `DynamicGraphMinFixedPoint#checkEdge`.
    fn check_edge(&mut self, from: i64, to: i64, new_level_from: i32, only_decreased: bool) {
        let level_to = self.get_level(to);
        let old_computed = self.graph_ref().computed(to);
        self.check_edge_full(from, to, new_level_from, level_to, old_computed, only_decreased);
        let g = self.graph();
        g.has_work = !g.priority_queue.is_empty();
    }

    /// Port of the private 6-argument `DynamicGraphMinFixedPoint#checkEdge`.
    #[doc(hidden)]
    fn check_edge_full(
        &mut self,
        from: i64,
        to: i64,
        mut new_level_from: i32,
        mut level_to: i32,
        mut old_computed_level: i32,
        only_decreased: bool,
    ) {
        if self.is_source(to) {
            return;
        }
        let level_count = self.graph_ref().level_count;
        new_level_from = Mth::clamp_i32(new_level_from, 0, level_count - 1);
        level_to = Mth::clamp_i32(level_to, 0, level_count - 1);
        let was_consistent = old_computed_level == NO_COMPUTED_LEVEL;
        if was_consistent {
            old_computed_level = level_to;
        }
        let new_computed_level = if only_decreased {
            old_computed_level.min(new_level_from)
        } else {
            let c = self.get_computed_level(to, from, new_level_from);
            Mth::clamp_i32(c, 0, level_count - 1)
        };
        let old_priority = calculate_priority(level_count, level_to, old_computed_level);
        let g = self.graph();
        if level_to != new_computed_level {
            let new_priority = calculate_priority(level_count, level_to, new_computed_level);
            if old_priority != new_priority && !was_consistent {
                g.priority_queue.dequeue(to, old_priority, new_priority);
            }
            g.priority_queue.enqueue(to, new_priority);
            g.computed_levels.insert(to, new_computed_level as u8);
        } else if !was_consistent {
            g.priority_queue.dequeue(to, old_priority, level_count);
            g.computed_levels.remove(&to);
        }
    }

    /// Port of the protected final `DynamicGraphMinFixedPoint#checkNeighbor(long, long, int, boolean)`.
    fn check_neighbor(&mut self, from: i64, to: i64, level: i32, only_decreased: bool) {
        let stored_old_computed_level = self.graph_ref().computed(to);
        let level_count = self.graph_ref().level_count;
        let c = self.compute_level_from_neighbor(from, to, level);
        let level_from = Mth::clamp_i32(c, 0, level_count - 1);
        if only_decreased {
            let level_to = self.get_level(to);
            self.check_edge_full(from, to, level_from, level_to, stored_old_computed_level, only_decreased);
        } else {
            let was_consistent = stored_old_computed_level == NO_COMPUTED_LEVEL;
            let old_computed_level = if was_consistent {
                let l = self.get_level(to);
                Mth::clamp_i32(l, 0, level_count - 1)
            } else {
                stored_old_computed_level
            };
            if level_from == old_computed_level {
                let level_to = if was_consistent { old_computed_level } else { self.get_level(to) };
                self.check_edge_full(from, to, level_count - 1, level_to, stored_old_computed_level, only_decreased);
            }
        }
    }

    /// Port of the protected final `DynamicGraphMinFixedPoint#hasWork()`.
    fn has_work(&self) -> bool {
        self.graph_ref().has_work
    }

    /// Port of the protected final `DynamicGraphMinFixedPoint#runUpdates(int)`: processes at most
    /// `count` nodes and returns the unused budget.
    fn run_updates(&mut self, mut count: i32) -> i32 {
        if self.graph_ref().priority_queue.is_empty() {
            return count;
        }
        while !self.graph_ref().priority_queue.is_empty() && count > 0 {
            count -= 1;
            let node = self.graph().priority_queue.remove_first_long();
            let level_count = self.graph_ref().level_count;
            let l = self.get_level(node);
            let level = Mth::clamp_i32(l, 0, level_count - 1);
            let computed_level = self.graph().take_computed(node);
            if computed_level < level {
                self.set_level(node, computed_level);
                self.check_neighbors_after_update(node, computed_level, true);
            } else if computed_level > level {
                self.set_level(node, level_count - 1);
                if computed_level != level_count - 1 {
                    let g = self.graph();
                    g.priority_queue.enqueue(node, calculate_priority(level_count, level_count - 1, computed_level));
                    g.computed_levels.insert(node, computed_level as u8);
                }
                self.check_neighbors_after_update(node, level, false);
            }
        }
        let g = self.graph();
        g.has_work = !g.priority_queue.is_empty();
        count
    }

    /// Port of `DynamicGraphMinFixedPoint#getQueueSize()`: the number of nodes with a pending
    /// computed level.
    fn get_queue_size(&self) -> i32 {
        self.graph_ref().computed_levels.len() as i32
    }
}

/// Port of the private `DynamicGraphMinFixedPoint#calculatePriority(int, int)`.
fn calculate_priority(level_count: i32, level: i32, computed_level: i32) -> i32 {
    level.min(computed_level).min(level_count - 1)
}
