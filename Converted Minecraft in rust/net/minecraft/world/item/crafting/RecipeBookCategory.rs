//! Port of: net/minecraft/world/item/crafting/RecipeBookCategory.java
//! Java class(es): net.minecraft.world.item.crafting.RecipeBookCategory
//! Status: PORTED
//!
//! Behavior-free marker type: the Java class has an empty body (no fields, no methods,
//! no `equals`/`hashCode`/`toString` overrides), so it is a plain value type whose only
//! contract is `implements ExtendedRecipeBookCategory`. Instances are the singletons
//! registered by `RecipeBookCategories.register()` and are looked up from the registry,
//! so comparing them by value (unit struct) is behaviorally identical to Java's identity
//! comparison for these singleton instances.
//!
//! Nothing to oracle-test: there is no behavior, no exception text, and no constant to
//! pin. Proof is compilation plus `impl ExtendedRecipeBookCategory for RecipeBookCategory`,
//! which lets it be used as a `HashMap`/`Registry` key wherever Java uses the class.

use crate::net::minecraft::world::item::crafting::ExtendedRecipeBookCategory::ExtendedRecipeBookCategory;

/// Port of `net.minecraft.world.item.crafting.RecipeBookCategory`.
///
/// A zero-sized marker: Java's `new RecipeBookCategory()` carries no state, so this unit
/// struct is a faithful mirror. `Clone`/`Copy` match the trivial copy semantics of an
/// object with no fields.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RecipeBookCategory;

impl ExtendedRecipeBookCategory for RecipeBookCategory {}
