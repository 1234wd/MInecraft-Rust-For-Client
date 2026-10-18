//! Port of: net/minecraft/gizmos/GizmoStyle.java
//! Java class(es): net.minecraft.gizmos.GizmoStyle
//! Status: VERIFIED
//!
//! Session 22. `public record GizmoStyle(int stroke, float strokeWidth, int fill)`: four static
//! factories, two predicates and two alpha-scaled readers over `ARGB.multiplyAlpha`.
//!
//! # Names
//!
//! `stroke` and `fill` are each BOTH a record accessor and a static factory in Java -- an overload
//! by parameter list -- so per `DESIGN_DECISIONS.md#overload-naming` the accessors keep the plain
//! names and the factories are [`GizmoStyle::stroke_i32`], [`GizmoStyle::stroke_i32_f32`] and
//! [`GizmoStyle::fill_i32`].
//!
//! # `hasStroke` and NaN
//!
//! `hasStroke()` is `stroke != 0 && strokeWidth > 0.0F`, so a NaN width has no stroke, and so does
//! `-0.0`. `hasFill()` is just `fill != 0`.
//!
//! # Record members
//!
//! `equals` compares `strokeWidth` with `Float.compare` (NaN equals NaN, `0.0 != -0.0`), so it is
//! implemented by hand on [`java_object::equals_f32`]; `hashCode` is
//! [`GizmoStyle::java_hash_code`]; `toString` prints the width with Java's `Float.toString`.
//!
//! Verified by the `REC`, `RECEQ`, `GIZMO`, `GIZMOMUL` and `GIZMODEF` groups of `small_probe.txt`
//! (`tests/session22.rs`): both predicates over 5 strokes x 7 widths (NaN, both zeros, a
//! subnormal, infinity) x 2 fills, and both readers over 6 colours x 8 alpha multipliers.

use crate::javacompat::java_lang::float_to_string::float_to_string;
use crate::javacompat::java_object;
use crate::net::minecraft::util::ARGB;

/// Port of `net.minecraft.gizmos.GizmoStyle`.
#[derive(Debug, Clone, Copy)]
pub struct GizmoStyle {
    stroke: i32,
    stroke_width: f32,
    fill: i32,
}

impl GizmoStyle {
    /// Port of `private static final float DEFAULT_WIDTH = 2.5F`.
    const DEFAULT_WIDTH: f32 = 2.5;

    /// Port of the canonical constructor.
    pub const fn new(stroke: i32, stroke_width: f32, fill: i32) -> Self {
        GizmoStyle {
            stroke,
            stroke_width,
            fill,
        }
    }

    /// Port of the accessor `stroke()`.
    pub const fn stroke(&self) -> i32 {
        self.stroke
    }

    /// Port of the accessor `strokeWidth()`.
    pub const fn stroke_width(&self) -> f32 {
        self.stroke_width
    }

    /// Port of the accessor `fill()`.
    pub const fn fill(&self) -> i32 {
        self.fill
    }

    /// Port of `static GizmoStyle stroke(int argb)`: the default width, no fill.
    pub const fn stroke_i32(argb: i32) -> GizmoStyle {
        GizmoStyle::new(argb, Self::DEFAULT_WIDTH, 0)
    }

    /// Port of `static GizmoStyle stroke(int argb, float width)`.
    pub const fn stroke_i32_f32(argb: i32, width: f32) -> GizmoStyle {
        GizmoStyle::new(argb, width, 0)
    }

    /// Port of `static GizmoStyle fill(int argb)`: no stroke, a `0.0F` width.
    pub const fn fill_i32(argb: i32) -> GizmoStyle {
        GizmoStyle::new(0, 0.0, argb)
    }

    /// Port of `strokeAndFill(int stroke, float strokeWidth, int fill)`.
    pub const fn stroke_and_fill(stroke: i32, stroke_width: f32, fill: i32) -> GizmoStyle {
        GizmoStyle::new(stroke, stroke_width, fill)
    }

    /// Port of `hasFill()`.
    pub const fn has_fill(&self) -> bool {
        self.fill != 0
    }

    /// Port of `hasStroke()`: false for a NaN or non-positive width.
    pub fn has_stroke(&self) -> bool {
        self.stroke != 0 && self.stroke_width > 0.0
    }

    /// Port of `multipliedStroke(float alphaMultiplier)`.
    pub fn multiplied_stroke(&self, alpha_multiplier: f32) -> i32 {
        ARGB::multiply_alpha(self.stroke, alpha_multiplier)
    }

    /// Port of `multipliedFill(float alphaMultiplier)`.
    pub fn multiplied_fill(&self, alpha_multiplier: f32) -> i32 {
        ARGB::multiply_alpha(self.fill, alpha_multiplier)
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            self.stroke,
            java_object::hash_f32(self.stroke_width),
            self.fill,
        ])
    }
}

impl PartialEq for GizmoStyle {
    /// Port of the record's generated `equals`: `Float.compare` on the width.
    fn eq(&self, other: &Self) -> bool {
        self.stroke == other.stroke
            && java_object::equals_f32(self.stroke_width, other.stroke_width)
            && self.fill == other.fill
    }
}

impl Eq for GizmoStyle {}

impl std::hash::Hash for GizmoStyle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.java_hash_code().hash(state)
    }
}

impl std::fmt::Display for GizmoStyle {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GizmoStyle[stroke={}, strokeWidth={}, fill={}]",
            self.stroke,
            float_to_string(self.stroke_width),
            self.fill
        )
    }
}
