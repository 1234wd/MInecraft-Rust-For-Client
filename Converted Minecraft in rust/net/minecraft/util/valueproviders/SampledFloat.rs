//! Port of: net/minecraft/util/valueproviders/SampledFloat.java
//! Java class(es): net.minecraft.util.valueproviders.SampledFloat
//! Status: VERIFIED
//!
//! Session 22. `public interface SampledFloat` -- declaration only.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

use crate::net::minecraft::util::RandomSource::RandomSource;

/// Port of `net.minecraft.util.valueproviders.SampledFloat`.
pub trait SampledFloat {
    /// Port of `float sample(RandomSource random)`.
    fn sample(&self, random: &mut dyn RandomSource) -> f32;
}
