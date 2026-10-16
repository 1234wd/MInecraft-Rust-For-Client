//! Port of: net/minecraft/world/level/GameType.java
//! Java class(es): net.minecraft.world.level.GameType
//! Status: PARTIAL
//!
//! Session 19. Ported: the four constants with `id` and `name`, `DEFAULT_MODE`, `NOT_SET`,
//! `CODEC` as the ported `StringRepresentable.EnumCodec` (name lookup only), the private
//! `BY_ID` (`ByIdMap`, `ZERO`), `getId`, `getName`, `getSerializedName`,
//! `isBlockPlacingRestricted`, `isCreative`, `isSurvival`, `byId`, both `byName` overloads,
//! `getNullableId`, `byNullableId` and `isValidId`. Ordinals, `id` and `name` are checked
//! against the jar; the behaviour is tested in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED:
//! * `getShortDisplayName` / `getLongDisplayName` (the `shortName` / `longName` fields) on
//!   `net.minecraft.network.chat.Component`. The jar has them as
//!   `translatable("selectWorld.gameMode.<name>")` and `translatable("gameMode.<name>")`.
//! * `updatePlayerAbilities(Abilities)` on `net.minecraft.world.entity.player.Abilities`.
//! * `STREAM_CODEC` on netty `ByteBuf`; `LEGACY_ID_CODEC` on DataFixerUpper `Codec`;
//!   the encode/decode half of `CODEC` (in `StringRepresentable.rs`) on DataFixerUpper.
//!
//! # `isSurvival` INCLUDES ADVENTURE; `byId` FALLS BACK TO SURVIVAL
//!
//! * `isSurvival()` is `SURVIVAL || ADVENTURE`; `isBlockPlacingRestricted()` is
//!   `ADVENTURE || SPECTATOR`. Spectator is neither creative nor survival.
//! * `byId` uses `OutOfBoundsStrategy.ZERO`, so any unknown id (including `-1`) is `SURVIVAL`.
//!   `byNullableId` special-cases exactly `-1` (`NOT_SET`) to `null` first; `-2` still goes
//!   through `byId` and becomes `SURVIVAL`.
//! * `byName(String)` defaults to `SURVIVAL`; `byName(String, null)` returns `null` for an
//!   unknown name. Lookup is exact and case-sensitive (`"Creative"` is unknown).

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable::{self as string_representable, EnumCodec};

/// Port of `net.minecraft.world.level.GameType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameType {
    /// Port of `GameType#SURVIVAL` -- ordinal 0, id `0`, name `"survival"`.
    Survival = 0,
    /// Port of `GameType#CREATIVE` -- ordinal 1, id `1`, name `"creative"`.
    Creative = 1,
    /// Port of `GameType#ADVENTURE` -- ordinal 2, id `2`, name `"adventure"`.
    Adventure = 2,
    /// Port of `GameType#SPECTATOR` -- ordinal 3, id `3`, name `"spectator"`.
    Spectator = 3,
}

/// Java: `GameType.values()` -- declaration order.
pub const VALUES: [GameType; 4] = [
    GameType::Survival,
    GameType::Creative,
    GameType::Adventure,
    GameType::Spectator,
];

/// Java: `public static final GameType DEFAULT_MODE = SURVIVAL`.
pub const DEFAULT_MODE: GameType = GameType::Survival;

/// Java: `private static final int NOT_SET = -1` -- the id `getNullableId` writes for `null`.
pub const NOT_SET: i32 = -1;

impl GameType {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            GameType::Survival => "SURVIVAL",
            GameType::Creative => "CREATIVE",
            GameType::Adventure => "ADVENTURE",
            GameType::Spectator => "SPECTATOR",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `GameType#getId()` -- the stored `id` field.
    #[inline]
    pub const fn get_id(&self) -> i32 {
        match self {
            GameType::Survival => 0,
            GameType::Creative => 1,
            GameType::Adventure => 2,
            GameType::Spectator => 3,
        }
    }

