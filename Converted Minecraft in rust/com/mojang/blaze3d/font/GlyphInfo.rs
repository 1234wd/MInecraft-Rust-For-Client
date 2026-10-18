//! Port of: com/mojang/blaze3d/font/GlyphInfo.java
//! Java class(es): com.mojang.blaze3d.font.GlyphInfo
//! Status: VERIFIED
//!
//! Session 22. `public interface GlyphInfo`: a glyph's horizontal metrics. One abstract method
//! (`getAdvance()`), three defaults and a static factory.
//!
//! * `getAdvance(bold)` is `getAdvance() + (bold ? getBoldOffset() : 0.0F)` -- a float add even when
//!   not bold, so an advance of `-0.0` comes back as `+0.0` (`-0.0 + 0.0`). Probed.
//! * `getBoldOffset()` and `getShadowOffset()` default to `1.0F`.
//! * `simple(advance)` is a `GlyphInfo` whose `getAdvance()` is `advance` and which keeps the
//!   defaults.
//!
//! `getAdvance` is overloaded, so per `DESIGN_DECISIONS.md#overload-naming` the bold form is
//! [`GlyphInfo::get_advance_bool`].
//!
//! Verified by the `GLYPH` group of `small_probe.txt` (`tests/session22.rs`): seven advances
//! (both zeros, NaN, `MAX_VALUE`), every method.

/// Port of `com.mojang.blaze3d.font.GlyphInfo`.
pub trait GlyphInfo {
    /// Port of `float getAdvance()`.
    fn get_advance(&self) -> f32;

    /// Port of `default float getAdvance(boolean bold)`.
    fn get_advance_bool(&self, bold: bool) -> f32 {
        self.get_advance() + if bold { self.get_bold_offset() } else { 0.0 }
    }

    /// Port of `default float getBoldOffset()`: `1.0F`.
    fn get_bold_offset(&self) -> f32 {
        1.0
    }

    /// Port of `default float getShadowOffset()`: `1.0F`.
    fn get_shadow_offset(&self) -> f32 {
        1.0
    }
}

/// The `GlyphInfo` that `GlyphInfo.simple(advance)` returns.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimpleGlyphInfo {
    advance: f32,
}

impl GlyphInfo for SimpleGlyphInfo {
    fn get_advance(&self) -> f32 {
        self.advance
    }
}

/// Port of `static GlyphInfo simple(float advance)`.
pub const fn simple(advance: f32) -> SimpleGlyphInfo {
    SimpleGlyphInfo { advance }
}
