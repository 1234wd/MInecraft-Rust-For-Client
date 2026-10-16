//! Port of: net/minecraft/client/InputType.java
//! Java class(es): net.minecraft.client.InputType
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`.

/// Port of `net.minecraft.client.InputType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputType {
    /// `NONE` -- ordinal 0.
    None = 0,
    /// `MOUSE` -- ordinal 1.
    Mouse = 1,
    /// `KEYBOARD_ARROW` -- ordinal 2.
    KeyboardArrow = 2,
    /// `KEYBOARD_TAB` -- ordinal 3.
    KeyboardTab = 3,
}

/// Java: `InputType.values()`, declaration order.
pub const VALUES: [InputType; 4] = [
    InputType::None,
    InputType::Mouse,
    InputType::KeyboardArrow,
    InputType::KeyboardTab,
];

impl InputType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            InputType::None => "NONE",
            InputType::Mouse => "MOUSE",
            InputType::KeyboardArrow => "KEYBOARD_ARROW",
            InputType::KeyboardTab => "KEYBOARD_TAB",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `InputType#isMouse()`.
    pub const fn is_mouse(&self) -> bool {
        matches!(self, InputType::Mouse)
    }

    /// Port of `InputType#isKeyboard()`: arrow OR tab navigation, never `NONE`.
    pub const fn is_keyboard(&self) -> bool {
        matches!(self, InputType::KeyboardArrow | InputType::KeyboardTab)
    }
}

impl std::fmt::Display for InputType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
