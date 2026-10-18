//! Port of: net/minecraft/client/gui/screens/recipebook/SlotSelectTime.java
//! Java class(es): net.minecraft.client.gui.screens.recipebook.SlotSelectTime
//! Status: VERIFIED
//!
//! Session 22. `@FunctionalInterface public interface SlotSelectTime` -- declaration only.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `net.minecraft.client.gui.screens.recipebook.SlotSelectTime`.
pub trait SlotSelectTime {
    /// Port of `int currentIndex()`.
    fn current_index(&self) -> i32;
}
