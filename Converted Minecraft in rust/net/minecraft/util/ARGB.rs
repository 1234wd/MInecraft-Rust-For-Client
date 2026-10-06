//! Port of: net/minecraft/util/ARGB.java
//! Java class(es): net.minecraft.util.ARGB
//! Status: PORTED
//!
//! All 23 `argb.*` golden groups in `batch2.txt` are bit-exact, and the two embedded sRGB
//! tables are verified value-for-value against the jar by
//! `parity_batch2::argb_tables_match_the_golden`.
//!
//! # THE TWO EMBEDDED TABLES, AND WHY THEY ARE NOT COMPUTED
//!
//! `SRGB_TO_LINEAR` (a Java `short[256]`, values 0..1023) and `LINEAR_TO_SRGB` (a Java
//! `byte[1024]`, values 0..255) are built at class-initialisation time by `Math.pow`, and
//! `Math.pow` is a HotSpot intrinsic we have not ported. Recomputing them in Rust would add a
//! third unported intrinsic and would be wrong in a way no test could distinguish from "the
//! table is fine". They are embedded instead -- the same `embedded-trig-tables` decision as
//! `Mth`'s 65,536-entry SIN/COS tables.
//!
//! The values live in `_porting/generated/argb_srgb_tables.rs`, derived from the jar by
//! `_porting/tools/gen_argb_tables.py`. 1280 numbers cannot be typed, and typing them is the
//! hand-derived-constant error this project forbids.
//!
//! The two tables are DIFFERENT TYPES AND DIFFERENT LENGTHS, which the decompiled source makes
//! easy to get wrong because the declarations sit next to each other and look symmetric. From
//! `javap -p -cp minecraft-merged-deobf-26.2.jar net.minecraft.util.ARGB`:
//!
//! ```text
//! private static final short[] SRGB_TO_LINEAR;
//! private static final byte[]  LINEAR_TO_SRGB;
//! ```
//!
//! # WHERE VANILLA ACTUALLY CRASHES, AND WHY THAT IS NOT "FIXED" HERE
//!
//! `linearLerp` and `linearToSrgbChannel` index `LINEAR_TO_SRGB` with a value computed by
//! `Mth#lerpInt`, which is `(int)(start + alpha * (end - start))` with **no clamping**. A
//! negative `alpha`, or one above 1, produces an index below 0 or above 1023, and vanilla 26.2
//! throws:
//!
//! ```text
//! java.lang.ArrayIndexOutOfBoundsException: Index -211 out of bounds for length 1024
//! ```
//!
//! That is a real crash in the game, reached from entity rendering and from any code that fades
//! a colour with an unclamped factor. This port **reproduces the throw** rather than clamping,
//! because a clamping "fix" would return a colour where the game crashes -- a gameplay change,
//! and the most tempting kind of silent one. See DESIGN_DECISIONS.md
//! (`#runtime-exceptions`) for the rule this establishes.
//!
//! # THE `as usize` AUDIT
//!
//! Every index into either table goes through a checked accessor that validates the Java `int`
//! **before** the cast. That ordering is the whole point: a negative Java index must not be
//! allowed to become a huge `usize` and silently read out of bounds, or -- worse -- wrap into a
//! *valid* index and return a plausible wrong colour. `checked_linear_to_srgb` and
//! `checked_srgb_to_linear` are the only two places in the file that index a table, and neither
//! contains a bare `as usize`.

use crate::javacompat::java_lang;
use crate::javacompat::joml::Vector3f;
use crate::net::minecraft::util::Mth::Mth;
use crate::argb_srgb_tables::{LINEAR_TO_SRGB, SRGB_TO_LINEAR};

// ============================================================================
// Table access -- the ONLY two places a table is indexed
// ============================================================================

/// Java: `SRGB_TO_LINEAR[srgb]`.
///
/// The parameter is a raw `int` on the public `srgbToLinearChannel`, so it can be any value and
/// the JVM bounds-checks it. The check happens BEFORE the `as usize`, so a negative index cannot
/// wrap around into a valid slot.
#[inline]
fn checked_srgb_to_linear(srgb: i32) -> u16 {
    if srgb < 0 || srgb >= SRGB_TO_LINEAR.len() as i32 {
        throw_array_index_out_of_bounds(srgb, SRGB_TO_LINEAR.len());
    }
    SRGB_TO_LINEAR[srgb as usize]
}

