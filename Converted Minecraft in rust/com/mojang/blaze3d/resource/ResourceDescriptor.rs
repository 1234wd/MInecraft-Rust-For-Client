//! Port of: com/mojang/blaze3d/resource/ResourceDescriptor.java
//! Java class(es): com.mojang.blaze3d.resource.ResourceDescriptor
//! Status: VERIFIED
//!
//! Session 22. `public interface ResourceDescriptor<T>`: describes a frame-graph resource well
//! enough to allocate, prepare and free one. Two abstract methods and two defaults.
//!
//! * `prepare(resource)` does nothing by default.
//! * `canUsePhysicalResource(other)` is `this.equals(other)`. Implementors are records, so that is
//!   VALUE equality within one descriptor type and `false` across types (a record's `equals` checks
//!   the class first) -- both probed with two record implementations. The default here therefore
//!   downcasts `other` to `Self` and compares with `PartialEq`, and is only callable when `Self` is
//!   `PartialEq + 'static`.
//!
//! Verified by the `RESDESC` group of `small_probe.txt` (`tests/session22.rs`).

use std::any::Any;

/// Port of `com.mojang.blaze3d.resource.ResourceDescriptor<T>`.
pub trait ResourceDescriptor<T> {
    /// Port of `T allocate()`.
    fn allocate(&self) -> T;

    /// Port of `default void prepare(T resource)`: nothing.
    fn prepare(&self, _resource: &T) {}

    /// Port of `void free(T resource)`.
    fn free(&self, resource: T);

    /// Port of `default boolean canUsePhysicalResource(ResourceDescriptor<?> other)`: `this.equals(other)`.
    fn can_use_physical_resource(&self, other: &dyn Any) -> bool
    where
        Self: PartialEq + Sized + 'static,
    {
        other.downcast_ref::<Self>().is_some_and(|o| self == o)
    }
}
