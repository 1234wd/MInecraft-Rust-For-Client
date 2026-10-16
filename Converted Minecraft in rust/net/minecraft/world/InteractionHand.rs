//! Port of: net/minecraft/world/InteractionHand.java
//! Java class(es): net.minecraft.world.InteractionHand
//! Status: VERIFIED
//!
//! Two constants, each carrying a private `int id`. Verified against the jar by
//! `_porting/tests/consts.rs`: ordinals plus the `id` field for every constant.
//!
//! # `asEquipmentSlot`
//!
//! ```java
//! return this == MAIN_HAND ? EquipmentSlot.MAINHAND : EquipmentSlot.OFFHAND;
//! ```
//!
//! Session 19: unblocked by the `EquipmentSlot` port (tested in `_porting/tests/session19_d.rs`).
//!
//! # `BY_ID` IS PORTED, `STREAM_CODEC` IS PORT-BLOCKED
//!
//! Session 19: `BY_ID` is built on the ported `ByIdMap` (`OutOfBoundsStrategy.ZERO`, so an unknown
//! id yields `MAIN_HAND`; tested in `session19_consts.rs`). `STREAM_CODEC` still needs netty's
//! `ByteBuf` and `ByteBufCodecs`.

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

/// Java: `public InteractionHand#asEquipmentSlot()`.
///
/// `MAIN_HAND` -> `EquipmentSlot.MAINHAND`; anything else -> `EquipmentSlot.OFFHAND`.
pub fn as_equipment_slot(hand: InteractionHand) -> crate::net::minecraft::world::entity::EquipmentSlot::EquipmentSlot {
    use crate::net::minecraft::world::entity::EquipmentSlot::EquipmentSlot;
    if hand == InteractionHand::MainHand {
        EquipmentSlot::Mainhand
    } else {
        EquipmentSlot::Offhand
    }
}

impl std::fmt::Display for InteractionHand {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            InteractionHand::MainHand => "MAIN_HAND",
            InteractionHand::OffHand => "OFF_HAND",
        })
    }
}

static BY_ID: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<InteractionHand>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            InteractionHand::id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Zero,
        )
    });

/// Java: `private static final IntFunction<InteractionHand> BY_ID`, applied.
///
/// Session 19: unblocked by the `ByIdMap` port. `OutOfBoundsStrategy.ZERO` means an unknown id
/// (negative, or `>= 2`) yields `MAIN_HAND`.
pub fn by_id(id: i32) -> InteractionHand {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final StreamCodec<ByteBuf, InteractionHand> STREAM_CODEC`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
