//! Port of: net/minecraft/world/level/levelgen/synth/PerlinNoise.java
//! Java class(es): net.minecraft.world.level.levelgen.synth.PerlinNoise
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Verified bit-for-bit against the real 26.2 jar by
//! `_porting/tests/session19_i.rs` replaying `_porting/test-data/noise_probe.txt`: the four
//! constructors (new positional initialisation, `create(random, List<Integer>)`, and both
//! deprecated legacy initialisations), `getValue` with and without the y fudge, `maxBrokenValue`,
//! `wrap`, `parityConfigString`, and the two constructor exceptions.
//!
//! # TWO INITIALISATIONS THAT CONSUME RANDOMNESS DIFFERENTLY
//!
//! * New (`useNewInitialization`): `random.forkPositional()` once, then each non-zero octave gets
//!   `positional.fromHashOf("octave_" + octave)`. Octaves are independent of each other.
//! * Legacy: the zero octave is ALWAYS constructed first (even if its amplitude is 0, so it still
//!   consumes randomness), then octaves `-1, -2, ...` in DESCENDING index order, each either built
//!   or skipped with `consumeCount(262)` -- 262 being exactly what an `ImprovedNoise` constructor
//!   draws (3 doubles = 6 ints + 256 shuffle ints). Positive octaves are rejected.
//!
//! # `Math.pow(2.0, n)`
//!
//! Only ever called with integer exponents, where the result is an exact power of two. The port
//! builds those values from their bit patterns ([`pow2`]) so no rounding question arises.
//!
//! # `wrap`
//!
//! Folds coordinates into `[-2^24, 2^24]` (`3.3554432E7 = 2^25`) so the octave lattice never
//! loses precision far from the origin. `Mth.lfloor` returns a `long`, multiplied as a double.

use std::collections::BTreeSet;

use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::util::RandomSource::RandomSource;
use crate::net::minecraft::world::level::levelgen::synth::ImprovedNoise::{java_format_fixed, ImprovedNoise};

/// Java: `private static final int ROUND_OFF = 33554432`.
pub const ROUND_OFF: i32 = 33554432;

/// `Math.pow(2.0, e)` for an integer exponent: exact.
pub(crate) fn pow2(e: i32) -> f64 {
    match e {
        -1074..=-1023 => f64::from_bits(1u64 << (e + 1074)),
        -1022..=1023 => {
            f64::from_bits(((e + 1023) as u64) << 52)
        }
        e if e < -1074 => 0.0,
        _ => f64::INFINITY,
    }
}

#[cold]
#[inline(never)]
fn throw(class: &str, message: &str) -> ! {
    panic!("{class}: {message}")
}

/// Port of `net.minecraft.world.level.levelgen.synth.PerlinNoise`.
#[derive(Clone, Debug, PartialEq)]
pub struct PerlinNoise {
    noise_levels: Vec<Option<ImprovedNoise>>,
    first_octave: i32,
    amplitudes: Vec<f64>,
    lowest_freq_value_factor: f64,
    lowest_freq_input_factor: f64,
    max_value: f64,
}

/// Port of the private `PerlinNoise#makeAmplitudes(IntSortedSet)`.
///
/// Returns `(firstOctave, amplitudes)` with amplitude `1.0` at every listed octave.
pub(crate) fn make_amplitudes(octave_set: &BTreeSet<i32>) -> (i32, Vec<f64>) {
    let (Some(&first), Some(&last)) = (octave_set.first(), octave_set.last()) else {
        throw("java.lang.IllegalArgumentException", "Need some octaves!");
    };
    let low_freq_octaves = first.wrapping_neg();
    let high_freq_octaves = last;
    let octaves = low_freq_octaves.wrapping_add(high_freq_octaves).wrapping_add(1);
    if octaves < 1 {
        throw("java.lang.IllegalArgumentException", "Total number of octaves needs to be >= 1");
    }
    let mut amplitudes = vec![0.0f64; octaves as usize];
    for &octave in octave_set {
        amplitudes[(octave + low_freq_octaves) as usize] = 1.0;
    }
    (low_freq_octaves.wrapping_neg(), amplitudes)
}

