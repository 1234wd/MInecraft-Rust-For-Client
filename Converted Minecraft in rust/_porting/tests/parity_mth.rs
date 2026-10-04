//! Parity tests: `net.minecraft.util.Mth` (Minecraft 26.2).
//!
//! Golden data: `_porting/test-data/mth.txt`, produced by `_porting/java-oracle`
//! running the ORIGINAL, UNMODIFIED `Mth.java`.
//!
//! Every assertion compares RAW IEEE-754 bits (`to_bits()`), never `==`, so
//! `-0.0` vs `+0.0`, subnormals and NaN payloads all have to match exactly.
//! The edge-case corpus includes negatives, +/-0.0, NaN, +/-Infinity,
//! `Float/Double.MIN_VALUE` (subnormal), `MAX_VALUE`, `MIN_NORMAL`, and the
//! float/double -> int saturation boundaries (2^31, 2^32, 2^53).

#![allow(non_snake_case)]

use minecraft_rust::javacompat::golden::{self, Golden, Row};
// `Mth` is both a module (one file per Java class) and a unit struct inside it, so
// the associated functions live at `.../util::Mth::Mth`. The tables are siblings of
// that struct, hence two imports from the same module path.
use minecraft_rust::net::minecraft::util::Mth::Mth;
use minecraft_rust::net::minecraft::util::Mth::SIN;

/// Runs `f` over every row of `method`, feeding it the golden inputs.
fn each(g: &Golden, method: &str, mut f: impl FnMut(&Row)) {
    let rows = g.rows(method);
    assert!(!rows.is_empty(), "golden group `{method}` is empty");
    for row in rows {
        f(row);
    }
}

fn count(g: &Golden) -> usize {
    g.method_names().iter().map(|m| g.rows(m).len()).sum()
}

// ---------------------------------------------------------------------------
// The 65536-entry sin lookup table
// ---------------------------------------------------------------------------

