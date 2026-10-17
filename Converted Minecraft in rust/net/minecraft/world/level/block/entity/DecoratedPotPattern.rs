//! Port of: net/minecraft/world/level/block/entity/DecoratedPotPattern.java
//! Java class(es): net.minecraft.world.level.block.entity.DecoratedPotPattern
//! Status: VERIFIED
//!
//! Session 22. `public record DecoratedPotPattern(Identifier assetId)` -- one component, no body.
//!
//! `equals` is `Identifier#equals` (derived, namespace and path); `hashCode` is
//! [`DecoratedPotPattern::java_hash_code`], which for a one-component record is the component's
//! own hash, `Identifier#hashCode`; `toString` is `DecoratedPotPattern[assetId=<ns>:<path>]`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`), including
//! an identifier built by `parse` that must equal one built by `withDefaultNamespace`.

use crate::javacompat::java_object;
use crate::net::minecraft::resources::Identifier::Identifier;

/// Port of `net.minecraft.world.level.block.entity.DecoratedPotPattern`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecoratedPotPattern {
    asset_id: Identifier,
}

impl DecoratedPotPattern {
    /// Port of the canonical constructor.
    pub fn new(asset_id: Identifier) -> Self {
        DecoratedPotPattern { asset_id }
    }

    /// Port of the accessor `assetId()`.
    pub fn asset_id(&self) -> &Identifier {
        &self.asset_id
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[self.asset_id.java_hash_code()])
    }
}

impl std::fmt::Display for DecoratedPotPattern {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DecoratedPotPattern[assetId={}]", self.asset_id)
    }
}
