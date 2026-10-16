//! Port of: net/minecraft/world/level/block/state/properties/ComparatorMode.java
//! Java class(es): net.minecraft.world.level.block.state.properties.ComparatorMode
//! Status: VERIFIED
//!
//! Session 19. Two constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to return the `name` field, so `Display` prints `compare`.

/// Port of `net.minecraft.world.level.block.state.properties.ComparatorMode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ComparatorMode {
    /// `COMPARE` -- ordinal 0, `"compare"`.
    Compare = 0,
    /// `SUBTRACT` -- ordinal 1, `"subtract"`.
    Subtract = 1,
}

/// Java: `ComparatorMode.values()`, declaration order.
pub const VALUES: [ComparatorMode; 2] = [ComparatorMode::Compare, ComparatorMode::Subtract];

impl ComparatorMode {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            ComparatorMode::Compare => "COMPARE",
            ComparatorMode::Subtract => "SUBTRACT",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ComparatorMode#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            ComparatorMode::Compare => "compare",
            ComparatorMode::Subtract => "subtract",
        }
    }
}

impl std::fmt::Display for ComparatorMode {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for ComparatorMode {
    fn get_serialized_name(&self) -> &str {
        ComparatorMode::get_serialized_name(self)
    }
}
