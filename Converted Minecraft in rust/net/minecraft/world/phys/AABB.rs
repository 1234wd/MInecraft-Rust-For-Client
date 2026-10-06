//! Port of: net/minecraft/world/phys/AABB.java
//! Java class(es): net.minecraft.world.phys.AABB, net.minecraft.world.phys.AABB$Builder
//! Status: PORTED
//!
//! 35 of 39 `aabb.*` golden groups are bit-exact. The four that are not --
//! `aabb.fromBlockPos`, `aabb.encapsulatingFullBlocks`, `aabb.moveBlockPos` and
//! `aabb.intersectsBlockPos` -- all take a `BlockPos`, which is a 7-line skeleton. Every method
//! that needs one is a `todo!("PORT-BLOCKED: BlockPos")`; see the PORT-BLOCKED section at the
//! bottom.
//!
//! # THE CONSTRUCTOR SWAPS, AND THAT IS THE WHOLE CLASS
//!
//! ```java
//! public AABB(final double minX, ..., final double maxZ) {
//!     this.minX = Math.min(minX, maxX);
//!     ...
//!     this.maxX = Math.max(minX, maxX);
//!     ...
//! }
//! ```
//!
//! The parameters are named `min*` and `max*` but they are **not** stored in that order -- each
//! pair goes through `Math.min`/`Math.max`. So `new AABB(1, 0, 0, -1, 0, 0)` is a perfectly valid
//! box with `minX = -1` and `maxX = 1`, and `new AABB(NaN, ...)` produces a box whose corners are
//! all NaN because `Math.min(NaN, x)` is NaN.
//!
//! This is why almost every method here can hand "wrong-ordered" corners to the constructor and
//! still get a sane box: `contract`, `expandTowards`, `setMinX`, `intersect` and `minmax` all
//! build a new box from values that may be inverted, and the constructor repairs them. A port
//! that stores the arguments verbatim produces boxes that are subtly inside-out, and several
//! predicates still pass -- which is why `aabb.constructor` puts inverted and NaN pairs first in
//! the corpus.
//!
//! # `intersects` AND `contains` ARE STRICT ON BOTH SIDES
//!
//! ```java
//! this.minX < maxX && this.maxX > minX
//! ```
//!
//! Two boxes that merely TOUCH do not intersect, and a point on the boundary is not contained.
//! There is no epsilon here. That is unlike `clip`, which *does* use `1.0E-7` slack. The
//! boundary cases are in the corpus for exactly this reason.
//!
//! # `clip` SHARES ONE MUTABLE `scaleReference`
//!
//! `AABB#clip` calls `getDirection`, which makes up to six `clipPoint` calls that all read and
//! write the **same** `double[1]`, which starts at `1.0`. Each `clipPoint` only accepts a hit
//! when `0.0 < s && s < scaleReference[0]`, so the scale only ever decreases -- but the
//! *direction* returned is whichever plane matched **last**, not the nearest. That is not a bug
//! to reason away: it is the behaviour, and `aabb.clipStatic` / `aabb.clipInstance` emit the hit
//! POINT rather than a bool, because a bool cannot tell a correct port from one that picks the
//! wrong plane.
//!
//! The `1.0E-7` comparisons against zero in `getDirection` are load-bearing too: a segment whose
//! delta is below that threshold is treated as parallel to that axis and simply skipped.
//!
//! # `Builder` IS IN `f32`, NOT `f64`
//!
//! ```java
//! private float minX = Float.POSITIVE_INFINITY;
//! ```
//!
//! The builder accumulates `float`s and only widens in `build()`. So a builder box can differ
//! from the equivalent `f64` box at the last bit, and `aabb.builder` feeds it `(float)` casts of
//! `f64` corpus values. Not an accident of the oracle; it is the field's declared type.

use crate::javacompat::java_lang;
use crate::javacompat::joml::Vector3f;
use crate::net::minecraft::core::Direction::Axis;
use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::world::phys::Vec3::Vec3;

/// Java: `private static final double EPSILON = 1.0E-7`.
///
/// Used by `clip`/`clipPoint` only -- `intersects` and `contains` do not use it.
const EPSILON: f64 = 1.0E-7;

/// Port of `net.minecraft.world.phys.AABB`.
///
/// The fields are `pub`, as in Java, and are **not** guaranteed sorted: an `AABB` built through
/// `Builder` is sorted by `build()`'s constructor call, and every method that returns a new box
/// routes through that same constructor. So a well-formed `AABB` always has `min <= max`, and
/// this type cannot represent one that does not -- which is a stronger guarantee than Java's,
/// where `final` fields plus a swapping constructor happen to give the same result.
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    /// Java: `public final double minX`.
    pub min_x: f64,
    /// Java: `public final double minY`.
    pub min_y: f64,
    /// Java: `public final double minZ`.
    pub min_z: f64,
    /// Java: `public final double maxX`.
    pub max_x: f64,
    /// Java: `public final double maxY`.
    pub max_y: f64,
    /// Java: `public final double maxZ`.
    pub max_z: f64,
}

