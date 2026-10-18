//! Port of: net/minecraft/world/level/entity/ChunkStatusUpdateListener.java
//! Java class(es): net.minecraft.world.level.entity.ChunkStatusUpdateListener
//! Status: VERIFIED
//!
//! Session 22. `@FunctionalInterface public interface ChunkStatusUpdateListener` -- declaration only.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

use crate::net::minecraft::server::level::FullChunkStatus::FullChunkStatus;
use crate::net::minecraft::world::level::ChunkPos::ChunkPos;

/// Port of `net.minecraft.world.level.entity.ChunkStatusUpdateListener`.
pub trait ChunkStatusUpdateListener {
    /// Port of `void onChunkStatusChange(ChunkPos pos, FullChunkStatus chunkStatus)`.
    fn on_chunk_status_change(&mut self, pos: ChunkPos, chunk_status: FullChunkStatus);
}
