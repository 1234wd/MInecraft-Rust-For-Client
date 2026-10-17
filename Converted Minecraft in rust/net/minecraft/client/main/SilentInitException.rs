//! Port of: net/minecraft/client/main/SilentInitException.java
//! Java class(es): net.minecraft.client.main.SilentInitException
//! Status: VERIFIED
//!
//! Session 22. `public class SilentInitException extends RuntimeException`, with a
//! `(String message)` and a `(String message, Throwable cause)` constructor and no other members.
//!
//! Unchecked. `GameRenderer` throws it; `client/main/Main` catches it around `new Minecraft(...)`,
//! logs "Failed to create window: " and returns -- no crash report, hence "silent".
//!
//! # Port shape
//!
//! A value type carrying the (nullable) message and the optional cause, so a caller can throw it
//! with [`throw`] (a panic whose text is Java's `toString`, per
//! `DESIGN_DECISIONS.md#runtime-exceptions`) or return it in a `Result`. With the two-argument
//! constructor the message is the one PASSED, not the cause's -- only `Throwable(Throwable)`
//! derives a message from its cause, and this class never calls that one.
//!
//! Verified by the `EXC` group of `small_probe.txt` (`tests/session22.rs`): class name,
//! superclass, `getMessage()`, `toString()` and the cause's `toString()` for five messages, each
//! alone and with each of four causes (including a null cause).

use crate::javacompat::throwable::{self, Cause};

/// `SilentInitException.class.getName()`.
pub const CLASS_NAME: &str = "net.minecraft.client.main.SilentInitException";

/// The direct Java superclass. Unchecked.
pub const SUPERCLASS: &str = "java.lang.RuntimeException";

/// Port of `net.minecraft.client.main.SilentInitException`.
#[derive(Debug)]
pub struct SilentInitException {
    message: Option<String>,
    cause: Option<Cause>,
}

impl SilentInitException {
    /// Port of `new SilentInitException(String message)`.
    pub fn new(message: Option<&str>) -> Self {
        SilentInitException {
            message: message.map(str::to_string),
            cause: None,
        }
    }

    /// Port of `new SilentInitException(String message, Throwable cause)`. A null cause is legal.
    pub fn with_cause(message: Option<&str>, cause: Option<Cause>) -> Self {
        SilentInitException {
            message: message.map(str::to_string),
            cause,
        }
    }

    /// Port of `Throwable#getMessage()`.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Port of `Throwable#getCause()`.
    pub fn get_cause(&self) -> Option<&Cause> {
        self.cause.as_ref()
    }
}

impl std::fmt::Display for SilentInitException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for SilentInitException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        throwable::source_of(&self.cause)
    }
}

/// `throw new SilentInitException(message)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("{}", SilentInitException::new(Some(message)))
}
