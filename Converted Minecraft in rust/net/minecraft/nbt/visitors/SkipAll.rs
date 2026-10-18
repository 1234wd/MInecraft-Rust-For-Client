//! Port of: net/minecraft/nbt/visitors/SkipAll.java
//! Java class(es): net.minecraft.nbt.visitors.SkipAll
//! Status: VERIFIED
//!
//! Session 22. `public interface SkipAll extends StreamTagVisitor`: a visitor that reads past a whole
//! tag -- every `visit*` answers `CONTINUE`, every `visitEntry`/`visitElement` answers `SKIP` --
//! with one shared `INSTANCE`.
//!
//! # Port shape
//!
//! In Java `SkipAll` supplies DEFAULTS for all seventeen `StreamTagVisitor` methods. A Rust trait
//! cannot default its supertrait's methods, and nothing but `INSTANCE` implements `SkipAll` in the
//! game, so the port is a unit struct that implements `StreamTagVisitor` with those seventeen
//! answers, and [`INSTANCE`] is that struct.
//!
//! Verified by the `SKIPALL` group of `small_probe.txt` (`tests/session22.rs`): all seventeen
//! answers of `SkipAll.INSTANCE`.

use crate::net::minecraft::nbt::StreamTagVisitor::{EntryResult, StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::TagType::TagTypeRef;

/// Port of `net.minecraft.nbt.visitors.SkipAll`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SkipAll;

/// Port of `SkipAll.INSTANCE`.
pub const INSTANCE: SkipAll = SkipAll;

impl StreamTagVisitor for SkipAll {
    fn visit_end(&mut self) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_string(&mut self, _value: &str) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_byte(&mut self, _value: i8) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_short(&mut self, _value: i16) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_int(&mut self, _value: i32) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_long(&mut self, _value: i64) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_float(&mut self, _value: f32) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_double(&mut self, _value: f64) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_byte_array(&mut self, _value: &[i8]) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_int_array(&mut self, _value: &[i32]) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_long_array(&mut self, _value: &[i64]) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_list(&mut self, _element_type: &TagTypeRef, _size: i32) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_entry(&mut self, _tag_type: &TagTypeRef) -> EntryResult {
        EntryResult::Skip
    }

    fn visit_entry_named(&mut self, _tag_type: &TagTypeRef, _id: &str) -> EntryResult {
        EntryResult::Skip
    }

    fn visit_element(&mut self, _tag_type: &TagTypeRef, _index: i32) -> EntryResult {
        EntryResult::Skip
    }

    fn visit_container_end(&mut self) -> ValueResult {
        ValueResult::Continue
    }

    fn visit_root_entry(&mut self, _tag_type: &TagTypeRef) -> ValueResult {
        ValueResult::Continue
    }
}