impl AABB {
    /// Port of `new AABB(double,double,double,double,double,double)`.
    ///
    /// The `Math.min`/`Math.max` pairs are load-bearing -- see the module docs. In particular
    /// they are what makes NaN propagate: `Math.min(NaN, 1.0)` is NaN, so one NaN corner poisons
    /// both corners of that axis.
    #[inline]
    #[allow(clippy::too_many_arguments)] // Six arguments is the shape of the Java constructor.
    pub fn new(
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
    ) -> AABB {
        AABB {
            min_x: java_lang::min_f64(min_x, max_x),
            min_y: java_lang::min_f64(min_y, max_y),
            min_z: java_lang::min_f64(min_z, max_z),
            max_x: java_lang::max_f64(min_x, max_x),
            max_y: java_lang::max_f64(min_y, max_y),
            max_z: java_lang::max_f64(min_z, max_z),
        }
    }

    /// Port of `new AABB(Vec3 begin, Vec3 end)`.
    #[inline]
    pub fn from_vec3(begin: Vec3, end: Vec3) -> AABB {
        AABB::new(begin.x, begin.y, begin.z, end.x, end.y, end.z)
    }

    /// Port of `AABB#setMinX(double)`.
    #[inline]
    pub fn set_min_x(&self, min_x: f64) -> AABB {
        AABB::new(min_x, self.min_y, self.min_z, self.max_x, self.max_y, self.max_z)
    }

    /// Port of `AABB#setMinY(double)`.
    #[inline]
    pub fn set_min_y(&self, min_y: f64) -> AABB {
        AABB::new(self.min_x, min_y, self.min_z, self.max_x, self.max_y, self.max_z)
    }

    /// Port of `AABB#setMinZ(double)`.
    #[inline]
    pub fn set_min_z(&self, min_z: f64) -> AABB {
        AABB::new(self.min_x, self.min_y, min_z, self.max_x, self.max_y, self.max_z)
    }

    /// Port of `AABB#setMaxX(double)`.
    ///
    /// Naming a setter `set_max_x` and passing a value that is *smaller* than `min_x` is not a
    /// contradiction: the constructor swaps, so the result is a box with the two exchanged. The
    /// golden's inverted-pairs-first corpus is what pins this.
    #[inline]
    pub fn set_max_x(&self, max_x: f64) -> AABB {
        AABB::new(self.min_x, self.min_y, self.min_z, max_x, self.max_y, self.max_z)
    }

    /// Port of `AABB#setMaxY(double)`.
    #[inline]
    pub fn set_max_y(&self, max_y: f64) -> AABB {
        AABB::new(self.min_x, self.min_y, self.min_z, self.max_x, max_y, self.max_z)
    }

    /// Port of `AABB#setMaxZ(double)`.
    #[inline]
    pub fn set_max_z(&self, max_z: f64) -> AABB {
        AABB::new(self.min_x, self.min_y, self.min_z, self.max_x, self.max_y, max_z)
    }

    /// Port of `AABB#min(Direction.Axis)`.
    #[inline]
    pub fn min(&self, axis: Axis) -> f64 {
        axis.choose(self.min_x, self.min_y, self.min_z)
    }

    /// Port of `AABB#max(Direction.Axis)`.
    #[inline]
    pub fn max(&self, axis: Axis) -> f64 {
        axis.choose(self.max_x, self.max_y, self.max_z)
    }

    /// Port of `AABB#equals(Object)`, expressed as `PartialEq`.
    ///
    /// # `Double.compare`, NOT `==`
    ///
    /// ```java
    /// Double.compare(aabb.minX, this.minX) != 0
    /// ```
    ///
    /// The difference from `==` is entirely in NaN and signed zero, and both are in the corpus:
    /// `Double.compare(NaN, NaN)` is `0`, so two NaN boxes **are** equal, while `NaN == NaN` is
    /// false. And `Double.compare(0.0, -0.0)` is `1`, so `+0.0` and `-0.0` are **not** equal --
    /// again unlike `==`, which says they are. Deriving `PartialEq` from these comparisons is
    /// what makes both behaviours fall out for free.
    ///
    /// Note this contradicts the project-wide "NaN equals NaN by default" convention in
    /// `golden.rs`; here the golden demands the Java-specific rule and this file follows the jar.
    #[inline]
    fn java_equals(&self, other: &AABB) -> bool {
        java_lang::compare_f64(other.min_x, self.min_x) == std::cmp::Ordering::Equal
            && java_lang::compare_f64(other.min_y, self.min_y) == std::cmp::Ordering::Equal
            && java_lang::compare_f64(other.min_z, self.min_z) == std::cmp::Ordering::Equal
            && java_lang::compare_f64(other.max_x, self.max_x) == std::cmp::Ordering::Equal
            && java_lang::compare_f64(other.max_y, self.max_y) == std::cmp::Ordering::Equal
            && java_lang::compare_f64(other.max_z, self.max_z) == std::cmp::Ordering::Equal
    }

