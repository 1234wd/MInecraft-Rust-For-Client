//! Port of: net/minecraft/network/codec/StreamDecoder.java
//! Java class(es): net.minecraft.network.codec.StreamDecoder
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A `@FunctionalInterface` with one method: read a `T` out of an input `I`. It is the read
//! half of every network codec, and the reason the interface is small is that the whole point
//! is to be written as a lambda at each call site.
//!
//! # `ByteBuf` IS NOT PORTED, SO THE TYPE PARAMETERS ARE FREE
//!
//! Java fixes `I` to `ByteBuf` at most call sites. Here both parameters stay generic, which is
//! the honest translation and costs nothing: `netty::ByteBuf` is not ported, so pinning `I` to
//! it would make this trait unusable rather than more faithful. When netty arrives, `I` will be
//! `ByteBuf` in practice without needing a change here.

/// Port of `net.minecraft.network.codec.StreamDecoder<B, I, T>`.
///
/// `I` is the input buffer, `T` the decoded value.
pub trait StreamDecoder<I, T> {
    /// Port of `StreamDecoder#decode(I)`.
    fn decode(&self, input: &I) -> T;
}