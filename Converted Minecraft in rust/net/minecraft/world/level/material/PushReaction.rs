//! Port of: net/minecraft/world/level/material/PushReaction.java
//! Java class(es): net.minecraft.world.level.material.PushReaction
//! Status: VERIFIED
//!
//! Session 19. A plain five-constant enum (how a block or entity reacts to a piston). Names and
//! ordinals are checked against the jar in `_porting/tests/session19_b.rs`.

/// Port of `net.minecraft.world.level.material.PushReaction`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PushReaction {
    /// Port of `PushReaction#NORMAL` -- ordinal 0.
    Normal = 0,
    /// Port of `PushReaction#DESTROY` -- ordinal 1.
    Destroy = 1,
    /// Port of `PushReaction#BLOCK` -- ordinal 2.
    Block = 2,
    /// Port of `PushReaction#IGNORE` -- ordinal 3.
    Ignore = 3,
    /// Port of `PushReaction#PUSH_ONLY` -- ordinal 4.
    PushOnly = 4,
}

/// Java: `PushReaction.values()` -- declaration order.
pub const VALUES: [PushReaction; 5] = [
    PushReaction::Normal,
    PushReaction::Destroy,
    PushReaction::Block,
    PushReaction::Ignore,
    PushReaction::PushOnly,
];

impl PushReaction {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PushReaction::Normal => "NORMAL",
            PushReaction::Destroy => "DESTROY",
            PushReaction::Block => "BLOCK",
            PushReaction::Ignore => "IGNORE",
            PushReaction::PushOnly => "PUSH_ONLY",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for PushReaction {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
