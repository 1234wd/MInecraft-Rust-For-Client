//! Port of: net/minecraft/nbt/StreamTagVisitor.java
//! Java class(es): net.minecraft.nbt.StreamTagVisitor, net.minecraft.nbt.EntryResult, net.minecraft.nbt.ValueResult
//! Status: PORTED
//!
//! Session 21. Both nested enums and all sixteen interface methods.
//!
//! This is the *pull* visitor: `TagType.parse` reads the stream and calls into it, and the
//! visitor's return value decides whether the reader keeps going, skips the current value,
//! or stops the whole traversal. That is what makes partial reads possible --
//! `NbtIo.readAndSkip` uses it to pull two fields out of a 2 MB chunk without building the
//! tree.
//!
//! # `visit` IS OVERLOADED TEN WAYS
//!
//! Java distinguishes `visit(byte)` from `visit(short)` from `visit(byte[])` by type.
//! `DESIGN_DECISIONS.md#overload-naming` gives these `_<type>` suffixes: [`visit_byte`],
//! [`visit_short`], [`visit_byte_array`] and so on. `visitEnd` and `visitContainerEnd` are
//! not overloaded and keep their plain names.
//!
//! [`visit_byte`]: StreamTagVisitor::visit_byte
//! [`visit_short`]: StreamTagVisitor::visit_short
//! [`visit_byte_array`]: StreamTagVisitor::visit_byte_array
//!
//! # The two result enums are NOT interchangeable
//!
//! `ValueResult` is `CONTINUE, BREAK, HALT`; `EntryResult` is `ENTER, SKIP, BREAK, HALT`.
//! `EntryResult` has the extra `ENTER`/`SKIP` pair because an entry can be declined without
//! ending the traversal. Both carry `BREAK` and `HALT` and they mean different things:
//! `BREAK` ends the current container, `HALT` ends everything. Keeping them as two Rust
//! enums rather than one shared enum is what stops an arm from being handled in the wrong
//! switch.
//!
//! Declaration order is the Java source order, so the discriminants match
//! `Enum#ordinal()`.

use crate::net::minecraft::nbt::TagType::TagTypeRef;

/// Port of `StreamTagVisitor.EntryResult`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntryResult {
    /// Port of `EntryResult.ENTER` (ordinal 0).
    Enter,
    /// Port of `EntryResult.SKIP` (ordinal 1).
    Skip,
    /// Port of `EntryResult.BREAK` (ordinal 2).
    Break,
    /// Port of `EntryResult.HALT` (ordinal 3).
    Halt,
}

/// Port of `StreamTagVisitor.ValueResult`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValueResult {
    /// Port of `ValueResult.CONTINUE` (ordinal 0).
    Continue,
    /// Port of `ValueResult.BREAK` (ordinal 1).
    Break,
    /// Port of `ValueResult.HALT` (ordinal 2).
    Halt,
}

/// Port of `net.minecraft.nbt.StreamTagVisitor`.
pub trait StreamTagVisitor {
    /// Port of `StreamTagVisitor#visitEnd()`.
    fn visit_end(&mut self) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(String)`.
    fn visit_string(&mut self, value: &str) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(byte)`.
    fn visit_byte(&mut self, value: i8) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(short)`.
    fn visit_short(&mut self, value: i16) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(int)`.
    fn visit_int(&mut self, value: i32) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(long)`.
    fn visit_long(&mut self, value: i64) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(float)`.
    fn visit_float(&mut self, value: f32) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(double)`.
    fn visit_double(&mut self, value: f64) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(byte[])`.
    fn visit_byte_array(&mut self, value: &[i8]) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(int[])`.
    fn visit_int_array(&mut self, value: &[i32]) -> ValueResult;

    /// Port of `StreamTagVisitor#visit(long[])`.
    fn visit_long_array(&mut self, value: &[i64]) -> ValueResult;

    /// Port of `StreamTagVisitor#visitList(TagType, int)`.
    fn visit_list(&mut self, element_type: &TagTypeRef, size: i32) -> ValueResult;

    /// Port of `StreamTagVisitor#visitEntry(TagType)`.
    fn visit_entry(&mut self, tag_type: &TagTypeRef) -> EntryResult;

    /// Port of `StreamTagVisitor#visitEntry(TagType, String)`.
    fn visit_entry_named(&mut self, tag_type: &TagTypeRef, id: &str) -> EntryResult;

    /// Port of `StreamTagVisitor#visitElement(TagType, int)`.
    fn visit_element(&mut self, tag_type: &TagTypeRef, index: i32) -> EntryResult;

    /// Port of `StreamTagVisitor#visitContainerEnd()`.
    fn visit_container_end(&mut self) -> ValueResult;

    /// Port of `StreamTagVisitor#visitRootEntry(TagType)`.
    fn visit_root_entry(&mut self, tag_type: &TagTypeRef) -> ValueResult;
}