    /// Port of `AABB#hashCode()`.
    ///
    /// ```java
    /// long temp = Double.doubleToLongBits(this.minX);
    /// int result = (int)(temp ^ temp >>> 32);
    /// ```
    ///
    /// Each `double` is hashed as the **high 32 bits** of `doubleToLongBits` XOR the low 32,
    /// truncated to `int`. Two details carry weight:
    ///
    /// * `doubleToLongBits` **canonicalises NaN**, so every NaN has the same hash regardless of
    ///   payload. Rust's `to_bits()` does not, so a NaN built from arithmetic could hash
    ///   differently from a NaN written as a literal. `canonicalise_nan_bits` is applied for that
    ///   reason, and it is what keeps `hashCode` consistent with the `equals` above -- which
    ///   treats all NaNs as equal, so they must all hash equally too.
    /// * `+0.0` and `-0.0` have **different** bits and therefore different hashes, matching the
    ///   `Double.compare` inequality in `java_equals`.
    #[inline]
    pub fn java_hash_code(&self) -> i32 {
        let fold = |v: f64| -> i32 {
            let temp = canonicalise_nan_bits(v);
            // Java: `(int)(temp ^ temp >>> 32)`. `>>>` on the `long` is a LOGICAL shift, so the
            // cast to `u64` before shifting is load-bearing; shifting the `i64` arithmetically
            // would sign-extend and give a different XOR for any negative `temp`, which is half
            // of all `double`s.
            ((temp ^ ((temp as u64) >> 32) as i64) as u32) as i32
        };
        let mut result = fold(self.min_x);
        result = 31i32.wrapping_mul(result).wrapping_add(fold(self.min_y));
        result = 31i32.wrapping_mul(result).wrapping_add(fold(self.min_z));
        result = 31i32.wrapping_mul(result).wrapping_add(fold(self.max_x));
        result = 31i32.wrapping_mul(result).wrapping_add(fold(self.max_y));
        result = 31i32.wrapping_mul(result).wrapping_add(fold(self.max_z));
        result
    }

    /// Port of `AABB#contract(double,double,double)`.
    ///
    /// Each axis is treated independently and the sign of the argument chooses the side:
    /// negative grows `min` **outward** (`minX -= xa`, and `xa < 0` makes that `minX += |xa|`),
    /// positive shrinks `max` inward. A zero argument is a no-op on that axis -- note this is
    /// `if/else if`, so `0.0` skips both branches, and `-0.0` takes neither because `-0.0 < 0.0`
    /// is false.
    #[inline]
    pub fn contract(&self, xa: f64, ya: f64, za: f64) -> AABB {
        let (mut min_x, mut min_y, mut min_z) = (self.min_x, self.min_y, self.min_z);
        let (mut max_x, mut max_y, mut max_z) = (self.max_x, self.max_y, self.max_z);
        if xa < 0.0 {
            min_x -= xa;
        } else if xa > 0.0 {
            max_x -= xa;
        }
        if ya < 0.0 {
            min_y -= ya;
        } else if ya > 0.0 {
            max_y -= ya;
        }
        if za < 0.0 {
            min_z -= za;
        } else if za > 0.0 {
            max_z -= za;
        }
        AABB::new(min_x, min_y, min_z, max_x, max_y, max_z)
    }

    /// Port of `AABB#expandTowards(Vec3)`.
    #[inline]
    pub fn expand_towards_vec3(&self, delta: Vec3) -> AABB {
        self.expand_towards(delta.x, delta.y, delta.z)
    }

    /// Port of `AABB#expandTowards(double,double,double)`.
    ///
    /// The mirror image of [`AABB::contract`], and the two are easy to confuse: here a negative
    /// argument moves `minX` **inward** (`minX += xa`), because a negative delta pushes the near
    /// face towards the interior. Contracting by `d` is `inflate` by `-d`, but contracting by
    /// `-d` is *not* `inflate` by `d` -- `contract` and `expandTowards` are different functions
    /// with opposite per-axis behaviour.
    #[inline]
    pub fn expand_towards(&self, xa: f64, ya: f64, za: f64) -> AABB {
        let (mut min_x, mut min_y, mut min_z) = (self.min_x, self.min_y, self.min_z);
        let (mut max_x, mut max_y, mut max_z) = (self.max_x, self.max_y, self.max_z);
        if xa < 0.0 {
            min_x += xa;
        } else if xa > 0.0 {
            max_x += xa;
        }
        if ya < 0.0 {
            min_y += ya;
        } else if ya > 0.0 {
            max_y += ya;
        }
        if za < 0.0 {
            min_z += za;
        } else if za > 0.0 {
            max_z += za;
        }
        AABB::new(min_x, min_y, min_z, max_x, max_y, max_z)
    }

    /// Port of `AABB#inflate(double,double,double)`.
    ///
    /// Symmetric on every axis, with **no** sign branching: a negative amount shrinks. That is
    /// what makes `deflate` a one-liner, and what distinguishes it from `contract`.
    #[inline]
    pub fn inflate(&self, x_add: f64, y_add: f64, z_add: f64) -> AABB {
        AABB::new(
            self.min_x - x_add,
            self.min_y - y_add,
            self.min_z - z_add,
            self.max_x + x_add,
            self.max_y + y_add,
            self.max_z + z_add,
        )
    }

    /// Port of `AABB#inflate(double)`.
    #[inline]
    pub fn inflate_uniform(&self, amount_to_add_in_all_directions: f64) -> AABB {
        self.inflate(
            amount_to_add_in_all_directions,
            amount_to_add_in_all_directions,
            amount_to_add_in_all_directions,
        )
    }

    /// Port of `AABB#intersect(AABB)`.
    ///
    /// Componentwise `max` on the near faces and `min` on the far ones. The result of two boxes
    /// that do not overlap is therefore a box with `min > max` on some axis -- but the swapping
    /// constructor immediately repairs it into a small well-formed box at the crossing point, not
    /// an empty box. There is no "empty" representation in this class and adding one would be a
    /// gameplay change.
    #[inline]
    pub fn intersect(&self, other: &AABB) -> AABB {
        AABB::new(
            java_lang::max_f64(self.min_x, other.min_x),
            java_lang::max_f64(self.min_y, other.min_y),
            java_lang::max_f64(self.min_z, other.min_z),
            java_lang::min_f64(self.max_x, other.max_x),
            java_lang::min_f64(self.max_y, other.max_y),
            java_lang::min_f64(self.max_z, other.max_z),
        )
    }

