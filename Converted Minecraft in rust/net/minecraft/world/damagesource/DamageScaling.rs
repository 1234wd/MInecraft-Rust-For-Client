//! Port of: net/minecraft/world/damagesource/DamageScaling.java
//! Java class(es): net.minecraft.world.damagesource.DamageScaling
//! Status: VERIFIED
//!
//! Three constants, each carrying a `String id`. Verified against the jar by
//! `_porting/tests/consts.rs`: ordinals plus the `id` field for every constant.
//!
//! # ORDINALS DO NOT MATCH ALPHABETICAL ORDER
//!
//! Declaration order (and therefore ordinal order) is `NEVER(0)`, `WHEN_CAUSED_BY_LIVING_NON_PLAYER(1)`,
//! `ALWAYS(2)`. The jar confirms this; `ALWAYS` is NOT ordinal 0 despite coming first
//! alphabetically.
//!
//! # `CODEC` IS PORT-BLOCKED
//!
//! `com.mojang.serialization.Codec` (via `StringRepresentable.fromEnum`) is not ported.
//! See DESIGN_DECISIONS.md (#dfu-proposal).

/// Port of `net.minecraft.world.damagesource.DamageScaling`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DamageScaling {
    /// Port of `DamageScaling#NEVER` -- ordinal 0, id `"never"`.
    Never = 0,
    /// Port of `DamageScaling#WHEN_CAUSED_BY_LIVING_NON_PLAYER` -- ordinal 1,
    /// id `"when_caused_by_living_non_player"`.
    WhenCausedByLivingNonPlayer = 1,
    /// Port of `DamageScaling#ALWAYS` -- ordinal 2, id `"always"`.
    Always = 2,
}

/// Java: `DamageScaling.values()` -- declaration order.
pub const VALUES: [DamageScaling; 3] = [
    DamageScaling::Never,
    DamageScaling::WhenCausedByLivingNonPlayer,
    DamageScaling::Always,
];

impl DamageScaling {
    /// Port of `DamageScaling#getSerializedName()` -- the `id` field, used in damage-type tags.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            DamageScaling::Never => "never",
            DamageScaling::WhenCausedByLivingNonPlayer => "when_caused_by_living_non_player",
            DamageScaling::Always => "always",
        }
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<DamageScaling> CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`. See DESIGN_DECISIONS.md (#dfu-proposal).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
