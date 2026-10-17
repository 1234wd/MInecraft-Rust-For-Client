//! Port of: net/minecraft/world/flag/FeatureFlagUniverse.java
//! Java class(es): net.minecraft.world.flag.FeatureFlagUniverse
//! Status: VERIFIED
//!
//! Session 22. `public class FeatureFlagUniverse`: a named namespace that feature flags belong to.
//! One `final String id`, and `toString()` returns it.
//!
//! # Identity, not value
//!
//! The class does not override `equals`/`hashCode`, so two universes with the same id are NOT
//! equal in Java -- flags from different universe objects never mix, even with matching names. The
//! port therefore derives no `PartialEq`; compare universes with `std::ptr::eq`, as Java's `==`
//! does. The probe pins `new FeatureFlagUniverse("main").equals(new FeatureFlagUniverse("main"))`
//! as `false`.
//!
//! Verified by the `FFUNIV` group of `small_probe.txt` (`tests/session22.rs`).

/// Port of `net.minecraft.world.flag.FeatureFlagUniverse`.
#[derive(Debug)]
pub struct FeatureFlagUniverse {
    id: String,
}

impl FeatureFlagUniverse {
    /// Port of `new FeatureFlagUniverse(String id)`.
    pub fn new(id: &str) -> Self {
        FeatureFlagUniverse { id: id.to_string() }
    }
}

impl std::fmt::Display for FeatureFlagUniverse {
    /// Port of `toString()`: the id.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}
