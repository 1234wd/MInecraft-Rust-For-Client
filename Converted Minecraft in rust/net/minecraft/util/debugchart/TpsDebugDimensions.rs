//! Port of: net/minecraft/util/debugchart/TpsDebugDimensions.java
//! Java class(es): net.minecraft.util.debugchart.TpsDebugDimensions
//! Status: VERIFIED
//!
//! Session 19. A plain four-constant enum (the dimensions of the TPS debug chart sample).
//! Names and ordinals are checked against the jar in `_porting/tests/session19_b.rs`.

/// Port of `net.minecraft.util.debugchart.TpsDebugDimensions`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TpsDebugDimensions {
    /// Port of `TpsDebugDimensions#FULL_TICK` -- ordinal 0.
    FullTick = 0,
    /// Port of `TpsDebugDimensions#TICK_SERVER_METHOD` -- ordinal 1.
    TickServerMethod = 1,
    /// Port of `TpsDebugDimensions#SCHEDULED_TASKS` -- ordinal 2.
    ScheduledTasks = 2,
    /// Port of `TpsDebugDimensions#IDLE` -- ordinal 3.
    Idle = 3,
}

/// Java: `TpsDebugDimensions.values()` -- declaration order.
pub const VALUES: [TpsDebugDimensions; 4] = [
    TpsDebugDimensions::FullTick,
    TpsDebugDimensions::TickServerMethod,
    TpsDebugDimensions::ScheduledTasks,
    TpsDebugDimensions::Idle,
];

impl TpsDebugDimensions {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            TpsDebugDimensions::FullTick => "FULL_TICK",
            TpsDebugDimensions::TickServerMethod => "TICK_SERVER_METHOD",
            TpsDebugDimensions::ScheduledTasks => "SCHEDULED_TASKS",
            TpsDebugDimensions::Idle => "IDLE",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for TpsDebugDimensions {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
