//! Port of: net/minecraft/network/SkipPacketException.java
//! Java class(es): net.minecraft.network.SkipPacketException
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A **marker interface** with no members: `net.minecraft` tests `instanceof SkipPacketException`
//! to decide whether a packet should be dropped without an error. There is no method to get
//! wrong and no state, so the faithful Rust shape is an empty trait that other types implement
//! to opt in.
//!
//! Not `dyn`-compatible by intent -- a Java marker interface can be tested against any
//! implementing type, and an empty Rust trait does the same job.

/// Port of `net.minecraft.network.SkipPacketException`.
///
/// Empty on purpose: the whole contract is "this type is one of these", which in Rust is
/// satisfied by implementing the trait at all.
pub trait SkipPacketException {}