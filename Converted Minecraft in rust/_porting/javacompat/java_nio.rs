//! The narrow `java.nio.file` option subset used by Minecraft's file-fix code.
//!
//! This mirrors the JDK 25 marker relationships and enum constants needed by
//! `FileSystemCapabilities`; it does not emulate filesystem operations.

use crate::javacompat::throwable::{JavaThrowable, ThrowableRef};

/// Java's marker interface `java.nio.file.CopyOption`.
pub trait CopyOption {}

/// Java's marker interface `java.nio.file.OpenOption`.
pub trait OpenOption {}

/// `java.nio.file.LinkOption`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LinkOption {
    NOFOLLOW_LINKS,
}

impl CopyOption for LinkOption {}
impl OpenOption for LinkOption {}

impl LinkOption {
    pub const fn values() -> [Self; 1] {
        [Self::NOFOLLOW_LINKS]
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::NOFOLLOW_LINKS => "NOFOLLOW_LINKS",
        }
    }

    pub const fn ordinal(self) -> i32 {
        match self {
            Self::NOFOLLOW_LINKS => 0,
        }
    }

    pub fn value_of(name: Option<&str>) -> Result<Self, ThrowableRef> {
        match name {
            Some("NOFOLLOW_LINKS") => Ok(Self::NOFOLLOW_LINKS),
            Some(name) => Err(JavaThrowable::new(
                "java.lang.IllegalArgumentException",
                Some(format!("No enum constant java.nio.file.LinkOption.{name}")),
            )),
            None => Err(JavaThrowable::new(
                "java.lang.NullPointerException",
                Some("Name is null".to_string()),
            )),
        }
    }
}

impl std::fmt::Display for LinkOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// `java.nio.file.StandardCopyOption`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StandardCopyOption {
    REPLACE_EXISTING,
    COPY_ATTRIBUTES,
    ATOMIC_MOVE,
}

impl CopyOption for StandardCopyOption {}

impl StandardCopyOption {
    pub const fn values() -> [Self; 3] {
        [
            Self::REPLACE_EXISTING,
            Self::COPY_ATTRIBUTES,
            Self::ATOMIC_MOVE,
        ]
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::REPLACE_EXISTING => "REPLACE_EXISTING",
            Self::COPY_ATTRIBUTES => "COPY_ATTRIBUTES",
            Self::ATOMIC_MOVE => "ATOMIC_MOVE",
        }
    }

    pub const fn ordinal(self) -> i32 {
        match self {
            Self::REPLACE_EXISTING => 0,
            Self::COPY_ATTRIBUTES => 1,
            Self::ATOMIC_MOVE => 2,
        }
    }

    pub fn value_of(name: Option<&str>) -> Result<Self, ThrowableRef> {
        match name {
            Some("REPLACE_EXISTING") => Ok(Self::REPLACE_EXISTING),
            Some("COPY_ATTRIBUTES") => Ok(Self::COPY_ATTRIBUTES),
            Some("ATOMIC_MOVE") => Ok(Self::ATOMIC_MOVE),
            Some(name) => Err(JavaThrowable::new(
                "java.lang.IllegalArgumentException",
                Some(format!(
                    "No enum constant java.nio.file.StandardCopyOption.{name}"
                )),
            )),
            None => Err(JavaThrowable::new(
                "java.lang.NullPointerException",
                Some("Name is null".to_string()),
            )),
        }
    }
}

impl std::fmt::Display for StandardCopyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Owned sum type for a heterogeneous Java `CopyOption[]`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CopyOptionValue {
    Link(LinkOption),
    Standard(StandardCopyOption),
}

impl From<LinkOption> for CopyOptionValue {
    fn from(value: LinkOption) -> Self {
        Self::Link(value)
    }
}

impl From<StandardCopyOption> for CopyOptionValue {
    fn from(value: StandardCopyOption) -> Self {
        Self::Standard(value)
    }
}

impl std::fmt::Display for CopyOptionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Link(option) => option.fmt(f),
            Self::Standard(option) => option.fmt(f),
        }
    }
}

/// Narrow model of `java.nio.file.Path` used as a value type in records and interfaces.
///
/// Wraps `std::path::PathBuf` and provides Java-compatible `equals`, `hashCode`, and
/// `toString`. Does not emulate filesystem operations (resolution, normalization, etc.).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Path {
    inner: std::path::PathBuf,
}

impl Path {
    /// Port of `Path.of(String first, String... more)`.
    pub fn of(first: &str, more: &[&str]) -> Self {
        let mut p = std::path::PathBuf::from(first);
        for m in more {
            p.push(m);
        }
        Path { inner: p }
    }

    /// Port of `Path.toString()`.
    pub fn to_string(&self) -> String {
        self.inner.to_string_lossy().into_owned()
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.display())
    }
}
