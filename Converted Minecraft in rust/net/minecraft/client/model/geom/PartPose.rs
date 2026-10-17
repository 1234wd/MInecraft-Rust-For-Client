//! Port of: net/minecraft/client/model/geom/PartPose.java
//! Java class(es): net.minecraft.client.model.geom.PartPose
//! Status: VERIFIED
//!
//! Session 22. `public record PartPose(float x, float y, float z, float xRot, float yRot,
//! float zRot, float xScale, float yScale, float zScale)` -- a model part's offset, rotation and
//! scale -- with a `ZERO`, three factories and four derivers.
//!
//! # Arithmetic
//!
//! Every deriver is plain `float` arithmetic, one IEEE operation per component, so `+` and `*` on
//! `f32` here give the same bits Java does. A NaN result's payload is not observable through any
//! member of this record (`toString` prints `NaN`, `equals` and `hashCode` canonicalise), so the
//! NaN-sign care of `javacompat::java_lang::add_f32` is not needed.
//!
//! `scaled(float factor)` returns `this` unchanged when `factor == 1.0F` -- a float `==`, so
//! `-0.0`, `NaN` and everything else go through `scaled(factor, factor, factor)`. Here "unchanged"
//! is an equal copy.
//!
//! # Names
//!
//! `scaled` is overloaded, so per `DESIGN_DECISIONS.md#overload-naming` the two are
//! [`PartPose::scaled_f32`] and [`PartPose::scaled_f32_f32_f32`].
//!
//! # Record members
//!
//! `equals` compares all nine floats with `Float.compare` (hand-written on
//! [`java_object::equals_f32`]); `hashCode` is the record fold over `Float.hashCode`; `toString`
//! prints each with Java's `Float.toString`.
//!
//! Verified by the `REC`, `RECEQ`, `PPOSE` and `PPOSEMAKE` groups of `small_probe.txt`
//! (`tests/session22.rs`): seven poses (including `-0.0`, NaN, a subnormal, `MAX_VALUE` and
//! infinities) through every deriver with eight factors.

use crate::javacompat::java_lang::float_to_string::float_to_string;
use crate::javacompat::java_object;

/// Port of `net.minecraft.client.model.geom.PartPose`.
#[derive(Debug, Clone, Copy)]
pub struct PartPose {
    x: f32,
    y: f32,
    z: f32,
    x_rot: f32,
    y_rot: f32,
    z_rot: f32,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
}

