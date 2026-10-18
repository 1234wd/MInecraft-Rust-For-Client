//! Port of: com/mojang/blaze3d/textures/GpuSampler.java
//! Java class(es): com.mojang.blaze3d.textures.GpuSampler
//! Status: VERIFIED
//!
//! Session 22. `public abstract class GpuSampler implements AutoCloseable` -- declaration only.
//!
//! An ABSTRACT CLASS with no fields and only abstract methods, so it is a trait like the
//! interfaces. `OptionalDouble` becomes `Option<f64>`; `AutoCloseable`'s `close()` is a method here.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

use crate::com::mojang::blaze3d::textures::AddressMode::AddressMode;
use crate::com::mojang::blaze3d::textures::FilterMode::FilterMode;

/// Port of `com.mojang.blaze3d.textures.GpuSampler`.
pub trait GpuSampler {
    /// Port of `AddressMode getAddressModeU()`.
    fn get_address_mode_u(&self) -> AddressMode;

    /// Port of `AddressMode getAddressModeV()`.
    fn get_address_mode_v(&self) -> AddressMode;

    /// Port of `FilterMode getMinFilter()`.
    fn get_min_filter(&self) -> FilterMode;

    /// Port of `FilterMode getMagFilter()`.
    fn get_mag_filter(&self) -> FilterMode;

    /// Port of `int getMaxAnisotropy()`.
    fn get_max_anisotropy(&self) -> i32;

    /// Port of `OptionalDouble getMaxLod()`.
    fn get_max_lod(&self) -> Option<f64>;

    /// Port of `void close()`.
    fn close(&mut self);
}
