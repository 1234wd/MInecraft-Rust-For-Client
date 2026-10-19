//! Port of: net/minecraft/util/profiling/metrics/storage/RecordedDeviation.java
//! Java class(es): net.minecraft.util.profiling.metrics.storage.RecordedDeviation
//! Status: VERIFIED
//!
//! Session 24. `public final class RecordedDeviation`: a tick whose duration deviated, kept by the
//! metrics recorder -- when it happened, which tick, and the profiler's results for that tick.
//! Three `public final` fields set by the one constructor; no methods, and `equals`, `hashCode`
//! and `toString` are `Object`'s (identity), so the port has no `PartialEq`/`Display`.
//!
//! * `Instant timestamp` is [`javacompat::java_time::Instant`](crate::javacompat::java_time::Instant),
//!   a value type, as `java.time.Instant` is;
//! * `ProfileResults profilerResultAtTick` is the SAME results object the caller passes (the
//!   recorder keeps it and later saves it), so it is shared: `Arc<dyn ProfileResults>`.
//!
//! Verified by the `RECDEV` group of `sweep_probe.txt` (`tests/session24.rs`), together with the
//! `Instant` model it holds (`INSTANT`).

use std::sync::Arc;

use crate::javacompat::java_time::Instant;
use crate::net::minecraft::util::profiling::ProfileResults::ProfileResults;

/// Port of `net.minecraft.util.profiling.metrics.storage.RecordedDeviation`.
#[derive(Clone)]
pub struct RecordedDeviation {
    /// Port of `public final Instant timestamp`.
    pub timestamp: Instant,
    /// Port of `public final int tick`.
    pub tick: i32,
    /// Port of `public final ProfileResults profilerResultAtTick`.
    pub profiler_result_at_tick: Arc<dyn ProfileResults>,
}

impl RecordedDeviation {
    /// Port of `public RecordedDeviation(Instant timestamp, int tick, ProfileResults profilerResultAtTick)`.
    pub fn new(
        timestamp: Instant,
        tick: i32,
        profiler_result_at_tick: Arc<dyn ProfileResults>,
    ) -> Self {
        RecordedDeviation {
            timestamp,
            tick,
            profiler_result_at_tick,
        }
    }
}

impl std::fmt::Debug for RecordedDeviation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecordedDeviation")
            .field("timestamp", &self.timestamp)
            .field("tick", &self.tick)
            .finish_non_exhaustive()
    }
}
