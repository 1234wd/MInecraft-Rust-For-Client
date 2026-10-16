//! Port of: net/minecraft/client/InactivityFpsLimit.java
//! Java class(es): net.minecraft.client.InactivityFpsLimit
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, serialized names, caption keys.
//! `Component` is not ported, so the getters that return the translated `Component` are
//! PORT-BLOCKED. The translation KEY each one is built from is ported as a `*_key()` getter and is
//! checked against the dump, which records the component as `translation{key='...', args=[]}`.
//! `CODEC` is PORT-BLOCKED on DataFixerUpper.

/// Port of `net.minecraft.client.InactivityFpsLimit`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InactivityFpsLimit {
    /// `MINIMIZED` -- ordinal 0.
    Minimized = 0,
    /// `AFK` -- ordinal 1.
    Afk = 1,
}

/// Java: `InactivityFpsLimit.values()`, declaration order.
pub const VALUES: [InactivityFpsLimit; 2] = [
    InactivityFpsLimit::Minimized,
    InactivityFpsLimit::Afk,
];

impl InactivityFpsLimit {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            InactivityFpsLimit::Minimized => "MINIMIZED",
            InactivityFpsLimit::Afk => "AFK",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `getSerializedName()`.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            InactivityFpsLimit::Minimized => "minimized",
            InactivityFpsLimit::Afk => "afk",
        }
    }

    /// The translation key `caption` is built from.
    pub const fn caption_key(&self) -> &'static str {
        match self {
            InactivityFpsLimit::Minimized => "options.inactivityFpsLimit.minimized",
            InactivityFpsLimit::Afk => "options.inactivityFpsLimit.afk",
        }
    }

    /// Java: `public Component caption()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn caption(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }
}

impl std::fmt::Display for InactivityFpsLimit {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for InactivityFpsLimit {
    fn get_serialized_name(&self) -> &str {
        InactivityFpsLimit::get_serialized_name(self)
    }
}

static CODEC_BY_NAME: std::sync::LazyLock<
    crate::net::minecraft::util::StringRepresentable::EnumCodec<InactivityFpsLimit>,
> = std::sync::LazyLock::new(|| crate::net::minecraft::util::StringRepresentable::from_enum(&VALUES));

/// The name lookup behind `CODEC = StringRepresentable.fromEnum(values)` (the field's static type
/// is a bare `Codec`, so vanilla callers only reach it through decoding).
pub fn by_serialized_name(name: &str) -> Option<InactivityFpsLimit> {
    CODEC_BY_NAME.by_name(name)
}

/// Java: `public static final Codec<InactivityFpsLimit> CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
