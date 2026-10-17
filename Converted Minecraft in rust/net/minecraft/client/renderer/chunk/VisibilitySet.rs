//! Port of: net/minecraft/client/renderer/chunk/VisibilitySet.java
//! Java class(es): net.minecraft.client.renderer.chunk.VisibilitySet
//! Status: VERIFIED
//!
//! Session 22. `public class VisibilitySet`: which faces of a chunk section can see which others
//! through the section, as a symmetric 6x6 bit matrix.
//!
//! # Storage
//!
//! Java holds a `BitSet(36)` indexed `a.ordinal() + b.ordinal() * 6`. A `BitSet` of 36 bits
//! allocates one 64-bit word, and `setAll` writes `set(0, data.size(), visible)` -- `size()` is the
//! ALLOCATED width, 64, not 36. The extra 28 bits are never read (every reader indexes below 36),
//! so a single `u64` reproduces it exactly, including `setAll` touching all 64.
//!
//! `set(a, b, v)` writes BOTH `[a][b]` and `[b][a]`, so the matrix stays symmetric; `add(set)` sets
//! every ordered pair of the given faces, a face with itself included.
//!
//! # `toString`
//!
//! A header of `' '` then `' ' + initial` for each face, then one row per face: its initial, then
//! for each column `"  "` on the diagonal and `' ' + ('Y' | 'n')` elsewhere, each line ending in
//! `'\n'`. The initial is `direction.toString().toUpperCase(Locale.ROOT).charAt(0)` -- `D U N S W E`.
//!
//! Verified by the `VISSET` group of `small_probe.txt` (`tests/session22.rs`): three scripts, the
//! full 6x6 matrix and `toString` after every step.

use crate::net::minecraft::core::Direction::{self as direction, Direction};

/// `Direction.values().length`.
const FACINGS: u32 = 6;

/// Port of `net.minecraft.client.renderer.chunk.VisibilitySet`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VisibilitySet {
    data: u64,
}

impl VisibilitySet {
    /// Port of the implicit constructor: nothing visible.
    pub const fn new() -> Self {
        VisibilitySet { data: 0 }
    }

    fn bit(a: Direction, b: Direction) -> u32 {
        a.ordinal() as u32 + b.ordinal() as u32 * FACINGS
    }

    /// Port of `add(Set<Direction> directions)`: every ordered pair of the given faces.
    pub fn add(&mut self, directions: &[Direction]) {
        for &direction1 in directions {
            for &direction2 in directions {
                self.set(direction1, direction2, true);
            }
        }
    }

    /// Port of `set(Direction direction1, Direction direction2, boolean value)`: both orders.
    pub fn set(&mut self, direction1: Direction, direction2: Direction, value: bool) {
        for bit in [Self::bit(direction1, direction2), Self::bit(direction2, direction1)] {
            if value {
                self.data |= 1u64 << bit;
            } else {
                self.data &= !(1u64 << bit);
            }
        }
    }

    /// Port of `setAll(boolean visible)`: `data.set(0, data.size(), visible)`, all 64 bits.
    pub fn set_all(&mut self, visible: bool) {
        self.data = if visible { u64::MAX } else { 0 };
    }

    /// Port of `visibilityBetween(Direction direction1, Direction direction2)`.
    pub const fn visibility_between(&self, direction1: Direction, direction2: Direction) -> bool {
        let bit = direction1.ordinal() as u32 + direction2.ordinal() as u32 * FACINGS;
        (self.data >> bit) & 1 != 0
    }
}

impl std::fmt::Display for VisibilitySet {
    /// Port of `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let initial = |d: Direction| d.name().chars().next().map(|c| c.to_ascii_uppercase()).unwrap_or(' ');
        let mut builder = String::new();
        builder.push(' ');
        for d in direction::VALUES {
            builder.push(' ');
            builder.push(initial(d));
        }
        builder.push('\n');
        for d1 in direction::VALUES {
            builder.push(initial(d1));
            for d2 in direction::VALUES {
                if d1 == d2 {
                    builder.push_str("  ");
                } else {
                    builder.push(' ');
                    builder.push(if self.visibility_between(d1, d2) { 'Y' } else { 'n' });
                }
            }
            builder.push('\n');
        }
        f.write_str(&builder)
    }
}