/// Java: `LINEAR_TO_SRGB[index] & 0xFF`.
///
/// `index` comes from `Mth#floor(linear * 1023.0F)` or from `Mth#lerpInt`, neither of which
/// clamps, so it really does go out of range in vanilla. Same ordering rule as above.
#[inline]
fn checked_linear_to_srgb(index: i32) -> u8 {
    if index < 0 || index >= LINEAR_TO_SRGB.len() as i32 {
        throw_array_index_out_of_bounds(index, LINEAR_TO_SRGB.len());
    }
    LINEAR_TO_SRGB[index as usize]
}

/// Reproduces Java's `ArrayIndexOutOfBoundsException`, message text and all.
///
/// # WHY A PANIC AND NOT A `Result`
///
/// Vanilla throws an unchecked exception here, so this panics -- see DESIGN_DECISIONS.md
/// (`#runtime-exceptions`). The message is Java's own wording, including the index and the array
/// length, because that text is what a player or a crash log would show and it is the thing a
/// maintainer greps for. `argb.linearLerpThrows` records the JVM's exact strings and
/// `parity_batch2::argb_linear_lerp_panics_like_java` compares against them.
#[cold]
#[inline(never)]
fn throw_array_index_out_of_bounds(index: i32, length: usize) -> ! {
    panic!(
        "java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {length}"
    );
}

// ============================================================================
// Channel accessors
// ============================================================================

/// Port of `ARGB#alpha(int)`: `color >>> 24`.
///
/// The unsigned shift matters: it is what makes the alpha channel 0..255 for a negative `color`.
#[inline]
pub fn alpha(color: i32) -> i32 {
    color >> 24 & 0xFF
}

/// Port of `ARGB#red(int)`.
#[inline]
pub fn red(color: i32) -> i32 {
    color >> 16 & 0xFF
}

/// Port of `ARGB#green(int)`.
#[inline]
pub fn green(color: i32) -> i32 {
    color >> 8 & 0xFF
}

/// Port of `ARGB#blue(int)`.
#[inline]
pub fn blue(color: i32) -> i32 {
    color & 0xFF
}

/// Port of `ARGB#color(int,int,int,int)`.
///
/// Every channel is masked with `0xFF`, so out-of-range channels are truncated rather than
/// wrapped into the neighbouring channel.
#[inline]
pub fn color_4(alpha: i32, red: i32, green: i32, blue: i32) -> i32 {
    (alpha & 0xFF) << 24 | (red & 0xFF) << 16 | (green & 0xFF) << 8 | blue & 0xFF
}

/// Port of `ARGB#color(int,int,int)`.
#[inline]
pub fn color_3(red: i32, green: i32, blue: i32) -> i32 {
    color_4(255, red, green, blue)
}

/// Port of `ARGB#color(int,int)` -- an alpha plus a packed `0xRRGGBB`.
///
/// Note the alpha is **not** masked, exactly as in Java: `alpha << 24` keeps only the low byte
/// anyway because the shift discards the rest.
#[inline]
pub fn color_alpha_rgb(alpha: i32, rgb: i32) -> i32 {
    alpha << 24 | rgb & 16_777_215
}

/// Port of `ARGB#color(float,int)`.
#[inline]
pub fn color_float_alpha_rgb(alpha: f32, rgb: i32) -> i32 {
    as_8bit_channel(alpha) << 24 | rgb & 16_777_215
}

/// Port of `ARGB#color(Vec3)`.
///
/// Each component is narrowed to `f32` first, as Java's `(float)vec.x()` does, and then floored
/// after scaling. The narrowing is not cosmetic: `Vec3` holds `f64`, and scaling in `f64` and
/// narrowing afterwards would floor a different value.
#[inline]
pub fn color_vec3(vec: crate::net::minecraft::world::phys::Vec3::Vec3) -> i32 {
    color_3(
        as_8bit_channel(vec.x as f32),
        as_8bit_channel(vec.y as f32),
        as_8bit_channel(vec.z as f32),
    )
}

/// Port of `ARGB#as8BitChannel(float)`: `Mth.floor(value * 255.0F)`.
///
/// No clamping, matching vanilla. Negative and above-1 inputs therefore produce indices or
/// channel values outside 0..255, and `color_4` masks them -- that is vanilla's behaviour, not a
/// bug in this port.
#[inline]
pub fn as_8bit_channel(value: f32) -> i32 {
    Mth::floor_f32(value * 255.0)
}

