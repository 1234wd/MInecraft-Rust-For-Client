//! Port of: net/minecraft/world/effect/MobEffectCategory.java
//! Java class(es): net.minecraft.world.effect.MobEffectCategory
//! Status: VERIFIED
//!
//! Three constants, each carrying a `ChatFormatting` for tooltip colouring. Verified against the
//! jar by `_porting/tests/consts.rs`: ordinals plus the `tooltipFormatting` field for every
//! constant.
//!
//! # `TASK` AND `GOAL` SHARE `GREEN` -- AND SO DO `BENEFICIAL` AND `NEUTRAL`
//!
//! The jar records `BENEFICIAL.tooltipFormatting = BLUE` and `NEUTRAL.tooltipFormatting = BLUE`.
//! They are different constants with the same formatting -- it is not a typo. Similarly,
//! `HARMFUL` is `RED`. This matches `AdvancementType`'s pattern where two constants share the
//! same colour value.
//!
//! # `getTooltipFormatting` IS PORT-BLOCKED ON `ChatFormatting`
//!
//! `net.minecraft.ChatFormatting` is still a SKELETON. The method signature is documented here;
//! once `ChatFormatting` is ported, `get_tooltip_formatting` can be completed. The dump confirms
//! the field values: `BENEFICIAL` and `NEUTRAL` → `"BLUE"`, `HARMFUL` → `"RED"`.

/// Port of `net.minecraft.world.effect.MobEffectCategory`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MobEffectCategory {
    /// Port of `MobEffectCategory#BENEFICIAL` -- ordinal 0, `tooltipFormatting = ChatFormatting.BLUE`.
    Beneficial = 0,
    /// Port of `MobEffectCategory#HARMFUL` -- ordinal 1, `tooltipFormatting = ChatFormatting.RED`.
    Harmful = 1,
    /// Port of `MobEffectCategory#NEUTRAL` -- ordinal 2, `tooltipFormatting = ChatFormatting.BLUE`.
    ///
    /// Note: `NEUTRAL` and `BENEFICIAL` share the same `BLUE` formatting. This is intentional
    /// in the Java source; the neutral tooltip colour deliberately matches the beneficial one.
    Neutral = 2,
}

/// Java: `MobEffectCategory.values()` -- declaration order.
pub const VALUES: [MobEffectCategory; 3] = [
    MobEffectCategory::Beneficial,
    MobEffectCategory::Harmful,
    MobEffectCategory::Neutral,
];

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public ChatFormatting getTooltipFormatting()`.
///
/// PORT-BLOCKED on `net.minecraft.ChatFormatting`, which is still a SKELETON.
/// When `ChatFormatting` is ported, this should return:
/// - `Beneficial` and `Neutral` → `ChatFormatting::Blue`
/// - `Harmful` → `ChatFormatting::Red`
pub fn get_tooltip_formatting(_cat: MobEffectCategory) {
    todo!("PORT-BLOCKED: net.minecraft.ChatFormatting is not ported")
}
