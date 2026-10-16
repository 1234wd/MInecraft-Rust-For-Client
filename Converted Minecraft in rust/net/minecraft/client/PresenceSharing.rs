//! Port of: net/minecraft/client/PresenceSharing.java
//! Java class(es): net.minecraft.client.PresenceSharing
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, `TRANSLATION_KEY_BASE`, serialized names, the two component keys.
//! `Component` is not ported, so the getters that return the translated `Component` are
//! PORT-BLOCKED. The translation KEY each one is built from is ported as a `*_key()` getter and is
//! checked against the dump, which records the component as `translation{key='...', args=[]}`.
//! `CODEC` is PORT-BLOCKED on DataFixerUpper.

/// Java: `public static final String TRANSLATION_KEY_BASE`.
pub const TRANSLATION_KEY_BASE: &str = "options.sharePresence";

/// Port of `net.minecraft.client.PresenceSharing`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PresenceSharing {
    /// `NONE` -- ordinal 0.
    None = 0,
    /// `LIMITED` -- ordinal 1.
    Limited = 1,
    /// `ALL` -- ordinal 2.
    All = 2,
}

/// Java: `PresenceSharing.values()`, declaration order.
pub const VALUES: [PresenceSharing; 3] = [
    PresenceSharing::None,
    PresenceSharing::Limited,
    PresenceSharing::All,
];

impl PresenceSharing {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PresenceSharing::None => "NONE",
            PresenceSharing::Limited => "LIMITED",
            PresenceSharing::All => "ALL",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `getSerializedName()`.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            PresenceSharing::None => "none",
            PresenceSharing::Limited => "limited",
            PresenceSharing::All => "all",
        }
    }

    /// The key the private `translatable` component is built from: `"options.sharePresence." + name`.
    pub const fn translation_key(&self) -> &'static str {
        match self {
            PresenceSharing::None => "options.sharePresence.none",
            PresenceSharing::Limited => "options.sharePresence.limited",
            PresenceSharing::All => "options.sharePresence.all",
        }
    }

    /// The key the private `tooltip` component is built from.
    pub const fn tooltip_key(&self) -> &'static str {
        match self {
            PresenceSharing::None => "options.sharePresence.none.tooltip",
            PresenceSharing::Limited => "options.sharePresence.limited.tooltip",
            PresenceSharing::All => "options.sharePresence.all.tooltip",
        }
    }

    /// Java: `public Component getTranslation()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_translation(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Java: `public Component getTooltip()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_tooltip(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }
}

impl std::fmt::Display for PresenceSharing {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for PresenceSharing {
    fn get_serialized_name(&self) -> &str {
        PresenceSharing::get_serialized_name(self)
    }
}

static CODEC_BY_NAME: std::sync::LazyLock<
    crate::net::minecraft::util::StringRepresentable::EnumCodec<PresenceSharing>,
> = std::sync::LazyLock::new(|| crate::net::minecraft::util::StringRepresentable::from_enum(&VALUES));

/// The name lookup behind `CODEC = StringRepresentable.fromEnum(values)` (the field's static type
/// is a bare `Codec`, so vanilla callers only reach it through decoding).
pub fn by_serialized_name(name: &str) -> Option<PresenceSharing> {
    CODEC_BY_NAME.by_name(name)
}

/// Java: `public static final Codec<PresenceSharing> CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
