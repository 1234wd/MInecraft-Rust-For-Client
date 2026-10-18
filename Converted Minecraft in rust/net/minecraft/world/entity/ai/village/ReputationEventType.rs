//! Port of: net/minecraft/world/entity/ai/village/ReputationEventType.java
//! Java class(es): net.minecraft.world.entity.ai.village.ReputationEventType, ReputationEventType::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 22. `public interface ReputationEventType`: the kinds of event that change a villager's
//! gossip about a player. No methods; five constants, each made by the static `register(name)`,
//! which returns an anonymous instance whose `toString()` is the name.
//!
//! # Identity, not value
//!
//! The anonymous class has no `equals`, so the constants are distinguished by IDENTITY (probed:
//! `TRADE == TRADE`, `TRADE.equals(GOLEM_KILLED)` is false). Here they are `static`s of type
//! [`Registered`], one address each; no `PartialEq` is derived, and callers compare with
//! `std::ptr::eq`, as Java does with `==`. `register` takes a `&'static str` so its result can
//! initialise a `static`.
//!
//! Verified by the `REPEVENT` and `REPEVENTSAME` groups of `small_probe.txt`
//! (`tests/session22.rs`), which list the interface's constants by reflection.

/// Port of `net.minecraft.world.entity.ai.village.ReputationEventType`.
pub trait ReputationEventType: std::fmt::Display {}

/// The anonymous class `ReputationEventType.register(name)` returns.
#[derive(Debug)]
pub struct Registered {
    name: &'static str,
}

impl ReputationEventType for Registered {}

impl std::fmt::Display for Registered {
    /// Port of the anonymous class's `toString()`: the registered name.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

/// Port of `static ReputationEventType register(String name)`.
pub const fn register(name: &'static str) -> Registered {
    Registered { name }
}

/// Port of `ReputationEventType.ZOMBIE_VILLAGER_CURED`.
pub static ZOMBIE_VILLAGER_CURED: Registered = register("zombie_villager_cured");
/// Port of `ReputationEventType.GOLEM_KILLED`.
pub static GOLEM_KILLED: Registered = register("golem_killed");
/// Port of `ReputationEventType.VILLAGER_HURT`.
pub static VILLAGER_HURT: Registered = register("villager_hurt");
/// Port of `ReputationEventType.VILLAGER_KILLED`.
pub static VILLAGER_KILLED: Registered = register("villager_killed");
/// Port of `ReputationEventType.TRADE`.
pub static TRADE: Registered = register("trade");
