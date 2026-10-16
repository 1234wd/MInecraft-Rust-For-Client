//! Port of: net/minecraft/world/item/component/MapPostProcessing.java
//! Java class(es): net.minecraft.world.item.component.MapPostProcessing
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the two constants with `id`, `ID_MAP`, `id()`. Constants and the
//! `id` field are checked against the jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `STREAM_CODEC` on netty `ByteBuf`.

use crate::net::minecraft::util::ByIdMap;

/// Port of `net.minecraft.world.item.component.MapPostProcessing`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapPostProcessing {
    /// Port of `MapPostProcessing#LOCK` -- ordinal 0, id 0.
    Lock = 0,
    /// Port of `MapPostProcessing#SCALE` -- ordinal 1, id 1.
    Scale = 1,
}

/// Java: `MapPostProcessing.values()` -- declaration order.
pub const VALUES: [MapPostProcessing; 2] = [MapPostProcessing::Lock, MapPostProcessing::Scale];

impl MapPostProcessing {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            MapPostProcessing::Lock => "LOCK",
            MapPostProcessing::Scale => "SCALE",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `MapPostProcessing#id()`.
    pub const fn id(&self) -> i32 {
        match self {
            MapPostProcessing::Lock => 0,
            MapPostProcessing::Scale => 1,
        }
    }
}

impl std::fmt::Display for MapPostProcessing {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

static ID_MAP_TABLE: std::sync::LazyLock<ByIdMap::Continuous<MapPostProcessing>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(MapPostProcessing::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Java: `public static final IntFunction<MapPostProcessing> ID_MAP`, applied.
///
/// `OutOfBoundsStrategy.ZERO`: any id other than 0 or 1 yields `LOCK`.
pub fn ID_MAP(id: i32) -> MapPostProcessing {
    ID_MAP_TABLE.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final StreamCodec<ByteBuf, MapPostProcessing> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(ID_MAP, MapPostProcessing::id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
