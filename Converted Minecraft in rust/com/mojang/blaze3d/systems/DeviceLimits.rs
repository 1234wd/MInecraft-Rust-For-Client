//! Port of: com/mojang/blaze3d/systems/DeviceLimits.java
//! Java class(es): com.mojang.blaze3d.systems.DeviceLimits
//! Status: VERIFIED
//!
//! Session 24. `public record DeviceLimits(int maxAnisotropy, int minUniformOffsetAlignment, int
//! maxTextureSize, long maxMemoryAllocationSize, int maxMultiDrawDirectInterleavedDrawCount, int
//! maxColorAttachments)`: what the GPU device reports, plus one derived limit.
//!
//! ```java
//! public int maxTextureSizeForFormat(GpuFormat format) {
//!     return Integer.highestOneBit(Math.min(this.maxTextureSize,
//!         (int) Math.sqrt((double) this.maxMemoryAllocationSize / format.blockSize())));
//! }
//! ```
//!
//! * `(double) long / int` widens both to `double`; `Math.sqrt` is correctly rounded on every
//!   JVM and in Rust ([`java_lang::sqrt_f64`]);
//! * `(int)` of a `double` saturates and maps NaN to 0 (`d2i`), which is Rust's `as i32`. A
//!   negative allocation size gives `sqrt` NaN, hence 0;
//! * `Integer.highestOneBit` keeps the top set bit, and of a negative int that is the sign bit
//!   (`Integer.MIN_VALUE`).
//!
//! Every component is a primitive, so the record's `equals`, `hashCode` and `toString` are
//! portable and ported: `hashCode` is the record fold (`31 * h + c`, from 0) of `Integer.hashCode`
//! and `Long.hashCode`, and `toString` is `DeviceLimits[maxAnisotropy=.., ...]`.
//!
//! Verified by the `DEVLIM` group of `sweep_probe.txt` (`tests/session24.rs`).

use crate::com::mojang::blaze3d::GpuFormat::GpuFormat;
use crate::javacompat::java_lang;
use crate::javacompat::java_object;

/// Port of `com.mojang.blaze3d.systems.DeviceLimits`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceLimits {
    max_anisotropy: i32,
    min_uniform_offset_alignment: i32,
    max_texture_size: i32,
    max_memory_allocation_size: i64,
    max_multi_draw_direct_interleaved_draw_count: i32,
    max_color_attachments: i32,
}

/// Java's `Integer.highestOneBit(int)`.
fn highest_one_bit(i: i32) -> i32 {
    // `i & (MIN_VALUE >>> numberOfLeadingZeros(i))`; 0 for 0.
    if i == 0 {
        0
    } else {
        ((i32::MIN as u32) >> i.leading_zeros()) as i32
    }
}

impl DeviceLimits {
    /// Port of the canonical constructor.
    pub const fn new(
        max_anisotropy: i32,
        min_uniform_offset_alignment: i32,
        max_texture_size: i32,
        max_memory_allocation_size: i64,
        max_multi_draw_direct_interleaved_draw_count: i32,
        max_color_attachments: i32,
    ) -> Self {
        DeviceLimits {
            max_anisotropy,
            min_uniform_offset_alignment,
            max_texture_size,
            max_memory_allocation_size,
            max_multi_draw_direct_interleaved_draw_count,
            max_color_attachments,
        }
    }

    /// Port of the record accessor `int maxAnisotropy()`.
    pub const fn max_anisotropy(&self) -> i32 {
        self.max_anisotropy
    }

    /// Port of the record accessor `int minUniformOffsetAlignment()`.
    pub const fn min_uniform_offset_alignment(&self) -> i32 {
        self.min_uniform_offset_alignment
    }

    /// Port of the record accessor `int maxTextureSize()`.
    pub const fn max_texture_size(&self) -> i32 {
        self.max_texture_size
    }

    /// Port of the record accessor `long maxMemoryAllocationSize()`.
    pub const fn max_memory_allocation_size(&self) -> i64 {
        self.max_memory_allocation_size
    }

    /// Port of the record accessor `int maxMultiDrawDirectInterleavedDrawCount()`.
    pub const fn max_multi_draw_direct_interleaved_draw_count(&self) -> i32 {
        self.max_multi_draw_direct_interleaved_draw_count
    }

    /// Port of the record accessor `int maxColorAttachments()`.
    pub const fn max_color_attachments(&self) -> i32 {
        self.max_color_attachments
    }

    /// Port of `public int maxTextureSizeForFormat(GpuFormat format)`.
    pub fn max_texture_size_for_format(&self, format: GpuFormat) -> i32 {
        let per_block = self.max_memory_allocation_size as f64 / f64::from(format.block_size());
        let side = java_lang::sqrt_f64(per_block) as i32;
        highest_one_bit(java_lang::min_i32(self.max_texture_size, side))
    }

    /// Port of the record's `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            self.max_anisotropy,
            self.min_uniform_offset_alignment,
            self.max_texture_size,
            // `Long.hashCode`: `(int) (v ^ (v >>> 32))`.
            (self.max_memory_allocation_size
                ^ ((self.max_memory_allocation_size as u64) >> 32) as i64) as i32,
            self.max_multi_draw_direct_interleaved_draw_count,
            self.max_color_attachments,
        ])
    }
}

impl std::fmt::Display for DeviceLimits {
    /// Port of the record's `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeviceLimits[maxAnisotropy={}, minUniformOffsetAlignment={}, maxTextureSize={}, \
             maxMemoryAllocationSize={}, maxMultiDrawDirectInterleavedDrawCount={}, maxColorAttachments={}]",
            self.max_anisotropy,
            self.min_uniform_offset_alignment,
            self.max_texture_size,
            self.max_memory_allocation_size,
            self.max_multi_draw_direct_interleaved_draw_count,
            self.max_color_attachments
        )
    }
}
