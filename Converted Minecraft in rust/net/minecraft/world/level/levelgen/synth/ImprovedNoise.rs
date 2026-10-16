//! Port of: net/minecraft/world/level/levelgen/synth/ImprovedNoise.java
//! Java class(es): net.minecraft.world.level.levelgen.synth.ImprovedNoise
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Verified bit-for-bit against the real 26.2 jar by
//! `_porting/tests/session19_i.rs` replaying `_porting/test-data/noise_probe.txt`
//! (`improved.origin`, `improved.noise`, `improved.fudge`, `improved.deriv`, `improved.parity`).
//!
//! # THE PERMUTATION TABLE IS A SIGNED `byte[]`, READ UNSIGNED
//!
//! Java stores `(byte) i`, so entries 128..255 are negative bytes, and `p(x)` reads them back with
//! `& 0xFF`. The port stores `i8` (so `parityConfigString`'s `p0`/`p255` print the same signed
//! values the jar prints) and widens through `u8` in [`ImprovedNoise::p`].
//!
//! # `1.0E-7F` IS A FLOAT LITERAL
//!
//! `Mth.floor(fudgeLimit / yScale + 1.0E-7F)` widens the `float` literal, so the epsilon added is
//! `1.0000000116860974E-7`, not `1.0E-7`. The port spells it `1.0E-7f32 as f64`.
//!
//! # `noise(x, y, z, yScale, yFudge)` IS DEPRECATED BUT LIVE
//!
//! Only `BlendedNoise` (old-style terrain) passes a non-zero `yScale`. With `yScale == 0.0` the
//! fudge term is exactly `0.0` and the method reduces to the plain sample.

use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::util::RandomSource::RandomSource;
use crate::net::minecraft::world::level::levelgen::synth::SimplexNoise::{dot, GRADIENT};

/// Java: `private static final float SHIFT_UP_EPSILON = 1.0E-7F`.
pub const SHIFT_UP_EPSILON: f32 = 1.0E-7;

/// Port of `net.minecraft.world.level.levelgen.synth.ImprovedNoise`.
#[derive(Clone, Debug, PartialEq)]
pub struct ImprovedNoise {
    p: [i8; 256],
    /// Java: `public final double xo`.
    pub xo: f64,
    /// Java: `public final double yo`.
    pub yo: f64,
    /// Java: `public final double zo`.
    pub zo: f64,
}

#[inline]
fn grad_dot(hash: i32, x: f64, y: f64, z: f64) -> f64 {
    dot(&GRADIENT[(hash & 15) as usize], x, y, z)
}

impl ImprovedNoise {
    /// Port of `ImprovedNoise(RandomSource)`.
    pub fn new(random: &mut dyn RandomSource) -> ImprovedNoise {
        let xo = random.next_double() * 256.0;
        let yo = random.next_double() * 256.0;
        let zo = random.next_double() * 256.0;
        let mut p = [0i8; 256];
        for (i, slot) in p.iter_mut().enumerate() {
            *slot = i as u8 as i8;
        }
        for i in 0..256usize {
            let offset = random.next_int_bounded(256 - i as i32) as usize;
            p.swap(i, i + offset);
        }
        ImprovedNoise { p, xo, yo, zo }
    }

    /// The raw signed permutation byte at `index` (Java's `this.p[index]`).
    #[inline]
    pub fn permutation_byte(&self, index: usize) -> i8 {
        self.p[index]
    }

    #[inline]
    fn p(&self, x: i32) -> i32 {
        self.p[(x & 0xFF) as usize] as u8 as i32
    }

