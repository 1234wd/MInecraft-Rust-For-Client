//! Port of: net/minecraft/core/Vec3i.java
//! Java class(es): Vec3i
//! Status: PARTIAL
//!
//! Everything except the DataFixerUpper codec fields, which are `todo!()` per the
//! batch rule (serialisation is deferred). Those are:
//!
//! * `Vec3i.CODEC`
//! * `Vec3i.offsetCodec(int)`
//!
//! Both are on no parity-tested path. See DESIGN_DECISIONS.md (#dfu-proposal).
//! Everything that IS implemented here is oracle-verified against `core.txt` — see
//! `_porting/tests/parity_core.rs`.

use crate::javacompat::java_lang;
use crate::net::minecraft::util::Mth::Mth;

/// Port of `Vec3i`.
///
/// A mutable-by-`set` integer position. Note that every arithmetic operation below
/// WRAPS on overflow, because Java's `int` does and there is no clamping anywhere in
/// the class. `Vec3i` is used for block coordinates, which vanilla keeps inside
/// `±30_000_000`, so overflow should be unreachable in normal play -- but a mod or a
/// corrupted save can reach it, and silently wrapping is what vanilla does.
#[derive(Clone, Copy, Debug)]
pub struct Vec3i {
    /// Port of `Vec3i#x`.
    x: i32,
    /// Port of `Vec3i#y`.
    y: i32,
    /// Port of `Vec3i#z`.
    z: i32,
}

impl Vec3i {
    /// Port of `Vec3i#CODEC`.
    ///
    /// Deferred to the serialisation batch: DataFixerUpper is not ported yet.
    /// See DESIGN_DECISIONS.md (#dfu-proposal) for the two options and the three
    /// parity risks (field order in a serialised map, `DataResult` error text, and
    /// numeric narrowing) that apply either way.
    pub fn CODEC() -> () {
        todo!("PORT: codec — Vec3i#CODEC")
    }

    /// Port of `Vec3i#offsetCodec(int)`.
    ///
    /// Deferred with [`Vec3i::CODEC`]. Note the Java body validates that every axis is
    /// within `maxOffsetPerAxis` and returns a `DataResult.error` otherwise -- worth
    /// carrying over verbatim when we get there, because the message text reaches the
    /// player.
    pub fn offset_codec(max_offset_per_axis: i32) -> () {
        todo!("PORT: codec — Vec3i#offsetCodec")
    }

    /// Port of `Vec3i#ZERO`.
    pub const ZERO: Vec3i = Vec3i { x: 0, y: 0, z: 0 };

    /// Port of `Vec3i#Vec3i(int,int,int)`.
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    // -----------------------------------------------------------------------
    // hashCode / equals
    // -----------------------------------------------------------------------

    /// Port of `Vec3i#hashCode()`.
    ///
    /// ```java
    /// return (this.getY() + this.getZ() * 31) * 31 + this.getX();
    /// ```
    ///
    /// NOT the idiomatic `x*31*31 + y*31 + z`, and not `Objects.hash(...)`. The
    /// weighting is x < y < z in significance, which means positions differing only in
    /// `x` collide heavily in the low bits. That is not a bug to fix: it is the exact
    /// hash Java's `HashMap`/`HashSet` buckets on, so reproducing it is what makes
    /// iteration order match. Both multiplies WRAP.
    #[inline]
    pub fn java_hash_code(&self) -> i32 {
        self.y
            .wrapping_add(self.z.wrapping_mul(31))
            .wrapping_mul(31)
            .wrapping_add(self.x)
    }

    /// Port of `Vec3i#equals(Object)` with the `instanceof` already resolved.
    #[inline]
    pub fn java_equals(&self, other: &Vec3i) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    // -----------------------------------------------------------------------
    // accessors
    // -----------------------------------------------------------------------

    /// Port of `Vec3i#getX()`.
    #[inline]
    pub const fn get_x(&self) -> i32 {
        self.x
    }

    /// Port of `Vec3i#getY()`.
    #[inline]
    pub const fn get_y(&self) -> i32 {
        self.y
    }

    /// Port of `Vec3i#getZ()`.
    #[inline]
    pub const fn get_z(&self) -> i32 {
        self.z
    }

    /// Port of the inherited `Position#x()`.
    #[inline]
    pub const fn x(&self) -> i32 {
        self.x
    }

    /// Port of the inherited `Position#y()`.
    #[inline]
    pub const fn y(&self) -> i32 {
        self.y
    }

