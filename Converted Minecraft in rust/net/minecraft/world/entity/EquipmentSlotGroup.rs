//! Port of: net/minecraft/world/entity/EquipmentSlotGroup.java
//! Java class(es): net.minecraft.world.entity.EquipmentSlotGroup
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the eleven constants with `id`, `key`, the `predicate` (as
//! [`EquipmentSlotGroup::test`]) and the derived `slots` list, `BY_ID`, `bySlot`,
//! `getSerializedName`, `slots()` and `Iterable#iterator()` (also as `IntoIterator`). Constants,
//! `id`, `key` and every group's `slots` list are checked against the jar in
//! `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `CODEC` on DataFixerUpper, `STREAM_CODEC` on netty `ByteBuf`.
//!
//! # `slots` IS DERIVED FROM THE PREDICATE, IN `EquipmentSlot` ORDER
//!
//! Java computes `slots = EquipmentSlot.VALUES.stream().filter(predicate).toList()` in the
//! constructor; the port does the same (once, lazily), so iteration order is `EquipmentSlot`'s
//! declaration order: `ARMOR` iterates `FEET, LEGS, CHEST, HEAD, BODY` -- boots first, and
//! `BODY` (animal armour) included because `EquipmentSlot#isArmor` covers `ANIMAL_ARMOR`.
//!
//! # `bySlot` NEVER RETURNS `ANY`, `HAND` OR `ARMOR`
//!
//! It maps each slot to its single-slot group. The three multi-slot groups are only reachable by
//! name or id.

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable;
use crate::net::minecraft::world::entity::EquipmentSlot::{self as equipment_slot, EquipmentSlot};

/// Port of `net.minecraft.world.entity.EquipmentSlotGroup`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EquipmentSlotGroup {
    /// Port of `EquipmentSlotGroup#ANY` -- ordinal 0, id 0, key `"any"`: every slot.
    Any = 0,
    /// Port of `EquipmentSlotGroup#MAINHAND` -- ordinal 1, id 1, key `"mainhand"`.
    Mainhand = 1,
    /// Port of `EquipmentSlotGroup#OFFHAND` -- ordinal 2, id 2, key `"offhand"`.
    Offhand = 2,
    /// Port of `EquipmentSlotGroup#HAND` -- ordinal 3, id 3, key `"hand"`: `type == HAND`.
    Hand = 3,
    /// Port of `EquipmentSlotGroup#FEET` -- ordinal 4, id 4, key `"feet"`.
    Feet = 4,
    /// Port of `EquipmentSlotGroup#LEGS` -- ordinal 5, id 5, key `"legs"`.
    Legs = 5,
    /// Port of `EquipmentSlotGroup#CHEST` -- ordinal 6, id 6, key `"chest"`.
    Chest = 6,
    /// Port of `EquipmentSlotGroup#HEAD` -- ordinal 7, id 7, key `"head"`.
    Head = 7,
    /// Port of `EquipmentSlotGroup#ARMOR` -- ordinal 8, id 8, key `"armor"`: `EquipmentSlot::isArmor`.
    Armor = 8,
    /// Port of `EquipmentSlotGroup#BODY` -- ordinal 9, id 9, key `"body"`.
    Body = 9,
    /// Port of `EquipmentSlotGroup#SADDLE` -- ordinal 10, id 10, key `"saddle"`.
    Saddle = 10,
}

/// Java: `EquipmentSlotGroup.values()` -- declaration order.
pub const VALUES: [EquipmentSlotGroup; 11] = [
    EquipmentSlotGroup::Any,
    EquipmentSlotGroup::Mainhand,
    EquipmentSlotGroup::Offhand,
    EquipmentSlotGroup::Hand,
    EquipmentSlotGroup::Feet,
    EquipmentSlotGroup::Legs,
    EquipmentSlotGroup::Chest,
    EquipmentSlotGroup::Head,
    EquipmentSlotGroup::Armor,
    EquipmentSlotGroup::Body,
    EquipmentSlotGroup::Saddle,
];

/// The `slots` field of every constant, indexed by ordinal: `EquipmentSlot.VALUES` filtered by
/// the constant's predicate, as in the Java constructor.
static SLOTS: std::sync::LazyLock<Vec<Vec<EquipmentSlot>>> = std::sync::LazyLock::new(|| {
    VALUES
        .iter()
        .map(|group| equipment_slot::VALUES.iter().copied().filter(|s| group.test(*s)).collect())
        .collect()
});

impl EquipmentSlotGroup {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            EquipmentSlotGroup::Any => "ANY",
            EquipmentSlotGroup::Mainhand => "MAINHAND",
            EquipmentSlotGroup::Offhand => "OFFHAND",
            EquipmentSlotGroup::Hand => "HAND",
            EquipmentSlotGroup::Feet => "FEET",
            EquipmentSlotGroup::Legs => "LEGS",
            EquipmentSlotGroup::Chest => "CHEST",
            EquipmentSlotGroup::Head => "HEAD",
            EquipmentSlotGroup::Armor => "ARMOR",
            EquipmentSlotGroup::Body => "BODY",
            EquipmentSlotGroup::Saddle => "SADDLE",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `id` field (what `BY_ID` / `STREAM_CODEC` use).
    pub const fn id(&self) -> i32 {
        match self {
            EquipmentSlotGroup::Any => 0,
            EquipmentSlotGroup::Mainhand => 1,
            EquipmentSlotGroup::Offhand => 2,
            EquipmentSlotGroup::Hand => 3,
            EquipmentSlotGroup::Feet => 4,
            EquipmentSlotGroup::Legs => 5,
            EquipmentSlotGroup::Chest => 6,
            EquipmentSlotGroup::Head => 7,
            EquipmentSlotGroup::Armor => 8,
            EquipmentSlotGroup::Body => 9,
            EquipmentSlotGroup::Saddle => 10,
        }
    }

