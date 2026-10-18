//! Port of: com/mojang/blaze3d/systems/GpuQueryPool.java
//! Java class(es): com.mojang.blaze3d.systems.GpuQueryPool
//! Status: VERIFIED
//!
//! Session 22. `public interface GpuQueryPool extends AutoCloseable` -- declaration only.
//!
//! `OptionalLong` becomes `Option<i64>` and `OptionalLong[]` a `Vec`; `AutoCloseable`'s `close()` is
//! a method of this trait.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `com.mojang.blaze3d.systems.GpuQueryPool`.
pub trait GpuQueryPool {
    /// Port of `int size()`.
    fn size(&self) -> i32;

    /// Port of `OptionalLong getValue(int index)`.
    fn get_value(&self, index: i32) -> Option<i64>;

    /// Port of `OptionalLong[] getValues(int index, int count)`.
    fn get_values(&self, index: i32, count: i32) -> Vec<Option<i64>>;

    /// Port of `void close()`.
    fn close(&mut self);
}
