//! Port of: net/minecraft/world/inventory/RecipeBookType.java
//! Java class(es): net.minecraft.world.inventory.RecipeBookType
//! Status: VERIFIED
//!
//! Four constants with no payload fields. Verified against the jar by `_porting/tests/consts.rs`:
//! ordinals for every constant.
//!
//! # DECLARATION ORDER = ORDINAL ORDER = THE WIRE FORMAT
//!
//! `RecipeBookType` is serialised by ordinal in `RecipeBookState` save data. The ordinals are
//! `CRAFTING(0)`, `FURNACE(1)`, `BLAST_FURNACE(2)`, `SMOKER(3)`. Adding or reordering constants
//! would corrupt existing saves.

/// Port of `net.minecraft.world.inventory.RecipeBookType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecipeBookType {
    /// Port of `RecipeBookType#CRAFTING` -- ordinal 0.
    Crafting = 0,
    /// Port of `RecipeBookType#FURNACE` -- ordinal 1.
    Furnace = 1,
    /// Port of `RecipeBookType#BLAST_FURNACE` -- ordinal 2.
    BlastFurnace = 2,
    /// Port of `RecipeBookType#SMOKER` -- ordinal 3.
    Smoker = 3,
}

/// Java: `RecipeBookType.values()` -- declaration order.
pub const VALUES: [RecipeBookType; 4] = [
    RecipeBookType::Crafting,
    RecipeBookType::Furnace,
    RecipeBookType::BlastFurnace,
    RecipeBookType::Smoker,
];