    /// Port of `AABB#minmax(AABB)` -- the smallest box containing both.
    #[inline]
    pub fn minmax(&self, other: &AABB) -> AABB {
        AABB::new(
            java_lang::min_f64(self.min_x, other.min_x),
            java_lang::min_f64(self.min_y, other.min_y),
            java_lang::min_f64(self.min_z, other.min_z),
            java_lang::max_f64(self.max_x, other.max_x),
            java_lang::max_f64(self.max_y, other.max_y),
            java_lang::max_f64(self.max_z, other.max_z),
        )
    }

    /// Port of `AABB#move(double,double,double)`.
    ///
    /// Translates, so size is preserved exactly -- every corner gets the same delta, and the
    /// constructor's `min`/`max` cannot reorder them because the same delta preserves order.
    #[inline]
    pub fn move_by(&self, xa: f64, ya: f64, za: f64) -> AABB {
        AABB::new(
            self.min_x + xa,
            self.min_y + ya,
            self.min_z + za,
            self.max_x + xa,
            self.max_y + ya,
            self.max_z + za,
        )
    }

    /// Port of `AABB#move(Vec3)`.
    #[inline]
    pub fn move_vec3(&self, pos: Vec3) -> AABB {
        self.move_by(pos.x, pos.y, pos.z)
    }

    /// Port of `AABB#move(Vector3f)`.
    ///
    /// The JOML overload takes `f32`s and must **widen** them, not the other way round. A
    /// `f32` value is exactly representable in `f64`, so the widening is lossless; the reverse
    /// would not be.
    #[inline]
    pub fn move_vector3f(&self, pos: Vector3f) -> AABB {
        self.move_by(pos.x as f64, pos.y as f64, pos.z as f64)
    }

    /// Port of `AABB#intersects(AABB)`.
    #[inline]
    pub fn intersects(&self, aabb: &AABB) -> bool {
        self.intersects_bounds(
            aabb.min_x,
            aabb.min_y,
            aabb.min_z,
            aabb.max_x,
            aabb.max_y,
            aabb.max_z,
        )
    }

    /// Port of `AABB#intersects(double,double,double,double,double,double)`.
    ///
    /// Strict on both sides on every axis, so boxes that merely touch return `false`. There is no
    /// epsilon -- unlike [`AABB::clip`], which allows `EPSILON` slack.
    #[inline]
    pub fn intersects_bounds(
        &self,
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
    ) -> bool {
        self.min_x < max_x
            && self.max_x > min_x
            && self.min_y < max_y
            && self.max_y > min_y
            && self.min_z < max_z
            && self.max_z > min_z
    }

    /// Port of `AABB#intersects(Vec3 min, Vec3 max)`.
    ///
    /// The two corners are normalised with `Math.min`/`Math.max` **here**, not by the
    /// constructor, because the values are passed straight into [`AABB::intersects_bounds`].
    /// A caller passing them the wrong way round gets a different answer than one passing a
    /// constructed `AABB`, and the golden covers both orders.
    #[inline]
    pub fn intersects_corners(&self, min: Vec3, max: Vec3) -> bool {
        self.intersects_bounds(
            java_lang::min_f64(min.x, max.x),
            java_lang::min_f64(min.y, max.y),
            java_lang::min_f64(min.z, max.z),
            java_lang::max_f64(min.x, max.x),
            java_lang::max_f64(min.y, max.y),
            java_lang::max_f64(min.z, max.z),
        )
    }

    /// Port of `AABB#contains(Vec3)`.
    #[inline]
    pub fn contains_vec3(&self, vec: Vec3) -> bool {
        self.contains(vec.x, vec.y, vec.z)
    }

