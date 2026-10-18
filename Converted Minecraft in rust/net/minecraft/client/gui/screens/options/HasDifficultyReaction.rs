//! Port of: net/minecraft/client/gui/screens/options/HasDifficultyReaction.java
//! Java class(es): net.minecraft.client.gui.screens.options.HasDifficultyReaction
//! Status: VERIFIED
//!
//! Session 22. `public interface HasDifficultyReaction` -- declaration only.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `net.minecraft.client.gui.screens.options.HasDifficultyReaction`.
pub trait HasDifficultyReaction {
    /// Port of `void onDifficultyChanged()`.
    fn on_difficulty_changed(&mut self);
}
