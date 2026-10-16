//! Port of: net/minecraft/world/level/entity/Visibility.java
//! Java class(es): net.minecraft.world.level.entity.Visibility
//! Status: VERIFIED
//!
//! Session 19. Three constants with `accessible`/`ticking` flags, plus
//! `fromFullChunkStatus`. Names, ordinals and both flags are checked against the jar in
//! `_porting/tests/session19_b.rs`.
//!
//! # `BLOCK_TICKING` MAPS TO `TRACKED`, NOT `TICKING`
//!
//! `fromFullChunkStatus` tests `ENTITY_TICKING` first, then `FULL`. A block-ticking chunk is
//! "or after" `FULL` but not `ENTITY_TICKING`, so its entities are tracked but not ticked.

use crate::net::minecraft::server::level::FullChunkStatus::FullChunkStatus;

/// Port of `net.minecraft.world.level.entity.Visibility`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Visibility {
    /// Port of `Visibility#HIDDEN` -- ordinal 0, accessible `false`, ticking `false`.
    Hidden = 0,
    /// Port of `Visibility#TRACKED` -- ordinal 1, accessible `true`, ticking `false`.
    Tracked = 1,
    /// Port of `Visibility#TICKING` -- ordinal 2, accessible `true`, ticking `true`.
    Ticking = 2,
}

/// Java: `Visibility.values()` -- declaration order.
pub const VALUES: [Visibility; 3] = [Visibility::Hidden, Visibility::Tracked, Visibility::Ticking];

impl Visibility {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Visibility::Hidden => "HIDDEN",
            Visibility::Tracked => "TRACKED",
            Visibility::Ticking => "TICKING",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Visibility#isTicking()` -- the `ticking` field.
    #[inline]
    pub const fn is_ticking(&self) -> bool {
        match self {
            Visibility::Hidden => false,
            Visibility::Tracked => false,
            Visibility::Ticking => true,
        }
    }

    /// Port of `Visibility#isAccessible()` -- the `accessible` field.
    #[inline]
    pub const fn is_accessible(&self) -> bool {
        match self {
            Visibility::Hidden => false,
            Visibility::Tracked => true,
            Visibility::Ticking => true,
        }
    }
}

/// Port of `Visibility#fromFullChunkStatus(FullChunkStatus)`.
pub const fn from_full_chunk_status(status: FullChunkStatus) -> Visibility {
    if status.is_or_after(FullChunkStatus::EntityTicking) {
        Visibility::Ticking
    } else if status.is_or_after(FullChunkStatus::Full) {
        Visibility::Tracked
    } else {
        Visibility::Hidden
    }
}

impl std::fmt::Display for Visibility {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
