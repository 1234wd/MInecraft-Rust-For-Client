//! Port of: net/minecraft/world/scores/TeamColor.java
//! Java class(es): net.minecraft.world.scores.TeamColor
//! Status: PARTIAL
//!
//! Session 19. Ported: the 16 constants with `id`, `name` and `displaySlot`, the `VALUES` list,
//! `CODEC` as the ported `StringRepresentable.EnumCodec` (name lookup), `byName`, the private
//! `BY_ID` (via the ported `ByIdMap`, `ZERO`), `displaySlot()` and `getSerializedName()`.
//! Ordinals, `id`, `name` and `displaySlot` are checked against the jar in
//! `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED:
//! * `textColor()` and `rgb()` (the `format` field) on `net.minecraft.network.chat.TextColor`
//!   (SKELETON; its `ChatFormatting` source is being ported elsewhere). Each constant uses the
//!   `TextColor` constant of the same name (`BLACK -> TextColor.BLACK`, ...); the jar's
//!   `toString()` of the field is the colour name, equal to this enum's serialized name.
//! * `STREAM_CODEC` on netty `ByteBuf`; `CODEC`'s encode/decode half on DataFixerUpper
//!   (inside `StringRepresentable.rs`).
//!
//! # IDS 0..15 ARE THE LEGACY CHAT-COLOUR ORDER
//!
//! `BLACK, DARK_BLUE, DARK_GREEN, DARK_AQUA, DARK_RED, DARK_PURPLE, GOLD, GRAY, DARK_GRAY,
//! BLUE, GREEN, AQUA, RED, LIGHT_PURPLE, YELLOW, WHITE` -- the `§0`..`§f` order. Each maps to
//! the `DisplaySlot.TEAM_<colour>` whose id is this id + 3. `BY_ID` is `ZERO`, so an unknown
//! id from the network is `BLACK`.

use crate::net::minecraft::util::StringRepresentable::{self as string_representable, EnumCodec};
use crate::net::minecraft::world::scores::DisplaySlot::DisplaySlot;

/// Port of `net.minecraft.world.scores.TeamColor`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TeamColor {
    /// Port of `TeamColor#BLACK` -- ordinal 0, id `0`, `TEAM_BLACK`.
    Black = 0,
    /// Port of `TeamColor#DARK_BLUE` -- ordinal 1, id `1`, `TEAM_DARK_BLUE`.
    DarkBlue = 1,
    /// Port of `TeamColor#DARK_GREEN` -- ordinal 2, id `2`, `TEAM_DARK_GREEN`.
    DarkGreen = 2,
    /// Port of `TeamColor#DARK_AQUA` -- ordinal 3, id `3`, `TEAM_DARK_AQUA`.
    DarkAqua = 3,
    /// Port of `TeamColor#DARK_RED` -- ordinal 4, id `4`, `TEAM_DARK_RED`.
    DarkRed = 4,
    /// Port of `TeamColor#DARK_PURPLE` -- ordinal 5, id `5`, `TEAM_DARK_PURPLE`.
    DarkPurple = 5,
    /// Port of `TeamColor#GOLD` -- ordinal 6, id `6`, `TEAM_GOLD`.
    Gold = 6,
    /// Port of `TeamColor#GRAY` -- ordinal 7, id `7`, `TEAM_GRAY`.
    Gray = 7,
    /// Port of `TeamColor#DARK_GRAY` -- ordinal 8, id `8`, `TEAM_DARK_GRAY`.
    DarkGray = 8,
    /// Port of `TeamColor#BLUE` -- ordinal 9, id `9`, `TEAM_BLUE`.
    Blue = 9,
    /// Port of `TeamColor#GREEN` -- ordinal 10, id `10`, `TEAM_GREEN`.
    Green = 10,
    /// Port of `TeamColor#AQUA` -- ordinal 11, id `11`, `TEAM_AQUA`.
    Aqua = 11,
    /// Port of `TeamColor#RED` -- ordinal 12, id `12`, `TEAM_RED`.
    Red = 12,
    /// Port of `TeamColor#LIGHT_PURPLE` -- ordinal 13, id `13`, `TEAM_LIGHT_PURPLE`.
    LightPurple = 13,
    /// Port of `TeamColor#YELLOW` -- ordinal 14, id `14`, `TEAM_YELLOW`.
    Yellow = 14,
    /// Port of `TeamColor#WHITE` -- ordinal 15, id `15`, `TEAM_WHITE`.
    White = 15,
}

/// Java: `TeamColor.values()`, and also `public static final List<TeamColor> VALUES =
/// List.of(values())` -- the same elements in the same (declaration) order.
pub const VALUES: [TeamColor; 16] = [
    TeamColor::Black,
    TeamColor::DarkBlue,
    TeamColor::DarkGreen,
    TeamColor::DarkAqua,
    TeamColor::DarkRed,
    TeamColor::DarkPurple,
    TeamColor::Gold,
    TeamColor::Gray,
    TeamColor::DarkGray,
    TeamColor::Blue,
    TeamColor::Green,
    TeamColor::Aqua,
    TeamColor::Red,
    TeamColor::LightPurple,
    TeamColor::Yellow,
    TeamColor::White,
];

