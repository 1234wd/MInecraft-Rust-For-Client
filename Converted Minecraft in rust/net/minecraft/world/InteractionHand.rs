//! Port of: net/minecraft/world/InteractionHand.java
//! Java class(es): net.minecraft.world.InteractionHand
//! Status: VERIFIED
//!
//! Two constants, each carrying a private `int id`. Verified against the jar by
//! `_porting/tests/consts.rs`: ordinals plus the `id` field for every constant.
//!
//! # `asEquipmentSlot` IS PORT-BLOCKED ON `EquipmentSlot`
//!
//! ```java
//! return this == MAIN_HAND ? EquipmentSlot.MAINHAND : EquipmentSlot.OFFHAND;
//! ```
//!
//! `net.minecraft.world.entity.EquipmentSlot` is still a SKELETON. The method is documented here
//! for the caller; once `EquipmentSlot` is ported, `as_equipment_slot` can be completed.
//!
//! # `BY_ID` AND `STREAM_CODEC` ARE PORT-BLOCKED
//!
//! `ByIdMap` and `ByteBufCodecs` are not ported. `OutOfBoundsStrategy.ZERO` means an unknown id
//! yields `MAIN_HAND`.

/// Port of `net.minecraft.world.InteractionHand`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InteractionHand {
    /// Port of `InteractionHand#MAIN_HAND` -- ordinal 0, id `0`.
    MainHand = 0,
    /// Port of `InteractionHand#OFF_HAND` -- ordinal 1, id `1`.
    OffHand = 1,
}

/// Java: `InteractionHand.values()` -- declaration order.
pub const VALUES: [InteractionHand; 2] = [InteractionHand::MainHand, InteractionHand::OffHand];

impl InteractionHand {
    /// Port of the private `id` field.
    ///
    /// The id equals the ordinal for this enum; they are modelled separately so any future
    /// divergence is explicit.
    #[inline]
    pub const fn id(&self) -> i32 {
        match self {
            InteractionHand::MainHand => 0,
            InteractionHand::OffHand => 1,
        }
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public InteractionHand#asEquipmentSlot()`.
///
/// PORT-BLOCKED on `net.minecraft.world.entity.EquipmentSlot`, which is still a SKELETON.
pub fn as_equipment_slot(_hand: InteractionHand) {
    todo!("PORT-BLOCKED: net.minecraft.world.entity.EquipmentSlot is not ported")
}

/// Java: `private static final IntFunction<InteractionHand> BY_ID`.
///
/// PORT-BLOCKED on `net.minecraft.util.ByIdMap`. `OutOfBoundsStrategy.ZERO` means an unknown id
/// yields `MAIN_HAND`.
pub fn by_id(_id: i32) -> InteractionHand {
    todo!("PORT-BLOCKED: net.minecraft.util.ByIdMap is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, InteractionHand> STREAM_CODEC`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
