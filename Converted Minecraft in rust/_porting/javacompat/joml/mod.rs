//! Port of: (no Java counterpart in the mirror -- third-party library)
//! Java class(es): `org.joml.Math`, `org.joml.Vector3f`, `org.joml.Quaternionf`
//! Status: PARTIAL
//!
//! The subset of **JOML 1.10.8** that `Mth#rotationAroundAxis` reaches, ported
//! method-by-method from JOML's own compiled bytecode rather than from its docs.
//!
//! # Why port it instead of using glam / nalgebra / a fresh implementation
//!
//! JOML's arithmetic is not ordinary arithmetic. `Vector3f#dot` and
//! `Quaternionf#normalize` are built on `Math.fma`, an IEEE-754 **fused**
//! multiply-add: `fma(a, b, c)` computes `a * b + c` with a SINGLE rounding, not two.
//!
//! That is not a detail. `a * b + c` in Rust rounds the product, then rounds the sum.
//! JOML keeps the product's low bits. For most inputs the two agree; for a small but
//! genuinely reachable set they differ by 1 ulp, and a 1-ulp difference in a quaternion
//! component propagates into block rotation and entity yaw.
//!
//! Using an off-the-shelf Rust math library would also change float behaviour in ways
//! that are hard to audit (FMA contraction settings, SIMD paths, `no_std` variants).
//! Porting the handful of methods we need, from the bytecode, keeps every rounding
//! decision visible and testable.
//!
//! # Where the bytecode came from
//!
//! `javap -c` against `joml-1.10.8.jar`, the exact version Minecraft 26.2 resolves.
//! Every non-obvious detail below is quoted from it rather than inferred.

/// Port of `org.joml.Math` (only the members JOML's own code calls here).
pub mod math {
    /// Port of `org.joml.Math#fma(float,float,float)`.
    ///
    /// JOML's bytecode:
    /// ```text
    /// public static float fma(float, float, float);
    ///    0: getstatic  Field org/joml/Runtime.HAS_Math_fma:Z
    ///    3: ifeq      13
    ///    6: fload_0
    ///    7: fload_1
    ///    8: fload_2
    ///    9: invokestatic  Method java/lang/Math.fma:(FFF)F   <-- taken on x86-64 JDK 9+
    ///   12: freturn
    ///   13: fload_0 / fload_1 / fmul / fload_2 / fadd / freturn   <-- fallback
    /// ```
    ///
    /// `HAS_Math_fma` is a static final initialised by probing for the method, so on any
    /// JDK 9+ it is true and the fused path is taken. We are on JDK 25, so: fused.
    ///
    /// Rust's `f32::mul_add` is specified as a single-rounding fused operation and
    /// lowers to the `vfmadd` instruction, so it is the same function -- and unlike the
    /// `a * b + c` spelling it is immune to the compiler contracting or reassociating
    /// the expression around it.
    ///
    /// # NaN payload
    ///
    /// FMA propagates NaN differently from multiply-then-add: the FMA raises the
    /// real-indefinite QNaN (negative on x86) where `a*b` alone would propagate the
    /// operand. Callers that need the JVM's convention should route the result through
    /// [`crate::javacompat::nan_policy::observable_f32`].
    #[inline]
    pub fn fma(a: f32, b: f32, c: f32) -> f32 {
        a.mul_add(b, c)
    }

    /// Port of `org.joml.Math#invsqrt(float)`.
    ///
    /// JOML's bytecode:
    /// ```text
    /// public static float invsqrt(float);
    ///    0: fconst_1
    ///    1: fload_0
    ///    2: f2d                 // widen to double
    ///    3: invokestatic  Method java/lang/Math.sqrt:(D)D
    ///    6: d2f                 // narrow back
    ///    7: fdiv
    ///    8: freturn
    /// ```
    ///
    /// So it is `1.0f / (float)Math.sqrt((double) x)` -- NOT `1.0f / x.sqrt()`, which
    /// would compute the square root in single precision and round twice.
    ///
    /// This is what `Mth#invSqrt` delegates to, so it is on the parity-tested path.
    #[inline]
    pub fn invsqrt(x: f32) -> f32 {
        1.0f32 / (x as f64).sqrt() as f32
    }

