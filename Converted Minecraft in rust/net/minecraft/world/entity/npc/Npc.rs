//! Port of: net/minecraft/world/entity/npc/Npc.java
//! Java class(es): net.minecraft.world.entity.npc.Npc
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A marker interface. `EntityType` tests `instanceof Npc` to pick the villager renderer and
//! the trading behaviour; no method is ever called through it.

/// Port of `net.minecraft.world.entity.npc.Npc`.
///
/// Empty on purpose -- see the module docs.
pub trait Npc {}