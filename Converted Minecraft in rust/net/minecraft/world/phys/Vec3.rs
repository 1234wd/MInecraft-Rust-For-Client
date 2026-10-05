//! Port of: net/minecraft/world/phys/Vec3.java
//! Java class(es): net.minecraft.world.phys.Vec3
//! Status: PARTIAL
//!
//! PARTIAL for exactly one reason: `CODEC`, `STREAM_CODEC` and `LP_STREAM_CODEC` are
//! `todo!()`, deferred to the serialisation batch (DataFixerUpper is not ported yet).
//! See DESIGN_DECISIONS.md (#dfu-proposal).
//!
//! # COVERAGE: 40 OF 41 GROUPS BIT-EXACT -- SO SAY SO PRECISELY
//!
//! `batch2.txt` has 41 `vec3.*` groups. **40 are bit-exact.** The one that is not is
//! `vec3.rotation`, and it is measured rather than waved away:
//!
//! | quantity | golden rows | bit-exact | cause |
//! |---|---|---|---|
//! | `rotation` yaw | 512 | **376** | host `atan2` vs HotSpot, 1 ULP |
//! | `rotation` pitch | 512 | **333** | host `asin` vs HotSpot, 1 ULP |
//! | `rotation` axis-aligned yaw | 112 | **112** | asserted exactly |
//! | `addLocalCoordinates` (uses `rotation`) | 512 | **512** | - |
//!
//! So `addLocalCoordinates` is FULLY exact despite going through `rotation()`: the 1-ULP
//! yaw/pitch difference is absorbed by the following `Mth` table lookups and `float`
//! products. The exposure is code that reads `rotation()` directly -- entity yaw/pitch,
//! camera angles.
//!
//! Closing this needs `jvm_math::asin` and `jvm_math::atan2`. Until then this file is
//! PARTIAL for TWO reasons: the three codec members, and `rotation()`.
//! See `_porting/tests/parity_batch2.rs`.
//!
//! # WIDTHS
//!
//! `Vec3` fields are Java `double`, not `float`, and every method here keeps them `f64`.
//! The places that DO narrow are narrowed deliberately and marked:
//! * `xRot`/`yRot`/`zRot` call `Mth.cos`/`Mth.sin`, which take and return `float`;
//! * `directionFromRotation` and `rotation()` work in `float` and widen on return.
//!
//! # `Math.sqrt` NEEDS NO SPECIAL TREATMENT
//!
//! Unlike `Math.log`, `Math.sqrt` is required by IEEE-754 to be correctly rounded, so the
//! hardware `SQRTSD` instruction and Rust's `f64::sqrt` agree bit for bit everywhere.
//! `jvm_math` is NOT used for it, and that is a measured decision rather than an omission.
//! See DESIGN_DECISIONS.md (#math-log-is-not-fdlibm).
//!
//! # `Math.atan2` AND `Math.asin` IN `rotation()`
//!
//! These two ARE host transcendentals, so `rotation()` -- and therefore
//! `addLocalCoordinates()`, which calls it -- is a known-divergence site. It is pinned
//! rather than hidden; see `parity_batch2.rs`.
//!
//! # `equals` USES `Double.compare`, NOT `==`
//!
//! Two consequences that `==` would get wrong:
//! * `Double.compare(NaN, NaN) == 0`, so a `Vec3` of NaN **equals itself**.
//! * `Double.compare(+0.0, -0.0) != 0`, so `+0.0` does **not** equal `-0.0`.
//!
//! Both are asserted by `vec3.equalsSpecial`.

use crate::javacompat::java_lang;
use crate::net::minecraft::core::Direction::{Axis, Direction};
use crate::net::minecraft::core::Vec3i::Vec3i;
use crate::net::minecraft::util::Mth::Mth;

/// Port of `Vec3`.
///
/// Fields are `pub` and public-by-value like the Java ones: `public final double x`.
#[derive(Clone, Copy)]
pub struct Vec3 {
    /// Port of `Vec3#x`.
    pub x: f64,
    /// Port of `Vec3#y`.
    pub y: f64,
    /// Port of `Vec3#z`.
    pub z: f64,
}

/// Degrees per radian, as written in the Java source.
const DEG_TO_RAD_F32: f32 = (std::f64::consts::PI / 180.0) as f32;
/// `180.0F / (float) Math.PI`, the inverse used by `rotation()`.
const RAD_TO_DEG_F32: f32 = (180.0 / std::f64::consts::PI) as f32;
/// `(float) Math.PI`, as written in `directionFromRotation`.
const PI_F32: f32 = std::f64::consts::PI as f32;

impl Vec3 {
    // -----------------------------------------------------------------------
    // Deferred codec members
    // -----------------------------------------------------------------------

