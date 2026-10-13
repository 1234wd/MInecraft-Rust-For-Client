//! Port of: net/minecraft/world/entity/ItemSteerable.java
//! Java class(es): net.minecraft.world.entity.ItemSteerable
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! One boolean method. `Entity#getSteering` calls it on saddled mobs, and a pig saddled by a
//! player returns true -- which is what makes the pig follow and the rider dismount. The
//! declaration carries no logic; the behaviour is in the implementors.

/// Port of `net.minecraft.world.entity.ItemSteerable`.
pub trait ItemSteerable {
    /// Port of `ItemSteerable#boost()`.
    fn boost(&self) -> bool;
}