    /// Port of `GameType#getName()` -- the private `name` field.
    #[inline]
    pub const fn get_name(&self) -> &'static str {
        match self {
            GameType::Survival => "survival",
            GameType::Creative => "creative",
            GameType::Adventure => "adventure",
            GameType::Spectator => "spectator",
        }
    }

    /// Port of `GameType#getSerializedName()` -- the same `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        self.get_name()
    }

    /// Port of `GameType#isBlockPlacingRestricted()`: `ADVENTURE || SPECTATOR`.
    #[inline]
    pub const fn is_block_placing_restricted(&self) -> bool {
        matches!(self, GameType::Adventure | GameType::Spectator)
    }

    /// Port of `GameType#isCreative()`: `CREATIVE` only.
    #[inline]
    pub const fn is_creative(&self) -> bool {
        matches!(self, GameType::Creative)
    }

    /// Port of `GameType#isSurvival()`: `SURVIVAL || ADVENTURE`.
    #[inline]
    pub const fn is_survival(&self) -> bool {
        matches!(self, GameType::Survival | GameType::Adventure)
    }

    /// Java: `public Component getLongDisplayName()` -- `translatable("gameMode." + name)`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_long_display_name(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Java: `public Component getShortDisplayName()` --
    /// `translatable("selectWorld.gameMode." + name)`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_short_display_name(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Java: `public void updatePlayerAbilities(Abilities abilities)`.
    ///
    /// PORT-BLOCKED on `net.minecraft.world.entity.player.Abilities` (SKELETON). Java sets
    /// `mayfly/instabuild/invulnerable` to `true/true/true` for CREATIVE (leaving `flying`
    /// untouched), `true/false/true` plus `flying = true` for SPECTATOR, all four `false`
    /// otherwise, then `mayBuild = !isBlockPlacingRestricted()`.
    pub fn update_player_abilities(&self) {
        todo!("PORT-BLOCKED: net.minecraft.world.entity.player.Abilities is not ported")
    }
}

impl std::fmt::Display for GameType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl string_representable::StringRepresentable for GameType {
    fn get_serialized_name(&self) -> &str {
        GameType::get_serialized_name(self)
    }
}

static CODEC_INSTANCE: std::sync::LazyLock<EnumCodec<GameType>> =
    std::sync::LazyLock::new(|| string_representable::from_enum(&VALUES));

/// Java: `public static final StringRepresentable.EnumCodec<GameType> CODEC`.
///
/// The ported `EnumCodec`: `by_name` works; its `encode`/`decode` are PORT-BLOCKED on
/// DataFixerUpper inside `StringRepresentable.rs`.
pub fn CODEC() -> &'static EnumCodec<GameType> {
    &CODEC_INSTANCE
}

/// Java: `private static final IntFunction<GameType> BY_ID =
/// ByIdMap.continuous(GameType::getId, values(), ZERO)`.
static BY_ID: std::sync::LazyLock<ByIdMap::Continuous<GameType>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(GameType::get_id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Port of `GameType#byId(int)`: `ZERO`, so any id outside `0..4` is `SURVIVAL`.
pub fn by_id(id: i32) -> GameType {
    BY_ID.apply(id)
}

/// Port of `GameType#byName(String)`: `byName(name, SURVIVAL)`.
pub fn by_name(name: &str) -> GameType {
    // `byName(name, SURVIVAL)` is non-null whenever the default is non-null.
    by_name_or(name, Some(GameType::Survival)).unwrap_or(GameType::Survival)
}

/// Port of `GameType#byName(String, @Nullable GameType)`: `CODEC.byName(name)`, else the default
/// (which may itself be `null`, i.e. `None`).
pub fn by_name_or(name: &str, default_mode: Option<GameType>) -> Option<GameType> {
    match CODEC_INSTANCE.by_name(name) {
        Some(result) => Some(result),
        None => default_mode,
    }
}

/// Port of `GameType#getNullableId(@Nullable GameType)`: the id, or `-1` (`NOT_SET`) for `null`.
pub fn get_nullable_id(game_type: Option<GameType>) -> i32 {
    match game_type {
        Some(game_type) => game_type.get_id(),
        None => NOT_SET,
    }
}

/// Port of `GameType#byNullableId(int)`: `id == -1 ? null : byId(id)`.
pub fn by_nullable_id(id: i32) -> Option<GameType> {
    if id == NOT_SET {
        None
    } else {
        Some(by_id(id))
    }
}

/// Port of `GameType#isValidId(int)`: whether any constant's `id` equals `id`.
pub fn is_valid_id(id: i32) -> bool {
    VALUES.iter().any(|game_type| game_type.get_id() == id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final StreamCodec<ByteBuf, GameType> STREAM_CODEC =
/// ByteBufCodecs.idMapper(BY_ID, GameType::getId)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}

/// Java: `@Deprecated public static final Codec<GameType> LEGACY_ID_CODEC =
/// Codec.INT.xmap(GameType::byId, GameType::getId)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`. The mapping it would use is [`by_id`] /
/// [`GameType::get_id`].
pub fn LEGACY_ID_CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
