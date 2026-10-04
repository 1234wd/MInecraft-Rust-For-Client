//! Parity tests: the batch-2 core value types.
//!
//! Golden data: `_porting/test-data/core.txt`, produced by `_porting/java-oracle`
//! running the ORIGINAL, UNMODIFIED `Vec3i.java`, `Direction.java`, `BlockPos.java`,
//! `ChunkPos.java`, `Vec3.java`, `AABB.java`, `ARGB.java` and `Identifier.java`.
//!
//! # What is verified here vs. what is still a stub
//!
//! Only `Vec3i` and `Direction` are ported so far, so only their groups are claimed.
//! `BLOCKED_ON_UNPORTED_TYPES` names the rest. The coverage guard fails if a new group
//! appears unclaimed, so this file cannot silently rot as the batch proceeds.
//!
//! # The two things this batch is really about
//!
//! 1. **`hashCode` is not arbitrary.** `Vec3i#hashCode` weights z and y above x, which
//!    looks like a mistake and is not. It is the value Java's `HashMap` buckets on, and
//!    reproducing it is what makes iteration order match later.
//! 2. **Width changes hide in plain sight.** `distManhattan` sums in `float`; every
//!    neighbouring distance method uses `int`. Both are tested against extreme
//!    coordinates, where they disagree.

#![allow(non_snake_case)]

use minecraft_rust::javacompat::golden::{self, Golden};
use minecraft_rust::net::minecraft::core::Direction::{Axis, AxisDirection, Direction};
use minecraft_rust::net::minecraft::core::Vec3i::Vec3i;

fn each(g: &Golden, method: &str, mut f: impl FnMut(&minecraft_rust::javacompat::golden::Row)) {
    let rows = g.rows(method);
    assert!(!rows.is_empty(), "golden group `{method}` is empty");
    for row in rows {
        f(row);
    }
}

// ---------------------------------------------------------------------------
// Vec3i
// ---------------------------------------------------------------------------

#[test]
fn vec3i_hash_code_matches_java() {
    let g = Golden::load("core.txt");
    // 36^3 rows: every combination of the coordinate set on each axis.
    each(&g, "vec3i.hashCode", |r| {
        let v = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        golden::assert_i32("vec3i.hashCode", r, v.java_hash_code());
    });
    each(&g, "vec3i.hashEdge", |r| {
        let v = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        golden::assert_i32("vec3i.hashEdge", r, v.java_hash_code());
    });
}

#[test]
fn vec3i_equals_and_ordering() {
    let g = Golden::load("core.txt");
    each(&g, "vec3i.equals", |r| {
        let a = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let b = Vec3i::new(r.arg(3).as_i32(), r.arg(4).as_i32(), r.arg(5).as_i32());
        assert_eq!(a.java_equals(&b), r.exp(0).as_bool(), "vec3i.equals line {}", r.line);
    });
    each(&g, "vec3i.compareTo", |r| {
        let a = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let b = Vec3i::new(r.arg(3).as_i32(), r.arg(4).as_i32(), r.arg(5).as_i32());
        golden::assert_i32("vec3i.compareTo", r, a.compare_to(&b));
        // compareTo's SIGN is what matters, so the derived Ord must agree with it.
        assert_eq!(
            (a < b),
            a.compare_to(&b) < 0,
            "Ord disagrees with compareTo at line {}",
            r.line
        );
    });
}

#[test]
fn vec3i_arithmetic_wraps_like_java() {
    let g = Golden::load("core.txt");
    each(&g, "vec3i.offset", |r| {
        let p = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let got = p.offset(r.arg(3).as_i32(), r.arg(4).as_i32(), r.arg(5).as_i32());
        golden::assert_multi_i32("vec3i.offset", r, &[got.get_x(), got.get_y(), got.get_z()]);
    });
    each(&g, "vec3i.multiply", |r| {
        let p = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let got = p.multiply(r.arg(3).as_i32());
        golden::assert_multi_i32("vec3i.multiply", r, &[got.get_x(), got.get_y(), got.get_z()]);
    });
    each(&g, "vec3i.cross", |r| {
        let a = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let b = Vec3i::new(r.arg(3).as_i32(), r.arg(4).as_i32(), r.arg(5).as_i32());
        let got = a.cross(&b);
        golden::assert_multi_i32("vec3i.cross", r, &[got.get_x(), got.get_y(), got.get_z()]);
    });
    each(&g, "vec3i.relative", |r| {
        let p = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let dir = Direction::from_ordinal(r.arg(3).as_i32());
        let got = p.relative(dir, r.arg(4).as_i32());
        golden::assert_multi_i32("vec3i.relative", r, &[got.get_x(), got.get_y(), got.get_z()]);
    });
    each(&g, "vec3i.relativeAxis", |r| {
        let p = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let axis = Axis::from_ordinal(r.arg(3).as_i32());
        let got = p.relative_axis(axis, r.arg(4).as_i32());
        golden::assert_multi_i32("vec3i.relativeAxis", r, &[got.get_x(), got.get_y(), got.get_z()]);
    });
}

