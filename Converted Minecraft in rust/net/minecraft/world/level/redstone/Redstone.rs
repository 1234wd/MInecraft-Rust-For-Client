//! Port of: net/minecraft/world/level/redstone/Redstone.java
//! Java class(es): net.minecraft.world.level.redstone.Redstone
//! Status: VERIFIED
//!
//! Redstone signal-strength bounds. A pure constant holder; all five fields are checked against
//! the jar's constant dump by `_porting/tests/session19_e.rs`. A signal fits in `BITS` = 4 bits,
//! so `MASK` and `SIGNAL_MAX` are both 15.

/// Java: `Redstone.BITS` = `4`.
pub const BITS: i32 = 4;
/// Java: `Redstone.MASK` = `15`.
pub const MASK: i32 = 15;
/// Java: `Redstone.SIGNAL_MIN` = `0`.
pub const SIGNAL_MIN: i32 = 0;
/// Java: `Redstone.SIGNAL_MAX` = `15`.
pub const SIGNAL_MAX: i32 = 15;
/// Java: `Redstone.SIGNAL_NONE` = `0`.
pub const SIGNAL_NONE: i32 = 0;
