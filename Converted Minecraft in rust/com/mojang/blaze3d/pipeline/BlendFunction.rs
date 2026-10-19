//! Port of: com/mojang/blaze3d/pipeline/BlendFunction.java
//! Java class(es): com.mojang.blaze3d.pipeline.BlendFunction
//! Status: PARTIAL
//!
//! Session 24. `public record BlendFunction(BlendEquation color, BlendEquation alpha)`: the colour
//! and alpha blend equations of a render pipeline, eight named presets, and five convenience
//! constructors that all end in the canonical one.
//!
//! | Java constructor | here | builds |
//! |---|---|---|
//! | `(BlendEquation color, BlendEquation alpha)` (canonical) | [`BlendFunction::new`] | as given |
//! | `(srcC, dstC, colorOp, srcA, dstA, alphaOp)` | [`BlendFunction::of_factors_ops`] | two equations |
//! | `(srcC, dstC, srcA, dstA)` | [`BlendFunction::of_factors`] | both ops `ADD` |
//! | `(BlendEquation equation)` | [`BlendFunction::of_equation`] | the same equation twice |
//! | `(srcFactor, dstFactor, BlendOp op)` | [`BlendFunction::of_factor_op`] | one equation, twice |
//! | `(srcFactor, dstFactor)` | [`BlendFunction::of_factor_pair`] | one `ADD` equation, twice |
//!
//! `equals` and `toString` are the record's (`BlendFunction[color=BlendEquation[...],
//! alpha=BlendEquation[...]]`); `hashCode` folds [`BlendEquation`]'s, which is built from enum
//! identity hashes and is not portable (OPEN_QUESTIONS #30, as for `BlendEquation` in session
//! 22), so it is not ported.
//!
//! Verified by the `BLENDFN` group of `sweep_probe.txt` (`tests/session24.rs`): the eight
//! presets, every constructor, and the `equals` matrix.

use crate::com::mojang::blaze3d::pipeline::BlendEquation::BlendEquation;
use crate::com::mojang::blaze3d::platform::BlendFactor::BlendFactor;
use crate::com::mojang::blaze3d::platform::BlendOp::BlendOp;

/// Port of `com.mojang.blaze3d.pipeline.BlendFunction`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendFunction {
    color: BlendEquation,
    alpha: BlendEquation,
}

impl BlendFunction {
    /// Port of the canonical constructor `BlendFunction(BlendEquation color, BlendEquation alpha)`.
    pub const fn new(color: BlendEquation, alpha: BlendEquation) -> Self {
        BlendFunction { color, alpha }
    }

    /// Port of `BlendFunction(BlendFactor srcColorFactor, BlendFactor dstColorFactor, BlendOp
    /// colorOp, BlendFactor srcAlphaFactor, BlendFactor dstAlphaFactor, BlendOp alphaOp)`.
    pub const fn of_factors_ops(
        src_color_factor: BlendFactor,
        dst_color_factor: BlendFactor,
        color_op: BlendOp,
        src_alpha_factor: BlendFactor,
        dst_alpha_factor: BlendFactor,
        alpha_op: BlendOp,
    ) -> Self {
        Self::new(
            BlendEquation::new(src_color_factor, dst_color_factor, color_op),
            BlendEquation::new(src_alpha_factor, dst_alpha_factor, alpha_op),
        )
    }

    /// Port of `BlendFunction(BlendFactor srcColorFactor, BlendFactor dstColorFactor, BlendFactor
    /// srcAlphaFactor, BlendFactor dstAlphaFactor)`: both ops `BlendOp.ADD`.
    pub const fn of_factors(
        src_color_factor: BlendFactor,
        dst_color_factor: BlendFactor,
        src_alpha_factor: BlendFactor,
        dst_alpha_factor: BlendFactor,
    ) -> Self {
        Self::of_factors_ops(
            src_color_factor,
            dst_color_factor,
            BlendOp::Add,
            src_alpha_factor,
            dst_alpha_factor,
            BlendOp::Add,
        )
    }

    /// Port of `BlendFunction(BlendEquation equation)`: `this(equation, equation)`.
    pub const fn of_equation(equation: BlendEquation) -> Self {
        Self::new(equation, equation)
    }

