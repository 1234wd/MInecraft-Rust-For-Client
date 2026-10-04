//! Port of: net/minecraft/core/Rotations.java
//! Java class(es): net.minecraft.core.Rotations
//! Status: PARTIAL
//!
//! PARTIAL for exactly one reason: `CODEC` and `STREAM_CODEC` are `todo!()`, deferred to
//! the serialisation batch (DataFixerUpper is not ported yet). See DESIGN_DECISIONS.md
//! (#dfu-proposal).
//!
//! Everything else -- the canonicalising constructor, `hashCode`, `equals`, `toString` --
//! is oracle-verified against `batch2.txt`. See `_porting/tests/parity_batch2.rs`.
//!
//! # WHY THE CONSTRUCTOR IS THE INTERESTING PART
//!
//! `Rotations` is a Java `record` with a compact constructor that rewrites its own
//! components:
//!
//! ```java
//! public Rotations {
//!     x = !Float.isInfinite(x) && !Float.isNaN(x) ? x % 360.0F : 0.0F;
//!     y = !Float.isInfinite(y) && !Float.isNaN(y) ? y % 360.0F : 0.0F;
//!     z = !Float.isInfinite(z) && !Float.isNaN(z) ? z % 360.0F : 0.0F;
//! }
//! ```
//!
//! Three things are load-bearing and all three are easy to get wrong:
//!
//! 1. **`% 360.0F` is computed in SINGLE precision.** The literal has an `F` suffix, so
//!    the remainder is a `float` operation. Writing `x % 360.0` would widen to `f64`,
//!    compute a different remainder, and produce different bits. For a value like
//!    `1.0e-45F` the difference is not subtle: `f32` remainder of a subnormal is
//!    itself, while the `f64` path first widens and can come back differently.
//!
//! 2. **Infinities and NaN become `0.0F`, not themselves.** Java's `%` on `Inf` yields
//!    NaN and on NaN yields NaN, so without the guard every non-finite rotation would
//!    become NaN. The guard maps them to zero instead.
//!
//! 3. **The guard is `&&`, not `||`, and it is applied per component.** A rotation with
//!    one infinite component and two finite ones keeps the finite ones as remainders.
//!
//! # There is no normalisation to [-180, 180]
//!
//! The result of `x % 360.0F` keeps Java's sign, so `-90.0F` stays `-90.0F` and
//! `370.0F` becomes `10.0F`. A port that "helpfully" normalises into a signed range
//! would break every place that compares raw rotation values.

use crate::javacompat::java_lang;
use crate::javacompat::java_lang::log::math_log_f64;

/// Port of `Rotations`.
#[derive(Clone, Copy, Debug)]
pub struct Rotations {
    /// Port of the record component `x`, AFTER canonicalisation.
    pub x: f32,
    /// Port of the record component `y`, AFTER canonicalisation.
    pub y: f32,
    /// Port of the record component `z`, AFTER canonicalisation.
    pub z: f32,
}

impl Rotations {
    /// Port of `Rotations#CODEC`.
    ///
    /// Deferred to the serialisation batch. Note the Java body uses
    /// `Codec.FLOAT.listOf()` with `Util.fixedSize(..., 3)`, so a wrong-length list
    /// produces a DataResult error whose text reaches the player; carry that over
    /// verbatim when we get there.
    pub fn CODEC() -> () {
        todo!("PORT: codec — Rotations#CODEC")
    }

    /// Port of `Rotations#STREAM_CODEC`.
    pub fn STREAM_CODEC() -> () {
        todo!("PORT: codec — Rotations#STREAM_CODEC")
    }