// ============================================================================
// Whole-colour transforms
// ============================================================================

/// Port of `ARGB#opaque(int)`.
///
/// Java's `0xFF000000` is the `int` literal `-16777216`; written as hex in Rust it would not fit
/// `i32`, which is why it appears as a signed constant here.
#[inline]
pub fn opaque(color: i32) -> i32 {
    color | -16_777_216
}

/// Port of `ARGB#transparent(int)`.
#[inline]
pub fn transparent(color: i32) -> i32 {
    color & 16_777_215
}

/// Port of `ARGB#white(float)`.
#[inline]
pub fn white_float(alpha: f32) -> i32 {
    as_8bit_channel(alpha) << 24 | 16_777_215
}

/// Port of `ARGB#white(int)`.
#[inline]
pub fn white_int(alpha: i32) -> i32 {
    alpha << 24 | 16_777_215
}

/// Port of `ARGB#black(float)`.
#[inline]
pub fn black_float(alpha: f32) -> i32 {
    as_8bit_channel(alpha) << 24
}

/// Port of `ARGB#black(int)`.
#[inline]
pub fn black_int(alpha: i32) -> i32 {
    alpha << 24
}

/// Port of `ARGB#gray(float)`.
#[inline]
pub fn gray(brightness: f32) -> i32 {
    let channel = as_8bit_channel(brightness);
    color_3(channel, channel, channel)
}

/// Port of `ARGB#colorFromFloat(float,float,float,float)`.
#[inline]
pub fn color_from_float(alpha: f32, red: f32, green: f32, blue: f32) -> i32 {
    color_4(
        as_8bit_channel(alpha),
        as_8bit_channel(red),
        as_8bit_channel(green),
        as_8bit_channel(blue),
    )
}

/// Port of `ARGB#average(int,int)`.
///
/// Integer division on the sum of two channels, so it truncates toward zero -- including for
/// negative colours, where the channels are still 0..255 because the accessors mask.
#[inline]
pub fn average(lhs: i32, rhs: i32) -> i32 {
    color_4(
        (alpha(lhs) + alpha(rhs)) / 2,
        (red(lhs) + red(rhs)) / 2,
        (green(lhs) + green(rhs)) / 2,
        (blue(lhs) + blue(rhs)) / 2,
    )
}

/// Port of `ARGB#multiply(int,int)`.
///
/// `-1` is the "no filter" sentinel and short-circuits. The channel products use integer division
/// by 255, which truncates.
#[inline]
pub fn multiply(lhs: i32, rhs: i32) -> i32 {
    if lhs == -1 {
        return rhs;
    }
    if rhs == -1 {
        return lhs;
    }
    color_4(
        alpha(lhs) * alpha(rhs) / 255,
        red(lhs) * red(rhs) / 255,
        green(lhs) * green(rhs) / 255,
        blue(lhs) * blue(rhs) / 255,
    )
}

/// Port of `ARGB#addRgb(int,int)`: saturating add on RGB, alpha untouched.
#[inline]
pub fn add_rgb(lhs: i32, rhs: i32) -> i32 {
    color_4(
        alpha(lhs),
        java_lang::min_i32(red(lhs) + red(rhs), 255),
        java_lang::min_i32(green(lhs) + green(rhs), 255),
        java_lang::min_i32(blue(lhs) + blue(rhs), 255),
    )
}

/// Port of `ARGB#subtractRgb(int,int)`: saturating subtract on RGB, alpha untouched.
#[inline]
pub fn subtract_rgb(lhs: i32, rhs: i32) -> i32 {
    color_4(
        alpha(lhs),
        java_lang::max_i32(red(lhs) - red(rhs), 0),
        java_lang::max_i32(green(lhs) - green(rhs), 0),
        java_lang::max_i32(blue(lhs) - blue(rhs), 0),
    )
}

/// Port of `ARGB#multiplyAlpha(int,float)`.
///
/// Three branches, all load-bearing: a zero colour or a non-positive multiplier returns 0, a
/// multiplier of 1 or more returns the colour unchanged, and only the middle case rebuilds it.
/// The middle case goes through the `color(float,int)` overload, so the alpha is *floored after
/// scaling* rather than rounded -- which is why `alphaFloat(color) * alphaMultiplier` being
/// slightly under 1 can lose a whole level.
#[inline]
pub fn multiply_alpha(color: i32, alpha_multiplier: f32) -> i32 {
    if color == 0 || alpha_multiplier <= 0.0 {
        return 0;
    }
    if alpha_multiplier >= 1.0 {
        return color;
    }
    color_float_alpha_rgb(alpha_float(color) * alpha_multiplier, color)
}

