//! Port of: net/minecraft/util/debugchart/RemoteDebugSampleType.java
//! Java class(es): net.minecraft.util.debugchart.RemoteDebugSampleType
//! Status: PARTIAL
//!
//! Session 19. The single constant `TICK_TIME` is ported and its name and ordinal are checked
//! against the jar in `_porting/tests/session19_b.rs`.
//!
//! PORT-BLOCKED: `subscription()` returns a `net.minecraft.util.debug.DebugSubscription<?>`
//! (`DebugSubscriptions.DEDICATED_SERVER_TICK_TIME`); `DebugSubscription` and
//! `DebugSubscriptions` are SKELETONs. The jar's dump records its `toString()` as
//! `minecraft:dedicated_server_tick_time`.

/// Port of `net.minecraft.util.debugchart.RemoteDebugSampleType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RemoteDebugSampleType {
    /// Port of `RemoteDebugSampleType#TICK_TIME` -- ordinal 0, subscription
    /// `DebugSubscriptions.DEDICATED_SERVER_TICK_TIME`.
    TickTime = 0,
}

/// Java: `RemoteDebugSampleType.values()`.
pub const VALUES: [RemoteDebugSampleType; 1] = [RemoteDebugSampleType::TickTime];

impl RemoteDebugSampleType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            RemoteDebugSampleType::TickTime => "TICK_TIME",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `public DebugSubscription<?> subscription()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.util.debug.DebugSubscription` /
    /// `net.minecraft.util.debug.DebugSubscriptions`.
    pub fn subscription(&self) {
        todo!("PORT-BLOCKED: net.minecraft.util.debug.DebugSubscription is not ported")
    }
}

impl std::fmt::Display for RemoteDebugSampleType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
