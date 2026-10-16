//! Port of: net/minecraft/world/entity/HumanoidArm.java
//! Java class(es): net.minecraft.world.entity.HumanoidArm
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the two constants with `id` and `name`, the private `BY_ID` (as
//! [`by_id`]), `getOpposite`, `getSerializedName`, and the caption's translation key. Constants,
//! `id`, `name` and the `caption`'s key are checked against the jar in
//! `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `caption()` on `net.minecraft.network.chat.Component`; `CODEC` on
//! DataFixerUpper; `STREAM_CODEC` on netty `ByteBuf`.

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.entity.HumanoidArm`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HumanoidArm {
    /// Port of `HumanoidArm#LEFT` -- ordinal 0, id 0, `"left"`.
    Left = 0,
    /// Port of `HumanoidArm#RIGHT` -- ordinal 1, id 1, `"right"`.
    Right = 1,
}

/// Java: `HumanoidArm.values()` -- declaration order.
pub const VALUES: [HumanoidArm; 2] = [HumanoidArm::Left, HumanoidArm::Right];

impl HumanoidArm {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            HumanoidArm::Left => "LEFT",
            HumanoidArm::Right => "RIGHT",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `id` field (read by `BY_ID` / `STREAM_CODEC`).
    pub const fn id(&self) -> i32 {
        match self {
            HumanoidArm::Left => 0,
            HumanoidArm::Right => 1,
        }
    }

    /// Port of `HumanoidArm#getOpposite()`.
    pub const fn get_opposite(&self) -> HumanoidArm {
        match self {
            HumanoidArm::Left => HumanoidArm::Right,
            HumanoidArm::Right => HumanoidArm::Left,
        }
    }

    /// Port of `HumanoidArm#getSerializedName()` -- the `name` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            HumanoidArm::Left => "left",
            HumanoidArm::Right => "right",
        }
    }

    /// The constructor's `translationKey` argument. Java does not keep it as a field; it goes
    /// straight into `caption = Component.translatable(translationKey)`. Exposed so `caption()`
    /// can be built once `Component` is ported.
    pub const fn caption_translation_key(&self) -> &'static str {
        match self {
            HumanoidArm::Left => "options.mainHand.left",
            HumanoidArm::Right => "options.mainHand.right",
        }
    }
}

impl std::fmt::Display for HumanoidArm {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for HumanoidArm {
    fn get_serialized_name(&self) -> &str {
        HumanoidArm::get_serialized_name(self)
    }
}

static BY_ID: std::sync::LazyLock<ByIdMap::Continuous<HumanoidArm>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(HumanoidArm::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Java: `private static final IntFunction<HumanoidArm> BY_ID`, applied (the decoding half of
/// `STREAM_CODEC`).
///
/// `OutOfBoundsStrategy.ZERO`: any id other than 0 or 1 yields `LEFT`.
pub fn by_id(id: i32) -> HumanoidArm {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public Component caption()` -- `Component.translatable(translationKey)`.
///
/// PORT-BLOCKED on `net.minecraft.network.chat.Component`. The key is
/// [`HumanoidArm::caption_translation_key`].
pub fn caption(_arm: HumanoidArm) {
    todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
}

/// Java: `public static final Codec<HumanoidArm> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, HumanoidArm> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, a -> a.id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
