//! Port of: net/minecraft/util/debug/DebugEntityBlockIntersection.java
//! Java class(es): net.minecraft.util.debug.DebugEntityBlockIntersection
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the three constants with `id` and `color`, the private `BY_ID`
//! (as [`by_id`]), `color()`. Constants and both fields are checked against the jar in
//! `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `STREAM_CODEC` on netty `ByteBuf`.
//!
//! The colours are ARGB with alpha `0x60`: `IN_BLOCK` `0x6000FF00` (green), `IN_FLUID`
//! `0x600000FF` (blue), `IN_AIR` `0x60333333` (grey). The source spells them in decimal; the
//! port keeps those literals.

use crate::net::minecraft::util::ByIdMap;

/// Port of `net.minecraft.util.debug.DebugEntityBlockIntersection`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DebugEntityBlockIntersection {
    /// Port of `DebugEntityBlockIntersection#IN_BLOCK` -- ordinal 0, id 0, colour `0x6000FF00`.
    InBlock = 0,
    /// Port of `DebugEntityBlockIntersection#IN_FLUID` -- ordinal 1, id 1, colour `0x600000FF`.
    InFluid = 1,
    /// Port of `DebugEntityBlockIntersection#IN_AIR` -- ordinal 2, id 2, colour `0x60333333`.
    InAir = 2,
}

/// Java: `DebugEntityBlockIntersection.values()` -- declaration order.
pub const VALUES: [DebugEntityBlockIntersection; 3] = [
    DebugEntityBlockIntersection::InBlock,
    DebugEntityBlockIntersection::InFluid,
    DebugEntityBlockIntersection::InAir,
];

impl DebugEntityBlockIntersection {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            DebugEntityBlockIntersection::InBlock => "IN_BLOCK",
            DebugEntityBlockIntersection::InFluid => "IN_FLUID",
            DebugEntityBlockIntersection::InAir => "IN_AIR",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `id` field (read by `BY_ID` / `STREAM_CODEC`).
    pub const fn id(&self) -> i32 {
        match self {
            DebugEntityBlockIntersection::InBlock => 0,
            DebugEntityBlockIntersection::InFluid => 1,
            DebugEntityBlockIntersection::InAir => 2,
        }
    }

    /// Port of `DebugEntityBlockIntersection#color()` -- packed ARGB.
    pub const fn color(&self) -> i32 {
        match self {
            DebugEntityBlockIntersection::InBlock => 1610678016,
            DebugEntityBlockIntersection::InFluid => 1610612991,
            DebugEntityBlockIntersection::InAir => 1613968179,
        }
    }
}

impl std::fmt::Display for DebugEntityBlockIntersection {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

static BY_ID: std::sync::LazyLock<ByIdMap::Continuous<DebugEntityBlockIntersection>> =
    std::sync::LazyLock::new(|| {
        ByIdMap::continuous(DebugEntityBlockIntersection::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
    });

/// Java: `private static final IntFunction<DebugEntityBlockIntersection> BY_ID`, applied (the
/// decoding half of `STREAM_CODEC`).
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..3` yields `IN_BLOCK`.
pub fn by_id(id: i32) -> DebugEntityBlockIntersection {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final StreamCodec<ByteBuf, DebugEntityBlockIntersection> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, i -> i.id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
