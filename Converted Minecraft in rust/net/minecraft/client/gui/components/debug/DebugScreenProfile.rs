//! Port of: net/minecraft/client/gui/components/debug/DebugScreenProfile.java
//! Java class(es): net.minecraft.client.gui.components.debug.DebugScreenProfile
//! Status: PARTIAL
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`. Both string fields are checked too.
//!
//! `CODEC` is an `EnumCodec`: its name lookup (`CODEC().by_name(..)`) is ported via
//! `StringRepresentable::from_enum`; its `encode`/`decode` are PORT-BLOCKED inside
//! `StringRepresentable.rs`, so this file is `PARTIAL`.

/// Port of `net.minecraft.client.gui.components.debug.DebugScreenProfile`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DebugScreenProfile {
    /// `DEFAULT` -- ordinal 0.
    Default = 0,
    /// `PERFORMANCE` -- ordinal 1.
    Performance = 1,
}

/// Java: `DebugScreenProfile.values()`, declaration order.
pub const VALUES: [DebugScreenProfile; 2] = [
    DebugScreenProfile::Default,
    DebugScreenProfile::Performance,
];

impl DebugScreenProfile {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            DebugScreenProfile::Default => "DEFAULT",
            DebugScreenProfile::Performance => "PERFORMANCE",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `DebugScreenProfile#getSerializedName()` (the `name` field).
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            DebugScreenProfile::Default => "default",
            DebugScreenProfile::Performance => "performance",
        }
    }

    /// Port of `DebugScreenProfile#translationKey()`.
    pub const fn translation_key(&self) -> &'static str {
        match self {
            DebugScreenProfile::Default => "debug.options.profile.default",
            DebugScreenProfile::Performance => "debug.options.profile.performance",
        }
    }
}

impl std::fmt::Display for DebugScreenProfile {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for DebugScreenProfile {
    fn get_serialized_name(&self) -> &str {
        DebugScreenProfile::get_serialized_name(self)
    }
}

static CODEC_INSTANCE: std::sync::LazyLock<
    crate::net::minecraft::util::StringRepresentable::EnumCodec<DebugScreenProfile>,
> = std::sync::LazyLock::new(|| crate::net::minecraft::util::StringRepresentable::from_enum(&VALUES));

/// Java: `public static final EnumCodec<DebugScreenProfile> CODEC`.
pub fn CODEC() -> &'static crate::net::minecraft::util::StringRepresentable::EnumCodec<DebugScreenProfile> {
    &CODEC_INSTANCE
}