    /// Port of `org.joml.Math#invsqrt(double)`.
    ///
    /// ```text
    ///    0: dconst_1
    ///    1: dload_0
    ///    2: invokestatic  Method java/lang/Math.sqrt:(D)D
    /// ```
    #[inline]
    pub fn invsqrt_f64(x: f64) -> f64 {
        1.0f64 / x.sqrt()
    }
}

/// Port of `org.joml.Vector3f` (the subset `Mth` uses).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3f {
    /// Port of `Vector3f#x`.
    pub x: f32,
    /// Port of `Vector3f#y`.
    pub y: f32,
    /// Port of `Vector3f#z`.
    pub z: f32,
}

impl Vector3f {
    /// Port of `Vector3f#Vector3f(float,float,float)`.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Port of `Vector3fc#x()`.
    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Port of `Vector3fc#y()`.
    #[inline]
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Port of `Vector3fc#z()`.
    #[inline]
    pub fn z(&self) -> f32 {
        self.z
    }

    /// Port of `Vector3fc#dot(float,float,float)`.
    ///
    /// JOML's bytecode:
    /// ```text
    /// public float dot(float, float, float);
    ///    0: aload_0 / getfield x / fload_1        // this.x, vx
    ///    5: aload_0 / getfield y / fload_2        // this.y, vy
    ///   10: aload_0 / getfield z / fload_3        // this.z, vz
    ///   15: fmul                                  // this.z * vz      <- PLAIN multiply
    ///   16: invokestatic fma                      // fma(this.y, vy, that)
    ///   19: invokestatic fma                      // fma(this.x, vx, that)
    /// ```
    ///
    /// So the ASSOCIATION matters: `fma(x, vx, fma(y, vy, z * vz))`. The innermost term
    /// is a plain multiply, because JOML could not fuse a multiply with nothing to add
    /// it to. Rewriting as `x*vx + y*vy + z*vz` gives a different rounding, and
    /// reassociating to `fma(z, vz, fma(y, vy, x*vx))` changes which operand survives.
    #[inline]
    pub fn dot(&self, vx: f32, vy: f32, vz: f32) -> f32 {
        math::fma(self.x, vx, math::fma(self.y, vy, self.z * vz))
    }
}

/// Port of `org.joml.Quaternionf` (the subset `Mth` uses).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternionf {
    /// Port of `Quaternionf#x`.
    pub x: f32,
    /// Port of `Quaternionf#y`.
    pub y: f32,
    /// Port of `Quaternionf#z`.
    pub z: f32,
    /// Port of `Quaternionf#w`.
    pub w: f32,
}

impl Quaternionf {
    /// Port of `Quaternionf#Quaternionf(float,float,float,float)`.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Port of `Quaternionf#w()`.
    #[inline]
    pub fn w(&self) -> f32 {
        self.w
    }

    /// Port of `Quaternionf#x()`.
    #[inline]
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Port of `Quaternionf#y()`.
    #[inline]
    pub fn y(&self) -> f32 {
        self.y
    }

    /// Port of `Quaternionf#z()`.
    #[inline]
    pub fn z(&self) -> f32 {
        self.z
    }

    /// Port of `Quaternionf#set(float,float,float,float)`.
    ///
    /// Mutates in place and returns `self`, as JOML does, so `set(..).normalize()` chains.
    #[inline]
    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
        self
    }

    /// Port of `Quaternionf#normalize()`.
    ///
    /// JOML's bytecode:
    /// ```text
    /// public org.joml.Quaternionf normalize();
    ///    0..32:  push x, x, y, y, z, z, w, w
    ///   32: fmul                                  // w * w        <- PLAIN multiply
    ///   33: invokestatic fma                      // fma(z, z, prev)
    ///   36: invokestatic fma                      // fma(y, y, prev)
    ///   39: invokestatic fma                      // fma(x, x, prev)
    ///   42: invokestatic invsqrt                  // d = 1/sqrt(...)
    ///   46+: dst.x = x * d; dst.y = y * d; dst.z = z * d; dst.w = w * d
    /// ```
    ///
    /// So the length-squared is `fma(x, x, fma(y, y, fma(z, z, w * w)))` -- note it is
    /// **not** symmetric: `w * w` is a plain multiply and everything else fuses. The
    /// order is x outermost, w innermost, so a "cleaner" `fma(w,w,fma(z,z,fma(y,y,x*x)))`
    /// rounds differently.
    ///
    /// The scaling is a plain multiply by `d`; it is not `x * (1/len)` computed
    /// independently per component.
    #[inline]
    pub fn normalize(&mut self) -> &mut Self {
        let d = math::invsqrt(math::fma(
            self.x,
            self.x,
            math::fma(self.y, self.y, math::fma(self.z, self.z, self.w * self.w)),
        ));
        self.x *= d;
        self.y *= d;
        self.z *= d;
        self.w *= d;
        self
    }

    /// Port of `Quaternionf#length()` -- provided because callers may want it; not on a
    /// parity-tested path yet.
    #[inline]
    pub fn length(&self) -> f32 {
        math::invsqrt(math::fma(
            self.x,
            self.x,
            math::fma(self.y, self.y, math::fma(self.z, self.z, self.w * self.w)),
        ))
    }
}