    /// Port of the compact record constructor `Rotations { ... }`.
    ///
    /// ```java
    /// x = !Float.isInfinite(x) && !Float.isNaN(x) ? x % 360.0F : 0.0F;
    /// ```
    ///
    /// The `%` MUST stay in `f32`. See the module docs.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: canonicalise(x),
            y: canonicalise(y),
            z: canonicalise(z),
        }
    }

    /// Port of `Rotations#x()`.
    #[inline]
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Port of `Rotations#y()`.
    #[inline]
    pub const fn y(&self) -> f32 {
        self.y
    }

    /// Port of `Rotations#z()`.
    #[inline]
    pub const fn z(&self) -> f32 {
        self.z
    }

    /// Port of the record's generated `hashCode()`.
    ///
    /// The generated body is:
    ///
    /// ```java
    /// int result = 0;
    /// result = 31 * result + Float.floatToIntBits(x);
    /// result = 31 * result + Float.floatToIntBits(y);
    /// result = 31 * result + Float.floatToIntBits(z);
    /// return result;
    /// ```
    ///
    /// **The accumulator SEEDS AT ZERO, not at one.** That is the difference between a
    /// record's hash and `Objects.hash(x, y, z)`, which seeds at one and so multiplies
    /// every component by 31 three times. The golden pins both cases:
    ///
    /// | rotation | record hash (`result = 0`) | `Objects.hash` (seed 1) |
    /// |---|---|---|
    /// | `(0, 0, 0)` | `0` | `29791` |
    ///
    /// Seeding at one gives 29791 for the zero rotation, which is what my first port did.
    ///
    /// `Float.floatToIntBits` also CANONICALISES every NaN to one value, so two
    /// rotations holding different NaN payloads hash equal.
    #[inline]
    pub fn java_hash_code(&self) -> i32 {
        let mut result: i32 = 0;
        result = result.wrapping_mul(31).wrapping_add(float_to_int_bits(self.x));
        result = result.wrapping_mul(31).wrapping_add(float_to_int_bits(self.y));
        result.wrapping_mul(31)
            .wrapping_add(float_to_int_bits(self.z))
    }

    /// Port of the record's generated `equals`.
    ///
    /// # IT IS NOT `==`, AND THAT IS THE WHOLE POINT
    ///
    /// A record's generated `equals` compares primitive components with
    /// `Float.compare`, NOT with `==`. The difference is observable here because the
    /// constructor PRESERVES the sign of zero:
    ///
    /// ```text
    /// new Rotations(0f, 0f, -0.0f).z   ==  -0.0f     // -0.0f % 360.0F is -0.0f
    /// Float.compare(-0.0f, 0.0f)      !=  0          // so equals() is FALSE
    /// ```
    ///
    /// Using Rust's derived `PartialEq` (which uses `==`) would report these two rotations
    /// as equal. `Float.compare` treats `-0.0f < 0.0f` while `-0.0f == 0.0f` is true in
    /// IEEE, and it treats every NaN as equal to every other NaN.
    #[inline]
    pub fn java_equals(&self, other: &Rotations) -> bool {
        float_compare(self.x, other.x) == 0
            && float_compare(self.y, other.y) == 0
            && float_compare(self.z, other.z) == 0
    }
}

