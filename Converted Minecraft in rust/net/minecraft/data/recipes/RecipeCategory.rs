//! Port of: net/minecraft/data/recipes/RecipeCategory.java
//! Java class(es): net.minecraft.data.recipes.RecipeCategory
//! Status: VERIFIED
//!
//! Session 19. Nine constants, each with the `recipeFolderName` used by the recipe data
//! generator. Names, ordinals and `recipeFolderName` are checked against the jar in
//! `_porting/tests/session19_b.rs`.

/// Port of `net.minecraft.data.recipes.RecipeCategory`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RecipeCategory {
    /// Port of `RecipeCategory#BUILDING_BLOCKS` -- ordinal 0, folder `"building_blocks"`.
    BuildingBlocks = 0,
    /// Port of `RecipeCategory#DECORATIONS` -- ordinal 1, folder `"decorations"`.
    Decorations = 1,
    /// Port of `RecipeCategory#REDSTONE` -- ordinal 2, folder `"redstone"`.
    Redstone = 2,
    /// Port of `RecipeCategory#TRANSPORTATION` -- ordinal 3, folder `"transportation"`.
    Transportation = 3,
    /// Port of `RecipeCategory#TOOLS` -- ordinal 4, folder `"tools"`.
    Tools = 4,
    /// Port of `RecipeCategory#COMBAT` -- ordinal 5, folder `"combat"`.
    Combat = 5,
    /// Port of `RecipeCategory#FOOD` -- ordinal 6, folder `"food"`.
    Food = 6,
    /// Port of `RecipeCategory#BREWING` -- ordinal 7, folder `"brewing"`.
    Brewing = 7,
    /// Port of `RecipeCategory#MISC` -- ordinal 8, folder `"misc"`.
    Misc = 8,
}

/// Java: `RecipeCategory.values()` -- declaration order.
pub const VALUES: [RecipeCategory; 9] = [
    RecipeCategory::BuildingBlocks,
    RecipeCategory::Decorations,
    RecipeCategory::Redstone,
    RecipeCategory::Transportation,
    RecipeCategory::Tools,
    RecipeCategory::Combat,
    RecipeCategory::Food,
    RecipeCategory::Brewing,
    RecipeCategory::Misc,
];

impl RecipeCategory {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            RecipeCategory::BuildingBlocks => "BUILDING_BLOCKS",
            RecipeCategory::Decorations => "DECORATIONS",
            RecipeCategory::Redstone => "REDSTONE",
            RecipeCategory::Transportation => "TRANSPORTATION",
            RecipeCategory::Tools => "TOOLS",
            RecipeCategory::Combat => "COMBAT",
            RecipeCategory::Food => "FOOD",
            RecipeCategory::Brewing => "BREWING",
            RecipeCategory::Misc => "MISC",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `RecipeCategory#getFolderName()` -- the `recipeFolderName` field.
    #[inline]
    pub const fn get_folder_name(&self) -> &'static str {
        match self {
            RecipeCategory::BuildingBlocks => "building_blocks",
            RecipeCategory::Decorations => "decorations",
            RecipeCategory::Redstone => "redstone",
            RecipeCategory::Transportation => "transportation",
            RecipeCategory::Tools => "tools",
            RecipeCategory::Combat => "combat",
            RecipeCategory::Food => "food",
            RecipeCategory::Brewing => "brewing",
            RecipeCategory::Misc => "misc",
        }
    }
}

impl std::fmt::Display for RecipeCategory {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
