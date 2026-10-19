//! Port of: net/minecraft/util/datafix/fixes/RenamedCoralFansFix.java
//! Java class(es): net.minecraft.util.datafix.fixes.RenamedCoralFansFix
//! Status: VERIFIED
//!
//! Session 24. `public class RenamedCoralFansFix`: a data-only class -- coral fan ids and their wall-fan replacements.
//! One constant, `RENAMED_IDS`, a Guava `ImmutableMap<String, String>` of 5 old id -> new id
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

/// Port of `public static final Map<String, String> RENAMED_IDS` (5 entries, in put order).
pub static RENAMED_IDS: ImmutableStrMap = ImmutableStrMap::new(&[
    ("minecraft:tube_coral_fan", "minecraft:tube_coral_wall_fan"),
    (
        "minecraft:brain_coral_fan",
        "minecraft:brain_coral_wall_fan",
    ),
    (
        "minecraft:bubble_coral_fan",
        "minecraft:bubble_coral_wall_fan",
    ),
    ("minecraft:fire_coral_fan", "minecraft:fire_coral_wall_fan"),
    ("minecraft:horn_coral_fan", "minecraft:horn_coral_wall_fan"),
]);