impl PartPose {
    /// Port of `PartPose.ZERO` = `offsetAndRotation(0, 0, 0, 0, 0, 0)`.
    pub const ZERO: PartPose = PartPose::offset_and_rotation(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    /// Port of the canonical constructor.
    #[allow(clippy::too_many_arguments)] // Nine components is the shape of the record.
    pub const fn new(
        x: f32,
        y: f32,
        z: f32,
        x_rot: f32,
        y_rot: f32,
        z_rot: f32,
        x_scale: f32,
        y_scale: f32,
        z_scale: f32,
    ) -> Self {
        PartPose {
            x,
            y,
            z,
            x_rot,
            y_rot,
            z_rot,
            x_scale,
            y_scale,
            z_scale,
        }
    }

    /// Port of the accessor `x()`.
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Port of the accessor `y()`.
    pub const fn y(&self) -> f32 {
        self.y
    }

    /// Port of the accessor `z()`.
    pub const fn z(&self) -> f32 {
        self.z
    }

    /// Port of the accessor `xRot()`.
    pub const fn x_rot(&self) -> f32 {
        self.x_rot
    }

    /// Port of the accessor `yRot()`.
    pub const fn y_rot(&self) -> f32 {
        self.y_rot
    }

    /// Port of the accessor `zRot()`.
    pub const fn z_rot(&self) -> f32 {
        self.z_rot
    }

    /// Port of the accessor `xScale()`.
    pub const fn x_scale(&self) -> f32 {
        self.x_scale
    }

    /// Port of the accessor `yScale()`.
    pub const fn y_scale(&self) -> f32 {
        self.y_scale
    }

    /// Port of the accessor `zScale()`.
    pub const fn z_scale(&self) -> f32 {
        self.z_scale
    }

    /// Port of `offset(float x, float y, float z)`: no rotation, unit scale.
    pub const fn offset(x: f32, y: f32, z: f32) -> PartPose {
        Self::offset_and_rotation(x, y, z, 0.0, 0.0, 0.0)
    }

    /// Port of `rotation(float x, float y, float z)`: no offset, unit scale.
    pub const fn rotation(x: f32, y: f32, z: f32) -> PartPose {
        Self::offset_and_rotation(0.0, 0.0, 0.0, x, y, z)
    }

    /// Port of `offsetAndRotation(offsetX, offsetY, offsetZ, rotationX, rotationY, rotationZ)`.
    pub const fn offset_and_rotation(
        offset_x: f32,
        offset_y: f32,
        offset_z: f32,
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
    ) -> PartPose {
        PartPose::new(offset_x, offset_y, offset_z, rotation_x, rotation_y, rotation_z, 1.0, 1.0, 1.0)
    }

    /// Port of `translated(float x, float y, float z)`: adds to the offset only.
    pub fn translated(&self, x: f32, y: f32, z: f32) -> PartPose {
        PartPose::new(
            self.x + x,
            self.y + y,
            self.z + z,
            self.x_rot,
            self.y_rot,
            self.z_rot,
            self.x_scale,
            self.y_scale,
            self.z_scale,
        )
    }

    /// Port of `withScale(float scale)`: replaces all three scales.
    pub fn with_scale(&self, scale: f32) -> PartPose {
        PartPose::new(self.x, self.y, self.z, self.x_rot, self.y_rot, self.z_rot, scale, scale, scale)
    }

    /// Port of `scaled(float factor)`: `factor == 1.0F ? this : scaled(factor, factor, factor)`.
    pub fn scaled_f32(&self, factor: f32) -> PartPose {
        if factor == 1.0 {
            *self
        } else {
            self.scaled_f32_f32_f32(factor, factor, factor)
        }
    }

    /// Port of `scaled(float scaleX, float scaleY, float scaleZ)`: scales the offset AND the scale,
    /// not the rotation.
    pub fn scaled_f32_f32_f32(&self, scale_x: f32, scale_y: f32, scale_z: f32) -> PartPose {
        PartPose::new(
            self.x * scale_x,
            self.y * scale_y,
            self.z * scale_z,
            self.x_rot,
            self.y_rot,
            self.z_rot,
            self.x_scale * scale_x,
            self.y_scale * scale_y,
            self.z_scale * scale_z,
        )
    }

    fn components(&self) -> [f32; 9] {
        [
            self.x,
            self.y,
            self.z,
            self.x_rot,
            self.y_rot,
            self.z_rot,
            self.x_scale,
            self.y_scale,
            self.z_scale,
        ]
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&self.components().map(java_object::hash_f32))
    }
}

impl PartialEq for PartPose {
    /// Port of the record's generated `equals`: `Float.compare` on all nine components.
    fn eq(&self, other: &Self) -> bool {
        self.components()
            .iter()
            .zip(other.components().iter())
            .all(|(a, b)| java_object::equals_f32(*a, *b))
    }
}

impl Eq for PartPose {}

impl std::hash::Hash for PartPose {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.java_hash_code().hash(state)
    }
}

impl std::fmt::Display for PartPose {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PartPose[x={}, y={}, z={}, xRot={}, yRot={}, zRot={}, xScale={}, yScale={}, zScale={}]",
            float_to_string(self.x),
            float_to_string(self.y),
            float_to_string(self.z),
            float_to_string(self.x_rot),
            float_to_string(self.y_rot),
            float_to_string(self.z_rot),
            float_to_string(self.x_scale),
            float_to_string(self.y_scale),
            float_to_string(self.z_scale)
        )
    }
}
