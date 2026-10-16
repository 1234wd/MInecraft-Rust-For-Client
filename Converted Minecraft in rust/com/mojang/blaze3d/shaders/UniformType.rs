//! Port of: com/mojang/blaze3d/shaders/UniformType.java
//! Java class(es): com.mojang.blaze3d.shaders.UniformType
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.

/// Port of `com.mojang.blaze3d.shaders.UniformType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UniformType {
    /// `UNIFORM_BUFFER` -- ordinal 0.
    UniformBuffer = 0,
    /// `TEXEL_BUFFER` -- ordinal 1.
    TexelBuffer = 1,
}

/// Java: `UniformType.values()`, declaration order.
pub const VALUES: [UniformType; 2] = [
    UniformType::UniformBuffer,
    UniformType::TexelBuffer,
];

impl UniformType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            UniformType::UniformBuffer => "UNIFORM_BUFFER",
            UniformType::TexelBuffer => "TEXEL_BUFFER",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for UniformType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