    /// Port of `EquipmentSlotGroup#getSerializedName()` -- the `key` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            EquipmentSlotGroup::Any => "any",
            EquipmentSlotGroup::Mainhand => "mainhand",
            EquipmentSlotGroup::Offhand => "offhand",
            EquipmentSlotGroup::Hand => "hand",
            EquipmentSlotGroup::Feet => "feet",
            EquipmentSlotGroup::Legs => "legs",
            EquipmentSlotGroup::Chest => "chest",
            EquipmentSlotGroup::Head => "head",
            EquipmentSlotGroup::Armor => "armor",
            EquipmentSlotGroup::Body => "body",
            EquipmentSlotGroup::Saddle => "saddle",
        }
    }

    /// Port of `EquipmentSlotGroup#test(EquipmentSlot)` -- the constructor's `predicate`:
    ///
    /// * `ANY`: `slot -> true`;
    /// * `HAND`: `slot.getType() == EquipmentSlot.Type.HAND`;
    /// * `ARMOR`: `EquipmentSlot::isArmor`;
    /// * every other group: `s == slot` for its single slot.
    pub const fn test(&self, slot: EquipmentSlot) -> bool {
        match self {
            EquipmentSlotGroup::Any => true,
            EquipmentSlotGroup::Mainhand => matches!(slot, EquipmentSlot::Mainhand),
            EquipmentSlotGroup::Offhand => matches!(slot, EquipmentSlot::Offhand),
            EquipmentSlotGroup::Hand => matches!(slot.get_type(), equipment_slot::Type::Hand),
            EquipmentSlotGroup::Feet => matches!(slot, EquipmentSlot::Feet),
            EquipmentSlotGroup::Legs => matches!(slot, EquipmentSlot::Legs),
            EquipmentSlotGroup::Chest => matches!(slot, EquipmentSlot::Chest),
            EquipmentSlotGroup::Head => matches!(slot, EquipmentSlot::Head),
            EquipmentSlotGroup::Armor => slot.is_armor(),
            EquipmentSlotGroup::Body => matches!(slot, EquipmentSlot::Body),
            EquipmentSlotGroup::Saddle => matches!(slot, EquipmentSlot::Saddle),
        }
    }

    /// Port of `EquipmentSlotGroup#slots()` -- the matching slots in `EquipmentSlot` order.
    pub fn slots(&self) -> &'static [EquipmentSlot] {
        &SLOTS[*self as usize]
    }

    /// Port of `EquipmentSlotGroup#iterator()` (`Iterable<EquipmentSlot>`): iterates `slots()`.
    pub fn iterator(&self) -> std::iter::Copied<std::slice::Iter<'static, EquipmentSlot>> {
        self.slots().iter().copied()
    }
}

impl IntoIterator for EquipmentSlotGroup {
    type Item = EquipmentSlot;
    type IntoIter = std::iter::Copied<std::slice::Iter<'static, EquipmentSlot>>;

    /// Java's enhanced `for` over an `Iterable<EquipmentSlot>`.
    fn into_iter(self) -> Self::IntoIter {
        self.iterator()
    }
}

impl std::fmt::Display for EquipmentSlotGroup {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for EquipmentSlotGroup {
    fn get_serialized_name(&self) -> &str {
        EquipmentSlotGroup::get_serialized_name(self)
    }
}

static BY_ID_MAP: std::sync::LazyLock<ByIdMap::Continuous<EquipmentSlotGroup>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(EquipmentSlotGroup::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Java: `public static final IntFunction<EquipmentSlotGroup> BY_ID`, applied.
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..11` yields `ANY`.
pub fn BY_ID(id: i32) -> EquipmentSlotGroup {
    BY_ID_MAP.apply(id)
}

/// Port of `EquipmentSlotGroup#bySlot(EquipmentSlot)`: the single-slot group for `slot`.
pub const fn by_slot(slot: EquipmentSlot) -> EquipmentSlotGroup {
    match slot {
        EquipmentSlot::Mainhand => EquipmentSlotGroup::Mainhand,
        EquipmentSlot::Offhand => EquipmentSlotGroup::Offhand,
        EquipmentSlot::Feet => EquipmentSlotGroup::Feet,
        EquipmentSlot::Legs => EquipmentSlotGroup::Legs,
        EquipmentSlot::Chest => EquipmentSlotGroup::Chest,
        EquipmentSlot::Head => EquipmentSlotGroup::Head,
        EquipmentSlot::Body => EquipmentSlotGroup::Body,
        EquipmentSlot::Saddle => EquipmentSlotGroup::Saddle,
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<EquipmentSlotGroup> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, EquipmentSlotGroup> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, s -> s.id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`. Decoding is [`BY_ID`], encoding is
/// [`EquipmentSlotGroup::id`].
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
