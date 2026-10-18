//! Port of: com/mojang/blaze3d/GpuFormat.java
//! Java class(es): com.mojang.blaze3d.GpuFormat, com.mojang.blaze3d.ComponentType
//! Status: VERIFIED
//!
//! Session 22. `public enum GpuFormat`: the 56 texture/vertex formats the GPU layer knows, each a
//! [`ComponentType`] and a component count, with its nested `enum ComponentType` (a byte size each).
//!
//! * `blockSize()` is `componentType.byteSize() * componentCount`; `byteAlignment()` is the component
//!   size alone. The packed formats (`RGB10A2_*`, `RG11B10_FLOAT`, the depth/stencil formats) are one
//!   `OPAQUE_*` component, so their block size is that component's size.
//! * `hasDepthAspect()` is true for `D32_FLOAT`, `D32_FLOAT_S8_UINT`, `D24_UNORM_S8_UINT`, `D16_UNORM`;
//!   `hasStencilAspect()` for `S8_UINT`, `D32_FLOAT_S8_UINT`, `D24_UNORM_S8_UINT`; `hasColorAspect()`
//!   is neither.
//!
//! The variant tables below were generated from the probe's rows and compared constant by constant
//! with the decompiled source; the jar and the source agree on every name, component type, count
//! and byte size. Verified by the `GPUFMT` and `GPUCOMP` groups of `small_probe.txt`
//! (`tests/session22.rs`): every constant's ordinal and every method.

/// Port of `com.mojang.blaze3d.GpuFormat.ComponentType`. Declaration order is the ordinal order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentType {
    /// `UNORM_8(1)` -- ordinal 0.
    Unorm8 = 0,
    /// `SNORM_8(1)` -- ordinal 1.
    Snorm8 = 1,
    /// `UINT_8(1)` -- ordinal 2.
    Uint8 = 2,
    /// `SINT_8(1)` -- ordinal 3.
    Sint8 = 3,
    /// `UNORM_16(2)` -- ordinal 4.
    Unorm16 = 4,
    /// `SNORM_16(2)` -- ordinal 5.
    Snorm16 = 5,
    /// `UINT_16(2)` -- ordinal 6.
    Uint16 = 6,
    /// `SINT_16(2)` -- ordinal 7.
    Sint16 = 7,
    /// `FLOAT_16(2)` -- ordinal 8.
    Float16 = 8,
    /// `UINT_32(4)` -- ordinal 9.
    Uint32 = 9,
    /// `SINT_32(4)` -- ordinal 10.
    Sint32 = 10,
    /// `FLOAT_32(4)` -- ordinal 11.
    Float32 = 11,
    /// `OPAQUE_8(1)` -- ordinal 12.
    Opaque8 = 12,
    /// `OPAQUE_16(2)` -- ordinal 13.
    Opaque16 = 13,
    /// `OPAQUE_32(4)` -- ordinal 14.
    Opaque32 = 14,
    /// `OPAQUE_64(8)` -- ordinal 15.
    Opaque64 = 15,
}

impl ComponentType {
    /// Java: `ComponentType.values()`.
    pub const VALUES: [ComponentType; 16] = [
        ComponentType::Unorm8,
        ComponentType::Snorm8,
        ComponentType::Uint8,
        ComponentType::Sint8,
        ComponentType::Unorm16,
        ComponentType::Snorm16,
        ComponentType::Uint16,
        ComponentType::Sint16,
        ComponentType::Float16,
        ComponentType::Uint32,
        ComponentType::Sint32,
        ComponentType::Float32,
        ComponentType::Opaque8,
        ComponentType::Opaque16,
        ComponentType::Opaque32,
        ComponentType::Opaque64,
    ];

