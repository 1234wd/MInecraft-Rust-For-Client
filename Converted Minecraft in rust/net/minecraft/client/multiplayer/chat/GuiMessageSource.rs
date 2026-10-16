//! Port of: net/minecraft/client/multiplayer/chat/GuiMessageSource.java
//! Java class(es): net.minecraft.client.multiplayer.chat.GuiMessageSource
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`.

/// Port of `net.minecraft.client.multiplayer.chat.GuiMessageSource`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GuiMessageSource {
    /// `PLAYER` -- ordinal 0.
    Player = 0,
    /// `SYSTEM_SERVER` -- ordinal 1.
    SystemServer = 1,
    /// `SYSTEM_CLIENT` -- ordinal 2.
    SystemClient = 2,
}

/// Java: `GuiMessageSource.values()`, declaration order.
pub const VALUES: [GuiMessageSource; 3] = [
    GuiMessageSource::Player,
    GuiMessageSource::SystemServer,
    GuiMessageSource::SystemClient,
];

impl GuiMessageSource {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            GuiMessageSource::Player => "PLAYER",
            GuiMessageSource::SystemServer => "SYSTEM_SERVER",
            GuiMessageSource::SystemClient => "SYSTEM_CLIENT",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for GuiMessageSource {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
