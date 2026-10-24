//! Port of: net/minecraft/server/packs/resources/ReloadInstance.java
//! Java class(es): net.minecraft.server.packs.resources.ReloadInstance
//! Status: PORTED
//!
//! An interface with `done()` returning `CompletableFuture<?>`, `getActualProgress()`
//! returning float, and default methods `isDone()` and `checkExceptions()`.
//! External contracts:
//! - `java.util.concurrent.CompletableFuture<?>` -> `javacompat::java_concurrent::CompletableFuture` (modeled).
//!
//! `isDone()` delegates to `done().isDone()`. `checkExceptions()` delegates to
//! `done()`, checking `isCompletedExceptionally()` and calling `join()`.

use crate::javacompat::java_concurrent::CompletableFuture;

/// Port of `net.minecraft.server.packs.resources.ReloadInstance`.
pub trait ReloadInstance {
    /// Port of `CompletableFuture<?> done()`.
    fn done(&self) -> Box<dyn CompletableFuture<()>>;

    /// Port of `float getActualProgress()`.
    fn get_actual_progress(&self) -> f32;

    /// Port of `default boolean isDone()`: `this.done().isDone()`.
    fn is_done(&self) -> bool {
        self.done().is_done()
    }

    /// Port of `default void checkExceptions()`.
    ///
    /// If the returned future completed exceptionally, `join()` is called to surface
    /// the exception (matching Java's `CompletableFuture.join` throwing).
    fn check_exceptions(&self) {
        let done = self.done();
        if done.is_completed_exceptionally() {
            let _: () = done.join();
        }
    }
}
