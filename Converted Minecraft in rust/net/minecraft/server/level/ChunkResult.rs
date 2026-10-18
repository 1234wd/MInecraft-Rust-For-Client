//! Port of: net/minecraft/server/level/ChunkResult.java
//! Java class(es): net.minecraft.server.level.ChunkResult, net.minecraft.server.level.Fail, net.minecraft.server.level.Success
//! Status: VERIFIED
//!
//! Session 22. `public interface ChunkResult<T>`: the outcome of a chunk-loading step, implemented by
//! two records -- `Success<T>(T value)` and `Fail<T>(Supplier<String> error)` -- with three static
//! factories and a static `orElse`.
//!
//! # Port shape
//!
//! The two records are the two variants of an enum. The error is a SUPPLIER, called afresh on every
//! `getError()` (probed: three calls, three different messages from a counting supplier); `map` on a
//! `Fail` keeps the very same supplier (probed by identity). Here the supplier is an
//! `Arc<dyn Fn() -> String>`, and `Arc::ptr_eq` is that identity.
//!
//! | Java | `Success` | `Fail` |
//! |---|---|---|
//! | `isSuccess()` | true | false |
//! | `orElse(x)` | the value | `x` |
//! | `getError()` | null | `error.get()` |
//! | `ifSuccess(c)` | runs `c`, returns `this` | returns `this` |
//! | `map(f)` | `Success(f(value))` | `Fail(error)` |
//! | `orElseThrow(s)` | the value | throws `s.get()` |
//!
//! `orElseThrow` returns `Result<T, E>` -- the Rust form of "throw what the supplier built". The
//! static `orElse(chunkResult, orElse)` (here [`or_else_of`]) is the instance `orElse` with a null
//! check that a Rust value never needs.
//!
//! # Record members
//!
//! `Success` is a one-component record: `equals` compares the value, `hashCode` is the value's hash
//! ([`ChunkResult::java_hash_code_with`] takes it as a function, as Rust has no universal Java
//! `hashCode`), `toString` is `Success[value=..]`. `Fail` compares its supplier by IDENTITY (a lambda
//! has no `equals`), and its `hashCode`/`toString` print the lambda's identity hash, which differs
//! per JVM run -- so they are not ported, and the `Display` impl covers `Success` only.
//!
//! Verified by the `CHUNKRES`, `CHUNKRESERR`, `CHUNKRESFAILEQ`, `REC` and `RECEQ` groups of
//! `small_probe.txt` (`tests/session22.rs`).

use std::sync::Arc;

/// A `Supplier<String>`, shared so that its identity can be compared.
pub type ErrorSupplier = Arc<dyn Fn() -> String + Send + Sync>;

/// Port of `net.minecraft.server.level.ChunkResult<T>`: its two records as variants.
pub enum ChunkResult<T> {
    /// Port of `ChunkResult.Success<T>(T value)`.
    Success(T),
    /// Port of `ChunkResult.Fail<T>(Supplier<String> error)`.
    Fail(ErrorSupplier),
}

impl<T> ChunkResult<T> {
    /// Port of `static <T> ChunkResult<T> of(T value)`.
    pub fn of(value: T) -> ChunkResult<T> {
        ChunkResult::Success(value)
    }

    /// Port of `static <T> ChunkResult<T> error(String error)`: `error(() -> error)`.
    pub fn error_string(error: &str) -> ChunkResult<T> {
        let error = error.to_string();
        ChunkResult::Fail(Arc::new(move || error.clone()))
    }

    /// Port of `static <T> ChunkResult<T> error(Supplier<String> errorSupplier)`.
    pub fn error(error_supplier: ErrorSupplier) -> ChunkResult<T> {
        ChunkResult::Fail(error_supplier)
    }

    /// Port of `boolean isSuccess()`.
    pub fn is_success(&self) -> bool {
        matches!(self, ChunkResult::Success(_))
    }

    /// Port of `T orElse(T orElse)`: the value of a `Success`, else `or_else`. `None` is a Java null.
    pub fn or_else<'a>(&'a self, or_else: Option<&'a T>) -> Option<&'a T> {
        match self {
            ChunkResult::Success(value) => Some(value),
            ChunkResult::Fail(_) => or_else,
        }
    }

    /// Port of `String getError()`: calls the supplier each time; `None` (null) for a `Success`.
    pub fn get_error(&self) -> Option<String> {
        match self {
            ChunkResult::Success(_) => None,
            ChunkResult::Fail(error) => Some(error()),
        }
    }

    /// Port of `ChunkResult<T> ifSuccess(Consumer<T> consumer)`: returns `this`.
    pub fn if_success(&self, consumer: impl FnOnce(&T)) -> &Self {
        if let ChunkResult::Success(value) = self {
            consumer(value);
        }
        self
    }

    /// Port of `<R> ChunkResult<R> map(Function<T, R> map)`: a `Fail` keeps the same supplier.
    pub fn map<R>(&self, map: impl FnOnce(&T) -> R) -> ChunkResult<R> {
        match self {
            ChunkResult::Success(value) => ChunkResult::Success(map(value)),
            ChunkResult::Fail(error) => ChunkResult::Fail(error.clone()),
        }
    }

    /// Port of `<E extends Throwable> T orElseThrow(Supplier<E> exceptionSupplier) throws E`.
    pub fn or_else_throw<E>(self, exception_supplier: impl FnOnce() -> E) -> Result<T, E> {
        match self {
            ChunkResult::Success(value) => Ok(value),
            ChunkResult::Fail(_) => Err(exception_supplier()),
        }
    }

    /// Port of `Success`'s generated `hashCode()` (the value's hash); `None` for a `Fail`, whose
    /// Java hash is the supplier's per-run identity hash.
    pub fn java_hash_code_with(&self, hash: impl Fn(&T) -> i32) -> Option<i32> {
        match self {
            ChunkResult::Success(value) => Some(hash(value)),
            ChunkResult::Fail(_) => None,
        }
    }
}

/// Port of `static <R> R orElse(ChunkResult<? extends R> chunkResult, R orElse)`.
pub fn or_else_of<'a, R>(chunk_result: &'a ChunkResult<R>, or_else: Option<&'a R>) -> Option<&'a R> {
    chunk_result.or_else(None).or(or_else)
}

impl<T: PartialEq> PartialEq for ChunkResult<T> {
    /// Port of the two records' generated `equals`: `Success` by value, `Fail` by supplier identity.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ChunkResult::Success(a), ChunkResult::Success(b)) => a == b,
            (ChunkResult::Fail(a), ChunkResult::Fail(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ChunkResult<T> {
    /// Port of `Success`'s generated `toString()`. A `Fail` prints a placeholder: its Java `toString`
    /// embeds the supplier's per-run identity hash and has no reproducible form.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkResult::Success(value) => write!(f, "Success[value={value}]"),
            ChunkResult::Fail(_) => f.write_str("Fail[error=<supplier>]"),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ChunkResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkResult::Success(value) => f.debug_tuple("Success").field(value).finish(),
            ChunkResult::Fail(_) => f.write_str("Fail(<supplier>)"),
        }
    }
}