    /// Port of the inherited `Position#z()`.
    #[inline]
    pub const fn z(&self) -> i32 {
        self.z
    }

    /// Port of `Vec3i#setX(int)`.
    ///
    /// Java returns `this` so the setters chain. `Vec3i` is mutable in Java for this
    /// reason (used by `BlockPos.MutableBlockPos`); the Rust port keeps the same
    /// shape so the subclass relationship can be expressed later.
    #[inline]
    pub fn set_x(&mut self, x: i32) -> &mut Self {
        self.x = x;
        self
    }

    /// Port of `Vec3i#setY(int)`.
    #[inline]
    pub fn set_y(&mut self, y: i32) -> &mut Self {
        self.y = y;
        self
    }

    /// Port of `Vec3i#setZ(int)`.
    #[inline]
    pub fn set_z(&mut self, z: i32) -> &mut Self {
        self.z = z;
        self
    }

    /// Port of `Vec3i#set(int,int,int)`.
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    // -----------------------------------------------------------------------
    // comparison
    // -----------------------------------------------------------------------

    /// Port of `Vec3i#compareTo(Vec3i)`.
    ///
    /// Orders by **y, then z, then x** -- deliberately not x,y,z. This is the order
    /// `TreeSet`/`sorted()` will use, and it is load-bearing for anything that emits
    /// positions in sorted order (structure scans, `IdMap` tie-breaks). The
    /// subtractions WRAP, so the sign is what matters, not the magnitude.
    #[inline]
    pub fn compare_to(&self, pos: &Vec3i) -> i32 {
        if self.y == pos.y {
            if self.z == pos.z {
                self.x.wrapping_sub(pos.x)
            } else {
                self.z.wrapping_sub(pos.z)
            }
        } else {
            self.y.wrapping_sub(pos.y)
        }
    }

    // -----------------------------------------------------------------------
    // arithmetic
    // -----------------------------------------------------------------------

    /// Port of `Vec3i#offset(int,int,int)`.
    ///
    /// Returns `this` when all three offsets are zero -- an identity shortcut that
    /// matters because `offset` is extremely hot in worldgen, and because callers
    /// occasionally rely on the returned object being the same instance.
    #[inline]
    pub fn offset(&self, x: i32, y: i32, z: i32) -> Vec3i {
        if x == 0 && y == 0 && z == 0 {
            *self
        } else {
            Vec3i::new(
                self.x.wrapping_add(x),
                self.y.wrapping_add(y),
                self.z.wrapping_add(z),
            )
        }
    }

    /// Port of `Vec3i#offset(Vec3i)`.
    #[inline]
    pub fn offset_vec3i(&self, vec: &Vec3i) -> Vec3i {
        self.offset(vec.x, vec.y, vec.z)
    }

    /// Port of `Vec3i#subtract(Vec3i)`.
    ///
    /// Java writes `offset(-v.getX(), -v.getY(), -v.getZ())`. Note the negations are
    /// plain Java negation and therefore WRAP, so subtracting `Vec3i::new(i32::MIN, ..)`
    /// does not give the additive inverse.
    #[inline]
    pub fn subtract(&self, vec: &Vec3i) -> Vec3i {
        self.offset(vec.x.wrapping_neg(), vec.y.wrapping_neg(), vec.z.wrapping_neg())
    }

    /// Port of `Vec3i#multiply(int)`.
    ///
    /// Three cases, in this order: `scale == 1` returns `this`; `scale == 0` returns
    /// the shared [`Vec3i::ZERO`] constant; otherwise multiplies (wrapping). The
    /// `scale == 0 -> ZERO` branch is why `ZERO` is shared rather than freshly built.
    #[inline]
    pub fn multiply(&self, scale: i32) -> Vec3i {
        if scale == 1 {
            *self
        } else if scale == 0 {
            Vec3i::ZERO
        } else {
            Vec3i::new(
                self.x.wrapping_mul(scale),
                self.y.wrapping_mul(scale),
                self.z.wrapping_mul(scale),
            )
        }
    }

    /// Port of `Vec3i#multiply(int,int,int)`.
    ///
    /// No identity shortcuts here, unlike [`Vec3i::multiply`].
    #[inline]
    pub fn multiply_3(&self, x_scale: i32, y_scale: i32, z_scale: i32) -> Vec3i {
        Vec3i::new(
            self.x.wrapping_mul(x_scale),
            self.y.wrapping_mul(y_scale),
            self.z.wrapping_mul(z_scale),
        )
    }