/// Port of `ARGB#scaleRGB(int,float)`.
#[inline]
pub fn scale_rgb_uniform(color: i32, scale: f32) -> i32 {
    scale_rgb(color, scale, scale, scale)
}

/// Port of `ARGB#scaleRGB(int,float,float,float)`.
///
/// Each channel is truncated by the `(int)` cast **before** being clamped, so a negative product
/// truncates toward zero and then clamps to 0. Reversing those two steps would differ for
/// fractional negative products.
#[inline]
pub fn scale_rgb(color: i32, scale_r: f32, scale_g: f32, scale_b: f32) -> i32 {
    color_4(
        alpha(color),
        Mth::clamp_i32((red(color) as f32 * scale_r) as i32, 0, 255),
        Mth::clamp_i32((green(color) as f32 * scale_g) as i32, 0, 255),
        Mth::clamp_i32((blue(color) as f32 * scale_b) as i32, 0, 255),
    )
}

/// Port of `ARGB#scaleRGB(int,int)`.
///
/// The Java widens to `long` before multiplying, and that is load-bearing: `red(color) * scale`
/// overflows `int` for large scales and would wrap to a negative number that then clamps to 0.
/// Done in `i64`, it saturates at 255 instead.
#[inline]
pub fn scale_rgb_int(color: i32, scale: i32) -> i32 {
    color_4(
        alpha(color),
        Mth::clamp_i64(red(color) as i64 * scale as i64 / 255, 0, 255) as i32,
        Mth::clamp_i64(green(color) as i64 * scale as i64 / 255, 0, 255) as i32,
        Mth::clamp_i64(blue(color) as i64 * scale as i64 / 255, 0, 255) as i32,
    )
}

/// Port of `ARGB#greyscale(int)`: the classic 0.3/0.59/0.11 luma weights.
///
/// The weights are `f32`, so the sum is computed in single precision and truncated once at the
/// end -- not per term.
#[inline]
pub fn greyscale(color: i32) -> i32 {
    let grey = (red(color) as f32 * 0.3 + green(color) as f32 * 0.59 + blue(color) as f32 * 0.11)
        as i32;
    color_4(alpha(color), grey, grey, grey)
}

/// Port of `ARGB#alphaBlend(int,int)` -- `destination` under `source`.
///
/// The two early returns are opaque and fully transparent sources. The composite alpha
/// `sourceAlpha + destinationAlpha * (255 - sourceAlpha) / 255` truncates, and each channel then
/// divides by that alpha, so a truncating alpha slightly darkens the result. Faithful, not fixed.
#[inline]
pub fn alpha_blend(destination: i32, source: i32) -> i32 {
    let destination_alpha = alpha(destination);
    let source_alpha = alpha(source);
    if source_alpha == 255 {
        return source;
    }
    if source_alpha == 0 {
        return destination;
    }
    let a = source_alpha + destination_alpha * (255 - source_alpha) / 255;
    color_4(
        a,
        alpha_blend_channel(a, source_alpha, red(destination), red(source)),
        alpha_blend_channel(a, source_alpha, green(destination), green(source)),
        alpha_blend_channel(a, source_alpha, blue(destination), blue(source)),
    )
}

/// Port of `ARGB#alphaBlendChannel(int,int,int,int)`. Private in Java.
#[inline]
fn alpha_blend_channel(result_alpha: i32, source_alpha: i32, destination: i32, source: i32) -> i32 {
    (source * source_alpha + destination * (result_alpha - source_alpha)) / result_alpha
}

/// Port of `ARGB#srgbLerp(float,int,int)`.
///
/// Interpolation happens on the **sRGB-encoded** channel values, in integer, via `Mth#lerpInt`.
/// That is the cheap wrong-looking one; [`linear_lerp`] is the physically correct one.
///
/// The parameter is `alpha_factor`, not Java's `alpha`, because a local named `alpha` shadows
/// the [`alpha`] function and `alpha(p0)` then fails to resolve. Java has no such problem --
/// `alpha(p0)` is unambiguously a method call there -- so the rename is a language artefact, not
/// a deviation. `Mth::lerp_int` already names its parameter `alpha1` for the same reason.
#[inline]
pub fn srgb_lerp(alpha_factor: f32, p0: i32, p1: i32) -> i32 {
    color_4(
        Mth::lerp_int(alpha_factor, alpha(p0), alpha(p1)),
        Mth::lerp_int(alpha_factor, red(p0), red(p1)),
        Mth::lerp_int(alpha_factor, green(p0), green(p1)),
        Mth::lerp_int(alpha_factor, blue(p0), blue(p1)),
    )
}