impl PerlinNoise {
    /// Port of the deprecated `PerlinNoise#createLegacyForBlendedNoise(RandomSource, IntStream)`.
    pub fn create_legacy_for_blended_noise(random: &mut dyn RandomSource, octaves: impl IntoIterator<Item = i32>) -> PerlinNoise {
        let set: BTreeSet<i32> = octaves.into_iter().collect();
        PerlinNoise::new(random, make_amplitudes(&set), false)
    }

    /// Port of the deprecated `PerlinNoise#createLegacyForLegacyNetherBiome(RandomSource, int, DoubleList)`.
    pub fn create_legacy_for_legacy_nether_biome(random: &mut dyn RandomSource, first_octave: i32, amplitudes: Vec<f64>) -> PerlinNoise {
        PerlinNoise::new(random, (first_octave, amplitudes), false)
    }

    /// Port of `PerlinNoise#create(RandomSource, IntStream)` and `create(RandomSource,
    /// List<Integer>)`: the octaves form a SORTED SET, so order and duplicates do not matter.
    pub fn create_from_octaves(random: &mut dyn RandomSource, octave_set: impl IntoIterator<Item = i32>) -> PerlinNoise {
        let set: BTreeSet<i32> = octave_set.into_iter().collect();
        PerlinNoise::new(random, make_amplitudes(&set), true)
    }

    /// Port of `PerlinNoise#create(RandomSource, int, double, double...)`.
    pub fn create(random: &mut dyn RandomSource, first_octave: i32, first_amplitude: f64, amplitudes: &[f64]) -> PerlinNoise {
        let mut list = Vec::with_capacity(amplitudes.len() + 1);
        list.push(first_amplitude);
        list.extend_from_slice(amplitudes);
        PerlinNoise::new(random, (first_octave, list), true)
    }

    /// Port of `PerlinNoise#create(RandomSource, int, DoubleList)`.
    pub fn create_with_amplitudes(random: &mut dyn RandomSource, first_octave: i32, amplitudes: Vec<f64>) -> PerlinNoise {
        PerlinNoise::new(random, (first_octave, amplitudes), true)
    }

    /// Port of the protected constructor `PerlinNoise(RandomSource, Pair<Integer, DoubleList>, boolean)`.
    pub fn new(random: &mut dyn RandomSource, pair: (i32, Vec<f64>), use_new_initialization: bool) -> PerlinNoise {
        let (first_octave, amplitudes) = pair;
        let octaves = amplitudes.len() as i32;
        let zero_octave_index = first_octave.wrapping_neg();
        let mut noise_levels: Vec<Option<ImprovedNoise>> = vec![None; octaves as usize];
        if use_new_initialization {
            let positional = random.fork_positional();
            for i in 0..octaves {
                if amplitudes[i as usize] != 0.0 {
                    let octave = first_octave.wrapping_add(i);
                    let mut source = positional.from_hash_of(&format!("octave_{octave}"));
                    noise_levels[i as usize] = Some(ImprovedNoise::new(&mut *source));
                }
            }
        } else {
            let zero_octave = ImprovedNoise::new(random);
            if zero_octave_index >= 0 && zero_octave_index < octaves {
                let zero_octave_amplitude = amplitudes[zero_octave_index as usize];
                if zero_octave_amplitude != 0.0 {
                    noise_levels[zero_octave_index as usize] = Some(zero_octave);
                }
            }
            let mut i = zero_octave_index.wrapping_sub(1);
            while i >= 0 {
                if i < octaves {
                    let amplitude = amplitudes[i as usize];
                    if amplitude != 0.0 {
                        noise_levels[i as usize] = Some(ImprovedNoise::new(random));
                    } else {
                        skip_octave(random);
                    }
                } else {
                    skip_octave(random);
                }
                i -= 1;
            }
            let built = noise_levels.iter().filter(|n| n.is_some()).count();
            let non_zero = amplitudes.iter().filter(|a| **a != 0.0).count();
            if built != non_zero {
                throw(
                    "java.lang.IllegalStateException",
                    "Failed to create correct number of noise levels for given non-zero amplitudes",
                );
            }
            if zero_octave_index < octaves - 1 {
                throw("java.lang.IllegalArgumentException", "Positive octaves are temporarily disabled");
            }
        }
        let lowest_freq_input_factor = pow2(zero_octave_index.wrapping_neg());
        let lowest_freq_value_factor = pow2(octaves - 1) / (pow2(octaves) - 1.0);
        let mut noise = PerlinNoise {
            noise_levels,
            first_octave,
            amplitudes,
            lowest_freq_value_factor,
            lowest_freq_input_factor,
            max_value: 0.0,
        };
        noise.max_value = noise.edge_value(2.0);
        noise
    }

