//! Port of: net/minecraft/util/profiling/metrics/MetricCategory.java
//! Java class(es): net.minecraft.util.profiling.metrics.MetricCategory
//! Status: VERIFIED
//!
//! Session 19. Nine constants, each with a human-readable `description`. Names, ordinals and
//! `description` are checked against the jar in `_porting/tests/session19_b.rs`.
//!
//! The descriptions are not uniformly spelled: `"event-loops"` uses a hyphen while
//! `"chunk rendering"` uses a space, and `TICK_LOOP` is `"ticking"`. They are copied as-is.

/// Port of `net.minecraft.util.profiling.metrics.MetricCategory`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MetricCategory {
    /// Port of `MetricCategory#PATH_FINDING` -- ordinal 0, `"pathfinding"`.
    PathFinding = 0,
    /// Port of `MetricCategory#EVENT_LOOPS` -- ordinal 1, `"event-loops"`.
    EventLoops = 1,
    /// Port of `MetricCategory#CONSECUTIVE_EXECUTORS` -- ordinal 2, `"consecutive-executors"`.
    ConsecutiveExecutors = 2,
    /// Port of `MetricCategory#TICK_LOOP` -- ordinal 3, `"ticking"`.
    TickLoop = 3,
    /// Port of `MetricCategory#JVM` -- ordinal 4, `"jvm"`.
    Jvm = 4,
    /// Port of `MetricCategory#CHUNK_RENDERING` -- ordinal 5, `"chunk rendering"`.
    ChunkRendering = 5,
    /// Port of `MetricCategory#CHUNK_RENDERING_DISPATCHING` -- ordinal 6,
    /// `"chunk rendering dispatching"`.
    ChunkRenderingDispatching = 6,
    /// Port of `MetricCategory#CPU` -- ordinal 7, `"cpu"`.
    Cpu = 7,
    /// Port of `MetricCategory#GPU` -- ordinal 8, `"gpu"`.
    Gpu = 8,
}

/// Java: `MetricCategory.values()` -- declaration order.
pub const VALUES: [MetricCategory; 9] = [
    MetricCategory::PathFinding,
    MetricCategory::EventLoops,
    MetricCategory::ConsecutiveExecutors,
    MetricCategory::TickLoop,
    MetricCategory::Jvm,
    MetricCategory::ChunkRendering,
    MetricCategory::ChunkRenderingDispatching,
    MetricCategory::Cpu,
    MetricCategory::Gpu,
];

impl MetricCategory {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            MetricCategory::PathFinding => "PATH_FINDING",
            MetricCategory::EventLoops => "EVENT_LOOPS",
            MetricCategory::ConsecutiveExecutors => "CONSECUTIVE_EXECUTORS",
            MetricCategory::TickLoop => "TICK_LOOP",
            MetricCategory::Jvm => "JVM",
            MetricCategory::ChunkRendering => "CHUNK_RENDERING",
            MetricCategory::ChunkRenderingDispatching => "CHUNK_RENDERING_DISPATCHING",
            MetricCategory::Cpu => "CPU",
            MetricCategory::Gpu => "GPU",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `MetricCategory#getDescription()`.
    #[inline]
    pub const fn get_description(&self) -> &'static str {
        match self {
            MetricCategory::PathFinding => "pathfinding",
            MetricCategory::EventLoops => "event-loops",
            MetricCategory::ConsecutiveExecutors => "consecutive-executors",
            MetricCategory::TickLoop => "ticking",
            MetricCategory::Jvm => "jvm",
            MetricCategory::ChunkRendering => "chunk rendering",
            MetricCategory::ChunkRenderingDispatching => "chunk rendering dispatching",
            MetricCategory::Cpu => "cpu",
            MetricCategory::Gpu => "gpu",
        }
    }
}

impl std::fmt::Display for MetricCategory {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
