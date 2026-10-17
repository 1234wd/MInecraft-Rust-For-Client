//! Port of: com/mojang/blaze3d/buffers/Std140SizeCalculator.java
//! Java class(es): com.mojang.blaze3d.buffers.Std140SizeCalculator
//! Status: VERIFIED
//!
//! Session 22. `public class Std140SizeCalculator`: accumulates the byte size of a GLSL `std140`
//! uniform block, one member at a time. Every `put*` aligns the running size first, then adds the
//! member's size, and returns `this` for chaining (`&mut Self` here).
//!
//! | member | align | size |
//! |---|---|---|
//! | `float`, `int` | 4 | 4 |
//! | `vec2`, `ivec2` | 8 | 8 |
//! | `vec3`, `ivec3` | 16 | **16** (padded, as std140 requires) |
//! | `vec4`, `ivec4` | 16 | 16 |
//! | `mat4` | 16 | 64 |
//!
//! `align(n)` is `Mth.roundToward(size, n)`: round UP to a multiple of `n`. Nothing checks that
//! `n` is a power of two, so `align(3)` really rounds to a multiple of 3 (the probe has one).
//!
//! Verified by the `STD140` group of `small_probe.txt` (`tests/session22.rs`): seven scripts,
//! the size after every step.

use crate::net::minecraft::util::Mth::Mth;

/// Port of `com.mojang.blaze3d.buffers.Std140SizeCalculator`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Std140SizeCalculator {
    size: i32,
}

impl Std140SizeCalculator {
    /// Port of the implicit constructor: size 0.
    pub const fn new() -> Self {
        Std140SizeCalculator { size: 0 }
    }

    /// Port of `get()`.
    pub const fn get(&self) -> i32 {
        self.size
    }

    /// Port of `align(int alignment)`: `size = Mth.roundToward(size, alignment)`.
    pub fn align(&mut self, alignment: i32) -> &mut Self {
        self.size = Mth::round_toward_i32(self.size, alignment);
        self
    }

    fn put(&mut self, alignment: i32, size: i32) -> &mut Self {
        self.align(alignment);
        self.size = self.size.wrapping_add(size);
        self
    }

    /// Port of `putFloat()`.
    pub fn put_float(&mut self) -> &mut Self {
        self.put(4, 4)
    }

    /// Port of `putInt()`.
    pub fn put_int(&mut self) -> &mut Self {
        self.put(4, 4)
    }

    /// Port of `putVec2()`.
    pub fn put_vec2(&mut self) -> &mut Self {
        self.put(8, 8)
    }

    /// Port of `putIVec2()`.
    pub fn put_ivec2(&mut self) -> &mut Self {
        self.put(8, 8)
    }

    /// Port of `putVec3()`.
    pub fn put_vec3(&mut self) -> &mut Self {
        self.put(16, 16)
    }

    /// Port of `putIVec3()`.
    pub fn put_ivec3(&mut self) -> &mut Self {
        self.put(16, 16)
    }

    /// Port of `putVec4()`.
    pub fn put_vec4(&mut self) -> &mut Self {
        self.put(16, 16)
    }

    /// Port of `putIVec4()`.
    pub fn put_ivec4(&mut self) -> &mut Self {
        self.put(16, 16)
    }

    /// Port of `putMat4f()`.
    pub fn put_mat4f(&mut self) -> &mut Self {
        self.put(16, 64)
    }
}
