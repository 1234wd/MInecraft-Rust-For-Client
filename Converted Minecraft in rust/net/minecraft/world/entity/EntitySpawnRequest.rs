//! Port of: net/minecraft/world/entity/EntitySpawnRequest.java
//! Java class(es): net.minecraft.world.entity.EntitySpawnRequest
//! Status: VERIFIED
//!
//! Session 22. `public record EntitySpawnRequest(EntitySpawnReason reason, boolean ignoreChecks)`
//! -- components only, no body.
//!
//! # `hashCode` is not portable, by design of the JVM
//!
//! The record hash folds `reason.hashCode()`, an enum's identity hash, which differs on every JVM
//! run. No `java_hash_code` is ported; the derived Rust `Hash` is consistent with `equals`, which
//! is all a `HashMap` needs. The probe writes `-` for these hashes.
//!
//! `equals` (enum by identity, i.e. by variant, and the boolean) is the derived one; `toString`
//! prints the enum's `name()` -- `EntitySpawnReason` does not override `toString`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`).

use crate::net::minecraft::world::entity::EntitySpawnReason::EntitySpawnReason;

/// Port of `net.minecraft.world.entity.EntitySpawnRequest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntitySpawnRequest {
    reason: EntitySpawnReason,
    ignore_checks: bool,
}

impl EntitySpawnRequest {
    /// Port of the canonical constructor.
    pub const fn new(reason: EntitySpawnReason, ignore_checks: bool) -> Self {
        EntitySpawnRequest { reason, ignore_checks }
    }

    /// Port of the accessor `reason()`.
    pub const fn reason(&self) -> EntitySpawnReason {
        self.reason
    }

    /// Port of the accessor `ignoreChecks()`.
    pub const fn ignore_checks(&self) -> bool {
        self.ignore_checks
    }
}

impl std::fmt::Display for EntitySpawnRequest {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EntitySpawnRequest[reason={}, ignoreChecks={}]",
            self.reason.name(),
            self.ignore_checks
        )
    }
}
