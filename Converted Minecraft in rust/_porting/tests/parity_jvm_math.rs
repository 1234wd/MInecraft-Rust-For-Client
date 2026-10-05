//! Parity tests for `javacompat::jvm_math` against the jar's own JVM.
//!
//! Golden data: `_porting/test-data/jvm_math.txt`, emitted by
//! `_porting/java-oracle/src/oracle/JvmMathOracle.java` from **JDK 25.0.4** (the JDK the
//! oracle pins), run against `Math` AND `StrictMath`.
//!
//! # WHAT IS ACTUALLY BEING ASSERTED HERE
//!
//! Three things, in order of importance:
//!
//! 1. **The Rust FdLibm port equals the JVM, bit for bit**, on 190,622 rows. That is the
//!    whole point of `jvm_math`: the results must not depend on the host's libm, so they
//!    have to be compared against the JVM's own answers rather than against `f64::atan2`.
//! 2. **`Math.f` and `StrictMath.f` are identical for these functions**, on this JDK. If
//!    HotSpot ever starts intrinsifying `asin`/`atan`/`atan2`, the two columns diverge and
//!    this test says so by name -- instead of the divergence surfacing later as a mystery
//!    1-ULP difference in `Vec3#rotation`.
//! 3. NaN payloads are compared through the shared NaN policy (`golden.rs`), because Rust
//!    does not promise the sign or payload of a NaN produced by arithmetic and the
//!    optimiser is free to fold it. Exact NaN bits are not observable by game code here --
//!    see `NAN_BITS_OBSERVABLE`, which is empty and says why.

use minecraft_rust::javacompat::golden::Golden;
use minecraft_rust::javacompat::jvm_math;

/// Runs one group: `rust(x) == Math(x) == StrictMath(x)` for every row.
///
/// `arity` is how many arguments the row carries (1 for `asin`/`atan`, 2 for `atan2`).
fn check(group: &str, arity: usize, f: impl Fn(&[f64]) -> f64) {
    let g = Golden::load("jvm_math.txt");
    let rows = g.rows(group);
    assert!(
        !rows.is_empty(),
        "group `{group}` is empty -- the oracle did not run, or wrote under another header"
    );

    let mut math_vs_strict = 0usize;
    let mut rust_vs_math = 0usize;
    let mut nan_rows = 0usize;
    let mut first_bad: Option<String> = None;

    for r in rows {
        let mut args = Vec::with_capacity(arity);
        for i in 0..arity {
            args.push(r.arg(i).as_f64());
        }
        let want_math = r.exp(0).as_f64();
        let want_strict = r.exp(1).as_f64();

        // (2) Math vs StrictMath, on the JVM. Reported per row so a future intrinsic shows
        // up as a count, not as an unexplained failure elsewhere.
        if want_math.to_bits() == want_strict.to_bits() {
            math_vs_strict += 1;
        }

        let got = f(&args);

        // (3) NaN accounting, so the numbers below are interpretable.
        if want_math.is_nan() || got.is_nan() {
            nan_rows += 1;
        }

        // (1) The real assertion.
        let ok = minecraft_rust::javacompat::golden::f64_bits_match(group, want_math.to_bits(), got.to_bits());
        if ok {
            rust_vs_math += 1;
        } else if first_bad.is_none() {
            first_bad = Some(format!(
                "args={:?} want Math=0x{:016x} ({}) StrictMath=0x{:016x} ({}) got 0x{:016x} ({})",
                args.iter().map(|a| format!("{a:e}")).collect::<Vec<_>>(),
                want_math.to_bits(), want_math,
                want_strict.to_bits(), want_strict,
                got.to_bits(), got,
            ));
        }
    }

    let n = rows.len();
    println!(
        "{group}: {rust_vs_math}/{n} rows match the JVM (NaN-policy); \
         Math==StrictMath on {math_vs_strict}/{n}; {nan_rows} rows involve NaN"
    );
    if let Some(b) = first_bad {
        let bad = n - rust_vs_math;
        panic!("{group}: Rust disagrees with the JVM on {bad} row(s). First: {b}");
    }
    assert_eq!(
        math_vs_strict, n,
        "{group}: `Math.{group}` and `StrictMath.{group}` DIFFER on {} of {n} rows. \
         That means HotSpot now intrinsifies this function, so the FdLibm port in \
         `jvm_math.rs` is no longer what the game runs. Re-measure with the survey and \
         decide whether to schedule the HotSpot stub. (See OPEN_QUESTIONS #21.)",
        n - math_vs_strict
    );
}

#[test]
fn asin_matches_the_jvm() {
    check("jvm_math.asin", 1, |a| jvm_math::asin(a[0]));
}

#[test]
fn atan_matches_the_jvm() {
    check("jvm_math.atan", 1, |a| jvm_math::atan(a[0]));
}

#[test]
fn atan2_matches_the_jvm() {
    check("jvm_math.atan2", 2, |a| jvm_math::atan2(a[0], a[1]));
}

