//! Port of: net/minecraft/world/scores/DisplaySlot.java
//! Java class(es): net.minecraft.world.scores.DisplaySlot
//! Status: PARTIAL
//!
//! Session 19. Ported: the 19 constants with `id` and `name`, `id()`, `getSerializedName()`,
//! `BY_ID` (via the ported `ByIdMap`, `ZERO`) and `CODEC` as the ported
//! `StringRepresentable.EnumCodec` (name lookup). Ordinals and both fields are checked against
//! the jar; the lookups are tested in `_porting/tests/session19_c.rs`.
//!
//! Only partially blocked: `CODEC`'s encode/decode half is PORT-BLOCKED on DataFixerUpper inside
//! `StringRepresentable.rs`. Nothing in this file is a `todo!`.
//!
//! # THE TEAM SLOTS ARE DOTTED NAMES
//!
//! The 16 team sidebars serialize as `sidebar.team.<colour>` (dots, as typed in
//! `/scoreboard objectives setdisplay`), and their ids run 3..18 in `TeamColor` order. With 19
//! values the codec's name lookup is the `> 16` hash-map branch of `createNameLookup`.
//! `BY_ID` is `ZERO`: an unknown id is `LIST`.

use crate::net::minecraft::util::StringRepresentable::{self as string_representable, EnumCodec};

/// Port of `net.minecraft.world.scores.DisplaySlot`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DisplaySlot {
    /// Port of `DisplaySlot#LIST` -- ordinal 0, id `0`, name `"list"`.
    List = 0,
    /// Port of `DisplaySlot#SIDEBAR` -- ordinal 1, id `1`, name `"sidebar"`.
    Sidebar = 1,
    /// Port of `DisplaySlot#BELOW_NAME` -- ordinal 2, id `2`, name `"below_name"`.
    BelowName = 2,
    /// Port of `DisplaySlot#TEAM_BLACK` -- ordinal 3, id `3`.
    TeamBlack = 3,
    /// Port of `DisplaySlot#TEAM_DARK_BLUE` -- ordinal 4, id `4`.
    TeamDarkBlue = 4,
    /// Port of `DisplaySlot#TEAM_DARK_GREEN` -- ordinal 5, id `5`.
    TeamDarkGreen = 5,
    /// Port of `DisplaySlot#TEAM_DARK_AQUA` -- ordinal 6, id `6`.
    TeamDarkAqua = 6,
    /// Port of `DisplaySlot#TEAM_DARK_RED` -- ordinal 7, id `7`.
    TeamDarkRed = 7,
    /// Port of `DisplaySlot#TEAM_DARK_PURPLE` -- ordinal 8, id `8`.
    TeamDarkPurple = 8,
    /// Port of `DisplaySlot#TEAM_GOLD` -- ordinal 9, id `9`.
    TeamGold = 9,
    /// Port of `DisplaySlot#TEAM_GRAY` -- ordinal 10, id `10`.
    TeamGray = 10,
    /// Port of `DisplaySlot#TEAM_DARK_GRAY` -- ordinal 11, id `11`.
    TeamDarkGray = 11,
    /// Port of `DisplaySlot#TEAM_BLUE` -- ordinal 12, id `12`.
    TeamBlue = 12,
    /// Port of `DisplaySlot#TEAM_GREEN` -- ordinal 13, id `13`.
    TeamGreen = 13,
    /// Port of `DisplaySlot#TEAM_AQUA` -- ordinal 14, id `14`.
    TeamAqua = 14,
    /// Port of `DisplaySlot#TEAM_RED` -- ordinal 15, id `15`.
    TeamRed = 15,
    /// Port of `DisplaySlot#TEAM_LIGHT_PURPLE` -- ordinal 16, id `16`.
    TeamLightPurple = 16,
    /// Port of `DisplaySlot#TEAM_YELLOW` -- ordinal 17, id `17`.
    TeamYellow = 17,
    /// Port of `DisplaySlot#TEAM_WHITE` -- ordinal 18, id `18`.
    TeamWhite = 18,
}

/// Java: `DisplaySlot.values()` -- declaration order.
pub const VALUES: [DisplaySlot; 19] = [
    DisplaySlot::List,
    DisplaySlot::Sidebar,
    DisplaySlot::BelowName,
    DisplaySlot::TeamBlack,
    DisplaySlot::TeamDarkBlue,
    DisplaySlot::TeamDarkGreen,
    DisplaySlot::TeamDarkAqua,
    DisplaySlot::TeamDarkRed,
    DisplaySlot::TeamDarkPurple,
    DisplaySlot::TeamGold,
    DisplaySlot::TeamGray,
    DisplaySlot::TeamDarkGray,
    DisplaySlot::TeamBlue,
    DisplaySlot::TeamGreen,
    DisplaySlot::TeamAqua,
    DisplaySlot::TeamRed,
    DisplaySlot::TeamLightPurple,
    DisplaySlot::TeamYellow,
    DisplaySlot::TeamWhite,
];

