//! Port of: net/minecraft/nbt/visitors/CollectFields.java
//! Java class(es): net.minecraft.nbt.visitors.CollectFields
//! Status: VERIFIED
//!
//! Session 23. `public class CollectFields extends CollectToTag`: collects ONLY the fields its
//! `FieldSelector`s name (and the compounds on the way to them), and stops reading as soon as it
//! has them all.
//!
//! * The root must be a compound: `visitRootEntry` answers `HALT` for any other type.
//! * At the depth of the current `FieldTree` frame, `visitEntry(type)` answers `BREAK` once every
//!   wanted field has been seen (`fieldsToGetCount <= 0`), `SKIP` for a type no selector wants
//!   (`COMPOUND` is always wanted, to reach nested fields), and `CollectToTag`'s `ENTER` otherwise.
//!   Deeper than the frame -- inside a field that is being collected -- everything is collected.
//! * `visitEntry(type, id)` at frame depth: a selected `(id, type)` is REMOVED from the frame
//!   (`selectedFields().remove(id, type)` -- the type must match) and counted down; a compound on a
//!   selector's path pushes its frame; anything else is `SKIP`.
//! * `visitContainerEnd` pops the frame when the container it belongs to ends.
//! * `getMissingFieldCount()` is what is left of the count: selectors that matched nothing.
//!
//! # Port shape
//!
//! Java's `Deque<FieldTree>` holds references INTO the tree it built, and `remove` mutates a
//! frame through one. Here the root tree is owned and the deque holds each frame's PATH from the
//! root; [`FieldTree::descend`] turns a path back into the frame. The tree is never restructured
//! after construction, so a path stays valid. `ArrayDeque.push`/`element`/`pop` work at the head,
//! which is a `Vec`'s tail here. `ImmutableSet<TagType<?>>` is only asked `contains`, so it is a
//! `Vec`.
//!
//! The superclass is the `base: CollectToTag` field; methods this class does not override forward
//! to it, and `super.x(..)` is `self.base.x(..)`.
//!
//! Verified by the `CF<n>` rows of every `GEN`/`CASE` tree in `_porting/test-data/nbt_probe.txt`
//! (`NbtIo.parse` into a `CollectFields` over each of the probe's three selector sets: the
//! collected tag's bytes and the missing-field count), in `tests/session23.rs`.

use crate::net::minecraft::nbt::visitors::CollectToTag::{forward_collect_to_tag, CollectToTag};
use crate::net::minecraft::nbt::visitors::FieldSelector::FieldSelector;
use crate::net::minecraft::nbt::visitors::FieldTree::FieldTree;
use crate::net::minecraft::nbt::CompoundTag;
use crate::net::minecraft::nbt::StreamTagVisitor::{EntryResult, StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagType::TagTypeRef;

/// Port of `net.minecraft.nbt.visitors.CollectFields`.
#[derive(Debug, Clone)]
pub struct CollectFields {
    base: CollectToTag,
    fields_to_get_count: i32,
    wanted_types: Vec<TagTypeRef>,
    root: FieldTree,
    /// The frames' paths from `root`; the last is the deque's head.
    stack: Vec<Vec<String>>,
}

impl CollectFields {
    /// Port of `new CollectFields(FieldSelector...)`.
    pub fn new(wanted_fields: &[FieldSelector]) -> Self {
        let fields_to_get_count = wanted_fields.len() as i32;
        let mut wanted_types: Vec<TagTypeRef> = Vec::new();
        let mut root_frame = FieldTree::create_root();
        for wanted_field in wanted_fields {
            root_frame.add_entry(wanted_field);
            wanted_types.push(wanted_field.tag_type());
        }
        wanted_types.push(CompoundTag::TYPE);
        CollectFields {
            base: CollectToTag::new(),
            fields_to_get_count,
            wanted_types,
            root: root_frame,
            stack: vec![Vec::new()],
        }
    }

    /// `this.stack.element()`: the frame at the head.
    fn current_frame(&self) -> &FieldTree {
        let path = self
            .stack
            .last()
            .unwrap_or_else(|| panic!("java.util.NoSuchElementException"));
        self.root.descend(path)
    }

    /// Port of `getMissingFieldCount()`.
    pub fn get_missing_field_count(&self) -> i32 {
        self.fields_to_get_count
    }

    /// Port of the inherited `getResult()`.
    pub fn get_result(&self) -> Option<Tag> {
        self.base.get_result()
    }
}

impl StreamTagVisitor for CollectFields {
    /// Port of `visitRootEntry(TagType)`: `HALT` unless the root is a compound.
    fn visit_root_entry(&mut self, tag_type: &TagTypeRef) -> ValueResult {
        if *tag_type != CompoundTag::TYPE {
            ValueResult::Halt
        } else {
            self.base.visit_root_entry(tag_type)
        }
    }

    /// Port of `visitEntry(TagType)`.
    fn visit_entry(&mut self, tag_type: &TagTypeRef) -> EntryResult {
        let current_depth = self.current_frame().depth();
        if self.base.depth() > current_depth {
            self.base.visit_entry(tag_type)
        } else if self.fields_to_get_count <= 0 {
            EntryResult::Break
        } else if !self.wanted_types.contains(tag_type) {
            EntryResult::Skip
        } else {
            self.base.visit_entry(tag_type)
        }
    }

    /// Port of `visitEntry(TagType, String)`.
    fn visit_entry_named(&mut self, tag_type: &TagTypeRef, id: &str) -> EntryResult {
        if self.base.depth() > self.current_frame().depth() {
            return self.base.visit_entry_named(tag_type, id);
        }
        let path = self.stack.last().expect("a frame").clone();
        let frame = self.root.descend_mut(&path);
        // `selectedFields().remove(id, type)`: only when `id` maps to exactly this type.
        if frame.selected_fields().get(id) == Some(tag_type) {
            frame.selected_fields_mut().remove(id);
            self.fields_to_get_count -= 1;
            return self.base.visit_entry_named(tag_type, id);
        }
        if *tag_type == CompoundTag::TYPE && frame.fields_to_recurse().contains_key(id) {
            let mut next = path;
            next.push(id.to_string());
            self.stack.push(next);
            return self.base.visit_entry_named(tag_type, id);
        }
        EntryResult::Skip
    }

    /// Port of `visitContainerEnd()`: pop the frame whose container just ended.
    fn visit_container_end(&mut self) -> ValueResult {
        if self.base.depth() == self.current_frame().depth() {
            self.stack.pop();
        }
        self.base.visit_container_end()
    }

    forward_collect_to_tag! {
        visit_end() -> ValueResult;
        visit_string(value: &str) -> ValueResult;
        visit_byte(value: i8) -> ValueResult;
        visit_short(value: i16) -> ValueResult;
        visit_int(value: i32) -> ValueResult;
        visit_long(value: i64) -> ValueResult;
        visit_float(value: f32) -> ValueResult;
        visit_double(value: f64) -> ValueResult;
        visit_byte_array(value: &[i8]) -> ValueResult;
        visit_int_array(value: &[i32]) -> ValueResult;
        visit_long_array(value: &[i64]) -> ValueResult;
        visit_list(element_type: &TagTypeRef, size: i32) -> ValueResult;
        visit_element(tag_type: &TagTypeRef, index: i32) -> EntryResult;
    }
}
