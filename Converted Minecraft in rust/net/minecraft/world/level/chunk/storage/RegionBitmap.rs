//! Port of: net/minecraft/world/level/chunk/storage/RegionBitmap.java
//! Java class(es): net.minecraft.world.level.chunk.storage.RegionBitmap
//! Status: PORTED
//!
//! Session 19. Every member is ported (`force`, `free`, `allocate`, `getUsed`), with the subset
//! of `java.util.BitSet` they use reproduced exactly, including its exception messages. Tested
//! in `_porting/tests/session19_consts.rs` against the Java source's semantics (no golden group
//! exists for this class).
//!
//! # WHAT IT IS
//!
//! The sector allocator of an `.mca` region file: bit `n` marks 4 KiB sector `n` as used.
//! `allocate(size)` is FIRST-FIT from sector 0 -- the lowest run of `size` free sectors, or the
//! end of the used area. Which sector a chunk lands in is visible in the file's bytes, so the
//! search order matters for byte-identical region files.

/// The parts of `java.util.BitSet` that `RegionBitmap` uses.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct JavaBitSet {
    words: Vec<u64>,
}

#[cold]
#[inline(never)]
fn index_out_of_bounds(message: String) -> ! {
    panic!("java.lang.IndexOutOfBoundsException: {message}")
}

impl JavaBitSet {
    fn check_range(from: i32, to: i32) {
        if from < 0 {
            index_out_of_bounds(format!("fromIndex < 0: {from}"));
        }
        if to < 0 {
            index_out_of_bounds(format!("toIndex < 0: {to}"));
        }
        if from > to {
            index_out_of_bounds(format!("fromIndex: {from} > toIndex: {to}"));
        }
    }

    fn get(&self, i: i32) -> bool {
        let w = (i / 64) as usize;
        w < self.words.len() && (self.words[w] >> (i % 64)) & 1 == 1
    }

    fn put(&mut self, i: i32, v: bool) {
        let w = (i / 64) as usize;
        if v {
            if w >= self.words.len() {
                self.words.resize(w + 1, 0);
            }
            self.words[w] |= 1u64 << (i % 64);
        } else if w < self.words.len() {
            self.words[w] &= !(1u64 << (i % 64));
        }
    }

    /// `BitSet#set(int fromIndex, int toIndex)`.
    fn set_range(&mut self, from: i32, to: i32) {
        Self::check_range(from, to);
        for i in from..to {
            self.put(i, true);
        }
    }

    /// `BitSet#clear(int fromIndex, int toIndex)`.
    fn clear_range(&mut self, from: i32, to: i32) {
        Self::check_range(from, to);
        for i in from..to {
            self.put(i, false);
        }
    }

    /// `BitSet#nextClearBit(int)`.
    fn next_clear_bit(&self, from: i32) -> i32 {
        if from < 0 {
            index_out_of_bounds(format!("fromIndex < 0: {from}"));
        }
        let mut i = from;
        while self.get(i) {
            i += 1;
        }
        i
    }

    /// `BitSet#nextSetBit(int)`: `-1` if there is none.
    fn next_set_bit(&self, from: i32) -> i32 {
        if from < 0 {
            index_out_of_bounds(format!("fromIndex < 0: {from}"));
        }
        let limit = (self.words.len() * 64) as i32;
        let mut i = from;
        while i < limit {
            if self.get(i) {
                return i;
            }
            i += 1;
        }
        -1
    }

    fn set_bits(&self) -> Vec<i32> {
        let limit = (self.words.len() * 64) as i32;
        (0..limit).filter(|i| self.get(*i)).collect()
    }
}

/// Port of `net.minecraft.world.level.chunk.storage.RegionBitmap`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegionBitmap {
    used: JavaBitSet,
}

impl RegionBitmap {
    /// `new RegionBitmap()`.
    pub fn new() -> RegionBitmap {
        RegionBitmap::default()
    }

    /// Port of `RegionBitmap#force(int, int)`: marks `[position, position + size)` used.
    pub fn force(&mut self, position: i32, size: i32) {
        self.used.set_range(position, position.wrapping_add(size));
    }

    /// Port of `RegionBitmap#free(int, int)`.
    pub fn free(&mut self, position: i32, size: i32) {
        self.used.clear_range(position, position.wrapping_add(size));
    }

    /// Port of `RegionBitmap#allocate(int)`: first fit, then marks the run used.
    pub fn allocate(&mut self, size: i32) -> i32 {
        let mut current = 0;
        loop {
            let free_start = self.used.next_clear_bit(current);
            let free_end = self.used.next_set_bit(free_start);
            if free_end == -1 || free_end - free_start >= size {
                self.force(free_start, size);
                return free_start;
            }
            current = free_end;
        }
    }

    /// Port of `RegionBitmap#getUsed()` (`@VisibleForTesting`): the used sectors. Java collects
    /// `BitSet.stream()` (ascending) into an `IntArraySet`, which iterates in insertion order --
    /// so ascending.
    pub fn get_used(&self) -> Vec<i32> {
        self.used.set_bits()
    }
}