    /// Port of `BlendFunction(BlendFactor srcFactor, BlendFactor dstFactor, BlendOp op)`.
    pub const fn of_factor_op(
        src_factor: BlendFactor,
        dst_factor: BlendFactor,
        op: BlendOp,
    ) -> Self {
        Self::of_equation(BlendEquation::new(src_factor, dst_factor, op))
    }

    /// Port of `BlendFunction(BlendFactor srcFactor, BlendFactor dstFactor)`: one `ADD`
    /// equation for both colour and alpha.
    pub const fn of_factor_pair(src_factor: BlendFactor, dst_factor: BlendFactor) -> Self {
        Self::of_equation(BlendEquation::new(src_factor, dst_factor, BlendOp::Add))
    }

    /// Port of the record accessor `BlendEquation color()`.
    pub const fn color(&self) -> BlendEquation {
        self.color
    }

    /// Port of the record accessor `BlendEquation alpha()`.
    pub const fn alpha(&self) -> BlendEquation {
        self.alpha
    }
}

/// Port of `public static final BlendFunction LIGHTNING = new BlendFunction(SRC_ALPHA, ONE)`.
pub const LIGHTNING: BlendFunction =
    BlendFunction::of_factor_pair(BlendFactor::SrcAlpha, BlendFactor::One);

/// Port of `GLINT = new BlendFunction(SRC_COLOR, ONE, ZERO, ONE)`.
pub const GLINT: BlendFunction = BlendFunction::of_factors(
    BlendFactor::SrcColor,
    BlendFactor::One,
    BlendFactor::Zero,
    BlendFactor::One,
);

/// Port of `OVERLAY = new BlendFunction(SRC_ALPHA, ONE, ONE, ZERO)`.
pub const OVERLAY: BlendFunction = BlendFunction::of_factors(
    BlendFactor::SrcAlpha,
    BlendFactor::One,
    BlendFactor::One,
    BlendFactor::Zero,
);

/// Port of `TRANSLUCENT = new BlendFunction(SRC_ALPHA, ONE_MINUS_SRC_ALPHA, ONE, ONE_MINUS_SRC_ALPHA)`.
pub const TRANSLUCENT: BlendFunction = BlendFunction::of_factors(
    BlendFactor::SrcAlpha,
    BlendFactor::OneMinusSrcAlpha,
    BlendFactor::One,
    BlendFactor::OneMinusSrcAlpha,
);

/// Port of `TRANSLUCENT_PREMULTIPLIED_ALPHA = new BlendFunction(ONE, ONE_MINUS_SRC_ALPHA, ONE,
/// ONE_MINUS_SRC_ALPHA)`.
pub const TRANSLUCENT_PREMULTIPLIED_ALPHA: BlendFunction = BlendFunction::of_factors(
    BlendFactor::One,
    BlendFactor::OneMinusSrcAlpha,
    BlendFactor::One,
    BlendFactor::OneMinusSrcAlpha,
);

/// Port of `ADDITIVE = new BlendFunction(ONE, ONE)`.
pub const ADDITIVE: BlendFunction =
    BlendFunction::of_factor_pair(BlendFactor::One, BlendFactor::One);

/// Port of `ENTITY_OUTLINE_BLIT = new BlendFunction(SRC_ALPHA, ONE_MINUS_SRC_ALPHA, ZERO, ONE)`.
pub const ENTITY_OUTLINE_BLIT: BlendFunction = BlendFunction::of_factors(
    BlendFactor::SrcAlpha,
    BlendFactor::OneMinusSrcAlpha,
    BlendFactor::Zero,
    BlendFactor::One,
);

/// Port of `INVERT = new BlendFunction(ONE_MINUS_DST_COLOR, ONE_MINUS_SRC_COLOR, ONE, ZERO)`.
pub const INVERT: BlendFunction = BlendFunction::of_factors(
    BlendFactor::OneMinusDstColor,
    BlendFactor::OneMinusSrcColor,
    BlendFactor::One,
    BlendFactor::Zero,
);

impl std::fmt::Display for BlendFunction {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BlendFunction[color={}, alpha={}]",
            self.color, self.alpha
        )
    }
}
