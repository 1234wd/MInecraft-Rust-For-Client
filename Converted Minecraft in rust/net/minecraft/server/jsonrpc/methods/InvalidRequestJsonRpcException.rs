//! Port of: net/minecraft/server/jsonrpc/methods/InvalidRequestJsonRpcException.java
//! Java class(es): net.minecraft.server.jsonrpc.methods.InvalidRequestJsonRpcException
//! Status: VERIFIED
//!
//! Session 22. `public class InvalidRequestJsonRpcException extends RuntimeException`, with one `(String message)` constructor and no
//! other members.
//!
//! Unchecked. One of the four JSON-RPC management-protocol errors. `jsonrpc/Connection.java`
//! catches each of the four in its own `catch` and answers `JsonRPCErrors.INVALID_REQUEST` with
//! `e.getMessage()` as the data, so the CLASS (not just the message) is what a caller distinguishes.
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

/// `InvalidRequestJsonRpcException.class.getName()`.
pub const CLASS_NAME: &str = "net.minecraft.server.jsonrpc.methods.InvalidRequestJsonRpcException";

/// The direct Java superclass. Unchecked.
pub const SUPERCLASS: &str = "java.lang.RuntimeException";

/// Port of `net.minecraft.server.jsonrpc.methods.InvalidRequestJsonRpcException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidRequestJsonRpcException {
    message: Option<String>,
}

impl InvalidRequestJsonRpcException {
    /// Port of `new InvalidRequestJsonRpcException(String message)`. Java accepts a null message, and so does this.
    pub fn new(message: Option<&str>) -> Self {
        InvalidRequestJsonRpcException {
            message: message.map(str::to_string),
        }
    }

    /// Port of `Throwable#getMessage()`.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl std::fmt::Display for InvalidRequestJsonRpcException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for InvalidRequestJsonRpcException {}

/// `throw new InvalidRequestJsonRpcException(message)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("{}", InvalidRequestJsonRpcException::new(Some(message)))
}
