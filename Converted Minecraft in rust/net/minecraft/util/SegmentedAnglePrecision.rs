//! Port of: net/minecraft/util/SegmentedAnglePrecision.java
//! Java class(es): net.minecraft.util.SegmentedAnglePrecision
//! Status: VERIFIED
//!
//! Session 22. `public class SegmentedAnglePrecision`: binary angles with `2^bitPrecision` steps
//! per full turn, with the conversions to and from degrees. `RotationSegment` holds the one vanilla
//! instance, `new SegmentedAnglePrecision(4)` -- sixteen steps.
//!
//! # Construction
//!
//! `bitPrecision` must be in `2..=30`, else `IllegalArgumentException` with
//! `"Precision cannot be less than 2 bits"` / `"Precision cannot be greater than 30 bits"` -- the
//! port panics with that exact text. `twoPi = 1 << bitPrecision`, `mask = twoPi - 1`, and the two
//! scale factors are `float`s computed from the `int` (`twoPi / 360.0F`, `360.0F / twoPi`).
//!
//! # Conversions
//!
//! * `fromDegreesWithTurns(deg)` is `Math.round(deg * degreeToAngle)` -- Java's `Math.round(float)`,
//!   ties toward positive infinity, NaN to 0, saturating -- which is
//!   `javacompat::java_lang::round_f32`, NOT `f32::round` (that rounds ties away from zero).
//! * `fromDegrees` and `toDegrees` reduce with `& mask`, so they work for any `int`, negative or not.
//! * `toDegrees` maps `[180, 360)` to `[-180, 0)` with a `float` subtraction.
//! * `fromDirection` is `0` for UP/DOWN, else `get2DDataValue() << (precision - 2)`.
//! * `isSameAxis(a, b)` compares the two angles modulo a HALF turn (`mask >> 1`).
//!
//! Verified by the `SEGCTOR`, `SEGDIR`, `SEGDEG` and `SEGANG` groups of `small_probe.txt`
//! (`tests/session22.rs`): every precision from -1 to 32 through the constructor, and seven
//! precisions through every method with 21 degree values (NaN, infinity, `1e9`) and 21 angles
//! (both `int` extremes), including the full `isSameAxis` matrix.

use crate::javacompat::java_lang;
use crate::net::minecraft::core::Direction::Direction;

/// Port of `net.minecraft.util.SegmentedAnglePrecision`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SegmentedAnglePrecision {
    mask: i32,
    precision: i32,
    degree_to_angle: f32,
    angle_to_degree: f32,
}

impl SegmentedAnglePrecision {
    /// Port of `new SegmentedAnglePrecision(int bitPrecision)`.
    ///
    /// # Panics
    ///
    /// With Java's `IllegalArgumentException` text when `bitPrecision` is outside `2..=30`.
    pub fn new(bit_precision: i32) -> Self {
        if bit_precision < 2 {
            panic!("java.lang.IllegalArgumentException: Precision cannot be less than 2 bits");
        }
        if bit_precision > 30 {
            panic!("java.lang.IllegalArgumentException: Precision cannot be greater than 30 bits");
        }
        let two_pi = 1i32 << bit_precision;
        SegmentedAnglePrecision {
            mask: two_pi - 1,
            precision: bit_precision,
            degree_to_angle: two_pi as f32 / 360.0,
            angle_to_degree: 360.0 / two_pi as f32,
        }
    }

    /// Port of `isSameAxis(int binaryAngleA, int binaryAngleB)`.
    pub fn is_same_axis(&self, binary_angle_a: i32, binary_angle_b: i32) -> bool {
        let semicircle_mask = self.get_mask() >> 1;
        (binary_angle_a & semicircle_mask) == (binary_angle_b & semicircle_mask)
    }

    /// Port of `fromDirection(Direction direction)`.
    pub fn from_direction(&self, direction: Direction) -> i32 {
        if direction.get_axis().is_vertical() {
            return 0;
        }
        let segmented_angle_2bit = direction.get_2d_data_value();
        segmented_angle_2bit << (self.precision - 2)
    }

    /// Port of `fromDegreesWithTurns(float degrees)`: `Math.round(degrees * degreeToAngle)`.
    pub fn from_degrees_with_turns(&self, degrees: f32) -> i32 {
        java_lang::round_f32(degrees * self.degree_to_angle)
    }

    /// Port of `fromDegrees(float degrees)`.
    pub fn from_degrees(&self, degrees: f32) -> i32 {
        self.normalize(self.from_degrees_with_turns(degrees))
    }

    /// Port of `toDegreesWithTurns(int binaryAngle)`: `binaryAngle * angleToDegree` in `float`.
    pub fn to_degrees_with_turns(&self, binary_angle: i32) -> f32 {
        binary_angle as f32 * self.angle_to_degree
    }

    /// Port of `toDegrees(int binaryAngle)`: into `[-180, 180)`.
    pub fn to_degrees(&self, binary_angle: i32) -> f32 {
        let degrees = self.to_degrees_with_turns(self.normalize(binary_angle));
        if degrees >= 180.0 {
            degrees - 360.0
        } else {
            degrees
        }
    }

    /// Port of `normalize(int binaryAngle)`: `binaryAngle & mask`.
    pub const fn normalize(&self, binary_angle: i32) -> i32 {
        binary_angle & self.mask
    }

    /// Port of `getMask()`.
    pub const fn get_mask(&self) -> i32 {
        self.mask
    }
}
