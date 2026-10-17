//! Port of: net/minecraft/client/model/geom/builders/UVPair.java
//! Java class(es): net.minecraft.client.model.geom.builders.UVPair
//! Status: VERIFIED
//!
//! Session 22. `public record UVPair(float u, float v)` with an overridden `toString` and three
//! static helpers that pack a UV pair into a `long`.
//!
//! # Packing goes through `floatToIntBits`, which canonicalises NaN
//!
//! ```java
//! long high = Float.floatToIntBits(u) & 4294967295L;
//! long low = Float.floatToIntBits(v) & 4294967295L;
//! return high << 32 | low;
//! ```
//!
//! `floatToIntBits` -- not `floatToRawIntBits` -- so every NaN packs as `0x7fc00000`, whatever its
//! payload; `pack` is therefore NOT injective on NaNs, and the probe has a signalling-NaN input to
//! show it. `unpackU` is `intBitsToFloat((int) (packed >> 32))` (an arithmetic shift, then a
//! narrowing that keeps the low 32 bits), `unpackV` is `intBitsToFloat((int) packed)`; both keep the
//! raw bits, so unpacking an arbitrary `long` -- including NaN patterns no `pack` produces -- returns
//! exactly those bits.
//!
//! # Record members
//!
//! `toString` is OVERRIDDEN: `"(" + u + "," + v + ")"`, with Java's `Float.toString` and no space.
//! `equals` compares both floats with `Float.compare` (hand-written on
//! [`java_object::equals_f32`]); `hashCode` is the record fold over `Float.hashCode`.
//!
//! Verified by the `REC`, `RECEQ`, `UVPACK` and `UVUNPACK` groups of `small_probe.txt`
//! (`tests/session22.rs`): every pair of ten special floats through `pack`/`unpackU`/`unpackV`, and
//! eleven raw `long`s through the unpackers.

use crate::javacompat::java_lang::float_to_string::float_to_string;
use crate::javacompat::java_object;

/// Port of `net.minecraft.client.model.geom.builders.UVPair`.
#[derive(Debug, Clone, Copy)]
pub struct UVPair {
    u: f32,
    v: f32,
}

impl UVPair {
    /// Port of the canonical constructor.
    pub const fn new(u: f32, v: f32) -> Self {
        UVPair { u, v }
    }

    /// Port of the accessor `u()`.
    pub const fn u(&self) -> f32 {
        self.u
    }

    /// Port of the accessor `v()`.
    pub const fn v(&self) -> f32 {
        self.v
    }

    /// Port of `pack(float u, float v)`.
    pub fn pack(u: f32, v: f32) -> i64 {
        let high = java_object::float_to_int_bits(u) as i64 & 4_294_967_295;
        let low = java_object::float_to_int_bits(v) as i64 & 4_294_967_295;
        (high << 32) | low
    }

    /// Port of `unpackU(long packedUV)`: `Float.intBitsToFloat((int) (packedUV >> 32))`.
    pub fn unpack_u(packed_uv: i64) -> f32 {
        let bits = (packed_uv >> 32) as i32;
        f32::from_bits(bits as u32)
    }

    /// Port of `unpackV(long packedUV)`: `Float.intBitsToFloat((int) packedUV)`.
    pub fn unpack_v(packed_uv: i64) -> f32 {
        f32::from_bits(packed_uv as i32 as u32)
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[java_object::hash_f32(self.u), java_object::hash_f32(self.v)])
    }
}

impl PartialEq for UVPair {
    /// Port of the record's generated `equals`: `Float.compare` on both components.
    fn eq(&self, other: &Self) -> bool {
        java_object::equals_f32(self.u, other.u) && java_object::equals_f32(self.v, other.v)
    }
}

impl Eq for UVPair {}

impl std::hash::Hash for UVPair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.java_hash_code().hash(state)
    }
}

impl std::fmt::Display for UVPair {
    /// Port of the `toString()` override: `"(" + u + "," + v + ")"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", float_to_string(self.u), float_to_string(self.v))
    }
}