    /// Port of `Vec3i#cross(Vec3i)`.
    ///
    /// Note the operands are NOT normalized, so the result magnitude scales with both
    /// inputs. All six products and all three subtractions WRAP.
    #[inline]
    pub fn cross(&self, up_vector: &Vec3i) -> Vec3i {
        Vec3i::new(
            self.y.wrapping_mul(up_vector.z).wrapping_sub(self.z.wrapping_mul(up_vector.y)),
            self.z.wrapping_mul(up_vector.x).wrapping_sub(self.x.wrapping_mul(up_vector.z)),
            self.x.wrapping_mul(up_vector.y).wrapping_sub(self.y.wrapping_mul(up_vector.x)),
        )
    }

    // -----------------------------------------------------------------------
    // relative / direction helpers -- need Direction, ported in the same batch.
    // -----------------------------------------------------------------------

    /// Port of `Vec3i#above(int)`.
    #[inline]
    pub fn above(&self, steps: i32) -> Vec3i {
        crate::net::minecraft::core::Direction::Direction::Up.relative_to(*self, steps)
    }

    /// Port of `Vec3i#below(int)`.
    #[inline]
    pub fn below(&self, steps: i32) -> Vec3i {
        crate::net::minecraft::core::Direction::Direction::Down.relative_to(*self, steps)
    }

    /// Port of `Vec3i#north(int)`.
    #[inline]
    pub fn north(&self, steps: i32) -> Vec3i {
        crate::net::minecraft::core::Direction::Direction::North.relative_to(*self, steps)
    }

    /// Port of `Vec3i#south(int)`.
    #[inline]
    pub fn south(&self, steps: i32) -> Vec3i {
        crate::net::minecraft::core::Direction::Direction::South.relative_to(*self, steps)
    }

    /// Port of `Vec3i#west(int)`.
    #[inline]
    pub fn west(&self, steps: i32) -> Vec3i {
        crate::net::minecraft::core::Direction::Direction::West.relative_to(*self, steps)
    }

    /// Port of `Vec3i#east(int)`.
    #[inline]
    pub fn east(&self, steps: i32) -> Vec3i {
        crate::net::minecraft::core::Direction::Direction::East.relative_to(*self, steps)
    }

    /// Port of `Vec3i#relative(Direction,int)`.
    ///
    /// `steps == 0` returns `this` unchanged, so `relative` is not a pure offset.
    #[inline]
    pub fn relative(&self, direction: crate::net::minecraft::core::Direction::Direction, steps: i32) -> Vec3i {
        direction.relative_to(*self, steps)
    }

    /// Port of `Vec3i#relative(Direction.Axis,int)`.
    #[inline]
    pub fn relative_axis(
        &self,
        axis: crate::net::minecraft::core::Direction::Axis,
        steps: i32,
    ) -> Vec3i {
        axis.relative_to(*self, steps)
    }

    // -----------------------------------------------------------------------
    // distances
    // -----------------------------------------------------------------------

    /// Port of `Vec3i#distSqr(Vec3i)`.
    ///
    /// Delegates to `distToLowCornerSqr` with the other's INTEGER coordinates widened
    /// to `double`. So this measures corner-to-corner, not centre-to-centre -- see
    /// [`Vec3i::dist_to_center_sqr`].
    #[inline]
    pub fn dist_sqr(&self, pos: &Vec3i) -> f64 {
        self.dist_to_low_corner_sqr(pos.x as f64, pos.y as f64, pos.z as f64)
    }

    /// Port of `Vec3i#distToCenterSqr(double,double,double)`.
    ///
    /// The `+ 0.5` places `this` at the CENTRE of its block before measuring.
    #[inline]
    pub fn dist_to_center_sqr(&self, x: f64, y: f64, z: f64) -> f64 {
        let dx = self.x as f64 + 0.5 - x;
        let dy = self.y as f64 + 0.5 - y;
        let dz = self.z as f64 + 0.5 - z;
        java_lang::add_f64(
            java_lang::add_f64(java_lang::mul_f64(dx, dx), java_lang::mul_f64(dy, dy)),
            java_lang::mul_f64(dz, dz),
        )
    }