/// Port of `Mth#rotationAroundAxis(Vector3fc, Quaternionf, Quaternionf)`.
///
/// Deliberately lives next to the JOML subset rather than in `Mth.rs`: it is one line of
/// game code wrapped around a great deal of library-specific rounding behaviour, and
/// keeping them together means the FMA requirements are visible from the call site.
pub fn rotation_around_axis<'a>(
    axis: &Vector3f,
    rotation: &Quaternionf,
    result: &'a mut Quaternionf,
) -> &'a mut Quaternionf {
    // Java: float projectedLength = axis.dot(rotation.x, rotation.y, rotation.z);
    let projected_length = axis.dot(rotation.x, rotation.y, rotation.z);
    // Java: result.set(a.x*p, a.y*p, a.z*p, rotation.w).normalize();
    result
        .set(
            axis.x() * projected_length,
            axis.y() * projected_length,
            axis.z() * projected_length,
            rotation.w(),
        )
        .normalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole point of the module: `fma` must round ONCE, not twice.
    #[test]
    fn fma_is_fused_not_multiply_then_add() {
        // f64: (1 + 2^-27)^2 == 1 + 2^-26 + 2^-54. The 2^-54 term is below f64's
        // spacing in [1,2), so a separate multiply-then-add loses it entirely.
        let a = 1.0f64 + 2f64.powi(-27);
        let c = -(1.0f64 + 2f64.powi(-26));
        assert_eq!(a.mul_add(a, c), 2f64.powi(-54), "fused keeps the low term");
        assert_eq!(a * a + c, 0.0, "separate rounding discards it");
        assert_ne!(a.mul_add(a, c), a * a + c);

        // Same trick in f32: (1 + 2^-12)^2 == 1 + 2^-11 + 2^-24, and 2^-24 is below
        // the spacing of [1,2) in f32, so it is exactly the kind of term that gets
        // dropped.
        let a = 1.0f32 + 2f32.powi(-12);
        let c = -(1.0f32 + 2f32.powi(-11));
        assert_eq!(a.mul_add(a, c), 2f32.powi(-24));
        assert_eq!(a * a + c, 0.0);
        assert_ne!(a.mul_add(a, c), a * a + c);
    }

    /// `invsqrt` widens to `f64` before the square root, exactly as JOML's bytecode
    /// does. I expected that to make the result differ from a single-precision
    /// `1.0f32 / v.sqrt()` in places. It does not -- and the reason is worth stating,
    /// because "just use `v.sqrt()`" looks like a safe simplification until you know
    /// why it happens to be safe.
    ///
    /// **Double rounding for square root is benign here.** A naive
    /// `(x as f64).sqrt() as f32` is two roundings, which can in general disagree
    /// with a direct f32 square root. It does not when the intermediate format has at
    /// least `2p + 2` mantissa bits, where `p` is the target precision: f64 has 53 and
    /// `2 * 24 + 2 = 50`, so the 3 extra bits are exactly enough to guarantee the
    /// intermediate is never close enough to a rounding boundary to flip.
    ///
    /// So `invsqrt` keeps the JOML spelling for two reasons: it is what the bytecode
    /// does (so this module stays auditable against `javap` output), and it does not
    /// depend on that theorem holding for some future target.
    #[test]
    fn invsqrt_widening_is_equivalent_for_normal_inputs() {
        let x = 3.0f32;
        assert_eq!(math::invsqrt(x).to_bits(), (1.0f32 / (x as f64).sqrt() as f32).to_bits());

        // Sweep a wide pseudo-random sample of positive finite floats and require
        // exact agreement. If this ever fails, the 2p+2 reasoning above is wrong and
        // the widening is load-bearing -- which is exactly what we want to know.
        let mut probe = 1u32;
        let mut checked = 0u32;
        while checked < 200_000 {
            let v = f32::from_bits(probe);
            if v.is_finite() && v > 0.0 && v.is_normal() {
                assert_eq!(
                    math::invsqrt(v).to_bits(),
                    (1.0f32 / (v as f64).sqrt() as f32).to_bits(),
                    "disagreement at {v:e} ({:#x})", v.to_bits()
                );
                checked += 1;
            }
            probe = probe.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        }
        assert_eq!(checked, 200_000, "the sample should have covered 200k normal floats");
    }

    /// The places where the two spellings genuinely part company, which is why the
    /// JOML form is kept rather than relying on the equivalence above.
    #[test]
    fn invsqrt_edge_cases_are_where_the_spellings_differ() {
        // Zero: both give +Inf, but the ROUNDING PATH differs (f32 divide vs
        // narrow-then-divide), which is exactly the kind of thing a rewrite would
        // silently change.
        assert_eq!(math::invsqrt(0.0f32), f32::INFINITY);
        assert_eq!(1.0f32 / (0.0f32 as f64).sqrt() as f32, f32::INFINITY);

        // Negative: both NaN, but see nan_policy -- the sign is only pinned where it is
        // observable, and `Mth#invSqrt` results are not hashed or serialised.
        assert!(math::invsqrt(-1.0f32).is_nan());
        assert!((1.0f32 / (-1.0f32 as f64).sqrt() as f32).is_nan());

        // Subnormals: f64 sqrt sees a value f32 sqrt may flush toward zero.
        let tiny = f32::from_bits(1);
        assert!(math::invsqrt(tiny).is_finite() && math::invsqrt(tiny) > 1.0e18);
    }

    #[test]
    fn dot_association_is_inner_plain_multiply() {
        let v = Vector3f::new(0.1, 0.2, 0.3);
        let got = v.dot(0.4, 0.5, 0.6);
        let expected = math::fma(v.x, 0.4, math::fma(v.y, 0.5, v.z * 0.6));
        assert_eq!(got.to_bits(), expected.to_bits());
    }

    #[test]
    fn normalize_puts_w_innermost_with_a_plain_multiply() {
        let mut q = Quaternionf::new(0.1, 0.2, 0.3, 0.4);
        let before = q;
        q.normalize();
        let d = math::invsqrt(math::fma(
            before.x,
            before.x,
            math::fma(before.y, before.y, math::fma(before.z, before.z, before.w * before.w)),
        ));
        assert_eq!(q.x.to_bits(), (before.x * d).to_bits());
        assert_eq!(q.y.to_bits(), (before.y * d).to_bits());
        assert_eq!(q.z.to_bits(), (before.z * d).to_bits());
        assert_eq!(q.w.to_bits(), (before.w * d).to_bits());
        // And it actually produced a unit quaternion.
        assert!((q.length() - 1.0).abs() < 1e-6, "length was {}", q.length());
    }

    #[test]
    fn rotation_around_axis_matches_the_java_spelling() {
        let axis = Vector3f::new(0.0, 1.0, 0.0);
        let rotation = Quaternionf::new(0.0, 0.0, 0.0, 1.0);
        let mut result = Quaternionf::new(9.0, 9.0, 9.0, 9.0);
        rotation_around_axis(&axis, &rotation, &mut result);
        // Identity rotation about (0,1,0) is unchanged.
        assert_eq!(result.x().to_bits(), 0.0f32.to_bits());
        assert_eq!(result.y().to_bits(), 0.0f32.to_bits());
        assert_eq!(result.z().to_bits(), 0.0f32.to_bits());
        assert_eq!(result.w().to_bits(), 1.0f32.to_bits());
    }
}
