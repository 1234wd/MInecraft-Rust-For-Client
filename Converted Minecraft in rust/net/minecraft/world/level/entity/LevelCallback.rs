//! Port of: net/minecraft/world/level/entity/LevelCallback.java
//! Java class(es): net.minecraft.world.level.entity.LevelCallback
//! Status: VERIFIED
//!
//! Session 22. `public interface LevelCallback<T>` -- declaration only.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `net.minecraft.world.level.entity.LevelCallback`.
pub trait LevelCallback<T> {
    /// Port of `void onCreated(T entity)`.
    fn on_created(&mut self, entity: T);

    /// Port of `void onDestroyed(T entity)`.
    fn on_destroyed(&mut self, entity: T);

    /// Port of `void onTickingStart(T entity)`.
    fn on_ticking_start(&mut self, entity: T);

    /// Port of `void onTickingEnd(T entity)`.
    fn on_ticking_end(&mut self, entity: T);

    /// Port of `void onTrackingStart(T entity)`.
    fn on_tracking_start(&mut self, entity: T);

    /// Port of `void onTrackingEnd(T entity)`.
    fn on_tracking_end(&mut self, entity: T);

    /// Port of `void onSectionChange(T entity)`.
    fn on_section_change(&mut self, entity: T);
}
