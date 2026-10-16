//! Port of: net/minecraft/world/item/enchantment/EnchantmentTarget.java
//! Java class(es): net.minecraft.world.item.enchantment.EnchantmentTarget
//! Status: PARTIAL
//!
//! Session 19. The three constants and `getSerializedName()` are ported; ordinals and the `id`
//! field are checked against the jar in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED on DataFixerUpper `Codec`: `CODEC` and `NON_DAMAGE_CODEC`.
//!
//! # `NON_DAMAGE_CODEC` IS A TWO-VALUE SUBSET
//!
//! ```java
//! NON_DAMAGE_CODEC = StringRepresentable.fromEnum(() -> new EnchantmentTarget[]{ATTACKER, VICTIM});
//! ```
//!
//! It accepts `"attacker"` and `"victim"` only; `"damaging_entity"` does not decode through it.
//! Whoever ports it must build it from that two-element array, not from `VALUES`.

/// Port of `net.minecraft.world.item.enchantment.EnchantmentTarget`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnchantmentTarget {
    /// Port of `EnchantmentTarget#ATTACKER` -- ordinal 0, id `"attacker"`.
    Attacker = 0,
    /// Port of `EnchantmentTarget#DAMAGING_ENTITY` -- ordinal 1, id `"damaging_entity"`.
    DamagingEntity = 1,
    /// Port of `EnchantmentTarget#VICTIM` -- ordinal 2, id `"victim"`.
    Victim = 2,
}

/// Java: `EnchantmentTarget.values()` -- declaration order.
pub const VALUES: [EnchantmentTarget; 3] = [
    EnchantmentTarget::Attacker,
    EnchantmentTarget::DamagingEntity,
    EnchantmentTarget::Victim,
];

impl EnchantmentTarget {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            EnchantmentTarget::Attacker => "ATTACKER",
            EnchantmentTarget::DamagingEntity => "DAMAGING_ENTITY",
            EnchantmentTarget::Victim => "VICTIM",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `EnchantmentTarget#getSerializedName()` -- the private `id` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            EnchantmentTarget::Attacker => "attacker",
            EnchantmentTarget::DamagingEntity => "damaging_entity",
            EnchantmentTarget::Victim => "victim",
        }
    }
}

impl std::fmt::Display for EnchantmentTarget {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for EnchantmentTarget {
    fn get_serialized_name(&self) -> &str {
        EnchantmentTarget::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<EnchantmentTarget> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final Codec<EnchantmentTarget> NON_DAMAGE_CODEC` over `{ATTACKER, VICTIM}`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper). See the module docs.
pub fn NON_DAMAGE_CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
