//! Port of: net/minecraft/client/multiplayer/chat/report/ReportType.java
//! Java class(es): net.minecraft.client.multiplayer.chat.report.ReportType
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`. The `backendName` field is checked too.
//!
//! The constructor stores `name.toUpperCase(Locale.ROOT)`, so the constructor argument (`"chat"`) is
//! NOT what the abuse-report backend receives; `backendName()` is `"CHAT"`.

/// Port of `net.minecraft.client.multiplayer.chat.report.ReportType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReportType {
    /// `CHAT` -- ordinal 0.
    Chat = 0,
    /// `SKIN` -- ordinal 1.
    Skin = 1,
    /// `USERNAME` -- ordinal 2.
    Username = 2,
}

/// Java: `ReportType.values()`, declaration order.
pub const VALUES: [ReportType; 3] = [
    ReportType::Chat,
    ReportType::Skin,
    ReportType::Username,
];

impl ReportType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ReportType::Chat => "CHAT",
            ReportType::Skin => "SKIN",
            ReportType::Username => "USERNAME",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ReportType#backendName()` -- the upper-cased constructor argument.
    pub const fn backend_name(&self) -> &'static str {
        match self {
            ReportType::Chat => "CHAT",
            ReportType::Skin => "SKIN",
            ReportType::Username => "USERNAME",
        }
    }
}

impl std::fmt::Display for ReportType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
