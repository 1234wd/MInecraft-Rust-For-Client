//! Port of: net/minecraft/world/level/storage/LevelResource.java
//! Java class(es): net.minecraft.world.level.storage.LevelResource
//! Status: VERIFIED
//!
//! Session 22. `public record LevelResource(String id)`: the relative paths of the files and
//! folders inside a world save, as thirteen constants, with an overridden `toString`.
//!
//! These ids are on disk: `level.dat`, `level.dat_old`, `session.lock`, `players/data` ... are the
//! names a save folder actually contains, so a changed character is a world that will not load.
//! Every constant is checked against the jar.
//!
//! # Port shape
//!
//! The id is a `Cow<'static, str>` so the constants can be `const` (borrowed literals) while
//! [`LevelResource::new`] still takes any string (owned). `equals`/`hashCode` see only the text.
//!
//! `toString` is OVERRIDDEN: `"/" + id` -- so `ROOT` (`"."`) prints `/.` and an empty id prints `/`.
//! `hashCode` is the record's, `id.hashCode()` for one component.
//!
//! The id is NON-NULL here, because a Rust string cannot be null. Java would accept one (printing
//! `/null`, hashing `0`); a caller that passes null would need `Option`.
//!
//! Verified by the `REC`, `RECEQ` and `LEVELRES` groups of `small_probe.txt`
//! (`tests/session22.rs`): every constant, a fresh `new LevelResource("level.dat")` that must equal
//! `LEVEL_DATA_FILE`, and an empty id.

use std::borrow::Cow;

use crate::javacompat::java_lang;

/// Port of `net.minecraft.world.level.storage.LevelResource`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelResource {
    id: Cow<'static, str>,
}

impl LevelResource {
    /// Port of `LevelResource.PLAYER_ADVANCEMENTS_DIR`.
    pub const PLAYER_ADVANCEMENTS_DIR: LevelResource =
        LevelResource::constant("players/advancements");
    /// Port of `LevelResource.PLAYER_STATS_DIR`.
    pub const PLAYER_STATS_DIR: LevelResource = LevelResource::constant("players/stats");
    /// Port of `LevelResource.PLAYER_DATA_DIR`.
    pub const PLAYER_DATA_DIR: LevelResource = LevelResource::constant("players/data");
    /// Port of `LevelResource.PLAYER_OLD_DATA_DIR`.
    pub const PLAYER_OLD_DATA_DIR: LevelResource = LevelResource::constant("players");
    /// Port of `LevelResource.LEVEL_DATA_FILE`.
    pub const LEVEL_DATA_FILE: LevelResource = LevelResource::constant("level.dat");
    /// Port of `LevelResource.OLD_LEVEL_DATA_FILE`.
    pub const OLD_LEVEL_DATA_FILE: LevelResource = LevelResource::constant("level.dat_old");
    /// Port of `LevelResource.ICON_FILE`.
    pub const ICON_FILE: LevelResource = LevelResource::constant("icon.png");
    /// Port of `LevelResource.LOCK_FILE`.
    pub const LOCK_FILE: LevelResource = LevelResource::constant("session.lock");
    /// Port of `LevelResource.GENERATED_DIR`.
    pub const GENERATED_DIR: LevelResource = LevelResource::constant("generated");
    /// Port of `LevelResource.DATAPACK_DIR`.
    pub const DATAPACK_DIR: LevelResource = LevelResource::constant("datapacks");
    /// Port of `LevelResource.MAP_RESOURCE_FILE`.
    pub const MAP_RESOURCE_FILE: LevelResource =
        LevelResource::constant("resourcepacks/resources.zip");
    /// Port of `LevelResource.DATA`.
    pub const DATA: LevelResource = LevelResource::constant("data");
    /// Port of `LevelResource.ROOT`.
    pub const ROOT: LevelResource = LevelResource::constant(".");

    const fn constant(id: &'static str) -> Self {
        LevelResource {
            id: Cow::Borrowed(id),
        }
    }

    /// Port of the canonical constructor.
    pub fn new(id: &str) -> Self {
        LevelResource {
            id: Cow::Owned(id.to_string()),
        }
    }

    /// Port of the accessor `id()`.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Port of the record's generated `hashCode()`: `id.hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_lang::string_hash_code(&self.id)
    }
}

impl std::fmt::Display for LevelResource {
    /// Port of the `toString()` override: `"/" + this.id`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}", self.id)
    }
}
