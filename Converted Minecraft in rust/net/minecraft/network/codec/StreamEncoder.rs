//! Port of: net/minecraft/network/codec/StreamEncoder.java
//! Java class(es): net.minecraft.network.codec.StreamEncoder
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! The write half of [`StreamDecoder`](super::StreamDecoder): a `@FunctionalInterface` with one
//! method that writes a `T` into an output `O`. Taken together the two interfaces are how every
//! network packet field is described -- `StreamCodec.composite(...)` pairs a decoder and an
//! encoder per field.
//!
//! `netty::ByteBuf` is not ported, so `O` stays generic; see `StreamDecoder` for the same note.

/// Port of `net.minecraft.network.codec.StreamEncoder<B, O, T>`.
///
/// `O` is the output buffer, `T` the value to write.
pub trait StreamEncoder<O, T> {
    /// Port of `StreamEncoder#encode(O, T)`.
    fn encode(&self, output: &mut O, value: T);
}