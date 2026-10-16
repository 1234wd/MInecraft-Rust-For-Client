//! Port of: net/minecraft/world/item/crafting/CookingBookCategory.java
//! Java class(es): net.minecraft.world.item.crafting.CookingBookCategory
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the three constants with `id` and `name`, the private `BY_ID`
//! (as [`by_id`]), `getSerializedName`. Constants and fields are checked against the jar in
//! `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `CODEC` on DataFixerUpper, `STREAM_CODEC` on netty `ByteBuf`.
//!
//! Java has no getter for `id`; it is only read by `BY_ID`/`STREAM_CODEC` through `e -> e.id`.
//! The port exposes it as [`CookingBookCategory::id`] for the same purpose.

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.item.crafting.CookingBookCategory`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CookingBookCategory {
    /// Port of `CookingBookCategory#FOOD` -- ordinal 0, id 0, `"food"`.
    Food = 0,
    /// Port of `CookingBookCategory#BLOCKS` -- ordinal 1, id 1, `"blocks"`.
    Blocks = 1,
    /// Port of `CookingBookCategory#MISC` -- ordinal 2, id 2, `"misc"`.
    Misc = 2,
}

/// Java: `CookingBookCategory.values()` -- declaration order.
pub const VALUES: [CookingBookCategory; 3] = [
    CookingBookCategory::Food,
    CookingBookCategory::Blocks,
    CookingBookCategory::Misc,
];

impl CookingBookCategory {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            CookingBookCategory::Food => "FOOD",
            CookingBookCategory::Blocks => "BLOCKS",
            CookingBookCategory::Misc => "MISC",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `id` field (read by `BY_ID` / `STREAM_CODEC`).
    pub const fn id(&self) -> i32 {
        match self {
            CookingBookCategory::Food => 0,
            CookingBookCategory::Blocks => 1,
            CookingBookCategory::Misc => 2,
        }
    }

    /// Port of `CookingBookCategory#getSerializedName()` -- the `name` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            CookingBookCategory::Food => "food",
            CookingBookCategory::Blocks => "blocks",
            CookingBookCategory::Misc => "misc",
        }
    }
}

impl std::fmt::Display for CookingBookCategory {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for CookingBookCategory {
    fn get_serialized_name(&self) -> &str {
        CookingBookCategory::get_serialized_name(self)
    }
}

static BY_ID: std::sync::LazyLock<ByIdMap::Continuous<CookingBookCategory>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(CookingBookCategory::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Java: `private static final IntFunction<CookingBookCategory> BY_ID`, applied (the decoding
/// half of `STREAM_CODEC`).
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..3` yields `FOOD`.
pub fn by_id(id: i32) -> CookingBookCategory {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<CookingBookCategory> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, CookingBookCategory> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, e -> e.id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