    /// Java: `ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ComponentType::Unorm8 => "UNORM_8",
            ComponentType::Snorm8 => "SNORM_8",
            ComponentType::Uint8 => "UINT_8",
            ComponentType::Sint8 => "SINT_8",
            ComponentType::Unorm16 => "UNORM_16",
            ComponentType::Snorm16 => "SNORM_16",
            ComponentType::Uint16 => "UINT_16",
            ComponentType::Sint16 => "SINT_16",
            ComponentType::Float16 => "FLOAT_16",
            ComponentType::Uint32 => "UINT_32",
            ComponentType::Sint32 => "SINT_32",
            ComponentType::Float32 => "FLOAT_32",
            ComponentType::Opaque8 => "OPAQUE_8",
            ComponentType::Opaque16 => "OPAQUE_16",
            ComponentType::Opaque32 => "OPAQUE_32",
            ComponentType::Opaque64 => "OPAQUE_64",
        }
    }

    /// Port of `byteSize()`.
    pub const fn byte_size(&self) -> i32 {
        match self {
            ComponentType::Unorm8 => 1,
            ComponentType::Snorm8 => 1,
            ComponentType::Uint8 => 1,
            ComponentType::Sint8 => 1,
            ComponentType::Unorm16 => 2,
            ComponentType::Snorm16 => 2,
            ComponentType::Uint16 => 2,
            ComponentType::Sint16 => 2,
            ComponentType::Float16 => 2,
            ComponentType::Uint32 => 4,
            ComponentType::Sint32 => 4,
            ComponentType::Float32 => 4,
            ComponentType::Opaque8 => 1,
            ComponentType::Opaque16 => 2,
            ComponentType::Opaque32 => 4,
            ComponentType::Opaque64 => 8,
        }
    }
}

