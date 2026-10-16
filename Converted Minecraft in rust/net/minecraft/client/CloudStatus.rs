//! Port of: net/minecraft/client/CloudStatus.java
//! Java class(es): net.minecraft.client.CloudStatus
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, serialized names, caption keys.
//! `Component` is not ported, so the getters that return the translated `Component` are
//! PORT-BLOCKED. The translation KEY each one is built from is ported as a `*_key()` getter and is
//! checked against the dump, which records the component as `translation{key='...', args=[]}`.
//! `CODEC` is PORT-BLOCKED on DataFixerUpper.
//!
//! The serialized names are LEGACY booleans: `OFF` is `"false"` and `FANCY` is `"true"`, because
//! `options.txt` once stored clouds as a boolean. Only `FAST` has a descriptive name.

/// Port of `net.minecraft.client.CloudStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CloudStatus {
    /// `OFF` -- ordinal 0.
    Off = 0,
    /// `FAST` -- ordinal 1.
    Fast = 1,
    /// `FANCY` -- ordinal 2.
    Fancy = 2,
}

/// Java: `CloudStatus.values()`, declaration order.
pub const VALUES: [CloudStatus; 3] = [
    CloudStatus::Off,
    CloudStatus::Fast,
    CloudStatus::Fancy,
];

impl CloudStatus {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            CloudStatus::Off => "OFF",
            CloudStatus::Fast => "FAST",
            CloudStatus::Fancy => "FANCY",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `getSerializedName()` (the `legacyName` field).
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            CloudStatus::Off => "false",
            CloudStatus::Fast => "fast",
            CloudStatus::Fancy => "true",
        }
    }

    /// The translation key `caption` is built from.
    pub const fn caption_key(&self) -> &'static str {
        match self {
            CloudStatus::Off => "options.off",
            CloudStatus::Fast => "options.clouds.fast",
            CloudStatus::Fancy => "options.clouds.fancy",
        }
    }

    /// Java: `public Component caption()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn caption(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }
}

impl std::fmt::Display for CloudStatus {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for CloudStatus {
    fn get_serialized_name(&self) -> &str {
        CloudStatus::get_serialized_name(self)
    }
}

static CODEC_BY_NAME: std::sync::LazyLock<
    crate::net::minecraft::util::StringRepresentable::EnumCodec<CloudStatus>,
> = std::sync::LazyLock::new(|| crate::net::minecraft::util::StringRepresentable::from_enum(&VALUES));

/// The name lookup behind `CODEC = StringRepresentable.fromEnum(values)` (the field's static type
/// is a bare `Codec`, so vanilla callers only reach it through decoding).
pub fn by_serialized_name(name: &str) -> Option<CloudStatus> {
    CODEC_BY_NAME.by_name(name)
}

/// Java: `public static final Codec<CloudStatus> CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
