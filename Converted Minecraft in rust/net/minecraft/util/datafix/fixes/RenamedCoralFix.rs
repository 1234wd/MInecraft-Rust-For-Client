//! Port of: net/minecraft/util/datafix/fixes/RenamedCoralFix.java
//! Java class(es): net.minecraft.util.datafix.fixes.RenamedCoralFix
//! Status: VERIFIED
//!
//! Session 24. `public class RenamedCoralFix`: a data-only class -- old coral block, plant and fan ids and their replacements.
//! One constant, `RENAMED_IDS`, a Guava `ImmutableMap<String, String>` of 20 old id -> new id
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

/// Port of `public static final Map<String, String> RENAMED_IDS` (20 entries, in put order).
pub static RENAMED_IDS: ImmutableStrMap = ImmutableStrMap::new(&[
    ("minecraft:blue_coral", "minecraft:tube_coral_block"),
    ("minecraft:pink_coral", "minecraft:brain_coral_block"),
    ("minecraft:purple_coral", "minecraft:bubble_coral_block"),
    ("minecraft:red_coral", "minecraft:fire_coral_block"),
    ("minecraft:yellow_coral", "minecraft:horn_coral_block"),
    ("minecraft:blue_coral_plant", "minecraft:tube_coral"),
    ("minecraft:pink_coral_plant", "minecraft:brain_coral"),
    ("minecraft:purple_coral_plant", "minecraft:bubble_coral"),
    ("minecraft:red_coral_plant", "minecraft:fire_coral"),
    ("minecraft:yellow_coral_plant", "minecraft:horn_coral"),
    ("minecraft:blue_coral_fan", "minecraft:tube_coral_fan"),
    ("minecraft:pink_coral_fan", "minecraft:brain_coral_fan"),
    ("minecraft:purple_coral_fan", "minecraft:bubble_coral_fan"),
    ("minecraft:red_coral_fan", "minecraft:fire_coral_fan"),
    ("minecraft:yellow_coral_fan", "minecraft:horn_coral_fan"),
    ("minecraft:blue_dead_coral", "minecraft:dead_tube_coral"),
    ("minecraft:pink_dead_coral", "minecraft:dead_brain_coral"),
    ("minecraft:purple_dead_coral", "minecraft:dead_bubble_coral"),
    ("minecraft:red_dead_coral", "minecraft:dead_fire_coral"),
    ("minecraft:yellow_dead_coral", "minecraft:dead_horn_coral"),
]);