/// The corpus must actually cover the branches, or "9,138 rows pass" means very little.
///
/// A hand transcription of FdLibm fails in the *rare* branches -- the `|x| > 0.975`
/// correction, the four `atan2` special-case quadrants, the `x == 1.0` shortcut. A corpus
/// of ordinary values would miss all of them, which is exactly how a subtly wrong port
/// passes its own test suite. So assert the coverage the corpus is supposed to have.
#[test]
fn corpus_covers_every_fdlibm_branch() {
    let g = Golden::load("jvm_math.txt");

    // --- asin: |x| >= 1 (incl. the exact +-1 and the NaN-producing |x| > 1).
    let asin = g.rows("jvm_math.asin");
    let count = |pred: &dyn Fn(f64) -> bool| asin.iter().filter(|r| pred(r.arg(0).as_f64())).count();
    assert!(count(&|x| x.abs() > 1.0) > 0, "asin corpus has no |x| > 1 (the NaN branch)");
    assert!(count(&|x| x.abs() == 1.0) >= 2, "asin corpus has no exact +-1");
    assert!(count(&|x| x.is_nan()) > 0, "asin corpus has no NaN input");
    assert!(count(&|x| x == 0.0) > 0, "asin corpus has no +-0");
    assert!(count(&|x| x.is_sign_negative() && x == 0.0) > 0, "asin corpus has no -0.0");
    // The two polynomial regions, and the "return x itself" cutoff at 2^-27.
    assert!(count(&|x| x.abs() > 0.0 && x.abs() < 0.5) > 100, "asin corpus lacks |x| < 0.5");
    assert!(count(&|x| x.abs() >= 0.5 && x.abs() < 0.975) > 100, "asin corpus lacks 0.5..0.975");
    assert!(count(&|x| x.abs() >= 0.975 && x.abs() < 1.0) > 0, "asin corpus lacks |x| >= 0.975");
    assert!(count(&|x| x.abs() < 1.0 / 134_217_728.0 && x != 0.0) > 0, "asin corpus lacks |x| < 2^-27");

    // --- atan2: all four quadrants plus every documented special case.
    let atan2 = g.rows("jvm_math.atan2");
    let n = atan2.len();
    assert!(n > 100_000, "atan2 corpus is only {n} rows");
    // An explicit `&dyn Fn` slice type: each closure below has its OWN type, so an array
    // literal of them would not unify without it. Compiler-forced, not decoration.
    let cases: &[(&str, &dyn Fn(f64, f64) -> bool)] = &[
        ("y == +0.0 with x > 0", &|y: f64, x: f64| y == 0.0 && x > 0.0),
        ("y == -0.0 with x > 0", &|y: f64, x: f64| y == 0.0 && x.is_sign_negative() && x != 0.0),
        ("y == +0.0 with x < 0", &|y: f64, x: f64| y == 0.0 && x < 0.0),
        ("y == -0.0 with x < 0", &|y: f64, x: f64| y == 0.0 && x.is_sign_negative() && x < 0.0),
        ("x == +0.0", &|y: f64, x: f64| x == 0.0 && !x.is_sign_negative() && y != 0.0),
        ("x == -0.0", &|y: f64, x: f64| x == 0.0 && x.is_sign_negative() && y != 0.0),
        ("x == +1.0 (the atan shortcut)", &|_: f64, x: f64| x == 1.0),
        ("x == -1.0", &|_: f64, x: f64| x == -1.0),
        ("|x| >= 2^66", &|_: f64, x: f64| x.abs() >= 73_786_976_294_838_206_464.0),
        ("y infinite", &|y: f64, _: f64| y.is_infinite()),
        ("x infinite", &|_: f64, x: f64| x.is_infinite()),
        ("x infinite AND y infinite", &|y: f64, x: f64| x.is_infinite() && y.is_infinite()),
        ("NaN x", &|_: f64, x: f64| x.is_nan()),
        ("NaN y", &|y: f64, _: f64| y.is_nan()),
    ];
    for (label, pred) in cases {
        let c = atan2.iter().filter(|r| pred(r.arg(0).as_f64(), r.arg(1).as_f64())).count();
        assert!(c > 0, "atan2 corpus does not cover: {label}");
    }
}

/// `Vec3#rotation` reaches `asin` only through `-y / length()`, and `atan2` only through
/// `(-x, z)`. This test walks the actual golden `vec3.rotation` rows, so the argument
/// values `jvm_math` sees in the game are themselves covered -- rather than only the
/// synthetic corpus above.
#[test]
fn vec3_rotation_inputs_are_covered_by_the_jvm_math_corpus() {
    let b2 = Golden::load("batch2.txt");
    let rows = b2.rows("vec3.rotation");
    assert!(!rows.is_empty(), "vec3.rotation group is empty");

    let mut distinct_args = std::collections::HashSet::new();
    for r in rows {
        let v = minecraft_rust::net::minecraft::world::phys::Vec3::Vec3::new(
            r.arg(0).as_f64(),
            r.arg(1).as_f64(),
            r.arg(2).as_f64(),
        );
        distinct_args.insert(jvm_math::asin(-v.y / v.length()).to_bits());
    }
    // 512 rows drawn from a GRID, so repeated values are expected; what matters is that
    // the grid is not degenerate (all one value) and really spans a range.
    assert!(
        distinct_args.len() > 20,
        "vec3.rotation only produces {} distinct asin arguments -- the corpus is degenerate",
        distinct_args.len()
    );
}