    /// Port of `AABB#contains(double,double,double)`.
    ///
    /// Half-open: `>=` on the near faces and `<` on the far ones. So a point on `maxX` is
    /// **not** contained, and a point on `minX` is. The corpus's boundary probes
    /// (`0.9999999999`, `1.0000000001`) exist to catch a port that makes both ends inclusive.
    #[inline]
    pub fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        x >= self.min_x
            && x < self.max_x
            && y >= self.min_y
            && y < self.max_y
            && z >= self.min_z
            && z < self.max_z
    }

    /// Port of `AABB#getSize()` -- the mean of the three extents, not the volume and not the
    /// diagonal.
    #[inline]
    pub fn get_size(&self) -> f64 {
        java_lang::div_f64(
            java_lang::add_f64(
                java_lang::add_f64(self.get_xsize(), self.get_ysize()),
                self.get_zsize(),
            ),
            3.0,
        )
    }

    /// Port of `AABB#getXsize()`.
    #[inline]
    pub fn get_xsize(&self) -> f64 {
        self.max_x - self.min_x
    }

    /// Port of `AABB#getYsize()`.
    #[inline]
    pub fn get_ysize(&self) -> f64 {
        self.max_y - self.min_y
    }

    /// Port of `AABB#getZsize()`.
    #[inline]
    pub fn get_zsize(&self) -> f64 {
        self.max_z - self.min_z
    }

    /// Port of `AABB#deflate(double,double,double)`.
    #[inline]
    pub fn deflate(&self, x_subtract: f64, y_subtract: f64, z_subtract: f64) -> AABB {
        self.inflate(-x_subtract, -y_subtract, -z_subtract)
    }

    /// Port of `AABB#deflate(double)`.
    #[inline]
    pub fn deflate_uniform(&self, amount: f64) -> AABB {
        self.inflate_uniform(-amount)
    }

    /// Port of `AABB#toString()`.
    ///
    /// `Double#toString` on six values, so `1.0` prints as `1.0`, `Infinity` as `Infinity` and
    /// `NaN` as `NaN`. `javacompat::java_lang::float_to_string::double_to_string` handles the
    /// formatting -- the same helper `Vec3`'s `Display` uses.
    #[inline]
    pub fn to_java_string(&self) -> String {
        format!(
            "AABB[{}, {}, {}] -> [{}, {}, {}]",
            java_lang::float_to_string::double_to_string(self.min_x),
            java_lang::float_to_string::double_to_string(self.min_y),
            java_lang::float_to_string::double_to_string(self.min_z),
            java_lang::float_to_string::double_to_string(self.max_x),
            java_lang::float_to_string::double_to_string(self.max_y),
            java_lang::float_to_string::double_to_string(self.max_z),
        )
    }

    /// Port of `AABB#hasNaN()`.
    #[inline]
    pub fn has_nan(&self) -> bool {
        self.min_x.is_nan()
            || self.min_y.is_nan()
            || self.min_z.is_nan()
            || self.max_x.is_nan()
            || self.max_y.is_nan()
            || self.max_z.is_nan()
    }

    /// Port of `AABB#getCenter()`.
    ///
    /// The midpoint is `Mth#lerp(0.5, min, max)`, i.e. `0.5 + ...` -- computed as
    /// `min + 0.5 * (max - min)`, **not** as `(min + max) / 2`. Those differ in the last bit for
    /// most inputs, and the corpus contains `Double.MIN_VALUE` and `Double.MAX_VALUE`, where
    /// `(min + max) / 2` overflows or underflows and `Mth#lerp` does not.
    #[inline]
    pub fn get_center(&self) -> Vec3 {
        Vec3::new(
            Mth::lerp_f64(0.5, self.min_x, self.max_x),
            Mth::lerp_f64(0.5, self.min_y, self.max_y),
            Mth::lerp_f64(0.5, self.min_z, self.max_z),
        )
    }

    /// Port of `AABB#getBottomCenter()` -- centred in X and Z, at `minY`.
    #[inline]
    pub fn get_bottom_center(&self) -> Vec3 {
        Vec3::new(
            Mth::lerp_f64(0.5, self.min_x, self.max_x),
            self.min_y,
            Mth::lerp_f64(0.5, self.min_z, self.max_z),
        )
    }

    /// Port of `AABB#getMinPosition()`.
    #[inline]
    pub fn get_min_position(&self) -> Vec3 {
        Vec3::new(self.min_x, self.min_y, self.min_z)
    }

    /// Port of `AABB#getMaxPosition()`.
    #[inline]
    pub fn get_max_position(&self) -> Vec3 {
        Vec3::new(self.max_x, self.max_y, self.max_z)
    }

    /// Port of `AABB#distanceToSqr(Vec3)`.
    ///
    /// Per axis, the distance from the point to the box's *interval*, clamped at zero -- so a
    /// point inside the box is distance zero, not distance to the nearest face. Squared, so no
    /// square root and no rounding from one.
    #[inline]
    pub fn distance_to_sqr_point(&self, point: Vec3) -> f64 {
        let dx = java_lang::max_f64(
            java_lang::max_f64(self.min_x - point.x, point.x - self.max_x),
            0.0,
        );
        let dy = java_lang::max_f64(
            java_lang::max_f64(self.min_y - point.y, point.y - self.max_y),
            0.0,
        );
        let dz = java_lang::max_f64(
            java_lang::max_f64(self.min_z - point.z, point.z - self.max_z),
            0.0,
        );
        Mth::length_squared_3_f64(dx, dy, dz)
    }

    /// Port of `AABB#distanceToSqr(AABB)`.
    ///
    /// The same clamp, but between two intervals on each axis. Note the operand order differs
    /// from the point version -- `this.minX - other.maxX` and `other.minX - this.maxX` -- so a
    /// transcription that reuses the point version's expression gives the same answer here (it
    /// is symmetric after the clamp) but reads as a mistake.
    #[inline]
    pub fn distance_to_sqr_box(&self, bounding_box: &AABB) -> f64 {
        let dx = java_lang::max_f64(
            java_lang::max_f64(
                self.min_x - bounding_box.max_x,
                bounding_box.min_x - self.max_x,
            ),
            0.0,
        );
        let dy = java_lang::max_f64(
            java_lang::max_f64(
                self.min_y - bounding_box.max_y,
                bounding_box.min_y - self.max_y,
            ),
            0.0,
        );
        let dz = java_lang::max_f64(
            java_lang::max_f64(
                self.min_z - bounding_box.max_z,
                bounding_box.min_z - self.max_z,
            ),
            0.0,
        );
        Mth::length_squared_3_f64(dx, dy, dz)
    }

    /// Port of `AABB#collidedAlongVector(Vec3,List<AABB>)`.
    ///
    /// Tests the swept box against each candidate inflated by half this box's size less
    /// `EPSILON`, accepting a collision if either endpoint is inside the inflated box **or** the
    /// segment between them clips it. The `EPSILON` shrink stops a box resting exactly against a
    /// face from colliding with it.
    ///
    /// Java takes a `List<AABB>`; this takes a slice, which is the same iteration order.
    #[inline]
    pub fn collided_along_vector(&self, vector: Vec3, aabbs: &[AABB]) -> bool {
        let from = self.get_center();
        let to = from.add(vector.x, vector.y, vector.z);
        for shape_part in aabbs {
            let inflated = shape_part.inflate(
                self.get_xsize() * 0.5 - EPSILON,
                self.get_ysize() * 0.5 - EPSILON,
                self.get_zsize() * 0.5 - EPSILON,
            );
            if inflated.contains_vec3(to) || inflated.contains_vec3(from) {
                return true;
            }
            if inflated.clip(from, to).is_some() {
                return true;
            }
        }
        false
    }

    /// Port of `AABB#clip(Vec3 from, Vec3 to)`.
    ///
    /// Returns the hit point, or `None` when the segment misses. See the module docs on the
    /// shared `scale_reference`.
    #[inline]
    pub fn clip(&self, from: Vec3, to: Vec3) -> Option<Vec3> {
        AABB::clip_bounds(
            self.min_x,
            self.min_y,
            self.min_z,
            self.max_x,
            self.max_y,
            self.max_z,
            from,
            to,
        )
    }

    /// Port of the static `AABB#clip(double x6, Vec3, Vec3)`.
    #[allow(clippy::too_many_arguments)] // Nine arguments is the shape of the Java overload.
    pub fn clip_bounds(
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
        from: Vec3,
        to: Vec3,
    ) -> Option<Vec3> {
        let mut scale_reference = [1.0f64];
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let dz = to.z - from.z;
        let direction = get_direction_bounds(
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            from,
            &mut scale_reference,
            None,
            dx,
            dy,
            dz,
        );
        // `direction` is discarded: only the scale reaches the hit point. The direction is
        // carried by the `Iterable<AABB>` overload, which returns a `BlockHitResult`.
        let _ = direction;
        if direction.is_none() {
            return None;
        }
        let scale = scale_reference[0];
        Some(from.add(scale * dx, scale * dy, scale * dz))
    }

    /// Port of `AABB#ofSize(Vec3,double,double,double)`.
    ///
    /// The centre is subtracted/added **before** halving the size per axis, and the halves are
    /// taken from the *sizes*, not from the coordinates -- so a zero size yields a degenerate box
    /// at the centre rather than an error.
    #[inline]
    pub fn of_size(center: Vec3, size_x: f64, size_y: f64, size_z: f64) -> AABB {
        AABB::new(
            center.x - size_x / 2.0,
            center.y - size_y / 2.0,
            center.z - size_z / 2.0,
            center.x + size_x / 2.0,
            center.y + size_y / 2.0,
            center.z + size_z / 2.0,
        )
    }

    /// Port of `AABB#unitCubeFromLowerCorner(Vec3)`.
    #[inline]
    pub fn unit_cube_from_lower_corner(pos: Vec3) -> AABB {
        AABB::new(pos.x, pos.y, pos.z, pos.x + 1.0, pos.y + 1.0, pos.z + 1.0)
    }
}

