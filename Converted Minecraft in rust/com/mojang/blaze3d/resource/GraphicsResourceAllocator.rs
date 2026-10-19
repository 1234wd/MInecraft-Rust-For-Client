//! Port of: com/mojang/blaze3d/resource/GraphicsResourceAllocator.java
//! Java class(es): com.mojang.blaze3d.resource.GraphicsResourceAllocator, GraphicsResourceAllocator::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 24. `public interface GraphicsResourceAllocator`: hands out GPU resources for a
//! [`ResourceDescriptor`] and takes them back. Two generic abstract methods, and the constant
//! `UNPOOLED`, an anonymous implementation that pools nothing:
//!
//! ```java
//! public <T> T acquire(ResourceDescriptor<T> descriptor) {
//!     T resource = descriptor.allocate();
//!     descriptor.prepare(resource);
//!     return resource;
//! }
//! public <T> void release(ResourceDescriptor<T> descriptor, T resource) { descriptor.free(resource); }
//! ```
//!
//! The anonymous class is the unit struct [`Unpooled`]; [`UNPOOLED`] is its one value. The
//! methods are generic, as in Java, so the trait is used through generics (`CrossFrameResourcePool`,
//! its other implementation, is not ported yet).
//!
//! Verified by the `GRALLOC` group of `sweep_probe.txt` (`tests/session24.rs`): the call sequence
//! `UNPOOLED` makes on a recording descriptor, plus the `API24` reflection shape.

use crate::com::mojang::blaze3d::resource::ResourceDescriptor::ResourceDescriptor;

/// Port of `com.mojang.blaze3d.resource.GraphicsResourceAllocator`.
pub trait GraphicsResourceAllocator {
    /// Port of `<T> T acquire(ResourceDescriptor<T> descriptor)`.
    fn acquire<T>(&self, descriptor: &dyn ResourceDescriptor<T>) -> T;

    /// Port of `<T> void release(ResourceDescriptor<T> descriptor, T resource)`.
    fn release<T>(&self, descriptor: &dyn ResourceDescriptor<T>, resource: T);
}

/// The anonymous class behind `GraphicsResourceAllocator.UNPOOLED`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unpooled;

/// Port of `GraphicsResourceAllocator UNPOOLED`.
pub const UNPOOLED: Unpooled = Unpooled;

impl GraphicsResourceAllocator for Unpooled {
    /// `descriptor.allocate()`, then `descriptor.prepare(resource)`, then the resource.
    fn acquire<T>(&self, descriptor: &dyn ResourceDescriptor<T>) -> T {
        let resource = descriptor.allocate();
        descriptor.prepare(&resource);
        resource
    }

    /// `descriptor.free(resource)`.
    fn release<T>(&self, descriptor: &dyn ResourceDescriptor<T>, resource: T) {
        descriptor.free(resource);
    }
}
