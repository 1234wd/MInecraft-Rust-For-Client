//! Port of: net/minecraft/world/inventory/ClickAction.java
//! Java class(es): net.minecraft.world.inventory.ClickAction
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A two-constant enum. The **declaration order is the ordinal** and the ordinal is what
//! `AbstractContainerMenu` writes into its click packet, so the variant order below is
//! load-bearing: `PRIMARY` must stay 0 and `SECONDARY` 1, or every saved click would apply to
//! the wrong slot action.

/// Port of `net.minecraft.world.inventory.ClickAction`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClickAction {
    /// Port of `ClickAction#PRIMARY` -- ordinal 0.
    Primary = 0,
    /// Port of `ClickAction#SECONDARY` -- ordinal 1.
    Secondary = 1,
}

/// Java: `ClickAction.values()`.
///
/// Declaration order, which is also the persisted ordinal order.
pub const VALUES: [ClickAction; 2] = [ClickAction::Primary, ClickAction::Secondary];