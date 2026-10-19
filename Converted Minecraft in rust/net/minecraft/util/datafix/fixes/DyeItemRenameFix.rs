//! Port of: net/minecraft/util/datafix/fixes/DyeItemRenameFix.java
//! Java class(es): net.minecraft.util.datafix.fixes.DyeItemRenameFix
//! Status: VERIFIED
//!
//! Session 24. `public class DyeItemRenameFix`: a data-only class -- old dye item ids and their replacements.
//! One constant, `RENAMED_IDS`, a Guava `ImmutableMap<String, String>` of 3 old id -> new id
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

/// Port of `public static final Map<String, String> RENAMED_IDS` (3 entries, in put order).
pub static RENAMED_IDS: ImmutableStrMap = ImmutableStrMap::new(&[
    ("minecraft:cactus_green", "minecraft:green_dye"),
    ("minecraft:rose_red", "minecraft:red_dye"),
    ("minecraft:dandelion_yellow", "minecraft:yellow_dye"),
]);
