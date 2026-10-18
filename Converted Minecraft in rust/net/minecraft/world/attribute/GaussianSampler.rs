//! Port of: net/minecraft/world/attribute/GaussianSampler.java
//! Java class(es): net.minecraft.world.attribute.GaussianSampler, net.minecraft.world.attribute.Accumulator, net.minecraft.world.attribute.Sampler
//! Status: VERIFIED
//!
//! Session 22. `public class GaussianSampler`: blends a value field over the 6 x 6 x 6 cells around a
//! position with a smooth kernel, through two nested functional interfaces -- a `Sampler<V>` that
//! reads the value at a cell and an `Accumulator<V>` that is handed each value with its weight.
//!
//! # `sample`
//!
//! The position is shifted by `-0.5` on each axis, floored to a cell (`Mth.floor`, i.e.
//! `(int) Math.floor`) and split into that cell and a fraction. For `z`, then `x`, then `y` -- that
//! loop order is the order the accumulator sees -- each in `0..6`, the axis weight is
//! `Mth.lerp(fraction, KERNEL[i + 1], KERNEL[i])` over the kernel `{0, 1, 4, 6, 4, 1, 0}`, the cell
//! is `floor - 2 + i` (plain `int`, wrapping), and the accumulator gets
//! `weightX * weightY * weightZ` (in that multiplication order) with the sampled value. All 216
//! cells are visited even when a weight is zero.
//!
//! Verified by the `GAUSS` group of `small_probe.txt` (`tests/session22.rs`): five positions
//! (signed zeros, `1e-9`, a large coordinate), the full 216-call accumulator transcript of each,
//! weights as raw bits.

use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::world::phys::Vec3::Vec3;

/// Port of `private static final double[] GAUSSIAN_SAMPLE_KERNEL`.
const GAUSSIAN_SAMPLE_KERNEL: [f64; 7] = [0.0, 1.0, 4.0, 6.0, 4.0, 1.0, 0.0];

/// Port of `GaussianSampler.Accumulator<V>`.
pub trait Accumulator<V> {
    /// Port of `void accumulate(double weight, V value)`.
    fn accumulate(&mut self, weight: f64, value: V);
}

impl<V, F: FnMut(f64, V)> Accumulator<V> for F {
    fn accumulate(&mut self, weight: f64, value: V) {
        self(weight, value)
    }
}

/// Port of `GaussianSampler.Sampler<V>`.
pub trait Sampler<V> {
    /// Port of `V get(int x, int y, int z)`.
    fn get(&mut self, x: i32, y: i32, z: i32) -> V;
}

impl<V, F: FnMut(i32, i32, i32) -> V> Sampler<V> for F {
    fn get(&mut self, x: i32, y: i32, z: i32) -> V {
        self(x, y, z)
    }
}

/// Port of `static <V> void sample(Vec3 position, Sampler<V> sampler, Accumulator<V> accumulator)`.
pub fn sample<V>(
    position: &Vec3,
    mut sampler: impl Sampler<V>,
    mut accumulator: impl Accumulator<V>,
) {
    let position = position.subtract(0.5, 0.5, 0.5);
    let integral_x = Mth::floor_f64(position.x);
    let integral_y = Mth::floor_f64(position.y);
    let integral_z = Mth::floor_f64(position.z);
    let relative_x = position.x - integral_x as f64;
    let relative_y = position.y - integral_y as f64;
    let relative_z = position.z - integral_z as f64;
    for z in 0..6usize {
        let weight_z = Mth::lerp_f64(
            relative_z,
            GAUSSIAN_SAMPLE_KERNEL[z + 1],
            GAUSSIAN_SAMPLE_KERNEL[z],
        );
        let sample_z = integral_z.wrapping_sub(2).wrapping_add(z as i32);
        for x in 0..6usize {
            let weight_x = Mth::lerp_f64(
                relative_x,
                GAUSSIAN_SAMPLE_KERNEL[x + 1],
                GAUSSIAN_SAMPLE_KERNEL[x],
            );
            let sample_x = integral_x.wrapping_sub(2).wrapping_add(x as i32);
            for y in 0..6usize {
                let weight_y = Mth::lerp_f64(
                    relative_y,
                    GAUSSIAN_SAMPLE_KERNEL[y + 1],
                    GAUSSIAN_SAMPLE_KERNEL[y],
                );
                let sample_y = integral_y.wrapping_sub(2).wrapping_add(y as i32);
                let sample_weight = weight_x * weight_y * weight_z;
                let value = sampler.get(sample_x, sample_y, sample_z);
                accumulator.accumulate(sample_weight, value);
            }
        }
    }
}
