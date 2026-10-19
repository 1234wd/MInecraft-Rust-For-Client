//! Port of: com/mojang/blaze3d/textures/GpuTextureView.java
//! Java class(es): com.mojang.blaze3d.textures.GpuTextureView
//! Status: VERIFIED
//!
//! Session 24. `public abstract class GpuTextureView implements AutoCloseable`: a view of a range
//! of a [`GpuTexture`]'s mip levels -- the texture, the first mip level and the count, set by the
//! one `protected` constructor -- with the size of a mip level read through to the texture, and
//! abstract `close()`/`isClosed()` for the backend (`GlTextureView`, `VulkanTextureView`).
//!
//! ```java
//! public int getWidth(int mipLevel)  { return this.texture.getWidth(mipLevel + this.baseMipLevel); }
//! public int getHeight(int mipLevel) { return this.texture.getHeight(mipLevel + this.baseMipLevel); }
//! ```
//!
//! The `int` sum wraps, and the texture's `width >> level` then masks the shift count to 5 bits,
//! so a view's level `-1` at base `1` is the texture's level `0`, and an out-of-range level is
//! not an error. Nothing is checked against `mipLevels`.
//!
//! # Shape of the port
//!
//! The split of `GpuTexture` (this session): [`GpuTextureViewState`] holds the three fields and is
//! built by the Java constructor; a backend implements the [`GpuTextureView`] trait by exposing
//! it and the two abstract methods; the getters are the trait's provided methods. The texture is
//! shared (the texture's owner and every view of it hold it), hence `Arc<dyn GpuTexture>`.
//!
//! Verified by the `GPUTEXVIEW` group of `sweep_probe.txt` (`tests/session24.rs`), through
//! anonymous subclasses in the probe and test backends here.

use std::sync::Arc;

use crate::com::mojang::blaze3d::textures::GpuTexture::GpuTexture;

/// The three `private final` fields of `GpuTextureView`.
#[derive(Clone)]
pub struct GpuTextureViewState {
    texture: Arc<dyn GpuTexture>,
    base_mip_level: i32,
    mip_levels: i32,
}

impl GpuTextureViewState {
    /// Port of `protected GpuTextureView(GpuTexture texture, int baseMipLevel, int mipLevels)`.
    /// Nothing is checked.
    pub fn new(texture: Arc<dyn GpuTexture>, base_mip_level: i32, mip_levels: i32) -> Self {
        GpuTextureViewState {
            texture,
            base_mip_level,
            mip_levels,
        }
    }
}

impl std::fmt::Debug for GpuTextureViewState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpuTextureViewState")
            .field("texture", &self.texture.get_label())
            .field("base_mip_level", &self.base_mip_level)
            .field("mip_levels", &self.mip_levels)
            .finish()
    }
}

/// Port of `GpuTextureView`'s methods, for its backends.
pub trait GpuTextureView {
    /// The backend's view of the inherited fields.
    fn state(&self) -> &GpuTextureViewState;

    /// Port of `public abstract void close()`.
    fn close(&self);

    /// Port of `public abstract boolean isClosed()`.
    fn is_closed(&self) -> bool;

    /// Port of `public GpuTexture texture()`.
    fn texture(&self) -> &Arc<dyn GpuTexture> {
        &self.state().texture
    }

    /// Port of `public int baseMipLevel()`.
    fn base_mip_level(&self) -> i32 {
        self.state().base_mip_level
    }

    /// Port of `public int mipLevels()`.
    fn mip_levels(&self) -> i32 {
        self.state().mip_levels
    }

    /// Port of `public int getWidth(int mipLevel)`: `texture.getWidth(mipLevel + baseMipLevel)`.
    fn get_width(&self, mip_level: i32) -> i32 {
        let s = self.state();
        s.texture
            .get_width(mip_level.wrapping_add(s.base_mip_level))
    }

    /// Port of `public int getHeight(int mipLevel)`: `texture.getHeight(mipLevel + baseMipLevel)`.
    fn get_height(&self, mip_level: i32) -> i32 {
        let s = self.state();
        s.texture
            .get_height(mip_level.wrapping_add(s.base_mip_level))
    }
}
