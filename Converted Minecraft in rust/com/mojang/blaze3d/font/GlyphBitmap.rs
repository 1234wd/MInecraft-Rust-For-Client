//! Port of: com/mojang/blaze3d/font/GlyphBitmap.java
//! Java class(es): com.mojang.blaze3d.font.GlyphBitmap
//! Status: VERIFIED
//!
//! Session 24. `public interface GlyphBitmap`: a rasterised glyph -- its pixel size, oversampling
//! and bearings, and how to upload it into an atlas texture. Five abstract methods and six default
//! ones that place the glyph's quad:
//!
//! ```java
//! default float getLeft()         { return getBearingLeft(); }
//! default float getRight()        { return getLeft() + getPixelWidth() / getOversample(); }
//! default float getTop()          { return 7.0F - getBearingTop(); }
//! default float getBottom()       { return getTop() + getPixelHeight() / getOversample(); }
//! default float getBearingLeft()  { return 0.0F; }
//! default float getBearingTop()   { return 7.0F; }
//! ```
//!
//! `getPixelWidth() / getOversample()` is `int / float`: the `int` is widened to `float` first,
//! then a `float` division (so `0 / 0.0F` is NaN, and a huge width loses precision exactly as in
//! Java). Every default calls the others through the trait, so an implementation that overrides
//! `getLeft` or `getBearingTop` moves the dependent edges with it, as a Java override does.
//!
//! `upload(int x, int y, GpuTexture texture)` takes the atlas texture as `&dyn GpuTexture`.
//!
//! Verified by the `GLYPHBMP` group of `sweep_probe.txt` (`tests/session24.rs`): the defaults over
//! implementations with and without overrides, special values included, plus the `API24` shape.

use crate::com::mojang::blaze3d::textures::GpuTexture::GpuTexture;
use crate::javacompat::java_lang;

/// Port of `com.mojang.blaze3d.font.GlyphBitmap`.
pub trait GlyphBitmap {
    /// Port of `int getPixelWidth()`.
    fn get_pixel_width(&self) -> i32;

    /// Port of `int getPixelHeight()`.
    fn get_pixel_height(&self) -> i32;

    /// Port of `void upload(int x, int y, GpuTexture texture)`.
    fn upload(&self, x: i32, y: i32, texture: &dyn GpuTexture);

    /// Port of `boolean isColored()`.
    fn is_colored(&self) -> bool;

    /// Port of `float getOversample()`.
    fn get_oversample(&self) -> f32;

    /// Port of `default float getLeft()`: `getBearingLeft()`.
    fn get_left(&self) -> f32 {
        self.get_bearing_left()
    }

    /// Port of `default float getRight()`: `getLeft() + getPixelWidth() / getOversample()`.
    fn get_right(&self) -> f32 {
        java_lang::add_f32(
            self.get_left(),
            java_lang::div_f32(self.get_pixel_width() as f32, self.get_oversample()),
        )
    }

    /// Port of `default float getTop()`: `7.0F - getBearingTop()`.
    fn get_top(&self) -> f32 {
        java_lang::sub_f32(7.0, self.get_bearing_top())
    }

    /// Port of `default float getBottom()`: `getTop() + getPixelHeight() / getOversample()`.
    fn get_bottom(&self) -> f32 {
        java_lang::add_f32(
            self.get_top(),
            java_lang::div_f32(self.get_pixel_height() as f32, self.get_oversample()),
        )
    }

    /// Port of `default float getBearingLeft()`: `0.0F`.
    fn get_bearing_left(&self) -> f32 {
        0.0
    }

    /// Port of `default float getBearingTop()`: `7.0F`.
    fn get_bearing_top(&self) -> f32 {
        7.0
    }
}
