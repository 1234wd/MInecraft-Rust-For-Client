//! Port of: net/minecraft/world/level/block/state/properties/WallSide.java
//! Java class(es): net.minecraft.world.level.block.state.properties.WallSide
//! Status: VERIFIED
//!
//! Session 19. Three constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to `getSerializedName()`, so `Display` prints `none`.

/// Port of `net.minecraft.world.level.block.state.properties.WallSide`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WallSide {
    /// `NONE` -- ordinal 0, `"none"`.
    None = 0,
    /// `LOW` -- ordinal 1, `"low"`.
    Low = 1,
    /// `TALL` -- ordinal 2, `"tall"`.
    Tall = 2,
}

/// Java: `WallSide.values()`, declaration order.
pub const VALUES: [WallSide; 3] = [WallSide::None, WallSide::Low, WallSide::Tall];

impl WallSide {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            WallSide::None => "NONE",
            WallSide::Low => "LOW",
            WallSide::Tall => "TALL",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `WallSide#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            WallSide::None => "none",
            WallSide::Low => "low",
            WallSide::Tall => "tall",
        }
    }
}

impl std::fmt::Display for WallSide {
    /// Port of the `toString()` override: `getSerializedName()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for WallSide {
    fn get_serialized_name(&self) -> &str {
        WallSide::get_serialized_name(self)
    }
}
