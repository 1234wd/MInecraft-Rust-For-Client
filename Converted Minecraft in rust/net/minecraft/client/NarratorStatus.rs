//! Port of: net/minecraft/client/NarratorStatus.java
//! Java class(es): net.minecraft.client.NarratorStatus
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, `getId`, `byId` (`ByIdMap.continuous(.., WRAP)`), the three
//! `shouldNarrate*` predicates, and the key behind `getName()`.
//! `Component` is not ported, so the getters that return the translated `Component` are
//! PORT-BLOCKED. The translation KEY each one is built from is ported as a `*_key()` getter and is
//! checked against the dump, which records the component as `translation{key='...', args=[]}`.
//! `LEGACY_CODEC` is PORT-BLOCKED on DataFixerUpper.
//!
//! `ALL` narrates both chat and system messages; `shouldNarrateSystemOrChat` is true for every
//! constant except `OFF`.

/// Port of `net.minecraft.client.NarratorStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NarratorStatus {
    /// `OFF` -- ordinal 0.
    Off = 0,
    /// `ALL` -- ordinal 1.
    All = 1,
    /// `CHAT` -- ordinal 2.
    Chat = 2,
    /// `SYSTEM` -- ordinal 3.
    System = 3,
}

/// Java: `NarratorStatus.values()`, declaration order.
pub const VALUES: [NarratorStatus; 4] = [
    NarratorStatus::Off,
    NarratorStatus::All,
    NarratorStatus::Chat,
    NarratorStatus::System,
];

impl NarratorStatus {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            NarratorStatus::Off => "OFF",
            NarratorStatus::All => "ALL",
            NarratorStatus::Chat => "CHAT",
            NarratorStatus::System => "SYSTEM",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `NarratorStatus#getId()`.
    pub const fn get_id(&self) -> i32 {
        match self {
            NarratorStatus::Off => 0,
            NarratorStatus::All => 1,
            NarratorStatus::Chat => 2,
            NarratorStatus::System => 3,
        }
    }

    /// The translation key the private `name` component is built from.
    pub const fn name_key(&self) -> &'static str {
        match self {
            NarratorStatus::Off => "options.narrator.off",
            NarratorStatus::All => "options.narrator.all",
            NarratorStatus::Chat => "options.narrator.chat",
            NarratorStatus::System => "options.narrator.system",
        }
    }

    /// Java: `public Component getName()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_name(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Port of `NarratorStatus#shouldNarrateChat()`.
    pub const fn should_narrate_chat(&self) -> bool {
        matches!(self, NarratorStatus::All | NarratorStatus::Chat)
    }

    /// Port of `NarratorStatus#shouldNarrateSystem()`.
    pub const fn should_narrate_system(&self) -> bool {
        matches!(self, NarratorStatus::All | NarratorStatus::System)
    }

    /// Port of `NarratorStatus#shouldNarrateSystemOrChat()`.
    pub const fn should_narrate_system_or_chat(&self) -> bool {
        matches!(self, NarratorStatus::All | NarratorStatus::System | NarratorStatus::Chat)
    }
}

impl std::fmt::Display for NarratorStatus {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

static BY_ID: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<NarratorStatus>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            NarratorStatus::get_id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Wrap,
        )
    });

/// Port of `NarratorStatus#byId(int)`.
pub fn by_id(id: i32) -> NarratorStatus {
    BY_ID.apply(id)
}

/// Java: `public static final Codec<NarratorStatus> LEGACY_CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn LEGACY_CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
