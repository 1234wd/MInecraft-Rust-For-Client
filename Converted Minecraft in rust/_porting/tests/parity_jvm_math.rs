//! Parity tests for `javacompat::jvm_math` against the jar's own JVM.
//!
//! Golden data: `_porting/test-data/jvm_math.txt`, emitted by
//! `_porting/java-oracle/src/oracle/JvmMathOracle.java` from **JDK 25.0.4** (the JDK the
//! oracle pins), run against `Math` AND `StrictMath`.
//!
//! # WHAT IS ACTUALLY BEING ASSERTED HERE
//!
//! Four things, in order of importance:
//!
//! 1. **The Rust FdLibm ports equal the JVM, bit for bit**, on 613,221 rows. That is the whole
//!    point of `jvm_math`: results must not depend on the host's libm, so they have to be
//!    compared against the JVM's own answers rather than against `f64::atan2`.
//! 2. **`Math.f` and `StrictMath.f` are identical for `asin`/`atan`/`atan2`**, on this JDK. If
//!    HotSpot ever starts intrinsifying one of them, the two columns diverge and this test says
//!    so by name -- instead of the divergence surfacing later as a mystery 1-ULP difference in
//!    `Vec3#rotation`.
//! 3. **`log` is NOT bit-exact, because it cannot be**, and the gap is pinned precisely. See
//!    `log_is_pure_rust_and_pinned` below.
//! 4. NaN payloads are compared through the shared NaN policy (`golden.rs`), because Rust does
//!    not promise the sign or payload of a NaN produced by arithmetic and the optimiser is free
//!    to fold it. Exact NaN bits are not observable by game code here -- see
//!    `NAN_BITS_OBSERVABLE`, which is empty and says why.
//!
//! # A NOTE ON GROUP COVERAGE
//!
//! `asin`, `atan` and `atan2` are asserted STRICTLY: every row, no allowlist. `log` is the only
//! group with a documented gap, and it has one. If a group is not named in
//! [`GROUPS_ASSERTED_STRICTLY`] then it is not being checked at all, which is why that constant
//! is a list rather than a convention.

use minecraft_rust::javacompat::golden::Golden;
use minecraft_rust::javacompat::jvm_math;

/// Every group in `jvm_math.txt` that is asserted row-for-row with no tolerance.
///
/// Kept as a list so that adding a group to the golden without adding it here is a visible
/// omission rather than a silent one. `every_golden_group_is_accounted_for` enforces it.
const GROUPS_ASSERTED_STRICTLY: &[&str] = &["jvm_math.asin", "jvm_math.atan", "jvm_math.atan2"];

/// Groups measured against the JVM but pinned with an allowlist instead.
const GROUPS_ASSERTED_WITH_ALLOWLIST: &[&str] = &["jvm_math.log", "jvm_math.gaussianLog"];

