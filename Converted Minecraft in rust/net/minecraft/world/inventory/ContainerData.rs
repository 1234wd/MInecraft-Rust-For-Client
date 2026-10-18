//! Port of: net/minecraft/world/inventory/ContainerData.java
//! Java class(es): net.minecraft.world.inventory.ContainerData
//! Status: VERIFIED
//!
//! Session 22. `public interface ContainerData` -- declaration only.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `net.minecraft.world.inventory.ContainerData`.
pub trait ContainerData {
    /// Port of `int get(int dataId)`.
    fn get(&self, data_id: i32) -> i32;

    /// Port of `void set(int dataId, int value)`.
    fn set(&mut self, data_id: i32, value: i32);

    /// Port of `int getCount()`.
    fn get_count(&self) -> i32;
}