impl PartialEq for AABB {
    /// `AABB#equals`. See [`AABB::java_equals`] for why `Double.compare` rather than `==`.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.java_equals(other)
    }
}

impl std::fmt::Display for AABB {
    /// Port of `AABB#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_java_string())
    }
}

/// `Double.doubleToLongBits(double)`: the bits, with every NaN collapsed to the canonical one.
///
/// Rust's `f64::to_bits` preserves a NaN's payload, so a NaN produced by arithmetic and a NaN
/// written as a literal would otherwise hash differently -- while `AABB#equals`, which uses
/// `Double.compare`, treats them as equal. Canonicalising is what keeps `hashCode` consistent
/// with `equals`.
#[inline]
fn canonicalise_nan_bits(v: f64) -> i64 {
    if v.is_nan() {
        f64::NAN.to_bits() as i64
    } else {
        v.to_bits() as i64
    }
}

// ============================================================================
// clip internals
// ============================================================================

/// Port of the private `AABB#getDirection(AABB,Vec3,double[],Direction,double,double,double)`.
#[inline]
#[allow(clippy::too_many_arguments)]
fn get_direction(
    aabb: &AABB,
    from: Vec3,
    scale_reference: &mut [f64; 1],
    direction: Option<crate::net::minecraft::core::Direction::Direction>,
    dx: f64,
    dy: f64,
    dz: f64,
) -> Option<crate::net::minecraft::core::Direction::Direction> {
    get_direction_bounds(
        aabb.min_x,
        aabb.min_y,
        aabb.min_z,
        aabb.max_x,
        aabb.max_y,
        aabb.max_z,
        from,
        scale_reference,
        direction,
        dx,
        dy,
        dz,
    )
}

