//! Port of: net/minecraft/world/damagesource/DamageEffects.java
//! Java class(es): net.minecraft.world.damagesource.DamageEffects
//! Status: PARTIAL
//!
//! Session 19. The six constants with their `id` field and `getSerializedName()` are ported;
//! ordinals and `id` are checked against the jar in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED:
//! * `sound()` on `net.minecraft.sounds.SoundEvent` / `SoundEvents` (SKELETON). The jar
//!   records `HURT` and `THORNS` both as `entity.player.hurt` (not a typo: thorns damage plays
//!   the plain hurt sound), `DROWNING` `hurt_drown`, `BURNING` `hurt_on_fire`, `POKING`
//!   `hurt_sweet_berry_bush`, `FREEZING` `hurt_freeze`.
//! * `CODEC` on DataFixerUpper `Codec`.

/// Port of `net.minecraft.world.damagesource.DamageEffects`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DamageEffects {
    /// Port of `DamageEffects#HURT` -- ordinal 0, id `"hurt"`, `PLAYER_HURT`.
    Hurt = 0,
    /// Port of `DamageEffects#THORNS` -- ordinal 1, id `"thorns"`, `PLAYER_HURT`.
    Thorns = 1,
    /// Port of `DamageEffects#DROWNING` -- ordinal 2, id `"drowning"`, `PLAYER_HURT_DROWN`.
    Drowning = 2,
    /// Port of `DamageEffects#BURNING` -- ordinal 3, id `"burning"`, `PLAYER_HURT_ON_FIRE`.
    Burning = 3,
    /// Port of `DamageEffects#POKING` -- ordinal 4, id `"poking"`,
    /// `PLAYER_HURT_SWEET_BERRY_BUSH`.
    Poking = 4,
    /// Port of `DamageEffects#FREEZING` -- ordinal 5, id `"freezing"`, `PLAYER_HURT_FREEZE`.
    Freezing = 5,
}

/// Java: `DamageEffects.values()` -- declaration order.
pub const VALUES: [DamageEffects; 6] = [
    DamageEffects::Hurt,
    DamageEffects::Thorns,
    DamageEffects::Drowning,
    DamageEffects::Burning,
    DamageEffects::Poking,
    DamageEffects::Freezing,
];

impl DamageEffects {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            DamageEffects::Hurt => "HURT",
            DamageEffects::Thorns => "THORNS",
            DamageEffects::Drowning => "DROWNING",
            DamageEffects::Burning => "BURNING",
            DamageEffects::Poking => "POKING",
            DamageEffects::Freezing => "FREEZING",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `DamageEffects#getSerializedName()` -- the private `id` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            DamageEffects::Hurt => "hurt",
            DamageEffects::Thorns => "thorns",
            DamageEffects::Drowning => "drowning",
            DamageEffects::Burning => "burning",
            DamageEffects::Poking => "poking",
            DamageEffects::Freezing => "freezing",
        }
    }

    /// Java: `public SoundEvent sound()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.sounds.SoundEvent` (and the `SoundEvents.PLAYER_HURT*`
    /// constants), which are SKELETON. See the module docs for the values.
    pub fn sound(&self) {
        todo!("PORT-BLOCKED: net.minecraft.sounds.SoundEvent is not ported")
    }
}

impl std::fmt::Display for DamageEffects {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for DamageEffects {
    fn get_serialized_name(&self) -> &str {
        DamageEffects::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<DamageEffects> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
