//! Port of: net/minecraft/server/level/ParticleStatus.java
//! Java class(es): net.minecraft.server.level.ParticleStatus
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the three constants with `id`, the private `BY_ID` (as
//! [`by_id`]) and the caption's translation key. Constants, `id` and the `caption`'s key are
//! checked against the jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `caption()` on `net.minecraft.network.chat.Component`; `LEGACY_CODEC`
//! (`Codec.INT.xmap(BY_ID::apply, s -> s.id)`) on DataFixerUpper.
//!
//! # `BY_ID` WRAPS
//!
//! `OutOfBoundsStrategy.WRAP`: an out-of-range options value decodes by `Math.floorMod(id, 3)`
//! -- `3` is `ALL`, `-1` is `MINIMAL`.

use crate::net::minecraft::util::ByIdMap;

/// Port of `net.minecraft.server.level.ParticleStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParticleStatus {
    /// Port of `ParticleStatus#ALL` -- ordinal 0, id 0.
    All = 0,
    /// Port of `ParticleStatus#DECREASED` -- ordinal 1, id 1.
    Decreased = 1,
    /// Port of `ParticleStatus#MINIMAL` -- ordinal 2, id 2.
    Minimal = 2,
}

/// Java: `ParticleStatus.values()` -- declaration order.
pub const VALUES: [ParticleStatus; 3] = [ParticleStatus::All, ParticleStatus::Decreased, ParticleStatus::Minimal];

impl ParticleStatus {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ParticleStatus::All => "ALL",
            ParticleStatus::Decreased => "DECREASED",
            ParticleStatus::Minimal => "MINIMAL",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `id` field (what `LEGACY_CODEC` encodes).
    pub const fn id(&self) -> i32 {
        match self {
            ParticleStatus::All => 0,
            ParticleStatus::Decreased => 1,
            ParticleStatus::Minimal => 2,
        }
    }

    /// The constructor's `key` argument. Java does not keep it as a field; it goes straight into
    /// `caption = Component.translatable(key)`. Exposed so `caption()` can be built once
    /// `Component` is ported.
    pub const fn caption_translation_key(&self) -> &'static str {
        match self {
            ParticleStatus::All => "options.particles.all",
            ParticleStatus::Decreased => "options.particles.decreased",
            ParticleStatus::Minimal => "options.particles.minimal",
        }
    }
}

impl std::fmt::Display for ParticleStatus {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

static BY_ID: std::sync::LazyLock<ByIdMap::Continuous<ParticleStatus>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(ParticleStatus::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Wrap)
});

/// Java: `private static final IntFunction<ParticleStatus> BY_ID`, applied (the decoding half of
/// `LEGACY_CODEC`).
///
/// `OutOfBoundsStrategy.WRAP`: `Math.floorMod(id, 3)`.
pub fn by_id(id: i32) -> ParticleStatus {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public Component caption()`.
///
/// PORT-BLOCKED on `net.minecraft.network.chat.Component`. The key is
/// [`ParticleStatus::caption_translation_key`].
pub fn caption(_s: ParticleStatus) {
    todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
}

/// Java: `public static final Codec<ParticleStatus> LEGACY_CODEC = Codec.INT.xmap(BY_ID::apply, s -> s.id)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`. Decoding is [`by_id`], encoding is
/// [`ParticleStatus::id`].
pub fn LEGACY_CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