    /// Port of the protected `PerlinNoise#maxValue()`.
    pub fn max_value(&self) -> f64 {
        self.max_value
    }

    /// Port of `PerlinNoise#getValue(double, double, double)`.
    pub fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        self.get_value_with_fudge(x, y, z, 0.0, 0.0)
    }

    /// Port of the deprecated `PerlinNoise#getValue(double, double, double, double, double)`.
    pub fn get_value_with_fudge(&self, x: f64, y: f64, z: f64, y_scale: f64, y_fudge: f64) -> f64 {
        let mut value = 0.0;
        let mut factor = self.lowest_freq_input_factor;
        let mut value_factor = self.lowest_freq_value_factor;
        for (i, level) in self.noise_levels.iter().enumerate() {
            if let Some(noise) = level {
                let noise_val = noise.noise_with_fudge(
                    wrap(x * factor),
                    wrap(y * factor),
                    wrap(z * factor),
                    y_scale * factor,
                    y_fudge * factor,
                );
                value += self.amplitudes[i] * noise_val * value_factor;
            }
            factor *= 2.0;
            value_factor /= 2.0;
        }
        value
    }

    /// Port of `PerlinNoise#maxBrokenValue(double)`.
    pub fn max_broken_value(&self, y_scale: f64) -> f64 {
        self.edge_value(y_scale + 2.0)
    }

    fn edge_value(&self, noise_value: f64) -> f64 {
        let mut value = 0.0;
        let mut value_factor = self.lowest_freq_value_factor;
        for (i, level) in self.noise_levels.iter().enumerate() {
            if level.is_some() {
                value += self.amplitudes[i] * noise_value * value_factor;
            }
            value_factor /= 2.0;
        }
        value
    }

    /// Port of `PerlinNoise#getOctaveNoise(int)`: indexes from the HIGHEST-frequency end.
    ///
    /// Panics like Java's array access for an index outside the table.
    pub fn get_octave_noise(&self, i: i32) -> Option<&ImprovedNoise> {
        let len = self.noise_levels.len() as i32;
        let index = len.wrapping_sub(1).wrapping_sub(i);
        if index < 0 || index >= len {
            throw(
                "java.lang.ArrayIndexOutOfBoundsException",
                &format!("Index {index} out of bounds for length {len}"),
            );
        }
        self.noise_levels[index as usize].as_ref()
    }

    /// Port of the protected `PerlinNoise#firstOctave()`.
    pub fn first_octave(&self) -> i32 {
        self.first_octave
    }

    /// Port of the protected `PerlinNoise#amplitudes()`.
    pub fn amplitudes(&self) -> &[f64] {
        &self.amplitudes
    }

    /// Port of `PerlinNoise#parityConfigString(StringBuilder)` (`@VisibleForTesting`).
    ///
    /// Amplitudes print with `%.2f` inside `List#toString` brackets; every slot prints
    /// `"<i>: "` then the octave's config or `null`, followed by `", "` -- including after the
    /// last one, which is why the text ends in `", ]}"`.
    pub fn parity_config_string(&self, sb: &mut String) {
        sb.push_str("PerlinNoise{");
        let amps: Vec<String> = self.amplitudes.iter().map(|d| java_format_fixed(*d, 2)).collect();
        sb.push_str(&format!(
            "first octave: {}, amplitudes: [{}], noise levels: [",
            self.first_octave,
            amps.join(", ")
        ));
        for (i, level) in self.noise_levels.iter().enumerate() {
            sb.push_str(&format!("{i}: "));
            match level {
                None => sb.push_str("null"),
                Some(n) => n.parity_config_string(sb),
            }
            sb.push_str(", ");
        }
        sb.push(']');
        sb.push('}');
    }
}

fn skip_octave(random: &mut dyn RandomSource) {
    random.consume_count(262);
}

/// Port of `PerlinNoise#wrap(double)`.
pub fn wrap(x: f64) -> f64 {
    x - Mth::lfloor(x / 3.3554432E7 + 0.5) as f64 * 3.3554432E7
}
