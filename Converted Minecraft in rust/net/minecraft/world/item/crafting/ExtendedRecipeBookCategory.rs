//! Port of: net/minecraft/world/item/crafting/ExtendedRecipeBookCategory.java
//! Java class(es): net.minecraft.world.item.crafting.ExtendedRecipeBookCategory
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A marker interface implemented by the recipe-book categories (`Crafting`, `Furnace`,
//! `Campfire`, ...). `RecipeBookSettings` tests it with `instanceof` and casts to the concrete
//! enum; no method is declared and none is called.

/// Port of `net.minecraft.world.item.crafting.ExtendedRecipeBookCategory`.
///
/// Empty on purpose -- see the module docs.
pub trait ExtendedRecipeBookCategory {}