/// The interesting one. `distManhattan` sums in `f32`; `distChessboard` does not.
/// At the world border the two disagree, and vanilla's answer is the lossy one.
#[test]
fn vec3i_distances_including_the_float_narrowing() {
    let g = Golden::load("core.txt");
    each(&g, "vec3i.distManhattan", |r| {
        let a = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let b = Vec3i::new(r.arg(3).as_i32(), r.arg(4).as_i32(), r.arg(5).as_i32());
        golden::assert_i32("vec3i.distManhattan", r, a.dist_manhattan(&b));
    });
    each(&g, "vec3i.distChessboard", |r| {
        let a = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let b = Vec3i::new(r.arg(3).as_i32(), r.arg(4).as_i32(), r.arg(5).as_i32());
        golden::assert_i32("vec3i.distChessboard", r, a.dist_chessboard(&b));
    });
    each(&g, "vec3i.distSqr", |r| {
        let a = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        let b = Vec3i::new(r.arg(3).as_i32(), r.arg(4).as_i32(), r.arg(5).as_i32());
        golden::assert_f64_bits("vec3i.distSqr", r, a.dist_sqr(&b));
    });
    each(&g, "vec3i.distToCenterSqr", |r| {
        let a = Vec3i::new(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32());
        golden::assert_f64_bits(
            "vec3i.distToCenterSqr",
            r,
            a.dist_to_center_sqr(r.arg(3).as_f64(), r.arg(4).as_f64(), r.arg(5).as_f64()),
        );
    });
}

// ---------------------------------------------------------------------------
// Direction
// ---------------------------------------------------------------------------

/// Ordinals are persisted to `level.dat` and sent over the wire as `data3d`, so this
/// is the single highest-stakes test in the batch.
#[test]
fn direction_ordinals_and_data_values() {
    let g = Golden::load("core.txt");
    each(&g, "direction.ordinal", |r| {
        let d = Direction::from_ordinal(r.exp(0).as_i32());
        assert_eq!(d.ordinal(), r.exp(0).as_i32(), "line {}", r.line);
        assert_eq!(d.get_3d_data_value(), r.exp(1).as_i32(), "data3d line {}", r.line);
        assert_eq!(d.get_2d_data_value(), r.exp(2).as_i32(), "data2d line {}", r.line);
        assert_eq!(d.get_axis().ordinal(), r.exp(3).as_i32(), "axis line {}", r.line);
        assert_eq!(d.get_axis_direction().ordinal(), r.exp(4).as_i32(), "axisDir line {}", r.line);
        assert_eq!(d.name(), r.exp(5).as_opt_str().expect("direction name"), "name line {}", r.line);
    });
    each(&g, "direction.step", |r| {
        let d = Direction::from_ordinal(r.arg(0).as_i32());
        golden::assert_multi_i32(
            "direction.step",
            r,
            &[d.get_step_x(), d.get_step_y(), d.get_step_z()],
        );
    });
    each(&g, "direction.opposite", |r| {
        let d = Direction::from_ordinal(r.arg(0).as_i32());
        golden::assert_i32("direction.opposite", r, d.get_opposite().ordinal());
    });
    each(&g, "direction.axisGet", |r| {
        let axis = Axis::from_ordinal(r.exp(0).as_i32());
        assert_eq!(axis.get_positive().ordinal(), r.exp(1).as_i32());
        assert_eq!(axis.get_negative().ordinal(), r.exp(2).as_i32());
        assert_eq!(axis.is_vertical(), r.exp(3).as_i32() == 1);
    });
    each(&g, "direction.axisDirection", |r| {
        let ad = if r.exp(0).as_i32() == 0 { AxisDirection::Positive } else { AxisDirection::Negative };
        assert_eq!(ad.ordinal(), r.exp(0).as_i32());
        assert_eq!(ad.get_step(), r.exp(1).as_i32());
    });
}

