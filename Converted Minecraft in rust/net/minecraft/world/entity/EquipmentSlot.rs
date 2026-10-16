//! Port of: net/minecraft/world/entity/EquipmentSlot.java
//! Java class(es): net.minecraft.world.entity.EquipmentSlot, net.minecraft.world.entity.Type
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: all eight constants with their five fields (`type`, `index`,
//! `countLimit`, `id`, `name`), the nested `EquipmentSlot.Type` enum, `NO_COUNT_LIMIT`, `VALUES`,
//! `BY_ID`, every getter, `getIndex(int)`, `getFilterBit(int)`, `isArmor`,
//! `canIncreaseExperience`, `byName` (including its exception) and the name-resolution half of
//! `CODEC`. Constants and fields are checked against the jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `limit(ItemStack)` on `net.minecraft.world.item.ItemStack`; the `Codec` half of
//! `CODEC` on DataFixerUpper; `STREAM_CODEC` on netty `ByteBuf`.
//!
//! # ID, INDEX AND ORDINAL ARE THREE DIFFERENT NUMBERINGS
//!
//! * the ordinal is declaration order (`MAINHAND` 0 ... `SADDLE` 7);
//! * `index` is the position *within the slot's `Type`* (`FEET` 0 ... `HEAD` 3, `OFFHAND` 1,
//!   `BODY` 0, `SADDLE` 0) -- it indexes the per-type item lists on `LivingEntity`;
//! * `id` (Java's `filterFlag` constructor parameter) is the wire id that `BY_ID` maps from:
//!   `MAINHAND` 0, `FEET` 1, `LEGS` 2, `CHEST` 3, `HEAD` 4, `OFFHAND` **5**, `BODY` 6,
//!   `SADDLE` 7. So `BY_ID(1)` is `FEET`, not `OFFHAND`.
//!
//! # THE 4-ARGUMENT CONSTRUCTOR MEANS "NO COUNT LIMIT"
//!
//! `EquipmentSlot(type, index, filterFlag, name)` delegates with `countLimit = 0`, so only the
//! two hand slots have `countLimit == NO_COUNT_LIMIT`; every armour, body and saddle slot has a
//! limit of 1 (which is what `limit(ItemStack)` uses to split a stack down to one item).

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.entity.EquipmentSlot`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    /// Port of `EquipmentSlot#MAINHAND` -- ordinal 0: `HAND`, index 0, no count limit, id 0.
    Mainhand = 0,
    /// Port of `EquipmentSlot#OFFHAND` -- ordinal 1: `HAND`, index 1, no count limit, id 5.
    Offhand = 1,
    /// Port of `EquipmentSlot#FEET` -- ordinal 2: `HUMANOID_ARMOR`, index 0, limit 1, id 1.
    Feet = 2,
    /// Port of `EquipmentSlot#LEGS` -- ordinal 3: `HUMANOID_ARMOR`, index 1, limit 1, id 2.
    Legs = 3,
    /// Port of `EquipmentSlot#CHEST` -- ordinal 4: `HUMANOID_ARMOR`, index 2, limit 1, id 3.
    Chest = 4,
    /// Port of `EquipmentSlot#HEAD` -- ordinal 5: `HUMANOID_ARMOR`, index 3, limit 1, id 4.
    Head = 5,
    /// Port of `EquipmentSlot#BODY` -- ordinal 6: `ANIMAL_ARMOR`, index 0, limit 1, id 6.
    Body = 6,
    /// Port of `EquipmentSlot#SADDLE` -- ordinal 7: `SADDLE`, index 0, limit 1, id 7.
    Saddle = 7,
}

/// Java: `public static final int NO_COUNT_LIMIT = 0`.
pub const NO_COUNT_LIMIT: i32 = 0;

/// Java: `public static final List<EquipmentSlot> VALUES = List.of(values())` -- declaration order.
pub const VALUES: [EquipmentSlot; 8] = [
    EquipmentSlot::Mainhand,
    EquipmentSlot::Offhand,
    EquipmentSlot::Feet,
    EquipmentSlot::Legs,
    EquipmentSlot::Chest,
    EquipmentSlot::Head,
    EquipmentSlot::Body,
    EquipmentSlot::Saddle,
];