/// Port of `com.mojang.blaze3d.GpuFormat`. Declaration order is the ordinal order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuFormat {
    /// `R8_UNORM(UNORM_8, 1)` -- ordinal 0.
    R8Unorm = 0,
    /// `R8_SNORM(SNORM_8, 1)` -- ordinal 1.
    R8Snorm = 1,
    /// `RG8_UNORM(UNORM_8, 2)` -- ordinal 2.
    Rg8Unorm = 2,
    /// `RG8_SNORM(SNORM_8, 2)` -- ordinal 3.
    Rg8Snorm = 3,
    /// `RGB8_UNORM(UNORM_8, 3)` -- ordinal 4.
    Rgb8Unorm = 4,
    /// `RGB8_SNORM(SNORM_8, 3)` -- ordinal 5.
    Rgb8Snorm = 5,
    /// `RGBA8_UNORM(UNORM_8, 4)` -- ordinal 6.
    Rgba8Unorm = 6,
    /// `RGBA8_SNORM(SNORM_8, 4)` -- ordinal 7.
    Rgba8Snorm = 7,
    /// `R16_UNORM(UNORM_16, 1)` -- ordinal 8.
    R16Unorm = 8,
    /// `R16_SNORM(SNORM_16, 1)` -- ordinal 9.
    R16Snorm = 9,
    /// `RG16_UNORM(UNORM_16, 2)` -- ordinal 10.
    Rg16Unorm = 10,
    /// `RG16_SNORM(SNORM_16, 2)` -- ordinal 11.
    Rg16Snorm = 11,
    /// `RGB16_UNORM(UNORM_16, 3)` -- ordinal 12.
    Rgb16Unorm = 12,
    /// `RGB16_SNORM(SNORM_16, 3)` -- ordinal 13.
    Rgb16Snorm = 13,
    /// `RGBA16_UNORM(UNORM_16, 4)` -- ordinal 14.
    Rgba16Unorm = 14,
    /// `RGBA16_SNORM(SNORM_16, 4)` -- ordinal 15.
    Rgba16Snorm = 15,
    /// `R8_UINT(UINT_8, 1)` -- ordinal 16.
    R8Uint = 16,
    /// `R8_SINT(SINT_8, 1)` -- ordinal 17.
    R8Sint = 17,
    /// `RG8_UINT(UINT_8, 2)` -- ordinal 18.
    Rg8Uint = 18,
    /// `RG8_SINT(SINT_8, 2)` -- ordinal 19.
    Rg8Sint = 19,
    /// `RGB8_UINT(UINT_8, 3)` -- ordinal 20.
    Rgb8Uint = 20,
    /// `RGB8_SINT(SINT_8, 3)` -- ordinal 21.
    Rgb8Sint = 21,
    /// `RGBA8_UINT(UINT_8, 4)` -- ordinal 22.
    Rgba8Uint = 22,
    /// `RGBA8_SINT(SINT_8, 4)` -- ordinal 23.
    Rgba8Sint = 23,
    /// `R16_UINT(UINT_16, 1)` -- ordinal 24.
    R16Uint = 24,
    /// `R16_SINT(SINT_16, 1)` -- ordinal 25.
    R16Sint = 25,
    /// `RG16_UINT(UINT_16, 2)` -- ordinal 26.
    Rg16Uint = 26,
    /// `RG16_SINT(SINT_16, 2)` -- ordinal 27.
    Rg16Sint = 27,
    /// `RGB16_UINT(UINT_16, 3)` -- ordinal 28.
    Rgb16Uint = 28,
    /// `RGB16_SINT(SINT_16, 3)` -- ordinal 29.
    Rgb16Sint = 29,
    /// `RGBA16_UINT(UINT_16, 4)` -- ordinal 30.
    Rgba16Uint = 30,
    /// `RGBA16_SINT(SINT_16, 4)` -- ordinal 31.
    Rgba16Sint = 31,
    /// `R32_UINT(UINT_32, 1)` -- ordinal 32.
    R32Uint = 32,
    /// `R32_SINT(SINT_32, 1)` -- ordinal 33.
    R32Sint = 33,
    /// `RG32_UINT(UINT_32, 2)` -- ordinal 34.
    Rg32Uint = 34,
    /// `RG32_SINT(SINT_32, 2)` -- ordinal 35.
    Rg32Sint = 35,
    /// `RGB32_UINT(UINT_32, 3)` -- ordinal 36.
    Rgb32Uint = 36,
    /// `RGB32_SINT(SINT_32, 3)` -- ordinal 37.
    Rgb32Sint = 37,
    /// `RGBA32_UINT(UINT_32, 4)` -- ordinal 38.
    Rgba32Uint = 38,
    /// `RGBA32_SINT(SINT_32, 4)` -- ordinal 39.
    Rgba32Sint = 39,
    /// `R16_FLOAT(FLOAT_16, 1)` -- ordinal 40.
    R16Float = 40,
    /// `RG16_FLOAT(FLOAT_16, 2)` -- ordinal 41.
    Rg16Float = 41,
    /// `RGB16_FLOAT(FLOAT_16, 3)` -- ordinal 42.
    Rgb16Float = 42,
    /// `RGBA16_FLOAT(FLOAT_16, 4)` -- ordinal 43.
    Rgba16Float = 43,
    /// `R32_FLOAT(FLOAT_32, 1)` -- ordinal 44.
    R32Float = 44,
    /// `RG32_FLOAT(FLOAT_32, 2)` -- ordinal 45.
    Rg32Float = 45,
    /// `RGB32_FLOAT(FLOAT_32, 3)` -- ordinal 46.
    Rgb32Float = 46,
    /// `RGBA32_FLOAT(FLOAT_32, 4)` -- ordinal 47.
    Rgba32Float = 47,
    /// `RGB10A2_UNORM(OPAQUE_32, 1)` -- ordinal 48.
    Rgb10a2Unorm = 48,
    /// `RGB10A2_UINT(OPAQUE_32, 1)` -- ordinal 49.
    Rgb10a2Uint = 49,
    /// `RG11B10_FLOAT(OPAQUE_32, 1)` -- ordinal 50.
    Rg11b10Float = 50,
    /// `D32_FLOAT(OPAQUE_32, 1)` -- ordinal 51.
    D32Float = 51,
    /// `D32_FLOAT_S8_UINT(OPAQUE_64, 1)` -- ordinal 52.
    D32FloatS8Uint = 52,
    /// `D24_UNORM_S8_UINT(OPAQUE_32, 1)` -- ordinal 53.
    D24UnormS8Uint = 53,
    /// `D16_UNORM(OPAQUE_16, 1)` -- ordinal 54.
    D16Unorm = 54,
    /// `S8_UINT(OPAQUE_8, 1)` -- ordinal 55.
    S8Uint = 55,
}

