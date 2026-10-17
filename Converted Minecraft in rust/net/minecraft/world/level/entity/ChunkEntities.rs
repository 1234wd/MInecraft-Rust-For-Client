//! Port of: net/minecraft/world/level/entity/ChunkEntities.java
//! Java class(es): net.minecraft.world.level.entity.ChunkEntities
//! Status: VERIFIED
//!
//! Session 22. `public class ChunkEntities<T>`: the entities stored for one chunk -- a `ChunkPos`
//! and a `List<T>`.
//!
//! `getEntities()` returns a `Stream<T>` over the list, in list order; here it is an iterator over
//! borrowed entities. `isEmpty()` is the list's. The class holds the list it was given (no copy),
//! which in Rust means it takes ownership of the `Vec`.
//!
//! Verified by the `CHUNKENT` group of `small_probe.txt` (`tests/session22.rs`).

use crate::net::minecraft::world::level::ChunkPos::ChunkPos;

/// Port of `net.minecraft.world.level.entity.ChunkEntities<T>`.
#[derive(Debug, Clone)]
pub struct ChunkEntities<T> {
    pos: ChunkPos,
    entities: Vec<T>,
}

impl<T> ChunkEntities<T> {
    /// Port of `new ChunkEntities(ChunkPos pos, List<T> entities)`.
    pub fn new(pos: ChunkPos, entities: Vec<T>) -> Self {
        ChunkEntities { pos, entities }
    }

    /// Port of `getPos()`.
    pub fn get_pos(&self) -> ChunkPos {
        self.pos
    }

    /// Port of `getEntities()`: the entities in list order.
    pub fn get_entities(&self) -> impl Iterator<Item = &T> + '_ {
        self.entities.iter()
    }

    /// Port of `isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}
