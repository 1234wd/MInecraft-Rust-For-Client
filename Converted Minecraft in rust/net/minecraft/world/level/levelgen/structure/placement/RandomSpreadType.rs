//! Port of: net/minecraft/world/level/levelgen/structure/placement/RandomSpreadType.java
//! Java class(es): net.minecraft.world.level.levelgen.structure.placement.RandomSpreadType
//! Status: PARTIAL
//!
//! Session 19. Both constants, `getSerializedName()` and `evaluate(RandomSource, int)` are
//! ported; ordinals and the `id` field are checked against the jar, and `evaluate` is tested
//! against seeded `LegacyRandomSource` / `WorldgenRandom` in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec`).
//!
//! # `evaluate` IS WORLD-GENERATION RNG CONSUMPTION
//!
//! `RandomSpreadStructurePlacement` calls `evaluate` twice per region (x then z) on one
//! `WorldgenRandom`. `LINEAR` draws one `nextInt(limit)`, `TRIANGULAR` draws **two**, so the
//! number of draws is part of the contract: the z offset depends on how many draws x took.
//!
//! `TRIANGULAR` is `(nextInt(limit) + nextInt(limit)) / 2`: left draw first, `int` addition
//! (wraps for `limit > 2^30`), and `/` truncating toward zero. Rust's `i32 /` truncates the same
//! way; the addition uses `wrapping_add` so an overflow wraps as in Java instead of panicking.
//! `limit <= 0` throws in `nextInt`, which the ported `RandomSource` reproduces as a panic.

use crate::net::minecraft::util::RandomSource::RandomSource;

/// Port of `net.minecraft.world.level.levelgen.structure.placement.RandomSpreadType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RandomSpreadType {
    /// Port of `RandomSpreadType#LINEAR` -- ordinal 0, id `"linear"`.
    Linear = 0,
    /// Port of `RandomSpreadType#TRIANGULAR` -- ordinal 1, id `"triangular"`.
    Triangular = 1,
}

/// Java: `RandomSpreadType.values()` -- declaration order.
pub const VALUES: [RandomSpreadType; 2] = [RandomSpreadType::Linear, RandomSpreadType::Triangular];

impl RandomSpreadType {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            RandomSpreadType::Linear => "LINEAR",
            RandomSpreadType::Triangular => "TRIANGULAR",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `RandomSpreadType#getSerializedName()` -- the private `id` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            RandomSpreadType::Linear => "linear",
            RandomSpreadType::Triangular => "triangular",
        }
    }

    /// Port of `RandomSpreadType#evaluate(RandomSource, int)`.
    ///
    /// ```java
    /// case LINEAR -> random.nextInt(limit);
    /// case TRIANGULAR -> (random.nextInt(limit) + random.nextInt(limit)) / 2;
    /// ```
    pub fn evaluate(&self, random: &mut dyn RandomSource, limit: i32) -> i32 {
        match self {
            RandomSpreadType::Linear => random.next_int_bounded(limit),
            RandomSpreadType::Triangular => {
                // Java evaluates the left operand first.
                let first = random.next_int_bounded(limit);
                let second = random.next_int_bounded(limit);
                first.wrapping_add(second) / 2
            }
        }
    }
}

impl std::fmt::Display for RandomSpreadType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for RandomSpreadType {
    fn get_serialized_name(&self) -> &str {
        RandomSpreadType::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<RandomSpreadType> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
