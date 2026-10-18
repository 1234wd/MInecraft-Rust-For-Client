//! Port of: net/minecraft/util/filefix/access/FileResourceType.java
//! Java class(es): net.minecraft.util.filefix.access.FileResourceType, net.minecraft.util.filefix.access.AccessFactory
//! Status: VERIFIED
//!
//! Session 22. `public class FileResourceType<T extends AutoCloseable>`: a kind of file the file
//! fixer can open, wrapping the factory that opens one -- with the nested functional interface
//! `AccessFactory<T>` (`T create(Path path, int dataVersion)`).
//!
//! `create(path, dataVersion)` forwards both arguments to the factory, once per call, and returns
//! what it built. Java bounds `T` by `AutoCloseable`; Rust has no such trait, and drop is the
//! close, so the bound is not carried over. Any `Fn(&Path, i32) -> T` closure is an
//! `AccessFactory<T>`.
//!
//! Verified by the `FILERES` group of `small_probe.txt` (`tests/session22.rs`).

use std::path::Path;

/// Port of `FileResourceType.AccessFactory<T>`.
pub trait AccessFactory<T> {
    /// Port of `T create(Path path, int dataVersion)`.
    fn create(&self, path: &Path, data_version: i32) -> T;
}

impl<T, F: Fn(&Path, i32) -> T> AccessFactory<T> for F {
    fn create(&self, path: &Path, data_version: i32) -> T {
        self(path, data_version)
    }
}

/// Port of `net.minecraft.util.filefix.access.FileResourceType<T>`.
pub struct FileResourceType<T> {
    factory: Box<dyn AccessFactory<T>>,
}

impl<T> FileResourceType<T> {
    /// Port of `new FileResourceType(FileResourceType.AccessFactory<T> factory)`.
    pub fn new(factory: impl AccessFactory<T> + 'static) -> Self {
        FileResourceType {
            factory: Box::new(factory),
        }
    }

    /// Port of `create(Path path, int dataVersion)`: `factory.create(path, dataVersion)`.
    pub fn create(&self, path: &Path, data_version: i32) -> T {
        self.factory.create(path, data_version)
    }
}