/// Port of the nested enum `net.minecraft.world.entity.EquipmentSlot.Type`.
///
/// Nested classes are not in the constant dump; the ordinals follow the source's declaration
/// order, and each slot's `type` field (recorded by `name()`) is checked against the jar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    /// Port of `EquipmentSlot.Type#HAND` -- ordinal 0.
    Hand = 0,
    /// Port of `EquipmentSlot.Type#HUMANOID_ARMOR` -- ordinal 1.
    HumanoidArmor = 1,
    /// Port of `EquipmentSlot.Type#ANIMAL_ARMOR` -- ordinal 2.
    AnimalArmor = 2,
    /// Port of `EquipmentSlot.Type#SADDLE` -- ordinal 3.
    Saddle = 3,
}

/// Java: `EquipmentSlot.Type.values()` -- declaration order.
pub const TYPE_VALUES: [Type; 4] = [Type::Hand, Type::HumanoidArmor, Type::AnimalArmor, Type::Saddle];

impl Type {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Type::Hand => "HAND",
            Type::HumanoidArmor => "HUMANOID_ARMOR",
            Type::AnimalArmor => "ANIMAL_ARMOR",
            Type::Saddle => "SADDLE",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for Type {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl EquipmentSlot {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            EquipmentSlot::Mainhand => "MAINHAND",
            EquipmentSlot::Offhand => "OFFHAND",
            EquipmentSlot::Feet => "FEET",
            EquipmentSlot::Legs => "LEGS",
            EquipmentSlot::Chest => "CHEST",
            EquipmentSlot::Head => "HEAD",
            EquipmentSlot::Body => "BODY",
            EquipmentSlot::Saddle => "SADDLE",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `EquipmentSlot#getType()`.
    pub const fn get_type(&self) -> Type {
        match self {
            EquipmentSlot::Mainhand | EquipmentSlot::Offhand => Type::Hand,
            EquipmentSlot::Feet | EquipmentSlot::Legs | EquipmentSlot::Chest | EquipmentSlot::Head => {
                Type::HumanoidArmor
            }
            EquipmentSlot::Body => Type::AnimalArmor,
            EquipmentSlot::Saddle => Type::Saddle,
        }
    }

    /// Port of `EquipmentSlot#getIndex()` -- the position within the slot's `Type`.
    pub const fn get_index(&self) -> i32 {
        match self {
            EquipmentSlot::Mainhand => 0,
            EquipmentSlot::Offhand => 1,
            EquipmentSlot::Feet => 0,
            EquipmentSlot::Legs => 1,
            EquipmentSlot::Chest => 2,
            EquipmentSlot::Head => 3,
            EquipmentSlot::Body => 0,
            EquipmentSlot::Saddle => 0,
        }
    }

    /// Port of `EquipmentSlot#getIndex(int)`: `base + this.index`, with Java `int` wrap-around.
    pub const fn get_index_i32(&self, base: i32) -> i32 {
        base.wrapping_add(self.get_index())
    }

    /// Port of the private `countLimit` field (read by `limit(ItemStack)`).
    ///
    /// `NO_COUNT_LIMIT` (0) for the two hand slots, 1 for every other slot.
    pub const fn count_limit(&self) -> i32 {
        match self {
            EquipmentSlot::Mainhand | EquipmentSlot::Offhand => NO_COUNT_LIMIT,
            EquipmentSlot::Feet
            | EquipmentSlot::Legs
            | EquipmentSlot::Chest
            | EquipmentSlot::Head
            | EquipmentSlot::Body
            | EquipmentSlot::Saddle => 1,
        }
    }

    /// Port of `EquipmentSlot#getId()` -- the wire id `BY_ID` maps from (not the ordinal).
    pub const fn get_id(&self) -> i32 {
        match self {
            EquipmentSlot::Mainhand => 0,
            EquipmentSlot::Offhand => 5,
            EquipmentSlot::Feet => 1,
            EquipmentSlot::Legs => 2,
            EquipmentSlot::Chest => 3,
            EquipmentSlot::Head => 4,
            EquipmentSlot::Body => 6,
            EquipmentSlot::Saddle => 7,
        }
    }

    /// Port of `EquipmentSlot#getFilterBit(int)`: `this.id + offset`, with Java `int` wrap-around.
    pub const fn get_filter_bit(&self, offset: i32) -> i32 {
        self.get_id().wrapping_add(offset)
    }

    /// Port of `EquipmentSlot#getName()` -- the `name` field (same as the serialized name).
    pub const fn get_name(&self) -> &'static str {
        match self {
            EquipmentSlot::Mainhand => "mainhand",
            EquipmentSlot::Offhand => "offhand",
            EquipmentSlot::Feet => "feet",
            EquipmentSlot::Legs => "legs",
            EquipmentSlot::Chest => "chest",
            EquipmentSlot::Head => "head",
            EquipmentSlot::Body => "body",
            EquipmentSlot::Saddle => "saddle",
        }
    }

    /// Port of `EquipmentSlot#isArmor()`: `HUMANOID_ARMOR` or `ANIMAL_ARMOR` (so `BODY` is armour,
    /// `SADDLE` is not).
    pub const fn is_armor(&self) -> bool {
        matches!(self.get_type(), Type::HumanoidArmor | Type::AnimalArmor)
    }

    /// Port of `EquipmentSlot#getSerializedName()` -- the `name` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        self.get_name()
    }

