//! Port of: net/minecraft/client/particle/ParticleRenderType.java
//! Java class(es): net.minecraft.client.particle.ParticleRenderType
//! Status: VERIFIED
//!
//! Session 22. `public record ParticleRenderType(String name, String shorthand)` with four
//! constants: `SINGLE_QUADS` ("SQ"), `ITEM_PICKUP` ("IP"), `ELDER_GUARDIANS` ("EG"),
//! `NO_RENDER` ("NR").
//!
//! A record, not an enum: the four are ordinary instances, and `equals` is by value -- a fresh
//! `new ParticleRenderType("SINGLE_QUADS", "SQ")` equals `SINGLE_QUADS` (the probe checks it).
//! `hashCode` is the record fold over the two strings' hashes; `toString` is the record form.
//!
//! Both strings are `Cow<'static, str>` so the constants are `const`. They are NON-NULL here,
//! because a Rust string cannot be null; Java would accept a null (printing `null`, hashing `0`).
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`).

use std::borrow::Cow;

use crate::javacompat::java_lang;
use crate::javacompat::java_object;

/// Port of `net.minecraft.client.particle.ParticleRenderType`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParticleRenderType {
    name: Cow<'static, str>,
    shorthand: Cow<'static, str>,
}

impl ParticleRenderType {
    /// Port of `ParticleRenderType.SINGLE_QUADS`.
    pub const SINGLE_QUADS: ParticleRenderType = ParticleRenderType::constant("SINGLE_QUADS", "SQ");
    /// Port of `ParticleRenderType.ITEM_PICKUP`.
    pub const ITEM_PICKUP: ParticleRenderType = ParticleRenderType::constant("ITEM_PICKUP", "IP");
    /// Port of `ParticleRenderType.ELDER_GUARDIANS`.
    pub const ELDER_GUARDIANS: ParticleRenderType = ParticleRenderType::constant("ELDER_GUARDIANS", "EG");
    /// Port of `ParticleRenderType.NO_RENDER`.
    pub const NO_RENDER: ParticleRenderType = ParticleRenderType::constant("NO_RENDER", "NR");

    const fn constant(name: &'static str, shorthand: &'static str) -> Self {
        ParticleRenderType {
            name: Cow::Borrowed(name),
            shorthand: Cow::Borrowed(shorthand),
        }
    }

    /// Port of the canonical constructor.
    pub fn new(name: &str, shorthand: &str) -> Self {
        ParticleRenderType {
            name: Cow::Owned(name.to_string()),
            shorthand: Cow::Owned(shorthand.to_string()),
        }
    }

    /// Port of the accessor `name()`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Port of the accessor `shorthand()`.
    pub fn shorthand(&self) -> &str {
        &self.shorthand
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[java_lang::string_hash_code(&self.name), java_lang::string_hash_code(&self.shorthand)])
    }
}

impl std::fmt::Display for ParticleRenderType {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParticleRenderType[name={}, shorthand={}]", self.name, self.shorthand)
    }
}
