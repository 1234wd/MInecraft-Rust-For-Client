//! Port of: net/minecraft/world/level/levelgen/synth/NormalNoise.java
//! Java class(es): net.minecraft.world.level.levelgen.synth.NormalNoise, net.minecraft.world.level.levelgen.synth.NoiseParameters
//! Status: PARTIAL
//!
//! Session 19. Ported and verified bit-for-bit against the real 26.2 jar
//! (`_porting/tests/session19_i.rs`, `normal.*` rows of `_porting/test-data/noise_probe.txt`):
//! both factories, `getValue`, `maxValue`, `parameters`, `parityConfigString`, and the
//! `NoiseParameters` record with its three constructors.
//!
//! PORT-BLOCKED on DataFixerUpper / registries: `NoiseParameters.DIRECT_CODEC` and
//! `NoiseParameters.CODEC` (`RegistryFileCodec` over `Registries.NOISE`).
//!
//! # TWO PERLIN NOISES, THE SECOND SAMPLED SLIGHTLY STRETCHED
//!
//! `getValue` adds `first(x, y, z)` and `second(x * k, y * k, z * k)` with
//! `k = 1.0181268882175227`, so the two lattices never line up.
//!
//! # AN ALL-ZERO AMPLITUDE LIST OVERFLOWS AN `int`
//!
//! `minOctave` starts at `Integer.MAX_VALUE` and `maxOctave` at `Integer.MIN_VALUE`; with no
//! non-zero amplitude neither moves, and `maxOctave - minOctave` wraps to `1`. The port uses
//! `wrapping_sub` so that input produces Java's value instead of a debug-build panic.

use crate::net::minecraft::util::RandomSource::RandomSource;
use crate::net::minecraft::world::level::levelgen::synth::PerlinNoise::PerlinNoise;

/// Java: `private static final double INPUT_FACTOR = 1.0181268882175227`.
pub const INPUT_FACTOR: f64 = 1.0181268882175227;
/// Java: `private static final double TARGET_DEVIATION = 0.3333333333333333`.
pub const TARGET_DEVIATION: f64 = 0.3333333333333333;

/// Port of `NormalNoise.NoiseParameters(int firstOctave, DoubleList amplitudes)`.
///
/// Equality follows the record: `firstOctave` and fastutil's `DoubleArrayList.equals`, which
/// compares elements with primitive `!=` (so `NaN` never equals itself and `0.0 == -0.0`) --
/// exactly what `f64`'s `PartialEq` on a `Vec` does.
#[derive(Clone, Debug, PartialEq)]
pub struct NoiseParameters {
    first_octave: i32,
    amplitudes: Vec<f64>,
}

impl NoiseParameters {
    /// Canonical constructor (also `NoiseParameters(int, List<Double>)`).
    pub fn new(first_octave: i32, amplitudes: Vec<f64>) -> NoiseParameters {
        NoiseParameters { first_octave, amplitudes }
    }

    /// `NoiseParameters(int firstOctave, double firstAmplitude, double... amplitudes)`.
    pub fn with_first(first_octave: i32, first_amplitude: f64, amplitudes: &[f64]) -> NoiseParameters {
        let mut list = Vec::with_capacity(amplitudes.len() + 1);
        list.push(first_amplitude);
        list.extend_from_slice(amplitudes);
        NoiseParameters { first_octave, amplitudes: list }
    }

    /// Record accessor `firstOctave()`.
    pub fn first_octave(&self) -> i32 {
        self.first_octave
    }

    /// Record accessor `amplitudes()`.
    pub fn amplitudes(&self) -> &[f64] {
        &self.amplitudes
    }
}

