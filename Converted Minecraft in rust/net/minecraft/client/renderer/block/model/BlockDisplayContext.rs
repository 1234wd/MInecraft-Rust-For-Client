//! Port of: net/minecraft/client/renderer/block/model/BlockDisplayContext.java
//! Java class(es): net.minecraft.client.renderer.block.model.BlockDisplayContext
//! Status: VERIFIED
//!
//! Session 22. `public class BlockDisplayContext`: an opaque token passed through
//! `BlockModel.update(...)`. A private constructor and one factory, `create()`, which returns a new
//! object each call; no fields, no `equals`.
//!
//! Each `create()` is a distinct object and two are not `equals` (identity); the port therefore has
//! a private field so it can only be built through [`BlockDisplayContext::create`], and derives no
//! `PartialEq`. The jar's constructor really is `private` (the probe records its modifiers).
//!
//! Verified by the `BLOCKDISPCTX` group of `small_probe.txt` (`tests/session22.rs`).

/// Port of `net.minecraft.client.renderer.block.model.BlockDisplayContext`.
#[derive(Debug)]
pub struct BlockDisplayContext {
    _private: (),
}

impl BlockDisplayContext {
    /// Port of `public static BlockDisplayContext create()`: a new, distinct context.
    pub fn create() -> BlockDisplayContext {
        BlockDisplayContext { _private: () }
    }
}
