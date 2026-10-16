//! Port of: net/minecraft/world/level/block/state/properties/SlabType.java
//! Java class(es): net.minecraft.world.level.block.state.properties.SlabType
//! Status: VERIFIED
//!
//! Session 19. Three constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to return the `name` field, so `Display` prints `top`.

/// Port of `net.minecraft.world.level.block.state.properties.SlabType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SlabType {
    /// `TOP` -- ordinal 0, `"top"`.
    Top = 0,
    /// `BOTTOM` -- ordinal 1, `"bottom"`.
    Bottom = 1,
    /// `DOUBLE` -- ordinal 2, `"double"`.
    Double = 2,
}

/// Java: `SlabType.values()`, declaration order.
pub const VALUES: [SlabType; 3] = [SlabType::Top, SlabType::Bottom, SlabType::Double];

impl SlabType {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            SlabType::Top => "TOP",
            SlabType::Bottom => "BOTTOM",
            SlabType::Double => "DOUBLE",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `SlabType#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            SlabType::Top => "top",
            SlabType::Bottom => "bottom",
            SlabType::Double => "double",
        }
    }
}

impl std::fmt::Display for SlabType {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for SlabType {
    fn get_serialized_name(&self) -> &str {
        SlabType::get_serialized_name(self)
    }
}
