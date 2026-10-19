//! Port of: net/minecraft/util/datafix/fixes/RecipesRenameningFix.java
//! Java class(es): net.minecraft.util.datafix.fixes.RecipesRenameningFix
//! Status: VERIFIED
//!
//! Session 24. `public class RecipesRenameningFix`: a data-only class -- `*_bark` recipe ids and their `*_wood` replacements.
//! One constant, `RECIPES`, a Guava `ImmutableMap<String, String>` of 6 old id -> new id
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

/// Port of `public static final Map<String, String> RECIPES` (6 entries, in put order).
pub static RECIPES: ImmutableStrMap = ImmutableStrMap::new(&[
    ("minecraft:acacia_bark", "minecraft:acacia_wood"),
    ("minecraft:birch_bark", "minecraft:birch_wood"),
    ("minecraft:dark_oak_bark", "minecraft:dark_oak_wood"),
    ("minecraft:jungle_bark", "minecraft:jungle_wood"),
    ("minecraft:oak_bark", "minecraft:oak_wood"),
    ("minecraft:spruce_bark", "minecraft:spruce_wood"),
]);
