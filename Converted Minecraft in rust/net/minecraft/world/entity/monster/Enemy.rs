//! Port of: net/minecraft/world/entity/monster/Enemy.java
//! Java class(es): net.minecraft.world.entity.monster.Enemy
//! Status: VERIFIED
//!
//! `Enemy` is a marker interface (hostile mobs implement it; `instanceof Enemy` drives
//! targeting, peaceful despawning and mob-cap category checks) that also carries the six
//! experience-reward constants. Interface fields are implicitly `public static final`, so they
//! are module-level constants here; all six are checked against the jar's constant dump by
//! `_porting/tests/session19_e.rs`. The trait itself has no members.

/// Java: `Enemy.XP_REWARD_NONE` = `0`.
pub const XP_REWARD_NONE: i32 = 0;
/// Java: `Enemy.XP_REWARD_SMALL` = `3`.
pub const XP_REWARD_SMALL: i32 = 3;
/// Java: `Enemy.XP_REWARD_MEDIUM` = `5`.
pub const XP_REWARD_MEDIUM: i32 = 5;
/// Java: `Enemy.XP_REWARD_LARGE` = `10`.
pub const XP_REWARD_LARGE: i32 = 10;
/// Java: `Enemy.XP_REWARD_HUGE` = `20`.
pub const XP_REWARD_HUGE: i32 = 20;
/// Java: `Enemy.XP_REWARD_BOSS` = `50`.
pub const XP_REWARD_BOSS: i32 = 50;

/// Port of the marker interface `net.minecraft.world.entity.monster.Enemy`.
pub trait Enemy {}
