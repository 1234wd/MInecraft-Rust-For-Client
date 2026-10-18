//! Port of: net/minecraft/client/gui/components/debug/DebugScreenDisplayer.java
//! Java class(es): net.minecraft.client.gui.components.debug.DebugScreenDisplayer
//! Status: VERIFIED
//!
//! Session 22. `public interface DebugScreenDisplayer` -- declaration only.
//!
//! `addToGroup` is overloaded on `Collection<String>` and `String`; per
//! `DESIGN_DECISIONS.md#overload-naming` they are `add_to_group_collection` and `add_to_group_string`.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

use crate::net::minecraft::resources::Identifier::Identifier;

/// Port of `net.minecraft.client.gui.components.debug.DebugScreenDisplayer`.
pub trait DebugScreenDisplayer {
    /// Port of `void addPriorityLine(String line)`.
    fn add_priority_line(&mut self, line: &str);

    /// Port of `void addLine(String line)`.
    fn add_line(&mut self, line: &str);

    /// Port of `void addToGroup(Identifier group, Collection<String> lines)`.
    fn add_to_group_collection(&mut self, group: &Identifier, lines: &[String]);

    /// Port of `void addToGroup(Identifier group, String lines)`.
    fn add_to_group_string(&mut self, group: &Identifier, lines: &str);
}
