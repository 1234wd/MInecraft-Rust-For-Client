//! Port of: net/minecraft/util/filefix/virtualfilesystem/exception/CowFSSymlinkException.java
//! Java class(es): net.minecraft.util.filefix.virtualfilesystem.exception.CowFSSymlinkException
//! Status: VERIFIED
//!
//! Session 24. `public class CowFSSymlinkException extends CowFSCreationException`, with one
//! `(String message)` constructor and no other members.
//!
//! One level further down the same chain as
//! [`CowFSCreationException`](super::CowFSCreationException): `CowFSCreationException` ->
//! `CowFSFileSystemException` -> `java.nio.file.FileSystemException`. The argument is therefore the
//! FILE, not a message, and every accessor follows from that.
//!
//! Verified by the `EXC24` and `EXC24FS` groups of `sweep_probe.txt` (`tests/session24.rs`).

use crate::javacompat::throwable;

/// `CowFSSymlinkException.class.getName()`.
pub const CLASS_NAME: &str =
    "net.minecraft.util.filefix.virtualfilesystem.exception.CowFSSymlinkException";

/// The direct Java superclass.
pub const SUPERCLASS: &str =
    "net.minecraft.util.filefix.virtualfilesystem.exception.CowFSCreationException";

/// Port of `net.minecraft.util.filefix.virtualfilesystem.exception.CowFSSymlinkException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CowFSSymlinkException {
    file: Option<String>,
}

impl CowFSSymlinkException {
    /// Port of `new CowFSSymlinkException(String message)` -- stored as the file.
    pub fn new(message: Option<&str>) -> Self {
        CowFSSymlinkException {
            file: message.map(str::to_string),
        }
    }

    /// Port of `FileSystemException#getFile()`.
    pub fn get_file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    /// Port of `FileSystemException#getOtherFile()`. Always null: no constructor here sets it.
    pub fn get_other_file(&self) -> Option<&str> {
        None
    }

    /// Port of `FileSystemException#getReason()`. Always null: no constructor here sets it.
    pub fn get_reason(&self) -> Option<&str> {
        None
    }

    /// Port of `FileSystemException#getMessage()`: the file, since there is no other file and no
    /// reason.
    pub fn get_message(&self) -> Option<&str> {
        self.get_file()
    }
}

impl std::fmt::Display for CowFSSymlinkException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for CowFSSymlinkException {}

/// `throw new CowFSSymlinkException(message)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("{}", CowFSSymlinkException::new(Some(message)))
}
