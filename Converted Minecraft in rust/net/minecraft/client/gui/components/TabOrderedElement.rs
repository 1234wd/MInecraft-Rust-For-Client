//! Port of: net/minecraft/client/gui/components/TabOrderedElement.java
//! Java class(es): net.minecraft.client.gui.components.TabOrderedElement
//! Status: VERIFIED
//!
//! Session 22. `public interface TabOrderedElement`: one default method, the element's group for
//! Tab-key focus order, `0` unless an implementor overrides it.
//!
//! Verified by the `TABORDER` group of `small_probe.txt` (`tests/session22.rs`): the default on an
//! anonymous implementation.

/// Port of `net.minecraft.client.gui.components.TabOrderedElement`.
pub trait TabOrderedElement {
    /// Port of `default int getTabOrderGroup()`: `0`.
    fn get_tab_order_group(&self) -> i32 {
        0
    }
}
