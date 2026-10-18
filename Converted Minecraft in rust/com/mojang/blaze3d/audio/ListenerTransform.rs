//! Port of: com/mojang/blaze3d/audio/ListenerTransform.java
//! Java class(es): com.mojang.blaze3d.audio.ListenerTransform
//! Status: VERIFIED
//!
//! Session 22. `public record ListenerTransform(Vec3 position, Vec3 forward, Vec3 up)`: where the
//! sound listener stands and which way it faces, with one constant and one derived vector.
//!
//! * `INITIAL` is at the origin, facing `-Z`, with `+Y` up.
//! * `right()` is `forward.cross(up)`: [`Vec3::cross`], unnormalised, in `double`. For `INITIAL`
//!   it is `(1.0, -0.0, 0.0)` -- the `-0.0` is `(-1.0) * 0.0 - 0.0 * 0.0`, and the jar agrees.
//!
//! # Record members
//!
//! `equals` is the record's, component by component through `Vec3.equals` -- which compares each
//! axis with `Double.compare`, so a NaN position equals a NaN position and `-0.0` does not equal
//! `0.0`. That is `Vec3`'s `PartialEq`, so it is derived here. `hashCode` is the record fold (from
//! 0) over `Vec3.hashCode`, which canonicalises NaN and is stable across runs.
//! `Vec3` has no `Hash`, so neither does this. `toString` is
//! `ListenerTransform[position=(x, y, z), forward=(..), up=(..)]`, through Java's `Double.toString`.
//!
//! Verified by the `REC`, `RECEQ` and `LISTENER` groups of `small_probe.txt`
//! (`tests/session22.rs`): `INITIAL`, an equal fresh copy, a `-0.0` position, NaN, infinite,
//! huge and subnormal-adjacent components, and `right()` of each.

use crate::javacompat::java_object;
use crate::net::minecraft::world::phys::Vec3::Vec3;

/// Port of `com.mojang.blaze3d.audio.ListenerTransform`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ListenerTransform {
    position: Vec3,
    forward: Vec3,
    up: Vec3,
}

impl ListenerTransform {
    /// Port of `ListenerTransform.INITIAL`.
    pub const INITIAL: ListenerTransform = ListenerTransform::new(
        Vec3::ZERO,
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    /// Port of the canonical constructor.
    pub const fn new(position: Vec3, forward: Vec3, up: Vec3) -> Self {
        ListenerTransform {
            position,
            forward,
            up,
        }
    }

    /// Port of the accessor `position()`.
    pub const fn position(&self) -> Vec3 {
        self.position
    }

    /// Port of the accessor `forward()`.
    pub const fn forward(&self) -> Vec3 {
        self.forward
    }

    /// Port of the accessor `up()`.
    pub const fn up(&self) -> Vec3 {
        self.up
    }

    /// Port of `right()`: `this.forward.cross(this.up)`.
    pub fn right(&self) -> Vec3 {
        self.forward.cross(&self.up)
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            self.position.java_hash_code(),
            self.forward.java_hash_code(),
            self.up.java_hash_code(),
        ])
    }
}

impl std::fmt::Display for ListenerTransform {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ListenerTransform[position={}, forward={}, up={}]",
            self.position, self.forward, self.up
        )
    }
}
