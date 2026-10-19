//! Port of: net/minecraft/world/level/block/state/properties/RotationSegment.java
//! Java class(es): net.minecraft.world.level.block.state.properties.RotationSegment
//! Status: VERIFIED
//!
//! Session 24. `public class RotationSegment`: the 16-step rotation of signs, banners and skulls
//! (the `rotation` block property, 0..15), as static helpers over a
//! `SegmentedAnglePrecision(4)`.
//!
//! | Java | here |
//! |---|---|
//! | `SEGMENTED_ANGLE16`, `MAX_SEGMENT_INDEX` (private) | `SEGMENTED_ANGLE16`, [`get_max_segment_index`] |
//! | `NORTH_0`, `EAST_90`, `SOUTH_180`, `WEST_270` (private) | the same constants |
//! | `getMaxSegmentIndex()` | [`get_max_segment_index`] |
//! | `convertToSegment(Direction)`, `convertToSegment(float)` | [`convert_to_segment_direction`], [`convert_to_segment_f32`] (`DESIGN_DECISIONS.md#overload-naming`) |
//! | `convertToDirection(int)` | [`convert_to_direction`]: `Optional<Direction>` is `Option<Direction>` |
//! | `convertToDegrees(int)` | [`convert_to_degrees`] |
//!
//! Every method delegates to [`SegmentedAnglePrecision`] (session 19, VERIFIED), so the
//! float rounding of `fromDegrees` (`Math.round(float)`, NaN to 0, saturation) is that class's.
//! `convertToDirection` maps only the four exact cardinal segments; every other int, negatives
//! and 16+ included, is empty -- the switch does not normalise.
//!
//! Verified by the `ROTSEG` group of `sweep_probe.txt` (`tests/session24.rs`).

use std::sync::LazyLock;

use crate::net::minecraft::core::Direction::Direction;
use crate::net::minecraft::util::SegmentedAnglePrecision::SegmentedAnglePrecision;

/// Port of `private static final SegmentedAnglePrecision SEGMENTED_ANGLE16 = new SegmentedAnglePrecision(4)`.
static SEGMENTED_ANGLE16: LazyLock<SegmentedAnglePrecision> =
    LazyLock::new(|| SegmentedAnglePrecision::new(4));

/// Port of `private static final int NORTH_0 = 0`.
const NORTH_0: i32 = 0;
/// Port of `private static final int EAST_90 = 4`.
const EAST_90: i32 = 4;
/// Port of `private static final int SOUTH_180 = 8`.
const SOUTH_180: i32 = 8;
/// Port of `private static final int WEST_270 = 12`.
const WEST_270: i32 = 12;

/// Port of `public static int getMaxSegmentIndex()`: `MAX_SEGMENT_INDEX`, i.e.
/// `SEGMENTED_ANGLE16.getMask()` (15).
pub fn get_max_segment_index() -> i32 {
    SEGMENTED_ANGLE16.get_mask()
}

/// Port of `public static int convertToSegment(Direction direction)`.
pub fn convert_to_segment_direction(direction: Direction) -> i32 {
    SEGMENTED_ANGLE16.from_direction(direction)
}

/// Port of `public static int convertToSegment(float rotDegrees)`.
pub fn convert_to_segment_f32(rot_degrees: f32) -> i32 {
    SEGMENTED_ANGLE16.from_degrees(rot_degrees)
}

/// Port of `public static Optional<Direction> convertToDirection(int segment)`.
pub fn convert_to_direction(segment: i32) -> Option<Direction> {
    match segment {
        NORTH_0 => Some(Direction::North),
        EAST_90 => Some(Direction::East),
        SOUTH_180 => Some(Direction::South),
        WEST_270 => Some(Direction::West),
        _ => None,
    }
}

/// Port of `public static float convertToDegrees(int segment)`.
pub fn convert_to_degrees(segment: i32) -> f32 {
    SEGMENTED_ANGLE16.to_degrees(segment)
}