    /// Port of `ImprovedNoise#noise(double, double, double)`.
    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        self.noise_with_fudge(x, y, z, 0.0, 0.0)
    }

    /// Port of the deprecated `ImprovedNoise#noise(double, double, double, double, double)`.
    pub fn noise_with_fudge(&self, x_in: f64, y_in: f64, z_in: f64, y_scale: f64, y_fudge: f64) -> f64 {
        let x = x_in + self.xo;
        let y = y_in + self.yo;
        let z = z_in + self.zo;
        let xf = Mth::floor_f64(x);
        let yf = Mth::floor_f64(y);
        let zf = Mth::floor_f64(z);
        let xr = x - xf as f64;
        let yr = y - yf as f64;
        let zr = z - zf as f64;
        let yr_fudge = if y_scale != 0.0 {
            let fudge_limit = if y_fudge >= 0.0 && y_fudge < yr { y_fudge } else { yr };
            Mth::floor_f64(fudge_limit / y_scale + SHIFT_UP_EPSILON as f64) as f64 * y_scale
        } else {
            0.0
        };
        self.sample_and_lerp(xf, yf, zf, xr, yr - yr_fudge, zr, yr)
    }

    /// Port of `ImprovedNoise#noiseWithDerivative(double, double, double, double[])`.
    ///
    /// Like Java, the derivative is ADDED into `derivative_out` (callers accumulate octaves).
    pub fn noise_with_derivative(&self, x_in: f64, y_in: f64, z_in: f64, derivative_out: &mut [f64; 3]) -> f64 {
        let x = x_in + self.xo;
        let y = y_in + self.yo;
        let z = z_in + self.zo;
        let xf = Mth::floor_f64(x);
        let yf = Mth::floor_f64(y);
        let zf = Mth::floor_f64(z);
        let xr = x - xf as f64;
        let yr = y - yf as f64;
        let zr = z - zf as f64;
        self.sample_with_derivative(xf, yf, zf, xr, yr, zr, derivative_out)
    }

    #[allow(clippy::too_many_arguments)]
    fn sample_and_lerp(&self, x: i32, y: i32, z: i32, xr: f64, yr: f64, zr: f64, yr_original: f64) -> f64 {
        let x0 = self.p(x);
        let x1 = self.p(x.wrapping_add(1));
        let xy00 = self.p(x0.wrapping_add(y));
        let xy01 = self.p(x0.wrapping_add(y).wrapping_add(1));
        let xy10 = self.p(x1.wrapping_add(y));
        let xy11 = self.p(x1.wrapping_add(y).wrapping_add(1));
        let d000 = grad_dot(self.p(xy00.wrapping_add(z)), xr, yr, zr);
        let d100 = grad_dot(self.p(xy10.wrapping_add(z)), xr - 1.0, yr, zr);
        let d010 = grad_dot(self.p(xy01.wrapping_add(z)), xr, yr - 1.0, zr);
        let d110 = grad_dot(self.p(xy11.wrapping_add(z)), xr - 1.0, yr - 1.0, zr);
        let d001 = grad_dot(self.p(xy00.wrapping_add(z).wrapping_add(1)), xr, yr, zr - 1.0);
        let d101 = grad_dot(self.p(xy10.wrapping_add(z).wrapping_add(1)), xr - 1.0, yr, zr - 1.0);
        let d011 = grad_dot(self.p(xy01.wrapping_add(z).wrapping_add(1)), xr, yr - 1.0, zr - 1.0);
        let d111 = grad_dot(self.p(xy11.wrapping_add(z).wrapping_add(1)), xr - 1.0, yr - 1.0, zr - 1.0);
        let x_alpha = Mth::smoothstep(xr);
        let y_alpha = Mth::smoothstep(yr_original);
        let z_alpha = Mth::smoothstep(zr);
        Mth::lerp_3(x_alpha, y_alpha, z_alpha, d000, d100, d010, d110, d001, d101, d011, d111)
    }

    #[allow(clippy::too_many_arguments)]
    fn sample_with_derivative(
        &self,
        x: i32,
        y: i32,
        z: i32,
        xr: f64,
        yr: f64,
        zr: f64,
        derivative_out: &mut [f64; 3],
    ) -> f64 {
        let x0 = self.p(x);
        let x1 = self.p(x.wrapping_add(1));
        let xy00 = self.p(x0.wrapping_add(y));
        let xy01 = self.p(x0.wrapping_add(y).wrapping_add(1));
        let xy10 = self.p(x1.wrapping_add(y));
        let xy11 = self.p(x1.wrapping_add(y).wrapping_add(1));
        let p000 = self.p(xy00.wrapping_add(z));
        let p100 = self.p(xy10.wrapping_add(z));
        let p010 = self.p(xy01.wrapping_add(z));
        let p110 = self.p(xy11.wrapping_add(z));
        let p001 = self.p(xy00.wrapping_add(z).wrapping_add(1));
        let p101 = self.p(xy10.wrapping_add(z).wrapping_add(1));
        let p011 = self.p(xy01.wrapping_add(z).wrapping_add(1));
        let p111 = self.p(xy11.wrapping_add(z).wrapping_add(1));
        let g000 = &GRADIENT[(p000 & 15) as usize];
        let g100 = &GRADIENT[(p100 & 15) as usize];
        let g010 = &GRADIENT[(p010 & 15) as usize];
        let g110 = &GRADIENT[(p110 & 15) as usize];
        let g001 = &GRADIENT[(p001 & 15) as usize];
        let g101 = &GRADIENT[(p101 & 15) as usize];
        let g011 = &GRADIENT[(p011 & 15) as usize];
        let g111 = &GRADIENT[(p111 & 15) as usize];
        let d000 = dot(g000, xr, yr, zr);
        let d100 = dot(g100, xr - 1.0, yr, zr);
        let d010 = dot(g010, xr, yr - 1.0, zr);
        let d110 = dot(g110, xr - 1.0, yr - 1.0, zr);
        let d001 = dot(g001, xr, yr, zr - 1.0);
        let d101 = dot(g101, xr - 1.0, yr, zr - 1.0);
        let d011 = dot(g011, xr, yr - 1.0, zr - 1.0);
        let d111 = dot(g111, xr - 1.0, yr - 1.0, zr - 1.0);
        let x_alpha = Mth::smoothstep(xr);
        let y_alpha = Mth::smoothstep(yr);
        let z_alpha = Mth::smoothstep(zr);
        let c = |k: usize| {
            Mth::lerp_3(
                x_alpha,
                y_alpha,
                z_alpha,
                g000[k] as f64,
                g100[k] as f64,
                g010[k] as f64,
                g110[k] as f64,
                g001[k] as f64,
                g101[k] as f64,
                g011[k] as f64,
                g111[k] as f64,
            )
        };
        let d1x = c(0);
        let d1y = c(1);
        let d1z = c(2);
        let d2x = Mth::lerp_2(y_alpha, z_alpha, d100 - d000, d110 - d010, d101 - d001, d111 - d011);
        let d2y = Mth::lerp_2(z_alpha, x_alpha, d010 - d000, d011 - d001, d110 - d100, d111 - d101);
        let d2z = Mth::lerp_2(x_alpha, y_alpha, d001 - d000, d101 - d100, d011 - d010, d111 - d110);
        let x_sd = Mth::smoothstep_derivative(xr);
        let y_sd = Mth::smoothstep_derivative(yr);
        let z_sd = Mth::smoothstep_derivative(zr);
        let dx = d1x + x_sd * d2x;
        let dy = d1y + y_sd * d2y;
        let dz = d1z + z_sd * d2z;
        derivative_out[0] += dx;
        derivative_out[1] += dy;
        derivative_out[2] += dz;
        Mth::lerp_3(x_alpha, y_alpha, z_alpha, d000, d100, d010, d110, d001, d101, d011, d111)
    }

    /// Port of `ImprovedNoise#parityConfigString(StringBuilder)` (`@VisibleForTesting`):
    /// `NoiseUtils.parityNoiseOctaveConfigString(sb, xo, yo, zo, p)`.
    pub fn parity_config_string(&self, sb: &mut String) {
        sb.push_str(&parity_noise_octave_config_string(
            self.xo,
            self.yo,
            self.zo,
            self.p[0] as i32,
            self.p[255] as i32,
        ));
    }
}

