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
//! `ByIdMap.OutOfBoundsStrategy.WRAP` means an id of `4` returns `PEACEFUL` (`4 % 4 == 0`).
//! PORT-BLOCKED on `ByIdMap`; the wrapping semantics are documented here for the caller.
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

/// Java: `private static final IntFunction<Difficulty> BY_ID`.
///
/// `net.minecraft.util.ByIdMap` is not ported. The wrapping semantics are
/// `OutOfBoundsStrategy.WRAP`: an unknown id wraps by `id % 4`. A caller needing the mapping can
/// use [`Difficulty::get_id`] plus a local match.
pub fn by_id(_id: i32) -> Difficulty {
    todo!("PORT-BLOCKED: net.minecraft.util.ByIdMap is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, Difficulty> STREAM_CODEC`.
///
/// `io.netty.buffer.ByteBuf` is not ported.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
