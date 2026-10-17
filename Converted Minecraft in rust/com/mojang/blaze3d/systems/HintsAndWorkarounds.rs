//! Port of: com/mojang/blaze3d/systems/HintsAndWorkarounds.java
//! Java class(es): com.mojang.blaze3d.systems.HintsAndWorkarounds
//! Status: VERIFIED
//!
//! Session 22. `public record HintsAndWorkarounds(boolean writeToBufferIsSlow, boolean anisotropyHasKnownIssues)` -- components only, no body.
//!
//! # What the record generates, ported explicitly
//!
//! * the canonical constructor and one accessor per component;
//! * `equals` -- component-wise, which `#[derive(PartialEq, Eq)]` is for these component types;
//! * `hashCode` -- [`HintsAndWorkarounds::java_hash_code`], the record fold `acc * 31 + h` from `0` over the
//!   components' boxed hashes (see [`crate::javacompat::java_object`]);
//! * `toString` -- the `Display` impl, `HintsAndWorkarounds[writeToBufferIsSlow=..., anisotropyHasKnownIssues=...]`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`):
//! `toString`, `hashCode` and the full `equals` matrix, over five to nine instances per record that
//! include an equal-but-distinct duplicate and the components' extreme values.

use crate::javacompat::java_object;

/// Port of `com.mojang.blaze3d.systems.HintsAndWorkarounds`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HintsAndWorkarounds {
    write_to_buffer_is_slow: bool,
    anisotropy_has_known_issues: bool,
}

impl HintsAndWorkarounds {
    /// Port of the canonical constructor.
    pub fn new(write_to_buffer_is_slow: bool, anisotropy_has_known_issues: bool) -> Self {
        HintsAndWorkarounds {
            write_to_buffer_is_slow,
            anisotropy_has_known_issues,
        }
    }

    /// Port of the accessor `writeToBufferIsSlow()`.
    pub fn write_to_buffer_is_slow(&self) -> bool {
        self.write_to_buffer_is_slow
    }

    /// Port of the accessor `anisotropyHasKnownIssues()`.
    pub fn anisotropy_has_known_issues(&self) -> bool {
        self.anisotropy_has_known_issues
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            java_object::hash_bool(self.write_to_buffer_is_slow),
            java_object::hash_bool(self.anisotropy_has_known_issues),
        ])
    }
}

impl std::fmt::Display for HintsAndWorkarounds {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HintsAndWorkarounds[writeToBufferIsSlow={}, anisotropyHasKnownIssues={}]", self.write_to_buffer_is_slow, self.anisotropy_has_known_issues)
    }
}
