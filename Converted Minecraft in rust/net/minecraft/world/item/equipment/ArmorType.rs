//! Port of: net/minecraft/world/item/equipment/ArmorType.java
//! Java class(es): net.minecraft.world.item.equipment.ArmorType
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the five constants with their `slot`, `name` and `unitDurability`
//! fields, `getDurability(int)`, `getSlot`, `getName`, `getSerializedName`. Constants and all
//! three fields are checked against the jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `CODEC` (`StringRepresentable.fromValues`, a bare DFU `Codec`).
//!
//! # NOTE THE CONSTRUCTOR ARGUMENT ORDER
//!
//! Java declares `ArmorType(slot, unitDurability, name)` but assigns the fields in the order
//! `slot, name, unitDurability`. Only the values matter; they are pinned against the jar.
//! `getDurability` multiplies in Java `int`, so it wraps on overflow.

use crate::net::minecraft::util::StringRepresentable;
use crate::net::minecraft::world::entity::EquipmentSlot::EquipmentSlot;

/// Port of `net.minecraft.world.item.equipment.ArmorType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArmorType {
    /// Port of `ArmorType#HELMET` -- ordinal 0: slot `HEAD`, unit durability 11.
    Helmet = 0,
    /// Port of `ArmorType#CHESTPLATE` -- ordinal 1: slot `CHEST`, unit durability 16.
    Chestplate = 1,
    /// Port of `ArmorType#LEGGINGS` -- ordinal 2: slot `LEGS`, unit durability 15.
    Leggings = 2,
    /// Port of `ArmorType#BOOTS` -- ordinal 3: slot `FEET`, unit durability 13.
    Boots = 3,
    /// Port of `ArmorType#BODY` -- ordinal 4: slot `BODY`, unit durability 16.
    Body = 4,
}

/// Java: `ArmorType.values()` -- declaration order.
pub const VALUES: [ArmorType; 5] = [
    ArmorType::Helmet,
    ArmorType::Chestplate,
    ArmorType::Leggings,
    ArmorType::Boots,
    ArmorType::Body,
];

impl ArmorType {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ArmorType::Helmet => "HELMET",
            ArmorType::Chestplate => "CHESTPLATE",
            ArmorType::Leggings => "LEGGINGS",
            ArmorType::Boots => "BOOTS",
            ArmorType::Body => "BODY",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `unitDurability` field.
    pub const fn unit_durability(&self) -> i32 {
        match self {
            ArmorType::Helmet => 11,
            ArmorType::Chestplate => 16,
            ArmorType::Leggings => 15,
            ArmorType::Boots => 13,
            ArmorType::Body => 16,
        }
    }

    /// Port of `ArmorType#getDurability(int)`: `unitDurability * multiplier` in Java `int`
    /// arithmetic (wraps on overflow).
    pub const fn get_durability(&self, multiplier: i32) -> i32 {
        self.unit_durability().wrapping_mul(multiplier)
    }

    /// Port of `ArmorType#getSlot()`.
    pub const fn get_slot(&self) -> EquipmentSlot {
        match self {
            ArmorType::Helmet => EquipmentSlot::Head,
            ArmorType::Chestplate => EquipmentSlot::Chest,
            ArmorType::Leggings => EquipmentSlot::Legs,
            ArmorType::Boots => EquipmentSlot::Feet,
            ArmorType::Body => EquipmentSlot::Body,
        }
    }

    /// Port of `ArmorType#getName()` -- the `name` field.
    pub const fn get_name(&self) -> &'static str {
        match self {
            ArmorType::Helmet => "helmet",
            ArmorType::Chestplate => "chestplate",
            ArmorType::Leggings => "leggings",
            ArmorType::Boots => "boots",
            ArmorType::Body => "body",
        }
    }

    /// Port of `ArmorType#getSerializedName()` -- the `name` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        self.get_name()
    }
}

impl std::fmt::Display for ArmorType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for ArmorType {
    fn get_serialized_name(&self) -> &str {
        ArmorType::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<ArmorType> CODEC = StringRepresentable.fromValues(ArmorType::values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (`fromValues` returns a bare DFU codec).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
