//! Port of: net/minecraft/world/item/ItemUseAnimation.java
//! Java class(es): net.minecraft.world.item.ItemUseAnimation
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the twelve constants with `id`, `name` and `customArmTransform`,
//! the private `BY_ID` (as [`by_id`]), `getId`, `getSerializedName`, `hasCustomArmTransform`.
//! Constants and all three fields are checked against the jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `CODEC` on DataFixerUpper, `STREAM_CODEC` on netty `ByteBuf`.
//!
//! The 2-argument constructor defaults `customArmTransform` to `false`; only `EAT`, `DRINK` and
//! `SPEAR` pass `true`.

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.item.ItemUseAnimation`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemUseAnimation {
    /// Port of `ItemUseAnimation#NONE` -- ordinal 0, id 0.
    None = 0,
    /// Port of `ItemUseAnimation#EAT` -- ordinal 1, id 1, custom arm transform.
    Eat = 1,
    /// Port of `ItemUseAnimation#DRINK` -- ordinal 2, id 2, custom arm transform.
    Drink = 2,
    /// Port of `ItemUseAnimation#BLOCK` -- ordinal 3, id 3.
    Block = 3,
    /// Port of `ItemUseAnimation#BOW` -- ordinal 4, id 4.
    Bow = 4,
    /// Port of `ItemUseAnimation#TRIDENT` -- ordinal 5, id 5.
    Trident = 5,
    /// Port of `ItemUseAnimation#CROSSBOW` -- ordinal 6, id 6.
    Crossbow = 6,
    /// Port of `ItemUseAnimation#SPYGLASS` -- ordinal 7, id 7.
    Spyglass = 7,
    /// Port of `ItemUseAnimation#TOOT_HORN` -- ordinal 8, id 8.
    TootHorn = 8,
    /// Port of `ItemUseAnimation#BRUSH` -- ordinal 9, id 9.
    Brush = 9,
    /// Port of `ItemUseAnimation#BUNDLE` -- ordinal 10, id 10.
    Bundle = 10,
    /// Port of `ItemUseAnimation#SPEAR` -- ordinal 11, id 11, custom arm transform.
    Spear = 11,
}

/// Java: `ItemUseAnimation.values()` -- declaration order.
pub const VALUES: [ItemUseAnimation; 12] = [
    ItemUseAnimation::None,
    ItemUseAnimation::Eat,
    ItemUseAnimation::Drink,
    ItemUseAnimation::Block,
    ItemUseAnimation::Bow,
    ItemUseAnimation::Trident,
    ItemUseAnimation::Crossbow,
    ItemUseAnimation::Spyglass,
    ItemUseAnimation::TootHorn,
    ItemUseAnimation::Brush,
    ItemUseAnimation::Bundle,
    ItemUseAnimation::Spear,
];

impl ItemUseAnimation {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ItemUseAnimation::None => "NONE",
            ItemUseAnimation::Eat => "EAT",
            ItemUseAnimation::Drink => "DRINK",
            ItemUseAnimation::Block => "BLOCK",
            ItemUseAnimation::Bow => "BOW",
            ItemUseAnimation::Trident => "TRIDENT",
            ItemUseAnimation::Crossbow => "CROSSBOW",
            ItemUseAnimation::Spyglass => "SPYGLASS",
            ItemUseAnimation::TootHorn => "TOOT_HORN",
            ItemUseAnimation::Brush => "BRUSH",
            ItemUseAnimation::Bundle => "BUNDLE",
            ItemUseAnimation::Spear => "SPEAR",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ItemUseAnimation#getId()`.
    pub const fn get_id(&self) -> i32 {
        match self {
            ItemUseAnimation::None => 0,
            ItemUseAnimation::Eat => 1,
            ItemUseAnimation::Drink => 2,
            ItemUseAnimation::Block => 3,
            ItemUseAnimation::Bow => 4,
            ItemUseAnimation::Trident => 5,
            ItemUseAnimation::Crossbow => 6,
            ItemUseAnimation::Spyglass => 7,
            ItemUseAnimation::TootHorn => 8,
            ItemUseAnimation::Brush => 9,
            ItemUseAnimation::Bundle => 10,
            ItemUseAnimation::Spear => 11,
        }
    }

    /// Port of `ItemUseAnimation#getSerializedName()` -- the `name` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            ItemUseAnimation::None => "none",
            ItemUseAnimation::Eat => "eat",
            ItemUseAnimation::Drink => "drink",
            ItemUseAnimation::Block => "block",
            ItemUseAnimation::Bow => "bow",
            ItemUseAnimation::Trident => "trident",
            ItemUseAnimation::Crossbow => "crossbow",
            ItemUseAnimation::Spyglass => "spyglass",
            ItemUseAnimation::TootHorn => "toot_horn",
            ItemUseAnimation::Brush => "brush",
            ItemUseAnimation::Bundle => "bundle",
            ItemUseAnimation::Spear => "spear",
        }
    }

    /// Port of `ItemUseAnimation#hasCustomArmTransform()` -- `true` only for `EAT`, `DRINK`, `SPEAR`.
    pub const fn has_custom_arm_transform(&self) -> bool {
        matches!(self, ItemUseAnimation::Eat | ItemUseAnimation::Drink | ItemUseAnimation::Spear)
    }
}

impl std::fmt::Display for ItemUseAnimation {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for ItemUseAnimation {
    fn get_serialized_name(&self) -> &str {
        ItemUseAnimation::get_serialized_name(self)
    }
}

static BY_ID: std::sync::LazyLock<ByIdMap::Continuous<ItemUseAnimation>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(ItemUseAnimation::get_id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Java: `private static final IntFunction<ItemUseAnimation> BY_ID`, applied (the decoding half
/// of `STREAM_CODEC`).
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..12` yields `NONE`.
pub fn by_id(id: i32) -> ItemUseAnimation {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<ItemUseAnimation> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, ItemUseAnimation> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, ItemUseAnimation::getId)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