impl DisplaySlot {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            DisplaySlot::List => "LIST",
            DisplaySlot::Sidebar => "SIDEBAR",
            DisplaySlot::BelowName => "BELOW_NAME",
            DisplaySlot::TeamBlack => "TEAM_BLACK",
            DisplaySlot::TeamDarkBlue => "TEAM_DARK_BLUE",
            DisplaySlot::TeamDarkGreen => "TEAM_DARK_GREEN",
            DisplaySlot::TeamDarkAqua => "TEAM_DARK_AQUA",
            DisplaySlot::TeamDarkRed => "TEAM_DARK_RED",
            DisplaySlot::TeamDarkPurple => "TEAM_DARK_PURPLE",
            DisplaySlot::TeamGold => "TEAM_GOLD",
            DisplaySlot::TeamGray => "TEAM_GRAY",
            DisplaySlot::TeamDarkGray => "TEAM_DARK_GRAY",
            DisplaySlot::TeamBlue => "TEAM_BLUE",
            DisplaySlot::TeamGreen => "TEAM_GREEN",
            DisplaySlot::TeamAqua => "TEAM_AQUA",
            DisplaySlot::TeamRed => "TEAM_RED",
            DisplaySlot::TeamLightPurple => "TEAM_LIGHT_PURPLE",
            DisplaySlot::TeamYellow => "TEAM_YELLOW",
            DisplaySlot::TeamWhite => "TEAM_WHITE",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `DisplaySlot#id()` -- the stored `id` field.
    #[inline]
    pub const fn id(&self) -> i32 {
        match self {
            DisplaySlot::List => 0,
            DisplaySlot::Sidebar => 1,
            DisplaySlot::BelowName => 2,
            DisplaySlot::TeamBlack => 3,
            DisplaySlot::TeamDarkBlue => 4,
            DisplaySlot::TeamDarkGreen => 5,
            DisplaySlot::TeamDarkAqua => 6,
            DisplaySlot::TeamDarkRed => 7,
            DisplaySlot::TeamDarkPurple => 8,
            DisplaySlot::TeamGold => 9,
            DisplaySlot::TeamGray => 10,
            DisplaySlot::TeamDarkGray => 11,
            DisplaySlot::TeamBlue => 12,
            DisplaySlot::TeamGreen => 13,
            DisplaySlot::TeamAqua => 14,
            DisplaySlot::TeamRed => 15,
            DisplaySlot::TeamLightPurple => 16,
            DisplaySlot::TeamYellow => 17,
            DisplaySlot::TeamWhite => 18,
        }
    }

    /// Port of `DisplaySlot#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            DisplaySlot::List => "list",
            DisplaySlot::Sidebar => "sidebar",
            DisplaySlot::BelowName => "below_name",
            DisplaySlot::TeamBlack => "sidebar.team.black",
            DisplaySlot::TeamDarkBlue => "sidebar.team.dark_blue",
            DisplaySlot::TeamDarkGreen => "sidebar.team.dark_green",
            DisplaySlot::TeamDarkAqua => "sidebar.team.dark_aqua",
            DisplaySlot::TeamDarkRed => "sidebar.team.dark_red",
            DisplaySlot::TeamDarkPurple => "sidebar.team.dark_purple",
            DisplaySlot::TeamGold => "sidebar.team.gold",
            DisplaySlot::TeamGray => "sidebar.team.gray",
            DisplaySlot::TeamDarkGray => "sidebar.team.dark_gray",
            DisplaySlot::TeamBlue => "sidebar.team.blue",
            DisplaySlot::TeamGreen => "sidebar.team.green",
            DisplaySlot::TeamAqua => "sidebar.team.aqua",
            DisplaySlot::TeamRed => "sidebar.team.red",
            DisplaySlot::TeamLightPurple => "sidebar.team.light_purple",
            DisplaySlot::TeamYellow => "sidebar.team.yellow",
            DisplaySlot::TeamWhite => "sidebar.team.white",
        }
    }
}

impl std::fmt::Display for DisplaySlot {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl string_representable::StringRepresentable for DisplaySlot {
    fn get_serialized_name(&self) -> &str {
        DisplaySlot::get_serialized_name(self)
    }
}

static CODEC_INSTANCE: std::sync::LazyLock<EnumCodec<DisplaySlot>> =
    std::sync::LazyLock::new(|| string_representable::from_enum(&VALUES));

/// Java: `public static final StringRepresentable.EnumCodec<DisplaySlot> CODEC`.
///
/// The ported `EnumCodec`: `by_name` works; its `encode`/`decode` are PORT-BLOCKED on
/// DataFixerUpper inside `StringRepresentable.rs`.
pub fn CODEC() -> &'static EnumCodec<DisplaySlot> {
    &CODEC_INSTANCE
}

static BY_ID_MAP: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<DisplaySlot>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            DisplaySlot::id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Zero,
        )
    });

/// Java: `public static final IntFunction<DisplaySlot> BY_ID`, applied.
///
/// `OutOfBoundsStrategy.ZERO`: an id outside `0..19` yields `LIST`.
pub fn BY_ID(id: i32) -> DisplaySlot {
    BY_ID_MAP.apply(id)
}
