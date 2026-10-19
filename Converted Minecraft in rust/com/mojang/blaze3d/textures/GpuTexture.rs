//! Port of: com/mojang/blaze3d/textures/GpuTexture.java
//! Java class(es): com.mojang.blaze3d.textures.GpuTexture, com.mojang.blaze3d.textures.GpuTexture.Usage
//! Status: VERIFIED
//!
//! Session 24. `public abstract class GpuTexture implements AutoCloseable`: the backend-neutral
//! half of a GPU texture -- its format, size, mip count, usage bits and debug label, all
//! `private final` and set by the one constructor -- plus abstract `close()`/`isClosed()` for the
//! backend (`GlTexture`, `VulkanTexture`) to implement.
//!
//! # Shape of the port
//!
//! The fields are [`GpuTextureState`], built by [`GpuTextureState::new`] (the Java constructor),
//! with the seven concrete getters. A backend implements the [`GpuTexture`] trait by exposing its
//! state and the two abstract methods; the getters are the trait's provided methods, as they are
//! the abstract class's inherited ones. The same split as `AbstractSampleLogger` (this session).
//!
//! # `getWidth`/`getHeight` shift like Java
//!
//! `width >> mipLevel` is an ARITHMETIC shift whose count Java masks to its low 5 bits, so
//! `getWidth(32)` is `width` again and `getWidth(-1)` is `width >> 31`. `i32::wrapping_shr` masks
//! the same way.
//!
//! # `@interface Usage`
//!
//! A `@Retention(CLASS) @Target(TYPE_USE)` marker annotation on the `int usage` field: no members
//! and nothing visible at run time. It becomes the type alias [`Usage`] (`i32`), which keeps the
//! intent visible in signatures.
//!
//! Verified by the `GPUTEX` group of `sweep_probe.txt` (`tests/session24.rs`), through an
//! anonymous subclass in the probe and a test backend here.

use crate::com::mojang::blaze3d::GpuFormat::GpuFormat;

/// Port of `public static final int USAGE_COPY_DST = 1`.
pub const USAGE_COPY_DST: i32 = 1;
/// Port of `public static final int USAGE_COPY_SRC = 2`.
pub const USAGE_COPY_SRC: i32 = 2;
/// Port of `public static final int USAGE_TEXTURE_BINDING = 4`.
pub const USAGE_TEXTURE_BINDING: i32 = 4;
/// Port of `public static final int USAGE_RENDER_ATTACHMENT = 8`.
pub const USAGE_RENDER_ATTACHMENT: i32 = 8;
/// Port of `public static final int USAGE_CUBEMAP_COMPATIBLE = 16`.
pub const USAGE_CUBEMAP_COMPATIBLE: i32 = 16;

/// Port of the annotation type `GpuTexture.Usage`: an `int` holding `USAGE_*` bits.
pub type Usage = i32;

/// The seven `private final` fields of `GpuTexture`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuTextureState {
    format: GpuFormat,
    width: i32,
    height: i32,
    depth_or_layers: i32,
    mip_levels: i32,
    usage: Usage,
    label: String,
}

impl GpuTextureState {
    /// Port of `public GpuTexture(@Usage int usage, String label, GpuFormat format, int width,
    /// int height, int depthOrLayers, int mipLevels)`. Nothing is checked.
    pub fn new(
        usage: Usage,
        label: impl Into<String>,
        format: GpuFormat,
        width: i32,
        height: i32,
        depth_or_layers: i32,
        mip_levels: i32,
    ) -> Self {
        GpuTextureState {
            format,
            width,
            height,
            depth_or_layers,
            mip_levels,
            usage,
            label: label.into(),
        }
    }
}

/// Port of `GpuTexture`'s methods, for its backends.
pub trait GpuTexture {
    /// The backend's view of the inherited fields.
    fn state(&self) -> &GpuTextureState;

    /// Port of `public abstract void close()`.
    fn close(&mut self);

    /// Port of `public abstract boolean isClosed()`.
    fn is_closed(&self) -> bool;

    /// Port of `public int getWidth(int mipLevel)`: `width >> mipLevel`.
    fn get_width(&self, mip_level: i32) -> i32 {
        self.state().width.wrapping_shr(mip_level as u32)
    }

    /// Port of `public int getHeight(int mipLevel)`: `height >> mipLevel`.
    fn get_height(&self, mip_level: i32) -> i32 {
        self.state().height.wrapping_shr(mip_level as u32)
    }

    /// Port of `public int getDepthOrLayers()`.
    fn get_depth_or_layers(&self) -> i32 {
        self.state().depth_or_layers
    }

    /// Port of `public int getMipLevels()`.
    fn get_mip_levels(&self) -> i32 {
        self.state().mip_levels
    }

    /// Port of `public GpuFormat getFormat()`.
    fn get_format(&self) -> GpuFormat {
        self.state().format
    }

    /// Port of `public @Usage int usage()`.
    fn usage(&self) -> Usage {
        self.state().usage
    }

    /// Port of `public String getLabel()`.
    fn get_label(&self) -> &str {
        &self.state().label
    }
}
