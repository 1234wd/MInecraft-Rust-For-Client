//! Port of: net/minecraft/Optionull.java
//! Java class(es): net.minecraft.Optionull
//! Status: VERIFIED
//!
//! Session 22. `public class Optionull`: static helpers for nullable values and possibly-empty
//! collections and arrays.
//!
//! # Null becomes `Option`
//!
//! A `@Nullable T` parameter is an `Option<T>` here, so each helper keeps its Java meaning:
//!
//! | Java | Rust |
//! |---|---|
//! | `orElse(t, default)` (`Objects.requireNonNullElse`) | [`or_else`] |
//! | `map(t, f)`, `mapOrDefault`, `mapOrElse` | [`map`], [`map_or_default`], [`map_or_else`] |
//! | `first(collection)`, `firstOrDefault`, `firstOrElse` | [`first`], [`first_or_default`], [`first_or_else`] |
//! | `isNullOrEmpty(T[])`, `(boolean[])` ... `(double[])` | [`is_null_or_empty`] |
//!
//! Three Java behaviours have no Rust counterpart, and the probe pins each from the Java side:
//!
//! * `orElse(null, null)` throws `NullPointerException: defaultObj` -- a Rust default cannot be null.
//! * `map`'s function may return null, making the result null; in Rust the function returns `R`,
//!   and a caller that needs "maybe nothing" uses `R = Option<_>`.
//! * `first` returns null for an empty collection AND for one whose first element is null; Rust
//!   collections hold no nulls, so `None` means empty.
//!
//! The nine `isNullOrEmpty` overloads (one per primitive array type plus `T[]`) collapse into one
//! generic over the element type: their bodies are identical.
//!
//! Verified by the `OPTIONULL` and `OPTIONULLNPE` groups of `small_probe.txt` (`tests/session22.rs`).

/// Port of `Optionull.orElse(T t, T defaultValue)`: `t` unless null.
#[deprecated(note = "deprecated in vanilla")]
pub fn or_else<T>(t: Option<T>, default_value: T) -> T {
    t.unwrap_or(default_value)
}

/// Port of `Optionull.map(T t, Function<T, R> map)`: null in, null out.
pub fn map<T, R>(t: Option<T>, map: impl FnOnce(T) -> R) -> Option<R> {
    t.map(map)
}

/// Port of `Optionull.mapOrDefault(T t, Function<T, R> map, R defaultValue)`.
pub fn map_or_default<T, R>(t: Option<T>, map: impl FnOnce(T) -> R, default_value: R) -> R {
    match t {
        Some(v) => map(v),
        None => default_value,
    }
}

/// Port of `Optionull.mapOrElse(T t, Function<T, R> map, Supplier<R> elseSupplier)`.
pub fn map_or_else<T, R>(
    t: Option<T>,
    map: impl FnOnce(T) -> R,
    else_supplier: impl FnOnce() -> R,
) -> R {
    match t {
        Some(v) => map(v),
        None => else_supplier(),
    }
}

/// Port of `Optionull.first(Collection<T> collection)`: the first element, or null when empty.
pub fn first<I: IntoIterator>(collection: I) -> Option<I::Item> {
    collection.into_iter().next()
}

/// Port of `Optionull.firstOrDefault(Collection<T> collection, T defaultValue)`.
pub fn first_or_default<I: IntoIterator>(collection: I, default_value: I::Item) -> I::Item {
    collection.into_iter().next().unwrap_or(default_value)
}

/// Port of `Optionull.firstOrElse(Collection<T> collection, Supplier<T> elseSupplier)`.
pub fn first_or_else<I: IntoIterator>(
    collection: I,
    else_supplier: impl FnOnce() -> I::Item,
) -> I::Item {
    match collection.into_iter().next() {
        Some(v) => v,
        None => else_supplier(),
    }
}

/// Port of every `Optionull.isNullOrEmpty(X[] t)` overload: null or zero-length.
pub fn is_null_or_empty<T>(t: Option<&[T]>) -> bool {
    match t {
        None => true,
        Some(a) => a.is_empty(),
    }
}