/// Port of `ARGB#linearLerp(float,int,int)`.
///
/// Interpolates in linear light and re-encodes. **This panics on an out-of-range index**, exactly
/// as vanilla throws -- see the module docs. `Mth#lerpInt` does not clamp, so a negative or
/// above-1 `alpha` walks the index off the end of `LINEAR_TO_SRGB`.
#[inline]
pub fn linear_lerp(alpha_factor: f32, p0: i32, p1: i32) -> i32 {
    color_4(
        Mth::lerp_int(alpha_factor, alpha(p0), alpha(p1)),
        checked_linear_to_srgb(Mth::lerp_int(
            alpha_factor,
            checked_srgb_to_linear(red(p0)) as i32,
            checked_srgb_to_linear(red(p1)) as i32,
        )) as i32,
        checked_linear_to_srgb(Mth::lerp_int(
            alpha_factor,
            checked_srgb_to_linear(green(p0)) as i32,
            checked_srgb_to_linear(green(p1)) as i32,
        )) as i32,
        checked_linear_to_srgb(Mth::lerp_int(
            alpha_factor,
            checked_srgb_to_linear(blue(p0)) as i32,
            checked_srgb_to_linear(blue(p1)) as i32,
        )) as i32,
    )
}

/// Port of `ARGB#meanLinear(int,int,int,int)`.
#[inline]
pub fn mean_linear(srgb1: i32, srgb2: i32, srgb3: i32, srgb4: i32) -> i32 {
    color_4(
        (alpha(srgb1) + alpha(srgb2) + alpha(srgb3) + alpha(srgb4)) / 4,
        linear_channel_mean(red(srgb1), red(srgb2), red(srgb3), red(srgb4)),
        linear_channel_mean(green(srgb1), green(srgb2), green(srgb3), green(srgb4)),
        linear_channel_mean(blue(srgb1), blue(srgb2), blue(srgb3), blue(srgb4)),
    )
}

/// Port of `ARGB#linearChannelMean(int,int,int,int)`. Private in Java.
///
/// The index is the mean of four table entries, each 0..1023, so it is always 0..1023 and cannot
/// throw. It goes through the checked accessor anyway: one call site, and the guarantee is then
/// local rather than a comment.
#[inline]
fn linear_channel_mean(c1: i32, c2: i32, c3: i32, c4: i32) -> i32 {
    let linear = (checked_srgb_to_linear(c1) as i32
        + checked_srgb_to_linear(c2) as i32
        + checked_srgb_to_linear(c3) as i32
        + checked_srgb_to_linear(c4) as i32)
        / 4;
    checked_linear_to_srgb(linear) as i32
}

/// Port of `ARGB#srgbToLinearChannel(int)`.
///
/// Divides by **1023.0F**, not 255, because the table holds 0..1023. Panics for an out-of-range
/// channel, as the JVM does.
#[inline]
pub fn srgb_to_linear_channel(srgb: i32) -> f32 {
    checked_srgb_to_linear(srgb) as f32 / 1023.0
}

/// Port of `ARGB#linearToSrgbChannel(float)`.
///
/// `Mth.floor(linear * 1023.0F)` with no clamping, so this panics for a `linear` outside 0..1 --
/// as vanilla does. See the module docs.
#[inline]
pub fn linear_to_srgb_channel(linear: f32) -> i32 {
    checked_linear_to_srgb(Mth::floor_f32(linear * 1023.0)) as i32
}

// ============================================================================
// Float channel accessors
// ============================================================================

/// Port of `ARGB#from8BitChannel(int)`. Private in Java: `value / 255.0F`.
#[inline]
fn from_8bit_channel(value: i32) -> f32 {
    value as f32 / 255.0
}

/// Port of `ARGB#alphaFloat(int)`.
#[inline]
pub fn alpha_float(color: i32) -> f32 {
    from_8bit_channel(alpha(color))
}

