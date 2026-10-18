//! Port of: net/minecraft/client/renderer/block/model/BlockDisplayContext.java
//! Java class(es): net.minecraft.client.renderer.block.model.BlockDisplayContext
//! Status: VERIFIED
//!
//! Session 22. `public class BlockDisplayContext`: an opaque token passed through
//! `BlockModel.update(...)`. A private constructor and one factory, `create()`, which returns a new
//! object each call; no fields, no `equals`.
//!
//! # Identity is the whole class
//!
//! Each renderer that draws blocks holds its own `static final BLOCK_DISPLAY_CONTEXT =
//! BlockDisplayContext.create()`, and the `DisplayContext` select property hands the context on to
//! be matched -- by `Object.equals`, i.e. by identity. Two creates are not `equals`; a copied
//! reference is.
//!
//! A Rust value has no identity, and this type has no data. The first port was a zero-sized type
//! tested with `std::ptr::eq`, which Rust does not define for distinct zero-sized values: the
//! release build gave two creates the same address and the test failed where debug passed. So each
//! `create()` now takes a fresh number from a process-wide atomic counter, and the derived
//! `PartialEq`/`Hash` compare that number: two creates differ, and a copy is the same context, as a
//! copied Java reference is the same object. 2^64 creates do not wrap in practice.
//!
//! `hashCode` (the identity hash) is not ported -- OPEN_QUESTIONS #30; the derived `Hash` agrees
//! with the derived `Eq`, which is what a map needs. The field is private, so a context can only
//! come from [`BlockDisplayContext::create`]; the jar's constructor really is `private` (the probe
//! records its modifiers).
//!
//! Verified by the `BLOCKDISPCTX` group of `small_probe.txt` (`tests/session22.rs`).

use std::sync::atomic::{AtomicU64, Ordering};

/// Port of `net.minecraft.client.renderer.block.model.BlockDisplayContext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockDisplayContext {
    identity: u64,
}

impl BlockDisplayContext {
    /// Port of `public static BlockDisplayContext create()`: a new, distinct context.
    pub fn create() -> BlockDisplayContext {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        BlockDisplayContext {
            identity: NEXT.fetch_add(1, Ordering::Relaxed),
        }
    }
}
