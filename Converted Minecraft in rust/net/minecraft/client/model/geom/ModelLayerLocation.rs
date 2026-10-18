//! Port of: net/minecraft/client/model/geom/ModelLayerLocation.java
//! Java class(es): net.minecraft.client.model.geom.ModelLayerLocation
//! Status: VERIFIED
//!
//! Session 22. `public record ModelLayerLocation(Identifier model, String layer)` with an
//! overridden `toString`.
//!
//! `toString` is OVERRIDDEN: `model + "#" + layer`, e.g. `minecraft:player#main` -- not the record
//! form. `equals` is the record's (derived: `Identifier#equals` and a case-sensitive string
//! compare), and `hashCode` is the record fold `31 * model.hashCode() + layer.hashCode()`.
//!
//! The `layer` component is NON-NULL here, because a Rust `String` cannot be null. Java would
//! accept one (printing `minecraft:player#null`, hashing it as `0`); a caller that passes null would
//! need it to become `Option<String>`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`), including
//! an empty layer and two layers differing only in case.

use crate::javacompat::java_lang;
use crate::javacompat::java_object;
use crate::net::minecraft::resources::Identifier::Identifier;

/// Port of `net.minecraft.client.model.geom.ModelLayerLocation`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelLayerLocation {
    model: Identifier,
    layer: String,
}

impl ModelLayerLocation {
    /// Port of the canonical constructor.
    pub fn new(model: Identifier, layer: &str) -> Self {
        ModelLayerLocation {
            model,
            layer: layer.to_string(),
        }
    }

    /// Port of the accessor `model()`.
    pub fn model(&self) -> &Identifier {
        &self.model
    }

    /// Port of the accessor `layer()`.
    pub fn layer(&self) -> &str {
        &self.layer
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            self.model.java_hash_code(),
            java_lang::string_hash_code(&self.layer),
        ])
    }
}

impl std::fmt::Display for ModelLayerLocation {
    /// Port of the `toString()` override: `this.model + "#" + this.layer`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}#{}", self.model, self.layer)
    }
}
