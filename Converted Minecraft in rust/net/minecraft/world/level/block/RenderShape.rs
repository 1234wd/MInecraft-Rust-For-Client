//! Port of: net/minecraft/world/level/block/RenderShape.java
//! Java class(es): net.minecraft.world.level.block.RenderShape
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A two-constant enum returned by `BlockState#getRenderShape`. `INVISIBLE` means the block
//! contributes no model at all (air, barriers, structure voids); `MODEL` means the block model
//! layer renders it. The two constants are mutually exhaustive, so this is a closed choice
//! rather than a list.

/// Port of `net.minecraft.world.level.block.RenderShape`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RenderShape {
    /// Port of `RenderShape#INVISIBLE` -- ordinal 0.
    Invisible = 0,
    /// Port of `RenderShape#MODEL` -- ordinal 1.
    Model = 1,
}

/// Java: `RenderShape.values()`.
pub const VALUES: [RenderShape; 2] = [RenderShape::Invisible, RenderShape::Model];