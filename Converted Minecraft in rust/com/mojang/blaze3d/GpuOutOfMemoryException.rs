//! Port of: com/mojang/blaze3d/GpuOutOfMemoryException.java
//! Java class(es): com.mojang.blaze3d.GpuOutOfMemoryException
//! Status: VERIFIED
//!
//! Session 22. `public class GpuOutOfMemoryException extends RuntimeException`, with one `(String message)` constructor and no
//! other members.
//!
//! Unchecked. The GPU backends throw it when a device allocation fails.
//!
//! # Port shape
//!
//! A value type carrying the (nullable) message, so a caller can either throw it with [`throw`]
//! -- a panic whose text is Java's `toString`, per `DESIGN_DECISIONS.md#runtime-exceptions` --
//! or, where vanilla catches it and carries on, return it in a `Result`. `Display` is
//! `Throwable#toString`; see [`crate::javacompat::throwable`].
//!
//! Verified by the `EXC` group of `small_probe.txt` (`tests/session22.rs`): class name,
//! superclass, checked/unchecked, `getMessage()` and `toString()` for a plain, an empty, a null,
//! a control-character and a non-ASCII message.

use crate::javacompat::throwable;

/// `GpuOutOfMemoryException.class.getName()`.
pub const CLASS_NAME: &str = "com.mojang.blaze3d.GpuOutOfMemoryException";

/// The direct Java superclass. Unchecked.
pub const SUPERCLASS: &str = "java.lang.RuntimeException";

/// Port of `com.mojang.blaze3d.GpuOutOfMemoryException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuOutOfMemoryException {
    message: Option<String>,
}

impl GpuOutOfMemoryException {
    /// Port of `new GpuOutOfMemoryException(String message)`. Java accepts a null message, and so does this.
    pub fn new(message: Option<&str>) -> Self {
        GpuOutOfMemoryException {
            message: message.map(str::to_string),
        }
    }

    /// Port of `Throwable#getMessage()`.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl std::fmt::Display for GpuOutOfMemoryException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for GpuOutOfMemoryException {}

/// `throw new GpuOutOfMemoryException(message)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("{}", GpuOutOfMemoryException::new(Some(message)))
}
