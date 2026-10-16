//! Port of: net/minecraft/world/item/SwingAnimationType.java
//! Java class(es): net.minecraft.world.item.SwingAnimationType
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the three constants with `id` and `name`, the private `BY_ID`
//! (as [`by_id`]), `getId`, `getSerializedName`. Constants and fields are checked against the
//! jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `CODEC` on DataFixerUpper, `STREAM_CODEC` on netty `ByteBuf`.

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.item.SwingAnimationType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SwingAnimationType {
    /// Port of `SwingAnimationType#NONE` -- ordinal 0, id 0, `"none"`.
    None = 0,
    /// Port of `SwingAnimationType#WHACK` -- ordinal 1, id 1, `"whack"`.
    Whack = 1,
    /// Port of `SwingAnimationType#STAB` -- ordinal 2, id 2, `"stab"`.
    Stab = 2,
}

/// Java: `SwingAnimationType.values()` -- declaration order.
pub const VALUES: [SwingAnimationType; 3] = [
    SwingAnimationType::None,
    SwingAnimationType::Whack,
    SwingAnimationType::Stab,
];

impl SwingAnimationType {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            SwingAnimationType::None => "NONE",
            SwingAnimationType::Whack => "WHACK",
            SwingAnimationType::Stab => "STAB",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `SwingAnimationType#getId()`.
    pub const fn get_id(&self) -> i32 {
        match self {
            SwingAnimationType::None => 0,
            SwingAnimationType::Whack => 1,
            SwingAnimationType::Stab => 2,
        }
    }

    /// Port of `SwingAnimationType#getSerializedName()` -- the `name` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            SwingAnimationType::None => "none",
            SwingAnimationType::Whack => "whack",
            SwingAnimationType::Stab => "stab",
        }
    }
}

impl std::fmt::Display for SwingAnimationType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for SwingAnimationType {
    fn get_serialized_name(&self) -> &str {
        SwingAnimationType::get_serialized_name(self)
    }
}

static BY_ID: std::sync::LazyLock<ByIdMap::Continuous<SwingAnimationType>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(SwingAnimationType::get_id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Java: `private static final IntFunction<SwingAnimationType> BY_ID`, applied (the decoding
/// half of `STREAM_CODEC`).
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..3` yields `NONE`.
pub fn by_id(id: i32) -> SwingAnimationType {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<SwingAnimationType> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, SwingAnimationType> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, SwingAnimationType::getId)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
