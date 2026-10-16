//! Port of: net/minecraft/client/PrioritizeChunkUpdates.java
//! Java class(es): net.minecraft.client.PrioritizeChunkUpdates
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, the private `id` field, the private `BY_ID` lookup
//! (`ByIdMap.continuous(.., WRAP)`, applied by `LEGACY_CODEC`'s int mapping), and the caption key.
//! `Component` is not ported, so the getters that return the translated `Component` are
//! PORT-BLOCKED. The translation KEY each one is built from is ported as a `*_key()` getter and is
//! checked against the dump, which records the component as `translation{key='...', args=[]}`.
//! `LEGACY_CODEC` itself is PORT-BLOCKED on DataFixerUpper; its two mapping functions are
//! [`by_id`] and [`PrioritizeChunkUpdates::id`].
//!
//! `WRAP` means a corrupt `options.txt` integer such as `-1` or `3` reads back as a valid
//! constant (`floorMod`), never as an error.

/// Port of `net.minecraft.client.PrioritizeChunkUpdates`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrioritizeChunkUpdates {
    /// `NONE` -- ordinal 0.
    None = 0,
    /// `PLAYER_AFFECTED` -- ordinal 1.
    PlayerAffected = 1,
    /// `NEARBY` -- ordinal 2.
    Nearby = 2,
}

/// Java: `PrioritizeChunkUpdates.values()`, declaration order.
pub const VALUES: [PrioritizeChunkUpdates; 3] = [
    PrioritizeChunkUpdates::None,
    PrioritizeChunkUpdates::PlayerAffected,
    PrioritizeChunkUpdates::Nearby,
];

impl PrioritizeChunkUpdates {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PrioritizeChunkUpdates::None => "NONE",
            PrioritizeChunkUpdates::PlayerAffected => "PLAYER_AFFECTED",
            PrioritizeChunkUpdates::Nearby => "NEARBY",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: the private `id` field (what `LEGACY_CODEC` writes).
    pub const fn id(&self) -> i32 {
        match self {
            PrioritizeChunkUpdates::None => 0,
            PrioritizeChunkUpdates::PlayerAffected => 1,
            PrioritizeChunkUpdates::Nearby => 2,
        }
    }

    /// The translation key `caption` is built from (`Component.translatable(key)`).
    pub const fn caption_key(&self) -> &'static str {
        match self {
            PrioritizeChunkUpdates::None => "options.prioritizeChunkUpdates.none",
            PrioritizeChunkUpdates::PlayerAffected => "options.prioritizeChunkUpdates.byPlayer",
            PrioritizeChunkUpdates::Nearby => "options.prioritizeChunkUpdates.nearby",
        }
    }

    /// Java: `public Component caption()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn caption(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }
}

impl std::fmt::Display for PrioritizeChunkUpdates {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

static BY_ID: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<PrioritizeChunkUpdates>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            PrioritizeChunkUpdates::id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Wrap,
        )
    });

/// `BY_ID.apply(id)` -- the decode half of `LEGACY_CODEC` (`Codec.INT.xmap(BY_ID::apply, s -> s.id)`).
pub fn by_id(id: i32) -> PrioritizeChunkUpdates {
    BY_ID.apply(id)
}

/// Java: `public static final Codec<PrioritizeChunkUpdates> LEGACY_CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn LEGACY_CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
