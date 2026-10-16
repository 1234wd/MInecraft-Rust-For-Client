//! Port of: com/mojang/blaze3d/platform/BlendOp.java
//! Java class(es): com.mojang.blaze3d.platform.BlendOp
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.

/// Port of `com.mojang.blaze3d.platform.BlendOp`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlendOp {
    /// `ADD` -- ordinal 0.
    Add = 0,
    /// `SUBTRACT` -- ordinal 1.
    Subtract = 1,
    /// `REVERSE_SUBTRACT` -- ordinal 2.
    ReverseSubtract = 2,
    /// `MIN` -- ordinal 3.
    Min = 3,
    /// `MAX` -- ordinal 4.
    Max = 4,
}

/// Java: `BlendOp.values()`, declaration order.
pub const VALUES: [BlendOp; 5] = [
    BlendOp::Add,
    BlendOp::Subtract,
    BlendOp::ReverseSubtract,
    BlendOp::Min,
    BlendOp::Max,
];

impl BlendOp {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            BlendOp::Add => "ADD",
            BlendOp::Subtract => "SUBTRACT",
            BlendOp::ReverseSubtract => "REVERSE_SUBTRACT",
            BlendOp::Min => "MIN",
            BlendOp::Max => "MAX",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for BlendOp {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
