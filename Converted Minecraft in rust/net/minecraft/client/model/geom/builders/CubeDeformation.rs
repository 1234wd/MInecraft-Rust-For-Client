//! Port of: net/minecraft/client/model/geom/builders/CubeDeformation.java
//! Java class(es): net.minecraft.client.model.geom.builders.CubeDeformation
//! Status: VERIFIED
//!
//! Session 22. `public class CubeDeformation`: how far a model cube is inflated on each axis.
//! Three `final float` fields, a `NONE`, a uniform and a per-axis constructor, and two `extend`
//! methods that return a new deformation.
//!
//! The fields are PACKAGE-PRIVATE and read directly by `CubeDefinition`, so they are `pub` here.
//! `extend(f)` adds `f` to each axis and `extend(x, y, z)` adds per axis -- plain `float` adds, so
//! `MAX_VALUE + MAX_VALUE` is `Infinity` and `NONE.extend(-0.0F)` is `+0.0` (both probed). Per `DESIGN_DECISIONS.md#overload-naming` the two are [`CubeDeformation::extend_f32`] and
//! [`CubeDeformation::extend_f32_f32_f32`]; the constructors are [`CubeDeformation::new`] (per axis)
//! and [`CubeDeformation::uniform`].
//!
//! No `equals` override in Java (identity), so no `PartialEq` here.
//!
//! Verified by the `CUBEDEF` group of `small_probe.txt` (`tests/session22.rs`), which reads the
//! three fields back by reflection as raw bits.

/// Port of `net.minecraft.client.model.geom.builders.CubeDeformation`.
#[derive(Debug, Clone, Copy)]
pub struct CubeDeformation {
    /// Port of `final float growX` (package-private).
    pub grow_x: f32,
    /// Port of `final float growY` (package-private).
    pub grow_y: f32,
    /// Port of `final float growZ` (package-private).
    pub grow_z: f32,
}

impl CubeDeformation {
    /// Port of `CubeDeformation.NONE` = `new CubeDeformation(0.0F)`.
    pub const NONE: CubeDeformation = CubeDeformation::uniform(0.0);

    /// Port of `new CubeDeformation(float growX, float growY, float growZ)`.
    pub const fn new(grow_x: f32, grow_y: f32, grow_z: f32) -> Self {
        CubeDeformation {
            grow_x,
            grow_y,
            grow_z,
        }
    }

    /// Port of `new CubeDeformation(float grow)`: the same growth on every axis.
    pub const fn uniform(grow: f32) -> Self {
        CubeDeformation::new(grow, grow, grow)
    }

    /// Port of `extend(float factor)`.
    pub fn extend_f32(&self, factor: f32) -> CubeDeformation {
        CubeDeformation::new(
            self.grow_x + factor,
            self.grow_y + factor,
            self.grow_z + factor,
        )
    }

    /// Port of `extend(float factorX, float factorY, float factorZ)`.
    pub fn extend_f32_f32_f32(
        &self,
        factor_x: f32,
        factor_y: f32,
        factor_z: f32,
    ) -> CubeDeformation {
        CubeDeformation::new(
            self.grow_x + factor_x,
            self.grow_y + factor_y,
            self.grow_z + factor_z,
        )
    }
}
