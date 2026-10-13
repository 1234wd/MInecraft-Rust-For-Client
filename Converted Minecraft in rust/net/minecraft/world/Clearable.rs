//! Port of: net/minecraft/world/Clearable.java
//! Java class(es): net.minecraft.world.Clearable
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! One method, no arguments beyond `this`, no return value. `Level#setBlocksDirty` and the
//! chest/barrel inventory code call it to empty a container. There is nothing to get wrong in
//! the *declaration*; the risk in this interface is entirely in the implementations, which are
//! ported with their own classes.

/// Port of `net.minecraft.world.Clearable`.
pub trait Clearable {
    /// Port of `Clearable#clearContent()`.
    fn clear_content(&mut self);
}