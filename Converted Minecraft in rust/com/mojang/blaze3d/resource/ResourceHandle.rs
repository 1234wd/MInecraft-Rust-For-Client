//! Port of: com/mojang/blaze3d/resource/ResourceHandle.java
//! Java class(es): com.mojang.blaze3d.resource.ResourceHandle
//! Status: VERIFIED
//!
//! Session 22. `public interface ResourceHandle<T>`: a handle to a frame-graph resource, with one
//! method, `get()`, and a shared `INVALID_HANDLE` whose `get()` throws
//! `IllegalStateException("Cannot dereference handle with no underlying resource")`.
//!
//! `invalid()` returns `INVALID_HANDLE` itself (unchecked-cast to any `T`), so every invalid handle
//! is the same object. Here `INVALID_HANDLE` is a unit struct that is a `ResourceHandle<T>` for
//! every `T`, and [`invalid`] returns it. `get` returns a borrow, as Java returns the resource
//! object without copying it.
//!
//! Verified by the `RESHANDLE` group of `small_probe.txt` (`tests/session22.rs`): the exception text,
//! and that `invalid()` is `INVALID_HANDLE`.

/// Port of `com.mojang.blaze3d.resource.ResourceHandle<T>`.
pub trait ResourceHandle<T> {
    /// Port of `T get()`.
    fn get(&self) -> &T;
}

/// The type of `ResourceHandle.INVALID_HANDLE`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InvalidHandle;

impl<T> ResourceHandle<T> for InvalidHandle {
    /// Throws `IllegalStateException`: there is no resource to dereference.
    fn get(&self) -> &T {
        panic!("java.lang.IllegalStateException: Cannot dereference handle with no underlying resource")
    }
}

/// Port of `ResourceHandle.INVALID_HANDLE`.
pub const INVALID_HANDLE: InvalidHandle = InvalidHandle;

/// Port of `static <T> ResourceHandle<T> invalid()`: `INVALID_HANDLE`.
pub const fn invalid() -> InvalidHandle {
    INVALID_HANDLE
}
