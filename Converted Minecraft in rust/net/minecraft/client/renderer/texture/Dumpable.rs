//! Port of: net/minecraft/client/renderer/texture/Dumpable.java
//! Java class(es): net.minecraft.client.renderer.texture.Dumpable
//! Status: VERIFIED
//!
//! Session 22. `public interface Dumpable` -- declaration only.
//!
//! `dumpContents` declares `throws IOException`, so the port returns `std::io::Result<()>`.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

use crate::net::minecraft::resources::Identifier::Identifier;

/// Port of `net.minecraft.client.renderer.texture.Dumpable`.
pub trait Dumpable {
    /// Port of `void dumpContents(Identifier selfId, Path dir) throws IOException`.
    fn dump_contents(&self, self_id: &Identifier, dir: &std::path::Path) -> std::io::Result<()>;
}
