//! Port of: net/minecraft/world/level/gamerules/GameRuleType.java
//! Java class(es): net.minecraft.world.level.gamerules.GameRuleType
//! Status: VERIFIED
//!
//! Session 19. Two constants carrying a private `String name`, exposed only through
//! `getSerializedName()`. The class has no codec and no other members, so nothing is
//! PORT-BLOCKED. Ordinals and the `name` field are checked against the jar in
//! `_porting/tests/session19_c.rs`.
//!
//! The serialized names are `"integer"` / `"boolean"`, not the constant names `INT` / `BOOL`.

/// Port of `net.minecraft.world.level.gamerules.GameRuleType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameRuleType {
    /// Port of `GameRuleType#INT` -- ordinal 0, name `"integer"`.
    Int = 0,
    /// Port of `GameRuleType#BOOL` -- ordinal 1, name `"boolean"`.
    Bool = 1,
}

/// Java: `GameRuleType.values()` -- declaration order.
pub const VALUES: [GameRuleType; 2] = [GameRuleType::Int, GameRuleType::Bool];

impl GameRuleType {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            GameRuleType::Int => "INT",
            GameRuleType::Bool => "BOOL",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `GameRuleType#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            GameRuleType::Int => "integer",
            GameRuleType::Bool => "boolean",
        }
    }
}

impl std::fmt::Display for GameRuleType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for GameRuleType {
    fn get_serialized_name(&self) -> &str {
        GameRuleType::get_serialized_name(self)
    }
}
