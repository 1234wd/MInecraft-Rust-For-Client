//! Port of: net/minecraft/util/filefix/virtualfilesystem/exception/CowFSNoSuchFileException.java
//! Java class(es): net.minecraft.util.filefix.virtualfilesystem.exception.CowFSNoSuchFileException
//! Status: VERIFIED
//!
//! Session 22. `public class CowFSNoSuchFileException extends NoSuchFileException`, with one `(String message)`
//! constructor and no other members.
//!
//! CHECKED: `java.nio.file.NoSuchFileException` (itself a `FileSystemException`) is an `IOException`. The copy-on-write virtual file system in
//! `util/filefix` throws it where the `java.nio.file` contract calls for the JDK exception, so a
//! caller catching `NoSuchFileException` or `IOException` catches this one too -- which is why
//! [`SUPERCLASS`] is recorded.
//!
//! # The constructor argument is the FILE, not a message
//!
//! The parameter is named `message` in the Minecraft source, but it is passed to
//! `NoSuchFileException(String file)`, which stores it as the FILE and leaves the reason null. Every
//! accessor follows from that, and none of them is the obvious one:
//!
//! * `getFile()` is the string passed; `getOtherFile()` and `getReason()` are null.
//! * `getMessage()` is `FileSystemException#getMessage()`: with no other file and no reason it is
//!   just the file -- or null when the file is null.
//! * `toString()` is therefore `"<class>: <file>"`, and an empty file still prints the `": "`.
//!
//! Verified by the `EXC` and `EXCFS` groups of `small_probe.txt` (`tests/session22.rs`) for a
//! plain, an empty, a null, a control-character and a non-ASCII argument.

use crate::javacompat::throwable;

/// `CowFSNoSuchFileException.class.getName()`.
pub const CLASS_NAME: &str =
    "net.minecraft.util.filefix.virtualfilesystem.exception.CowFSNoSuchFileException";

/// The direct Java superclass. Checked: an `IOException`.
pub const SUPERCLASS: &str = "java.nio.file.NoSuchFileException";

/// Port of `net.minecraft.util.filefix.virtualfilesystem.exception.CowFSNoSuchFileException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CowFSNoSuchFileException {
    file: Option<String>,
}

impl CowFSNoSuchFileException {
    /// Port of `new CowFSNoSuchFileException(String message)` -- which `NoSuchFileException` stores as the file.
    pub fn new(message: Option<&str>) -> Self {
        CowFSNoSuchFileException {
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

    /// Port of `FileSystemException#getMessage()`.
    ///
    /// ```java
    /// if (file == null && other == null) return getReason();
    /// StringBuilder sb = new StringBuilder();
    /// if (file != null) sb.append(file);
    /// if (other != null) { sb.append(" -> "); sb.append(other); }
    /// if (getReason() != null) { sb.append(": "); sb.append(getReason()); }
    /// return sb.toString();
    /// ```
    ///
    /// With `other` and the reason both null that collapses to the file, so this returns it.
    pub fn get_message(&self) -> Option<&str> {
        self.get_file()
    }
}

impl std::fmt::Display for CowFSNoSuchFileException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for CowFSNoSuchFileException {}

/// `throw new CowFSNoSuchFileException(message)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("{}", CowFSNoSuchFileException::new(Some(message)))
}
