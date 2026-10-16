//! Port of: net/minecraft/world/level/lighting/LeveledPriorityQueue.java
//! Java class(es): net.minecraft.world.level.lighting.LeveledPriorityQueue, LeveledPriorityQueue::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Verified through `DynamicGraphMinFixedPoint` against the
//! real 26.2 jar: `_porting/tests/session19_consts.rs` replays `graph.*` rows of
//! `_porting/test-data/graph_probe.txt`, which include PARTIAL `runUpdates(n)` calls whose
//! intermediate results depend on the exact order this queue hands nodes out.
//!
//! # ONE INSERTION-ORDERED SET PER LEVEL
//!
//! Java keeps a fastutil `LongLinkedOpenHashSet` per level. Its iteration (and `removeFirstLong`)
//! order is INSERTION order, independent of hashing: re-adding a present node does not move it,
//! and removing one unlinks it. [`LinkedLongSet`] reproduces exactly that. The anonymous subclass
//! only overrides `rehash` to never shrink below `minSize`, which changes capacity, not order, so
//! `min_size` is accepted and ignored.
//!
//! # `firstQueuedLevel` IS ALWAYS THE LOWEST NON-EMPTY LEVEL
//!
//! `enqueue` lowers it, and an emptied first level rescans upward. `dequeue` takes an
//! `upperBound` so a caller that is about to re-enqueue at a known level can stop the scan there.

use std::collections::HashMap;

/// A set of `long`s iterating in insertion order: fastutil's `LongLinkedOpenHashSet` semantics
/// as far as `LeveledPriorityQueue` uses them (`add`, `remove`, `removeFirstLong`, `isEmpty`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkedLongSet {
    links: HashMap<i64, (Option<i64>, Option<i64>)>,
    head: Option<i64>,
    tail: Option<i64>,
}

impl LinkedLongSet {
    /// `add(long)`: appends at the tail; `false` (and no reordering) if already present.
    pub fn add(&mut self, k: i64) -> bool {
        if self.links.contains_key(&k) {
            return false;
        }
        self.links.insert(k, (self.tail, None));
        match self.tail {
            Some(t) => self.links.get_mut(&t).expect("tail is linked").1 = Some(k),
            None => self.head = Some(k),
        }
        self.tail = Some(k);
        true
    }

    /// `remove(long)`: unlinks `k`; `false` if absent.
    pub fn remove(&mut self, k: i64) -> bool {
        let Some((prev, next)) = self.links.remove(&k) else {
            return false;
        };
        match prev {
            Some(p) => self.links.get_mut(&p).expect("prev is linked").1 = next,
            None => self.head = next,
        }
        match next {
            Some(n) => self.links.get_mut(&n).expect("next is linked").0 = prev,
            None => self.tail = prev,
        }
        true
    }

    /// `removeFirstLong()`: the oldest element. Java throws `NoSuchElementException` (no
    /// message) when empty.
    pub fn remove_first_long(&mut self) -> i64 {
        let Some(first) = self.head else {
            panic!("java.util.NoSuchElementException");
        };
        self.remove(first);
        first
    }

    /// `isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// `size()`.
    pub fn len(&self) -> usize {
        self.links.len()
    }

    /// `contains(long)`.
    pub fn contains(&self, k: i64) -> bool {
        self.links.contains_key(&k)
    }

    /// Elements in iteration (insertion) order.
    pub fn to_vec(&self) -> Vec<i64> {
        let mut out = Vec::with_capacity(self.links.len());
        let mut cur = self.head;
        while let Some(k) = cur {
            out.push(k);
            cur = self.links[&k].1;
        }
        out
    }
}

/// Port of `net.minecraft.world.level.lighting.LeveledPriorityQueue`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeveledPriorityQueue {
    level_count: i32,
    queues: Vec<LinkedLongSet>,
    first_queued_level: i32,
}

#[cold]
#[inline(never)]
fn index_out_of_bounds(index: i32, len: usize) -> ! {
    panic!("java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {len}")
}

impl LeveledPriorityQueue {
    /// Port of `LeveledPriorityQueue(int levelCount, int minSize)`. `min_size` only sizes the Java
    /// hash tables; see the module docs.
    pub fn new(level_count: i32, min_size: i32) -> LeveledPriorityQueue {
        let _ = min_size;
        LeveledPriorityQueue {
            level_count,
            queues: (0..level_count.max(0)).map(|_| LinkedLongSet::default()).collect(),
            first_queued_level: level_count,
        }
    }

    fn queue(&mut self, key: i32) -> &mut LinkedLongSet {
        if key < 0 || key as usize >= self.queues.len() {
            index_out_of_bounds(key, self.queues.len());
        }
        &mut self.queues[key as usize]
    }

    /// Port of `LeveledPriorityQueue#removeFirstLong()`. Java throws when the queue is empty
    /// (`queues[levelCount]` is out of bounds); so does the port.
    pub fn remove_first_long(&mut self) -> i64 {
        let first = self.first_queued_level;
        let result = self.queue(first).remove_first_long();
        if self.queue(first).is_empty() {
            self.check_first_queued_level(self.level_count);
        }
        result
    }

    /// Port of `LeveledPriorityQueue#isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.first_queued_level >= self.level_count
    }

    /// Port of `LeveledPriorityQueue#dequeue(long, int, int)`.
    pub fn dequeue(&mut self, node: i64, key: i32, upper_bound: i32) {
        let queue = self.queue(key);
        queue.remove(node);
        if queue.is_empty() && self.first_queued_level == key {
            self.check_first_queued_level(upper_bound);
        }
    }

    /// Port of `LeveledPriorityQueue#enqueue(long, int)`.
    pub fn enqueue(&mut self, node: i64, key: i32) {
        self.queue(key).add(node);
        if self.first_queued_level > key {
            self.first_queued_level = key;
        }
    }

    fn check_first_queued_level(&mut self, upper_bound: i32) {
        let old_level = self.first_queued_level;
        self.first_queued_level = upper_bound;
        let mut i = old_level + 1;
        while i < upper_bound {
            if !self.queue(i).is_empty() {
                self.first_queued_level = i;
                break;
            }
            i += 1;
        }
    }

    /// The private `firstQueuedLevel` (read-only, for tests and diagnostics).
    pub fn first_queued_level(&self) -> i32 {
        self.first_queued_level
    }

    /// The queued nodes at `level`, in the order they would be handed out.
    pub fn queued_at(&self, level: i32) -> Vec<i64> {
        self.queues.get(level as usize).map(LinkedLongSet::to_vec).unwrap_or_default()
    }
}
