//! Port of: net/minecraft/world/level/dimension/DimensionDefaults.java
//! Java class(es): net.minecraft.world.level.dimension.DimensionDefaults
//! Status: VERIFIED
//!
//! Default heights, colours and cloud settings for the three vanilla dimensions. A pure
//! constant holder; all eighteen fields (including the two `float`s, compared as raw bits) are
//! checked against the jar's constant dump by `_porting/tests/session19_e.rs`.
//!
//! Note the asymmetry the names hide: the End's *logical* height is 256 but its *generation*
//! height is 128, while the Nether has 128 for both.

/// Java: `DimensionDefaults.CLOUD_THICKNESS` = `4`.
pub const CLOUD_THICKNESS: i32 = 4;
/// Java: `DimensionDefaults.BLOCK_LIGHT_TINT` = `-10100` (`0xFFFFD88C`).
pub const BLOCK_LIGHT_TINT: i32 = -10100;
/// Java: `DimensionDefaults.NIGHT_VISION_COLOR` = `-6710887` (`0xFF999999`).
pub const NIGHT_VISION_COLOR: i32 = -6710887;
/// Java: `DimensionDefaults.TURTLE_EGG_HATCH_CHANCE` = `0.002F`.
pub const TURTLE_EGG_HATCH_CHANCE: f32 = 0.002_f32;
/// Java: `DimensionDefaults.OVERWORLD_MIN_Y` = `-64`.
pub const OVERWORLD_MIN_Y: i32 = -64;
/// Java: `DimensionDefaults.OVERWORLD_LEVEL_HEIGHT` = `384`.
pub const OVERWORLD_LEVEL_HEIGHT: i32 = 384;
/// Java: `DimensionDefaults.OVERWORLD_GENERATION_HEIGHT` = `384`.
pub const OVERWORLD_GENERATION_HEIGHT: i32 = 384;
/// Java: `DimensionDefaults.OVERWORLD_LOGICAL_HEIGHT` = `384`.
pub const OVERWORLD_LOGICAL_HEIGHT: i32 = 384;
/// Java: `DimensionDefaults.OVERWORLD_CLOUD_HEIGHT` = `192.33F`.
pub const OVERWORLD_CLOUD_HEIGHT: f32 = 192.33_f32;
/// Java: `DimensionDefaults.NETHER_MIN_Y` = `0`.
pub const NETHER_MIN_Y: i32 = 0;
/// Java: `DimensionDefaults.NETHER_LEVEL_HEIGHT` = `256`.
pub const NETHER_LEVEL_HEIGHT: i32 = 256;
/// Java: `DimensionDefaults.NETHER_GENERATION_HEIGHT` = `128`.
pub const NETHER_GENERATION_HEIGHT: i32 = 128;
/// Java: `DimensionDefaults.NETHER_LOGICAL_HEIGHT` = `128`.
pub const NETHER_LOGICAL_HEIGHT: i32 = 128;
/// Java: `DimensionDefaults.END_MIN_Y` = `0`.
pub const END_MIN_Y: i32 = 0;
/// Java: `DimensionDefaults.END_LEVEL_HEIGHT` = `256`.
pub const END_LEVEL_HEIGHT: i32 = 256;
/// Java: `DimensionDefaults.END_GENERATION_HEIGHT` = `128`.
pub const END_GENERATION_HEIGHT: i32 = 128;
/// Java: `DimensionDefaults.END_LOGICAL_HEIGHT` = `256`.
pub const END_LOGICAL_HEIGHT: i32 = 256;
/// Java: `DimensionDefaults.END_ISLAND_BASE_Y` = `63`.
pub const END_ISLAND_BASE_Y: i32 = 63;
