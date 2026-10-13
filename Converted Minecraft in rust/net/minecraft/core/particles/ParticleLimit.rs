//! Port of: net/minecraft/core/particles/ParticleLimit.java
//! Java class(es): net.minecraft.core.particles.ParticleLimit
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A Java `record` with one `int` component and one static constant. The record gives it
//! `equals`/`hashCode`/`toString` for free, which is why it is a record rather than a class --
//! so this is a plain struct with `Copy`, and the derived traits are the port of what the Java
//! record generates.
//!
//! # `SPORE_BLOSSOM` IS NOT THE ONLY CONSTANT IN THE REAL CLASS
//!
//! The decompiled source shows one, and that is what is ported. If later versions add more,
//! they belong here as sibling `const`s; nothing about the shape changes.

/// Port of `net.minecraft.core.particles.ParticleLimit`.
///
/// Java `record ParticleLimit(int limit)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ParticleLimit {
    /// Port of the record component `limit`.
    pub limit: i32,
}

impl ParticleLimit {
    /// Port of `ParticleLimit#SPORE_BLOSSOM` = `new ParticleLimit(1000)`.
    pub const SPORE_BLOSSOM: ParticleLimit = ParticleLimit { limit: 1000 };
}