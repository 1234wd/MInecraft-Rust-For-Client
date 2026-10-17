//! Port of: net/minecraft/world/level/saveddata/SavedData.java
//! Java class(es): net.minecraft.world.level.saveddata.SavedData
//! Status: VERIFIED
//!
//! Session 22. `public abstract class SavedData`: the base of every per-world data file (maps,
//! raids, the scoreboard ...). All it contributes is a `dirty` flag, `false` at construction.
//!
//! # Port shape
//!
//! An abstract class with STATE has no Rust trait equivalent (a trait cannot hold the field), so
//! this is a struct that each ported subclass embeds and delegates to. It has no abstract methods
//! to translate -- the class is `abstract` only so it cannot be instantiated on its own.
//!
//! `setDirty()` is `setDirty(true)`; per `DESIGN_DECISIONS.md#overload-naming` the two are
//! [`SavedData::set_dirty`] and [`SavedData::set_dirty_bool`].
//!
//! Verified by the `SAVEDDATA` group of `small_probe.txt` (`tests/session22.rs`), on an anonymous
//! subclass driven through an eight-step script.

/// Port of `net.minecraft.world.level.saveddata.SavedData`'s state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SavedData {
    dirty: bool,
}

impl SavedData {
    /// Port of the implicit constructor: not dirty.
    pub const fn new() -> Self {
        SavedData { dirty: false }
    }

    /// Port of `setDirty()`: `setDirty(true)`.
    pub fn set_dirty(&mut self) {
        self.set_dirty_bool(true);
    }

    /// Port of `setDirty(boolean dirty)`.
    pub fn set_dirty_bool(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    /// Port of `isDirty()`.
    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }
}