/// Java: `public static final Codec<NoiseParameters> NoiseParameters.DIRECT_CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (`RecordCodecBuilder`).
pub fn NOISE_PARAMETERS_DIRECT_CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final Codec<Holder<NoiseParameters>> NoiseParameters.CODEC`.
///
/// PORT-BLOCKED on `net.minecraft.resources.RegistryFileCodec` and `net.minecraft.core.Holder`.
pub fn NOISE_PARAMETERS_CODEC() {
    todo!("PORT-BLOCKED: net.minecraft.resources.RegistryFileCodec is not ported")
}

/// Port of `net.minecraft.world.level.levelgen.synth.NormalNoise`.
#[derive(Clone, Debug, PartialEq)]
pub struct NormalNoise {
    value_factor: f64,
    first: PerlinNoise,
    second: PerlinNoise,
    max_value: f64,
    parameters: NoiseParameters,
}

fn expected_deviation(octave_span: i32) -> f64 {
    0.1 * (1.0 + 1.0 / octave_span.wrapping_add(1) as f64)
}

impl NormalNoise {
    /// Port of the deprecated `NormalNoise#createLegacyNetherBiome(RandomSource, NoiseParameters)`.
    pub fn create_legacy_nether_biome(random: &mut dyn RandomSource, parameters: NoiseParameters) -> NormalNoise {
        NormalNoise::new(random, parameters, false)
    }

    /// Port of `NormalNoise#create(RandomSource, int, double...)`.
    pub fn create(random: &mut dyn RandomSource, first_octave: i32, amplitudes: &[f64]) -> NormalNoise {
        NormalNoise::create_from_parameters(random, NoiseParameters::new(first_octave, amplitudes.to_vec()))
    }

    /// Port of `NormalNoise#create(RandomSource, NoiseParameters)`.
    pub fn create_from_parameters(random: &mut dyn RandomSource, parameters: NoiseParameters) -> NormalNoise {
        NormalNoise::new(random, parameters, true)
    }

    fn new(random: &mut dyn RandomSource, parameters: NoiseParameters, use_new_initialization: bool) -> NormalNoise {
        let first_octave = parameters.first_octave;
        let amplitudes = parameters.amplitudes.clone();
        let (first, second) = if use_new_initialization {
            let a = PerlinNoise::create_with_amplitudes(random, first_octave, amplitudes.clone());
            let b = PerlinNoise::create_with_amplitudes(random, first_octave, amplitudes.clone());
            (a, b)
        } else {
            let a = PerlinNoise::create_legacy_for_legacy_nether_biome(random, first_octave, amplitudes.clone());
            let b = PerlinNoise::create_legacy_for_legacy_nether_biome(random, first_octave, amplitudes.clone());
            (a, b)
        };
        let mut min_octave = i32::MAX;
        let mut max_octave = i32::MIN;
        for (i, amplitude) in amplitudes.iter().enumerate() {
            if *amplitude != 0.0 {
                min_octave = min_octave.min(i as i32);
                max_octave = max_octave.max(i as i32);
            }
        }
        let value_factor = 0.16666666666666666 / expected_deviation(max_octave.wrapping_sub(min_octave));
        let max_value = (first.max_value() + second.max_value()) * value_factor;
        NormalNoise { value_factor, first, second, max_value, parameters }
    }

    /// Port of `NormalNoise#maxValue()`.
    pub fn max_value(&self) -> f64 {
        self.max_value
    }

    /// Port of `NormalNoise#getValue(double, double, double)`.
    pub fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let x2 = x * INPUT_FACTOR;
        let y2 = y * INPUT_FACTOR;
        let z2 = z * INPUT_FACTOR;
        (self.first.get_value(x, y, z) + self.second.get_value(x2, y2, z2)) * self.value_factor
    }

    /// Port of `NormalNoise#parameters()`.
    pub fn parameters(&self) -> &NoiseParameters {
        &self.parameters
    }

    /// Port of `NormalNoise#parityConfigString(StringBuilder)` (`@VisibleForTesting`).
    pub fn parity_config_string(&self, sb: &mut String) {
        sb.push_str("NormalNoise {");
        sb.push_str("first: ");
        self.first.parity_config_string(sb);
        sb.push_str(", second: ");
        self.second.parity_config_string(sb);
        sb.push('}');
    }
}
