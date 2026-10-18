//! Port of: net/minecraft/client/resources/metadata/animation/FrameSize.java
//! Java class(es): net.minecraft.client.resources.metadata.animation.FrameSize
//! Status: VERIFIED
//!
//! Session 22. `public record FrameSize(int width, int height)` -- components only, no body.
//!
//! # What the record generates, ported explicitly
//!
//! * the canonical constructor and one accessor per component;
//! * `equals` -- component-wise, which `#[derive(PartialEq, Eq)]` is for these component types;
//! * `hashCode` -- [`FrameSize::java_hash_code`], the record fold `acc * 31 + h` from `0` over the
//!   components' boxed hashes (see [`crate::javacompat::java_object`]);
//! * `toString` -- the `Display` impl, `FrameSize[width=..., height=...]`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`):
//! `toString`, `hashCode` and the full `equals` matrix, over five to nine instances per record that
//! include an equal-but-distinct duplicate and the components' extreme values.

use crate::javacompat::java_object;

/// Port of `net.minecraft.client.resources.metadata.animation.FrameSize`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameSize {
    width: i32,
    height: i32,
}

impl FrameSize {
    /// Port of the canonical constructor.
    pub fn new(width: i32, height: i32) -> Self {
        FrameSize { width, height }
    }

    /// Port of the accessor `width()`.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Port of the accessor `height()`.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[self.width, self.height])
    }
}

impl std::fmt::Display for FrameSize {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FrameSize[width={}, height={}]", self.width, self.height)
    }
}
