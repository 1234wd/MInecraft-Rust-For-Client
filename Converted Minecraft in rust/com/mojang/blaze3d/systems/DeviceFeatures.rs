//! Port of: com/mojang/blaze3d/systems/DeviceFeatures.java
//! Java class(es): com.mojang.blaze3d.systems.DeviceFeatures
//! Status: VERIFIED
//!
//! Session 22. `public record DeviceFeatures(boolean shaderDrawParameters, boolean multiDrawDirectInterleaved, boolean multiDrawDirectSeparate, boolean multiDrawIndirect, boolean drawIndirect, boolean nonZeroFirstInstance, boolean persistentMapping)` -- components only, no body.
//!
//! # What the record generates, ported explicitly
//!
//! * the canonical constructor and one accessor per component;
//! * `equals` -- component-wise, which `#[derive(PartialEq, Eq)]` is for these component types;
//! * `hashCode` -- [`DeviceFeatures::java_hash_code`], the record fold `acc * 31 + h` from `0` over the
//!   components' boxed hashes (see [`crate::javacompat::java_object`]);
//! * `toString` -- the `Display` impl, `DeviceFeatures[shaderDrawParameters=..., multiDrawDirectInterleaved=..., multiDrawDirectSeparate=..., multiDrawIndirect=..., drawIndirect=..., nonZeroFirstInstance=..., persistentMapping=...]`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`):
//! `toString`, `hashCode` and the full `equals` matrix, over five to nine instances per record that
//! include an equal-but-distinct duplicate and the components' extreme values.

use crate::javacompat::java_object;

/// Port of `com.mojang.blaze3d.systems.DeviceFeatures`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceFeatures {
    shader_draw_parameters: bool,
    multi_draw_direct_interleaved: bool,
    multi_draw_direct_separate: bool,
    multi_draw_indirect: bool,
    draw_indirect: bool,
    non_zero_first_instance: bool,
    persistent_mapping: bool,
}

impl DeviceFeatures {
    /// Port of the canonical constructor.
    pub fn new(shader_draw_parameters: bool, multi_draw_direct_interleaved: bool, multi_draw_direct_separate: bool, multi_draw_indirect: bool, draw_indirect: bool, non_zero_first_instance: bool, persistent_mapping: bool) -> Self {
        DeviceFeatures {
            shader_draw_parameters,
            multi_draw_direct_interleaved,
            multi_draw_direct_separate,
            multi_draw_indirect,
            draw_indirect,
            non_zero_first_instance,
            persistent_mapping,
        }
    }

    /// Port of the accessor `shaderDrawParameters()`.
    pub fn shader_draw_parameters(&self) -> bool {
        self.shader_draw_parameters
    }

    /// Port of the accessor `multiDrawDirectInterleaved()`.
    pub fn multi_draw_direct_interleaved(&self) -> bool {
        self.multi_draw_direct_interleaved
    }

    /// Port of the accessor `multiDrawDirectSeparate()`.
    pub fn multi_draw_direct_separate(&self) -> bool {
        self.multi_draw_direct_separate
    }

    /// Port of the accessor `multiDrawIndirect()`.
    pub fn multi_draw_indirect(&self) -> bool {
        self.multi_draw_indirect
    }

    /// Port of the accessor `drawIndirect()`.
    pub fn draw_indirect(&self) -> bool {
        self.draw_indirect
    }

    /// Port of the accessor `nonZeroFirstInstance()`.
    pub fn non_zero_first_instance(&self) -> bool {
        self.non_zero_first_instance
    }

    /// Port of the accessor `persistentMapping()`.
    pub fn persistent_mapping(&self) -> bool {
        self.persistent_mapping
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            java_object::hash_bool(self.shader_draw_parameters),
            java_object::hash_bool(self.multi_draw_direct_interleaved),
            java_object::hash_bool(self.multi_draw_direct_separate),
            java_object::hash_bool(self.multi_draw_indirect),
            java_object::hash_bool(self.draw_indirect),
            java_object::hash_bool(self.non_zero_first_instance),
            java_object::hash_bool(self.persistent_mapping),
        ])
    }
}

impl std::fmt::Display for DeviceFeatures {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceFeatures[shaderDrawParameters={}, multiDrawDirectInterleaved={}, multiDrawDirectSeparate={}, multiDrawIndirect={}, drawIndirect={}, nonZeroFirstInstance={}, persistentMapping={}]", self.shader_draw_parameters, self.multi_draw_direct_interleaved, self.multi_draw_direct_separate, self.multi_draw_indirect, self.draw_indirect, self.non_zero_first_instance, self.persistent_mapping)
    }
}
