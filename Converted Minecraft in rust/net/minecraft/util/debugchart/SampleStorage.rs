//! Port of: net/minecraft/util/debugchart/SampleStorage.java
//! Java class(es): net.minecraft.util.debugchart.SampleStorage
//! Status: VERIFIED
//!
//! Session 22. `public interface SampleStorage` -- declaration only.
//!
//! `get` is overloaded, so per `DESIGN_DECISIONS.md#overload-naming` the two are `get_i32` and
//! `get_i32_i32`.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `net.minecraft.util.debugchart.SampleStorage`.
pub trait SampleStorage {
    /// Port of `int capacity()`.
    fn capacity(&self) -> i32;

    /// Port of `int size()`.
    fn size(&self) -> i32;

    /// Port of `long get(int index)`.
    fn get_i32(&self, index: i32) -> i64;

    /// Port of `long get(int index, int dimension)`.
    fn get_i32_i32(&self, index: i32, dimension: i32) -> i64;

    /// Port of `void reset()`.
    fn reset(&mut self);
}
