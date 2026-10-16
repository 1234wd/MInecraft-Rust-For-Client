//! Port of: net/minecraft/client/gui/narration/NarratedElementType.java
//! Java class(es): net.minecraft.client.gui.narration.NarratedElementType
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`.

/// Port of `net.minecraft.client.gui.narration.NarratedElementType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NarratedElementType {
    /// `TITLE` -- ordinal 0.
    Title = 0,
    /// `POSITION` -- ordinal 1.
    Position = 1,
    /// `HINT` -- ordinal 2.
    Hint = 2,
    /// `USAGE` -- ordinal 3.
    Usage = 3,
}

/// Java: `NarratedElementType.values()`, declaration order.
pub const VALUES: [NarratedElementType; 4] = [
    NarratedElementType::Title,
    NarratedElementType::Position,
    NarratedElementType::Hint,
    NarratedElementType::Usage,
];

impl NarratedElementType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            NarratedElementType::Title => "TITLE",
            NarratedElementType::Position => "POSITION",
            NarratedElementType::Hint => "HINT",
            NarratedElementType::Usage => "USAGE",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for NarratedElementType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