impl GpuFormat {
    /// Java: `GpuFormat.values()`.
    pub const VALUES: [GpuFormat; 56] = [
        GpuFormat::R8Unorm,
        GpuFormat::R8Snorm,
        GpuFormat::Rg8Unorm,
        GpuFormat::Rg8Snorm,
        GpuFormat::Rgb8Unorm,
        GpuFormat::Rgb8Snorm,
        GpuFormat::Rgba8Unorm,
        GpuFormat::Rgba8Snorm,
        GpuFormat::R16Unorm,
        GpuFormat::R16Snorm,
        GpuFormat::Rg16Unorm,
        GpuFormat::Rg16Snorm,
        GpuFormat::Rgb16Unorm,
        GpuFormat::Rgb16Snorm,
        GpuFormat::Rgba16Unorm,
        GpuFormat::Rgba16Snorm,
        GpuFormat::R8Uint,
        GpuFormat::R8Sint,
        GpuFormat::Rg8Uint,
        GpuFormat::Rg8Sint,
        GpuFormat::Rgb8Uint,
        GpuFormat::Rgb8Sint,
        GpuFormat::Rgba8Uint,
        GpuFormat::Rgba8Sint,
        GpuFormat::R16Uint,
        GpuFormat::R16Sint,
        GpuFormat::Rg16Uint,
        GpuFormat::Rg16Sint,
        GpuFormat::Rgb16Uint,
        GpuFormat::Rgb16Sint,
        GpuFormat::Rgba16Uint,
        GpuFormat::Rgba16Sint,
        GpuFormat::R32Uint,
        GpuFormat::R32Sint,
        GpuFormat::Rg32Uint,
        GpuFormat::Rg32Sint,
        GpuFormat::Rgb32Uint,
        GpuFormat::Rgb32Sint,
        GpuFormat::Rgba32Uint,
        GpuFormat::Rgba32Sint,
        GpuFormat::R16Float,
        GpuFormat::Rg16Float,
        GpuFormat::Rgb16Float,
        GpuFormat::Rgba16Float,
        GpuFormat::R32Float,
        GpuFormat::Rg32Float,
        GpuFormat::Rgb32Float,
        GpuFormat::Rgba32Float,
        GpuFormat::Rgb10a2Unorm,
        GpuFormat::Rgb10a2Uint,
        GpuFormat::Rg11b10Float,
        GpuFormat::D32Float,
        GpuFormat::D32FloatS8Uint,
        GpuFormat::D24UnormS8Uint,
        GpuFormat::D16Unorm,
        GpuFormat::S8Uint,
    ];

