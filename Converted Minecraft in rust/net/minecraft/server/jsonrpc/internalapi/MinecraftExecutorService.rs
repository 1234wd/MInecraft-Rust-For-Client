//! Port of: net/minecraft/server/jsonrpc/internalapi/MinecraftExecutorService.java
//! Java class(es): net.minecraft.server.jsonrpc.internalapi.MinecraftExecutorService
//! Status: PORTED
//!
//! An interface with two `submit` overloads returning `CompletableFuture<V>` /
//! `CompletableFuture<Void>`. External contracts:
//! - `java.util.concurrent.CompletableFuture<V>` / `CompletableFuture<Void>` -> `javacompat::java_concurrent::CompletableFuture` (modeled).
//! - `java.util.function.Supplier<V>` -> `javacompat::java_concurrent::Supplier` (modeled).
//!
//! Nothing is implemented here beyond the trait declaration; implementors provide the
//! actual future. The trait is the faithful mirror of the Java interface.

use crate::javacompat::java_concurrent::{CompletableFuture, Supplier};

/// Port of `net.minecraft.server.jsonrpc.internalapi.MinecraftExecutorService`.
pub trait MinecraftExecutorService {
    /// Port of `<V> CompletableFuture<V> submit(Supplier<V> supplier)`.
    fn submit<V>(&self, supplier: &dyn Supplier<V>) -> Box<dyn CompletableFuture<V>>;

    /// Port of `CompletableFuture<Void> submit(Runnable runnable)`.
    fn submit_runnable(&self, runnable: &dyn Fn()) -> Box<dyn CompletableFuture<()>>;
}