/// Runs one group: `rust(x) == Math(x) == StrictMath(x)` for every row.
///
/// `arity` is how many arguments the row carries (1 for `asin`/`atan`/`log`, 2 for `atan2`).
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

        // (2) Math vs StrictMath, on the JVM. Counted per row so a future intrinsic shows up
        // as a count here rather than as an unexplained failure somewhere else.
        if want_math.to_bits() == want_strict.to_bits() {
            math_vs_strict += 1;
        }

        let got = f(&args);

        if want_math.is_nan() || got.is_nan() {
            nan_rows += 1;
        }

        let ok = minecraft_rust::javacompat::golden::f64_bits_match(
            group,
            want_math.to_bits(),
            got.to_bits(),
        );
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
         decide whether to schedule the HotSpot stub. (See OPEN_QUESTIONS #22.)",
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

/// Map a double's bits onto a monotone integer so adjacent doubles differ by 1.
fn ordered(b: u64) -> u64 {
    if b & 0x8000_0000_0000_0000 != 0 {
        !b
    } else {
        b | 0x8000_0000_0000_0000
    }
}

// ============================================================================
// log -- the one group that is NOT bit-exact, and cannot be
// ============================================================================
//
// # WHY THIS GROUP IS DIFFERENT
//
// `asin`, `atan` and `atan2` are exact because `Math` delegates to `StrictMath` and `jvm_math`
// ports `StrictMath`. `Math.log` does NOT delegate: it is HotSpot's `_dlog` intrinsic, and it
// differs from FdLibm on 757 of 29,201 inputs. So an FdLibm `log` is 1 ULP off `_dlog` on those,
// and there is no pure-Rust way to be exactly right except transcribing the x86-64 assembly.
//
// The alternative was the host libm, which is not portable at all: the reviewer measured
// `nextGaussian` matching 248/256 draws on Windows and 256/256 on Linux. Same seed, different
// random numbers per OS. A pure-Rust 1-ULP gap on 2.6% of inputs was the better trade, and it is
// recorded as a trade rather than left to be discovered.
//
// # WHAT IS ASSERTED, AND WHY IT IS STILL A STRONG TEST
//
// Three properties, which together are much more than "it is close":
//
// 1. **Every single mismatch is exactly 1 ULP.** Not "at most 1" -- exactly. A regression that
//    broke the subnormal scaling the way session 08's own first port did (2,257,518 ULP) fails
//    here immediately.
// 2. **Every mismatching input is on the allowlist.** The allowlist is the set of inputs where
//    FdLibm and `_dlog` are known to differ; it is DERIVED FROM THE GOLDEN, not typed in, so it
//    cannot drift from the data by transcription. A new mismatch fails.
// 3. **The mismatch count never exceeds the measured value.** A ceiling, not a target: if a
//    future change happens to agree with `_dlog` more often, that is a good day and the suite
//    still passes. A test must never require a bug.
//
// Property 1 is what makes the allowlist safe to have at all. A list of tolerated differences is
// only meaningful if the tolerated differences are pinned in *magnitude* as well as *position*.

/// The inputs where FdLibm's `log` and HotSpot's `_dlog` are known to differ.
///
/// **Generated, not typed.** `jvm_math_log_allowlist.txt` is produced from `jvm_math.txt` by
/// [`log_allowlist_matches_the_golden`], which rewrites it when
/// `JVM_MATH_WRITE_ALLOWLIST=1` is set:
///
/// ```text
/// cargo test --test parity_jvm_math log_allowlist_matches_the_golden
/// JVM_MATH_WRITE_ALLOWLIST=1 cargo test --test parity_jvm_math log_allowlist_matches_the_golden
/// ```
///
/// Without the env var it only compares, and fails on any difference. So the file cannot drift
/// from the data by transcription, and cannot rot silently when the golden is regenerated.
///
/// Kept as a comma-separated text file rather than a 687-entry Rust literal so the test file
/// stays readable and the diff of a regeneration is one line.
const ALLOWED_LOG_MISMATCHES_CSV: &str = include_str!("jvm_math_log_allowlist.txt");

fn allowed_log_mismatches() -> std::collections::HashSet<u64> {
    ALLOWED_LOG_MISMATCHES_CSV
        .lines()
        // The file carries a `#` header explaining how to regenerate itself. Skipping comments
        // here rather than stripping them at generation time keeps the provenance next to the
        // data, where someone who distrusts the list will actually read it.
        .filter(|l| !l.trim_start().starts_with('#'))
        .flat_map(|l| l.split(','))
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let t = s.strip_prefix("0x").unwrap_or(s);
            u64::from_str_radix(t, 16).unwrap_or_else(|e| panic!("bad allowlist entry {s:?}: {e}"))
        })
        .collect()
}

/// Upper bounds on the number of 1-ULP mismatches, measured. Ceilings, not targets.
const MAX_LOG_MISMATCHES: usize = 757; // of 29,201 in jvm_math.log
const MAX_GAUSSIAN_LOG_MISMATCHES: usize = 102; // of 1,536 in jvm_math.gaussianLog

fn check_log(group: &str, max_mismatches: usize) {
    let g = Golden::load("jvm_math.txt");
    let rows = g.rows(group);
    assert!(!rows.is_empty(), "group `{group}` is empty");
    let allowed = allowed_log_mismatches();

    let mut mismatches = 0usize;
    let mut not_allowed: Vec<u64> = Vec::new();
    for r in rows {
        let x = r.arg(0).as_f64();
        let want = r.exp(0).as_f64(); // Math.log == HotSpot's _dlog
        let got = jvm_math::log(x);

        // The NaN policy: where the bits are not observable, any NaN equals any NaN.
        if minecraft_rust::javacompat::golden::f64_bits_match(group, want.to_bits(), got.to_bits()) {
            continue;
        }

        mismatches += 1;
        // (1) EXACTLY one ULP. This is the load-bearing assertion.
        let d = ordered(got.to_bits()).abs_diff(ordered(want.to_bits()));
        assert_eq!(
            d, 1,
            "{group}: x={x:e} (0x{:016x}): jvm_math::log is {d} ULP from Math.log, not 1.\n\
             Expected 0x{:016x}, got 0x{:016x}. A jump from 1 ULP to many is the signature of \
             a broken port -- session 08's own first version of this function was 2,257,518 ULP \
             off on subnormals because it cached the low word before FdLibm's TWO54 scaling.",
            x.to_bits(),
            want.to_bits(),
            got.to_bits(),
        );
        // (2) On the allowlist.
        if !allowed.contains(&x.to_bits()) {
            not_allowed.push(x.to_bits());
        }
    }

    println!(
        "{group}: {}/{} rows exact; {} mismatches, all exactly 1 ULP, ceiling {max_mismatches}",
        rows.len() - mismatches,
        rows.len(),
        mismatches,
    );
    assert!(
        not_allowed.is_empty(),
        "{group}: {} mismatch(es) on inputs NOT on the allowlist: {not_allowed:?}.\n\
         Either jvm_math::log regressed, or the golden changed.",
        not_allowed.len()
    );
    // (3) A ceiling, not a requirement.
    assert!(
        mismatches <= max_mismatches,
        "{group}: {mismatches} mismatches exceeds the measured ceiling {max_mismatches}. \
         All are 1 ULP, so this is almost certainly the golden gaining rows rather than a \
         regression -- but it must be looked at, not absorbed."
    );
}

