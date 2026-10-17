//! Port of: com/mojang/blaze3d/systems/SurfaceException.java
//! Java class(es): com.mojang.blaze3d.systems.SurfaceException
//! Status: VERIFIED
//!
//! Session 22. `public class SurfaceException extends Exception`, with a `(String message)` and a
//! `(Throwable cause)` constructor and no other members.
//!
//! CHECKED (`extends Exception`). `GpuSurface` and `VulkanGpuSurface` throw it and their callers
//! catch it, which is the case this value type exists for: such a caller returns it in a `Result`
//! rather than calling [`throw`].
//!
//! # The two constructors disagree about where the message comes from
//!
//! `SurfaceException(String)` stores the message it is given. `SurfaceException(Throwable)` calls
//! `Exception(Throwable)`, whose message is the cause's whole `toString()` -- class name and all,
//! or null for a null cause. So `new SurfaceException(new RuntimeException("x")).toString()` is
//! `"com.mojang.blaze3d.systems.SurfaceException: java.lang.RuntimeException: x"`. See
//! [`crate::javacompat::throwable::message_from_cause`].
//!
//! Verified by the `EXC` group of `small_probe.txt` (`tests/session22.rs`): five messages through
//! the first constructor and four causes (including null) through the second.

use crate::javacompat::throwable::{self, Cause};

/// `SurfaceException.class.getName()`.
pub const CLASS_NAME: &str = "com.mojang.blaze3d.systems.SurfaceException";

/// The direct Java superclass. Checked: not a `RuntimeException`.
pub const SUPERCLASS: &str = "java.lang.Exception";

/// Port of `com.mojang.blaze3d.systems.SurfaceException`.
#[derive(Debug)]
pub struct SurfaceException {
    message: Option<String>,
    cause: Option<Cause>,
}

impl SurfaceException {
    /// Port of `new SurfaceException(String message)`.
    pub fn new(message: Option<&str>) -> Self {
        SurfaceException {
            message: message.map(str::to_string),
            cause: None,
        }
    }

    /// Port of `new SurfaceException(Throwable cause)`: the message becomes `cause.toString()`.
    pub fn from_cause(cause: Option<Cause>) -> Self {
        SurfaceException {
            message: throwable::message_from_cause(cause.as_ref()),
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

impl std::fmt::Display for SurfaceException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for SurfaceException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        throwable::source_of(&self.cause)
    }
}

/// `throw new SurfaceException(message)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("{}", SurfaceException::new(Some(message)))
}
