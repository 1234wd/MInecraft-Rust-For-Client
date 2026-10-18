//! Port of: net/minecraft/util/filefix/virtualfilesystem/exception/CowFSIllegalArgumentException.java
//! Java class(es): net.minecraft.util.filefix.virtualfilesystem.exception.CowFSIllegalArgumentException
//! Status: VERIFIED
//!
//! Session 22. `public class CowFSIllegalArgumentException extends IllegalArgumentException`, with one `(String message)` constructor and no
//! other members.
//!
//! Unchecked. `CopyOnWriteFSPath` throws it ("Other path is of mismatching file system") where the
//! `java.nio.file.Path` contract calls for an `IllegalArgumentException`; a caller that catches
//! `IllegalArgumentException` catches this too, which is why [`SUPERCLASS`] is recorded.
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

/// `CowFSIllegalArgumentException.class.getName()`.
pub const CLASS_NAME: &str =
    "net.minecraft.util.filefix.virtualfilesystem.exception.CowFSIllegalArgumentException";

/// The direct Java superclass. Unchecked.
pub const SUPERCLASS: &str = "java.lang.IllegalArgumentException";

/// Port of `net.minecraft.util.filefix.virtualfilesystem.exception.CowFSIllegalArgumentException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CowFSIllegalArgumentException {
    message: Option<String>,
}

impl CowFSIllegalArgumentException {
    /// Port of `new CowFSIllegalArgumentException(String message)`. Java accepts a null message, and so does this.
    pub fn new(message: Option<&str>) -> Self {
        CowFSIllegalArgumentException {
            message: message.map(str::to_string),
        }
    }

    /// Port of `Throwable#getMessage()`.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl std::fmt::Display for CowFSIllegalArgumentException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for CowFSIllegalArgumentException {}

/// `throw new CowFSIllegalArgumentException(message)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("{}", CowFSIllegalArgumentException::new(Some(message)))
}