#[test]
fn log_is_pure_rust_and_pinned() {
    check_log("jvm_math.log", MAX_LOG_MISMATCHES);
    check_log("jvm_math.gaussianLog", MAX_GAUSSIAN_LOG_MISMATCHES);
}

/// The allowlist must describe exactly the inputs where the port misses -- no fewer, no more.
///
/// Without this, a stale entry would silently keep a real regression tolerated, which is the
/// mirror image of the drift bugs this project keeps hitting.
///
/// Set `JVM_MATH_WRITE_ALLOWLIST=1` to REGENERATE the file from the golden. That is the only
/// supported way to change it, which is what makes "the list matches the data" a checkable
/// claim rather than an intention.
#[test]
fn log_allowlist_matches_the_golden() {
    let g = Golden::load("jvm_math.txt");
    let mut derived: Vec<u64> = Vec::new();
    for group in GROUPS_ASSERTED_WITH_ALLOWLIST {
        for r in g.rows(group) {
            let x = r.arg(0).as_f64();
            let want = r.exp(0).as_f64().to_bits();
            if jvm_math::log(x).to_bits() != want {
                derived.push(x.to_bits());
            }
        }
    }
    derived.sort_unstable();
    derived.dedup();

    if std::env::var_os("JVM_MATH_WRITE_ALLOWLIST").is_some() {
        let header = "\
# INPUTS WHERE FdLibm's `log` AND HotSpot's `_dlog` ARE KNOWN TO DIFFER.
# GENERATED from jvm_math.txt -- do not edit by hand.
# Regenerate and verify with:
#   cargo test --test parity_jvm_math log_allowlist_matches_the_golden
#   JVM_MATH_WRITE_ALLOWLIST=1 cargo test --test parity_jvm_math log_allowlist_matches_the_golden
#
# Without the env var this test only COMPARES, and fails on any difference.
# Every one of these mismatches is EXACTLY 1 ULP; that is asserted, not assumed.
";
        let mut body = String::with_capacity(derived.len() * 19);
        for (i, b) in derived.iter().enumerate() {
            if i > 0 {
                body.push(',');
            }
            body.push_str(&format!("0x{b:016x}"));
        }
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("_porting/tests/jvm_math_log_allowlist.txt");
        std::fs::write(&path, format!("{header}{body}\n"))
            .unwrap_or_else(|e| panic!("could not write {}: {e}", path.display()));
        println!("REWROTE {} with {} entries", path.display(), derived.len());
        return;
    }

    let committed = allowed_log_mismatches();

    let missing: Vec<u64> = derived.iter().copied().filter(|b| !committed.contains(b)).collect();
    let stale: Vec<u64> = committed.iter().copied().filter(|b| !derived.contains(b)).collect();
    assert!(
        missing.is_empty(),
        "the allowlist is missing {} input(s) that DO mismatch: {missing:?}.\n\
         Regenerate it with:\n  JVM_MATH_WRITE_ALLOWLIST=1 cargo test --test parity_jvm_math log_allowlist_matches_the_golden",
        missing.len()
    );
    assert!(
        stale.is_empty(),
        "the allowlist names {} input(s) that no longer mismatch: {stale:?}.\n\
         Stale entries would silently keep a real regression tolerated. Regenerate it with:\n  \
         JVM_MATH_WRITE_ALLOWLIST=1 cargo test --test parity_jvm_math log_allowlist_matches_the_golden",
        stale.len()
    );
    println!("log allowlist: {} entries, all live", committed.len());
}

