//! Port of: net/minecraft/util/profiling/jfr/stats/FpsStat.java
//! Java class(es): net.minecraft.util.profiling.jfr.stats.FpsStat
//! Status: VERIFIED

use crate::javacompat::java_jfr::{null_event, RecordedEvent};
use crate::javacompat::java_object::record_hash;
use crate::javacompat::throwable::ThrowableRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FpsStat {
    fps: i32,
}

impl FpsStat {
    pub const fn new(fps: i32) -> Self {
        Self { fps }
    }

    pub fn from(event: Option<&RecordedEvent>, field: Option<&str>) -> Result<Self, ThrowableRef> {
        let event = event.ok_or_else(|| null_event("getInt"))?;
        Ok(Self::new(event.get_int(field)?))
    }

    pub const fn fps(self) -> i32 {
        self.fps
    }
    pub fn hash_code(self) -> i32 {
        record_hash(&[self.fps])
    }
}

impl std::fmt::Display for FpsStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FpsStat[fps={}]", self.fps)
    }
}
