//! Port of: net/minecraft/util/datafix/fixes/RecipesFix.java
//! Java class(es): net.minecraft.util.datafix.fixes.RecipesFix
//! Status: VERIFIED
//!
//! Session 24. `public class RecipesFix`: a data-only class -- old recipe ids and their replacements.
//! One constant, `RECIPES`, a Guava `ImmutableMap<String, String>` of 54 old id -> new id
//! entries, built with `ImmutableMap.builder().put(..)...build()`.
//!
//! Each is a [`ImmutableStrMap`] (`javacompat::guava`): the put order is kept for every view,
//! as Guava keeps it, and `get`, `hashCode`, `toString` and `equals` follow the `Map`
//! contract. The class has no other member besides the implicit constructor of a class that
//! is never instantiated.
//!
//! Verified by the `RENAMEMAP` group of `sweep_probe.txt` (`tests/session24.rs`): every entry in
//! iteration order, `size`, `hashCode`, `toString`, and `get` of each key and of a missing one.

use crate::javacompat::guava::ImmutableStrMap;

/// Port of `public static final Map<String, String> RECIPES` (54 entries, in put order).
pub static RECIPES: ImmutableStrMap = ImmutableStrMap::new(&[
    ("minecraft:acacia_wooden_slab", "minecraft:acacia_slab"),
    ("minecraft:birch_wooden_slab", "minecraft:birch_slab"),
    (
        "minecraft:black_stained_hardened_clay",
        "minecraft:black_terracotta",
    ),
    (
        "minecraft:blue_stained_hardened_clay",
        "minecraft:blue_terracotta",
    ),
    ("minecraft:boat", "minecraft:oak_boat"),
    (
        "minecraft:bone_meal_from_block",
        "minecraft:bone_meal_from_bone_block",
    ),
    ("minecraft:bone_meal_from_bone", "minecraft:bone_meal"),
    ("minecraft:brick_block", "minecraft:bricks"),
    (
        "minecraft:brown_stained_hardened_clay",
        "minecraft:brown_terracotta",
    ),
    (
        "minecraft:chiseled_stonebrick",
        "minecraft:chiseled_stone_bricks",
    ),
    (
        "minecraft:cyan_stained_hardened_clay",
        "minecraft:cyan_terracotta",
    ),
    ("minecraft:dark_oak_wooden_slab", "minecraft:dark_oak_slab"),
    ("minecraft:end_bricks", "minecraft:end_stone_bricks"),
    ("minecraft:fence_gate", "minecraft:oak_fence_gate"),
    ("minecraft:fence", "minecraft:oak_fence"),
    ("minecraft:golden_rail", "minecraft:powered_rail"),
    (
        "minecraft:gold_ingot_from_block",
        "minecraft:gold_ingot_from_gold_block",
    ),
    (
        "minecraft:gray_stained_hardened_clay",
        "minecraft:gray_terracotta",
    ),
    (
        "minecraft:green_stained_hardened_clay",
        "minecraft:green_terracotta",
    ),
    (
        "minecraft:iron_ingot_from_block",
        "minecraft:iron_ingot_from_iron_block",
    ),
    ("minecraft:jungle_wooden_slab", "minecraft:jungle_slab"),
    (
        "minecraft:light_blue_stained_hardened_clay",
        "minecraft:light_blue_terracotta",
    ),
    (
        "minecraft:light_gray_stained_hardened_clay",
        "minecraft:light_gray_terracotta",
    ),
    (
        "minecraft:lime_stained_hardened_clay",
        "minecraft:lime_terracotta",
    ),
    ("minecraft:lit_pumpkin", "minecraft:jack_o_lantern"),
    (
        "minecraft:magenta_stained_hardened_clay",
        "minecraft:magenta_terracotta",
    ),
    ("minecraft:magma", "minecraft:magma_block"),
    ("minecraft:melon_block", "minecraft:melon"),
    ("minecraft:mossy_stonebrick", "minecraft:mossy_stone_bricks"),
    ("minecraft:noteblock", "minecraft:note_block"),
    ("minecraft:oak_wooden_slab", "minecraft:oak_slab"),
    (
        "minecraft:orange_stained_hardened_clay",
        "minecraft:orange_terracotta",
    ),
    ("minecraft:pillar_quartz_block", "minecraft:quartz_pillar"),
    (
        "minecraft:pink_stained_hardened_clay",
        "minecraft:pink_terracotta",
    ),
    ("minecraft:purple_shulker_box", "minecraft:shulker_box"),
    (
        "minecraft:purple_stained_hardened_clay",
        "minecraft:purple_terracotta",
    ),
    ("minecraft:red_nether_brick", "minecraft:red_nether_bricks"),
    (
        "minecraft:red_stained_hardened_clay",
        "minecraft:red_terracotta",
    ),
    ("minecraft:slime", "minecraft:slime_block"),
    (
        "minecraft:smooth_red_sandstone",
        "minecraft:cut_red_sandstone",
    ),
    ("minecraft:smooth_sandstone", "minecraft:cut_sandstone"),
    ("minecraft:snow_layer", "minecraft:snow"),
    ("minecraft:snow", "minecraft:snow_block"),
    (
        "minecraft:speckled_melon",
        "minecraft:glistering_melon_slice",
    ),
    ("minecraft:spruce_wooden_slab", "minecraft:spruce_slab"),
    ("minecraft:stonebrick", "minecraft:stone_bricks"),
    ("minecraft:stone_stairs", "minecraft:cobblestone_stairs"),
    (
        "minecraft:string_to_wool",
        "minecraft:white_wool_from_string",
    ),
    ("minecraft:trapdoor", "minecraft:oak_trapdoor"),
    (
        "minecraft:white_stained_hardened_clay",
        "minecraft:white_terracotta",
    ),
    ("minecraft:wooden_button", "minecraft:oak_button"),
    ("minecraft:wooden_door", "minecraft:oak_door"),
    (
        "minecraft:wooden_pressure_plate",
        "minecraft:oak_pressure_plate",
    ),
    (
        "minecraft:yellow_stained_hardened_clay",
        "minecraft:yellow_terracotta",
    ),
]);
