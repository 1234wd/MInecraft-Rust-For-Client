//! Port of: net/minecraft/commands/CommandResultCallback.java
//! Java class(es): net.minecraft.commands.CommandResultCallback, CommandResultCallback::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 22. `@FunctionalInterface public interface CommandResultCallback`: told whether a command
//! succeeded and its result. One abstract method, two defaults, a no-op `EMPTY` and a static `chain`.
//!
//! # `chain` works by IDENTITY with `EMPTY`
//!
//! ```java
//! if (first == EMPTY) return second;
//! return second == EMPTY ? first : (success, result) -> { first.onResult(..); second.onResult(..); };
//! ```
//!
//! So chaining with `EMPTY` returns the other callback ITSELF, not a wrapper, and chaining two real
//! callbacks calls the first, then the second. To keep that identity, callbacks are shared as
//! `Arc<dyn CommandResultCallback>` and [`EMPTY`] is one `Arc`; [`chain`] compares with
//! `Arc::ptr_eq`, exactly as Java compares with `==`. Any `Fn(bool, i32)` closure is a callback.
//!
//! `onSuccess(r)` is `onResult(true, r)` and `onFailure()` is `onResult(false, 0)`. `EMPTY`'s
//! `toString()` is `"<empty>"` ([`Empty`]'s `Display`).
//!
//! Verified by the `CRCEMPTY`, `CRCCHAIN` and `CRCCALLS` groups of `small_probe.txt`
//! (`tests/session22.rs`).

use std::sync::{Arc, LazyLock};

/// Port of `net.minecraft.commands.CommandResultCallback`.
pub trait CommandResultCallback: Send + Sync {
    /// Port of `void onResult(boolean success, int result)`.
    fn on_result(&self, success: bool, result: i32);

    /// Port of `default void onSuccess(int result)`: `onResult(true, result)`.
    fn on_success(&self, result: i32) {
        self.on_result(true, result);
    }

    /// Port of `default void onFailure()`: `onResult(false, 0)`.
    fn on_failure(&self) {
        self.on_result(false, 0);
    }
}

impl<F: Fn(bool, i32) + Send + Sync> CommandResultCallback for F {
    fn on_result(&self, success: bool, result: i32) {
        self(success, result)
    }
}

/// The anonymous class behind `CommandResultCallback.EMPTY`: does nothing.
#[derive(Debug, Clone, Copy, Default)]
pub struct Empty;

impl CommandResultCallback for Empty {
    fn on_result(&self, _success: bool, _result: i32) {}
}

impl std::fmt::Display for Empty {
    /// Port of `EMPTY.toString()`: `"<empty>"`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<empty>")
    }
}

/// Port of `CommandResultCallback.EMPTY`: ONE shared instance, so `chain` can recognise it.
pub static EMPTY: LazyLock<Arc<dyn CommandResultCallback>> = LazyLock::new(|| Arc::new(Empty));

/// The lambda `chain` builds for two real callbacks.
struct Chained(
    Arc<dyn CommandResultCallback>,
    Arc<dyn CommandResultCallback>,
);

impl CommandResultCallback for Chained {
    fn on_result(&self, success: bool, result: i32) {
        self.0.on_result(success, result);
        self.1.on_result(success, result);
    }
}

/// Port of `static CommandResultCallback chain(CommandResultCallback first, CommandResultCallback second)`.
pub fn chain(
    first: Arc<dyn CommandResultCallback>,
    second: Arc<dyn CommandResultCallback>,
) -> Arc<dyn CommandResultCallback> {
    if Arc::ptr_eq(&first, &EMPTY) {
        second
    } else if Arc::ptr_eq(&second, &EMPTY) {
        first
    } else {
        Arc::new(Chained(first, second))
    }
}
