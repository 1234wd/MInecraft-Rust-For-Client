//! Port of: net/minecraft/util/filefix/virtualfilesystem/exception/CowFSCreationException.java
//! Java class(es): net.minecraft.util.filefix.virtualfilesystem.exception.CowFSCreationException
//! Status: VERIFIED
//!
//! Session 24. `public class CowFSCreationException extends CowFSFileSystemException`, with one
//! `(String message)` constructor and no other members.
//!
//! The superclass chain is `CowFSFileSystemException` -> `java.nio.file.FileSystemException` ->
//! `IOException`, so the argument is stored as the FILE, exactly as in
//! [`CowFSFileSystemException`](super::CowFSFileSystemException): `getFile()` is the string passed,
//! `getOtherFile()` and `getReason()` are null, `getMessage()` is the file, and `toString()` is
//! `"<class>: <file>"` (or just the class name for a null file).
//!
//! Verified by the `EXC24` and `EXC24FS` groups of `sweep_probe.txt` (`tests/session24.rs`) for a
//! plain, an empty, a null, a control-character and a non-ASCII argument.

use crate::javacompat::throwable;

/// `CowFSCreationException.class.getName()`.
pub const CLASS_NAME: &str =
    "net.minecraft.util.filefix.virtualfilesystem.exception.CowFSCreationException";

/// The direct Java superclass. Checked: an `IOException` by way of `FileSystemException`.
pub const SUPERCLASS: &str =
    "net.minecraft.util.filefix.virtualfilesystem.exception.CowFSFileSystemException";

/// Port of `net.minecraft.util.filefix.virtualfilesystem.exception.CowFSCreationException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CowFSCreationException {
    file: Option<String>,
}

impl CowFSCreationException {
    /// Port of `new CowFSCreationException(String message)` -- which `FileSystemException` stores
    /// as the file.
    pub fn new(message: Option<&str>) -> Self {
        CowFSCreationException {
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

    /// Port of `FileSystemException#getMessage()`. With no other file and no reason it is the
    /// file (see `CowFSFileSystemException::get_message` for the JDK body).
    pub fn get_message(&self) -> Option<&str> {
        self.get_file()
    }
}

impl std::fmt::Display for CowFSCreationException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for CowFSCreationException {}

/// `throw new CowFSCreationException(message)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("{}", CowFSCreationException::new(Some(message)))
}
