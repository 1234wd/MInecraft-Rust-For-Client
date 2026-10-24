//! Port of: net/minecraft/client/resources/server/PackReloadConfig.java
//! Java class(es): net.minecraft.client.resources.server.PackReloadConfig, net.minecraft.client.resources.server.PackReloadConfig.Callbacks, net.minecraft.client.resources.server.PackReloadConfig.IdAndPath
//! Status: PORTED
//!
//! An interface with a nested `Callbacks` interface and an `IdAndPath` record.
//! External contracts:
//! - `java.util.UUID` -> `javacompat::java_util::Uuid` (modeled).
//! - `java.nio.file.Path` -> `javacompat::java_nio::Path` (modeled).
//! - `java.util.List<E>` -> `javacompat::java_concurrent::List` (modeled).
//!
//! `scheduleReload(Callbacks)` is the single method on the outer interface.
//! `Callbacks` declares `onSuccess()`, `onFailure(boolean)`, and `List<IdAndPath> packsToLoad()`.
//! `IdAndPath` is a record with `UUID id` and `Path path` components.

use crate::javacompat::java_concurrent::List;
use crate::javacompat::java_nio::Path;
use crate::javacompat::java_util::Uuid;

/// Port of `net.minecraft.client.resources.server.PackReloadConfig`.
pub trait PackReloadConfig {
    /// Port of `void scheduleReload(PackReloadConfig.Callbacks callbacks)`.
    fn schedule_reload(&self, callbacks: &dyn Callbacks);
}

/// Port of `PackReloadConfig.Callbacks`.
pub trait Callbacks {
    /// Port of `void onSuccess()`.
    fn on_success(&self);

    /// Port of `void onFailure(boolean isRecovery)`.
    fn on_failure(&self, is_recovery: bool);

    /// Port of `List<PackReloadConfig.IdAndPath> packsToLoad()`.
    fn packs_to_load(&self) -> &dyn List<IdAndPath>;
}

/// Port of `PackReloadConfig.IdAndPath`, a record with `UUID id` and `Path path`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IdAndPath {
    /// Port of the `id` component.
    pub id: Uuid,
    /// Port of the `path` component.
    pub path: Path,
}
