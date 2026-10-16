//! Port of: net/minecraft/world/entity/animal/equine/Markings.java
//! Java class(es): net.minecraft.world.entity.animal.equine.Markings
//! Status: VERIFIED
//!
//! Session 19. Five constants carrying an `int id`, `getId()` and `byId(int)` (via the ported
//! `ByIdMap`). No codec, nothing PORT-BLOCKED. Ordinals and the `id` field are checked against
//! the jar, and the `WRAP` lookup is tested, in `_porting/tests/session19_c.rs`.
//!
//! `byId` uses `OutOfBoundsStrategy.WRAP` (`Math.floorMod`): `5` is `NONE`, `-1` is
//! `BLACK_DOTS`. A horse's markings are decoded from its packed variant int, so out-of-range ids
//! wrap rather than falling back to `NONE`.

/// Port of `net.minecraft.world.entity.animal.equine.Markings`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Markings {
    /// Port of `Markings#NONE` -- ordinal 0, id `0`.
    None = 0,
    /// Port of `Markings#WHITE` -- ordinal 1, id `1`.
    White = 1,
    /// Port of `Markings#WHITE_FIELD` -- ordinal 2, id `2`.
    WhiteField = 2,
    /// Port of `Markings#WHITE_DOTS` -- ordinal 3, id `3`.
    WhiteDots = 3,
    /// Port of `Markings#BLACK_DOTS` -- ordinal 4, id `4`.
    BlackDots = 4,
}

/// Java: `Markings.values()` -- declaration order.
pub const VALUES: [Markings; 5] = [
    Markings::None,
    Markings::White,
    Markings::WhiteField,
    Markings::WhiteDots,
    Markings::BlackDots,
];

impl Markings {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Markings::None => "NONE",
            Markings::White => "WHITE",
            Markings::WhiteField => "WHITE_FIELD",
            Markings::WhiteDots => "WHITE_DOTS",
            Markings::BlackDots => "BLACK_DOTS",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Markings#getId()` -- the stored `id` field (equal to the ordinal today, kept
    /// separate as in Java).
    #[inline]
    pub const fn get_id(&self) -> i32 {
        match self {
            Markings::None => 0,
            Markings::White => 1,
            Markings::WhiteField => 2,
            Markings::WhiteDots => 3,
            Markings::BlackDots => 4,
        }
    }
}

impl std::fmt::Display for Markings {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Java: `private static final IntFunction<Markings> BY_ID =
/// ByIdMap.continuous(Markings::getId, values(), WRAP)`.
static BY_ID: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<Markings>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            Markings::get_id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Wrap,
        )
    });

/// Port of `Markings#byId(int)`: `WRAP`, i.e. `values[floorMod(id, 5)]`.
pub fn by_id(id: i32) -> Markings {
    BY_ID.apply(id)
}