    /// Port of `Vec3#CODEC`.
    pub fn CODEC() -> () {
        todo!("PORT: codec — Vec3#CODEC")
    }

    /// Port of `Vec3#STREAM_CODEC`.
    pub fn STREAM_CODEC() -> () {
        todo!("PORT: codec — Vec3#STREAM_CODEC")
    }

    /// Port of `Vec3#LP_STREAM_CODEC`.
    pub fn LP_STREAM_CODEC() -> () {
        todo!("PORT: codec — Vec3#LP_STREAM_CODEC")
    }

    /// Port of `Vec3#ZERO`.
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// Port of `Vec3#X_AXIS`.
    pub const X_AXIS: Vec3 = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };

    /// Port of `Vec3#Y_AXIS`.
    pub const Y_AXIS: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    /// Port of `Vec3#Z_AXIS`.
    pub const Z_AXIS: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    /// Port of `Vec3#Vec3(double,double,double)`.
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // -----------------------------------------------------------------------
    // Position accessors
    // -----------------------------------------------------------------------

    /// Port of `Vec3#x()`.
    #[inline]
    pub const fn x(&self) -> f64 {
        self.x
    }

    /// Port of `Vec3#y()`.
    #[inline]
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Port of `Vec3#z()`.
    #[inline]
    pub const fn z(&self) -> f64 {
        self.z
    }

    /// Port of `Vec3#atLowerCornerOf(Vec3i)`.
    ///
    /// The `int` coordinates are WIDENED to `double`, so a block position up to
    /// 30,000,000 is exact (a `double` holds every integer below 2^53).
    #[inline]
    pub fn at_lower_corner_of(pos: &Vec3i) -> Self {
        Self::new(pos.get_x() as f64, pos.get_y() as f64, pos.get_z() as f64)
    }

    /// Port of `Vec3#atLowerCornerWithOffset(Vec3i,double,double,double)`.
    #[inline]
    pub fn at_lower_corner_with_offset(pos: &Vec3i, x: f64, y: f64, z: f64) -> Self {
        Self::new(
            pos.get_x() as f64 + x,
            pos.get_y() as f64 + y,
            pos.get_z() as f64 + z,
        )
    }

    /// Port of `Vec3#atCenterOf(Vec3i)`.
    #[inline]
    pub fn at_center_of(pos: &Vec3i) -> Self {
        Self::at_lower_corner_with_offset(pos, 0.5, 0.5, 0.5)
    }

    /// Port of `Vec3#atBottomCenterOf(Vec3i)`.
    ///
    /// Note the Y offset is `0.0`, not `0.5`: this is the bottom face's centre.
    #[inline]
    pub fn at_bottom_center_of(pos: &Vec3i) -> Self {
        Self::at_lower_corner_with_offset(pos, 0.5, 0.0, 0.5)
    }

    /// Port of `Vec3#upFromBottomCenterOf(Vec3i,double)`.
    #[inline]
    pub fn up_from_bottom_center_of(pos: &Vec3i, y_offset: f64) -> Self {
        Self::at_lower_corner_with_offset(pos, 0.5, y_offset, 0.5)
    }

    // -----------------------------------------------------------------------
    // Arithmetic
    // -----------------------------------------------------------------------

    /// Port of `Vec3#add(double,double,double)`.
    #[inline]
    pub fn add(&self, x: f64, y: f64, z: f64) -> Self {
        Self::new(
            java_lang::add_f64(self.x, x),
            java_lang::add_f64(self.y, y),
            java_lang::add_f64(self.z, z),
        )
    }

    /// Port of `Vec3#add(double)`.
    #[inline]
    pub fn add_scalar(&self, value: f64) -> Self {
        self.add(value, value, value)
    }

    /// Port of `Vec3#add(Vec3)`.
    #[inline]
    pub fn add_vec3(&self, vec: &Self) -> Self {
        self.add(vec.x, vec.y, vec.z)
    }

    /// Port of `Vec3#subtract(double,double,double)`.
    ///
    /// Java writes `this.add(-x, -y, -z)`, so the negations are plain Java negation and
    /// therefore WRAP on `-0.0` and on the int-range edge cases that don't apply to f64.
    #[inline]
    pub fn subtract(&self, x: f64, y: f64, z: f64) -> Self {
        self.add(-x, -y, -z)
    }

    /// Port of `Vec3#subtract(double)`.
    #[inline]
    pub fn subtract_scalar(&self, value: f64) -> Self {
        self.subtract(value, value, value)
    }

    /// Port of `Vec3#subtract(Vec3)`.
    #[inline]
    pub fn subtract_vec3(&self, vec: &Self) -> Self {
        self.subtract(vec.x, vec.y, vec.z)
    }

    /// Port of `Vec3#vectorTo(Vec3)`.
    #[inline]
    pub fn vector_to(&self, vec: &Self) -> Self {
        Self::new(
            java_lang::sub_f64(vec.x, self.x),
            java_lang::sub_f64(vec.y, self.y),
            java_lang::sub_f64(vec.z, self.z),
        )
    }

    /// Port of `Vec3#multiply(double,double,double)`.
    #[inline]
    pub fn multiply(&self, x_scale: f64, y_scale: f64, z_scale: f64) -> Self {
        Self::new(
            java_lang::mul_f64(self.x, x_scale),
            java_lang::mul_f64(self.y, y_scale),
            java_lang::mul_f64(self.z, z_scale),
        )
    }

    /// Port of `Vec3#multiply(Vec3)`.
    #[inline]
    pub fn multiply_vec3(&self, scale: &Self) -> Self {
        self.multiply(scale.x, scale.y, scale.z)
    }

    /// Port of `Vec3#scale(double)`.
    #[inline]
    pub fn scale(&self, scale: f64) -> Self {
        self.multiply(scale, scale, scale)
    }

    /// Port of `Vec3#reverse()` -- which is `this.scale(-1.0)`.
    #[inline]
    pub fn reverse(&self) -> Self {
        self.scale(-1.0)
    }

    /// Port of `Vec3#horizontal()`.
    #[inline]
    pub fn horizontal(&self) -> Self {
        Self::new(self.x, 0.0, self.z)
    }

    /// Port of `Vec3#dot(Vec3)`.
    ///
    /// Sum order is left-to-right: `x*x + y*y + z*z`, which for floating point is NOT
    /// the same as any other association.
    #[inline]
    pub fn dot(&self, vec: &Self) -> f64 {
        java_lang::add_f64(
            java_lang::add_f64(
                java_lang::mul_f64(self.x, vec.x),
                java_lang::mul_f64(self.y, vec.y),
            ),
            java_lang::mul_f64(self.z, vec.z),
        )
    }

    /// Port of `Vec3#cross(Vec3)`.
    ///
    /// Note the operands are NOT normalized, so the magnitude scales with both inputs.
    #[inline]
    pub fn cross(&self, vec: &Self) -> Self {
        Self::new(
            java_lang::sub_f64(
                java_lang::mul_f64(self.y, vec.z),
                java_lang::mul_f64(self.z, vec.y),
            ),
            java_lang::sub_f64(
                java_lang::mul_f64(self.z, vec.x),
                java_lang::mul_f64(self.x, vec.z),
            ),
            java_lang::sub_f64(
                java_lang::mul_f64(self.x, vec.y),
                java_lang::mul_f64(self.y, vec.x),
            ),
        )
    }

    // -----------------------------------------------------------------------
    // Lengths
    // -----------------------------------------------------------------------

    /// Port of `Vec3#lengthSqr()`.
    #[inline]
    pub fn length_sqr(&self) -> f64 {
        java_lang::add_f64(
            java_lang::add_f64(
                java_lang::mul_f64(self.x, self.x),
                java_lang::mul_f64(self.y, self.y),
            ),
            java_lang::mul_f64(self.z, self.z),
        )
    }

    /// Port of `Vec3#length()`.
    ///
    /// `Math.sqrt` on a `double` is correctly rounded by IEEE-754, so this is
    /// `f64::sqrt` and needs no `jvm_math` route.
    #[inline]
    pub fn length(&self) -> f64 {
        self.length_sqr().sqrt()
    }

    /// Port of `Vec3#horizontalDistanceSqr()`.
    #[inline]
    pub fn horizontal_distance_sqr(&self) -> f64 {
        java_lang::add_f64(
            java_lang::mul_f64(self.x, self.x),
            java_lang::mul_f64(self.z, self.z),
        )
    }

    /// Port of `Vec3#horizontalDistance()`.
    #[inline]
    pub fn horizontal_distance(&self) -> f64 {
        self.horizontal_distance_sqr().sqrt()
    }

    /// Port of `Vec3#normalize()`.
    ///
    /// ```java
    /// double dist = Math.sqrt(this.x * this.x + this.y * this.y + this.z * this.z);
    /// return dist < 1.0E-5F ? ZERO : new Vec3(this.x / dist, this.y / dist, this.z / dist);
    /// ```
    ///
    /// # THE `1.0E-5F` IS A FLOAT LITERAL COMPARED AGAINST A DOUBLE
    ///
    /// `1.0E-5F` widens to the double nearest `1.0e-5`, which is NOT the same as the
    /// double literal `1.0E-5`. Getting that wrong moves the zero-shortcut threshold by
    /// enough to change results for vectors whose length sits between the two, so the
    /// constant is written as an `f32` widened by `as f64` below rather than as an
    /// `f64` literal.
    #[inline]
    pub fn normalize(&self) -> Self {
        let dist = self.length();
        // `1.0E-5F` widened: NOT the f64 literal.
        if dist < (1.0e-5f32) as f64 {
            Self::ZERO
        } else {
            Self::new(
                java_lang::div_f64(self.x, dist),
                java_lang::div_f64(self.y, dist),
                java_lang::div_f64(self.z, dist),
            )
        }
    }

    // -----------------------------------------------------------------------
    // Distances
    // -----------------------------------------------------------------------

    /// Port of `Vec3#distanceToSqr(double,double,double)`.
    #[inline]
    pub fn distance_to_sqr(&self, x: f64, y: f64, z: f64) -> f64 {
        let xd = java_lang::sub_f64(x, self.x);
        let yd = java_lang::sub_f64(y, self.y);
        let zd = java_lang::sub_f64(z, self.z);
        java_lang::add_f64(
            java_lang::add_f64(
                java_lang::mul_f64(xd, xd),
                java_lang::mul_f64(yd, yd),
            ),
            java_lang::mul_f64(zd, zd),
        )
    }

    /// Port of `Vec3#distanceToSqr(Vec3)`.
    #[inline]
    pub fn distance_to_sqr_vec3(&self, vec: &Self) -> f64 {
        self.distance_to_sqr(vec.x, vec.y, vec.z)
    }

    /// Port of `Vec3#distanceTo(Vec3)`.
    #[inline]
    pub fn distance_to(&self, vec: &Self) -> f64 {
        self.distance_to_sqr(vec.x, vec.y, vec.z).sqrt()
    }

    /// Port of `Vec3#closerThan(Position,double)`.
    #[inline]
    pub fn closer_than(&self, x: f64, y: f64, z: f64, distance: f64) -> bool {
        self.distance_to_sqr(x, y, z) < Mth::square_f64(distance)
    }

    /// Port of `Vec3#closerThan(Vec3,double,double)`.
    ///
    /// Note this is the OVERLOAD taking two distances -- it compares the XZ distance
    /// squared against `distanceXZ^2` and the Y distance against `distanceY`
    /// ABSOLUTELY. The Y comparison is not squared, so the two arguments are not
    /// interchangeable.
    #[inline]
    pub fn closer_than_xz(&self, vec: &Self, distance_xz: f64, distance_y: f64) -> bool {
        let dx = java_lang::sub_f64(vec.x, self.x);
        let dy = java_lang::sub_f64(vec.y, self.y);
        let dz = java_lang::sub_f64(vec.z, self.z);
        Mth::length_squared_2_f64(dx, dz) < Mth::square_f64(distance_xz)
            && java_lang::abs_f64(dy) < distance_y
    }

    // -----------------------------------------------------------------------
    // equals / hashCode
    // -----------------------------------------------------------------------

    /// Port of `Vec3#equals(Object)`.
    ///
    /// Uses `Double.compare`, NOT `==`. See the module docs for the two consequences.
    #[inline]
    pub fn java_equals(&self, other: &Self) -> bool {
        double_compare(other.x, self.x) == 0
            && double_compare(other.y, self.y) == 0
            && double_compare(other.z, self.z) == 0
    }

    /// Port of `Vec3#hashCode()`.
    ///
    /// ```java
    /// long temp = Double.doubleToLongBits(this.x);
    /// int result = (int)(temp ^ temp >>> 32);
    /// ...
    /// ```
    ///
    /// `doubleToLongBits` (not `RawLongBits`) CANONICALISES every NaN, so all NaNs hash
    /// identically. Each component is folded to the low 32 bits by `x ^ (x >>> 32)`.
    #[inline]
    pub fn java_hash_code(&self) -> i32 {
        fn fold(v: f64) -> i32 {
            let bits = double_to_long_bits(v);
            (bits ^ ((bits as u64) >> 32)) as i32
        }
        let mut result = fold(self.x);
        result = result.wrapping_mul(31).wrapping_add(fold(self.y));
        result.wrapping_mul(31).wrapping_add(fold(self.z))
    }

    // -----------------------------------------------------------------------
    // Rotations
    // -----------------------------------------------------------------------

    /// Port of `Vec3#xRot(float)`.
    ///
    /// `Mth.cos`/`Mth.sin` take and return `float`, so the trigonometry is single
    /// precision; only the products with `this.y`/`this.z` are `double`.
    #[inline]
    pub fn x_rot(&self, radians: f32) -> Self {
        let cos = Mth::cos(radians as f64);
        let sin = Mth::sin(radians as f64);
        let xx = self.x;
        let yy = java_lang::add_f64(
            java_lang::mul_f64(self.y, cos as f64),
            java_lang::mul_f64(self.z, sin as f64),
        );
        let zz = java_lang::sub_f64(
            java_lang::mul_f64(self.z, cos as f64),
            java_lang::mul_f64(self.y, sin as f64),
        );
        Self::new(xx, yy, zz)
    }

    /// Port of `Vec3#yRot(float)`.
    #[inline]
    pub fn y_rot(&self, radians: f32) -> Self {
        let cos = Mth::cos(radians as f64);
        let sin = Mth::sin(radians as f64);
        let xx = java_lang::add_f64(
            java_lang::mul_f64(self.x, cos as f64),
            java_lang::mul_f64(self.z, sin as f64),
        );
        let yy = self.y;
        let zz = java_lang::sub_f64(
            java_lang::mul_f64(self.z, cos as f64),
            java_lang::mul_f64(self.x, sin as f64),
        );
        Self::new(xx, yy, zz)
    }

    /// Port of `Vec3#zRot(float)`.
    ///
    /// # DECOMPILER ARTIFACT: THE JAR'S SIGNS ARE OPPOSITE TO THE DECOMPILED SOURCE
    ///
    /// `minecraft-decompiled/net/minecraft/world/phys/Vec3.java` says
    ///
    /// ```text
    /// return new Vec3(this.x * f1 - this.y * f2, this.y * f1 + this.x * f2, this.z);
    /// ```
    ///
    /// `javap -c` on `minecraft-merged-deobf-26.2.jar` says otherwise -- `dadd` where the
    /// source has `-`, and `dsub` where the source has `+`:
    ///
    /// ```text
    /// 13: getfield  x    17: f2d   18: dmul     <- x * cos
    /// 20: getfield  y    25: f2d   26: dmul     <- y * sin
    /// 26: dadd            27: dstore 4        <- ADD, not sub
    /// 30: getfield  y    35: f2d   37: dmul     <- y * cos
    /// 37: getfield  x    42: f2d   43: dmul     <- x * sin
    /// 43: dsub            44: dstore 6        <- SUB, not add
    /// ```
    ///
    /// **The jar wins** (project rule: the jar is ground truth). So this rotates the
    /// OPPOSITE way from the decompiled source, and it is not a cosmetic difference:
    /// `zRot(pi)` sends `+X` to `-Y` in the jar and to `+Y` in the source.
    ///
    /// `xRot` and `yRot` were checked the same way and DO match the decompiled source
    /// (`xRot` uses `dadd` then `dsub`, `yRot` uses `dadd` then `dsub`) -- which is why
    /// only `vec3.zRot` failed and the other two rot groups passed on the first run.
    ///
    /// The `vec3.zRot` golden row that catches it is the signed-zero one:
    /// `Vec3(0,0,0).zRot(180.0F)` is `-0.0, +0.0, +0.0` in the jar, because
    /// `-0.0 + -0.0 == -0.0` whereas `-0.0 - -0.0 == +0.0`.
    #[inline]
    pub fn z_rot(&self, radians: f32) -> Self {
        let cos = Mth::cos(radians as f64);
        let sin = Mth::sin(radians as f64);
        // ADD in x, SUB in y -- the reverse of the decompiled source. See above.
        let xx = java_lang::add_f64(
            java_lang::mul_f64(self.x, cos as f64),
            java_lang::mul_f64(self.y, sin as f64),
        );
        let yy = java_lang::sub_f64(
            java_lang::mul_f64(self.y, cos as f64),
            java_lang::mul_f64(self.x, sin as f64),
        );
        let zz = self.z;
        Self::new(xx, yy, zz)
    }

    /// Port of `Vec3#rotateClockwise90()`.
    #[inline]
    pub fn rotate_clockwise_90(&self) -> Self {
        Self::new(-self.z, self.y, self.x)
    }

    /// Port of `Vec3#directionFromRotation(float,float)`.
    ///
    /// The whole expression is `float` arithmetic: `Mth.cos`/`Mth.sin` are `float`, the
    /// products are `float`, and only the final `Vec3` construction widens. Note the
    /// `-rotY * deg - PI` and `-rotX * deg` signs -- they are what makes yaw increase
    /// clockwise.
    #[inline]
    pub fn direction_from_rotation(rot_x: f32, rot_y: f32) -> Self {
        // ```java
        // float yCos = Mth.cos(-rotY * (float) (Math.PI / 180.0) - (float) Math.PI);
        // float ySin = Mth.sin(-rotY * (float) (Math.PI / 180.0) - (float) Math.PI);
        // float xCos = -Mth.cos(-rotX * (float) (Math.PI / 180.0));
        // float xSin =  Mth.sin(-rotX * (float) (Math.PI / 180.0));
        // return new Vec3(ySin * xCos, xSin, yCos * xCos);
        // ```
        //
        // TWO WIDTH FACTS, both easy to get wrong:
        //
        // 1. The ARGUMENT to each `Mth.cos`/`Mth.sin` is computed in `f32` and then
        //    WIDENED, because `Mth.cos` takes a `double`. So `-rotY * DEG - PI` is
        //    single precision arithmetic whose result is passed as a `f64`. Computing it
        //    in `f64` gives different bits.
        //
        // 2. `xCos` is NEGATED (`-Mth.cos(...)`), and only `xCos`. My first pass folded the
        //    minus into the argument of `Mth.sin` as well, which silently changed both
        //    components. The `vec3.directionFromRotation` group is what caught it.
        let y_arg = java_lang::sub_f32(
            java_lang::mul_f32(-rot_y, DEG_TO_RAD_F32),
            PI_F32,
        );
        let x_arg = java_lang::mul_f32(-rot_x, DEG_TO_RAD_F32);

        let y_cos = Mth::cos(y_arg as f64);
        let y_sin = Mth::sin(y_arg as f64);
        let x_cos = -Mth::cos(x_arg as f64);
        let x_sin = Mth::sin(x_arg as f64);

        Self::new(
            java_lang::mul_f32(y_sin, x_cos) as f64,
            x_sin as f64,
            java_lang::mul_f32(y_cos, x_cos) as f64,
        )
    }

    /// Port of `Vec3#directionFromRotation(Vec2)`.
    #[inline]
    pub fn direction_from_rotation_vec2(rotation: &crate::net::minecraft::world::phys::Vec2::Vec2) -> Self {
        Self::direction_from_rotation(rotation.x, rotation.y)
    }

    /// Port of `Vec3#rotation()`.
    ///
    /// # HOST TRANSCENDENTALS: `Math.atan2` and `Math.asin`
    ///
    /// ```java
    /// float yaw   = (float)Math.atan2(-this.x, this.z) * (180.0F / (float)Math.PI);
    /// float pitch = (float)Math.asin(-this.y / Math.sqrt(...)) * (180.0F / (float)Math.PI);
    /// ```
    ///
    /// Both are evaluated by HotSpot intrinsics that no Rust implementation is known to
    /// reproduce bit-for-bit (the `Math.log` case is documented in DESIGN_DECISIONS.md
    /// under `#math-log-is-not-fdlibm`). This is therefore a KNOWN
    /// DIVERGENCE site: `rotation()` and everything downstream of it --
    /// `addLocalCoordinates()`, entity yaw/pitch, mob facing -- is only as exact as the
    /// host libm happens to be.
    ///
    /// Pinned and counted in `parity_batch2.rs`; see OPEN_QUESTIONS #16.
    ///
    /// # THE ARGUMENT ORDER IS NOT SYMMETRIC -- WATCH IT
    ///
    /// ```java
    /// float yaw = (float) Math.atan2(-this.x, this.z) * (180.0F / (float) Math.PI);
    /// ```
    ///
    /// Rust writes this as `(-self.x).atan2(self.z)`. The tempting `self.x.atan2(-self.z)`
    /// is a DIFFERENT function -- negating both arguments shifts the result by `pi`, it
    /// does not cancel -- and it was wrong in my first pass:
    ///
    /// | input `(x, y, z)` | correct yaw | wrong-order yaw |
    /// |---|---|---|
    /// | `(0, 0, 0)` | `-0.0` | `180.0` |
    /// | `(0, 0, 1)` | `-0.0` | `180.0` |
    /// | `(0, 0, -1)` | `-180.0` | `0.0` |
    ///
    /// Note the signed zeros: `atan2(-0.0, 1.0)` is `-0.0`, so yaw is `-0.0` and not `0.0`.
    /// `vec3.rotation` is what caught this -- 0 of 512 rows matched.
    #[inline]
    pub fn rotation(&self) -> crate::net::minecraft::world::phys::Vec2::Vec2 {
        // ```java
        // float yaw   = (float) Math.atan2(-this.x, this.z) * (180.0F / (float) Math.PI);
        // float pitch = (float) Math.asin(-this.y / Math.sqrt(this.x*this.x + this.y*this.y + this.z*this.z))
        //                    * (180.0F / (float) Math.PI);
        // ```
        //
        // The divisor is `Math.sqrt(...)` -- the LENGTH -- so `self.length()`, not
        // `self.length().sqrt()`. (My first pass double-rooted it, which is a real
        // behaviour change even though the shape of the code looks plausible.)
        let yaw = ((-self.x).atan2(self.z) as f32) * RAD_TO_DEG_F32;
        let pitch = ((-self.y / self.length()).asin() as f32) * RAD_TO_DEG_F32;
        crate::net::minecraft::world::phys::Vec2::Vec2::new(pitch, yaw)
    }

    /// Port of `Vec3#align(EnumSet<Axis>)`.
    ///
    /// Java takes an `EnumSet`; Rust takes a slice of axes. An axis that is present has
    /// its component FLOORED, an absent one is left alone.
    #[inline]
    pub fn align(&self, axes: &[Axis]) -> Self {
        fn floor_if(present: bool, v: f64) -> f64 {
            if present {
                Mth::floor_f64(v) as f64
            } else {
                v
            }
        }
        Self::new(
            floor_if(axes.contains(&Axis::X), self.x),
            floor_if(axes.contains(&Axis::Y), self.y),
            floor_if(axes.contains(&Axis::Z), self.z),
        )
    }

    /// Port of `Vec3#get(Direction.Axis)`.
    ///
    /// Java writes `axis.choose(this.x, this.y, this.z)`. `Direction.Axis#choose` is a
    /// three-way select declared on `Direction.java`, and `Direction.rs` is item 9 of the
    /// session-06 loop rather than part of this file, so the dispatch is inlined here.
    /// When `Axis#choose` is ported it must be a plain `match` on the same three arms --
    /// there is no arithmetic in it, so inlining cannot change behaviour.
    #[inline]
    pub fn get(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    /// Port of `Vec3#with(Direction.Axis,double)`.
    ///
    /// Java writes `axis.choose(this.x, this.y, this.z)` per component. The `match` is
    /// written so each arm names the component it replaces, which is the same shape as
    /// `Vec3#get`.
    #[inline]
    pub fn with(&self, axis: Axis, value: f64) -> Self {
        match axis {
            Axis::X => Self::new(value, self.y, self.z),
            Axis::Y => Self::new(self.x, value, self.z),
            Axis::Z => Self::new(self.x, self.y, value),
        }
    }

    /// Port of `Vec3#relative(Direction,double)`.
    ///
    /// Uses the direction's integer unit vector, widened to `double`, so the offset is
    /// `distance * step` and can be fractional.
    #[inline]
    pub fn relative(&self, direction: Direction, distance: f64) -> Self {
        let normal = direction.get_unit_vec3i();
        Self::new(
            java_lang::add_f64(self.x, java_lang::mul_f64(distance, normal.get_x() as f64)),
            java_lang::add_f64(self.y, java_lang::mul_f64(distance, normal.get_y() as f64)),
            java_lang::add_f64(self.z, java_lang::mul_f64(distance, normal.get_z() as f64)),
        )
    }

    /// Port of `Vec3#lerp(Vec3,double)`.
    ///
    /// Component-wise `Mth.lerp`, i.e. `p0 + alpha * (p1 - p0)`.
    #[inline]
    pub fn lerp(&self, vec: &Self, a: f64) -> Self {
        Self::new(
            Mth::lerp_f64(a, self.x, vec.x),
            Mth::lerp_f64(a, self.y, vec.y),
            Mth::lerp_f64(a, self.z, vec.z),
        )
    }

    /// Port of `Vec3#projectedOn(Vec3)`.
    ///
    /// ```java
    /// return onto.lengthSqr() == 0.0 ? onto : onto.scale(this.dot(onto)).scale(1.0 / onto.lengthSqr());
    /// ```
    ///
    /// The zero-length case returns `onto` ITSELF, not a zero vector.
    #[inline]
    pub fn projected_on(&self, onto: &Self) -> Self {
        if onto.length_sqr() == 0.0 {
            *onto
        } else {
            onto.scale(self.dot(onto)).scale(1.0 / onto.length_sqr())
        }
    }

    /// Port of `Vec3#applyLocalCoordinatesToRotation(Vec2,Vec3)`.
    ///
    /// Entirely `float` arithmetic -- six `Mth.cos`/`Mth.sin` table lookups plus `float`
    /// products -- so this one IS exact, unlike `addLocalCoordinates` which reaches
    /// `rotation()`.
    #[inline]
    pub fn apply_local_coordinates_to_rotation(
        rotation: &crate::net::minecraft::world::phys::Vec2::Vec2,
        direction: &Self,
    ) -> Self {
        let y_cos = Mth::cos(java_lang::mul_f32(java_lang::add_f32(rotation.y, 90.0f32), DEG_TO_RAD_F32) as f64);
        let y_sin = Mth::sin(java_lang::mul_f32(java_lang::add_f32(rotation.y, 90.0f32), DEG_TO_RAD_F32) as f64);
        let x_cos = Mth::cos(java_lang::mul_f32(-rotation.x, DEG_TO_RAD_F32) as f64);
        let x_sin = Mth::sin(java_lang::mul_f32(-rotation.x, DEG_TO_RAD_F32) as f64);
        let x_cos_up = Mth::cos(java_lang::mul_f32(java_lang::add_f32(-rotation.x, 90.0f32), DEG_TO_RAD_F32) as f64);
        let x_sin_up = Mth::sin(java_lang::mul_f32(java_lang::add_f32(-rotation.x, 90.0f32), DEG_TO_RAD_F32) as f64);

        let forwards = Self::new(
            java_lang::mul_f32(y_cos, x_cos) as f64,
            x_sin as f64,
            java_lang::mul_f32(y_sin, x_cos) as f64,
        );
        let up = Self::new(
            java_lang::mul_f32(y_cos, x_cos_up) as f64,
            x_sin_up as f64,
            java_lang::mul_f32(y_sin, x_cos_up) as f64,
        );
        let left = forwards.cross(&up).scale(-1.0);

        let xa = java_lang::add_f64(
            java_lang::add_f64(
                java_lang::mul_f64(forwards.x, direction.z),
                java_lang::mul_f64(up.x, direction.y),
            ),
            java_lang::mul_f64(left.x, direction.x),
        );
        let ya = java_lang::add_f64(
            java_lang::add_f64(
                java_lang::mul_f64(forwards.y, direction.z),
                java_lang::mul_f64(up.y, direction.y),
            ),
            java_lang::mul_f64(left.y, direction.x),
        );
        let za = java_lang::add_f64(
            java_lang::add_f64(
                java_lang::mul_f64(forwards.z, direction.z),
                java_lang::mul_f64(up.z, direction.y),
            ),
            java_lang::mul_f64(left.z, direction.x),
        );
        Self::new(xa, ya, za)
    }

    /// Port of `Vec3#addLocalCoordinates(Vec3)`.
    ///
    /// Goes through `rotation()`, so it inherits the `atan2`/`asin` divergence noted on
    /// that method.
    #[inline]
    pub fn add_local_coordinates(&self, direction: &Self) -> Self {
        Self::apply_local_coordinates_to_rotation(&self.rotation(), direction)
    }

    /// Port of `Vec3#isFinite()`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Port of `Vec3#offsetRandom(RandomSource,float)`.
    ///
    /// The three draws are in X, Y, Z order and each is `(nextFloat() - 0.5f32) * offset`.
    #[inline]
    pub fn offset_random(&self, random: &mut dyn crate::net::minecraft::util::RandomSource::RandomSource, offset: f32) -> Self {
        self.add(
            java_lang::mul_f32(
                java_lang::sub_f32(random.next_float(), 0.5f32),
                offset,
            ) as f64,
            java_lang::mul_f32(
                java_lang::sub_f32(random.next_float(), 0.5f32),
                offset,
            ) as f64,
            java_lang::mul_f32(
                java_lang::sub_f32(random.next_float(), 0.5f32),
                offset,
            ) as f64,
        )
    }

    /// Port of `Vec3#offsetRandomXZ(RandomSource,float)`.
    ///
    /// Only TWO draws -- X then Z -- with Y pinned to `0.0`. A port that reused
    /// `offset_random` would consume three draws and desynchronise the stream.
    #[inline]
    pub fn offset_random_xz(&self, random: &mut dyn crate::net::minecraft::util::RandomSource::RandomSource, offset: f32) -> Self {
        self.add(
            java_lang::mul_f32(
                java_lang::sub_f32(random.next_float(), 0.5f32),
                offset,
            ) as f64,
            0.0,
            java_lang::mul_f32(
                java_lang::sub_f32(random.next_float(), 0.5f32),
                offset,
            ) as f64,
        )
    }
}

