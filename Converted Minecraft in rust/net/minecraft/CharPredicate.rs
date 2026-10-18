//! Port of: net/minecraft/CharPredicate.java
//! Java class(es): net.minecraft.CharPredicate
//! Status: VERIFIED
//!
//! Session 22. `@FunctionalInterface public interface CharPredicate`: `boolean test(char)` with the
//! `and` / `negate` / `or` combinators of `java.util.function.Predicate`, for a primitive `char`.
//!
//! # A Java `char` is a UTF-16 code unit
//!
//! The parameter is `u16`, not Rust's `char`: a Java `char` can be half of a surrogate pair
//! (`\u{d800}` ...), which no Rust `char` can hold, and callers test strings one code unit at a
//! time. The probe tests three surrogate units.
//!
//! # Combinators
//!
//! `and`/`or` short-circuit (`&&`/`||`) and call `this` first; `negate` is `!`. Java's
//! `Objects.requireNonNull(other)` has no counterpart -- a Rust predicate cannot be null. The
//! combinators return small adapter structs ([`And`], [`Or`], [`Negate`]); any `Fn(u16) -> bool`
//! closure is a `CharPredicate`.
//!
//! Verified by the `CHARPRED` and `CHARPREDSURR` groups of `small_probe.txt` (`tests/session22.rs`):
//! `and`, `or`, `negate` and a nested combination over every unit `0..0x300`.

/// Port of `net.minecraft.CharPredicate`.
pub trait CharPredicate {
    /// Port of `boolean test(char value)`.
    fn test(&self, value: u16) -> bool;

    /// Port of `default CharPredicate and(CharPredicate other)`.
    fn and<O: CharPredicate>(self, other: O) -> And<Self, O>
    where
        Self: Sized,
    {
        And(self, other)
    }

    /// Port of `default CharPredicate negate()`.
    fn negate(self) -> Negate<Self>
    where
        Self: Sized,
    {
        Negate(self)
    }

    /// Port of `default CharPredicate or(CharPredicate other)`.
    fn or<O: CharPredicate>(self, other: O) -> Or<Self, O>
    where
        Self: Sized,
    {
        Or(self, other)
    }
}

impl<F: Fn(u16) -> bool> CharPredicate for F {
    fn test(&self, value: u16) -> bool {
        self(value)
    }
}

/// `this.and(other)`: `value -> this.test(value) && other.test(value)`.
#[derive(Debug, Clone, Copy)]
pub struct And<A, B>(A, B);

impl<A: CharPredicate, B: CharPredicate> CharPredicate for And<A, B> {
    fn test(&self, value: u16) -> bool {
        self.0.test(value) && self.1.test(value)
    }
}

/// `this.or(other)`: `value -> this.test(value) || other.test(value)`.
#[derive(Debug, Clone, Copy)]
pub struct Or<A, B>(A, B);

impl<A: CharPredicate, B: CharPredicate> CharPredicate for Or<A, B> {
    fn test(&self, value: u16) -> bool {
        self.0.test(value) || self.1.test(value)
    }
}

/// `this.negate()`: `value -> !this.test(value)`.
#[derive(Debug, Clone, Copy)]
pub struct Negate<A>(A);

impl<A: CharPredicate> CharPredicate for Negate<A> {
    fn test(&self, value: u16) -> bool {
        !self.0.test(value)
    }
}
