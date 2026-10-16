//! Port of: net/minecraft/world/Difficulty.java
//! Java class(es): net.minecraft.world.Difficulty
//! Status: VERIFIED
//!
//! Four constants, each carrying an `int id` and a `String key`. Verified against the jar by
//! `_porting/tests/consts.rs`: ordinals plus both fields for every constant.
//!
//! # ORDINALS AND IDS ARE THE SAME BUT ARE NOT THE SAME THING
//!
//! `PEACEFUL` is ordinal 0 and id 0, `EASY` is ordinal 1 and id 1, etc. They agree today and
//! will likely always agree for this particular enum, but they are modelled as distinct values
//! (`id()` reads the stored field, the discriminant is the ordinal) so any future divergence
//! cannot produce a silently wrong result.
//!
//! # `byId` WRAPS OUT-OF-BOUNDS
//!
//! `ByIdMap.OutOfBoundsStrategy.WRAP` means an id of `4` returns `PEACEFUL` (`4 % 4 == 0`) and
//! `-1` returns `HARD` (`floorMod`). Ported in session 19 on top of `ByIdMap`, together with
//! `byName` (via `StringRepresentable.EnumCodec`). `getDisplayName`/`getInfo` remain
//! PORT-BLOCKED on `Component`.
//!
//! # CODEC AND STREAM_CODEC ARE PORT-BLOCKED
//!
//! `CODEC` needs `StringRepresentable.fromEnum`, which is a DataFixerUpper codec. `STREAM_CODEC`
//! needs `ByteBufCodecs.idMapper`, which depends on the netty `ByteBuf` type. Both belong to
//! future batches.

/// Port of `net.minecraft.world.Difficulty`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Difficulty {
    /// Port of `Difficulty#PEACEFUL` -- ordinal 0, id `0`, key `"peaceful"`.
    Peaceful = 0,
    /// Port of `Difficulty#EASY` -- ordinal 1, id `1`, key `"easy"`.
    Easy = 1,
    /// Port of `Difficulty#NORMAL` -- ordinal 2, id `2`, key `"normal"`.
    Normal = 2,
    /// Port of `Difficulty#HARD` -- ordinal 3, id `3`, key `"hard"`.
    Hard = 3,
}

/// Java: `Difficulty.values()` -- declaration order, which is the ordinal order.
pub const VALUES: [Difficulty; 4] = [
    Difficulty::Peaceful,
    Difficulty::Easy,
    Difficulty::Normal,
    Difficulty::Hard,
];

impl Difficulty {
    /// Port of `Difficulty#getId()`.
    ///
    /// Returns the stored `id` field. Today it equals the ordinal; `id()` reads the field rather
    /// than the discriminant so any future divergence is caught at compile time rather than
    /// silently.
    #[inline]
    pub const fn get_id(&self) -> i32 {
        match self {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        }
    }

    /// Port of `Difficulty#getSerializedName()` -- the `key` field used in commands and saves.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            Difficulty::Peaceful => "peaceful",
            Difficulty::Easy => "easy",
            Difficulty::Normal => "normal",
            Difficulty::Hard => "hard",
        }
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final StringRepresentable.EnumCodec<Difficulty> CODEC`.
///
/// `com.mojang.serialization.Codec` (via `StringRepresentable.fromEnum`) is not ported.
/// See DESIGN_DECISIONS.md (#dfu-proposal).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

impl std::fmt::Display for Difficulty {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Difficulty::Peaceful => "PEACEFUL",
            Difficulty::Easy => "EASY",
            Difficulty::Normal => "NORMAL",
            Difficulty::Hard => "HARD",
        })
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for Difficulty {
    fn get_serialized_name(&self) -> &str {
        Difficulty::get_serialized_name(self)
    }
}

static BY_ID: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<Difficulty>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            Difficulty::get_id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Wrap,
        )
    });

/// The name-resolution half of `Difficulty.CODEC` (`StringRepresentable.fromEnum(values)`).
static CODEC_BY_NAME: std::sync::LazyLock<crate::net::minecraft::util::StringRepresentable::EnumCodec<Difficulty>> =
    std::sync::LazyLock::new(|| crate::net::minecraft::util::StringRepresentable::from_enum(&VALUES));

/// Port of `Difficulty#byId(int)` (deprecated in vanilla, still called).
///
/// Session 19: unblocked by the `ByIdMap` port. `OutOfBoundsStrategy.WRAP` is `Math.floorMod`,
/// so `4` is `PEACEFUL` and `-1` is `HARD`.
pub fn by_id(id: i32) -> Difficulty {
    BY_ID.apply(id)
}

/// Port of `Difficulty#byName(String)`: `CODEC.byName(name)`, `None` for an unknown name.
pub fn by_name(name: &str) -> Option<Difficulty> {
    CODEC_BY_NAME.by_name(name)
}

/// Java: `public Component getDisplayName()` -- `Component.translatable("options.difficulty." +
/// key)`.
///
/// PORT-BLOCKED on `net.minecraft.network.chat.Component`. The translation key is available as
/// `format!("options.difficulty.{}", d.get_serialized_name())`.
pub fn get_display_name(_d: Difficulty) {
    todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
}

/// Java: `public Component getInfo()` -- `Component.translatable("options.difficulty." + key +
/// ".info")`.
///
/// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
pub fn get_info(_d: Difficulty) {
    todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, Difficulty> STREAM_CODEC`.
///
/// `io.netty.buffer.ByteBuf` is not ported.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
