//! Port of: net/minecraft/world/level/entity/UniquelyIdentifyable.java
//! Java class(es): net.minecraft.world.level.entity.UniquelyIdentifyable
//! Status: VERIFIED

use crate::javacompat::java_util::Uuid;

/// The two-member identity contract implemented by entities in this package.
pub trait UniquelyIdentifyable {
    /// `getUUID()`; Java reference results may be null.
    fn get_uuid(&self) -> Option<Uuid>;

    /// `isRemoved()`.
    fn is_removed(&self) -> bool;
}
