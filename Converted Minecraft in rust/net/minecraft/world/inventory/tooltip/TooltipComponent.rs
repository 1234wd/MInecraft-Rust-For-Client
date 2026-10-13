//! Port of: net/minecraft/world/inventory/tooltip/TooltipComponent.java
//! Java class(es): net.minecraft.world.inventory.tooltip.TooltipComponent
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A marker interface implemented by the payload records that decorate a tooltip (bundle
//! contents, map coordinates, the modded tooltip extensions). It is matched with `instanceof`
//! and, in vanilla, never called through -- each consumer casts to the concrete record it
//! wants. So an empty trait is the whole port.

/// Port of `net.minecraft.world.inventory.tooltip.TooltipComponent`.
///
/// Empty on purpose -- see the module docs.
pub trait TooltipComponent {}