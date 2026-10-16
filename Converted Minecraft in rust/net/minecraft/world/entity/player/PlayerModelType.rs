//! Port of: net/minecraft/world/entity/player/PlayerModelType.java
//! Java class(es): net.minecraft.world.entity.player.PlayerModelType
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the two constants with `id` and `legacyServicesId`,
//! `NAME_LOOKUP`, `byLegacyServicesName`, `getSerializedName`. Constants and both fields are
//! checked against the jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `CODEC` on DataFixerUpper, `STREAM_CODEC` on netty `ByteBuf`.
//!
//! # TWO SPELLINGS: `"wide"` VS `"default"`
//!
//! The serialized name of `WIDE` is `"wide"`, but the legacy services (skin texture metadata)
//! spelling is `"default"`. `byLegacyServicesName` looks up the *legacy* spelling only, so
//! `"wide"` is unknown there and -- like `null` or any other unknown string -- falls back to
//! `WIDE` anyway. Only the exact string `"slim"` yields `SLIM`.

use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.entity.player.PlayerModelType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayerModelType {
    /// Port of `PlayerModelType#SLIM` -- ordinal 0, id `"slim"`, legacy id `"slim"`.
    Slim = 0,
    /// Port of `PlayerModelType#WIDE` -- ordinal 1, id `"wide"`, legacy id `"default"`.
    Wide = 1,
}

/// Java: `PlayerModelType.values()` -- declaration order.
pub const VALUES: [PlayerModelType; 2] = [PlayerModelType::Slim, PlayerModelType::Wide];

impl PlayerModelType {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PlayerModelType::Slim => "SLIM",
            PlayerModelType::Wide => "WIDE",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `legacyServicesId` field (the key of `NAME_LOOKUP`).
    pub const fn legacy_services_id(&self) -> &'static str {
        match self {
            PlayerModelType::Slim => "slim",
            PlayerModelType::Wide => "default",
        }
    }

    /// Port of `PlayerModelType#getSerializedName()` -- the `id` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            PlayerModelType::Slim => "slim",
            PlayerModelType::Wide => "wide",
        }
    }
}

impl std::fmt::Display for PlayerModelType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for PlayerModelType {
    fn get_serialized_name(&self) -> &str {
        PlayerModelType::get_serialized_name(self)
    }
}

/// Java: `private static final Function<String, PlayerModelType> NAME_LOOKUP =
/// StringRepresentable.createNameLookup(values(), e -> e.legacyServicesId)`.
static NAME_LOOKUP: std::sync::LazyLock<StringRepresentable::NameLookup<PlayerModelType>> =
    std::sync::LazyLock::new(|| {
        StringRepresentable::create_name_lookup_with(&VALUES, |e| e.legacy_services_id().to_string())
    });

/// Port of `PlayerModelType#byLegacyServicesName(@Nullable String)`.
///
/// `Objects.requireNonNullElse(NAME_LOOKUP.apply(name), WIDE)`. `None` is Java's `null`: the
/// linear lookup compares `"slim".equals(null)` / `"default".equals(null)`, both false, so it
/// yields `WIDE` like any unknown name. Exact and case-sensitive.
pub fn by_legacy_services_name(name: Option<&str>) -> PlayerModelType {
    name.and_then(|n| NAME_LOOKUP.apply(n)).unwrap_or(PlayerModelType::Wide)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<PlayerModelType> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, PlayerModelType> STREAM_CODEC =
/// ByteBufCodecs.BOOL.map(slim -> slim ? SLIM : WIDE, type -> type == SLIM)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`. On the wire it is one boolean: `true` = `SLIM`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
