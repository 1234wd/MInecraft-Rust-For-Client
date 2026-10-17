//! Port of: net/minecraft/util/CryptException.java
//! Java class(es): net.minecraft.util.CryptException
//! Status: VERIFIED
//!
//! Session 22. `public class CryptException extends Exception` with exactly one constructor,
//! `(Throwable cause)`, and no other members.
//!
//! CHECKED (`extends Exception`). `Crypt`'s key-generation and digest helpers wrap whatever they
//! catch in it, and six files catch it, so this is a value for a `Result` far more often than
//! something to [`throw`].
//!
//! # The message is the CAUSE'S `toString`
//!
//! The only constructor calls `Exception(Throwable)`, whose message is `cause.toString()`:
//! `new CryptException(new RuntimeException("inner")).getMessage()` is
//! `"java.lang.RuntimeException: inner"`, so the outer `toString` names two classes. A null cause
//! gives a null message and a `toString` that is the bare class name. See
//! [`crate::javacompat::throwable::message_from_cause`].
//!
//! Verified by the `EXC` group of `small_probe.txt` (`tests/session22.rs`) for four causes,
//! including null and one whose own message is empty.

use crate::javacompat::throwable::{self, Cause};

/// `CryptException.class.getName()`.
pub const CLASS_NAME: &str = "net.minecraft.util.CryptException";

/// The direct Java superclass. Checked: not a `RuntimeException`.
pub const SUPERCLASS: &str = "java.lang.Exception";

/// Port of `net.minecraft.util.CryptException`.
#[derive(Debug)]
pub struct CryptException {
    message: Option<String>,
    cause: Option<Cause>,
}

impl CryptException {
    /// Port of `new CryptException(Throwable cause)`. A null cause is legal and gives a null message.
    pub fn new(cause: Option<Cause>) -> Self {
        CryptException {
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

impl std::fmt::Display for CryptException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for CryptException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        throwable::source_of(&self.cause)
    }
}

/// `throw new CryptException(cause)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(cause: Cause) -> ! {
    panic!("{}", CryptException::new(Some(cause)))
}
