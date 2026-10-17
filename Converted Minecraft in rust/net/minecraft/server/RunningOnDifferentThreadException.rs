//! Port of: net/minecraft/server/RunningOnDifferentThreadException.java
//! Java class(es): net.minecraft.server.RunningOnDifferentThreadException
//! Status: VERIFIED
//!
//! Session 22. `public final class RunningOnDifferentThreadException extends RuntimeException`: a
//! private no-argument constructor, one `public static final` instance, and an overridden
//! `fillInStackTrace()`.
//!
//! # What it is for
//!
//! `PacketUtils.ensureRunningOnSameThread` throws the shared instance to abandon handling a packet
//! on the network thread after re-queueing it for the main thread, and the connection catches it
//! and does nothing. It is control flow, not an error, so it is allocated once and never records
//! a stack trace: the constructor sets an empty trace and `fillInStackTrace` resets it instead of
//! walking the stack.
//!
//! # Port shape
//!
//! A unit struct with the one instance as a `const`. Rust exceptions carry no Java stack trace,
//! so [`RunningOnDifferentThreadException::fill_in_stack_trace`] is the identity Java's override
//! reduces to (it returns `this`), and [`RunningOnDifferentThreadException::get_stack_trace_len`]
//! is the length Java reports, which is always 0.
//!
//! The message is null -- the private constructor calls the implicit `RuntimeException()` -- so
//! `toString()` is the bare class name.
//!
//! Verified by the `EXC` and `EXCSTACK` groups of `small_probe.txt` (`tests/session22.rs`):
//! class name, superclass, null message, `toString`, a stack trace of length 0 both before and
//! after `fillInStackTrace()`, and `fillInStackTrace() == this`.

/// `RunningOnDifferentThreadException.class.getName()`.
pub const CLASS_NAME: &str = "net.minecraft.server.RunningOnDifferentThreadException";

/// The direct Java superclass. Unchecked.
pub const SUPERCLASS: &str = "java.lang.RuntimeException";

/// Port of `net.minecraft.server.RunningOnDifferentThreadException`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunningOnDifferentThreadException;

impl RunningOnDifferentThreadException {
    /// Port of `RunningOnDifferentThreadException.RUNNING_ON_DIFFERENT_THREAD`, the only instance.
    pub const RUNNING_ON_DIFFERENT_THREAD: RunningOnDifferentThreadException =
        RunningOnDifferentThreadException;

    /// Port of `Throwable#getMessage()`: always null for this class.
    pub fn get_message(&self) -> Option<&str> {
        None
    }

    /// Port of the `fillInStackTrace()` override: clears the trace and returns `this`.
    pub fn fill_in_stack_trace(&self) -> &Self {
        self
    }

    /// `getStackTrace().length`, which the constructor and `fillInStackTrace` both pin to 0.
    pub fn get_stack_trace_len(&self) -> i32 {
        0
    }
}

impl std::fmt::Display for RunningOnDifferentThreadException {
    /// Port of `Throwable#toString()`: the bare class name, because the message is null.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(CLASS_NAME)
    }
}

impl std::error::Error for RunningOnDifferentThreadException {}

/// `throw RunningOnDifferentThreadException.RUNNING_ON_DIFFERENT_THREAD`.
#[cold]
#[inline(never)]
pub fn throw() -> ! {
    panic!("{}", RunningOnDifferentThreadException::RUNNING_ON_DIFFERENT_THREAD)
}
