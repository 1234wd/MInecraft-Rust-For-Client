//! Port of: com/mojang/blaze3d/platform/CompareOp.java
//! Java class(es): com.mojang.blaze3d.platform.CompareOp
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.

/// Port of `com.mojang.blaze3d.platform.CompareOp`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CompareOp {
    /// `ALWAYS_PASS` -- ordinal 0.
    AlwaysPass = 0,
    /// `LESS_THAN` -- ordinal 1.
    LessThan = 1,
    /// `LESS_THAN_OR_EQUAL` -- ordinal 2.
    LessThanOrEqual = 2,
    /// `EQUAL` -- ordinal 3.
    Equal = 3,
    /// `NOT_EQUAL` -- ordinal 4.
    NotEqual = 4,
    /// `GREATER_THAN_OR_EQUAL` -- ordinal 5.
    GreaterThanOrEqual = 5,
    /// `GREATER_THAN` -- ordinal 6.
    GreaterThan = 6,
    /// `NEVER_PASS` -- ordinal 7.
    NeverPass = 7,
}

/// Java: `CompareOp.values()`, declaration order.
pub const VALUES: [CompareOp; 8] = [
    CompareOp::AlwaysPass,
    CompareOp::LessThan,
    CompareOp::LessThanOrEqual,
    CompareOp::Equal,
    CompareOp::NotEqual,
    CompareOp::GreaterThanOrEqual,
    CompareOp::GreaterThan,
    CompareOp::NeverPass,
];

impl CompareOp {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            CompareOp::AlwaysPass => "ALWAYS_PASS",
            CompareOp::LessThan => "LESS_THAN",
            CompareOp::LessThanOrEqual => "LESS_THAN_OR_EQUAL",
            CompareOp::Equal => "EQUAL",
            CompareOp::NotEqual => "NOT_EQUAL",
            CompareOp::GreaterThanOrEqual => "GREATER_THAN_OR_EQUAL",
            CompareOp::GreaterThan => "GREATER_THAN",
            CompareOp::NeverPass => "NEVER_PASS",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for CompareOp {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
