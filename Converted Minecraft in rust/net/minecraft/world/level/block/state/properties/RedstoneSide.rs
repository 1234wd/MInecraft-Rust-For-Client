//! Port of: net/minecraft/world/level/block/state/properties/RedstoneSide.java
//! Java class(es): net.minecraft.world.level.block.state.properties.RedstoneSide
//! Status: VERIFIED
//!
//! Session 19. Three constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `NONE` is the LAST constant (ordinal 2), unlike `WallSide`, where `NONE` is first.
//! `isConnected()` is `this != NONE`. `toString()` is overridden to `getSerializedName()`, so
//! `Display` prints `side`.

/// Port of `net.minecraft.world.level.block.state.properties.RedstoneSide`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RedstoneSide {
    /// `UP` -- ordinal 0, `"up"`.
    Up = 0,
    /// `SIDE` -- ordinal 1, `"side"`.
    Side = 1,
    /// `NONE` -- ordinal 2, `"none"`.
    None = 2,
}

/// Java: `RedstoneSide.values()`, declaration order.
pub const VALUES: [RedstoneSide; 3] = [RedstoneSide::Up, RedstoneSide::Side, RedstoneSide::None];

impl RedstoneSide {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            RedstoneSide::Up => "UP",
            RedstoneSide::Side => "SIDE",
            RedstoneSide::None => "NONE",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `RedstoneSide#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            RedstoneSide::Up => "up",
            RedstoneSide::Side => "side",
            RedstoneSide::None => "none",
        }
    }

    /// Port of `RedstoneSide#isConnected()`: `this != NONE`.
    #[inline]
    pub const fn is_connected(&self) -> bool {
        !matches!(self, RedstoneSide::None)
    }
}

impl std::fmt::Display for RedstoneSide {
    /// Port of the `toString()` override: `getSerializedName()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for RedstoneSide {
    fn get_serialized_name(&self) -> &str {
        RedstoneSide::get_serialized_name(self)
    }
}
