//! Port of: net/minecraft/world/item/Rarity.java
//! Java class(es): net.minecraft.world.item.Rarity
//! Status: PARTIAL
//!
//! Session 19. Four constants with `id`, `name` and `color` (`ChatFormatting`), `color()`,
//! `getSerializedName()` and `BY_ID` are ported; names, ordinals and all three fields are
//! checked against the jar in `_porting/tests/session19_b.rs`.
//!
//! PORT-BLOCKED: `CODEC` (`StringRepresentable.fromValues`, a bare DFU `Codec`) and
//! `STREAM_CODEC` (`ByteBufCodecs.idMapper`, netty `ByteBuf`).
//!
//! # `BY_ID` IS `ZERO`, NOT `WRAP` OR `CLAMP`
//!
//! `ByIdMap.OutOfBoundsStrategy.ZERO`: an unknown id (negative, or `>= 4`) decodes as `COMMON`,
//! so a corrupt rarity id from the network shows as a white item name rather than throwing.

use crate::net::minecraft::ChatFormatting::ChatFormatting;

/// Port of `net.minecraft.world.item.Rarity`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rarity {
    /// Port of `Rarity#COMMON` -- ordinal 0, id `0`, name `"common"`, color `WHITE`.
    Common = 0,
    /// Port of `Rarity#UNCOMMON` -- ordinal 1, id `1`, name `"uncommon"`, color `YELLOW`.
    Uncommon = 1,
    /// Port of `Rarity#RARE` -- ordinal 2, id `2`, name `"rare"`, color `AQUA`.
    Rare = 2,
    /// Port of `Rarity#EPIC` -- ordinal 3, id `3`, name `"epic"`, color `LIGHT_PURPLE`.
    Epic = 3,
}

/// Java: `Rarity.values()` -- declaration order.
pub const VALUES: [Rarity; 4] = [Rarity::Common, Rarity::Uncommon, Rarity::Rare, Rarity::Epic];

impl Rarity {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Rarity::Common => "COMMON",
            Rarity::Uncommon => "UNCOMMON",
            Rarity::Rare => "RARE",
            Rarity::Epic => "EPIC",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// The private `id` field, read by `BY_ID` and `STREAM_CODEC` (`r -> r.id`). Equal to the
    /// ordinal today, but a separate value.
    #[inline]
    pub const fn id(&self) -> i32 {
        match self {
            Rarity::Common => 0,
            Rarity::Uncommon => 1,
            Rarity::Rare => 2,
            Rarity::Epic => 3,
        }
    }

    /// Port of `Rarity#color()`.
    #[inline]
    pub const fn color(&self) -> ChatFormatting {
        match self {
            Rarity::Common => ChatFormatting::White,
            Rarity::Uncommon => ChatFormatting::Yellow,
            Rarity::Rare => ChatFormatting::Aqua,
            Rarity::Epic => ChatFormatting::LightPurple,
        }
    }

    /// Port of `Rarity#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            Rarity::Common => "common",
            Rarity::Uncommon => "uncommon",
            Rarity::Rare => "rare",
            Rarity::Epic => "epic",
        }
    }
}

impl std::fmt::Display for Rarity {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for Rarity {
    fn get_serialized_name(&self) -> &str {
        Rarity::get_serialized_name(self)
    }
}

static BY_ID_MAP: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<Rarity>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            Rarity::id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Zero,
        )
    });

/// Java: `public static final IntFunction<Rarity> BY_ID`, applied.
///
/// `ByIdMap.continuous(r -> r.id, values(), OutOfBoundsStrategy.ZERO)`: any id outside `0..4`
/// yields `COMMON`.
pub fn BY_ID(id: i32) -> Rarity {
    BY_ID_MAP.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<Rarity> CODEC = StringRepresentable.fromValues(Rarity::values)`.
///
/// `fromValues` returns a bare `com.mojang.serialization.Codec`, which is not ported.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, Rarity> STREAM_CODEC =
/// ByteBufCodecs.idMapper(BY_ID, r -> r.id)`.
///
/// `io.netty.buffer.ByteBuf` is not ported.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
