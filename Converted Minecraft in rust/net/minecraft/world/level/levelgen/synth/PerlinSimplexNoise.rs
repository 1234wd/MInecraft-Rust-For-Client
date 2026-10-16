//! Port of: net/minecraft/world/level/levelgen/synth/PerlinSimplexNoise.java
//! Java class(es): net.minecraft.world.level.levelgen.synth.PerlinSimplexNoise
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Verified bit-for-bit against the real 26.2 jar by
//! `_porting/tests/session19_i.rs` replaying the `perlinSimplex.value` rows of
//! `_porting/test-data/noise_probe.txt`, including octave sets with POSITIVE octaves, which take
//! the second random source described below.
//!
//! # POSITIVE OCTAVES ARE SEEDED FROM THE ZERO OCTAVE'S OWN VALUE
//!
//! ```java
//! long positiveOctaveSeed = (long)(zeroOctave.getValue(zeroOctave.xo, zeroOctave.yo, zeroOctave.zo) * 9.223372E18F);
//! RandomSource highFreqRandom = new WorldgenRandom(new LegacyRandomSource(positiveOctaveSeed));
//! ```
//!
//! `9.223372E18F` is a FLOAT literal widened to double, and the `(long)` cast saturates (Rust's
//! `as i64` saturates the same way, NaN included). The zero octave is constructed, and consumes
//! randomness, even when octave 0 is not in the set.
//!
//! # `+ 0.0` IS NOT A NO-OP
//!
//! `x * factor + (useNoiseStart ? noiseLevel.xo : 0.0)` always adds; `-0.0 + 0.0` is `+0.0`. The
//! port keeps the addition rather than branching around it.

use std::collections::BTreeSet;

use crate::net::minecraft::util::RandomSource::RandomSource;
use crate::net::minecraft::world::level::levelgen::synth::PerlinNoise::pow2;
use crate::net::minecraft::world::level::levelgen::synth::SimplexNoise::SimplexNoise;
use crate::net::minecraft::world::level::levelgen::LegacyRandomSource::LegacyRandomSource;
use crate::net::minecraft::world::level::levelgen::WorldgenRandom::WorldgenRandom;

/// Port of `net.minecraft.world.level.levelgen.synth.PerlinSimplexNoise`.
#[derive(Clone, Debug, PartialEq)]
pub struct PerlinSimplexNoise {
    noise_levels: Vec<Option<SimplexNoise>>,
    highest_freq_value_factor: f64,
    highest_freq_input_factor: f64,
}

#[cold]
#[inline(never)]
fn illegal_argument(message: &str) -> ! {
    panic!("java.lang.IllegalArgumentException: {message}")
}

impl PerlinSimplexNoise {
    /// Port of `PerlinSimplexNoise(RandomSource, List<Integer>)`: the octaves form a sorted set.
    pub fn new(random: &mut dyn RandomSource, octave_set: impl IntoIterator<Item = i32>) -> PerlinSimplexNoise {
        let octave_set: BTreeSet<i32> = octave_set.into_iter().collect();
        let (Some(&first), Some(&last)) = (octave_set.first(), octave_set.last()) else {
            illegal_argument("Need some octaves!");
        };
        let low_freq_octaves = first.wrapping_neg();
        let high_freq_octaves = last;
        let octaves = low_freq_octaves.wrapping_add(high_freq_octaves).wrapping_add(1);
        if octaves < 1 {
            illegal_argument("Total number of octaves needs to be >= 1");
        }
        let zero_octave = SimplexNoise::new(random);
        let zero_octave_index = high_freq_octaves;
        let mut noise_levels: Vec<Option<SimplexNoise>> = vec![None; octaves as usize];
        if zero_octave_index >= 0 && zero_octave_index < octaves && octave_set.contains(&0) {
            noise_levels[zero_octave_index as usize] = Some(zero_octave.clone());
        }
        let mut i = zero_octave_index.wrapping_add(1);
        while i < octaves {
            if i >= 0 && octave_set.contains(&zero_octave_index.wrapping_sub(i)) {
                noise_levels[i as usize] = Some(SimplexNoise::new(random));
            } else {
                random.consume_count(262);
            }
            i += 1;
        }
        if high_freq_octaves > 0 {
            let positive_octave_seed =
                (zero_octave.get_value_3d(zero_octave.xo, zero_octave.yo, zero_octave.zo) * (9.223372E18f32 as f64)) as i64;
            let mut high_freq_random = WorldgenRandom::new(Box::new(LegacyRandomSource::new(positive_octave_seed)));
            let mut i = zero_octave_index - 1;
            while i >= 0 {
                if i < octaves && octave_set.contains(&zero_octave_index.wrapping_sub(i)) {
                    noise_levels[i as usize] = Some(SimplexNoise::new(&mut high_freq_random));
                } else {
                    high_freq_random.consume_count(262);
                }
                i -= 1;
            }
        }
        PerlinSimplexNoise {
            noise_levels,
            highest_freq_input_factor: pow2(high_freq_octaves),
            highest_freq_value_factor: 1.0 / (pow2(octaves) - 1.0),
        }
    }

    /// Port of `PerlinSimplexNoise#getValue(double, double, boolean)`.
    pub fn get_value(&self, x: f64, y: f64, use_noise_start: bool) -> f64 {
        let mut value = 0.0;
        let mut factor = self.highest_freq_input_factor;
        let mut value_factor = self.highest_freq_value_factor;
        for level in &self.noise_levels {
            if let Some(noise) = level {
                value += noise.get_value_2d(
                    x * factor + if use_noise_start { noise.xo } else { 0.0 },
                    y * factor + if use_noise_start { noise.yo } else { 0.0 },
                ) * value_factor;
            }
            factor /= 2.0;
            value_factor *= 2.0;
        }
        value
    }
}
