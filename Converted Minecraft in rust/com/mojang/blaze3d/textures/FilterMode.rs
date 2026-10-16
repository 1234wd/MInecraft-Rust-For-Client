//! Port of: com/mojang/blaze3d/textures/FilterMode.java
//! Java class(es): com.mojang.blaze3d.textures.FilterMode
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.

/// Port of `com.mojang.blaze3d.textures.FilterMode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FilterMode {
    /// `NEAREST` -- ordinal 0.
    Nearest = 0,
    /// `LINEAR` -- ordinal 1.
    Linear = 1,
}

/// Java: `FilterMode.values()`, declaration order.
pub const VALUES: [FilterMode; 2] = [
    FilterMode::Nearest,
    FilterMode::Linear,
];

impl FilterMode {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            FilterMode::Nearest => "NEAREST",
            FilterMode::Linear => "LINEAR",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for FilterMode {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
