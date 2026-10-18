//! Port of: net/minecraft/util/context/ContextKey.java
//! Java class(es): net.minecraft.util.context.ContextKey
//! Status: VERIFIED
//!
//! Session 22. `public class ContextKey<T>`: a typed key into a `ContextMap` (loot and predicate
//! contexts), named by an `Identifier`.
//!
//! # Identity, not value
//!
//! No `equals`/`hashCode` override, so two keys with the same name are DIFFERENT keys in Java
//! (`ContextKey.vanilla("origin").equals(ContextKey.vanilla("origin"))` is `false`; probed). The
//! port therefore derives no `PartialEq`: compare keys with `std::ptr::eq`, as Java's `==` does.
//! `T` only types the value the key looks up, so it is a phantom.
//!
//! `toString()` is `"<parameter " + name + ">"`.
//!
//! Verified by the `CTXKEY` group of `small_probe.txt` (`tests/session22.rs`).

use std::marker::PhantomData;

use crate::net::minecraft::resources::Identifier::{self as identifier, Identifier};

/// Port of `net.minecraft.util.context.ContextKey<T>`.
pub struct ContextKey<T> {
    name: Identifier,
    _type: PhantomData<fn() -> T>,
}

impl<T> ContextKey<T> {
    /// Port of `new ContextKey<>(Identifier name)`.
    pub fn new(name: Identifier) -> Self {
        ContextKey {
            name,
            _type: PhantomData,
        }
    }

    /// Port of `ContextKey.vanilla(String name)`: `new ContextKey<>(Identifier.withDefaultNamespace(name))`.
    pub fn vanilla(name: &str) -> ContextKey<T> {
        ContextKey::new(identifier::with_default_namespace(name))
    }

    /// Port of `name()`.
    pub fn name(&self) -> &Identifier {
        &self.name
    }
}

impl<T> std::fmt::Debug for ContextKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextKey")
            .field("name", &self.name)
            .finish()
    }
}

impl<T> std::fmt::Display for ContextKey<T> {
    /// Port of `toString()`: `"<parameter " + this.name + ">"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<parameter {}>", self.name)
    }
}