/// `Double.compare`: NaN equals NaN, and `+0.0` sorts above `-0.0`.
#[inline]
fn double_compare(a: f64, b: f64) -> i32 {
    if a < b {
        -1
    } else if a > b {
        1
    } else {
        // Equal or unordered. Java's `Double.doubleToLongBits` canonicalises NaN, so
        // comparing the canonical bits reproduces "all NaNs are equal".
        let ab = double_to_long_bits(a);
        let bb = double_to_long_bits(b);
        match ab.cmp(&bb) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

/// `Double.doubleToLongBits`: canonicalises every NaN to `0x7ff8000000000000`.
#[inline]
fn double_to_long_bits(v: f64) -> u64 {
    if v.is_nan() {
        0x7ff8_0000_0000_0000u64
    } else {
        v.to_bits()
    }
}

impl std::fmt::Display for Vec3 {
    /// Port of `Vec3#toString()`: `"(x, y, z)"`.
    ///
    /// Java string-concatenates the `double`s, so this uses Java's `Double.toString`
    /// layout -- NOT Rust's `Display`, which prints `0` where Java prints `0.0` and
    /// switches to scientific notation at different magnitudes. See
    /// `javacompat::java_lang::float_to_string`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            java_lang::float_to_string::double_to_string(self.x),
            java_lang::float_to_string::double_to_string(self.y),
            java_lang::float_to_string::double_to_string(self.z)
        )
    }
}

impl PartialEq for Vec3 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.java_equals(other)
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