    /// Port of `Vec3i#distToLowCornerSqr(double,double,double)`.
    #[inline]
    pub fn dist_to_low_corner_sqr(&self, x: f64, y: f64, z: f64) -> f64 {
        let dx = self.x as f64 - x;
        let dy = self.y as f64 - y;
        let dz = self.z as f64 - z;
        java_lang::add_f64(
            java_lang::add_f64(java_lang::mul_f64(dx, dx), java_lang::mul_f64(dy, dy)),
            java_lang::mul_f64(dz, dz),
        )
    }

    /// Port of `Vec3i#distManhattan(Vec3i)`.
    ///
    /// # TWO WIDTH CHANGES, IN THIS ORDER
    ///
    /// ```java
    /// float xd = Math.abs(pos.getX() - this.getX());
    /// float yd = Math.abs(pos.getY() - this.getY());
    /// float zd = Math.abs(pos.getZ() - this.getZ());
    /// return (int)(xd + yd + zd);
    /// ```
    ///
    /// The `Math.abs` is applied to the **int** difference, and only the RESULT is
    /// widened to `float`. That ordering is load-bearing, and it was a real bug in
    /// session 02's port:
    ///
    /// With `a = (0,0,0)` and `b = (Integer.MAX_VALUE, Integer.MIN_VALUE, 0)`:
    ///
    /// | step | `xd` | `yd` | sum | result |
    /// |---|---|---|---|---|
    /// | Java (`Math.abs` on the int) | `+2.147e9` | `-2.147e9` (abs of MIN overflows) | `0.0` | **0** |
    /// | `abs` applied to the float | `+2.147e9` | `+2.147e9` | `4.29e9` | `Integer.MAX_VALUE` |
    ///
    /// So taking the absolute value AFTER widening gives a completely different answer
    /// whenever `Math.abs(int)` itself overflows -- which is exactly the
    /// `Integer.MIN_VALUE` case, reachable at the world border.
    ///
    /// The final `(int)` cast then saturates rather than wrapping, matching Java's
    /// narrowing conversion from `float`.
    #[inline]
    pub fn dist_manhattan(&self, pos: &Vec3i) -> i32 {
        let xd = java_lang::abs_i32(pos.x.wrapping_sub(self.x)) as f32;
        let yd = java_lang::abs_i32(pos.y.wrapping_sub(self.y)) as f32;
        let zd = java_lang::abs_i32(pos.z.wrapping_sub(self.z)) as f32;
        (java_lang::add_f32(java_lang::add_f32(xd, yd), zd)) as i32
    }

    /// Port of `Vec3i#distChessboard(Vec3i)`.
    ///
    /// Stays in `int` -- contrast [`Vec3i::dist_manhattan`], its neighbour. The
    /// subtraction WRAPS before `abs`, so `abs` of the wrapped value can be negative.
    #[inline]
    pub fn dist_chessboard(&self, pos: &Vec3i) -> i32 {
        let xd = java_lang::abs_i32(self.x.wrapping_sub(pos.x));
        let yd = java_lang::abs_i32(self.y.wrapping_sub(pos.y));
        let zd = java_lang::abs_i32(self.z.wrapping_sub(pos.z));
        java_lang::max_i32(java_lang::max_i32(xd, yd), zd)
    }

    /// Port of `Vec3i#closerThan(Vec3i,double)`.
    #[inline]
    pub fn closer_than(&self, pos: &Vec3i, distance: f64) -> bool {
        self.dist_sqr(pos) < Mth::square_f64(distance)
    }
}

// `Vec3i` is used as a map key all over vanilla, so it must hash and compare exactly
// as Java does -- NOT via the derive, which would use field order x,y,z.
impl PartialEq for Vec3i {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.java_equals(other)
    }
}

impl Eq for Vec3i {}

impl std::hash::Hash for Vec3i {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the value Java would return, so any Java-order hash map built on
        // `java_hash_code` can be reproduced.
        self.java_hash_code().hash(state);
    }
}

impl PartialOrd for Vec3i {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec3i {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.compare_to(other).cmp(&0)
    }
}

