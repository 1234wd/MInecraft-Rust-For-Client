//! Port of: net/minecraft/world/entity/EntitySpawnReason.java
//! Java class(es): net.minecraft.world.entity.EntitySpawnReason
//! Status: VERIFIED
//!
//! Session 19. Nineteen constants plus the static predicates `isSpawner` and
//! `ignoresLightRequirements`. Names and ordinals are checked against the jar in
//! `_porting/tests/session19_b.rs`.
//!
//! `isSpawner` is true for `SPAWNER` **and** `TRIAL_SPAWNER`; `ignoresLightRequirements` is
//! true for `TRIAL_SPAWNER` only. Both are explicit equality tests, not ordinal ranges.

/// Port of `net.minecraft.world.entity.EntitySpawnReason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntitySpawnReason {
    /// Port of `EntitySpawnReason#NATURAL` -- ordinal 0.
    Natural = 0,
    /// Port of `EntitySpawnReason#CHUNK_GENERATION` -- ordinal 1.
    ChunkGeneration = 1,
    /// Port of `EntitySpawnReason#SPAWNER` -- ordinal 2.
    Spawner = 2,
    /// Port of `EntitySpawnReason#STRUCTURE` -- ordinal 3.
    Structure = 3,
    /// Port of `EntitySpawnReason#BREEDING` -- ordinal 4.
    Breeding = 4,
    /// Port of `EntitySpawnReason#MOB_SUMMONED` -- ordinal 5.
    MobSummoned = 5,
    /// Port of `EntitySpawnReason#JOCKEY` -- ordinal 6.
    Jockey = 6,
    /// Port of `EntitySpawnReason#EVENT` -- ordinal 7.
    Event = 7,
    /// Port of `EntitySpawnReason#CONVERSION` -- ordinal 8.
    Conversion = 8,
    /// Port of `EntitySpawnReason#REINFORCEMENT` -- ordinal 9.
    Reinforcement = 9,
    /// Port of `EntitySpawnReason#TRIGGERED` -- ordinal 10.
    Triggered = 10,
    /// Port of `EntitySpawnReason#BUCKET` -- ordinal 11.
    Bucket = 11,
    /// Port of `EntitySpawnReason#SPAWN_ITEM_USE` -- ordinal 12.
    SpawnItemUse = 12,
    /// Port of `EntitySpawnReason#COMMAND` -- ordinal 13.
    Command = 13,
    /// Port of `EntitySpawnReason#DISPENSER` -- ordinal 14.
    Dispenser = 14,
    /// Port of `EntitySpawnReason#PATROL` -- ordinal 15.
    Patrol = 15,
    /// Port of `EntitySpawnReason#TRIAL_SPAWNER` -- ordinal 16.
    TrialSpawner = 16,
    /// Port of `EntitySpawnReason#LOAD` -- ordinal 17.
    Load = 17,
    /// Port of `EntitySpawnReason#DIMENSION_TRAVEL` -- ordinal 18.
    DimensionTravel = 18,
}

/// Java: `EntitySpawnReason.values()` -- declaration order.
pub const VALUES: [EntitySpawnReason; 19] = [
    EntitySpawnReason::Natural,
    EntitySpawnReason::ChunkGeneration,
    EntitySpawnReason::Spawner,
    EntitySpawnReason::Structure,
    EntitySpawnReason::Breeding,
    EntitySpawnReason::MobSummoned,
    EntitySpawnReason::Jockey,
    EntitySpawnReason::Event,
    EntitySpawnReason::Conversion,
    EntitySpawnReason::Reinforcement,
    EntitySpawnReason::Triggered,
    EntitySpawnReason::Bucket,
    EntitySpawnReason::SpawnItemUse,
    EntitySpawnReason::Command,
    EntitySpawnReason::Dispenser,
    EntitySpawnReason::Patrol,
    EntitySpawnReason::TrialSpawner,
    EntitySpawnReason::Load,
    EntitySpawnReason::DimensionTravel,
];

impl EntitySpawnReason {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            EntitySpawnReason::Natural => "NATURAL",
            EntitySpawnReason::ChunkGeneration => "CHUNK_GENERATION",
            EntitySpawnReason::Spawner => "SPAWNER",
            EntitySpawnReason::Structure => "STRUCTURE",
            EntitySpawnReason::Breeding => "BREEDING",
            EntitySpawnReason::MobSummoned => "MOB_SUMMONED",
            EntitySpawnReason::Jockey => "JOCKEY",
            EntitySpawnReason::Event => "EVENT",
            EntitySpawnReason::Conversion => "CONVERSION",
            EntitySpawnReason::Reinforcement => "REINFORCEMENT",
            EntitySpawnReason::Triggered => "TRIGGERED",
            EntitySpawnReason::Bucket => "BUCKET",
            EntitySpawnReason::SpawnItemUse => "SPAWN_ITEM_USE",
            EntitySpawnReason::Command => "COMMAND",
            EntitySpawnReason::Dispenser => "DISPENSER",
            EntitySpawnReason::Patrol => "PATROL",
            EntitySpawnReason::TrialSpawner => "TRIAL_SPAWNER",
            EntitySpawnReason::Load => "LOAD",
            EntitySpawnReason::DimensionTravel => "DIMENSION_TRAVEL",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

/// Port of `EntitySpawnReason#isSpawner(EntitySpawnReason)`:
/// `reason == SPAWNER || reason == TRIAL_SPAWNER`.
#[inline]
pub const fn is_spawner(reason: EntitySpawnReason) -> bool {
    matches!(reason, EntitySpawnReason::Spawner | EntitySpawnReason::TrialSpawner)
}

/// Port of `EntitySpawnReason#ignoresLightRequirements(EntitySpawnReason)`:
/// `reason == TRIAL_SPAWNER`.
#[inline]
pub const fn ignores_light_requirements(reason: EntitySpawnReason) -> bool {
    matches!(reason, EntitySpawnReason::TrialSpawner)
}

impl std::fmt::Display for EntitySpawnReason {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