/// `String.format(Locale.ROOT, "xo=%.3f, yo=%.3f, zo=%.3f, p0=%d, p255=%d", (float)xo, ...)`,
/// the body of `NoiseUtils.parityNoiseOctaveConfigString` (both overloads print the same text).
///
/// Kept private to this file so the noise port does not depend on another in-flight port of
/// `NoiseUtils`; `java_format_fixed` reproduces `java.util.Formatter`'s `%.Nf` on the float
/// widened to double and is checked against the jar's `improved.parity` rows.
pub(crate) fn parity_noise_octave_config_string(xo: f64, yo: f64, zo: f64, p0: i32, p255: i32) -> String {
    format!(
        "xo={}, yo={}, zo={}, p0={p0}, p255={p255}",
        java_format_fixed(xo as f32 as f64, 3),
        java_format_fixed(yo as f32 as f64, 3),
        java_format_fixed(zo as f32 as f64, 3)
    )
}

/// `String.format(Locale.ROOT, "%.<precision>f", value)` for finite values.
///
/// `java.util.Formatter` does NOT round the exact binary value: it takes the decimal digits of
/// the shortest representation that round-trips (the `Double.toString` digits) and rounds THOSE
/// half-up. So `0.125` with precision 2 is `"0.13"` (Rust's `{:.2}` gives `"0.12"`, half-even on
/// the exact value). The shortest digits come from Rust's `{:e}` formatting, which is also
/// shortest-round-trip.
pub(crate) fn java_format_fixed(value: f64, precision: usize) -> String {
    if !value.is_finite() {
        return if value.is_nan() {
            "NaN".to_string()
        } else if value > 0.0 {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        };
    }
    let negative = value.is_sign_negative();
    let sci = format!("{:e}", value.abs());
    let (mantissa, exp) = sci.split_once('e').expect("`{:e}` always has an exponent");
    let exp: i32 = exp.parse().expect("integer exponent");
    let digits: Vec<u8> = mantissa.bytes().filter(|b| b.is_ascii_digit()).map(|b| b - b'0').collect();
    // value = 0.d1 d2 d3 ... * 10^(exp + 1); `point` = number of integer digits.
    let point = exp + 1;
    // Build the integer of all digits up to `precision` decimals, plus the next digit for rounding.
    let keep = point + precision as i32; // digits kept from the front
    let mut kept: Vec<u8> = Vec::new();
    let mut round_up = false;
    if keep < 0 {
        // Everything is below the rounding position; half-up cannot reach it either (the first
        // dropped digit is an implicit 0).
    } else {
        for i in 0..keep as usize {
            kept.push(*digits.get(i).unwrap_or(&0));
        }
        if let Some(&next) = digits.get(keep as usize) {
            round_up = next >= 5;
        }
    }
    if round_up {
        let mut i = kept.len();
        loop {
            if i == 0 {
                kept.insert(0, 1);
                break;
            }
            i -= 1;
            if kept[i] == 9 {
                kept[i] = 0;
            } else {
                kept[i] += 1;
                break;
            }
        }
    }
    // kept represents an integer N with the value N / 10^precision.
    let mut n: Vec<u8> = kept;
    while n.len() < precision + 1 {
        n.insert(0, 0);
    }
    let split = n.len() - precision;
    let int_part: String = n[..split].iter().map(|d| (b'0' + d) as char).collect();
    let int_part = int_part.trim_start_matches('0');
    let int_part = if int_part.is_empty() { "0" } else { int_part };
    let frac: String = n[split..].iter().map(|d| (b'0' + d) as char).collect();
    let mut out = String::new();
    if negative {
        out.push('-');
    }
    out.push_str(int_part);
    if precision > 0 {
        out.push('.');
        out.push_str(&frac);
    }
    out
}
