//! Port of: net/minecraft/world/inventory/SlotRange.java
//! Java class(es): net.minecraft.world.inventory.SlotRange, SlotRange::<1 anonymous class(es)>
//! Status: PORTED
//!
//! Session 19 batch d. Ported: the interface (`slots()`, default `size()`, inherited
//! `getSerializedName()`) as the [`SlotRange`] trait, and `SlotRange.of(name, slots)` with its
//! anonymous implementation ([`SlotRangeOf`], whose `toString()` is the name). Nothing is
//! PORT-BLOCKED. Not `VERIFIED`: the interface declares no constants of its own (the dump group
//! only holds the inherited `StringRepresentable.PRE_BUILT_MAP_THRESHOLD`, which the test checks
//! is all there is), so there is no golden data for its behaviour.
//!
//! # `IntList` IS `Vec<i32>`, AND OWNERSHIP REPLACES ALIASING
//!
//! Java's `of` keeps the caller's `IntList` *reference*, so a later mutation of that list by the
//! caller would show through `slots()`. The only vanilla caller, `SlotRanges`, passes
//! `IntLists.singleton`, `IntLists.unmodifiable(...)` or `IntList.of(...)` and never mutates the
//! backing list afterwards, so the port takes ownership of the `Vec`.
//!
//! The anonymous class does not override `equals`/`hashCode`, so Java compares ranges by
//! identity; the port deliberately does not derive `PartialEq`.

use crate::net::minecraft::util::StringRepresentable::StringRepresentable;

/// Port of the `net.minecraft.world.inventory.SlotRange` interface.
pub trait SlotRange: StringRepresentable {
    /// Java: `IntList slots()`.
    fn slots(&self) -> &[i32];

    /// Java: `default int size()` -- `this.slots().size()`.
    fn size(&self) -> i32 {
        self.slots().len() as i32
    }
}

/// The anonymous class returned by `SlotRange.of(String, IntList)`.
#[derive(Clone, Debug)]
pub struct SlotRangeOf {
    name: String,
    slots: Vec<i32>,
}

/// Port of `SlotRange#of(String, IntList)`.
pub fn of(name: impl Into<String>, slots: Vec<i32>) -> SlotRangeOf {
    SlotRangeOf {
        name: name.into(),
        slots,
    }
}

impl SlotRange for SlotRangeOf {
    /// Returns the list given to `of`.
    fn slots(&self) -> &[i32] {
        &self.slots
    }
}

impl StringRepresentable for SlotRangeOf {
    /// Returns the name given to `of`.
    fn get_serialized_name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for SlotRangeOf {
    /// The anonymous class's `toString()`: the name given to `of`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
