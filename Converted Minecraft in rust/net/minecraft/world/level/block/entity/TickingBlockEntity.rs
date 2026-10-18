//! Port of: net/minecraft/world/level/block/entity/TickingBlockEntity.java
//! Java class(es): net.minecraft.world.level.block.entity.TickingBlockEntity
//! Status: VERIFIED
//!
//! Session 22. `public interface TickingBlockEntity` -- declaration only.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

use crate::net::minecraft::core::BlockPos::BlockPos;

/// Port of `net.minecraft.world.level.block.entity.TickingBlockEntity`.
pub trait TickingBlockEntity {
    /// Port of `void tick()`.
    fn tick(&mut self);

    /// Port of `boolean isRemoved()`.
    fn is_removed(&self) -> bool;

    /// Port of `BlockPos getPos()`.
    fn get_pos(&self) -> BlockPos;

    /// Port of `String getType()`.
    fn get_type(&self) -> String;
}
