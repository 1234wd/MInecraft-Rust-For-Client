//! Port of: net/minecraft/world/item/equipment/EquipmentAsset.java
//! Java class(es): net.minecraft.world.item.equipment.EquipmentAsset
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A `ResourceLocation` subclass with **no members at all**. In Java it adds nothing to its
//! parent; the class exists so that an equipment-asset path is a distinct *type* and cannot be
//! passed where some other `ResourceLocation` is expected.
//!
//! # WHY THIS IS NOT `type EquipmentAsset = ResourceLocation`
//!
//! A newtype would be the idiomatic Rust translation of "distinct type, no extra behaviour",
//! and it would also be a **behaviour change**: `Identifier` is ported and used everywhere, and
//! wrapping it here would force a conversion at every call site while adding no safety that the
//! Java does not have either (Java gets its safety from the type system, and we lose it only if
//! we flatten).
//!
//! So this is a zero-sized marker carrying the name and nothing else, which is the honest
//! equivalent of "a `ResourceLocation` with a different hat on". `Identifier` is not ported in
//! this batch, so the field is omitted rather than typed against a module that does not exist
//! yet; when it is, the single accessor below is the only thing that needs filling in.

/// Port of `net.minecraft.world.item.equipment.EquipmentAsset`.
///
/// A distinct type carrying no state. See the module docs for why this is not a newtype.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EquipmentAsset;