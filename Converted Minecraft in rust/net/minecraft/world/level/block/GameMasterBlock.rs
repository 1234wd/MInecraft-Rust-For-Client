//! Port of: net/minecraft/world/level/block/GameMasterBlock.java
//! Java class(es): net.minecraft.world.level.block.GameMasterBlock
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A marker interface. `GameMode` checks `instanceof GameMasterBlock` to allow the
//! `gamemode/command` family of block entities on creative-only blocks. Nothing is called
//! through it.

/// Port of `net.minecraft.world.level.block.GameMasterBlock`.
///
/// Empty on purpose -- see the module docs.
pub trait GameMasterBlock {}