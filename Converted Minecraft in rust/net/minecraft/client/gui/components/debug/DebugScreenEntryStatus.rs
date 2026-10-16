//! Port of: net/minecraft/client/gui/components/debug/DebugScreenEntryStatus.java
//! Java class(es): net.minecraft.client.gui.components.debug.DebugScreenEntryStatus
//! Status: PARTIAL
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`. The `name` field is checked too.
//!
//! The serialized names are camelCase (`alwaysOn`, `inOverlay`), unlike most enums in the game,
//! and they are what `options.txt`'s debug-entry settings store.
//!
//! `CODEC` is PORT-BLOCKED on DataFixerUpper (`com.mojang.serialization.Codec`); that is the only
//! member not ported, hence `PARTIAL`.

/// Port of `net.minecraft.client.gui.components.debug.DebugScreenEntryStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DebugScreenEntryStatus {
    /// `ALWAYS_ON` -- ordinal 0.
    AlwaysOn = 0,
    /// `IN_OVERLAY` -- ordinal 1.
    InOverlay = 1,
    /// `NEVER` -- ordinal 2.
    Never = 2,
}

/// Java: `DebugScreenEntryStatus.values()`, declaration order.
pub const VALUES: [DebugScreenEntryStatus; 3] = [
    DebugScreenEntryStatus::AlwaysOn,
    DebugScreenEntryStatus::InOverlay,
    DebugScreenEntryStatus::Never,
];

impl DebugScreenEntryStatus {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            DebugScreenEntryStatus::AlwaysOn => "ALWAYS_ON",
            DebugScreenEntryStatus::InOverlay => "IN_OVERLAY",
            DebugScreenEntryStatus::Never => "NEVER",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `DebugScreenEntryStatus#getSerializedName()` (the `name` field).
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            DebugScreenEntryStatus::AlwaysOn => "alwaysOn",
            DebugScreenEntryStatus::InOverlay => "inOverlay",
            DebugScreenEntryStatus::Never => "never",
        }
    }
}

impl std::fmt::Display for DebugScreenEntryStatus {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for DebugScreenEntryStatus {
    fn get_serialized_name(&self) -> &str {
        DebugScreenEntryStatus::get_serialized_name(self)
    }
}

/// Java: `public static final Codec<DebugScreenEntryStatus> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED: the static type is a bare `Codec`, which is DataFixerUpper.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
