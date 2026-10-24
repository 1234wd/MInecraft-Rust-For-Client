//! Port of: net/minecraft/client/resources/server/PackLoadFeedback.java
//! Java class(es): net.minecraft.client.resources.server.PackLoadFeedback, net.minecraft.client.resources.server.FinalResult, net.minecraft.client.resources.server.Update
//! Status: PORTED
//!
//! A callback interface: the game calls `reportUpdate`/`reportFinalResult` with a
//! `java.util.UUID` pack id and a result enum. The interface itself carries no state and no
//! default implementation -- behavior lives in implementors, so there is nothing to
//! oracle-test here. Proof is the declared API shape matching the Java interface (the two
//! methods, the two nested enums, and the `UUID` parameter type).
//!
//! `java.util.UUID` is the narrow value subset in `javacompat::java_util::Uuid` (two signed
//! 64-bit fields), sufficient because this file only passes `UUID` by value -- it never
//! calls a `UUID` method. See `_porting/javacompat/java_util.rs` for the scoped contract.

use crate::javacompat::java_util::Uuid;

/// Port of `net.minecraft.client.resources.server.PackLoadFeedback`.
pub trait PackLoadFeedback {
    /// Port of `void reportUpdate(UUID id, Update result)`.
    fn report_update(&self, id: Uuid, result: Update);

    /// Port of `void reportFinalResult(UUID id, FinalResult result)`.
    fn report_final_result(&self, id: Uuid, result: FinalResult);
}

/// Port of `PackLoadFeedback.FinalResult`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FinalResult {
    Declined,
    Applied,
    Discarded,
    DownloadFailed,
    ActivationFailed,
}

/// Port of `PackLoadFeedback.Update`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Update {
    Accepted,
    Downloaded,
}
