//! Port of: net/minecraft/world/level/block/entity/LidBlockEntity.java
//! Java class(es): net.minecraft.world.level.block.entity.LidBlockEntity
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! # THE PARAMETER IS THE **OPENNESS**, NOT THE DELTA
//!
//! ```java
//! float getOpenNess(float a);
//! ```
//!
//! The name in the decompiled source is a single-letter `a`, which hides what it is: the
//! caller passes the progress towards open (the vanilla call site is the lid animation, which
//! supplies `0.0`..`1.0`), and the implementation returns a *rendering* offset -- `0.0` when
//! fully closed, `a` when fully open. The vanilla implementation is
//! `1.0F - MathHelper.clamp(a, 0.0F, 1.0F)` scaled by the lid height, so it is a `clamp` then a
//! subtraction, in that order.
//!
//! Naming the argument `openness` rather than `a` is the whole port here. The clamp and the
//! scale live in the implementor, not in this interface.

/// Port of `net.minecraft.world.level.block.entity.LidBlockEntity`.
pub trait LidBlockEntity {
    /// Port of `LidBlockEntity#getOpenNess(float)` -- `a` in the decompiled source.
    ///
    /// `openness` is the progress towards open, `0.0` closed to `1.0` open; the return value is
    /// the vertical offset to render the lid at.
    fn get_open_ness(&self, openness: f32) -> f32;
}