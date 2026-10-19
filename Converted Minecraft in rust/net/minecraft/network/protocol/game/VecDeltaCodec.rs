//! Port of: net/minecraft/network/protocol/game/VecDeltaCodec.java
//! Java class(es): net.minecraft.network.protocol.game.VecDeltaCodec
//! Status: VERIFIED
//!
//! Session 24. `public class VecDeltaCodec`: an entity's position sent as deltas from a base,
//! in fixed point with 4096 steps per block (`ServerEntity` encodes, `Entity` decodes).
//!
//! ```java
//! static long encode(double input) { return Math.round(input * 4096.0); }
//! static double decode(long v)     { return v / 4096.0; }
//! public Vec3 decode(long xa, long ya, long za) {
//!     if (xa == 0L && ya == 0L && za == 0L) return this.base;
//!     double x = xa == 0L ? this.base.x : decode(encode(this.base.x) + xa);   // y, z alike
//!     return new Vec3(x, y, z);
//! }
//! public long encodeX(Vec3 pos) { return encode(pos.x) - encode(this.base.x); }  // Y, Z alike
//! public Vec3 delta(Vec3 pos)   { return pos.subtract(this.base); }
//! ```
//!
//! * `Math.round(double)` rounds half UP and maps NaN to 0 and the infinities to the `long`
//!   extremes -- `javacompat::java_lang::round_f64`, the JDK body, added this session;
//! * a zero delta keeps the base coordinate EXACTLY (it is not re-quantised), and only a non-zero
//!   one goes through `encode`/`decode`; so `decode(1, 0, 0)` moves `x` onto the 1/4096 grid and
//!   leaves `y` and `z` as they were;
//! * the `long` sums and differences wrap, as Java's `ladd`/`lsub` do;
//! * `decode(long)` is `(double) v / 4096.0`.
//!
//! The two static helpers are package-private `@VisibleForTesting` in Java; they are `pub` here
//! (as [`encode`] and [`decode_i64`]) so the tests can reach them. The overloaded `decode` is
//! named by its parameter types (`DESIGN_DECISIONS.md#overload-naming`).
//!
//! Verified by the `VECDELTA` group of `sweep_probe.txt` (`tests/session24.rs`).

use crate::javacompat::java_lang;
use crate::net::minecraft::world::phys::Vec3::Vec3;

/// Port of `private static final double TRUNCATION_STEPS = 4096.0`.
const TRUNCATION_STEPS: f64 = 4096.0;

/// Port of `static long encode(double input)`: `Math.round(input * 4096.0)`.
pub fn encode(input: f64) -> i64 {
    java_lang::round_f64(java_lang::mul_f64(input, TRUNCATION_STEPS))
}

/// Port of `static double decode(long v)`: `v / 4096.0`.
pub fn decode_i64(v: i64) -> f64 {
    java_lang::div_f64(v as f64, TRUNCATION_STEPS)
}

/// Port of `net.minecraft.network.protocol.game.VecDeltaCodec`.
#[derive(Clone, Copy)]
pub struct VecDeltaCodec {
    base: Vec3,
}

impl Default for VecDeltaCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl VecDeltaCodec {
    /// Port of the implicit constructor: `base = Vec3.ZERO`.
    pub const fn new() -> Self {
        VecDeltaCodec { base: Vec3::ZERO }
    }

    /// Port of `public Vec3 decode(long xa, long ya, long za)`.
    pub fn decode_i64_i64_i64(&self, xa: i64, ya: i64, za: i64) -> Vec3 {
        if xa == 0 && ya == 0 && za == 0 {
            return self.base;
        }
        let x = if xa == 0 {
            self.base.x
        } else {
            decode_i64(encode(self.base.x).wrapping_add(xa))
        };
        let y = if ya == 0 {
            self.base.y
        } else {
            decode_i64(encode(self.base.y).wrapping_add(ya))
        };
        let z = if za == 0 {
            self.base.z
        } else {
            decode_i64(encode(self.base.z).wrapping_add(za))
        };
        Vec3::new(x, y, z)
    }

    /// Port of `public long encodeX(Vec3 pos)`.
    pub fn encode_x(&self, pos: &Vec3) -> i64 {
        encode(pos.x).wrapping_sub(encode(self.base.x))
    }

    /// Port of `public long encodeY(Vec3 pos)`.
    pub fn encode_y(&self, pos: &Vec3) -> i64 {
        encode(pos.y).wrapping_sub(encode(self.base.y))
    }

    /// Port of `public long encodeZ(Vec3 pos)`.
    pub fn encode_z(&self, pos: &Vec3) -> i64 {
        encode(pos.z).wrapping_sub(encode(self.base.z))
    }

    /// Port of `public Vec3 delta(Vec3 pos)`: `pos.subtract(this.base)`.
    pub fn delta(&self, pos: &Vec3) -> Vec3 {
        pos.subtract_vec3(&self.base)
    }

    /// Port of `public void setBase(Vec3 base)`.
    pub fn set_base(&mut self, base: Vec3) {
        self.base = base;
    }

    /// Port of `public Vec3 getBase()`.
    pub fn get_base(&self) -> Vec3 {
        self.base
    }
}

impl std::fmt::Debug for VecDeltaCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VecDeltaCodec {{ base: {} }}", self.base)
    }
}
