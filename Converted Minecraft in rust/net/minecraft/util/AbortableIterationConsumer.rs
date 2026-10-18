//! Port of: net/minecraft/util/AbortableIterationConsumer.java
//! Java class(es): net.minecraft.util.AbortableIterationConsumer, net.minecraft.util.Continuation
//! Status: VERIFIED
//!
//! Session 22. `@FunctionalInterface public interface AbortableIterationConsumer<T>`: a per-element
//! callback that can stop an iteration, returning a [`Continuation`] -- `CONTINUE` or `ABORT` --
//! plus `forConsumer`, which adapts a plain consumer that never aborts.
//!
//! Any `FnMut(T) -> Continuation` closure is an `AbortableIterationConsumer<T>`. `accept` takes
//! `&mut self` because a Java consumer may carry state.
//!
//! Verified by the `ABORTCONT` and `ABORTFOR` groups of `small_probe.txt` (`tests/session22.rs`):
//! both constants with their ordinals and `shouldAbort()`, and `forConsumer` passing each element
//! through and answering `CONTINUE`.

/// Port of `net.minecraft.util.AbortableIterationConsumer<T>`.
pub trait AbortableIterationConsumer<T> {
    /// Port of `Continuation accept(T entry)`.
    fn accept(&mut self, entry: T) -> Continuation;
}

impl<T, F: FnMut(T) -> Continuation> AbortableIterationConsumer<T> for F {
    fn accept(&mut self, entry: T) -> Continuation {
        self(entry)
    }
}

/// The lambda `forConsumer(consumer)` returns: pass the element on, always `CONTINUE`.
#[derive(Debug, Clone, Copy)]
pub struct ForConsumer<F>(F);

impl<T, F: FnMut(T)> AbortableIterationConsumer<T> for ForConsumer<F> {
    fn accept(&mut self, entry: T) -> Continuation {
        (self.0)(entry);
        Continuation::Continue
    }
}

/// Port of `static <T> AbortableIterationConsumer<T> forConsumer(Consumer<T> consumer)`.
pub fn for_consumer<T, F: FnMut(T)>(consumer: F) -> ForConsumer<F> {
    ForConsumer(consumer)
}

/// Port of `AbortableIterationConsumer.Continuation`. Declaration order is the ordinal order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Continuation {
    /// `CONTINUE` -- ordinal 0.
    Continue = 0,
    /// `ABORT` -- ordinal 1.
    Abort = 1,
}

impl Continuation {
    /// Java: `Continuation.values()`.
    pub const VALUES: [Continuation; 2] = [Continuation::Continue, Continuation::Abort];

    /// Java: `ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Continuation::Continue => "CONTINUE",
            Continuation::Abort => "ABORT",
        }
    }

    /// Port of `shouldAbort()`: `this == ABORT`.
    pub const fn should_abort(&self) -> bool {
        matches!(self, Continuation::Abort)
    }
}
