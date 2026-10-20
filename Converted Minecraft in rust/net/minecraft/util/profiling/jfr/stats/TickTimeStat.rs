//! Port of: `net/minecraft/util/profiling/jfr/stats/TickTimeStat.java`
//! Java class(es): `net.minecraft.util.profiling.jfr.stats.TickTimeStat`
//! Status: VERIFIED
//!
//! The record components may be null at runtime even though its package is `@NullMarked`;
//! Java's record constructor does not enforce annotations. `Option` preserves those values and
//! the generated record methods' null behavior. The time value helpers implement the exact JDK
//! 25 value, hash, comparison, and formatting operations used by this record.

use crate::javacompat::java_jfr::{null_event_no_args, RecordedEvent};
use crate::javacompat::java_time::{Duration, Instant};
use crate::javacompat::throwable::ThrowableRef;

/// Port of `net.minecraft.util.profiling.jfr.stats.TickTimeStat`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TickTimeStat {
    timestamp: Option<Instant>,
    current_average: Option<Duration>,
}

impl TickTimeStat {
    /// Port of the public record canonical constructor.
    pub const fn new(timestamp: Option<Instant>, current_average: Option<Duration>) -> Self {
        Self {
            timestamp,
            current_average,
        }
    }

    /// Port of the record component accessor `timestamp()`.
    pub const fn timestamp(&self) -> Option<Instant> {
        self.timestamp
    }

    /// Port of the record component accessor `currentAverage()`.
    pub const fn current_average(&self) -> Option<Duration> {
        self.current_average
    }

    /// Port of the generated record `equals(Object)` for another TickTimeStat value.
    pub fn equals(&self, other: &TickTimeStat) -> bool {
        self == other
    }

    /// Port of `TickTimeStat.from(RecordedEvent)`. Java evaluates `getStartTime()` before the
    /// named duration lookup, and a null event fails at that first dereference.
    pub fn from(event: Option<&RecordedEvent>) -> Result<Self, ThrowableRef> {
        let Some(event) = event else {
            return Err(null_event_no_args("getStartTime"));
        };
        let timestamp = event.get_start_time();
        let current_average = event.get_duration(Some("averageTickDuration"))?;
        Ok(Self::new(Some(timestamp), Some(current_average)))
    }

    /// Java record `hashCode`: `31 * timestamp.hashCode() + currentAverage.hashCode()`,
    /// treating each null component as zero.
    pub fn hash_code(&self) -> i32 {
        let timestamp_hash = self.timestamp.map_or(0, |value| value.java_hash_code());
        let average_hash = self
            .current_average
            .map_or(0, |value| value.java_hash_code());
        timestamp_hash.wrapping_mul(31).wrapping_add(average_hash)
    }

    /// Java record `toString()` spelling and component order.
    pub fn java_to_string(&self) -> String {
        let timestamp = self
            .timestamp
            .map_or_else(|| "null".to_string(), |value| value.java_to_string());
        let current_average = self
            .current_average
            .map_or_else(|| "null".to_string(), |value| value.java_to_string());
        format!("TickTimeStat[timestamp={timestamp}, currentAverage={current_average}]")
    }
}

impl std::fmt::Display for TickTimeStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.java_to_string())
    }
}