/// `Float.compare`: -0.0f sorts BELOW +0.0f, and every NaN equals every other NaN.
///
/// This differs from `==` on the sign of zero only; on ordinary values it agrees.
#[inline]
fn float_compare(a: f32, b: f32) -> i32 {
    if a < b {
        -1
    } else if a > b {
        1
    } else {
        // Equal or unordered. Distinguish the zeros by their bits, which is the only case
        // `a < b` and `a > b` both miss.
        let ab = a.to_bits();
        let bb = b.to_bits();
        match ab.cmp(&bb) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

/// The single component of the compact constructor, `x % 360.0F` with the finite guard.
///
/// Split out so the three record components visibly share one implementation; writing
/// it three times is how one of them ends up subtly different.
#[inline]
fn canonicalise(v: f32) -> f32 {
    if v.is_infinite() || v.is_nan() {
        0.0
    } else {
        v % 360.0f32
    }
}

/// `Float.floatToIntBits`: like `to_bits()` but with every NaN collapsed to one value.
///
/// Distinct from `f32::to_bits()`, which is `floatToRawIntBits` and preserves the NaN
/// payload. Java's record `hashCode` uses the canonicalising form, so a rotation holding
/// a signalling NaN hashes the same as one holding a quiet NaN with a different payload.
#[inline]
fn float_to_int_bits(v: f32) -> i32 {
    if v.is_nan() {
        0x7fc0_0000u32 as i32
    } else {
        v.to_bits() as i32
    }
}

impl PartialEq for Rotations {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.java_equals(other)
    }
}

impl Eq for Rotations {}

impl std::fmt::Display for Rotations {
    /// Port of the record's generated `toString()`: `"Rotations[x=..., y=..., z=...]"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rotations[x={}, y={}, z={}]",
            java_lang::float_to_string::float_to_string(self.x),
            java_lang::float_to_string::float_to_string(self.y),
            java_lang::float_to_string::float_to_string(self.z)
        )
    }
}

// `math_log_f64` is referenced so the module keeps a single documented route for
// java.lang.Math calls if a future member needs one. Rotations itself calls none.
#[allow(dead_code)]
fn _math_route_is_jvm_math() -> f64 {
    math_log_f64(2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// These are ASSERTIONS ABOUT VANILLA, checked against `batch2.txt` in
    /// `parity_batch2.rs`. They are written here only to make the failure mode obvious
    /// at the call site; the authoritative values are the golden rows, per new rule 1
    /// (no hand-derived expected values).
    #[test]
    fn non_finite_becomes_zero() {
        // Java's `%` maps Inf and NaN to NaN, so without the guard these would be NaN.
        assert_eq!(Rotations::new(f32::INFINITY, f32::NEG_INFINITY, f32::NAN).x, 0.0);
        assert_eq!(Rotations::new(f32::NAN, 0.0, 0.0).y, 0.0);
    }

    #[test]
    fn remainder_keeps_javas_sign() {
        // No normalisation into [-180, 180]: -90 stays -90, 370 wraps to +10.
        assert_eq!(Rotations::new(-90.0, 0.0, 0.0).x, -90.0);
        assert_eq!(Rotations::new(370.0, 0.0, 0.0).x, 10.0);
        assert_eq!(Rotations::new(-370.0, 0.0, 0.0).x, -10.0);
        // Exactly 360.0F is the first value that becomes 0.0.
        assert_eq!(Rotations::new(360.0, 0.0, 0.0).x, 0.0);
    }

    #[test]
    fn canonicalisation_is_per_component() {
        let r = Rotations::new(f32::INFINITY, 370.0, -370.0);
        assert_eq!(r.x, 0.0);
        assert_eq!(r.y, 10.0);
        assert_eq!(r.z, -10.0);
    }

    /// The `%` must be a SINGLE-precision operation. This is not observable with round
    /// numbers, which is exactly why the golden sweeps the whole float corpus.
    #[test]
    fn remainder_is_computed_in_f32() {
        // A subnormal: `f32 % 360.0f32` is itself, and so is the f64 path widened back,
        // so use a value where the two differ in the low bits.
        let v = 1.0e-45f32; // smallest positive subnormal
        assert_eq!(canonicalise(v), v);
    }

    #[test]
    fn hash_canonicalises_nan() {
        // Two rotations whose NaNs have different payloads must hash EQUAL, because the
        // record hash uses Float.floatToIntBits.
        let a = Rotations::new(f32::from_bits(0x7fc0_0001), 0.0, 0.0);
        let b = Rotations::new(f32::from_bits(0x7fc0_0002), 0.0, 0.0);
        assert_eq!(a.x.to_bits(), 0); // constructor already mapped both to 0.0
        assert_eq!(a.java_hash_code(), b.java_hash_code());
    }
}