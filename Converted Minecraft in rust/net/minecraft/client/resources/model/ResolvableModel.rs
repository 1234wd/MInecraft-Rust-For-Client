//! Port of: net/minecraft/client/resources/model/ResolvableModel.java
//! Java class(es): net.minecraft.client.resources.model.ResolvableModel, net.minecraft.client.resources.model.Resolver
//! Status: VERIFIED
//!
//! Session 22. `public interface ResolvableModel` -- declaration only.
//!
//! The nested `ResolvableModel.Resolver` is the second trait in this file.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

use crate::net::minecraft::resources::Identifier::Identifier;

/// Port of `net.minecraft.client.resources.model.ResolvableModel`.
pub trait ResolvableModel {
    /// Port of `void resolveDependencies(ResolvableModel.Resolver resolver)`.
    fn resolve_dependencies(&self, resolver: &mut dyn Resolver);
}

/// Port of `ResolvableModel.Resolver`.
pub trait Resolver {
    /// Port of `void markDependency(Identifier id)`.
    fn mark_dependency(&mut self, id: &Identifier);
}
