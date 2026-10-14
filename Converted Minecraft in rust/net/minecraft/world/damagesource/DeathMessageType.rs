//! Port of: net/minecraft/world/damagesource/DeathMessageType.java
//! Java class(es): net.minecraft.world.damagesource.DeathMessageType
//! Status: VERIFIED
//!
//! Three constants, each carrying a `String id`. Verified against the jar by
//! `_porting/tests/consts.rs`: ordinals plus the `id` field for every constant.
//!
//! # `INTENTIONAL_GAME_DESIGN` REFERS TO THE NETHER PORTAL BUG
//!
//! This constant drives a special death-message path for damage sources tagged
//! `intentional_game_design` in the data pack -- a vanilla joke about a well-known
//! Nether portal suffocation edge case. Its id is `"intentional_game_design"` in the jar.
//!
//! # `CODEC` IS PORT-BLOCKED
//!
//! `com.mojang.serialization.Codec` (via `StringRepresentable.fromEnum`) is not ported.
//! See DESIGN_DECISIONS.md (#dfu-proposal).

/// Port of `net.minecraft.world.damagesource.DeathMessageType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeathMessageType {
    /// Port of `DeathMessageType#DEFAULT` -- ordinal 0, id `"default"`.
    Default = 0,
    /// Port of `DeathMessageType#FALL_VARIANTS` -- ordinal 1, id `"fall_variants"`.
    FallVariants = 1,
    /// Port of `DeathMessageType#INTENTIONAL_GAME_DESIGN` -- ordinal 2,
    /// id `"intentional_game_design"`.
    IntentionalGameDesign = 2,
}

/// Java: `DeathMessageType.values()` -- declaration order.
pub const VALUES: [DeathMessageType; 3] = [
    DeathMessageType::Default,
    DeathMessageType::FallVariants,
    DeathMessageType::IntentionalGameDesign,
];

impl DeathMessageType {
    /// Port of `DeathMessageType#getSerializedName()` -- the `id` field, used in damage-type tags.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            DeathMessageType::Default => "default",
            DeathMessageType::FallVariants => "fall_variants",
            DeathMessageType::IntentionalGameDesign => "intentional_game_design",
        }
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<DeathMessageType> CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`. See DESIGN_DECISIONS.md (#dfu-proposal).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
