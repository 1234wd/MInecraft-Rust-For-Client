//! Port of: net/minecraft/world/level/MoonPhase.java
//! Java class(es): net.minecraft.world.level.MoonPhase
//! Status: PARTIAL
//!
//! Session 19. The eight constants with their `index` and `name` fields, `COUNT`,
//! `PHASE_LENGTH`, `index()` and `startTick()` are ported. Ordinals and both fields are checked
//! against the jar in `_porting/tests/session19_c.rs`; `COUNT` / `PHASE_LENGTH` are not in the
//! dump (enum groups carry only the constants) and are pinned from the Java source.
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec`).
//!
//! # PHASE 0 IS THE FULL MOON
//!
//! The cycle starts at `FULL_MOON` and *wanes* first. `startTick()` is `index * 24000` written
//! with the literal `24000` (not `PHASE_LENGTH`), so the last phase starts at tick 168000; no
//! overflow is possible.

/// Port of `net.minecraft.world.level.MoonPhase`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MoonPhase {
    /// Port of `MoonPhase#FULL_MOON` -- ordinal 0, index `0`, name `"full_moon"`.
    FullMoon = 0,
    /// Port of `MoonPhase#WANING_GIBBOUS` -- ordinal 1, index `1`, name `"waning_gibbous"`.
    WaningGibbous = 1,
    /// Port of `MoonPhase#THIRD_QUARTER` -- ordinal 2, index `2`, name `"third_quarter"`.
    ThirdQuarter = 2,
    /// Port of `MoonPhase#WANING_CRESCENT` -- ordinal 3, index `3`, name `"waning_crescent"`.
    WaningCrescent = 3,
    /// Port of `MoonPhase#NEW_MOON` -- ordinal 4, index `4`, name `"new_moon"`.
    NewMoon = 4,
    /// Port of `MoonPhase#WAXING_CRESCENT` -- ordinal 5, index `5`, name `"waxing_crescent"`.
    WaxingCrescent = 5,
    /// Port of `MoonPhase#FIRST_QUARTER` -- ordinal 6, index `6`, name `"first_quarter"`.
    FirstQuarter = 6,
    /// Port of `MoonPhase#WAXING_GIBBOUS` -- ordinal 7, index `7`, name `"waxing_gibbous"`.
    WaxingGibbous = 7,
}

/// Java: `MoonPhase.values()` -- declaration order.
pub const VALUES: [MoonPhase; 8] = [
    MoonPhase::FullMoon,
    MoonPhase::WaningGibbous,
    MoonPhase::ThirdQuarter,
    MoonPhase::WaningCrescent,
    MoonPhase::NewMoon,
    MoonPhase::WaxingCrescent,
    MoonPhase::FirstQuarter,
    MoonPhase::WaxingGibbous,
];

/// Java: `public static final int COUNT = values().length`.
pub const COUNT: i32 = VALUES.len() as i32;

/// Java: `public static final int PHASE_LENGTH = 24000`.
pub const PHASE_LENGTH: i32 = 24000;

impl MoonPhase {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            MoonPhase::FullMoon => "FULL_MOON",
            MoonPhase::WaningGibbous => "WANING_GIBBOUS",
            MoonPhase::ThirdQuarter => "THIRD_QUARTER",
            MoonPhase::WaningCrescent => "WANING_CRESCENT",
            MoonPhase::NewMoon => "NEW_MOON",
            MoonPhase::WaxingCrescent => "WAXING_CRESCENT",
            MoonPhase::FirstQuarter => "FIRST_QUARTER",
            MoonPhase::WaxingGibbous => "WAXING_GIBBOUS",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `MoonPhase#index()` -- the private `index` field (equal to the ordinal today,
    /// but a separate field in Java).
    #[inline]
    pub const fn index(&self) -> i32 {
        match self {
            MoonPhase::FullMoon => 0,
            MoonPhase::WaningGibbous => 1,
            MoonPhase::ThirdQuarter => 2,
            MoonPhase::WaningCrescent => 3,
            MoonPhase::NewMoon => 4,
            MoonPhase::WaxingCrescent => 5,
            MoonPhase::FirstQuarter => 6,
            MoonPhase::WaxingGibbous => 7,
        }
    }

    /// Port of `MoonPhase#startTick()`: `this.index * 24000`.
    #[inline]
    pub const fn start_tick(&self) -> i32 {
        self.index() * 24000
    }

    /// Port of `MoonPhase#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            MoonPhase::FullMoon => "full_moon",
            MoonPhase::WaningGibbous => "waning_gibbous",
            MoonPhase::ThirdQuarter => "third_quarter",
            MoonPhase::WaningCrescent => "waning_crescent",
            MoonPhase::NewMoon => "new_moon",
            MoonPhase::WaxingCrescent => "waxing_crescent",
            MoonPhase::FirstQuarter => "first_quarter",
            MoonPhase::WaxingGibbous => "waxing_gibbous",
        }
    }
}

impl std::fmt::Display for MoonPhase {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for MoonPhase {
    fn get_serialized_name(&self) -> &str {
        MoonPhase::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<MoonPhase> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
