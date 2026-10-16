//! Port of: com/mojang/blaze3d/platform/PolygonMode.java
//! Java class(es): com.mojang.blaze3d.platform.PolygonMode
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.

/// Port of `com.mojang.blaze3d.platform.PolygonMode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PolygonMode {
    /// `FILL` -- ordinal 0.
    Fill = 0,
    /// `WIREFRAME` -- ordinal 1.
    Wireframe = 1,
}

/// Java: `PolygonMode.values()`, declaration order.
pub const VALUES: [PolygonMode; 2] = [
    PolygonMode::Fill,
    PolygonMode::Wireframe,
];

impl PolygonMode {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PolygonMode::Fill => "FILL",
            PolygonMode::Wireframe => "WIREFRAME",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for PolygonMode {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
