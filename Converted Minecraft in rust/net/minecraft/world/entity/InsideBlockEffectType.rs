//! Port of: net/minecraft/world/entity/InsideBlockEffectType.java
//! Java class(es): net.minecraft.world.entity.InsideBlockEffectType
//! Status: PARTIAL
//!
//! Session 19. The five constants are ported and their names and ordinals are checked against
//! the jar in `_porting/tests/session19_b.rs` (the dump also shows each constant holds its own,
//! distinct lambda).
//!
//! PORT-BLOCKED: `effect()` returns a `Consumer<Entity>`; `net.minecraft.world.entity.Entity`
//! and `net.minecraft.world.level.block.BaseFireBlock` are SKELETONs. What each consumer does,
//! for the porter:
//! - `FREEZE`: `setIsInPowderSnow(true)`; then, if `canFreeze()`,
//!   `setTicksFrozen(Math.min(getTicksRequiredToFreeze(), getTicksFrozen() + 1))`.
//! - `CLEAR_FREEZE`: `Entity::clearFreeze`.
//! - `FIRE_IGNITE`: `BaseFireBlock::fireIgnite`.
//! - `LAVA_IGNITE`: `Entity::lavaIgnite`.
//! - `EXTINGUISH`: `Entity::clearFire`.
//!
//! The declaration order is the order the effects are applied in
//! (`InsideBlockEffectApplier.StepBasedCollector.APPLY_ORDER = values()`, flushed per movement
//! step), so within one step `EXTINGUISH` runs after both ignites. Do not reorder the variants.

/// Port of `net.minecraft.world.entity.InsideBlockEffectType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InsideBlockEffectType {
    /// Port of `InsideBlockEffectType#FREEZE` -- ordinal 0.
    Freeze = 0,
    /// Port of `InsideBlockEffectType#CLEAR_FREEZE` -- ordinal 1.
    ClearFreeze = 1,
    /// Port of `InsideBlockEffectType#FIRE_IGNITE` -- ordinal 2.
    FireIgnite = 2,
    /// Port of `InsideBlockEffectType#LAVA_IGNITE` -- ordinal 3.
    LavaIgnite = 3,
    /// Port of `InsideBlockEffectType#EXTINGUISH` -- ordinal 4.
    Extinguish = 4,
}

/// Java: `InsideBlockEffectType.values()` -- declaration order.
pub const VALUES: [InsideBlockEffectType; 5] = [
    InsideBlockEffectType::Freeze,
    InsideBlockEffectType::ClearFreeze,
    InsideBlockEffectType::FireIgnite,
    InsideBlockEffectType::LavaIgnite,
    InsideBlockEffectType::Extinguish,
];

impl InsideBlockEffectType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            InsideBlockEffectType::Freeze => "FREEZE",
            InsideBlockEffectType::ClearFreeze => "CLEAR_FREEZE",
            InsideBlockEffectType::FireIgnite => "FIRE_IGNITE",
            InsideBlockEffectType::LavaIgnite => "LAVA_IGNITE",
            InsideBlockEffectType::Extinguish => "EXTINGUISH",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `public Consumer<Entity> effect()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.world.entity.Entity` (and, for `FIRE_IGNITE`,
    /// `net.minecraft.world.level.block.BaseFireBlock`). See the module docs for each body.
    pub fn effect(&self) {
        todo!("PORT-BLOCKED: net.minecraft.world.entity.Entity is not ported")
    }
}

impl std::fmt::Display for InsideBlockEffectType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
