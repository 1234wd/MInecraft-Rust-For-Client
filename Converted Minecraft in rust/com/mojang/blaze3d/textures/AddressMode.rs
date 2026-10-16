//! Port of: com/mojang/blaze3d/textures/AddressMode.java
//! Java class(es): com.mojang.blaze3d.textures.AddressMode
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.

/// Port of `com.mojang.blaze3d.textures.AddressMode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AddressMode {
    /// `REPEAT` -- ordinal 0.
    Repeat = 0,
    /// `CLAMP_TO_EDGE` -- ordinal 1.
    ClampToEdge = 1,
}

/// Java: `AddressMode.values()`, declaration order.
pub const VALUES: [AddressMode; 2] = [
    AddressMode::Repeat,
    AddressMode::ClampToEdge,
];

impl AddressMode {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            AddressMode::Repeat => "REPEAT",
            AddressMode::ClampToEdge => "CLAMP_TO_EDGE",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for AddressMode {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