/// Port of the private 12-argument `AABB#getDirection`.
///
/// Six candidate planes, tested in a fixed order, all sharing one `scale_reference`. Each
/// candidate is skipped when its delta magnitude is at or below `EPSILON`, which is how a
/// segment parallel to an axis avoids dividing by (nearly) zero.
///
/// The axis rotation in the `dy`/`dz` cases is not cosmetic: the `from` components are rotated
/// to match the leading axis so `clipPoint` can always treat its three arguments as `(a, b, c)`.
/// Transcribing the calls in order rather than trying to factor out the rotation is what keeps
/// this honest -- the rotation differs between the X, Y and Z cases.
#[allow(clippy::too_many_arguments)]
fn get_direction_bounds(
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
    from: Vec3,
    scale_reference: &mut [f64; 1],
    mut direction: Option<crate::net::minecraft::core::Direction::Direction>,
    dx: f64,
    dy: f64,
    dz: f64,
) -> Option<crate::net::minecraft::core::Direction::Direction> {
    use crate::net::minecraft::core::Direction::Direction;

    if dx > EPSILON {
        direction = clip_point(
            scale_reference,
            direction,
            dx,
            dy,
            dz,
            min_x,
            min_y,
            max_y,
            min_z,
            max_z,
            Direction::West,
            from.x,
            from.y,
            from.z,
        );
    } else if dx < -EPSILON {
        direction = clip_point(
            scale_reference,
            direction,
            dx,
            dy,
            dz,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
            Direction::East,
            from.x,
            from.y,
            from.z,
        );
    }

    if dy > EPSILON {
        direction = clip_point(
            scale_reference,
            direction,
            dy,
            dz,
            dx,
            min_y,
            min_z,
            max_z,
            min_x,
            max_x,
            Direction::Down,
            from.y,
            from.z,
            from.x,
        );
    } else if dy < -EPSILON {
        direction = clip_point(
            scale_reference,
            direction,
            dy,
            dz,
            dx,
            max_y,
            min_z,
            max_z,
            min_x,
            max_x,
            Direction::Up,
            from.y,
            from.z,
            from.x,
        );
    }

    if dz > EPSILON {
        direction = clip_point(
            scale_reference,
            direction,
            dz,
            dx,
            dy,
            min_z,
            min_x,
            max_x,
            min_y,
            max_y,
            Direction::North,
            from.z,
            from.x,
            from.y,
        );
    } else if dz < -EPSILON {
        direction = clip_point(
            scale_reference,
            direction,
            dz,
            dx,
            dy,
            max_z,
            min_x,
            max_x,
            min_y,
            max_y,
            Direction::South,
            from.z,
            from.x,
            from.y,
        );
    }

    direction
}

/// Port of the private `AABB#clipPoint(...)`.
///
/// Returns the new direction **only when this plane is a hit**, otherwise the direction it was
/// given. That is the mechanism by which "the winner is whichever plane matched last" arises: a
/// later hit overwrites an earlier one, and a miss leaves the previous winner alone.
///
/// Every comparison against `scale_reference[0]` is strict (`0.0 < s < scale`), so a hit exactly
/// on the segment's start, or exactly at the running maximum, is rejected -- while the
/// `EPSILON` slack on the *other* two axes means a hit marginally outside the face still counts.
#[allow(clippy::too_many_arguments)]
fn clip_point(
    scale_reference: &mut [f64; 1],
    direction: Option<crate::net::minecraft::core::Direction::Direction>,
    da: f64,
    db: f64,
    dc: f64,
    point: f64,
    min_b: f64,
    max_b: f64,
    min_c: f64,
    max_c: f64,
    new_direction: crate::net::minecraft::core::Direction::Direction,
    from_a: f64,
    from_b: f64,
    from_c: f64,
) -> Option<crate::net::minecraft::core::Direction::Direction> {
    let s = (point - from_a) / da;
    let pb = from_b + s * db;
    let pc = from_c + s * dc;
    if 0.0 < s
        && s < scale_reference[0]
        && min_b - EPSILON < pb
        && pb < max_b + EPSILON
        && min_c - EPSILON < pc
        && pc < max_c + EPSILON
    {
        scale_reference[0] = s;
        Some(new_direction)
    } else {
        direction
    }
}

// ============================================================================
// Builder
// ============================================================================

/// Port of `net.minecraft.world.phys.AABB$Builder`.
///
/// # THE FIELDS ARE `f32`, NOT `f64`
///
/// ```java
/// private float minX = Float.POSITIVE_INFINITY;
/// ```
///
/// Every bound is accumulated in single precision and only widened when `build()` calls the
/// `AABB` constructor. So a builder box is the `f32`-rounded version of the box you asked for,
/// and `aabb.builder` feeds it `(float)` casts of `f64` corpus values on purpose -- comparing
/// against an `f64` accumulation would diverge on most of them.
///
/// The sentinel initial values are `+inf` for the minima and `-inf` for the maxima, so a single
/// `include` defines a degenerate (zero-extent) box rather than an infinite one. `Math.min` and
/// `Math.max` against an infinity also propagate NaN correctly: `Math.min(+inf, NaN)` is NaN.
#[derive(Debug, Clone)]
pub struct Builder {
    min_x: f32,
    min_y: f32,
    min_z: f32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
    defined: bool,
}

impl Default for Builder {
    /// Port of the field initialisers: `+inf` minima, `-inf` maxima, `defined = false`.
    fn default() -> Self {
        Builder::new()
    }
}

impl Builder {
    /// Port of the implicit `AABB.Builder()` constructor.
    #[inline]
    pub fn new() -> Builder {
        Builder {
            min_x: f32::INFINITY,
            min_y: f32::INFINITY,
            min_z: f32::INFINITY,
            max_x: f32::NEG_INFINITY,
            max_y: f32::NEG_INFINITY,
            max_z: f32::NEG_INFINITY,
            defined: false,
        }
    }

