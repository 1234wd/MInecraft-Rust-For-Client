//! Port of: net/minecraft/util/profiling/jfr/stats/ChunkIdentification.java
//! Java class(es): net.minecraft.util.profiling.jfr.stats.ChunkIdentification
//! Status: VERIFIED

use crate::javacompat::java_jfr::{null_event, RecordedEvent};
use crate::javacompat::java_lang::string_hash_code;
use crate::javacompat::java_object::{record_hash, str_or_null};
use crate::javacompat::throwable::ThrowableRef;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChunkIdentification {
    level: Option<String>,
    dimension: Option<String>,
    x: i32,
    z: i32,
}

impl ChunkIdentification {
    pub fn new(level: Option<String>, dimension: Option<String>, x: i32, z: i32) -> Self {
        Self {
            level,
            dimension,
            x,
            z,
        }
    }

    pub fn from(event: Option<&RecordedEvent>) -> Result<Self, ThrowableRef> {
        let event = event.ok_or_else(|| null_event("getString"))?;
        Ok(Self::new(
            event.get_string(Some("level"))?,
            event.get_string(Some("dimension"))?,
            event.get_int(Some("chunkPosX"))?,
            event.get_int(Some("chunkPosZ"))?,
        ))
    }

    pub fn level(&self) -> Option<&str> {
        self.level.as_deref()
    }
    pub fn dimension(&self) -> Option<&str> {
        self.dimension.as_deref()
    }
    pub const fn x(&self) -> i32 {
        self.x
    }
    pub const fn z(&self) -> i32 {
        self.z
    }

    pub fn hash_code(&self) -> i32 {
        record_hash(&[
            self.level.as_deref().map_or(0, string_hash_code),
            self.dimension.as_deref().map_or(0, string_hash_code),
            self.x,
            self.z,
        ])
    }
}

impl std::fmt::Display for ChunkIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ChunkIdentification[level={}, dimension={}, x={}, z={}]",
            str_or_null(self.level.as_deref()),
            str_or_null(self.dimension.as_deref()),
            self.x,
            self.z
        )
    }
}