    /// Java: `ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            GpuFormat::R8Unorm => "R8_UNORM",
            GpuFormat::R8Snorm => "R8_SNORM",
            GpuFormat::Rg8Unorm => "RG8_UNORM",
            GpuFormat::Rg8Snorm => "RG8_SNORM",
            GpuFormat::Rgb8Unorm => "RGB8_UNORM",
            GpuFormat::Rgb8Snorm => "RGB8_SNORM",
            GpuFormat::Rgba8Unorm => "RGBA8_UNORM",
            GpuFormat::Rgba8Snorm => "RGBA8_SNORM",
            GpuFormat::R16Unorm => "R16_UNORM",
            GpuFormat::R16Snorm => "R16_SNORM",
            GpuFormat::Rg16Unorm => "RG16_UNORM",
            GpuFormat::Rg16Snorm => "RG16_SNORM",
            GpuFormat::Rgb16Unorm => "RGB16_UNORM",
            GpuFormat::Rgb16Snorm => "RGB16_SNORM",
            GpuFormat::Rgba16Unorm => "RGBA16_UNORM",
            GpuFormat::Rgba16Snorm => "RGBA16_SNORM",
            GpuFormat::R8Uint => "R8_UINT",
            GpuFormat::R8Sint => "R8_SINT",
            GpuFormat::Rg8Uint => "RG8_UINT",
            GpuFormat::Rg8Sint => "RG8_SINT",
            GpuFormat::Rgb8Uint => "RGB8_UINT",
            GpuFormat::Rgb8Sint => "RGB8_SINT",
            GpuFormat::Rgba8Uint => "RGBA8_UINT",
            GpuFormat::Rgba8Sint => "RGBA8_SINT",
            GpuFormat::R16Uint => "R16_UINT",
            GpuFormat::R16Sint => "R16_SINT",
            GpuFormat::Rg16Uint => "RG16_UINT",
            GpuFormat::Rg16Sint => "RG16_SINT",
            GpuFormat::Rgb16Uint => "RGB16_UINT",
            GpuFormat::Rgb16Sint => "RGB16_SINT",
            GpuFormat::Rgba16Uint => "RGBA16_UINT",
            GpuFormat::Rgba16Sint => "RGBA16_SINT",
            GpuFormat::R32Uint => "R32_UINT",
            GpuFormat::R32Sint => "R32_SINT",
            GpuFormat::Rg32Uint => "RG32_UINT",
            GpuFormat::Rg32Sint => "RG32_SINT",
            GpuFormat::Rgb32Uint => "RGB32_UINT",
            GpuFormat::Rgb32Sint => "RGB32_SINT",
            GpuFormat::Rgba32Uint => "RGBA32_UINT",
            GpuFormat::Rgba32Sint => "RGBA32_SINT",
            GpuFormat::R16Float => "R16_FLOAT",
            GpuFormat::Rg16Float => "RG16_FLOAT",
            GpuFormat::Rgb16Float => "RGB16_FLOAT",
            GpuFormat::Rgba16Float => "RGBA16_FLOAT",
            GpuFormat::R32Float => "R32_FLOAT",
            GpuFormat::Rg32Float => "RG32_FLOAT",
            GpuFormat::Rgb32Float => "RGB32_FLOAT",
            GpuFormat::Rgba32Float => "RGBA32_FLOAT",
            GpuFormat::Rgb10a2Unorm => "RGB10A2_UNORM",
            GpuFormat::Rgb10a2Uint => "RGB10A2_UINT",
            GpuFormat::Rg11b10Float => "RG11B10_FLOAT",
            GpuFormat::D32Float => "D32_FLOAT",
            GpuFormat::D32FloatS8Uint => "D32_FLOAT_S8_UINT",
            GpuFormat::D24UnormS8Uint => "D24_UNORM_S8_UINT",
            GpuFormat::D16Unorm => "D16_UNORM",
            GpuFormat::S8Uint => "S8_UINT",
        }
    }

    /// Port of `componentType()`.
    pub const fn component_type(&self) -> ComponentType {
        match self {
            GpuFormat::R8Unorm => ComponentType::Unorm8,
            GpuFormat::R8Snorm => ComponentType::Snorm8,
            GpuFormat::Rg8Unorm => ComponentType::Unorm8,
            GpuFormat::Rg8Snorm => ComponentType::Snorm8,
            GpuFormat::Rgb8Unorm => ComponentType::Unorm8,
            GpuFormat::Rgb8Snorm => ComponentType::Snorm8,
            GpuFormat::Rgba8Unorm => ComponentType::Unorm8,
            GpuFormat::Rgba8Snorm => ComponentType::Snorm8,
            GpuFormat::R16Unorm => ComponentType::Unorm16,
            GpuFormat::R16Snorm => ComponentType::Snorm16,
            GpuFormat::Rg16Unorm => ComponentType::Unorm16,
            GpuFormat::Rg16Snorm => ComponentType::Snorm16,
            GpuFormat::Rgb16Unorm => ComponentType::Unorm16,
            GpuFormat::Rgb16Snorm => ComponentType::Snorm16,
            GpuFormat::Rgba16Unorm => ComponentType::Unorm16,
            GpuFormat::Rgba16Snorm => ComponentType::Snorm16,
            GpuFormat::R8Uint => ComponentType::Uint8,
            GpuFormat::R8Sint => ComponentType::Sint8,
            GpuFormat::Rg8Uint => ComponentType::Uint8,
            GpuFormat::Rg8Sint => ComponentType::Sint8,
            GpuFormat::Rgb8Uint => ComponentType::Uint8,
            GpuFormat::Rgb8Sint => ComponentType::Sint8,
            GpuFormat::Rgba8Uint => ComponentType::Uint8,
            GpuFormat::Rgba8Sint => ComponentType::Sint8,
            GpuFormat::R16Uint => ComponentType::Uint16,
            GpuFormat::R16Sint => ComponentType::Sint16,
            GpuFormat::Rg16Uint => ComponentType::Uint16,
            GpuFormat::Rg16Sint => ComponentType::Sint16,
            GpuFormat::Rgb16Uint => ComponentType::Uint16,
            GpuFormat::Rgb16Sint => ComponentType::Sint16,
            GpuFormat::Rgba16Uint => ComponentType::Uint16,
            GpuFormat::Rgba16Sint => ComponentType::Sint16,
            GpuFormat::R32Uint => ComponentType::Uint32,
            GpuFormat::R32Sint => ComponentType::Sint32,
            GpuFormat::Rg32Uint => ComponentType::Uint32,
            GpuFormat::Rg32Sint => ComponentType::Sint32,
            GpuFormat::Rgb32Uint => ComponentType::Uint32,
            GpuFormat::Rgb32Sint => ComponentType::Sint32,
            GpuFormat::Rgba32Uint => ComponentType::Uint32,
            GpuFormat::Rgba32Sint => ComponentType::Sint32,
            GpuFormat::R16Float => ComponentType::Float16,
            GpuFormat::Rg16Float => ComponentType::Float16,
            GpuFormat::Rgb16Float => ComponentType::Float16,
            GpuFormat::Rgba16Float => ComponentType::Float16,
            GpuFormat::R32Float => ComponentType::Float32,
            GpuFormat::Rg32Float => ComponentType::Float32,
            GpuFormat::Rgb32Float => ComponentType::Float32,
            GpuFormat::Rgba32Float => ComponentType::Float32,
            GpuFormat::Rgb10a2Unorm => ComponentType::Opaque32,
            GpuFormat::Rgb10a2Uint => ComponentType::Opaque32,
            GpuFormat::Rg11b10Float => ComponentType::Opaque32,
            GpuFormat::D32Float => ComponentType::Opaque32,
            GpuFormat::D32FloatS8Uint => ComponentType::Opaque64,
            GpuFormat::D24UnormS8Uint => ComponentType::Opaque32,
            GpuFormat::D16Unorm => ComponentType::Opaque16,
            GpuFormat::S8Uint => ComponentType::Opaque8,
        }
    }

    /// Port of `componentCount()`.
    pub const fn component_count(&self) -> i32 {
        match self {
            GpuFormat::R8Unorm => 1,
            GpuFormat::R8Snorm => 1,
            GpuFormat::Rg8Unorm => 2,
            GpuFormat::Rg8Snorm => 2,
            GpuFormat::Rgb8Unorm => 3,
            GpuFormat::Rgb8Snorm => 3,
            GpuFormat::Rgba8Unorm => 4,
            GpuFormat::Rgba8Snorm => 4,
            GpuFormat::R16Unorm => 1,
            GpuFormat::R16Snorm => 1,
            GpuFormat::Rg16Unorm => 2,
            GpuFormat::Rg16Snorm => 2,
            GpuFormat::Rgb16Unorm => 3,
            GpuFormat::Rgb16Snorm => 3,
            GpuFormat::Rgba16Unorm => 4,
            GpuFormat::Rgba16Snorm => 4,
            GpuFormat::R8Uint => 1,
            GpuFormat::R8Sint => 1,
            GpuFormat::Rg8Uint => 2,
            GpuFormat::Rg8Sint => 2,
            GpuFormat::Rgb8Uint => 3,
            GpuFormat::Rgb8Sint => 3,
            GpuFormat::Rgba8Uint => 4,
            GpuFormat::Rgba8Sint => 4,
            GpuFormat::R16Uint => 1,
            GpuFormat::R16Sint => 1,
            GpuFormat::Rg16Uint => 2,
            GpuFormat::Rg16Sint => 2,
            GpuFormat::Rgb16Uint => 3,
            GpuFormat::Rgb16Sint => 3,
            GpuFormat::Rgba16Uint => 4,
            GpuFormat::Rgba16Sint => 4,
            GpuFormat::R32Uint => 1,
            GpuFormat::R32Sint => 1,
            GpuFormat::Rg32Uint => 2,
            GpuFormat::Rg32Sint => 2,
            GpuFormat::Rgb32Uint => 3,
            GpuFormat::Rgb32Sint => 3,
            GpuFormat::Rgba32Uint => 4,
            GpuFormat::Rgba32Sint => 4,
            GpuFormat::R16Float => 1,
            GpuFormat::Rg16Float => 2,
            GpuFormat::Rgb16Float => 3,
            GpuFormat::Rgba16Float => 4,
            GpuFormat::R32Float => 1,
            GpuFormat::Rg32Float => 2,
            GpuFormat::Rgb32Float => 3,
            GpuFormat::Rgba32Float => 4,
            GpuFormat::Rgb10a2Unorm => 1,
            GpuFormat::Rgb10a2Uint => 1,
            GpuFormat::Rg11b10Float => 1,
            GpuFormat::D32Float => 1,
            GpuFormat::D32FloatS8Uint => 1,
            GpuFormat::D24UnormS8Uint => 1,
            GpuFormat::D16Unorm => 1,
            GpuFormat::S8Uint => 1,
        }
    }

    /// Port of `blockSize()`: `componentType.byteSize() * componentCount`.
    pub const fn block_size(&self) -> i32 {
        self.component_type().byte_size() * self.component_count()
    }

    /// Port of `byteAlignment()`: `componentType.byteSize()`.
    pub const fn byte_alignment(&self) -> i32 {
        self.component_type().byte_size()
    }

    /// Port of `hasColorAspect()`: neither depth nor stencil.
    pub const fn has_color_aspect(&self) -> bool {
        !self.has_depth_aspect() && !self.has_stencil_aspect()
    }

    /// Port of `hasDepthAspect()`.
    pub const fn has_depth_aspect(&self) -> bool {
        matches!(
            self,
            GpuFormat::D32Float
                | GpuFormat::D32FloatS8Uint
                | GpuFormat::D24UnormS8Uint
                | GpuFormat::D16Unorm
        )
    }

    /// Port of `hasStencilAspect()`.
    pub const fn has_stencil_aspect(&self) -> bool {
        matches!(
            self,
            GpuFormat::S8Uint | GpuFormat::D32FloatS8Uint | GpuFormat::D24UnormS8Uint
        )
    }
}

impl std::fmt::Display for GpuFormat {
    /// Java: `Enum#toString()`, which `GpuFormat` does not override, is `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
