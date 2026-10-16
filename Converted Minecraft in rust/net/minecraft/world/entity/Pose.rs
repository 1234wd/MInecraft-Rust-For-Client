//! Port of: net/minecraft/world/entity/Pose.java
//! Java class(es): net.minecraft.world.entity.Pose
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the eighteen constants with `id` and `name`, `BY_ID`, `id()`,
//! `getSerializedName`. Constants and fields are checked against the jar in
//! `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `CODEC` on DataFixerUpper, `STREAM_CODEC` on netty `ByteBuf` (the entity
//! data serializer for poses goes through it).

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.entity.Pose`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pose {
    /// Port of `Pose#STANDING` -- ordinal 0, id 0.
    Standing = 0,
    /// Port of `Pose#FALL_FLYING` -- ordinal 1, id 1.
    FallFlying = 1,
    /// Port of `Pose#SLEEPING` -- ordinal 2, id 2.
    Sleeping = 2,
    /// Port of `Pose#SWIMMING` -- ordinal 3, id 3.
    Swimming = 3,
    /// Port of `Pose#SPIN_ATTACK` -- ordinal 4, id 4.
    SpinAttack = 4,
    /// Port of `Pose#CROUCHING` -- ordinal 5, id 5.
    Crouching = 5,
    /// Port of `Pose#LONG_JUMPING` -- ordinal 6, id 6.
    LongJumping = 6,
    /// Port of `Pose#DYING` -- ordinal 7, id 7.
    Dying = 7,
    /// Port of `Pose#CROAKING` -- ordinal 8, id 8.
    Croaking = 8,
    /// Port of `Pose#USING_TONGUE` -- ordinal 9, id 9.
    UsingTongue = 9,
    /// Port of `Pose#SITTING` -- ordinal 10, id 10.
    Sitting = 10,
    /// Port of `Pose#ROARING` -- ordinal 11, id 11.
    Roaring = 11,
    /// Port of `Pose#SNIFFING` -- ordinal 12, id 12.
    Sniffing = 12,
    /// Port of `Pose#EMERGING` -- ordinal 13, id 13.
    Emerging = 13,
    /// Port of `Pose#DIGGING` -- ordinal 14, id 14.
    Digging = 14,
    /// Port of `Pose#SLIDING` -- ordinal 15, id 15.
    Sliding = 15,
    /// Port of `Pose#SHOOTING` -- ordinal 16, id 16.
    Shooting = 16,
    /// Port of `Pose#INHALING` -- ordinal 17, id 17.
    Inhaling = 17,
}

/// Java: `Pose.values()` -- declaration order.
pub const VALUES: [Pose; 18] = [
    Pose::Standing,
    Pose::FallFlying,
    Pose::Sleeping,
    Pose::Swimming,
    Pose::SpinAttack,
    Pose::Crouching,
    Pose::LongJumping,
    Pose::Dying,
    Pose::Croaking,
    Pose::UsingTongue,
    Pose::Sitting,
    Pose::Roaring,
    Pose::Sniffing,
    Pose::Emerging,
    Pose::Digging,
    Pose::Sliding,
    Pose::Shooting,
    Pose::Inhaling,
];

impl Pose {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Pose::Standing => "STANDING",
            Pose::FallFlying => "FALL_FLYING",
            Pose::Sleeping => "SLEEPING",
            Pose::Swimming => "SWIMMING",
            Pose::SpinAttack => "SPIN_ATTACK",
            Pose::Crouching => "CROUCHING",
            Pose::LongJumping => "LONG_JUMPING",
            Pose::Dying => "DYING",
            Pose::Croaking => "CROAKING",
            Pose::UsingTongue => "USING_TONGUE",
            Pose::Sitting => "SITTING",
            Pose::Roaring => "ROARING",
            Pose::Sniffing => "SNIFFING",
            Pose::Emerging => "EMERGING",
            Pose::Digging => "DIGGING",
            Pose::Sliding => "SLIDING",
            Pose::Shooting => "SHOOTING",
            Pose::Inhaling => "INHALING",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Pose#id()`.
    pub const fn id(&self) -> i32 {
        match self {
            Pose::Standing => 0,
            Pose::FallFlying => 1,
            Pose::Sleeping => 2,
            Pose::Swimming => 3,
            Pose::SpinAttack => 4,
            Pose::Crouching => 5,
            Pose::LongJumping => 6,
            Pose::Dying => 7,
            Pose::Croaking => 8,
            Pose::UsingTongue => 9,
            Pose::Sitting => 10,
            Pose::Roaring => 11,
            Pose::Sniffing => 12,
            Pose::Emerging => 13,
            Pose::Digging => 14,
            Pose::Sliding => 15,
            Pose::Shooting => 16,
            Pose::Inhaling => 17,
        }
    }

    /// Port of `Pose#getSerializedName()` -- the `name` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            Pose::Standing => "standing",
            Pose::FallFlying => "fall_flying",
            Pose::Sleeping => "sleeping",
            Pose::Swimming => "swimming",
            Pose::SpinAttack => "spin_attack",
            Pose::Crouching => "crouching",
            Pose::LongJumping => "long_jumping",
            Pose::Dying => "dying",
            Pose::Croaking => "croaking",
            Pose::UsingTongue => "using_tongue",
            Pose::Sitting => "sitting",
            Pose::Roaring => "roaring",
            Pose::Sniffing => "sniffing",
            Pose::Emerging => "emerging",
            Pose::Digging => "digging",
            Pose::Sliding => "sliding",
            Pose::Shooting => "shooting",
            Pose::Inhaling => "inhaling",
        }
    }
}

impl std::fmt::Display for Pose {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for Pose {
    fn get_serialized_name(&self) -> &str {
        Pose::get_serialized_name(self)
    }
}

static BY_ID_MAP: std::sync::LazyLock<ByIdMap::Continuous<Pose>> =
    std::sync::LazyLock::new(|| ByIdMap::continuous(Pose::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero));

/// Java: `public static final IntFunction<Pose> BY_ID`, applied.
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..18` yields `STANDING`.
pub fn BY_ID(id: i32) -> Pose {
    BY_ID_MAP.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<Pose> CODEC = StringRepresentable.fromEnum(Pose::values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`. (With 18 values, more than
/// `PRE_BUILT_MAP_THRESHOLD`, its name lookup would be the `HashMap` branch.)
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, Pose> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, Pose::id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