/// `BY_3D_DATA` sorts by `data3d`; `BY_2D_DATA` filters to horizontal FIRST, so its
/// modulus is 4. Getting the filter wrong is the classic bug here.
#[test]
fn direction_data_value_lookup() {
    let g = Golden::load("core.txt");
    // `BY_ID` is ByIdMap.continuous(..., WRAP), i.e. the same abs(x % n) rule as
    // from3DDataValue. Test it separately anyway: the rules call it out, and it is the
    // lookup the network path uses.
    each(&g, "direction.byId", |r| {
        let data = r.arg(0).as_i32();
        assert_eq!(Direction::by_id(data).ordinal(), r.exp(0).as_i32(), "by_id({data})");
    });

    each(&g, "direction.fromData", |r| { 
        let data = r.arg(0).as_i32();
        assert_eq!(
            Direction::from_3d_data_value(data).ordinal(),
            r.exp(0).as_i32(),
            "from3DDataValue({data})"
        );
        assert_eq!(
            Direction::from_2d_data_value(data).ordinal(),
            r.exp(1).as_i32(),
            "from2DDataValue({data})"
        );
    });
}

/// `getClockWise` throws for the vertical directions; the oracle encodes that as -1.
#[test]
fn direction_rotations_including_the_thrown_cases() {
    let g = Golden::load("core.txt");
    each(&g, "direction.rotations", |r| {
        let d = Direction::from_ordinal(r.arg(0).as_i32());
        let axes = [Axis::X, Axis::Y, Axis::Z];
        for (i, axis) in axes.iter().enumerate() {
            // The oracle interleaves the pair per axis: [cw(X), ccw(X), cw(Y), ccw(Y),
            // cw(Z), ccw(Z)]. So cw is at 2*i and ccw at 2*i+1 -- not 3+i, which
            // silently reads ccw(Y) while claiming to check ccw(X).
            let got_cw = catch_direction(|| d.get_clock_wise_axis(*axis));
            let got_ccw = catch_direction(|| d.get_counter_clock_wise_axis(*axis));
            assert_eq!(got_cw, r.exp(2 * i).as_i32(), "cw about {axis:?} line {}", r.line);
            assert_eq!(got_ccw, r.exp(2 * i + 1).as_i32(), "ccw about {axis:?} line {}", r.line);
        }
        // The last two columns are the UNGUARDED Y rotations, which throw for
        // UP/DOWN -- the oracle encodes those as -1.
        assert_eq!(catch_direction(|| d.get_clock_wise()), r.exp(6).as_i32(), "cw(Y) line {}", r.line);
        assert_eq!(
            catch_direction(|| d.get_counter_clock_wise()),
            r.exp(7).as_i32(),
            "ccw(Y) line {}",
            r.line
        );
    });
}

/// `getApproximateNearest` seeds `highestDot` with `Float.MIN_VALUE` (a tiny POSITIVE
/// value), so a zero or slightly-negative dot never wins. Seeding with `-Infinity`
/// would pick a different direction.
#[test]
fn direction_get_nearest_and_approximate_nearest() {
    let g = Golden::load("core.txt");
    each(&g, "direction.getNearest", |r| {
        let got = Direction::get_nearest(r.arg(0).as_i32(), r.arg(1).as_i32(), r.arg(2).as_i32(), None);
        let got_ordinal = got.map(|d| d.ordinal()).unwrap_or(-1);
        golden::assert_i32("direction.getNearest", r, got_ordinal);
    });
    each(&g, "direction.getApproximateNearest_f", |r| {
        golden::assert_i32(
            "direction.getApproximateNearest_f",
            r,
            Direction::get_approximate_nearest_f32(r.arg(0).as_f32(), r.arg(1).as_f32(), r.arg(2).as_f32()).ordinal(),
        );
    });
    each(&g, "direction.getApproximateNearest_d", |r| {
        golden::assert_i32(
            "direction.getApproximateNearest_d",
            r,
            Direction::get_approximate_nearest_f64(r.arg(0).as_f64(), r.arg(1).as_f64(), r.arg(2).as_f64()).ordinal(),
        );
    });
    each(&g, "direction.fromYRot", |r| {
        golden::assert_i32("direction.fromYRot", r, Direction::from_y_rot(r.arg(0).as_f64()).ordinal());
    });
}

