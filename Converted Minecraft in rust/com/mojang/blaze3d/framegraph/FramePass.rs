//! Port of: com/mojang/blaze3d/framegraph/FramePass.java
//! Java class(es): com.mojang.blaze3d.framegraph.FramePass
//! Status: VERIFIED
//!
//! Session 26. This is an interface, so its entire contract is the six abstract methods checked
//! against the 26.2 jar by the `S26_API`/`S26_SUP` probe groups. The generic methods remain generic
//! in Rust. Shared Java references are represented by `Arc`; `executes` accepts a callable that
//! can be retained and invoked later when the frame graph runs. Resource payloads must be `'static`
//! because the returned shared handles own them. No default or guessed behavior is supplied here.

use std::sync::Arc;

use crate::com::mojang::blaze3d::resource::ResourceDescriptor::ResourceDescriptor;
use crate::com::mojang::blaze3d::resource::ResourceHandle::ResourceHandle;

/// Port of `com.mojang.blaze3d.framegraph.FramePass`.
pub trait FramePass {
    /// Port of `<T> ResourceHandle<T> createsInternal(String, ResourceDescriptor<T>)`.
    fn creates_internal<T: 'static>(
        &mut self,
        name: String,
        descriptor: Arc<dyn ResourceDescriptor<T>>,
    ) -> Arc<dyn ResourceHandle<T>>;

    /// Port of `<T> void reads(ResourceHandle<T>)`.
    fn reads<T: 'static>(&mut self, handle: Arc<dyn ResourceHandle<T>>);

    /// Port of `<T> ResourceHandle<T> readsAndWrites(ResourceHandle<T>)`.
    fn reads_and_writes<T: 'static>(
        &mut self,
        handle: Arc<dyn ResourceHandle<T>>,
    ) -> Arc<dyn ResourceHandle<T>>;

    /// Port of `void requires(FramePass)`.
    fn requires<P: FramePass>(&mut self, pass: Arc<P>);

    /// Port of `void disableCulling()`.
    fn disable_culling(&mut self);

    /// Port of `void executes(Runnable)`. `Fn` preserves Runnable's ability to be run repeatedly.
    fn executes(&mut self, task: Arc<dyn Fn()>);
}
