//! Port of: net/minecraft/world/entity/monster/piglin/PiglinArmPose.java
//! Java class(es): net.minecraft.world.entity.monster.piglin.PiglinArmPose
//! Status: VERIFIED
//!
//! Session 19. A plain six-constant enum (the arm pose a piglin renders with). Names and
//! ordinals are checked against the jar in `_porting/tests/session19_b.rs`.

/// Port of `net.minecraft.world.entity.monster.piglin.PiglinArmPose`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PiglinArmPose {
    /// Port of `PiglinArmPose#ATTACKING_WITH_MELEE_WEAPON` -- ordinal 0.
    AttackingWithMeleeWeapon = 0,
    /// Port of `PiglinArmPose#CROSSBOW_HOLD` -- ordinal 1.
    CrossbowHold = 1,
    /// Port of `PiglinArmPose#CROSSBOW_CHARGE` -- ordinal 2.
    CrossbowCharge = 2,
    /// Port of `PiglinArmPose#ADMIRING_ITEM` -- ordinal 3.
    AdmiringItem = 3,
    /// Port of `PiglinArmPose#DANCING` -- ordinal 4.
    Dancing = 4,
    /// Port of `PiglinArmPose#DEFAULT` -- ordinal 5.
    Default = 5,
}

/// Java: `PiglinArmPose.values()` -- declaration order.
pub const VALUES: [PiglinArmPose; 6] = [
    PiglinArmPose::AttackingWithMeleeWeapon,
    PiglinArmPose::CrossbowHold,
    PiglinArmPose::CrossbowCharge,
    PiglinArmPose::AdmiringItem,
    PiglinArmPose::Dancing,
    PiglinArmPose::Default,
];

impl PiglinArmPose {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PiglinArmPose::AttackingWithMeleeWeapon => "ATTACKING_WITH_MELEE_WEAPON",
            PiglinArmPose::CrossbowHold => "CROSSBOW_HOLD",
            PiglinArmPose::CrossbowCharge => "CROSSBOW_CHARGE",
            PiglinArmPose::AdmiringItem => "ADMIRING_ITEM",
            PiglinArmPose::Dancing => "DANCING",
            PiglinArmPose::Default => "DEFAULT",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for PiglinArmPose {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
