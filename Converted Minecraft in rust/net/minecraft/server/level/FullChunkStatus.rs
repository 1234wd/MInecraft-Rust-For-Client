//! Port of: net/minecraft/server/level/FullChunkStatus.java
//! Java class(es): net.minecraft.server.level.FullChunkStatus
//! Status: VERIFIED
//!
//! Session 19. Four constants and `isOrAfter`. Names and ordinals are checked against the jar in
//! `_porting/tests/session19_b.rs`.
//!
//! # `isOrAfter` IS AN ORDINAL COMPARISON
//!
//! `this.ordinal() >= step.ordinal()`: the declaration order *is* the level order
//! (`INACCESSIBLE < FULL < BLOCK_TICKING < ENTITY_TICKING`), and `isOrAfter(self)` is `true`.

/// Port of `net.minecraft.server.level.FullChunkStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FullChunkStatus {
    /// Port of `FullChunkStatus#INACCESSIBLE` -- ordinal 0.
    Inaccessible = 0,
    /// Port of `FullChunkStatus#FULL` -- ordinal 1.
    Full = 1,
    /// Port of `FullChunkStatus#BLOCK_TICKING` -- ordinal 2.
    BlockTicking = 2,
    /// Port of `FullChunkStatus#ENTITY_TICKING` -- ordinal 3.
    EntityTicking = 3,
}

/// Java: `FullChunkStatus.values()` -- declaration order.
pub const VALUES: [FullChunkStatus; 4] = [
    FullChunkStatus::Inaccessible,
    FullChunkStatus::Full,
    FullChunkStatus::BlockTicking,
    FullChunkStatus::EntityTicking,
];

impl FullChunkStatus {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            FullChunkStatus::Inaccessible => "INACCESSIBLE",
            FullChunkStatus::Full => "FULL",
            FullChunkStatus::BlockTicking => "BLOCK_TICKING",
            FullChunkStatus::EntityTicking => "ENTITY_TICKING",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `FullChunkStatus#isOrAfter(FullChunkStatus)`: `this.ordinal() >= step.ordinal()`.
    #[inline]
    pub const fn is_or_after(&self, step: FullChunkStatus) -> bool {
        self.ordinal() >= step.ordinal()
    }
}

impl std::fmt::Display for FullChunkStatus {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
