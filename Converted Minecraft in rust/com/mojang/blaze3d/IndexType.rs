//! Port of: com/mojang/blaze3d/IndexType.java
//! Java class(es): com.mojang.blaze3d.IndexType
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.
//!
//! `least(length)` tests `length & -65536` (`0xFFFF0000`): any bit above the low 16 selects `INT`.
//! So a NEGATIVE length selects `INT` too, and 65535 is the largest length that stays `SHORT`.

/// Port of `com.mojang.blaze3d.IndexType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IndexType {
    /// `SHORT` -- ordinal 0.
    Short = 0,
    /// `INT` -- ordinal 1.
    Int = 1,
}

/// Java: `IndexType.values()`, declaration order.
pub const VALUES: [IndexType; 2] = [
    IndexType::Short,
    IndexType::Int,
];

impl IndexType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            IndexType::Short => "SHORT",
            IndexType::Int => "INT",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `public final int bytes` -- bytes per index.
    pub const fn bytes(&self) -> i32 {
        match self {
            IndexType::Short => 2,
            IndexType::Int => 4,
        }
    }

    /// Port of `IndexType#least(int)`.
    pub const fn least(length: i32) -> IndexType {
        if (length & -65536) != 0 {
            IndexType::Int
        } else {
            IndexType::Short
        }
    }
}

impl std::fmt::Display for IndexType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
