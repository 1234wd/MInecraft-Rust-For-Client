//! Port of: net/minecraft/SuppressForbidden.java
//! Java class(es): net.minecraft.SuppressForbidden
//! Status: PORTED
//!
//! An annotation with a single element: `@interface SuppressForbidden { String reason(); }`,
//! `@Retention(CLASS)`, `@Target({CONSTRUCTOR, FIELD, METHOD, TYPE})`. Modeled as a trait
//! with a `reason()` accessor. Java annotations are retrieved reflectively; Rust has no
//! reflection, so the trait preserves the declared API shape for callers that pattern-match
//! on it.
//!
//! No oracle-testable behavior: the annotation's `reason()` value is supplied at the use
//! site and read reflectively by consumers, neither of which is reproduced here.

/// Port of `net.minecraft.SuppressForbidden`.
///
/// An annotation with a single `String reason()` element. The trait exposes `reason()`
/// so callers can pattern-match on it; the actual string value is attached at the use
/// site in Java and is not part of this declaration.
pub trait SuppressForbidden {
    /// Port of `String reason()`.
    fn reason(&self) -> &str;
}
