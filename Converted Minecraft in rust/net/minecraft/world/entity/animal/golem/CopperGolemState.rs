//! Port of: net/minecraft/world/entity/animal/golem/CopperGolemState.java
//! Java class(es): net.minecraft.world.entity.animal.golem.CopperGolemState
//! Status: PARTIAL
//!
//! Session 19. The five constants with their `name` and `id` fields, `getSerializedName()`,
//! the private `id()` and the private `BY_ID` lookup (via the ported `ByIdMap`, `ZERO`) are
//! ported. Ordinals and both fields are checked against the jar in
//! `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec`) and `STREAM_CODEC` (netty `ByteBuf`).
//!
//! # VISIBILITY
//!
//! `id()` and `BY_ID` are private in Java; their only consumer is `STREAM_CODEC`. They are `pub`
//! here so that codec's eventual port (and the tests) can reach them. `BY_ID` uses
//! `OutOfBoundsStrategy.ZERO`: any unknown id from the network decodes to `IDLE`.

/// Port of `net.minecraft.world.entity.animal.golem.CopperGolemState`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CopperGolemState {
    /// Port of `CopperGolemState#IDLE` -- ordinal 0, name `"idle"`, id `0`.
    Idle = 0,
    /// Port of `CopperGolemState#GETTING_ITEM` -- ordinal 1, name `"getting_item"`, id `1`.
    GettingItem = 1,
    /// Port of `CopperGolemState#GETTING_NO_ITEM` -- ordinal 2, name `"getting_no_item"`, id `2`.
    GettingNoItem = 2,
    /// Port of `CopperGolemState#DROPPING_ITEM` -- ordinal 3, name `"dropping_item"`, id `3`.
    DroppingItem = 3,
    /// Port of `CopperGolemState#DROPPING_NO_ITEM` -- ordinal 4, name `"dropping_no_item"`,
    /// id `4`.
    DroppingNoItem = 4,
}

/// Java: `CopperGolemState.values()` -- declaration order.
pub const VALUES: [CopperGolemState; 5] = [
    CopperGolemState::Idle,
    CopperGolemState::GettingItem,
    CopperGolemState::GettingNoItem,
    CopperGolemState::DroppingItem,
    CopperGolemState::DroppingNoItem,
];

impl CopperGolemState {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            CopperGolemState::Idle => "IDLE",
            CopperGolemState::GettingItem => "GETTING_ITEM",
            CopperGolemState::GettingNoItem => "GETTING_NO_ITEM",
            CopperGolemState::DroppingItem => "DROPPING_ITEM",
            CopperGolemState::DroppingNoItem => "DROPPING_NO_ITEM",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `CopperGolemState#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            CopperGolemState::Idle => "idle",
            CopperGolemState::GettingItem => "getting_item",
            CopperGolemState::GettingNoItem => "getting_no_item",
            CopperGolemState::DroppingItem => "dropping_item",
            CopperGolemState::DroppingNoItem => "dropping_no_item",
        }
    }

    /// Port of the private `CopperGolemState#id()` -- the stored `id` field.
    #[inline]
    pub const fn id(&self) -> i32 {
        match self {
            CopperGolemState::Idle => 0,
            CopperGolemState::GettingItem => 1,
            CopperGolemState::GettingNoItem => 2,
            CopperGolemState::DroppingItem => 3,
            CopperGolemState::DroppingNoItem => 4,
        }
    }
}

impl std::fmt::Display for CopperGolemState {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for CopperGolemState {
    fn get_serialized_name(&self) -> &str {
        CopperGolemState::get_serialized_name(self)
    }
}

static BY_ID_MAP: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<CopperGolemState>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            CopperGolemState::id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Zero,
        )
    });

/// Java: `private static final IntFunction<CopperGolemState> BY_ID`, applied.
///
/// `OutOfBoundsStrategy.ZERO`: an id outside `0..5` yields `IDLE`.
pub fn BY_ID(id: i32) -> CopperGolemState {
    BY_ID_MAP.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<CopperGolemState> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, CopperGolemState> STREAM_CODEC =
/// ByteBufCodecs.idMapper(BY_ID, CopperGolemState::id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