/// Port of `ARGB#redFloat(int)`.
#[inline]
pub fn red_float(color: i32) -> f32 {
    from_8bit_channel(red(color))
}

/// Port of `ARGB#greenFloat(int)`.
#[inline]
pub fn green_float(color: i32) -> f32 {
    from_8bit_channel(green(color))
}

/// Port of `ARGB#blueFloat(int)`.
#[inline]
pub fn blue_float(color: i32) -> f32 {
    from_8bit_channel(blue(color))
}

// ============================================================================
// Byte order
// ============================================================================

/// Port of `ARGB#toABGR(int)`: swaps red and blue, leaving alpha and green alone.
///
/// The masks are applied **before** the shifts. `(color & 0xFF0000) >> 16` and
/// `(color & 0xFF) << 16` cannot be reordered, because the unmasked high byte would land in
/// green's position.
#[inline]
pub fn to_abgr(color: i32) -> i32 {
    color & -16_711_936 | (color & 0xFF_0000) >> 16 | (color & 0xFF) << 16
}

/// Port of `ARGB#fromABGR(int)`: identical to [`to_abgr`], since the swap is its own inverse.
#[inline]
pub fn from_abgr(color: i32) -> i32 {
    to_abgr(color)
}

// ============================================================================
// setBrightness
// ============================================================================

/// Java's `Math.round(float)`: round half up toward positive infinity, returned as an `int`.
///
/// # WHY THIS IS NOT `(x + 0.5) as i32`, IN EITHER PRECISION
///
/// The tempting one-liner is wrong. In `f32`, `0.49999997 + 0.5` rounds **up to 1.0**, so
/// `floor(a + 0.5f)` returns 1 where Java returns 0 -- and `0.49999997` is the largest `f32`
/// below `0.5`, so it is not an exotic input. JDK 21+ does not implement `Math.round(float)` as
/// `(int)(a + 0.5f)` either; it uses bit manipulation precisely to avoid that.
///
/// Widening to `f64` first fixes it, and is exact rather than approximate: every `f32` is
/// representable in `f64`, and adding `0.5` to a magnitude below 2^24 needs at most 25 bits of
/// mantissa against `f64`'s 53. So no rounding occurs and `floor` gives the true answer.
///
/// The edges fall out correctly without special cases, because Rust's float-to-int `as` cast
/// saturates and `floor` of a NaN is NaN:
/// * NaN -> 0, matching Java.
/// * `x >= 2^31` -> `i32::MAX`, matching Java's clamp.
/// * `x <= -2^31` -> `i32::MIN`, matching Java's clamp.
#[inline]
fn math_round_f32(x: f32) -> i32 {
    Mth::floor_f64(x as f64 + 0.5)
}

