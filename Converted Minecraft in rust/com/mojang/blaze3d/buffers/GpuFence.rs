//! Port of: com/mojang/blaze3d/buffers/GpuFence.java
//! Java class(es): com.mojang.blaze3d.buffers.GpuFence
//! Status: VERIFIED
//!
//! Session 22. `public interface GpuFence extends AutoCloseable` -- declaration only.
//!
//! `AutoCloseable` has no Rust counterpart here; its `close()` is a method of this trait.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `com.mojang.blaze3d.buffers.GpuFence`.
pub trait GpuFence {
    /// Port of `void close()` (redeclared from `AutoCloseable` without its `throws Exception`).
    fn close(&mut self);

    /// Port of `boolean awaitCompletion(long timeoutNS)`.
    fn await_completion(&self, timeout_ns: i64) -> bool;
}
