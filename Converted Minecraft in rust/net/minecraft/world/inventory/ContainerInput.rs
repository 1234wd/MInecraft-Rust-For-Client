//! Port of: net/minecraft/world/inventory/ContainerInput.java
//! Java class(es): net.minecraft.world.inventory.ContainerInput
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the seven constants with `id`, the private `BY_ID` (as
//! [`by_id`]), `id()`. Constants and the `id` field are checked against the jar in
//! `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `STREAM_CODEC` on netty `ByteBuf`.
//!
//! `OutOfBoundsStrategy.ZERO`: a container-click packet carrying an unknown input id is decoded
//! as `PICKUP`, not rejected.

use crate::net::minecraft::util::ByIdMap;

/// Port of `net.minecraft.world.inventory.ContainerInput`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ContainerInput {
    /// Port of `ContainerInput#PICKUP` -- ordinal 0, id 0.
    Pickup = 0,
    /// Port of `ContainerInput#QUICK_MOVE` -- ordinal 1, id 1.
    QuickMove = 1,
    /// Port of `ContainerInput#SWAP` -- ordinal 2, id 2.
    Swap = 2,
    /// Port of `ContainerInput#CLONE` -- ordinal 3, id 3.
    Clone = 3,
    /// Port of `ContainerInput#THROW` -- ordinal 4, id 4.
    Throw = 4,
    /// Port of `ContainerInput#QUICK_CRAFT` -- ordinal 5, id 5.
    QuickCraft = 5,
    /// Port of `ContainerInput#PICKUP_ALL` -- ordinal 6, id 6.
    PickupAll = 6,
}

/// Java: `ContainerInput.values()` -- declaration order.
pub const VALUES: [ContainerInput; 7] = [
    ContainerInput::Pickup,
    ContainerInput::QuickMove,
    ContainerInput::Swap,
    ContainerInput::Clone,
    ContainerInput::Throw,
    ContainerInput::QuickCraft,
    ContainerInput::PickupAll,
];

impl ContainerInput {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ContainerInput::Pickup => "PICKUP",
            ContainerInput::QuickMove => "QUICK_MOVE",
            ContainerInput::Swap => "SWAP",
            ContainerInput::Clone => "CLONE",
            ContainerInput::Throw => "THROW",
            ContainerInput::QuickCraft => "QUICK_CRAFT",
            ContainerInput::PickupAll => "PICKUP_ALL",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ContainerInput#id()`.
    pub const fn id(&self) -> i32 {
        match self {
            ContainerInput::Pickup => 0,
            ContainerInput::QuickMove => 1,
            ContainerInput::Swap => 2,
            ContainerInput::Clone => 3,
            ContainerInput::Throw => 4,
            ContainerInput::QuickCraft => 5,
            ContainerInput::PickupAll => 6,
        }
    }
}

impl std::fmt::Display for ContainerInput {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

static BY_ID: std::sync::LazyLock<ByIdMap::Continuous<ContainerInput>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(ContainerInput::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Java: `private static final IntFunction<ContainerInput> BY_ID`, applied (the decoding half of
/// `STREAM_CODEC`).
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..7` yields `PICKUP`.
pub fn by_id(id: i32) -> ContainerInput {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final StreamCodec<ByteBuf, ContainerInput> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, ContainerInput::id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
