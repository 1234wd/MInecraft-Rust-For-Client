//! Port of: com/mojang/realmsclient/dto/Exclude.java
//! Java class(es): com.mojang.realmsclient.dto.Exclude
//! Status: PORTED
//!
//! A marker annotation: `@interface Exclude {}` with no methods, `@Retention(RUNTIME)`,
//! `@Target(FIELD)`. It carries no data and declares no methods, so it is modeled as an
//! empty marker trait. Nothing to oracle-test: annotations have no runtime behavior
//! beyond presence, and this port preserves the declared API shape.

/// Port of `com.mojang.realmsclient.dto.Exclude`.
///
/// An empty marker annotation — no methods, no data. `#[Retention(RUNTIME)]` and
/// `#[Target(FIELD)]` are Java meta-annotations with no Rust equivalent; the trait
/// itself preserves the declaration.
pub trait Exclude {}
