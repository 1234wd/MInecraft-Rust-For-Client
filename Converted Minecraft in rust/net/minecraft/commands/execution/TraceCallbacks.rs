//! Port of: net/minecraft/commands/execution/TraceCallbacks.java
//! Java class(es): net.minecraft.commands.execution.TraceCallbacks
//! Status: VERIFIED
//!
//! Session 22. `public interface TraceCallbacks extends AutoCloseable` -- declaration only.
//!
//! `AutoCloseable`'s `close()` is a method of this trait.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

use crate::net::minecraft::resources::Identifier::Identifier;

/// Port of `net.minecraft.commands.execution.TraceCallbacks`.
pub trait TraceCallbacks {
    /// Port of `void onCommand(int depth, String command)`.
    fn on_command(&mut self, depth: i32, command: &str);

    /// Port of `void onReturn(int depth, String command, int result)`.
    fn on_return(&mut self, depth: i32, command: &str, result: i32);

    /// Port of `void onError(String message)`.
    fn on_error(&mut self, message: &str);

    /// Port of `void onCall(int depth, Identifier function, int size)`.
    fn on_call(&mut self, depth: i32, function: &Identifier, size: i32);

    /// Port of `void close()`.
    fn close(&mut self);
}
