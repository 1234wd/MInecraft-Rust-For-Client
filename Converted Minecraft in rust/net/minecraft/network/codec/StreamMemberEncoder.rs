//! Port of: net/minecraft/network/codec/StreamMemberEncoder.java
//! Java class(es): net.minecraft.network.codec.StreamMemberEncoder
//! Status: VERIFIED
//!
//! Session 22. `@FunctionalInterface public interface StreamMemberEncoder<O, T>` -- declaration only.
//!
//! Note the argument order: the VALUE first, then the output -- the reverse of `StreamEncoder`.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `net.minecraft.network.codec.StreamMemberEncoder`.
pub trait StreamMemberEncoder<O, T> {
    /// Port of `void encode(T value, O output)`.
    fn encode(&self, value: T, output: O);
}