    /// Port of `EquipmentSlot#canIncreaseExperience()`: every type except `SADDLE`.
    pub const fn can_increase_experience(&self) -> bool {
        !matches!(self.get_type(), Type::Saddle)
    }
}

impl std::fmt::Display for EquipmentSlot {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for EquipmentSlot {
    fn get_serialized_name(&self) -> &str {
        EquipmentSlot::get_serialized_name(self)
    }
}

static BY_ID_MAP: std::sync::LazyLock<ByIdMap::Continuous<EquipmentSlot>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(EquipmentSlot::get_id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// The name-resolution half of `EquipmentSlot.CODEC` (`StringRepresentable.fromEnum(values)`).
static CODEC_BY_NAME: std::sync::LazyLock<StringRepresentable::EnumCodec<EquipmentSlot>> =
    std::sync::LazyLock::new(|| StringRepresentable::from_enum(&VALUES));

/// Java: `public static final IntFunction<EquipmentSlot> BY_ID`, applied.
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..8` yields `MAINHAND` (the slot whose id is 0).
pub fn BY_ID(id: i32) -> EquipmentSlot {
    BY_ID_MAP.apply(id)
}

/// Port of `EquipmentSlot#byName(String)`.
///
/// `CODEC.byName(name)`, exact and case-sensitive. An unknown name panics with Java's
/// `java.lang.IllegalArgumentException: Invalid slot '<name>'`.
pub fn by_name(name: &str) -> EquipmentSlot {
    match CODEC_BY_NAME.by_name(name) {
        Some(slot) => slot,
        None => panic!("java.lang.IllegalArgumentException: Invalid slot '{name}'"),
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public ItemStack limit(ItemStack toEquip)` --
/// `countLimit > 0 ? toEquip.split(countLimit) : toEquip`.
///
/// PORT-BLOCKED on `net.minecraft.world.item.ItemStack`. The limit it uses is
/// [`EquipmentSlot::count_limit`].
pub fn limit(_slot: EquipmentSlot) {
    todo!("PORT-BLOCKED: net.minecraft.world.item.ItemStack is not ported")
}

/// Java: `public static final StringRepresentable.EnumCodec<EquipmentSlot> CODEC`.
///
/// The `Codec` half is PORT-BLOCKED on `com.mojang.serialization.Codec`; name resolution is
/// available through [`by_name`].
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, EquipmentSlot> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, s -> s.id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`. Decoding is [`BY_ID`], encoding is
/// [`EquipmentSlot::get_id`].
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
