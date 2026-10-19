//! Port of: net/minecraft/util/datafix/fixes/CavesAndCliffsRenames.java
//! Java class(es): net.minecraft.util.datafix.fixes.CavesAndCliffsRenames
//! Status: VERIFIED
//!
//! Session 24. `public final class CavesAndCliffsRenames`: a data-only class -- removed or renamed biome ids and the biome each became.
//! One constant, `RENAMES`, a Guava `ImmutableMap<String, String>` of 38 old id -> new id
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

/// Port of `public static final ImmutableMap<String, String> RENAMES` (38 entries, in put order).
pub static RENAMES: ImmutableStrMap = ImmutableStrMap::new(&[
    ("minecraft:badlands_plateau", "minecraft:badlands"),
    ("minecraft:bamboo_jungle_hills", "minecraft:bamboo_jungle"),
    ("minecraft:birch_forest_hills", "minecraft:birch_forest"),
    ("minecraft:dark_forest_hills", "minecraft:dark_forest"),
    ("minecraft:desert_hills", "minecraft:desert"),
    ("minecraft:desert_lakes", "minecraft:desert"),
    (
        "minecraft:giant_spruce_taiga_hills",
        "minecraft:old_growth_spruce_taiga",
    ),
    (
        "minecraft:giant_spruce_taiga",
        "minecraft:old_growth_spruce_taiga",
    ),
    (
        "minecraft:giant_tree_taiga_hills",
        "minecraft:old_growth_pine_taiga",
    ),
    (
        "minecraft:giant_tree_taiga",
        "minecraft:old_growth_pine_taiga",
    ),
    (
        "minecraft:gravelly_mountains",
        "minecraft:windswept_gravelly_hills",
    ),
    ("minecraft:jungle_edge", "minecraft:sparse_jungle"),
    ("minecraft:jungle_hills", "minecraft:jungle"),
    ("minecraft:modified_badlands_plateau", "minecraft:badlands"),
    (
        "minecraft:modified_gravelly_mountains",
        "minecraft:windswept_gravelly_hills",
    ),
    ("minecraft:modified_jungle_edge", "minecraft:sparse_jungle"),
    ("minecraft:modified_jungle", "minecraft:jungle"),
    (
        "minecraft:modified_wooded_badlands_plateau",
        "minecraft:wooded_badlands",
    ),
    ("minecraft:mountain_edge", "minecraft:windswept_hills"),
    ("minecraft:mountains", "minecraft:windswept_hills"),
    (
        "minecraft:mushroom_field_shore",
        "minecraft:mushroom_fields",
    ),
    ("minecraft:shattered_savanna", "minecraft:windswept_savanna"),
    (
        "minecraft:shattered_savanna_plateau",
        "minecraft:windswept_savanna",
    ),
    ("minecraft:snowy_mountains", "minecraft:snowy_plains"),
    ("minecraft:snowy_taiga_hills", "minecraft:snowy_taiga"),
    ("minecraft:snowy_taiga_mountains", "minecraft:snowy_taiga"),
    ("minecraft:snowy_tundra", "minecraft:snowy_plains"),
    ("minecraft:stone_shore", "minecraft:stony_shore"),
    ("minecraft:swamp_hills", "minecraft:swamp"),
    ("minecraft:taiga_hills", "minecraft:taiga"),
    ("minecraft:taiga_mountains", "minecraft:taiga"),
    (
        "minecraft:tall_birch_forest",
        "minecraft:old_growth_birch_forest",
    ),
    (
        "minecraft:tall_birch_hills",
        "minecraft:old_growth_birch_forest",
    ),
    (
        "minecraft:wooded_badlands_plateau",
        "minecraft:wooded_badlands",
    ),
    ("minecraft:wooded_hills", "minecraft:forest"),
    ("minecraft:wooded_mountains", "minecraft:windswept_forest"),
    ("minecraft:lofty_peaks", "minecraft:jagged_peaks"),
    ("minecraft:snowcapped_peaks", "minecraft:frozen_peaks"),
]);
