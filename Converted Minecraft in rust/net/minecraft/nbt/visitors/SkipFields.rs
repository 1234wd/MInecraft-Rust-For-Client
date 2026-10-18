//! Port of: net/minecraft/nbt/visitors/SkipFields.java
//! Java class(es): net.minecraft.nbt.visitors.SkipFields
//! Status: VERIFIED
//!
//! Session 23. `public class SkipFields extends CollectToTag`: the inverse of `CollectFields` --
//! collects EVERYTHING except the fields its `FieldSelector`s name.
//!
//! * `visitEntry(type, id)`: a selected `(id, type)` in the current frame answers `SKIP` (the
//!   frame is NOT changed, unlike `CollectFields`, so the same field is skipped wherever that frame
//!   applies); a compound on a selector's path pushes its frame; then `CollectToTag`'s answer.
//! * `visitContainerEnd` pops the frame when the container it belongs to ends.
//!
//! It overrides nothing else -- in particular not `visitRootEntry`, so a non-compound root is
//! collected whole.
//!
//! # Port shape
//!
//! As `CollectFields`: an owned `FieldTree`, the frame deque as paths into it, and the superclass
//! as a `base: CollectToTag` field that the other methods forward to.
//!
//! Verified by the `SF<n>` rows of every `GEN`/`CASE` tree in `_porting/test-data/nbt_probe.txt`
//! (`NbtIo.parse` into a `SkipFields` over each of the probe's three selector sets), in
//! `tests/session23.rs`.

use crate::net::minecraft::nbt::visitors::CollectToTag::{forward_collect_to_tag, CollectToTag};
use crate::net::minecraft::nbt::visitors::FieldSelector::FieldSelector;
use crate::net::minecraft::nbt::visitors::FieldTree::FieldTree;
use crate::net::minecraft::nbt::CompoundTag;
use crate::net::minecraft::nbt::StreamTagVisitor::{EntryResult, StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagType::TagTypeRef;

/// Port of `net.minecraft.nbt.visitors.SkipFields`.
#[derive(Debug, Clone)]
pub struct SkipFields {
    base: CollectToTag,
    root: FieldTree,
    /// The frames' paths from `root`; the last is the deque's head.
    stack: Vec<Vec<String>>,
}

impl SkipFields {
    /// Port of `new SkipFields(FieldSelector...)`.
    pub fn new(wanted_fields: &[FieldSelector]) -> Self {
        let mut root_frame = FieldTree::create_root();
        for wanted_field in wanted_fields {
            root_frame.add_entry(wanted_field);
        }
        SkipFields {
            base: CollectToTag::new(),
            root: root_frame,
            stack: vec![Vec::new()],
        }
    }

    /// `this.stack.element()`.
    fn current_frame(&self) -> &FieldTree {
        let path = self
            .stack
            .last()
            .unwrap_or_else(|| panic!("java.util.NoSuchElementException"));
        self.root.descend(path)
    }

    /// Port of the inherited `getResult()`.
    pub fn get_result(&self) -> Option<Tag> {
        self.base.get_result()
    }
}

impl StreamTagVisitor for SkipFields {
    /// Port of `visitEntry(TagType, String)`.
    fn visit_entry_named(&mut self, tag_type: &TagTypeRef, id: &str) -> EntryResult {
        let current = self.current_frame();
        if current.is_selected(tag_type, id) {
            return EntryResult::Skip;
        }
        if *tag_type == CompoundTag::TYPE && current.fields_to_recurse().contains_key(id) {
            let mut next = self.stack.last().expect("a frame").clone();
            next.push(id.to_string());
            self.stack.push(next);
        }
        self.base.visit_entry_named(tag_type, id)
    }

    /// Port of `visitContainerEnd()`.
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
        visit_entry(tag_type: &TagTypeRef) -> EntryResult;
        visit_root_entry(tag_type: &TagTypeRef) -> ValueResult;
    }
}
