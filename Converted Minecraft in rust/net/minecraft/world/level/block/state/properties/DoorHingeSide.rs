//! Port of: net/minecraft/world/level/block/state/properties/DoorHingeSide.java
//! Java class(es): net.minecraft.world.level.block.state.properties.DoorHingeSide
//! Status: VERIFIED
//!
//! Session 19. Two constants, no fields. Every member ported: `getSerializedName`, the
//! `toString` override, plus the implicit `name()`/`ordinal()`/`values()`.
//!
//! Constant names and ordinals are checked against the jar (`session19_a.rs`). The serialized
//! names are computed by a ternary rather than stored in a field, so the dump has no column for
//! them; `"left"` / `"right"` are transcribed from the source:
//!
//! ```java
//! return this == LEFT ? "left" : "right";
//! ```
//!
//! `toString()` is overridden to return the serialized name, so `Display` prints `left`, not
//! `LEFT`.

/// Port of `net.minecraft.world.level.block.state.properties.DoorHingeSide`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DoorHingeSide {
    /// `LEFT` -- ordinal 0, serialized `"left"`.
    Left = 0,
    /// `RIGHT` -- ordinal 1, serialized `"right"`.
    Right = 1,
}

/// Java: `DoorHingeSide.values()`, declaration order.
pub const VALUES: [DoorHingeSide; 2] = [DoorHingeSide::Left, DoorHingeSide::Right];

impl DoorHingeSide {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            DoorHingeSide::Left => "LEFT",
            DoorHingeSide::Right => "RIGHT",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `DoorHingeSide#getSerializedName()`: `this == LEFT ? "left" : "right"`.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            DoorHingeSide::Left => "left",
            DoorHingeSide::Right => "right",
        }
    }
}

impl std::fmt::Display for DoorHingeSide {
    /// Port of the `toString()` override: `getSerializedName()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for DoorHingeSide {
    fn get_serialized_name(&self) -> &str {
        DoorHingeSide::get_serialized_name(self)
    }
}
