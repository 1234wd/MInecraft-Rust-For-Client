//! Port of: net/minecraft/client/gui/components/Whence.java
//! Java class(es): net.minecraft.client.gui.components.Whence
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`.

/// Port of `net.minecraft.client.gui.components.Whence`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Whence {
    /// `ABSOLUTE` -- ordinal 0.
    Absolute = 0,
    /// `RELATIVE` -- ordinal 1.
    Relative = 1,
    /// `END` -- ordinal 2.
    End = 2,
}

/// Java: `Whence.values()`, declaration order.
pub const VALUES: [Whence; 3] = [
    Whence::Absolute,
    Whence::Relative,
    Whence::End,
];

impl Whence {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Whence::Absolute => "ABSOLUTE",
            Whence::Relative => "RELATIVE",
            Whence::End => "END",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for Whence {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
