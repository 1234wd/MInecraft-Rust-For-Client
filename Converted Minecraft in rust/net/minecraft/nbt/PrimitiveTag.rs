//! Port of: net/minecraft/nbt/PrimitiveTag.java
//! Java class(es): net.minecraft.nbt.PrimitiveTag
//! Status: PORTED
//!
//! Session 21. The whole interface, which is one `default` method:
//!
//! ```java
//! public sealed interface PrimitiveTag extends Tag permits NumericTag, StringTag {
//!     @Override
//!     default Tag copy() { return this; }
//! }
//! ```
//!
//! # WHY THIS TRAIT HAS NO METHODS
//!
//! `copy()` returning `this` is the statement "a primitive tag is immutable, so a copy is
//! itself". In the Rust model that is not a method at all -- [`Tag`](crate::net::minecraft::nbt::Tag)
//! is an enum whose `copy()` is `clone()`, and cloning a `ByteTag` already copies nine bytes
//! of stack rather than aliasing anything. There is nothing left to override.
//!
//! So this is a marker trait, implemented by the six numeric tags and `StringTag`, and it
//! earns its place by being the thing that *fails to compile* if a future container tag is
//! wired into the primitive group by mistake. It is `permits NumericTag, StringTag` written
//! in a form the compiler checks.
//!
//! Vanilla's own grouping is load-bearing elsewhere for the same reason: `Tag`'s `permits`
//! clause lists `PrimitiveTag` rather than the seven concrete kinds.

/// Port of the `sealed interface net.minecraft.nbt.PrimitiveTag`.
///
/// A marker. `copy()` lives on [`Tag`](crate::net::minecraft::nbt::Tag) -- see the module
/// docs.
pub trait PrimitiveTag {}
