//! Port of: net/minecraft/util/datafix/fixes/StonecutterRecipeRenameFix.java
//! Java class(es): net.minecraft.util.datafix.fixes.StonecutterRecipeRenameFix
//! Status: VERIFIED
//!
//! Session 24. `public class StonecutterRecipeRenameFix`: a data-only class -- stonecutter recipe ids, and their advancement ids, and their replacements.
//! 2 constants, each a Guava `ImmutableMap<String, String>` of old id -> new id, built with
//! `ImmutableMap.builder().put(..)...build()`: `RECIPE_RENAMES` (11 entries), `ADVANCEMENT_RENAMES` (11 entries).
//!
//! Each is a [`ImmutableStrMap`] (`javacompat::guava`): the put order is kept for every view,
//! as Guava keeps it, and `get`, `hashCode`, `toString` and `equals` follow the `Map`
//! contract. The class has no other member besides the implicit constructor of a class that
//! is never instantiated.
//!
//! Verified by the `RENAMEMAP` group of `sweep_probe.txt` (`tests/session24.rs`): every entry in
//! iteration order, `size`, `hashCode`, `toString`, and `get` of each key and of a missing one.

use crate::javacompat::guava::ImmutableStrMap;

/// Port of `public static final ImmutableMap<String, String> RECIPE_RENAMES` (11 entries, in put order).
pub static RECIPE_RENAMES: ImmutableStrMap = ImmutableStrMap::new(&[
    (
        "minecraft:chiseled_stone_bricks_stone_from_stonecutting",
        "minecraft:chiseled_stone_bricks_from_stone_stonecutting",
    ),
    (
        "minecraft:end_stone_brick_slab_from_end_stone_brick_stonecutting",
        "minecraft:end_stone_brick_slab_from_end_stone_bricks_stonecutting",
    ),
    (
        "minecraft:end_stone_brick_stairs_from_end_stone_brick_stonecutting",
        "minecraft:end_stone_brick_stairs_from_end_stone_bricks_stonecutting",
    ),
    (
        "minecraft:end_stone_brick_wall_from_end_stone_brick_stonecutting",
        "minecraft:end_stone_brick_wall_from_end_stone_bricks_stonecutting",
    ),
    (
        "minecraft:mossy_stone_brick_slab_from_mossy_stone_brick_stonecutting",
        "minecraft:mossy_stone_brick_slab_from_mossy_stone_bricks_stonecutting",
    ),
    (
        "minecraft:mossy_stone_brick_stairs_from_mossy_stone_brick_stonecutting",
        "minecraft:mossy_stone_brick_stairs_from_mossy_stone_bricks_stonecutting",
    ),
    (
        "minecraft:mossy_stone_brick_wall_from_mossy_stone_brick_stonecutting",
        "minecraft:mossy_stone_brick_wall_from_mossy_stone_bricks_stonecutting",
    ),
    (
        "minecraft:prismarine_brick_slab_from_prismarine_stonecutting",
        "minecraft:prismarine_brick_slab_from_prismarine_bricks_stonecutting",
    ),
    (
        "minecraft:prismarine_brick_stairs_from_prismarine_stonecutting",
        "minecraft:prismarine_brick_stairs_from_prismarine_bricks_stonecutting",
    ),
    (
        "minecraft:quartz_slab_from_stonecutting",
        "minecraft:quartz_slab_from_quartz_block_stonecutting",
    ),
    (
        "minecraft:stone_brick_walls_from_stone_stonecutting",
        "minecraft:stone_brick_wall_from_stone_stonecutting",
    ),
]);

/// Port of `public static final ImmutableMap<String, String> ADVANCEMENT_RENAMES` (11 entries, in put order).
pub static ADVANCEMENT_RENAMES: ImmutableStrMap = ImmutableStrMap::new(&[
    ("minecraft:recipes/building_blocks/chiseled_stone_bricks_stone_from_stonecutting", "minecraft:recipes/building_blocks/chiseled_stone_bricks_from_stone_stonecutting"),
    ("minecraft:recipes/building_blocks/end_stone_brick_slab_from_end_stone_brick_stonecutting", "minecraft:recipes/building_blocks/end_stone_brick_slab_from_end_stone_bricks_stonecutting"),
    ("minecraft:recipes/building_blocks/end_stone_brick_stairs_from_end_stone_brick_stonecutting", "minecraft:recipes/building_blocks/end_stone_brick_stairs_from_end_stone_bricks_stonecutting"),
    ("minecraft:recipes/decorations/end_stone_brick_wall_from_end_stone_brick_stonecutting", "minecraft:recipes/decorations/end_stone_brick_wall_from_end_stone_bricks_stonecutting"),
    ("minecraft:recipes/building_blocks/mossy_stone_brick_slab_from_mossy_stone_brick_stonecutting", "minecraft:recipes/building_blocks/mossy_stone_brick_slab_from_mossy_stone_bricks_stonecutting"),
    ("minecraft:recipes/building_blocks/mossy_stone_brick_stairs_from_mossy_stone_brick_stonecutting", "minecraft:recipes/building_blocks/mossy_stone_brick_stairs_from_mossy_stone_bricks_stonecutting"),
    ("minecraft:recipes/building_blocks/mossy_stone_brick_wall_from_mossy_stone_brick_stonecutting", "minecraft:recipes/decorations/mossy_stone_brick_wall_from_mossy_stone_bricks_stonecutting"),
    ("minecraft:recipes/building_blocks/prismarine_brick_slab_from_prismarine_stonecutting", "minecraft:recipes/building_blocks/prismarine_brick_slab_from_prismarine_bricks_stonecutting"),
    ("minecraft:recipes/building_blocks/prismarine_brick_stairs_from_prismarine_stonecutting", "minecraft:recipes/building_blocks/prismarine_brick_stairs_from_prismarine_bricks_stonecutting"),
    ("minecraft:recipes/building_blocks/quartz_slab_from_stonecutting", "minecraft:recipes/building_blocks/quartz_slab_from_quartz_block_stonecutting"),
    ("minecraft:recipes/decorations/stone_brick_walls_from_stone_stonecutting", "minecraft:recipes/decorations/stone_brick_wall_from_stone_stonecutting"),
]);
