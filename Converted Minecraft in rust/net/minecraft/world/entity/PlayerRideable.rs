//! Port of: net/minecraft/world/entity/PlayerRideable.java
//! Java class(es): net.minecraft.world.entity.PlayerRideable
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A marker interface, like `SkipPacketException`: `Entity` code tests `instanceof
//! PlayerRideable` and nothing is called on it. The contract is entirely "is or is not of this
//! kind", so an empty trait is the whole port.

/// Port of `net.minecraft.world.entity.PlayerRideable`.
///
/// Empty on purpose -- see the module docs.
pub trait PlayerRideable {}