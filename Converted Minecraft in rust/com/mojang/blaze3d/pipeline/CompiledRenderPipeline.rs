//! Port of: com/mojang/blaze3d/pipeline/CompiledRenderPipeline.java
//! Java class(es): com.mojang.blaze3d.pipeline.CompiledRenderPipeline
//! Status: VERIFIED
//!
//! Session 22. `public interface CompiledRenderPipeline` -- declaration only.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `com.mojang.blaze3d.pipeline.CompiledRenderPipeline`.
pub trait CompiledRenderPipeline {
    /// Port of `boolean isValid()`.
    fn is_valid(&self) -> bool;
}
