//! Port of: net/minecraft/world/level/block/piston/PistonMath.java
//! Java class(es): net.minecraft.world.level.block.piston.PistonMath
//! Status: VERIFIED
//!
//! Session 22. `public class PistonMath`: one static helper, `getMovementArea(aabb, direction,
//! amount)` -- the slab of space a face of `aabb` sweeps through when a piston moves it.
//!
//! # Signed zeros are load-bearing
//!
//! ```java
//! double delta = amount * direction.getAxisDirection().getStep();
//! double min = Math.min(delta, 0.0);
//! double max = Math.max(delta, 0.0);
//! ```
//!
//! For a NEGATIVE direction a zero amount gives `delta = -0.0`, and Java's `Math.min(-0.0, 0.0)` is
//! `-0.0`, which then survives the `aabb.minZ + min` add when `minZ` is itself `-0.0`. Rust's
//! `f64::min` does not order the zeros, so this uses `javacompat::java_lang::min_f64`/`max_f64`.
//! NaN amounts propagate through both. The resulting box goes through `new AABB(...)`, which
//! re-sorts each axis.
//!
//! A NaN result's SIGN is not part of the contract, in either language. HotSpot computes
//! `NaN * -1` with a multiply and keeps `7ff8...`; release-mode Rust folds `amount * -1.0` into a
//! negation and returns `fff8...`. Java code can only tell them apart through
//! `doubleToRawLongBits` (`AABB.equals`/`hashCode` canonicalise), so the test treats every NaN
//! corner as one NaN. Found when the session-22 release run failed where debug passed.
//!
//! The switch reads the MIN face for `WEST`/`DOWN`/`NORTH` and the MAX face for
//! `EAST`/`UP`/`SOUTH`; `UP` is also the `default` arm, unreachable for a non-null direction.
//!
//! Verified by the `PISTON` group of `small_probe.txt` (`tests/session22.rs`): three boxes (one
//! all `-0.0`) x six directions x seven amounts (both zeros, a subnormal-adjacent `1e-300`, NaN),
//! all six corners as raw bits, NaNs aside.

use crate::javacompat::java_lang;
use crate::net::minecraft::core::Direction::Direction;
use crate::net::minecraft::world::phys::AABB::AABB;

/// Port of `PistonMath.getMovementArea(AABB aabb, Direction direction, double amount)`.
pub fn get_movement_area(aabb: &AABB, direction: Direction, amount: f64) -> AABB {
    let delta = amount * direction.get_axis_direction().get_step() as f64;
    let min = java_lang::min_f64(delta, 0.0);
    let max = java_lang::max_f64(delta, 0.0);
    match direction {
        Direction::West => AABB::new(
            aabb.min_x + min,
            aabb.min_y,
            aabb.min_z,
            aabb.min_x + max,
            aabb.max_y,
            aabb.max_z,
        ),
        Direction::East => AABB::new(
            aabb.max_x + min,
            aabb.min_y,
            aabb.min_z,
            aabb.max_x + max,
            aabb.max_y,
            aabb.max_z,
        ),
        Direction::Down => AABB::new(
            aabb.min_x,
            aabb.min_y + min,
            aabb.min_z,
            aabb.max_x,
            aabb.min_y + max,
            aabb.max_z,
        ),
        Direction::Up => AABB::new(
            aabb.min_x,
            aabb.max_y + min,
            aabb.min_z,
            aabb.max_x,
            aabb.max_y + max,
            aabb.max_z,
        ),
        Direction::North => AABB::new(
            aabb.min_x,
            aabb.min_y,
            aabb.min_z + min,
            aabb.max_x,
            aabb.max_y,
            aabb.min_z + max,
        ),
        Direction::South => AABB::new(
            aabb.min_x,
            aabb.min_y,
            aabb.max_z + min,
            aabb.max_x,
            aabb.max_y,
            aabb.max_z + max,
        ),
    }
}
