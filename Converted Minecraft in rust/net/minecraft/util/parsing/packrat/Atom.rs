//! Port of: net/minecraft/util/parsing/packrat/Atom.java
//! Java class(es): net.minecraft.util.parsing.packrat.Atom
//! Status: VERIFIED
//!
//! Session 22. `public record Atom<T>(String name)`: a typed name for a packrat-parser rule, with
//! an `of` factory and an overridden `toString`.
//!
//! # The type parameter is phantom
//!
//! `T` appears in no component -- it only types the rule the atom names -- so it takes no part in
//! `equals` or `hashCode`: Java's erased `Atom<Integer>("a")` equals `Atom<String>("a")`. Here `T`
//! is a `PhantomData<fn() -> T>`, so an `Atom<T>` is `Clone`/`Eq`/`Hash` whatever `T` is, and the
//! trait impls are written by hand to avoid derive's `T: Trait` bounds. Comparing two DIFFERENT
//! `T`s is a type error in Rust; the probe compares across `T`s only to show the name alone decides.
//!
//! `toString` is OVERRIDDEN: `"<" + name + ">"`. `hashCode` is the record's, which for one
//! component is `name.hashCode()`.
//!
//! The `name` is NON-NULL here, because a Rust `String` cannot be null. Java would accept one
//! (printing `<null>`, hashing `0`); a caller that passes null would need `Option<String>`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`), including
//! an empty name and a non-ASCII one.

use std::marker::PhantomData;

use crate::javacompat::java_lang;

/// Port of `net.minecraft.util.parsing.packrat.Atom<T>`.
pub struct Atom<T> {
    name: String,
    _type: PhantomData<fn() -> T>,
}

impl<T> Atom<T> {
    /// Port of the canonical constructor.
    pub fn new(name: &str) -> Self {
        Atom {
            name: name.to_string(),
            _type: PhantomData,
        }
    }

    /// Port of `of(String name)`: `new Atom<>(name)`.
    pub fn of(name: &str) -> Atom<T> {
        Atom::new(name)
    }

    /// Port of the accessor `name()`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Port of the record's generated `hashCode()`: `name.hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_lang::string_hash_code(&self.name)
    }
}

impl<T> Clone for Atom<T> {
    fn clone(&self) -> Self {
        Atom::new(&self.name)
    }
}

impl<T> PartialEq for Atom<T> {
    /// Port of the record's generated `equals`: the name decides.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T> Eq for Atom<T> {}

impl<T> std::hash::Hash for Atom<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl<T> std::fmt::Debug for Atom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Atom").field("name", &self.name).finish()
    }
}

impl<T> std::fmt::Display for Atom<T> {
    /// Port of the `toString()` override: `"<" + this.name + ">"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.name)
    }
}
