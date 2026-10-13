//! Port of: net/minecraft/util/VisibleForDebug.java
//! Java class(es): net.minecraft.util.VisibleForDebug
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A marker **annotation** (`@interface` with no members), i.e. a documentation-only flag that
//! the DevTools debugger reads to decide whether a field is shown. It affects tooling, not
//! behaviour, and has no runtime representation in Java either -- the annotation has no
//! `@Retention(RUNTIME)`, so it is not even visible to reflection.
//!
//! Rust has no equivalent, so this is a documentation marker with no type. The port is the
//! module comment plus this note; there is deliberately no empty `struct`, because an empty
//! struct would imply a value users are meant to construct, which the Java does not offer.

/// Port of the `@VisibleForDebug` annotation.
///
/// Nothing to port at the type level: see the module docs. Kept as a named item so that a
/// future port which *does* need to record "this was annotated for debugging" has an obvious
/// home, rather than inventing one inline.
#[allow(dead_code)]
pub const VISIBLE_FOR_DEBUG: () = ();