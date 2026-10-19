//! Port of: net/minecraft/world/level/PotentialCalculator.java
//! Java class(es): net.minecraft.world.level.PotentialCalculator, net.minecraft.world.level.PotentialCalculator.PointCharge
//! Status: VERIFIED
//!
//! Session 24. `public class PotentialCalculator`: the spawn "charge" field `NaturalSpawner` uses
//! to keep mobs of a capped category apart -- a list of point charges and the potential energy a
//! new charge would add at a position.
//!
//! ```java
//! public void addCharge(BlockPos pos, double charge) {
//!     if (charge != 0.0) charges.add(new PointCharge(pos, charge));
//! }
//! public double getPotentialEnergyChange(BlockPos pos, double charge) {
//!     if (charge == 0.0) return 0.0;
//!     double potentialChange = 0.0;
//!     for (PointCharge point : charges) potentialChange += point.getPotentialChange(pos);
//!     return potentialChange * charge;
//! }
//! // PointCharge
//! double distSqr = this.pos.distSqr(pos);
//! return distSqr == 0.0 ? Double.POSITIVE_INFINITY : this.charge / Math.sqrt(distSqr);
//! ```
//!
//! * `charge != 0.0` is a `double` comparison: `-0.0` is NOT added, NaN IS added (and makes every
//!   later energy NaN); `charge == 0.0` makes `-0.0` return `0.0`, and NaN goes on to multiply;
//! * the sum is taken in insertion order, so the float rounding is Java's; the arithmetic goes
//!   through `java_lang`'s `add_f64`/`mul_f64`/`div_f64`, which also give a NaN the JVM's bits;
//! * `distSqr` is `Vec3i.distSqr(Vec3i)` -- corner to corner, integer coordinates widened to
//!   `double` -- which `BlockPos` inherits; the port reaches it through [`Vec3i`];
//! * a point charge AT the queried position contributes `+Infinity` WHATEVER its sign (the
//!   ternary returns before dividing). The total is then `+/-Infinity` by the sign of the new
//!   charge, or NaN when another stored charge is `-Infinity` (`Infinity + -Infinity`).
//!
//! Verified by the `POTCALC` group of `sweep_probe.txt` (`tests/session24.rs`).

use crate::javacompat::java_lang;
use crate::net::minecraft::core::BlockPos::BlockPos;
use crate::net::minecraft::core::Vec3i::Vec3i;

/// Port of the private static nested class `PotentialCalculator.PointCharge`.
#[derive(Debug, Clone, Copy)]
struct PointCharge {
    pos: BlockPos,
    charge: f64,
}

impl PointCharge {
    /// Port of `public double getPotentialChange(BlockPos pos)`.
    fn get_potential_change(&self, pos: &BlockPos) -> f64 {
        let here = Vec3i::new(self.pos.get_x(), self.pos.get_y(), self.pos.get_z());
        let there = Vec3i::new(pos.get_x(), pos.get_y(), pos.get_z());
        let dist_sqr = here.dist_sqr(&there);
        if dist_sqr == 0.0 {
            f64::INFINITY
        } else {
            java_lang::div_f64(self.charge, java_lang::sqrt_f64(dist_sqr))
        }
    }
}

/// Port of `net.minecraft.world.level.PotentialCalculator`.
#[derive(Debug, Clone, Default)]
pub struct PotentialCalculator {
    charges: Vec<PointCharge>,
}

impl PotentialCalculator {
    /// Port of the implicit `PotentialCalculator()`: no charges.
    pub fn new() -> Self {
        Self::default()
    }

    /// Port of `public void addCharge(BlockPos pos, double charge)`.
    pub fn add_charge(&mut self, pos: BlockPos, charge: f64) {
        if charge != 0.0 {
            self.charges.push(PointCharge { pos, charge });
        }
    }

    /// Port of `public double getPotentialEnergyChange(BlockPos pos, double charge)`.
    pub fn get_potential_energy_change(&self, pos: &BlockPos, charge: f64) -> f64 {
        if charge == 0.0 {
            return 0.0;
        }
        let mut potential_change = 0.0;
        for point in &self.charges {
            potential_change =
                java_lang::add_f64(potential_change, point.get_potential_change(pos));
        }
        java_lang::mul_f64(potential_change, charge)
    }
}
