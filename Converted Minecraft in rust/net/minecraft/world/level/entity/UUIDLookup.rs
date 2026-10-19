//! Port of: net/minecraft/world/level/entity/UUIDLookup.java
//! Java class(es): net.minecraft.world.level.entity.UUIDLookup
//! Status: VERIFIED

use crate::javacompat::java_util::Uuid;
use crate::net::minecraft::world::level::entity::UniquelyIdentifyable::UniquelyIdentifyable;

/// Generic nullable lookup contract over Java UUID values.
pub trait UUIDLookup<IdentifiedType: UniquelyIdentifyable> {
    fn lookup(&self, uuid: Uuid) -> Option<IdentifiedType>;
}
