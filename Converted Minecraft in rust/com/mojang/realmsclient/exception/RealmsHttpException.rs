//! Port of: com/mojang/realmsclient/exception/RealmsHttpException.java
//! Java class(es): com.mojang.realmsclient.exception.RealmsHttpException
//! Status: VERIFIED
//!
//! Session 22. `public class RealmsHttpException extends RuntimeException` with exactly one
//! constructor, `(String s, Exception e)`, and no other members.
//!
//! Unchecked. `realmsclient/client/Request` wraps what it catches in it as
//! `new RealmsHttpException(e.getMessage(), e)`. The message is the string PASSED, not derived from
//! the cause: `RuntimeException(String, Throwable)` keeps both as given.
//!
//! # `Exception`, not `Throwable`
//!
//! Java narrows the cause parameter to `Exception`, so an `Error` cannot be passed. Rust has no
//! such distinction; [`Cause`] is any error, and the narrowing is recorded here rather than
//! enforced.
//!
//! Verified by the `EXC` group of `small_probe.txt` (`tests/session22.rs`): five messages, each
//! with each of four causes (including null).

use crate::javacompat::throwable::{self, Cause};

/// `RealmsHttpException.class.getName()`.
pub const CLASS_NAME: &str = "com.mojang.realmsclient.exception.RealmsHttpException";

/// The direct Java superclass. Unchecked.
pub const SUPERCLASS: &str = "java.lang.RuntimeException";

/// Port of `com.mojang.realmsclient.exception.RealmsHttpException`.
#[derive(Debug)]
pub struct RealmsHttpException {
    message: Option<String>,
    cause: Option<Cause>,
}

impl RealmsHttpException {
    /// Port of `new RealmsHttpException(String s, Exception e)`. Both may be null.
    pub fn new(s: Option<&str>, e: Option<Cause>) -> Self {
        RealmsHttpException {
            message: s.map(str::to_string),
            cause: e,
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

impl std::fmt::Display for RealmsHttpException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for RealmsHttpException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        throwable::source_of(&self.cause)
    }
}

/// `throw new RealmsHttpException(s, e)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(s: &str, e: Option<Cause>) -> ! {
    panic!("{}", RealmsHttpException::new(Some(s), e))
}