/// Port of `ARGB#setBrightness(int,float)`.
///
/// The longest method in the class, and the one with the most room to be subtly wrong. What
/// matters:
///
/// * `saturation` and `hue` are `f32`; the hue is computed only when `saturation != 0`, and the
///   greyscale shortcut is a **separate** later branch, so the two are not merged.
/// * The three "constant" terms are `(rgbMax - channel) / rgbConstantRange`, i.e. distance from
///   the maximum, not the usual saturation formula. Faithfully transcribed.
/// * `hue /= 6.0F` then wraps with `if (hue < 0.0F) hue++` -- a single increment, not a modulo.
/// * The colour wheel is split by `(int)colorWheelSegment` over SIX cases with no `default`, so
///   a segment of exactly 6.0 (hue == 1.0) would fall through and leave red/green/blue at their
///   pre-switch values. Reproduced, not "fixed".
/// * **Case 4's blue channel uses `brightness`, not `secondaryColor`.** The decompiled source
///   says `secondaryColor` and is WRONG. `javap -c` on the jar shows the blue assignment in
///   case 4 loading `fload_1`, which is `brightness`, where cases 1, 3 and 5 load `fload 14`,
///   which is `secondaryColor`. Caught by golden row 146 -- colour `0x000100FF` at brightness
///   0.5 -- where the source's formula yields blue 127 and the jar yields 128. Since
///   `0x000100FF` is blue-dominant, case 4 is reached by ordinary colours and this is not an
///   unreachable corner. See DESIGN_DECISIONS.md (`#decompiler-artifacts`).
#[inline]
pub fn set_brightness(color: i32, brightness: f32) -> i32 {
    let mut r = red(color);
    let mut g = green(color);
    let mut b = blue(color);
    let a = alpha(color);
    let rgb_max = java_lang::max_i32(java_lang::max_i32(r, g), b);
    let rgb_min = java_lang::min_i32(java_lang::min_i32(r, g), b);
    let rgb_constant_range = (rgb_max - rgb_min) as f32;
    let saturation = if rgb_max != 0 {
        rgb_constant_range / rgb_max as f32
    } else {
        0.0
    };

    let mut hue = 0.0f32;
    if saturation != 0.0 {
        let constant_red = (rgb_max - r) as f32 / rgb_constant_range;
        let constant_green = (rgb_max - g) as f32 / rgb_constant_range;
        let constant_blue = (rgb_max - b) as f32 / rgb_constant_range;
        if r == rgb_max {
            hue = constant_blue - constant_green;
        } else if g == rgb_max {
            hue = 2.0 + constant_red - constant_blue;
        } else {
            hue = 4.0 + constant_green - constant_red;
        }
        hue /= 6.0;
        if hue < 0.0 {
            hue += 1.0;
        }
    }

    if saturation == 0.0 {
        let grey = math_round_f32(brightness * 255.0) as i32;
        return color_4(a, grey, grey, grey);
    }

    let color_wheel_segment = (hue - hue.floor()) * 6.0;
    let color_wheel_offset = color_wheel_segment - color_wheel_segment.floor();
    let primary = brightness * (1.0 - saturation);
    let secondary = brightness * (1.0 - saturation * color_wheel_offset);
    let tertiary = brightness * (1.0 - saturation * (1.0 - color_wheel_offset));
    match color_wheel_segment as i32 {
        0 => {
            r = math_round_f32(brightness * 255.0) as i32;
            g = math_round_f32(tertiary * 255.0) as i32;
            b = math_round_f32(primary * 255.0) as i32;
        }
        1 => {
            r = math_round_f32(secondary * 255.0) as i32;
            g = math_round_f32(brightness * 255.0) as i32;
            b = math_round_f32(primary * 255.0) as i32;
        }
        2 => {
            r = math_round_f32(primary * 255.0) as i32;
            g = math_round_f32(brightness * 255.0) as i32;
            b = math_round_f32(tertiary * 255.0) as i32;
        }
        3 => {
            r = math_round_f32(primary * 255.0) as i32;
            g = math_round_f32(secondary * 255.0) as i32;
            b = math_round_f32(brightness * 255.0) as i32;
        }
        4 => {
            r = math_round_f32(tertiary * 255.0) as i32;
            g = math_round_f32(primary * 255.0) as i32;
            // `brightness`, NOT `secondary`. The decompiled source says `secondaryColor` here and
            // is WRONG -- see the note on `set_brightness` and DESIGN_DECISIONS.md
            // (`#decompiler-artifacts`). Caught by golden row 146: colour 65791 at brightness
            // 0.5, where the source's formula gives blue 127 and the jar gives 128.
            b = math_round_f32(brightness * 255.0) as i32;
        }
        5 => {
            r = math_round_f32(brightness * 255.0) as i32;
            g = math_round_f32(primary * 255.0) as i32;
            b = math_round_f32(secondary * 255.0) as i32;
        }
        _ => {}
    }
    color_4(a, r, g, b)
}

// ============================================================================
// JOML interop
// ============================================================================

/// Port of `ARGB#vector3fFromRGB24(int)`.
///
/// **Not covered by any golden group** -- `api_list.py` shows the method exists and this is a
/// faithful three-line transcription, but nothing in `batch2.txt` exercises it. Flagged rather
/// than claimed as verified.
#[inline]
pub fn vector3f_from_rgb24(color: i32) -> Vector3f {
    Vector3f::new(red_float(color), green_float(color), blue_float(color))
}

/// Port of `ARGB#vector4fFromARGB32(int)`.
///
/// `todo!("PORT-BLOCKED: JOML Vector4f")` -- `javacompat::joml` has `Vector3f` but no
/// `Vector4f`. Unblocked by porting `Vector4f`, which is a 4-field struct plus setters.
pub fn vector4f_from_argb32(color: i32) -> i32 {
    todo!("PORT-BLOCKED: JOML Vector4f is not ported")
}

/// Port of `ARGB#setVector4fFromARGB32(Vector4f,int)`.
pub fn set_vector4f_from_argb32() -> i32 {
    todo!("PORT-BLOCKED: JOML Vector4f is not ported")
}