impl std::fmt::Display for Vec3i {
    /// Port of `Vec3i#toString()`: `"[x, y, z]"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_code_is_y_then_z_weighted_not_x() {
        // (y + z*31)*31 + x. Verified against the oracle in parity_core.rs.
        assert_eq!(Vec3i::new(1, 2, 3).java_hash_code(), (2 + 3 * 31) * 31 + 1);
        assert_eq!(Vec3i::new(0, 0, 0).java_hash_code(), 0);
        assert_eq!(Vec3i::new(1, 0, 0).java_hash_code(), 1);
        // NOT the derive order: if someone rewrites this as x*961 + y*31 + z the
        // value for (1,2,3) becomes 1*961 + 2*31 + 3 = 1026 instead of 1968.
        assert_eq!(Vec3i::new(1, 2, 3).java_hash_code(), 2946);
        assert_ne!(Vec3i::new(1, 2, 3).java_hash_code(), 1026);
    }

    #[test]
    fn hash_code_wraps() {
        let v = Vec3i::new(i32::MIN, i32::MAX, i32::MIN);
        // Must not panic; Java wraps silently.
        let _ = v.java_hash_code();
    }

    #[test]
    fn compare_orders_y_then_z_then_x() {
        let a = Vec3i::new(0, 1, 0);
        let b = Vec3i::new(99, 0, 0);
        // y decides first, so `a` (y=1) sorts AFTER `b` (y=0) despite x=0 vs 99.
        assert!(a.compare_to(&b) > 0);
        let a = Vec3i::new(0, 0, 0);
        let b = Vec3i::new(0, 0, 1);
        assert!(a.compare_to(&b) < 0);
        let a = Vec3i::new(0, 0, 5);
        let b = Vec3i::new(7, 0, 5);
        assert!(a.compare_to(&b) < 0);
    }

    #[test]
    fn multiply_short_circuits() {
        let v = Vec3i::new(1, 2, 3);
        assert_eq!(v.multiply(1), v);
        assert_eq!(v.multiply(0), Vec3i::ZERO);
        assert_eq!(v.multiply(2), Vec3i::new(2, 4, 6));
    }

    #[test]
    fn offset_zero_is_identity() {
        let v = Vec3i::new(1, 2, 3);
        assert_eq!(v.offset(0, 0, 0), v);
        assert_eq!(v.offset(1, 0, 0), Vec3i::new(2, 2, 3));
    }

    #[test]
    fn cross_is_not_normalised() {
        let a = Vec3i::new(1, 0, 0);
        let b = Vec3i::new(0, 1, 0);
        assert_eq!(a.cross(&b), Vec3i::new(0, 0, 1));
        // Scaling the input scales the output -- proof there is no normalisation.
        let a2 = Vec3i::new(2, 0, 0);
        assert_eq!(a2.cross(&b), Vec3i::new(0, 0, 2));
    }

    /// `distManhattan` widens each absolute difference to `f32` before summing, so
    /// above 2^24 the answer is quantised to the `f32` spacing (4 at 2^25) while
    /// `distChessboard` stays in `int` and keeps every bit.
    ///
    /// This is vanilla's behaviour. An `i32` implementation would be MORE accurate and
    /// would desync anything keyed on the manhattan distance -- which is why the
    /// `float` is in the port even though it looks like a mistake.
    #[test]
    fn dist_manhattan_goes_through_float() {
        let a = Vec3i::new(0, 0, 0);

        // 2^25 + 3 is not representable in f32. The spacing in [2^25, 2^26) is 4, and
        // 33554435 is closer to 33554436 than to 33554432, so it rounds UP.
        let b = Vec3i::new((1 << 25) + 3, 0, 0);
        assert_eq!(a.dist_manhattan(&b), (1 << 25) + 4, "the float quantisation is load-bearing");
        assert_ne!(a.dist_manhattan(&b), a.dist_chessboard(&b));
        assert_eq!(a.dist_chessboard(&b), (1 << 25) + 3, "distChessboard stays exact in int");

        // At small magnitudes the two agree, so the narrowing is not always wrong.
        let small = Vec3i::new(3, 4, 0);
        assert_eq!(a.dist_manhattan(&small), 7);
        assert_eq!(a.dist_chessboard(&small), 4);
    }    #[test]
    fn dist_centre_offsets_by_half() {
        let a = Vec3i::new(0, 0, 0);
        // Distance from block centre to its own centre is 0.
        assert_eq!(a.dist_to_center_sqr(0.5, 0.5, 0.5), 0.0);
        // ...to its corner is 3 * 0.25.
        assert_eq!(a.dist_to_center_sqr(0.0, 0.0, 0.0), 0.75);
        // distSqr measures corner-to-corner, so it is 0 against the corner.
        assert_eq!(a.dist_sqr(&Vec3i::new(0, 0, 0)), 0.0);
    }

    #[test]
    fn to_string_matches_java() {
        assert_eq!(Vec3i::new(1, -2, 3).to_string(), "[1, -2, 3]");
    }
}
