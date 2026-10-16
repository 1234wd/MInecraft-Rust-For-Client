//! Port of: net/minecraft/world/entity/player/PlayerModelPart.java
//! Java class(es): net.minecraft.world.entity.player.PlayerModelPart
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the seven constants with `bit`, `mask` (`1 << bit`) and `id`,
//! `getMask`, `getBit`, `getId`, `getSerializedName`, and the translation key behind `getName`.
//! Constants, `bit`, `mask`, `id` and the `name` component's key are checked against the jar in
//! `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `getName()` on `net.minecraft.network.chat.Component`; `CODEC` on
//! DataFixerUpper.
//!
//! # THE MASKS ARE A WIRE / OPTIONS BIT FIELD
//!
//! The client sends its enabled parts as one byte of OR-ed masks (and saves them in the options
//! file per part). `bit` is the ordinal today, but `mask` is derived from `bit`, not from the
//! ordinal, so the port computes it from [`PlayerModelPart::get_bit`] with Java's `int` shift
//! (`1 << bit`, shift distance masked to 5 bits).

use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.entity.player.PlayerModelPart`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayerModelPart {
    /// Port of `PlayerModelPart#CAPE` -- ordinal 0, bit 0 (mask `0x01`).
    Cape = 0,
    /// Port of `PlayerModelPart#JACKET` -- ordinal 1, bit 1 (mask `0x02`).
    Jacket = 1,
    /// Port of `PlayerModelPart#LEFT_SLEEVE` -- ordinal 2, bit 2 (mask `0x04`).
    LeftSleeve = 2,
    /// Port of `PlayerModelPart#RIGHT_SLEEVE` -- ordinal 3, bit 3 (mask `0x08`).
    RightSleeve = 3,
    /// Port of `PlayerModelPart#LEFT_PANTS_LEG` -- ordinal 4, bit 4 (mask `0x10`).
    LeftPantsLeg = 4,
    /// Port of `PlayerModelPart#RIGHT_PANTS_LEG` -- ordinal 5, bit 5 (mask `0x20`).
    RightPantsLeg = 5,
    /// Port of `PlayerModelPart#HAT` -- ordinal 6, bit 6 (mask `0x40`).
    Hat = 6,
}

/// Java: `PlayerModelPart.values()` -- declaration order.
pub const VALUES: [PlayerModelPart; 7] = [
    PlayerModelPart::Cape,
    PlayerModelPart::Jacket,
    PlayerModelPart::LeftSleeve,
    PlayerModelPart::RightSleeve,
    PlayerModelPart::LeftPantsLeg,
    PlayerModelPart::RightPantsLeg,
    PlayerModelPart::Hat,
];

impl PlayerModelPart {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PlayerModelPart::Cape => "CAPE",
            PlayerModelPart::Jacket => "JACKET",
            PlayerModelPart::LeftSleeve => "LEFT_SLEEVE",
            PlayerModelPart::RightSleeve => "RIGHT_SLEEVE",
            PlayerModelPart::LeftPantsLeg => "LEFT_PANTS_LEG",
            PlayerModelPart::RightPantsLeg => "RIGHT_PANTS_LEG",
            PlayerModelPart::Hat => "HAT",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `PlayerModelPart#getBit()`.
    pub const fn get_bit(&self) -> i32 {
        match self {
            PlayerModelPart::Cape => 0,
            PlayerModelPart::Jacket => 1,
            PlayerModelPart::LeftSleeve => 2,
            PlayerModelPart::RightSleeve => 3,
            PlayerModelPart::LeftPantsLeg => 4,
            PlayerModelPart::RightPantsLeg => 5,
            PlayerModelPart::Hat => 6,
        }
    }

    /// Port of `PlayerModelPart#getMask()` -- the `mask` field, `1 << bit` in Java `int`
    /// arithmetic.
    pub const fn get_mask(&self) -> i32 {
        1i32.wrapping_shl(self.get_bit() as u32)
    }

    /// Port of `PlayerModelPart#getId()` -- the `id` field (the constructor's `name` argument).
    pub const fn get_id(&self) -> &'static str {
        match self {
            PlayerModelPart::Cape => "cape",
            PlayerModelPart::Jacket => "jacket",
            PlayerModelPart::LeftSleeve => "left_sleeve",
            PlayerModelPart::RightSleeve => "right_sleeve",
            PlayerModelPart::LeftPantsLeg => "left_pants_leg",
            PlayerModelPart::RightPantsLeg => "right_pants_leg",
            PlayerModelPart::Hat => "hat",
        }
    }

    /// Port of `PlayerModelPart#getSerializedName()` -- the `id` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        self.get_id()
    }

    /// The translation key of the `name` component: `"options.modelPart." + name`, as built in
    /// the Java constructor. Exposed so `getName()` can be built once `Component` is ported.
    pub fn name_translation_key(&self) -> String {
        format!("options.modelPart.{}", self.get_id())
    }
}

impl std::fmt::Display for PlayerModelPart {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for PlayerModelPart {
    fn get_serialized_name(&self) -> &str {
        PlayerModelPart::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public Component getName()` -- `Component.translatable("options.modelPart." + name)`.
///
/// PORT-BLOCKED on `net.minecraft.network.chat.Component`. The key is
/// [`PlayerModelPart::name_translation_key`].
pub fn get_name(_part: PlayerModelPart) {
    todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
}

/// Java: `public static final Codec<PlayerModelPart> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