/// The corpus must actually cover the branches, or "29,201 rows pass" means very little.
///
/// A hand transcription of FdLibm fails in the *rare* branches -- the `|x| > 0.975` correction,
/// the four `atan2` special-case quadrants, the `x == 1.0` shortcut -- and a corpus of ordinary
/// values would miss all of them, which is exactly how a subtly wrong port passes its own suite.
/// This test exists because it FIRED: the first version of the random corpus had nothing in
/// `[0.5, 0.975)` and therefore did not reach `asin`'s fiddliest path at all.
#[test]
fn corpus_covers_every_fdlibm_branch() {
    let g = Golden::load("jvm_math.txt");

    // --- asin: |x| >= 1 (incl. exact +-1 and the NaN-producing |x| > 1).
    let asin = g.rows("jvm_math.asin");
    let count = |pred: &dyn Fn(f64) -> bool| asin.iter().filter(|r| pred(r.arg(0).as_f64())).count();
    assert!(count(&|x| x.abs() > 1.0) > 0, "asin corpus has no |x| > 1 (the NaN branch)");
    assert!(count(&|x| x.abs() == 1.0) >= 2, "asin corpus has no exact +-1");
    assert!(count(&|x| x.is_nan()) > 0, "asin corpus has no NaN input");
    assert!(count(&|x| x == 0.0) > 0, "asin corpus has no +-0");
    assert!(count(&|x| x.is_sign_negative() && x == 0.0) > 0, "asin corpus has no -0.0");
    assert!(count(&|x| x.abs() > 0.0 && x.abs() < 0.5) > 100, "asin corpus lacks |x| < 0.5");
    assert!(count(&|x| x.abs() >= 0.5 && x.abs() < 0.975) > 100, "asin corpus lacks 0.5..0.975");
    assert!(count(&|x| x.abs() >= 0.975 && x.abs() < 1.0) > 0, "asin corpus lacks |x| >= 0.975");
    assert!(count(&|x| x.abs() < 1.0 / 134_217_728.0 && x != 0.0) > 0, "asin corpus lacks |x| < 2^-27");

    // --- atan2: all four quadrants plus every documented special case.
    let atan2 = g.rows("jvm_math.atan2");
    assert!(atan2.len() > 100_000, "atan2 corpus is only {} rows", atan2.len());

    // An explicit `&dyn Fn` slice: each closure below has its OWN type, so an array literal of
    // them would not unify without it.
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

    // --- log: the subnormal branch is the one session 08's first port got wrong by 2.2M ULP.
    let log = g.rows("jvm_math.log");
    let subnormals = log
        .iter()
        .filter(|r| {
            let b = r.arg(0).as_f64().to_bits();
            b != 0 && (b & 0x7ff0_0000_0000_0000) == 0
        })
        .count();
    assert!(
        subnormals >= 100,
        "log corpus has only {subnormals} subnormal inputs. That is the branch \
         jvm_math::log got catastrophically wrong before, and a corpus without it would have \
         hidden that bug behind a green suite."
    );
}

/// Every group the oracle emits is accounted for: asserted strictly, or pinned with an
/// allowlist. A new group that is neither is a group nobody is checking.
#[test]
fn every_golden_group_is_accounted_for() {
    let g = Golden::load("jvm_math.txt");
    for name in g.method_names() {
        assert!(
            GROUPS_ASSERTED_STRICTLY.contains(&name)
                || GROUPS_ASSERTED_WITH_ALLOWLIST.contains(&name),
            "jvm_math.txt contains group `{name}`, which no test in this file accounts for. \
             Add it to GROUPS_ASSERTED_STRICTLY or GROUPS_ASSERTED_WITH_ALLOWLIST."
        );
    }
    println!(
        "{} groups: {} strict, {} allowlisted",
        GROUPS_ASSERTED_STRICTLY.len() + GROUPS_ASSERTED_WITH_ALLOWLIST.len(),
        GROUPS_ASSERTED_STRICTLY.len(),
        GROUPS_ASSERTED_WITH_ALLOWLIST.len(),
    );
}

/// `Vec3#rotation` reaches `asin` only through `-y / length()` and `atan2` only through
/// `(-x, z)`. This walks the actual golden `vec3.rotation` rows, so the argument values
/// `jvm_math` sees in the game are themselves covered -- rather than only the synthetic corpus.
#[test]
fn vec3_rotation_inputs_are_covered_by_the_jvm_math_corpus() {
    let b2 = Golden::load("batch2.txt");
    let rows = b2.rows("vec3.rotation");
    assert!(!rows.is_empty(), "vec3.rotation group is empty");

    let mut distinct = std::collections::HashSet::new();
    for r in rows {
        let v = minecraft_rust::net::minecraft::world::phys::Vec3::Vec3::new(
            r.arg(0).as_f64(),
            r.arg(1).as_f64(),
            r.arg(2).as_f64(),
        );
        distinct.insert(jvm_math::asin(-v.y / v.length()).to_bits());
    }
    // 512 rows drawn from a GRID, so repeats are expected; what matters is that the grid is
    // not degenerate (all one value).
    assert!(
        distinct.len() > 20,
        "vec3.rotation only produces {} distinct asin arguments -- the corpus is degenerate",
        distinct.len()
    );
}