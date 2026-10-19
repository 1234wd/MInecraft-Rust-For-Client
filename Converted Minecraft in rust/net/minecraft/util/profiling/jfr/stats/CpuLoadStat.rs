//! Port of: net/minecraft/util/profiling/jfr/stats/CpuLoadStat.java
//! Java class(es): net.minecraft.util.profiling.jfr.stats.CpuLoadStat
//! Status: VERIFIED

use crate::javacompat::java_jfr::{null_event, RecordedEvent};
use crate::javacompat::java_lang::float_to_string::double_to_string;
use crate::javacompat::java_object::{equals_f64, hash_f64, record_hash};
use crate::javacompat::throwable::ThrowableRef;

#[derive(Clone, Copy, Debug)]
pub struct CpuLoadStat {
    jvm: f64,
    user_jvm: f64,
    system: f64,
}

impl CpuLoadStat {
    pub const fn new(jvm: f64, user_jvm: f64, system: f64) -> Self {
        Self {
            jvm,
            user_jvm,
            system,
        }
    }

    pub fn from(event: Option<&RecordedEvent>) -> Result<Self, ThrowableRef> {
        let event = event.ok_or_else(|| null_event("getFloat"))?;
        Ok(Self::new(
            f64::from(event.get_float(Some("jvmSystem"))?),
            f64::from(event.get_float(Some("jvmUser"))?),
            f64::from(event.get_float(Some("machineTotal"))?),
        ))
    }

    pub const fn jvm(self) -> f64 {
        self.jvm
    }
    pub const fn user_jvm(self) -> f64 {
        self.user_jvm
    }
    pub const fn system(self) -> f64 {
        self.system
    }

    pub fn hash_code(self) -> i32 {
        record_hash(&[
            hash_f64(self.jvm),
            hash_f64(self.user_jvm),
            hash_f64(self.system),
        ])
    }
}

impl PartialEq for CpuLoadStat {
    fn eq(&self, other: &Self) -> bool {
        equals_f64(self.jvm, other.jvm)
            && equals_f64(self.user_jvm, other.user_jvm)
            && equals_f64(self.system, other.system)
    }
}

impl Eq for CpuLoadStat {}

impl std::fmt::Display for CpuLoadStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CpuLoadStat[jvm={}, userJvm={}, system={}]",
            double_to_string(self.jvm),
            double_to_string(self.user_jvm),
            double_to_string(self.system)
        )
    }
}
