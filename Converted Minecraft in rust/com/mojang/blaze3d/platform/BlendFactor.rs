//! Port of: com/mojang/blaze3d/platform/BlendFactor.java
//! Java class(es): com.mojang.blaze3d.platform.BlendFactor
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.

/// Port of `com.mojang.blaze3d.platform.BlendFactor`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlendFactor {
    /// `CONSTANT_ALPHA` -- ordinal 0.
    ConstantAlpha = 0,
    /// `CONSTANT_COLOR` -- ordinal 1.
    ConstantColor = 1,
    /// `DST_ALPHA` -- ordinal 2.
    DstAlpha = 2,
    /// `DST_COLOR` -- ordinal 3.
    DstColor = 3,
    /// `ONE` -- ordinal 4.
    One = 4,
    /// `ONE_MINUS_CONSTANT_ALPHA` -- ordinal 5.
    OneMinusConstantAlpha = 5,
    /// `ONE_MINUS_CONSTANT_COLOR` -- ordinal 6.
    OneMinusConstantColor = 6,
    /// `ONE_MINUS_DST_ALPHA` -- ordinal 7.
    OneMinusDstAlpha = 7,
    /// `ONE_MINUS_DST_COLOR` -- ordinal 8.
    OneMinusDstColor = 8,
    /// `ONE_MINUS_SRC_ALPHA` -- ordinal 9.
    OneMinusSrcAlpha = 9,
    /// `ONE_MINUS_SRC_COLOR` -- ordinal 10.
    OneMinusSrcColor = 10,
    /// `SRC_ALPHA` -- ordinal 11.
    SrcAlpha = 11,
    /// `SRC_ALPHA_SATURATE` -- ordinal 12.
    SrcAlphaSaturate = 12,
    /// `SRC_COLOR` -- ordinal 13.
    SrcColor = 13,
    /// `ZERO` -- ordinal 14.
    Zero = 14,
}

/// Java: `BlendFactor.values()`, declaration order.
pub const VALUES: [BlendFactor; 15] = [
    BlendFactor::ConstantAlpha,
    BlendFactor::ConstantColor,
    BlendFactor::DstAlpha,
    BlendFactor::DstColor,
    BlendFactor::One,
    BlendFactor::OneMinusConstantAlpha,
    BlendFactor::OneMinusConstantColor,
    BlendFactor::OneMinusDstAlpha,
    BlendFactor::OneMinusDstColor,
    BlendFactor::OneMinusSrcAlpha,
    BlendFactor::OneMinusSrcColor,
    BlendFactor::SrcAlpha,
    BlendFactor::SrcAlphaSaturate,
    BlendFactor::SrcColor,
    BlendFactor::Zero,
];

impl BlendFactor {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            BlendFactor::ConstantAlpha => "CONSTANT_ALPHA",
            BlendFactor::ConstantColor => "CONSTANT_COLOR",
            BlendFactor::DstAlpha => "DST_ALPHA",
            BlendFactor::DstColor => "DST_COLOR",
            BlendFactor::One => "ONE",
            BlendFactor::OneMinusConstantAlpha => "ONE_MINUS_CONSTANT_ALPHA",
            BlendFactor::OneMinusConstantColor => "ONE_MINUS_CONSTANT_COLOR",
            BlendFactor::OneMinusDstAlpha => "ONE_MINUS_DST_ALPHA",
            BlendFactor::OneMinusDstColor => "ONE_MINUS_DST_COLOR",
            BlendFactor::OneMinusSrcAlpha => "ONE_MINUS_SRC_ALPHA",
            BlendFactor::OneMinusSrcColor => "ONE_MINUS_SRC_COLOR",
            BlendFactor::SrcAlpha => "SRC_ALPHA",
            BlendFactor::SrcAlphaSaturate => "SRC_ALPHA_SATURATE",
            BlendFactor::SrcColor => "SRC_COLOR",
            BlendFactor::Zero => "ZERO",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for BlendFactor {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
