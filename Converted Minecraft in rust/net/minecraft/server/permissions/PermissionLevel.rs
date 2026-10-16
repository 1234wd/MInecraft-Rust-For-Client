//! Port of: net/minecraft/server/permissions/PermissionLevel.java
//! Java class(es): net.minecraft.server.permissions.PermissionLevel
//! Status: PARTIAL
//!
//! Session 19. Ported: the five constants with `name` and `id`, `isEqualOrHigherThan`, `byId`
//! (via the ported `ByIdMap`, `CLAMP`), `id()` and `getSerializedName()`. Ordinals and both
//! fields are checked against the jar; the lookup and comparison are tested in
//! `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED on DataFixerUpper `Codec`: `CODEC` and `INT_CODEC`.
//!
//! # `byId` CLAMPS -- AN OUT-OF-RANGE OP LEVEL IS NOT "NO PERMISSION"
//!
//! `OutOfBoundsStrategy.CLAMP`: `byId(5)` and `byId(Integer.MAX_VALUE)` are `OWNERS`, and
//! `byId(-1)` is `ALL`. A config with `op-permission-level=7` therefore grants owner level, it
//! does not wrap or fall back to zero. `INT_CODEC` decodes through the same `BY_ID`.

/// Port of `net.minecraft.server.permissions.PermissionLevel`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PermissionLevel {
    /// Port of `PermissionLevel#ALL` -- ordinal 0, name `"all"`, id `0`.
    All = 0,
    /// Port of `PermissionLevel#MODERATORS` -- ordinal 1, name `"moderators"`, id `1`.
    Moderators = 1,
    /// Port of `PermissionLevel#GAMEMASTERS` -- ordinal 2, name `"gamemasters"`, id `2`.
    Gamemasters = 2,
    /// Port of `PermissionLevel#ADMINS` -- ordinal 3, name `"admins"`, id `3`.
    Admins = 3,
    /// Port of `PermissionLevel#OWNERS` -- ordinal 4, name `"owners"`, id `4`.
    Owners = 4,
}

/// Java: `PermissionLevel.values()` -- declaration order.
pub const VALUES: [PermissionLevel; 5] = [
    PermissionLevel::All,
    PermissionLevel::Moderators,
    PermissionLevel::Gamemasters,
    PermissionLevel::Admins,
    PermissionLevel::Owners,
];

impl PermissionLevel {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            PermissionLevel::All => "ALL",
            PermissionLevel::Moderators => "MODERATORS",
            PermissionLevel::Gamemasters => "GAMEMASTERS",
            PermissionLevel::Admins => "ADMINS",
            PermissionLevel::Owners => "OWNERS",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `PermissionLevel#id()` -- the stored `id` field.
    #[inline]
    pub const fn id(&self) -> i32 {
        match self {
            PermissionLevel::All => 0,
            PermissionLevel::Moderators => 1,
            PermissionLevel::Gamemasters => 2,
            PermissionLevel::Admins => 3,
            PermissionLevel::Owners => 4,
        }
    }

    /// Port of `PermissionLevel#isEqualOrHigherThan(PermissionLevel)`: `this.id >= other.id`.
    #[inline]
    pub const fn is_equal_or_higher_than(&self, other: PermissionLevel) -> bool {
        self.id() >= other.id()
    }

    /// Port of `PermissionLevel#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            PermissionLevel::All => "all",
            PermissionLevel::Moderators => "moderators",
            PermissionLevel::Gamemasters => "gamemasters",
            PermissionLevel::Admins => "admins",
            PermissionLevel::Owners => "owners",
        }
    }
}

impl std::fmt::Display for PermissionLevel {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for PermissionLevel {
    fn get_serialized_name(&self) -> &str {
        PermissionLevel::get_serialized_name(self)
    }
}

/// Java: `private static final IntFunction<PermissionLevel> BY_ID =
/// ByIdMap.continuous(level -> level.id, values(), CLAMP)`.
static BY_ID: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<PermissionLevel>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            PermissionLevel::id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Clamp,
        )
    });

/// Port of `PermissionLevel#byId(int)`: `CLAMP` to `ALL..=OWNERS`.
pub fn by_id(level: i32) -> PermissionLevel {
    BY_ID.apply(level)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<PermissionLevel> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final Codec<PermissionLevel> INT_CODEC =
/// Codec.INT.xmap(BY_ID::apply, level -> level.id)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`. The mapping it would use is [`by_id`] /
/// [`PermissionLevel::id`].
pub fn INT_CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
