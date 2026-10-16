//! Port of: net/minecraft/world/entity/ai/gossip/GossipType.java
//! Java class(es): net.minecraft.world.entity.ai.gossip.GossipType
//! Status: PARTIAL
//!
//! Session 19. The five constants with all five public fields (`id`, `weight`, `max`,
//! `decayPerDay`, `decayPerTransfer`) are ported and checked against the jar in
//! `_porting/tests/session19_c.rs`, as are the three `REPUTATION_CHANGE_*` constants (those
//! are not in the jar dump -- enum groups only carry the constants -- so they are pinned from
//! the Java source).
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec`).
//!
//! # THE NUMBERS ARE NOT SYMMETRIC
//!
//! `MAJOR_POSITIVE` never decays per day (`decayPerDay = 0`) but has the smallest `max` (20);
//! `MINOR_NEGATIVE` has the largest `max` (200). `MINOR_POSITIVE` and `TRADING` share
//! `weight = 1, max = 25` and differ only in decay. Easy to "tidy up" into a wrong table.

/// Port of `net.minecraft.world.entity.ai.gossip.GossipType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GossipType {
    /// Port of `GossipType#MAJOR_NEGATIVE` -- ordinal 0, `("major_negative", -5, 100, 10, 10)`.
    MajorNegative = 0,
    /// Port of `GossipType#MINOR_NEGATIVE` -- ordinal 1, `("minor_negative", -1, 200, 20, 20)`.
    MinorNegative = 1,
    /// Port of `GossipType#MINOR_POSITIVE` -- ordinal 2, `("minor_positive", 1, 25, 1, 5)`.
    MinorPositive = 2,
    /// Port of `GossipType#MAJOR_POSITIVE` -- ordinal 3, `("major_positive", 5, 20, 0, 20)`.
    MajorPositive = 3,
    /// Port of `GossipType#TRADING` -- ordinal 4, `("trading", 1, 25, 2, 20)`.
    Trading = 4,
}

/// Java: `GossipType.values()` -- declaration order.
pub const VALUES: [GossipType; 5] = [
    GossipType::MajorNegative,
    GossipType::MinorNegative,
    GossipType::MinorPositive,
    GossipType::MajorPositive,
    GossipType::Trading,
];

/// Java: `public static final int REPUTATION_CHANGE_PER_EVENT = 25`.
pub const REPUTATION_CHANGE_PER_EVENT: i32 = 25;
/// Java: `public static final int REPUTATION_CHANGE_PER_EVERLASTING_MEMORY = 20`.
pub const REPUTATION_CHANGE_PER_EVERLASTING_MEMORY: i32 = 20;
/// Java: `public static final int REPUTATION_CHANGE_PER_TRADE = 2`.
pub const REPUTATION_CHANGE_PER_TRADE: i32 = 2;

impl GossipType {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            GossipType::MajorNegative => "MAJOR_NEGATIVE",
            GossipType::MinorNegative => "MINOR_NEGATIVE",
            GossipType::MinorPositive => "MINOR_POSITIVE",
            GossipType::MajorPositive => "MAJOR_POSITIVE",
            GossipType::Trading => "TRADING",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `public final String id`.
    #[inline]
    pub const fn id(&self) -> &'static str {
        match self {
            GossipType::MajorNegative => "major_negative",
            GossipType::MinorNegative => "minor_negative",
            GossipType::MinorPositive => "minor_positive",
            GossipType::MajorPositive => "major_positive",
            GossipType::Trading => "trading",
        }
    }

    /// Java: `public final int weight`.
    #[inline]
    pub const fn weight(&self) -> i32 {
        match self {
            GossipType::MajorNegative => -5,
            GossipType::MinorNegative => -1,
            GossipType::MinorPositive => 1,
            GossipType::MajorPositive => 5,
            GossipType::Trading => 1,
        }
    }

    /// Java: `public final int max`.
    #[inline]
    pub const fn max(&self) -> i32 {
        match self {
            GossipType::MajorNegative => 100,
            GossipType::MinorNegative => 200,
            GossipType::MinorPositive => 25,
            GossipType::MajorPositive => 20,
            GossipType::Trading => 25,
        }
    }

    /// Java: `public final int decayPerDay`.
    #[inline]
    pub const fn decay_per_day(&self) -> i32 {
        match self {
            GossipType::MajorNegative => 10,
            GossipType::MinorNegative => 20,
            GossipType::MinorPositive => 1,
            GossipType::MajorPositive => 0,
            GossipType::Trading => 2,
        }
    }

    /// Java: `public final int decayPerTransfer`.
    #[inline]
    pub const fn decay_per_transfer(&self) -> i32 {
        match self {
            GossipType::MajorNegative => 10,
            GossipType::MinorNegative => 20,
            GossipType::MinorPositive => 5,
            GossipType::MajorPositive => 20,
            GossipType::Trading => 20,
        }
    }

    /// Port of `GossipType#getSerializedName()` -- returns `id`.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        self.id()
    }
}

impl std::fmt::Display for GossipType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for GossipType {
    fn get_serialized_name(&self) -> &str {
        GossipType::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<GossipType> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
