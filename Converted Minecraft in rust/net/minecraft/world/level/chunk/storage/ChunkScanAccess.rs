//! Port of: net/minecraft/world/level/chunk/storage/ChunkScanAccess.java
//! Java class(es): net.minecraft.world.level.chunk.storage.ChunkScanAccess
//! Status: PORTED
//!
//! An interface with a single method `scanChunk(ChunkPos, StreamTagVisitor)` returning
//! `CompletableFuture<Void>`. External contracts:
//! - `java.util.concurrent.CompletableFuture<Void>` -> `javacompat::java_concurrent::CompletableFuture` (modeled).
//! - `net.minecraft.nbt.StreamTagVisitor` -> already ported (session 23).
//! - `net.minecraft.world.level.ChunkPos` -> already ported (session 16).
//!
//! Nothing is implemented here beyond the trait declaration; implementors provide the
//! actual future. The trait is the faithful mirror of the Java interface.

use crate::javacompat::java_concurrent::CompletableFuture;
use crate::net::minecraft::nbt::StreamTagVisitor::StreamTagVisitor;
use crate::net::minecraft::world::level::ChunkPos::ChunkPos;

/// Port of `net.minecraft.world.level.chunk.storage.ChunkScanAccess`.
pub trait ChunkScanAccess {
    /// Port of `CompletableFuture<Void> scanChunk(ChunkPos pos, StreamTagVisitor visitor)`.
    fn scan_chunk(&self, pos: &ChunkPos, visitor: &dyn StreamTagVisitor) -> Box<dyn CompletableFuture<()>>;
}
