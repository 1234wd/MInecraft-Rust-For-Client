//! Port of: com/mojang/blaze3d/vulkan/glsl/SpvcUtil.java
//! Java class(es): com.mojang.blaze3d.vulkan.glsl.SpvcUtil
//! Status: VERIFIED
//!
//! Session 22. `class SpvcUtil` (package-private, private constructor): one static helper that
//! names a SPIR-V image dimension.
//!
//! `0..=6` map to `1D`, `2D`, `3D`, `Cube`, `Rect`, `Buffer`, `SubpassData`; anything else is
//! `"0x" + Integer.toHexString(dimension)`. `toHexString` is UNSIGNED and lower-case with no
//! padding, so `-1` is `0xffffffff` and `16` is `0x10` -- `{:x}` of the value as `u32`.
//!
//! The Java class is package-private; the port is `pub` so the parity tests can reach it.
//!
//! Verified by the `SPVCDIM` group of `small_probe.txt` (`tests/session22.rs`), including
//! negative values and both `int` extremes.

/// Port of `SpvcUtil.imageDimensionToString(int dimension)`.
pub fn image_dimension_to_string(dimension: i32) -> String {
    match dimension {
        0 => "1D".to_string(),
        1 => "2D".to_string(),
        2 => "3D".to_string(),
        3 => "Cube".to_string(),
        4 => "Rect".to_string(),
        5 => "Buffer".to_string(),
        6 => "SubpassData".to_string(),
        _ => format!("0x{:x}", dimension as u32),
    }
}
