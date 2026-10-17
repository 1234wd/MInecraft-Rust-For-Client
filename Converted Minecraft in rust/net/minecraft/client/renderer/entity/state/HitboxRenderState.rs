//! Port of: net/minecraft/client/renderer/entity/state/HitboxRenderState.java
//! Java class(es): net.minecraft.client.renderer.entity.state.HitboxRenderState
//! Status: VERIFIED
//!
//! Session 22. `public record HitboxRenderState(double x0, double y0, double z0, double x1,
//! double y1, double z1, float offsetX, float offsetY, float offsetZ, float red, float green,
//! float blue)` with one extra constructor that leaves the offsets at `0.0F`.
//!
//! # Record members
//!
//! `equals` compares every `double` with `Double.compare` and every `float` with `Float.compare`
//! (NaN equals NaN of any payload, `0.0 != -0.0`), so it is hand-written on
//! [`java_object::equals_f64`] / [`java_object::equals_f32`]; `hashCode` is the record fold over
//! `Double.hashCode` / `Float.hashCode`; `toString` prints each with Java's `Double.toString` /
//! `Float.toString`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`): both
//! constructors, a `-0.0` corner, and two instances that differ only in their NaN payloads (which
//! Java, and this port, call equal).

use crate::javacompat::java_lang::float_to_string::{double_to_string, float_to_string};
use crate::javacompat::java_object;

/// Port of `net.minecraft.client.renderer.entity.state.HitboxRenderState`.
#[derive(Debug, Clone, Copy)]
pub struct HitboxRenderState {
    x0: f64,
    y0: f64,
    z0: f64,
    x1: f64,
    y1: f64,
    z1: f64,
    offset_x: f32,
    offset_y: f32,
    offset_z: f32,
    red: f32,
    green: f32,
    blue: f32,
}

impl HitboxRenderState {
    /// Port of the canonical constructor.
    #[allow(clippy::too_many_arguments)] // Twelve components is the shape of the record.
    pub const fn new(
        x0: f64,
        y0: f64,
        z0: f64,
        x1: f64,
        y1: f64,
        z1: f64,
        offset_x: f32,
        offset_y: f32,
        offset_z: f32,
        red: f32,
        green: f32,
        blue: f32,
    ) -> Self {
        HitboxRenderState {
            x0,
            y0,
            z0,
            x1,
            y1,
            z1,
            offset_x,
            offset_y,
            offset_z,
            red,
            green,
            blue,
        }
    }

    /// Port of `HitboxRenderState(x0, y0, z0, x1, y1, z1, red, green, blue)`: zero offsets.
    #[allow(clippy::too_many_arguments)]
    pub const fn new_without_offset(
        x0: f64,
        y0: f64,
        z0: f64,
        x1: f64,
        y1: f64,
        z1: f64,
        red: f32,
        green: f32,
        blue: f32,
    ) -> Self {
        HitboxRenderState::new(x0, y0, z0, x1, y1, z1, 0.0, 0.0, 0.0, red, green, blue)
    }

    /// Port of the accessor `x0()`.
    pub const fn x0(&self) -> f64 {
        self.x0
    }

    /// Port of the accessor `y0()`.
    pub const fn y0(&self) -> f64 {
        self.y0
    }

    /// Port of the accessor `z0()`.
    pub const fn z0(&self) -> f64 {
        self.z0
    }

    /// Port of the accessor `x1()`.
    pub const fn x1(&self) -> f64 {
        self.x1
    }

    /// Port of the accessor `y1()`.
    pub const fn y1(&self) -> f64 {
        self.y1
    }

    /// Port of the accessor `z1()`.
    pub const fn z1(&self) -> f64 {
        self.z1
    }

    /// Port of the accessor `offsetX()`.
    pub const fn offset_x(&self) -> f32 {
        self.offset_x
    }

    /// Port of the accessor `offsetY()`.
    pub const fn offset_y(&self) -> f32 {
        self.offset_y
    }

    /// Port of the accessor `offsetZ()`.
    pub const fn offset_z(&self) -> f32 {
        self.offset_z
    }

    /// Port of the accessor `red()`.
    pub const fn red(&self) -> f32 {
        self.red
    }

    /// Port of the accessor `green()`.
    pub const fn green(&self) -> f32 {
        self.green
    }

    /// Port of the accessor `blue()`.
    pub const fn blue(&self) -> f32 {
        self.blue
    }

    fn doubles(&self) -> [f64; 6] {
        [self.x0, self.y0, self.z0, self.x1, self.y1, self.z1]
    }

    fn floats(&self) -> [f32; 6] {
        [self.offset_x, self.offset_y, self.offset_z, self.red, self.green, self.blue]
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        let mut hashes = Vec::with_capacity(12);
        hashes.extend(self.doubles().map(java_object::hash_f64));
        hashes.extend(self.floats().map(java_object::hash_f32));
        java_object::record_hash(&hashes)
    }
}

impl PartialEq for HitboxRenderState {
    /// Port of the record's generated `equals`: `Double.compare` / `Float.compare` per component.
    fn eq(&self, other: &Self) -> bool {
        self.doubles().iter().zip(other.doubles().iter()).all(|(a, b)| java_object::equals_f64(*a, *b))
            && self.floats().iter().zip(other.floats().iter()).all(|(a, b)| java_object::equals_f32(*a, *b))
    }
}

impl Eq for HitboxRenderState {}

impl std::hash::Hash for HitboxRenderState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.java_hash_code().hash(state)
    }
}

impl std::fmt::Display for HitboxRenderState {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HitboxRenderState[x0={}, y0={}, z0={}, x1={}, y1={}, z1={}, offsetX={}, offsetY={}, offsetZ={}, red={}, green={}, blue={}]",
            double_to_string(self.x0),
            double_to_string(self.y0),
            double_to_string(self.z0),
            double_to_string(self.x1),
            double_to_string(self.y1),
            double_to_string(self.z1),
            float_to_string(self.offset_x),
            float_to_string(self.offset_y),
            float_to_string(self.offset_z),
            float_to_string(self.red),
            float_to_string(self.green),
            float_to_string(self.blue)
        )
    }
}
