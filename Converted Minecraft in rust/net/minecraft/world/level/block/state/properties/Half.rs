//! Port of: net/minecraft/world/level/block/state/properties/Half.java
//! Java class(es): net.minecraft.world.level.block.state.properties.Half
//! Status: VERIFIED
//!
//! Session 19. Two constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to return the `name` field, so `Display` prints `top`.

/// Port of `net.minecraft.world.level.block.state.properties.Half`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Half {
    /// `TOP` -- ordinal 0, `"top"`.
    Top = 0,
    /// `BOTTOM` -- ordinal 1, `"bottom"`.
    Bottom = 1,
}

/// Java: `Half.values()`, declaration order.
pub const VALUES: [Half; 2] = [Half::Top, Half::Bottom];

impl Half {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Half::Top => "TOP",
            Half::Bottom => "BOTTOM",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Half#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            Half::Top => "top",
            Half::Bottom => "bottom",
        }
    }
}

impl std::fmt::Display for Half {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for Half {
    fn get_serialized_name(&self) -> &str {
        Half::get_serialized_name(self)
    }
}