/// The table is verified as a WHOLE via an FNV-1a hash over all 65536 raw bits,
/// plus an explicit spot-check of ~1500 sampled entries. A single flipped entry
/// anywhere in the table changes the hash, so this is a complete check.
#[test]
fn sin_lookup_table_matches_bit_for_bit() {
    let g = Golden::load("mth.txt");

    const FNV_OFFSET: u64 = 0xCBF2_9CE4_8422_2325;
    const FNV_PRIME: u64 = 0x1000_0000_1B3;
    let mut h = FNV_OFFSET;
    for entry in SIN.iter() {
        let bits = entry.to_bits();
        for byte in 0..4 {
            h ^= ((bits >> (8 * byte)) & 0xFF) as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
    }

    let expected = match g.single("sinTableHash") {
        golden::Val::I64(v) => v as u64,
        ref other => panic!("sinTableHash returned {other:?}, expected i64"),
    };
    assert_eq!(h, expected, "Mth.SIN differs from vanilla: FNV-1a 0x{h:016x} != 0x{expected:016x}");

    each(&g, "sinTable", |row| golden::assert_f32_bits("sinTable", row, SIN[row.arg(0).as_i32() as usize]));
}

// ---------------------------------------------------------------------------
// Trig
// ---------------------------------------------------------------------------

#[test]
fn sin_and_cos() {
    let g = Golden::load("mth.txt");
    each(&g, "sin", |r| golden::assert_f64_bits("sin", r, Mth::sin(r.arg(0).as_f64()) as f64));
    each(&g, "cos", |r| golden::assert_f64_bits("cos", r, Mth::cos(r.arg(0).as_f64()) as f64));
}

// ---------------------------------------------------------------------------
// Rounding, conversion, clamping
// ---------------------------------------------------------------------------

#[test]
fn rounding_and_conversion() {
    let g = Golden::load("mth.txt");
    each(&g, "sqrt", |r| golden::assert_f32_bits("sqrt", r, Mth::sqrt(r.arg(0).as_f32())));
    each(&g, "floor", |r| golden::assert_i32("floor", r, Mth::floor_f32(r.arg(0).as_f32())));
    each(&g, "floor_d", |r| golden::assert_i32("floor_d", r, Mth::floor_f64(r.arg(0).as_f64())));
    // `Mth#lfloor` returns `long`, but the oracle widens it to `f64` via the
    // `DF` functional interface, so the golden rows are f64-encoded.
    each(&g, "lfloor", |r| golden::assert_f64_bits("lfloor", r, Mth::lfloor(r.arg(0).as_f64()) as f64));
    each(&g, "ceil", |r| golden::assert_i32("ceil", r, Mth::ceil_f32(r.arg(0).as_f32())));
    each(&g, "ceil_d", |r| golden::assert_i32("ceil_d", r, Mth::ceil_f64(r.arg(0).as_f64())));
    each(&g, "ceilLong", |r| golden::assert_i64("ceilLong", r, Mth::ceil_long(r.arg(0).as_f64())));
    each(&g, "frac_f", |r| golden::assert_f32_bits("frac_f", r, Mth::frac_f32(r.arg(0).as_f32())));
    each(&g, "frac_d", |r| golden::assert_f64_bits("frac_d", r, Mth::frac_f64(r.arg(0).as_f64())));
    each(&g, "sign", |r| golden::assert_i32("sign", r, Mth::sign(r.arg(0).as_f64())));
}

#[test]
fn abs_clamp_and_modular() {
    let g = Golden::load("mth.txt");
    each(&g, "abs", |r| golden::assert_f32_bits("abs", r, Mth::abs_f32(r.arg(0).as_f32())));
    each(&g, "abs_i", |r| golden::assert_i32("abs_i", r, Mth::abs_i32(r.arg(0).as_i32())));
    each(&g, "absMax_f", |r| golden::assert_f32_bits("absMax_f", r, Mth::abs_max_f32(r.arg(0).as_f32(), r.arg(1).as_f32())));
    each(&g, "absMax_d", |r| golden::assert_f64_bits("absMax_d", r, Mth::abs_max_f64(r.arg(0).as_f64(), r.arg(1).as_f64())));
    each(&g, "absMax_i", |r| golden::assert_i32("absMax_i", r, Mth::abs_max_i32(r.arg(0).as_i32(), r.arg(1).as_i32())));

    each(&g, "clamp_i", |r| golden::assert_i32("clamp_i", r, Mth::clamp_i32(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32())));
    each(&g, "clamp_l", |r| golden::assert_i64("clamp_l", r, Mth::clamp_i64(r.arg(0).as_i64(), r.arg(1).as_i64(), r.arg(2).as_i64())));
    each(&g, "clamp_f", |r| golden::assert_f32_bits("clamp_f", r, Mth::clamp_f32(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "clamp_d", |r| golden::assert_f64_bits("clamp_d", r, Mth::clamp_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64())));

    each(&g, "floorDiv", |r| golden::assert_i32("floorDiv", r, Mth::floor_div(r.arg(0).as_i32(), 7)));
    each(&g, "positiveModulo_i", |r| golden::assert_i32("positiveModulo_i", r, Mth::positive_modulo_i32(r.arg(0).as_i32(), 7)));
    each(&g, "positiveModulo", |r| golden::assert_f64_bits("positiveModulo", r, Mth::positive_modulo_f64(r.arg(0).as_f64(), 7.0)));
    each(&g, "positiveModulo_f", |r| golden::assert_f32_bits("positiveModulo_f", r, Mth::positive_modulo_f32(r.arg(0).as_f32(), 7.0)));
    each(&g, "isMultipleOf", |r| golden::assert_bool("isMultipleOf", r, Mth::is_multiple_of(r.arg(0).as_i32(), 7)));
    each(&g, "positiveCeilDiv", |r| golden::assert_i32("positiveCeilDiv", r, Mth::positive_ceil_div_i32(r.arg(0).as_i32(), 7)));
    each(&g, "roundToward_i", |r| golden::assert_i32("roundToward_i", r, Mth::round_toward_i32(r.arg(0).as_i32(), 16)));
    each(&g, "roundToward_l", |r| golden::assert_i64("roundToward_l", r, Mth::round_toward_i64(r.arg(0).as_i64(), 16)));
}

// ---------------------------------------------------------------------------
// Bit twiddling
// ---------------------------------------------------------------------------

#[test]
fn bit_operations() {
    let g = Golden::load("mth.txt");
    each(&g, "smallestEncompassingPowerOfTwo", |r| golden::assert_i32("smallestEncompassingPowerOfTwo", r, Mth::smallest_encompassing_power_of_two(r.arg(0).as_i32())));
    each(&g, "smallestSquareSide", |r| golden::assert_i32("smallestSquareSide", r, Mth::smallest_square_side(r.arg(0).as_i32())));
    each(&g, "isPowerOfTwo_i", |r| golden::assert_bool("isPowerOfTwo_i", r, Mth::is_power_of_two_i32(r.arg(0).as_i32())));
    each(&g, "isPowerOfTwo_l", |r| golden::assert_bool("isPowerOfTwo_l", r, Mth::is_power_of_two_i64(r.arg(0).as_i64())));
    each(&g, "ceillog2", |r| golden::assert_i32("ceillog2", r, Mth::ceillog2(r.arg(0).as_i32())));
    each(&g, "log2", |r| golden::assert_i32("log2", r, Mth::log2(r.arg(0).as_i32())));
    each(&g, "murmurHash3Mixer", |r| golden::assert_i32("murmurHash3Mixer", r, Mth::murmur_hash3_mixer(r.arg(0).as_i32())));
    each(&g, "square_i", |r| golden::assert_i32("square_i", r, Mth::square_i32(r.arg(0).as_i32())));
    each(&g, "murmurHash3Mixer", |r| golden::assert_i32("murmurHash3Mixer", r, Mth::murmur_hash3_mixer(r.arg(0).as_i32())));
    each(&g, "square_l", |r| golden::assert_i64("square_l", r, Mth::square_i64(r.arg(0).as_i64())));
    each(&g, "chessboardDistance", |r| golden::assert_i32("chessboardDistance", r, Mth::chessboard_distance(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32(), r.arg(3).as_i32())));
}

// ---------------------------------------------------------------------------
// Angles -- everything that turns a block into a position
// ---------------------------------------------------------------------------

#[test]
fn angles() {
    let g = Golden::load("mth.txt");
    each(&g, "packDegrees", |r| golden::assert_i32("packDegrees", r, Mth::pack_degrees(r.arg(0).as_f32()) as i32));
    each(&g, "unpackDegrees", |r| golden::assert_f32_bits("unpackDegrees", r, Mth::unpack_degrees(r.arg(0).as_i32() as i8)));
    each(&g, "wrapDegrees_i", |r| golden::assert_i32("wrapDegrees_i", r, Mth::wrap_degrees_i32(r.arg(0).as_i32())));
    each(&g, "wrapDegrees_l", |r| golden::assert_f32_bits("wrapDegrees_l", r, Mth::wrap_degrees_i64(r.arg(0).as_i64())));
    each(&g, "wrapDegrees_f", |r| golden::assert_f32_bits("wrapDegrees_f", r, Mth::wrap_degrees_f32(r.arg(0).as_f32())));
    each(&g, "wrapDegrees_d", |r| golden::assert_f64_bits("wrapDegrees_d", r, Mth::wrap_degrees_f64(r.arg(0).as_f64())));
    each(&g, "wrapDegrees90", |r| golden::assert_f32_bits("wrapDegrees90", r, Mth::wrap_degrees_90(r.arg(0).as_f32())));
    each(&g, "degreesDifference", |r| golden::assert_f32_bits("degreesDifference", r, Mth::degrees_difference(r.arg(0).as_f32(), r.arg(1).as_f32())));
    each(&g, "degreesDifferenceAbs", |r| golden::assert_f32_bits("degreesDifferenceAbs", r, Mth::degrees_difference_abs(r.arg(0).as_f32(), r.arg(1).as_f32())));
    each(&g, "rotateIfNecessary", |r| golden::assert_f32_bits("rotateIfNecessary", r, Mth::rotate_if_necessary(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "approach", |r| golden::assert_f32_bits("approach", r, Mth::approach(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "approachDegrees", |r| golden::assert_f32_bits("approachDegrees", r, Mth::approach_degrees(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "rotLerp_f", |r| golden::assert_f32_bits("rotLerp_f", r, Mth::rot_lerp_f32(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "rotLerp_d", |r| golden::assert_f64_bits("rotLerp_d", r, Mth::rot_lerp_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64())));
    each(&g, "rotLerpRad", |r| golden::assert_f32_bits("rotLerpRad", r, Mth::rot_lerp_rad(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "triangleWave", |r| golden::assert_f32_bits("triangleWave", r, Mth::triangle_wave(r.arg(0).as_f32(), 4.0)));
}

// ---------------------------------------------------------------------------
// Interpolation
// ---------------------------------------------------------------------------

#[test]
fn interpolation() {
    let g = Golden::load("mth.txt");
    each(&g, "clampedLerp_f", |r| golden::assert_f32_bits("clampedLerp_f", r, Mth::clamped_lerp_f32(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "clampedLerp_d", |r| golden::assert_f64_bits("clampedLerp_d", r, Mth::clamped_lerp_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64())));
    each(&g, "lerp_f", |r| golden::assert_f32_bits("lerp_f", r, Mth::lerp_f32(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "lerp_d", |r| golden::assert_f64_bits("lerp_d", r, Mth::lerp_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64())));
    each(&g, "lerpInt", |r| golden::assert_i32("lerpInt", r, Mth::lerp_int(r.arg(0).as_f32(), r.arg(1).as_i32(), r.arg(2).as_i32())));
    each(&g, "lerpDiscrete", |r| golden::assert_i32("lerpDiscrete", r, Mth::lerp_discrete(r.arg(0).as_f32(), r.arg(1).as_i32(), r.arg(2).as_i32())));
    each(&g, "inverseLerp_f", |r| golden::assert_f32_bits("inverseLerp_f", r, Mth::inverse_lerp_f32(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "inverseLerp_d", |r| golden::assert_f64_bits("inverseLerp_d", r, Mth::inverse_lerp_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64())));
    each(&g, "catmullrom", |r| golden::assert_f32_bits("catmullrom", r, Mth::catmullrom(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32(), r.arg(3).as_f32(), r.arg(4).as_f32())));
    each(&g, "smoothstep", |r| golden::assert_f64_bits("smoothstep", r, Mth::smoothstep(r.arg(0).as_f64())));
    each(&g, "smoothstepDerivative", |r| golden::assert_f64_bits("smoothstepDerivative", r, Mth::smoothstep_derivative(r.arg(0).as_f64())));
    each(&g, "lerp2", |r| golden::assert_f64_bits("lerp2", r, Mth::lerp_2(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64(), r.arg(3).as_f64(), r.arg(4).as_f64(), r.arg(5).as_f64())));
    each(&g, "lerp3", |r| golden::assert_f64_bits("lerp3", r, Mth::lerp_3(
        r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64(), r.arg(3).as_f64(), r.arg(4).as_f64(),
        r.arg(5).as_f64(), r.arg(6).as_f64(), r.arg(7).as_f64(), r.arg(8).as_f64(), r.arg(9).as_f64(), r.arg(10).as_f64())));
    each(&g, "clampedMap_f", |r| golden::assert_f32_bits("clampedMap_f", r, Mth::clamped_map_f32(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32(), r.arg(3).as_f32(), r.arg(4).as_f32())));
    each(&g, "clampedMap_d", |r| golden::assert_f64_bits("clampedMap_d", r, Mth::clamped_map_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64(), r.arg(3).as_f64(), r.arg(4).as_f64())));
    each(&g, "map_f", |r| golden::assert_f32_bits("map_f", r, Mth::map_f32(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32(), r.arg(3).as_f32(), r.arg(4).as_f32())));
    each(&g, "map_d", |r| golden::assert_f64_bits("map_d", r, Mth::map_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64(), r.arg(3).as_f64(), r.arg(4).as_f64())));
}

// ---------------------------------------------------------------------------
// Roots and the hand-rolled atan2
// ---------------------------------------------------------------------------

#[test]
fn roots_and_atan2() {
    let g = Golden::load("mth.txt");
    each(&g, "atan2", |r| golden::assert_f64_bits("atan2", r, Mth::atan2(r.arg(0).as_f64(), r.arg(0).as_f64() * 0.5)));
    each(&g, "fastInvSqrt", |r| golden::assert_f64_bits("fastInvSqrt", r, Mth::fast_inv_sqrt(r.arg(0).as_f64())));
    each(&g, "invSqrt_f", |r| golden::assert_f32_bits("invSqrt_f", r, Mth::inv_sqrt_f32(r.arg(0).as_f32())));
    each(&g, "invSqrt_d", |r| golden::assert_f64_bits("invSqrt_d", r, Mth::inv_sqrt_f64(r.arg(0).as_f64())));
    each(&g, "fastInvCubeRoot", |r| golden::assert_f32_bits("fastInvCubeRoot", r, Mth::fast_inv_cube_root(r.arg(0).as_f32())));
    each(&g, "square_f", |r| golden::assert_f32_bits("square_f", r, Mth::square_f32(r.arg(0).as_f32())));
    each(&g, "square_d", |r| golden::assert_f64_bits("square_d", r, Mth::square_f64(r.arg(0).as_f64())));
    each(&g, "cube", |r| golden::assert_f32_bits("cube", r, Mth::cube(r.arg(0).as_f32())));
    each(&g, "lengthSquared2", |r| golden::assert_f64_bits("lengthSquared2", r, Mth::length_squared_2_f64(r.arg(0).as_f64(), r.arg(1).as_f64())));
    each(&g, "length2", |r| golden::assert_f64_bits("length2", r, Mth::length_2_f64(r.arg(0).as_f64(), r.arg(1).as_f64())));
    each(&g, "lengthF2", |r| golden::assert_f32_bits("lengthF2", r, Mth::length_2_f32(r.arg(0).as_f32(), r.arg(1).as_f32())));
    each(&g, "lengthSquared3", |r| golden::assert_f64_bits("lengthSquared3", r, Mth::length_squared_3_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64())));
    each(&g, "length3", |r| golden::assert_f64_bits("length3", r, Mth::length_3_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64())));
    each(&g, "lengthSquaredF3", |r| golden::assert_f32_bits("lengthSquaredF3", r, Mth::length_squared_3_f32(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32())));
    each(&g, "quantize", |r| golden::assert_i32("quantize", r, Mth::quantize(r.arg(0).as_f64(), r.arg(1).as_i32())));
    each(&g, "equal_f", |r| golden::assert_bool("equal_f", r, Mth::equal_f32(r.arg(0).as_f32(), r.arg(1).as_f32())));
    each(&g, "equal_d", |r| golden::assert_bool("equal_d", r, Mth::equal_f64(r.arg(0).as_f64(), r.arg(1).as_f64())));
}

// ---------------------------------------------------------------------------
// Seeds, strings, colour
// ---------------------------------------------------------------------------

#[test]
fn seeds_and_strings() {
    let g = Golden::load("mth.txt");
    each(&g, "getSeed3", |r| golden::assert_i64("getSeed3", r, Mth::get_seed(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32())));
    each(&g, "getInt", |r| golden::assert_i32("getInt", r, Mth::get_int(r.arg(0).as_opt_str(), r.arg(1).as_i32())));
    each(&g, "binarySearch", |r| golden::assert_i32(
        "binarySearch", r,
        Mth::binary_search(r.arg(0).as_i32(), r.arg(1).as_i32(), |i| i % 3 == 0),
    ));
    each(&g, "outFromOrigin", |r| golden::assert_ints(
        "outFromOrigin", r,
        &Mth::out_from_origin(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32()),
    ));
}

#[test]
fn random_backed_helpers() {
    let g = Golden::load("mth.txt");

    // These groups are TRANSCRIPTS: the oracle builds ONE `RandomSource` per seed
    // (and, for `nextInt_rand` / `nextIntBound_rand`, per range) and then draws N
    // times from it. Creating a fresh source per row would compare unrelated
    // stream positions and fail for reasons that have nothing to do with the port.
    //
    // `replay_stream` restarts the source whenever `key` changes and otherwise keeps
    // advancing it, which mirrors the oracle's nesting exactly.
    use minecraft_rust::net::minecraft::util::RandomSource::create_with_seed;

    let mut next_int_rng: Option<(i64, i32, i32, Box<dyn minecraft_rust::net::minecraft::util::RandomSource::RandomSource>)> = None;
    for r in g.rows("nextInt_rand") {
        let (seed, lo, hi) = (r.arg(0).as_i64(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let keep = next_int_rng.as_ref().map(|(s, l, h, _)| (*s, *l, *h)) == Some((seed, lo, hi));
        let rng = if keep {
            next_int_rng.as_mut().unwrap().3.as_mut()
        } else {
            let fresh = create_with_seed(seed);
            next_int_rng = Some((seed, lo, hi, fresh));
            next_int_rng.as_mut().unwrap().3.as_mut()
        };
        golden::assert_i32("nextInt_rand", r, Mth::next_int(rng, lo, hi));
    }

    let mut bound_rng: Option<(i64, i32, Box<dyn minecraft_rust::net::minecraft::util::RandomSource::RandomSource>)> = None;
    for r in g.rows("nextIntBound_rand") {
        let (seed, bound) = (r.arg(0).as_i64(), r.arg(1).as_i32());
        let keep = bound_rng.as_ref().map(|(s, b, _)| (*s, *b)) == Some((seed, bound));
        let rng = if keep {
            bound_rng.as_mut().unwrap().2.as_mut()
        } else {
            bound_rng = Some((seed, bound, create_with_seed(seed)));
            bound_rng.as_mut().unwrap().2.as_mut()
        };
        golden::assert_i32("nextIntBound_rand", r, rng.next_int_bounded(bound));
    }

    // These groups each build ONE source per SEED and draw N times from it, so the
    // source must be recreated whenever the seed changes and advanced otherwise.
    // The oracle re-uses a single `RandomSource r` for the whole seed, which is why
    // a fresh source per row would compare unrelated stream positions.
    let mut shared: Option<(i64, Box<dyn minecraft_rust::net::minecraft::util::RandomSource::RandomSource>)> = None;
    for (method, which) in [
        ("randomBetweenInclusive", 0usize),
        ("nextFloat_rand", 1),
        ("randomBetween", 2),
        ("nextDouble_rand", 3),
        ("normal", 4),
    ] {
        shared = None;
        for r in g.rows(method) {
            let seed = r.arg(0).as_i64();
            let keep = shared.as_ref().map(|(s, _)| *s) == Some(seed);
            let rng = if keep {
                shared.as_mut().unwrap().1.as_mut()
            } else {
                shared = Some((seed, create_with_seed(seed)));
                shared.as_mut().unwrap().1.as_mut()
            };
            match which {
                0 => golden::assert_i32(method, r, Mth::random_between_inclusive(rng, r.arg(1).as_i32(), r.arg(2).as_i32())),
                1 => golden::assert_f32_bits(method, r, Mth::next_float(rng, r.arg(1).as_f32(), r.arg(2).as_f32())),
                2 => golden::assert_f32_bits(method, r, Mth::random_between(rng, r.arg(1).as_f32(), r.arg(2).as_f32())),
                3 => golden::assert_f64_bits(method, r, Mth::next_double(rng, r.arg(1).as_f64(), r.arg(2).as_f64())),
                _ => golden::assert_f32_bits(method, r, Mth::normal(rng, r.arg(1).as_f32(), r.arg(2).as_f32())),
            }
        }
    }

    // Also a transcript: one source per seed, four draws.
    let mut origin_rng: Option<(i64, Box<dyn minecraft_rust::net::minecraft::util::RandomSource::RandomSource>)> = None;
    for r in g.rows("nextIntOriginBound") {
        let seed = r.arg(0).as_i64();
        let keep = origin_rng.as_ref().map(|(s, _)| *s) == Some(seed);
        let rng = if keep {
            origin_rng.as_mut().unwrap().1.as_mut()
        } else {
            origin_rng = Some((seed, create_with_seed(seed)));
            origin_rng.as_mut().unwrap().1.as_mut()
        };
        golden::assert_i32("nextIntOriginBound", r, rng.next_int_origin_bound(r.arg(1).as_i32(), r.arg(2).as_i32()));
    }
    each(&g, "createInsecureUUID", |r| {
        let mut random = create_with_seed(r.arg(0).as_i64());
        let (most, least) = Mth::create_insecure_uuid(&mut *random);
        assert_eq!(
            (most, least),
            (r.exp(0).as_i64(), r.exp(1).as_i64()),
            "createInsecureUUID (golden line {}) seed {}", r.line, r.arg(0).as_i64()
        );
    });
}

// ---------------------------------------------------------------------------
// Coverage guard: every golden method group must be claimed by a test above.
// If someone adds a group to mth.txt and forgets to test it, this fails.
// ---------------------------------------------------------------------------

/// Groups the oracle emits that CANNOT be parity-tested yet, because their Java
/// signatures depend on classes that are still `SKELETON` (or on a third-party
/// library we have not ported). Listing them here is deliberate: it means the
/// coverage guard can still fail if a NEW untested group appears, without forcing
/// us to fake a test for something we cannot compile yet.
///
/// Each maps to an entry in _porting/OPEN_QUESTIONS.md and _porting/PORTING_PLAN.md.
/// Mth members with no parity test yet.
///
/// # TWO OF THESE NAMES WERE WRONG UNTIL SESSION 05
///
/// `javap -p` on the real 26.2 jar shows the methods are
///
/// ```text
/// public static long getSeed(Vec3i)
/// public static Vec3 lerp(double, Vec3, Vec3)
/// ```
///
/// There is no `getSeedVec3i` and no `lerpVec3`. Those names sat in this list from
/// sessions 02/03 and were copied from a plan document rather than checked. Completing
/// them by name would have written two `todo!()` stubs for methods that do not exist,
/// while leaving the two vanilla actually uses untested. See OPEN_QUESTIONS #19.
const BLOCKED_ON_UNPORTED_TYPES: &[&str] = &[
    "mulAndTruncate",      // needs commons-lang3 Fraction
    "rayIntersectsAABB",   // needs Vec3 + AABB
    "rotationAroundAxis",  // needs JOML Quaternionf/Vector3fc
    "getSeed",             // needs Vec3i
    "lerp",                // needs Vec3
];

#[test]
fn every_golden_group_is_covered() {
    let g = Golden::load("mth.txt");
    const COVERED: &[&str] = &[
        "sinTableHash", "sinTable", "sin", "cos", "sqrt", "floor", "floor_d", "lfloor", "abs", "abs_i",
        "ceil", "ceil_d", "ceilLong", "positiveModulo", "positiveModulo_f", "floorDiv", "positiveModulo_i",
        "isMultipleOf", "positiveCeilDiv", "roundToward_i", "roundToward_l",
        "smallestEncompassingPowerOfTwo", "smallestSquareSide", "isPowerOfTwo_i", "isPowerOfTwo_l",
        "ceillog2", "log2", "frac_f", "frac_d", "sign", "packDegrees", "unpackDegrees", "wrapDegrees_i",
        "wrapDegrees_l", "wrapDegrees_f", "wrapDegrees_d", "wrapDegrees90", "degreesDifference",
        "degreesDifferenceAbs", "rotateIfNecessary", "approach", "approachDegrees", "rotLerp_f", "rotLerp_d",
        "rotLerpRad", "triangleWave", "clampedLerp_f", "clampedLerp_d", "lerp_f", "lerp_d", "lerpInt",
        "lerpDiscrete", "inverseLerp_f", "inverseLerp_d", "catmullrom", "smoothstep", "smoothstepDerivative",
        "lerp2", "lerp3", "clampedMap_f", "clampedMap_d", "map_f", "map_d", "lengthSquared2", "length2",
        "lengthF2", "lengthSquared3", "length3", "lengthSquaredF3", "quantize", "equal_f", "equal_d",
        "atan2", "fastInvSqrt", "invSqrt_f", "invSqrt_d", "fastInvCubeRoot", "square_f", "square_d",
        "square_i", "square_l", "cube", "getSeed3", "getInt", "binarySearch", "outFromOrigin", "nextInt_rand",
        "randomBetweenInclusive", "nextFloat_rand", "randomBetween", "nextDouble_rand", "normal",
        "nextIntBound_rand", "createInsecureUUID", "absMax_i", "absMax_f", "absMax_d",
        "clamp_i", "clamp_l", "clamp_f", "clamp_d", "chessboardDistance",
        "murmurHash3Mixer", "nextIntOriginBound",
    ];

    let uncovered: Vec<&str> = g
        .method_names()
        .into_iter()
        .filter(|m| !COVERED.contains(m) && !BLOCKED_ON_UNPORTED_TYPES.contains(m))
        .collect();
    assert!(
        uncovered.is_empty(),
        "golden groups not claimed by any test: {uncovered:?}. \
         If a new group appears in the golden file, add a test for it -- or, if it \
         depends on an unported type, add it to BLOCKED_ON_UNPORTED_TYPES with a note."
    );

    // The blocked list must not grow silently: every entry is a real open question.
    for name in BLOCKED_ON_UNPORTED_TYPES {
        assert!(
            g.rows(name).len() > 0,
            "BLOCKED_ON_UNPORTED_TYPES lists `{name}` but the golden file has no such group -- \
             it has probably been ported now, so remove it and write a real test."
        );
    }
}

/// The `nextDouble` float-narrowing "quirk" that session 02 believed in was a
/// DECOMPILER ARTIFACT. The real jar widens `long -> double` with `l2d` and
/// multiplies by a `double` constant, so vanilla carries the full 53 bits.
///
/// This test exists to make the wrong value impossible to reintroduce: it asserts
/// the *correct* bits AND that the float-narrowed value does not appear. Session 03's
/// version of this test asserted the opposite and passed, because the oracle was
/// compiling the same decompiled source we were porting from -- two copies of the same
/// mistake agreeing with each other.
///
/// See `_porting/DESIGN_DECISIONS.md` (#decompiler-artifacts).
#[test]
fn legacy_next_double_carries_all_53_bits() {
    use minecraft_rust::net::minecraft::world::level::levelgen::BitRandomSource::BitRandomSource;
    use minecraft_rust::net::minecraft::world::level::levelgen::LegacyRandomSource::LegacyRandomSource;

    let mut r = LegacyRandomSource::new(0);
    assert_eq!(BitRandomSource::next(&mut r, 26), 0x02ec82d1);
    assert_eq!(BitRandomSource::next(&mut r, 27), 0x006a6ca89);

    let mut r = LegacyRandomSource::new(0);
    let got = BitRandomSource::next_double(&mut r);
    assert_eq!(
        got.to_bits(),
        0x3fe764168ea6ca89,
        "vanilla widens long->double directly; the low 32 bits survive"
    );
    // The value the decompiled source produces. It must NOT be what we return.
    assert_ne!(
        got.to_bits(),
        0x3fe7641680000000,
        "this is the float-narrowed (decompiled-source) value -- a decompiler artifact"
    );
}

/// Sanity check that the reader itself works and the file is non-trivial.
#[test]
fn golden_file_is_substantial() {
    let g = Golden::load("mth.txt");
    let total = count(&g);
    assert!(total > 5000, "expected thousands of golden rows, found {total}");
}