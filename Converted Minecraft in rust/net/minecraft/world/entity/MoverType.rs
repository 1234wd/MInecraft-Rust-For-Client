//! Port of: net/minecraft/world/entity/MoverType.java
//! Java class(es): net.minecraft.world.entity.MoverType
//! Status: VERIFIED
//!
//! Session 19. A plain five-constant enum naming what moved an entity (`Entity#move`).
//! Names and ordinals are checked against the jar in `_porting/tests/session19_b.rs`.

/// Port of `net.minecraft.world.entity.MoverType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MoverType {
    /// Port of `MoverType#SELF` -- ordinal 0.
    SelfMove = 0,
    /// Port of `MoverType#PLAYER` -- ordinal 1.
    Player = 1,
    /// Port of `MoverType#PISTON` -- ordinal 2.
    Piston = 2,
    /// Port of `MoverType#SHULKER_BOX` -- ordinal 3.
    ShulkerBox = 3,
    /// Port of `MoverType#SHULKER` -- ordinal 4.
    Shulker = 4,
}

/// Java: `MoverType.values()` -- declaration order.
pub const VALUES: [MoverType; 5] = [
    MoverType::SelfMove,
    MoverType::Player,
    MoverType::Piston,
    MoverType::ShulkerBox,
    MoverType::Shulker,
];

impl MoverType {
    /// Java `Enum#name()`. Note `SELF` is the variant `SelfMove` because `Self` is a Rust keyword.
    pub const fn name(&self) -> &'static str {
        match self {
            MoverType::SelfMove => "SELF",
            MoverType::Player => "PLAYER",
            MoverType::Piston => "PISTON",
            MoverType::ShulkerBox => "SHULKER_BOX",
            MoverType::Shulker => "SHULKER",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for MoverType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
