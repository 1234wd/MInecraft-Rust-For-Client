//! Port of: net/minecraft/world/level/block/state/properties/BambooLeaves.java
//! Java class(es): net.minecraft.world.level.block.state.properties.BambooLeaves
//! Status: VERIFIED
//!
//! Session 19. Three constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to return the `name` field, so `Display` prints `none`.

/// Port of `net.minecraft.world.level.block.state.properties.BambooLeaves`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BambooLeaves {
    /// `NONE` -- ordinal 0, `"none"`.
    None = 0,
    /// `SMALL` -- ordinal 1, `"small"`.
    Small = 1,
    /// `LARGE` -- ordinal 2, `"large"`.
    Large = 2,
}

/// Java: `BambooLeaves.values()`, declaration order.
pub const VALUES: [BambooLeaves; 3] = [BambooLeaves::None, BambooLeaves::Small, BambooLeaves::Large];

impl BambooLeaves {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            BambooLeaves::None => "NONE",
            BambooLeaves::Small => "SMALL",
            BambooLeaves::Large => "LARGE",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `BambooLeaves#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            BambooLeaves::None => "none",
            BambooLeaves::Small => "small",
            BambooLeaves::Large => "large",
        }
    }
}

impl std::fmt::Display for BambooLeaves {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for BambooLeaves {
    fn get_serialized_name(&self) -> &str {
        BambooLeaves::get_serialized_name(self)
    }
}
