//! Port of: net/minecraft/world/level/chunk/CarvingMask.java
//! Java class(es): net.minecraft.world.level.chunk.CarvingMask, net.minecraft.world.level.chunk.Mask
//! Status: VERIFIED
//!
//! Session 22. `public class CarvingMask`: which blocks of a chunk a carver has cut out, as one bit
//! per block in a `java.util.BitSet`, plus an optional extra predicate (`Mask`) that can declare
//! more blocks carved without setting bits.
//!
//! # The index
//!
//! `x & 15 | (z & 15) << 4 | y - minY << 8` -- precedence makes the last term `(y - minY) << 8`.
//! `x` and `z` are masked to the chunk, but `y` is not: a `y` below `minY` gives a NEGATIVE index,
//! and the `BitSet` throws `IndexOutOfBoundsException("bitIndex < 0: " + i)` (probed, for both `set`
//! and `get`).
//!
//! # `get` short-circuits
//!
//! `get(x, y, z)` is `additionalMask.test(x, y, z) || mask.get(index)`: when the extra mask says yes,
//! the bit set is NOT consulted, so a coordinate whose index would throw does not throw (probed).
//! The default extra mask is `(x, y, z) -> false`.
//!
//! # Construction and export
//!
//! * `new CarvingMask(height, minY)` sizes the bit set for `256 * height` bits (an `int` product); a
//!   negative product throws `NegativeArraySizeException("nbits < 0: " + n)` (probed).
//! * `new CarvingMask(long[] array, minY)` is `BitSet.valueOf(array)`, which drops trailing zero
//!   words; `toArray()` is `toLongArray()`, which returns words only up to the last non-zero one.
//! * `stream(chunkPos)` maps each set bit `i` to `chunkPos.getBlockAt(i & 15, (i >> 8) + minY,
//!   i >> 4 & 15)`, i.e. `new BlockPos(chunkX * 16 + x, y, chunkZ * 16 + z)`, in ascending bit order.
//!   `ChunkPos.getBlockAt` is not ported yet, so its one-line body is inlined here over the ported
//!   `get_block_x`/`get_block_z`.
//!
//! The bit set is `javacompat::bit_set::BitSet`. The two constructors are overloaded, so they are
//! [`CarvingMask::new`] and [`CarvingMask::from_array`].
//!
//! Verified by the `CARVING` group of `small_probe.txt` (`tests/session22.rs`): seven scripts over
//! both constructors, the extra mask, negative indices, `toArray` and `stream`.

use crate::javacompat::bit_set::BitSet;
use crate::net::minecraft::core::BlockPos::BlockPos;
use crate::net::minecraft::world::level::ChunkPos::ChunkPos;

/// Port of `CarvingMask.Mask`.
pub trait Mask {
    /// Port of `boolean test(int x, int y, int z)`.
    fn test(&self, x: i32, y: i32, z: i32) -> bool;
}

impl<F: Fn(i32, i32, i32) -> bool> Mask for F {
    fn test(&self, x: i32, y: i32, z: i32) -> bool {
        self(x, y, z)
    }
}

/// Port of `net.minecraft.world.level.chunk.CarvingMask`.
pub struct CarvingMask {
    min_y: i32,
    mask: BitSet,
    additional_mask: Box<dyn Mask>,
}

impl CarvingMask {
    /// Port of `new CarvingMask(int height, int minY)`.
    pub fn new(height: i32, min_y: i32) -> Self {
        CarvingMask {
            min_y,
            mask: BitSet::with_nbits(256i32.wrapping_mul(height)),
            additional_mask: Box::new(|_x: i32, _y: i32, _z: i32| false),
        }
    }

    /// Port of `new CarvingMask(long[] array, int minY)`: `BitSet.valueOf(array)`.
    pub fn from_array(array: &[i64], min_y: i32) -> Self {
        CarvingMask {
            min_y,
            mask: BitSet::value_of(array),
            additional_mask: Box::new(|_x: i32, _y: i32, _z: i32| false),
        }
    }

    /// Port of `setAdditionalMask(CarvingMask.Mask additionalMask)`.
    pub fn set_additional_mask(&mut self, additional_mask: impl Mask + 'static) {
        self.additional_mask = Box::new(additional_mask);
    }

    /// Port of `private int getIndex(int x, int y, int z)`.
    fn get_index(&self, x: i32, y: i32, z: i32) -> i32 {
        (x & 15) | ((z & 15) << 4) | (y.wrapping_sub(self.min_y) << 8)
    }

    /// Port of `set(int x, int y, int z)`.
    pub fn set(&mut self, x: i32, y: i32, z: i32) {
        let index = self.get_index(x, y, z);
        self.mask.set(index);
    }

    /// Port of `get(int x, int y, int z)`: the extra mask first, short-circuiting the bit set.
    pub fn get(&self, x: i32, y: i32, z: i32) -> bool {
        self.additional_mask.test(x, y, z) || self.mask.get(self.get_index(x, y, z))
    }

    /// Port of `stream(ChunkPos pos)`: each carved block, in ascending bit order.
    pub fn stream<'a>(&'a self, pos: &'a ChunkPos) -> impl Iterator<Item = BlockPos> + 'a {
        self.mask.stream().map(move |i| {
            let x = i & 15;
            let z = (i >> 4) & 15;
            let y = i >> 8;
            BlockPos::new(pos.get_block_x(x), y.wrapping_add(self.min_y), pos.get_block_z(z))
        })
    }

    /// Port of `toArray()`: `mask.toLongArray()`.
    pub fn to_array(&self) -> Vec<i64> {
        self.mask.to_long_array()
    }
}

impl std::fmt::Debug for CarvingMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CarvingMask").field("min_y", &self.min_y).field("mask", &self.mask).finish_non_exhaustive()
    }
}
