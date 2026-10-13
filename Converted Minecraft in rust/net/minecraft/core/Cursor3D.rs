//! Port of: net/minecraft/core/Cursor3D.java
//! Java class(es): net.minecraft.core.Cursor3D
//! Status: VERIFIED
//!
//! A 3D box cursor: walks every integer position in a box, **x fastest, then y, then z**, and
//! classifies each position as inside / on a face / on an edge / on a corner.
//!
//! # THE INDEX DECOMPOSITION IS THE ORDER
//!
//! ```java
//! this.x = this.index % this.width;
//! int slice = this.index / this.width;
//! this.y = slice % this.height;
//! this.z = slice / this.height;
//! ```
//!
//! So `x` varies fastest, `y` next, `z` slowest -- the same nesting as `BlockPos#betweenClosed`,
//! and the traversal order IS the contract: the structure placer and `SectionPos`'s spliterator
//! both consume it, and a reordered walk changes which blocks win when two placements collide.
//!
//! # `end` IS COMPUTED ONCE, IN THE CONSTRUCTOR
//!
//! ```java
//! this.end = this.width * this.height * this.depth;
//! ```
//!
//! An `int` product computed **once**. A cursor over a huge box would overflow here and then
//! `advance` would return `false` early or never; the product is written the same way so the
//! overflow behaviour matches rather than being accidentally avoided by `checked_mul`.
//!
//! # `getNextType` IS NOT "HOW MANY COORDINATES ARE ON A BOUNDARY" IN GENERAL
//!
//! It counts, independently per axis, whether the current offset is `0` or `size - 1`. The sizes
//! come from the same `width`/`height`/`depth` as the walk, so for a **degenerate** box (a single
//! plane, or a single cell) one offset can satisfy the test on an axis in a way that does not
//! correspond to a real face. `TYPE_INSIDE = 0` therefore only occurs when every axis is
//! non-degenerate and the offset is interior.

/// Java: `Cursor3D.TYPE_INSIDE` = `0`.
pub const TYPE_INSIDE: i32 = 0;
/// Java: `Cursor3D.TYPE_FACE` = `1`.
pub const TYPE_FACE: i32 = 1;
/// Java: `Cursor3D.TYPE_EDGE` = `2`.
pub const TYPE_EDGE: i32 = 2;
/// Java: `Cursor3D.TYPE_CORNER` = `3`.
pub const TYPE_CORNER: i32 = 3;

/// Port of `net.minecraft.core.Cursor3D`.
#[derive(Clone, Debug)]
pub struct Cursor3D {
    origin_x: i32,
    origin_y: i32,
    origin_z: i32,
    width: i32,
    height: i32,
    depth: i32,
    end: i32,
    index: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl Cursor3D {
    /// Port of `new Cursor3D(minX, minY, minZ, maxX, maxY, maxZ)`.
    ///
    /// `max - min + 1` on each axis -- **inclusive** bounds, unlike `AABB`'s exclusive faces.
    /// `wrapping_sub` because Java's `int` subtraction wraps, and a caller passing
    /// `(Integer.MAX_VALUE, Integer.MIN_VALUE)` gets a wrapped size rather than an exception.
    #[allow(clippy::arithmetic_side_effects)] // Java's int arithmetic wraps; see above.
    pub fn new(min_x: i32, min_y: i32, min_z: i32, max_x: i32, max_y: i32, max_z: i32) -> Cursor3D {
        let width = max_x.wrapping_sub(min_x).wrapping_add(1);
        let height = max_y.wrapping_sub(min_y).wrapping_add(1);
        let depth = max_z.wrapping_sub(min_z).wrapping_add(1);
        let end = width.wrapping_mul(height).wrapping_mul(depth);
        Cursor3D {
            origin_x: min_x,
            origin_y: min_y,
            origin_z: min_z,
            width,
            height,
            depth,
            end,
            index: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    /// Port of `Cursor3D#advance()`.
    ///
    /// Returns `false` at the end **without** moving, so the caller can use `nextX()`/`nextY()`/
    /// `nextZ()` on the last successful step. Note the comparison is `==` against `end`, not
    /// `>=`: because `index` only ever increments while `< end`, the two are equivalent, and the
    /// `==` form is what the source says.
    ///
    /// `%` and `/` here are Java's truncating integer operators, so a negative `width` (from a
    /// wrapped constructor) gives a negative `x`. Reproduced by using the same operators rather
    /// than `rem_euclid`, which would differ on exactly that input.
    #[allow(clippy::arithmetic_side_effects)] // Java's int % and / truncate.
    pub fn advance(&mut self) -> bool {
        if self.index == self.end {
            return false;
        }
        self.x = self.index % self.width;
        let slice = self.index / self.width;
        self.y = slice % self.height;
        self.z = slice / self.height;
        self.index += 1;
        true
    }

    /// Port of `Cursor3D#nextX()`.
    #[inline]
    pub const fn next_x(&self) -> i32 {
        self.origin_x.wrapping_add(self.x)
    }

    /// Port of `Cursor3D#nextY()`.
    #[inline]
    pub const fn next_y(&self) -> i32 {
        self.origin_y.wrapping_add(self.y)
    }

    /// Port of `Cursor3D#nextZ()`.
    #[inline]
    pub const fn next_z(&self) -> i32 {
        self.origin_z.wrapping_add(self.z)
    }

    /// Port of `Cursor3D#getNextType()`.
    ///
    /// Counts, per axis, whether the current **offset** (not the world coordinate) sits at `0` or
    /// `size - 1`. Three axes therefore give `0..=3`. The three `if`s are independent and
    /// deliberately not folded into a single condition: the source shape is three separate
    /// increments, and a combined expression that short-circuits would under-count.
    #[inline]
    pub fn get_next_type(&self) -> i32 {
        let mut r#type = 0;
        if self.x == 0 || self.x == self.width.wrapping_sub(1) {
            r#type += 1;
        }
        if self.y == 0 || self.y == self.height.wrapping_sub(1) {
            r#type += 1;
        }
        if self.z == 0 || self.z == self.depth.wrapping_sub(1) {
            r#type += 1;
        }
        r#type
    }
}