impl TeamColor {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            TeamColor::Black => "BLACK",
            TeamColor::DarkBlue => "DARK_BLUE",
            TeamColor::DarkGreen => "DARK_GREEN",
            TeamColor::DarkAqua => "DARK_AQUA",
            TeamColor::DarkRed => "DARK_RED",
            TeamColor::DarkPurple => "DARK_PURPLE",
            TeamColor::Gold => "GOLD",
            TeamColor::Gray => "GRAY",
            TeamColor::DarkGray => "DARK_GRAY",
            TeamColor::Blue => "BLUE",
            TeamColor::Green => "GREEN",
            TeamColor::Aqua => "AQUA",
            TeamColor::Red => "RED",
            TeamColor::LightPurple => "LIGHT_PURPLE",
            TeamColor::Yellow => "YELLOW",
            TeamColor::White => "WHITE",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// The private `id` field (read by `BY_ID` and `STREAM_CODEC` as `v -> v.id`).
    #[inline]
    pub const fn id(&self) -> i32 {
        match self {
            TeamColor::Black => 0,
            TeamColor::DarkBlue => 1,
            TeamColor::DarkGreen => 2,
            TeamColor::DarkAqua => 3,
            TeamColor::DarkRed => 4,
            TeamColor::DarkPurple => 5,
            TeamColor::Gold => 6,
            TeamColor::Gray => 7,
            TeamColor::DarkGray => 8,
            TeamColor::Blue => 9,
            TeamColor::Green => 10,
            TeamColor::Aqua => 11,
            TeamColor::Red => 12,
            TeamColor::LightPurple => 13,
            TeamColor::Yellow => 14,
            TeamColor::White => 15,
        }
    }

    /// Port of `TeamColor#displaySlot()`.
    #[inline]
    pub const fn display_slot(&self) -> DisplaySlot {
        match self {
            TeamColor::Black => DisplaySlot::TeamBlack,
            TeamColor::DarkBlue => DisplaySlot::TeamDarkBlue,
            TeamColor::DarkGreen => DisplaySlot::TeamDarkGreen,
            TeamColor::DarkAqua => DisplaySlot::TeamDarkAqua,
            TeamColor::DarkRed => DisplaySlot::TeamDarkRed,
            TeamColor::DarkPurple => DisplaySlot::TeamDarkPurple,
            TeamColor::Gold => DisplaySlot::TeamGold,
            TeamColor::Gray => DisplaySlot::TeamGray,
            TeamColor::DarkGray => DisplaySlot::TeamDarkGray,
            TeamColor::Blue => DisplaySlot::TeamBlue,
            TeamColor::Green => DisplaySlot::TeamGreen,
            TeamColor::Aqua => DisplaySlot::TeamAqua,
            TeamColor::Red => DisplaySlot::TeamRed,
            TeamColor::LightPurple => DisplaySlot::TeamLightPurple,
            TeamColor::Yellow => DisplaySlot::TeamYellow,
            TeamColor::White => DisplaySlot::TeamWhite,
        }
    }

    /// Port of `TeamColor#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            TeamColor::Black => "black",
            TeamColor::DarkBlue => "dark_blue",
            TeamColor::DarkGreen => "dark_green",
            TeamColor::DarkAqua => "dark_aqua",
            TeamColor::DarkRed => "dark_red",
            TeamColor::DarkPurple => "dark_purple",
            TeamColor::Gold => "gold",
            TeamColor::Gray => "gray",
            TeamColor::DarkGray => "dark_gray",
            TeamColor::Blue => "blue",
            TeamColor::Green => "green",
            TeamColor::Aqua => "aqua",
            TeamColor::Red => "red",
            TeamColor::LightPurple => "light_purple",
            TeamColor::Yellow => "yellow",
            TeamColor::White => "white",
        }
    }

    /// Java: `public TextColor textColor()` -- the `format` field.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.TextColor` (SKELETON).
    pub fn text_color(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.TextColor is not ported")
    }

    /// Java: `public int rgb()` -- `this.format.getValue()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.TextColor` (SKELETON).
    pub fn rgb(&self) -> i32 {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.TextColor is not ported")
    }
}

impl std::fmt::Display for TeamColor {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl string_representable::StringRepresentable for TeamColor {
    fn get_serialized_name(&self) -> &str {
        TeamColor::get_serialized_name(self)
    }
}

static CODEC_INSTANCE: std::sync::LazyLock<EnumCodec<TeamColor>> =
    std::sync::LazyLock::new(|| string_representable::from_enum(&VALUES));

/// Java: `public static final StringRepresentable.EnumCodec<TeamColor> CODEC`.
///
/// The ported `EnumCodec`: `by_name` works; its `encode`/`decode` are PORT-BLOCKED on
/// DataFixerUpper inside `StringRepresentable.rs`.
pub fn CODEC() -> &'static EnumCodec<TeamColor> {
    &CODEC_INSTANCE
}

/// Port of `TeamColor#byName(String)`: `CODEC.byName(name)`, `None` (Java `null`) if unknown.
pub fn by_name(name: &str) -> Option<TeamColor> {
    CODEC_INSTANCE.by_name(name)
}

static BY_ID_MAP: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<TeamColor>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            TeamColor::id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Zero,
        )
    });

/// Java: `private static final IntFunction<TeamColor> BY_ID`, applied (its only Java consumer
/// is `STREAM_CODEC`). `OutOfBoundsStrategy.ZERO`: an id outside `0..16` yields `BLACK`.
pub fn BY_ID(id: i32) -> TeamColor {
    BY_ID_MAP.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final StreamCodec<ByteBuf, TeamColor> STREAM_CODEC =
/// ByteBufCodecs.idMapper(BY_ID, v -> v.id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
