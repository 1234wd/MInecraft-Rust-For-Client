//! Port of: com/mojang/blaze3d/vulkan/glsl/SpvVariable.java
//! Java class(es): com.mojang.blaze3d.vulkan.glsl.SpvVariable
//! Status: VERIFIED
//!
//! Session 22. `record SpvVariable(String name, int locationOffset)` -- components only, no body.
//!
//! The Java record is PACKAGE-PRIVATE (no `public`). The port is `pub` so the parity tests, which
//! are a separate crate, can reach it; nothing outside `vulkan/glsl` should.
//!
//! # What the record generates, ported explicitly
//!
//! * the canonical constructor and one accessor per component;
//! * `equals` -- component-wise, which `#[derive(PartialEq, Eq)]` is for these component types;
//! * `hashCode` -- [`SpvVariable::java_hash_code`], the record fold `acc * 31 + h` from `0` over the
//!   components' boxed hashes (see [`crate::javacompat::java_object`]);
//! * `toString` -- the `Display` impl, `SpvVariable[name=..., locationOffset=...]`.
//!
//! `String` components are NON-NULL in this port, because a Rust `String` cannot be null. Java
//! would accept one (printing `null`, hashing `0`); a caller that passes null would need this
//! component to become `Option<String>`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`):
//! `toString`, `hashCode` and the full `equals` matrix, over five to nine instances per record that
//! include an equal-but-distinct duplicate and the components' extreme values.

use crate::javacompat::java_lang;
use crate::javacompat::java_object;

/// Port of `com.mojang.blaze3d.vulkan.glsl.SpvVariable`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpvVariable {
    name: String,
    location_offset: i32,
}

impl SpvVariable {
    /// Port of the canonical constructor.
    pub fn new(name: &str, location_offset: i32) -> Self {
        SpvVariable {
            name: name.to_string(),
            location_offset,
        }
    }

    /// Port of the accessor `name()`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Port of the accessor `locationOffset()`.
    pub fn location_offset(&self) -> i32 {
        self.location_offset
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            java_lang::string_hash_code(&self.name),
            self.location_offset,
        ])
    }
}

impl std::fmt::Display for SpvVariable {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpvVariable[name={}, locationOffset={}]", self.name, self.location_offset)
    }
}