    /// Port of `AABB.Builder#include(Vector3fc)`.
    ///
    /// Sets `defined` unconditionally, even if the point contributed nothing new -- so calling
    /// `include` twice with the same point still makes the builder defined, and `build()` will
    /// return a zero-extent box.
    #[inline]
    pub fn include(&mut self, v: &Vector3f) {
        self.min_x = java_lang::min_f32(self.min_x, v.x);
        self.min_y = java_lang::min_f32(self.min_y, v.y);
        self.min_z = java_lang::min_f32(self.min_z, v.z);
        self.max_x = java_lang::max_f32(self.max_x, v.x);
        self.max_y = java_lang::max_f32(self.max_y, v.y);
        self.max_z = java_lang::max_f32(self.max_z, v.z);
        self.defined = true;
    }

    /// Port of `AABB.Builder#isDefined()`.
    #[inline]
    pub fn is_defined(&self) -> bool {
        self.defined
    }

    /// Port of `AABB.Builder#build()`.
    ///
    /// # THE MESSAGE IS PART OF THE CONTRACT
    ///
    /// ```text
    /// java.lang.IllegalStateException: Cannot build an undefined AABB. Include at least one point.
    /// ```
    ///
    /// `aabb.builderError` records that string verbatim, class name and all, so it is reproduced
    /// exactly rather than paraphrased -- it is what a developer sees when they forget an
    /// `include`.
    #[inline]
    pub fn build(&self) -> AABB {
        if !self.defined {
            panic!(
                "java.lang.IllegalStateException: Cannot build an undefined AABB. Include at \
                 least one point."
            );
        }
        AABB::new(
            self.min_x as f64,
            self.min_y as f64,
            self.min_z as f64,
            self.max_x as f64,
            self.max_y as f64,
            self.max_z as f64,
        )
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================
//
// Seven methods, and the four golden groups they hold are `aabb.fromBlockPos`,
// `aabb.encapsulatingFullBlocks`, `aabb.moveBlockPos` and `aabb.intersectsBlockPos`.
//
// All seven are `todo!` with a named dependency rather than a stub, because every one of them
// either takes a `BlockPos` or returns a result derived from one, and a placeholder here would
// return plausible geometry that does not match the world -- the failure mode that is hardest to
// trace back to its cause.

/// Java: `new AABB(BlockPos pos)` -- the unit cube at that block.
///
/// `todo!("PORT-BLOCKED: BlockPos")`. Unblocked by porting `net/minecraft/core/BlockPos.rs`,
/// which is currently a 7-line skeleton. Note the constructor widens: `pos.getX() + 1` is `int`
/// arithmetic that becomes `double`, so it is exact for every legal block coordinate.
pub fn from_block_pos(_pos: i32, _y: i32, _z: i32) -> AABB {
    todo!("PORT-BLOCKED: BlockPos is not ported")
}

/// Java: `AABB#move(BlockPos pos)`.
///
/// `todo!("PORT-BLOCKED: BlockPos")`. The `int` -> `double` widening on the deltas is what
/// keeps this exact at the build limit: `30_000_001.0` is exactly representable, whereas
/// `f32` would not be.
///
/// A free function rather than a method, because an associated function cannot hold a `todo!` and
/// still read as an instance method -- this is called as `AABB::move_block_pos(&box, x, y, z)`.
pub fn move_block_pos(_self: &AABB, _pos: i32, _y: i32, _z: i32) -> AABB {
    todo!("PORT-BLOCKED: BlockPos is not ported")
}

/// Java: `AABB#intersects(BlockPos pos)`.
///
/// `todo!("PORT-BLOCKED: BlockPos")`. Free function for the reason on [`move_block_pos`].
pub fn intersects_block_pos(_self: &AABB, _pos: i32, _y: i32, _z: i32) -> bool {
    todo!("PORT-BLOCKED: BlockPos is not ported")
}

/// Java: `AABB.encapsulatingFullBlocks(BlockPos,BlockPos)`.
///
/// `todo!("PORT-BLOCKED: BlockPos")`. The `+ 1` on the maxima is what makes the result cover
/// whole blocks rather than their corners, and it is applied to the `Math.max` of the two
/// positions -- so the `+1` happens after the maximum, not before.
pub fn encapsulating_full_blocks(
    _p0: (i32, i32, i32),
    _p1: (i32, i32, i32),
) -> AABB {
    todo!("PORT-BLOCKED: BlockPos is not ported")
}

/// Java: `AABB.of(BoundingBox box)`.
///
/// `todo!("PORT-BLOCKED: levelgen.structure.BoundingBox")`. Not a `BlockPos` dependency: it
/// needs the structure-generation bounding box, which carries its own inclusive/exclusive
/// convention (`maxX() + 1`).
pub fn of_bounding_box(
    _min_x: i32,
    _min_y: i32,
    _min_z: i32,
    _max_x: i32,
    _max_y: i32,
    _max_z: i32,
) -> AABB {
    todo!("PORT-BLOCKED: levelgen.structure.BoundingBox is not ported")
}

/// Java: the static `AABB#clip(Iterable<AABB>,Vec3,Vec3,BlockPos)`.
///
/// `todo!("PORT-BLOCKED: BlockHitResult and BlockPos")`. The interesting part -- reusing one
/// `scale_reference` across every box so the nearest hit across the whole list wins -- is
/// expressible without either type, but the method's *result* is a `BlockHitResult` carrying the
/// winning `Direction` and the `BlockPos`, so there is nothing honest to return yet.
pub fn clip_aabbs(_aabbs: &[AABB], _from: Vec3, _to: Vec3, _pos: i32) -> i32 {
    todo!("PORT-BLOCKED: BlockHitResult and BlockPos are not ported")
}
