//! Port of: net/minecraft/world/level/block/state/properties/PistonType.java
//! Java class(es): net.minecraft.world.level.block.state.properties.PistonType
//! Status: VERIFIED
//!
//! Session 19. Two constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! # `DEFAULT` IS SERIALIZED AS `"normal"`
//!
//! ```java
//! DEFAULT("normal"),
//! STICKY("sticky");
//! ```
//!
//! The constant name and the persisted name disagree. Block states say `type=normal`; `toString()`
//! is overridden to the `name` field, so `Display` also prints `normal`, never `default`.

/// Port of `net.minecraft.world.level.block.state.properties.PistonType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PistonType {
    /// `DEFAULT` -- ordinal 0, serialized `"normal"` (not `"default"`).
    Default = 0,
    /// `STICKY` -- ordinal 1, `"sticky"`.
    Sticky = 1,
}

/// Java: `PistonType.values()`, declaration order.
pub const VALUES: [PistonType; 2] = [PistonType::Default, PistonType::Sticky];

impl PistonType {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            PistonType::Default => "DEFAULT",
            PistonType::Sticky => "STICKY",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `PistonType#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            PistonType::Default => "normal",
            PistonType::Sticky => "sticky",
        }
    }
}

impl std::fmt::Display for PistonType {
    /// Port of the `toString()` override: the `name` field (`"normal"` for `DEFAULT`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for PistonType {
    fn get_serialized_name(&self) -> &str {
        PistonType::get_serialized_name(self)
    }
}
