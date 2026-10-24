//! Port of: net/minecraft/util/SignatureUpdater.java
//! Java class(es): net.minecraft.util.SignatureUpdater, net.minecraft.util.SignatureUpdater.Output
//! Status: PORTED
//!
//! A `@FunctionalInterface` callback: `update(Output)` declares `throws SignatureException`, so
//! the Rust methods return `Result<(), SignatureException>` -- a `throws` method is a
//! call-and-continue contract (callers catch it), hence `Result` rather than a panic, per
//! `DESIGN_DECISIONS.md#panic-not-Result`.
//!
//! `SignatureException` lives in `javacompat::throwable` (no Java counterpart in
//! `minecraft-decompiled/`), and its `Display` is `java.security.SignatureException: <message>`.

use crate::javacompat::throwable::SignatureException;

/// Port of `net.minecraft.util.SignatureUpdater`.
pub trait SignatureUpdater {
    /// Port of `void update(SignatureUpdater.Output output) throws SignatureException`.
    fn update(&self, output: &dyn Output) -> Result<(), SignatureException>;
}

/// Port of `SignatureUpdater.Output`, a nested `@FunctionalInterface`.
pub trait Output {
    /// Port of `void update(byte[] payload) throws SignatureException`.
    fn update(&self, payload: &[u8]) -> Result<(), SignatureException>;
}
