//! Port of: net/minecraft/world/food/FoodConstants.java
//! Java class(es): net.minecraft.world.food.FoodConstants
//! Status: VERIFIED
//!
//! Hunger/saturation/exhaustion tuning plus the one helper `saturationByModifier`. All 25
//! constants are checked against the jar's constant dump (floats as raw bits) by
//! `_porting/tests/session19_e.rs`.
//!
//! `saturationByModifier(int, float)` is evaluated in `float`: Java widens `nutrition` to
//! `float` (round-to-nearest, exact for every realistic nutrition value), multiplies left to
//! right -- `(nutrition * modifier) * 2.0F` -- and never touches `double`. Reordering to
//! `nutrition * (modifier * 2.0F)` is exact too (multiplying by 2 only changes the exponent),
//! but the port keeps Java's order anyway.

/// Java: `FoodConstants.MAX_FOOD` = `20`.
pub const MAX_FOOD: i32 = 20;
/// Java: `FoodConstants.MAX_SATURATION` = `20.0F`.
pub const MAX_SATURATION: f32 = 20.0;
/// Java: `FoodConstants.START_SATURATION` = `5.0F`.
pub const START_SATURATION: f32 = 5.0;
/// Java: `FoodConstants.SATURATION_FLOOR` = `2.5F`.
pub const SATURATION_FLOOR: f32 = 2.5;
/// Java: `FoodConstants.EXHAUSTION_DROP` = `4.0F`.
pub const EXHAUSTION_DROP: f32 = 4.0;
/// Java: `FoodConstants.HEALTH_TICK_COUNT` = `80`.
pub const HEALTH_TICK_COUNT: i32 = 80;
/// Java: `FoodConstants.HEALTH_TICK_COUNT_SATURATED` = `10`.
pub const HEALTH_TICK_COUNT_SATURATED: i32 = 10;
/// Java: `FoodConstants.HEAL_LEVEL` = `18`.
pub const HEAL_LEVEL: i32 = 18;
/// Java: `FoodConstants.SPRINT_LEVEL` = `6`.
pub const SPRINT_LEVEL: i32 = 6;
/// Java: `FoodConstants.STARVE_LEVEL` = `0`.
pub const STARVE_LEVEL: i32 = 0;
/// Java: `FoodConstants.FOOD_SATURATION_POOR` = `0.1F`.
pub const FOOD_SATURATION_POOR: f32 = 0.1;
/// Java: `FoodConstants.FOOD_SATURATION_LOW` = `0.3F`.
pub const FOOD_SATURATION_LOW: f32 = 0.3;
/// Java: `FoodConstants.FOOD_SATURATION_NORMAL` = `0.6F`.
pub const FOOD_SATURATION_NORMAL: f32 = 0.6;
/// Java: `FoodConstants.FOOD_SATURATION_GOOD` = `0.8F`.
pub const FOOD_SATURATION_GOOD: f32 = 0.8;
/// Java: `FoodConstants.FOOD_SATURATION_MAX` = `1.0F`.
pub const FOOD_SATURATION_MAX: f32 = 1.0;
/// Java: `FoodConstants.FOOD_SATURATION_SUPERNATURAL` = `1.2F`.
pub const FOOD_SATURATION_SUPERNATURAL: f32 = 1.2;
/// Java: `FoodConstants.EXHAUSTION_HEAL` = `6.0F`.
pub const EXHAUSTION_HEAL: f32 = 6.0;
/// Java: `FoodConstants.EXHAUSTION_JUMP` = `0.05F`.
pub const EXHAUSTION_JUMP: f32 = 0.05;
/// Java: `FoodConstants.EXHAUSTION_SPRINT_JUMP` = `0.2F`.
pub const EXHAUSTION_SPRINT_JUMP: f32 = 0.2;
/// Java: `FoodConstants.EXHAUSTION_MINE` = `0.005F`.
pub const EXHAUSTION_MINE: f32 = 0.005;
/// Java: `FoodConstants.EXHAUSTION_ATTACK` = `0.1F`.
pub const EXHAUSTION_ATTACK: f32 = 0.1;
/// Java: `FoodConstants.EXHAUSTION_WALK` = `0.0F`.
pub const EXHAUSTION_WALK: f32 = 0.0;
/// Java: `FoodConstants.EXHAUSTION_CROUCH` = `0.0F`.
pub const EXHAUSTION_CROUCH: f32 = 0.0;
/// Java: `FoodConstants.EXHAUSTION_SPRINT` = `0.1F`.
pub const EXHAUSTION_SPRINT: f32 = 0.1;
/// Java: `FoodConstants.EXHAUSTION_SWIM` = `0.01F`.
pub const EXHAUSTION_SWIM: f32 = 0.01;

/// Port of `FoodConstants#saturationByModifier(int, float)`: `nutrition * modifier * 2.0F`,
/// all in `float`, left to right.
#[inline]
pub fn saturation_by_modifier(nutrition: i32, modifier: f32) -> f32 {
    nutrition as f32 * modifier * 2.0_f32
}
