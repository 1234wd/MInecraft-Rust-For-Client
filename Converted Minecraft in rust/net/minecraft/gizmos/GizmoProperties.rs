//! Port of: net/minecraft/gizmos/GizmoProperties.java
//! Java class(es): net.minecraft.gizmos.GizmoProperties
//! Status: VERIFIED
//!
//! Session 22. `public interface GizmoProperties` -- declaration only.
//!
//! Each method returns a `GizmoProperties` for chaining; here that is `&mut dyn GizmoProperties`,
//! which keeps the trait object-safe.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `net.minecraft.gizmos.GizmoProperties`.
pub trait GizmoProperties {
    /// Port of `GizmoProperties setAlwaysOnTop()`.
    fn set_always_on_top(&mut self) -> &mut dyn GizmoProperties;

    /// Port of `GizmoProperties persistForMillis(int milliseconds)`.
    fn persist_for_millis(&mut self, milliseconds: i32) -> &mut dyn GizmoProperties;

    /// Port of `GizmoProperties fadeOut()`.
    fn fade_out(&mut self) -> &mut dyn GizmoProperties;
}
