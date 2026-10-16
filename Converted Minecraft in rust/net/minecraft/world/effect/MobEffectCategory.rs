//! Port of: net/minecraft/world/effect/MobEffectCategory.java
//! Java class(es): net.minecraft.world.effect.MobEffectCategory
//! Status: VERIFIED
//!
//! Three constants, each carrying a `ChatFormatting` for tooltip colouring. Verified against the
//! jar by `_porting/tests/consts.rs` (ordinals and the dumped `tooltipFormatting` names) and by
//! `_porting/tests/session19_b.rs`, which checks the ported `getTooltipFormatting()` values
//! against the same dump. `getTooltipFormatting` was PORT-BLOCKED until `ChatFormatting` was
//! ported in session 19; there are no blocked members left.
//!
//! # `BENEFICIAL` AND `NEUTRAL` SHARE `BLUE`
//!
//! The jar records `BENEFICIAL.tooltipFormatting = BLUE` and `NEUTRAL.tooltipFormatting = BLUE`.
//! They are different constants with the same formatting -- it is not a typo. `HARMFUL` is
//! `RED`. This matches `AdvancementType`, where `TASK` and `GOAL` are both `GREEN`.

use crate::net::minecraft::ChatFormatting::ChatFormatting;

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

impl MobEffectCategory {
    /// Port of `MobEffectCategory#getTooltipFormatting()`.
    ///
    /// `Beneficial` and `Neutral` are both `Blue`; `Harmful` is `Red`.
    #[inline]
    pub const fn get_tooltip_formatting(&self) -> ChatFormatting {
        match self {
            MobEffectCategory::Beneficial => ChatFormatting::Blue,
            MobEffectCategory::Harmful => ChatFormatting::Red,
            MobEffectCategory::Neutral => ChatFormatting::Blue,
        }
    }
}
