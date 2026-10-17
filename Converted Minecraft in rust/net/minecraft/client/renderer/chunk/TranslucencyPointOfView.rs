//! Port of: net/minecraft/client/renderer/chunk/TranslucencyPointOfView.java
//! Java class(es): net.minecraft.client.renderer.chunk.TranslucencyPointOfView
//! Status: VERIFIED
//!
//! Session 22. `public final class TranslucencyPointOfView`: where the camera sits relative to a
//! chunk section, as `-1`/`0`/`1` per axis. `SectionRenderDispatcher` stores one with each compiled
//! section mesh, and its translucency re-sort task is cancelled when the point of view is unchanged
//! (`CompiledSectionMesh.isDifferentPointOfView`) and not axis-aligned.
//!
//! # Per axis
//!
//! `getCoordinate(camera, section)` is `Mth.clamp(SectionPos.blockToSectionCoord(camera) - section,
//! -1, 1)`: the camera's section coordinate (`floor(camera) >> 4`) minus the section's, in `int`
//! arithmetic (it can wrap for extreme inputs before the clamp), clamped to one section either side.
//! `set` reads the section coordinates out of a packed `SectionPos` long.
//!
//! # `equals` / `hashCode`
//!
//! Overridden: equal when all three components are equal (after the same-class `instanceof`), and
//! `hashCode` is `Objects.hash(x, y, z)` -- the fold from ONE, so `(0, 0, 0)` hashes to `29791`,
//! not `0`. `javacompat::java_object::objects_hash`.
//!
//! The class is mutable (`set` returns `this` for chaining); a default-constructed one is
//! `(0, 0, 0)`.
//!
//! Verified by the `TRANSPOV`, `TRANSPOVNEW` and `TRANSPOVEQ` groups of `small_probe.txt`
//! (`tests/session22.rs`): 44 camera/section pairs (signed zeros, section boundaries, `±1e9`),
//! the three components read by reflection, `isAxisAligned`, `hashCode` and an `equals` matrix.

use crate::javacompat::java_object;
use crate::net::minecraft::core::SectionPos::SectionPos;
use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::world::phys::Vec3::Vec3;

/// Port of `net.minecraft.client.renderer.chunk.TranslucencyPointOfView`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TranslucencyPointOfView {
    x: i32,
    y: i32,
    z: i32,
}

impl TranslucencyPointOfView {
    /// Port of the implicit constructor: `(0, 0, 0)`.
    pub const fn new() -> Self {
        TranslucencyPointOfView { x: 0, y: 0, z: 0 }
    }

    /// Port of `of(Vec3 cameraPos, long sectionNode)`.
    pub fn of(camera_pos: &Vec3, section_node: i64) -> TranslucencyPointOfView {
        let mut p = TranslucencyPointOfView::new();
        p.set(camera_pos, section_node);
        p
    }

    /// Port of `set(Vec3 cameraPos, long sectionPos)`: returns `this`.
    pub fn set(&mut self, camera_pos: &Vec3, section_pos: i64) -> &mut Self {
        self.x = Self::get_coordinate(camera_pos.x, SectionPos::x(section_pos));
        self.y = Self::get_coordinate(camera_pos.y, SectionPos::y(section_pos));
        self.z = Self::get_coordinate(camera_pos.z, SectionPos::z(section_pos));
        self
    }

    /// Port of `private static int getCoordinate(double cameraCoordinate, int section)`.
    fn get_coordinate(camera_coordinate: f64, section: i32) -> i32 {
        let relative_section = SectionPos::block_to_section_coord_f64(camera_coordinate).wrapping_sub(section);
        Mth::clamp_i32(relative_section, -1, 1)
    }

    /// Port of `isAxisAligned()`.
    pub const fn is_axis_aligned(&self) -> bool {
        self.x == 0 || self.y == 0 || self.z == 0
    }

    /// Port of the `hashCode()` override: `Objects.hash(this.x, this.y, this.z)`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::objects_hash(&[self.x, self.y, self.z])
    }
}
