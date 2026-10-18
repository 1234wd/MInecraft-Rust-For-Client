//! Port of: net/minecraft/nbt/visitors/FieldSelector.java
//! Java class(es): net.minecraft.nbt.visitors.FieldSelector
//! Status: VERIFIED
//!
//! Session 22. `public record FieldSelector(List<String> path, TagType<?> type, String name)`: which
//! NBT field a selective reader should collect -- a path of compound names, the expected tag type
//! and the field's name -- with three convenience constructors for paths of length 0, 1 and 2.
//!
//! The constructors build the path with `List.of(...)`, which is IMMUTABLE (probed: `add` throws).
//! Here the path is a `Vec<String>` behind a read-only accessor.
//!
//! # `equals`, and the members that are not portable
//!
//! `equals` is `List#equals` on the path (element-wise), `TagType` by IDENTITY (each tag type is
//! one anonymous singleton, so by kind here, as [`TagTypeRef`] already models it) and `String#equals`
//! on the name.
//!
//! `hashCode` and `toString` are NOT ported: both fold in the `TagType`, which is an anonymous
//! class with `Object`'s identity `hashCode` and a `toString` like `net.minecraft.nbt.IntTag$1@6d06d69c`.
//! Both change on every JVM run, so there is no Java value to match; the probe deliberately writes
//! neither. The derived Rust `Hash` is consistent with `equals`.
//!
//! Verified by the `FIELDSEL` group of `small_probe.txt` (`tests/session22.rs`): all four
//! constructors, the path, the tag type id, the name, path immutability and the `equals` matrix.

use crate::net::minecraft::nbt::TagType::TagTypeRef;

/// Port of `net.minecraft.nbt.visitors.FieldSelector`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldSelector {
    path: Vec<String>,
    tag_type: TagTypeRef,
    name: String,
}

impl FieldSelector {
    /// Port of the canonical constructor `FieldSelector(List<String> path, TagType<?> type, String name)`.
    pub fn new(path: &[&str], tag_type: TagTypeRef, name: &str) -> Self {
        FieldSelector {
            path: path.iter().map(|s| s.to_string()).collect(),
            tag_type,
            name: name.to_string(),
        }
    }

    /// Port of `FieldSelector(TagType<?> type, String name)`: an empty path.
    pub fn of_name(tag_type: TagTypeRef, name: &str) -> Self {
        FieldSelector::new(&[], tag_type, name)
    }

    /// Port of `FieldSelector(String parent, TagType<?> type, String name)`.
    pub fn in_parent(parent: &str, tag_type: TagTypeRef, name: &str) -> Self {
        FieldSelector::new(&[parent], tag_type, name)
    }

    /// Port of `FieldSelector(String grandparent, String parent, TagType<?> type, String name)`.
    pub fn in_grandparent(grandparent: &str, parent: &str, tag_type: TagTypeRef, name: &str) -> Self {
        FieldSelector::new(&[grandparent, parent], tag_type, name)
    }

    /// Port of the accessor `path()`. Read-only, as Java's `List.of` list is.
    pub fn path(&self) -> &[String] {
        &self.path
    }

    /// Port of the accessor `type()`.
    pub fn tag_type(&self) -> TagTypeRef {
        self.tag_type
    }

    /// Port of the accessor `name()`.
    pub fn name(&self) -> &str {
        &self.name
    }
}