/// Java throws `IllegalStateException` where the Rust port panics. Both are crashes,
/// so the panic is the faithful translation -- this test just proves the two agree on
/// WHICH inputs crash, which is the part that could drift.
fn catch_direction(f: impl Fn() -> Direction + std::panic::UnwindSafe) -> i32 {
    match std::panic::catch_unwind(f) {
        Ok(d) => d.ordinal(),
        Err(_) => -1,
    }
}

// ---------------------------------------------------------------------------
// Coverage guard
// ---------------------------------------------------------------------------

/// Every golden group must be claimed by a test, or listed here with a reason.
///
/// This is what stops the file rotting as the batch is ported incrementally: adding a
/// group to the oracle without adding a test (or a BLOCKED entry) fails the build.
const COVERED: &[&str] = &[
    "vec3i.hashCode",
    "vec3i.hashEdge",
    "vec3i.equals",
    "vec3i.compareTo",
    "vec3i.offset",
    "vec3i.multiply",
    "vec3i.cross",
    "vec3i.relative",
    "vec3i.relativeAxis",
    "vec3i.distManhattan",
    "vec3i.distChessboard",
    "vec3i.distSqr",
    "vec3i.distToCenterSqr",
    "direction.ordinal",
    "direction.step",
    "direction.opposite",
    "direction.axisGet",
    "direction.axisDirection",
    "direction.byId",
    "direction.fromData",
    "direction.rotations",
    "direction.getNearest",
    "direction.getApproximateNearest_f",
    "direction.getApproximateNearest_d",
    "direction.fromYRot",
];

/// Groups the oracle emits that cannot be tested yet, because the corresponding Rust
/// file is still `SKELETON`. Listing them is deliberate: the guard below still fails if
/// a NEW untested group appears, and also fails if one of these stops existing (i.e.
/// has quietly been deleted from the oracle).
const BLOCKED_ON_UNPORTED_TYPES: &[&str] = &[
    "blockpos.asLong",          // BlockPos.rs not ported
    "blockpos.fromLong",        //
    "blockpos.hashCode",        //
    "blockpos.withOffset",      //
    "chunkpos.toLong",          // ChunkPos.rs not ported
    "chunkpos.hashCode",        //
    "chunkpos.accessors",       //
    "chunkpos.fromLong",        //
    "chunkpos.minMaxBlock",     //
    "vec3.arith",               // Vec3.rs not ported
    "vec3.dot_cross_len",       //
    "vec3.normalize",           //
    "vec3.hashCode",            //
    "aabb.minmax",              // AABB.rs not ported
    "aabb.contains",            //
    "aabb.inflate",             //
    "argb.channels",            // ARGB.rs not ported
    "argb.combine",             //
    "argb.fromFloat",           //
    "identifier.tryParse",      // Identifier.rs not ported
    "identifier.parse",         //
    "identifier.withDefaultNamespace", //
    "identifier.tryBuild",      //
    "identifier.hashEquals",    //
    "identifier.namespacePath", //
];

#[test]
fn every_core_golden_group_is_covered() {
    let g = Golden::load("core.txt");
    let uncovered: Vec<&str> = g
        .method_names()
        .into_iter()
        .filter(|m| !COVERED.contains(m) && !BLOCKED_ON_UNPORTED_TYPES.contains(m))
        .collect();
    assert!(
        uncovered.is_empty(),
        "core.txt groups not claimed by any test: {uncovered:?}.\n\
         Add a test, or add it to BLOCKED_ON_UNPORTED_TYPES with a note saying which\n\
         unported file blocks it."
    );

    for name in BLOCKED_ON_UNPORTED_TYPES {
        assert!(
            !g.rows(name).is_empty(),
            "BLOCKED_ON_UNPORTED_TYPES lists `{name}` but the oracle no longer emits it -- \
             it has probably been ported, so remove it and write a real test."
        );
    }
}

#[test]
fn core_golden_file_is_substantial() {
    let g = Golden::load("core.txt");
    let total: usize = g.method_names().iter().map(|m| g.rows(m).len()).sum();
    assert!(total > 100_000, "core.txt only has {total} rows; the oracle regressed");
    assert!(g.method_names().len() >= 45, "expected 45+ groups, got {}", g.method_names().len());
}
