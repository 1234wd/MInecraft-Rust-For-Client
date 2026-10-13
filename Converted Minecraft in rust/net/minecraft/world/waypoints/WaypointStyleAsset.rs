//! Port of: net/minecraft/world/waypoints/WaypointStyleAsset.java
//! Java class(es): net.minecraft.world.waypoints.WaypointStyleAsset
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A `ResourceLocation` subclass with no members -- the same shape as `EquipmentAsset`. In Java
//! it is a distinct *type* whose only job is to stop a waypoint style path being passed where
//! some other `ResourceLocation` is expected.
//!
//! # WHY NO NEWTYPE, AGAIN
//!
//! `EquipmentAsset` carries the reasoning in full: wrapping [`Identifier`] in a newtype would be
//! idiomatic Rust but would force a conversion at every call site and add a guarantee the Java
//! does not have either. Here, `Identifier` is ported, so the honest difference is worth
//! recording: `EquipmentAsset` predates it in the queue and was written without it, and this
//! one could use it. They are kept consistent with each other rather than with an idea, because
//! two identically-motivated marker classes that disagree are harder to reason about than two
//! that agree.

/// Port of `net.minecraft.world.waypoints.WaypointStyleAsset`.
///
/// A distinct type carrying no state; see the module docs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WaypointStyleAsset;