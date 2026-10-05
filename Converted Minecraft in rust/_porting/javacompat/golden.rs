//! Port of: (no Java file -- see DESIGN_DECISIONS.md #javacompat)
//! Java class(es): (the oracle harness in _porting/java-oracle/)
//! Status: VERIFIED
//!
//! Reads the golden files produced by `_porting/java-oracle/run.ps1`.
//!
//! Format (tab-separated; `\r` tolerated because the checkout may be CRLF):
//!
//! ```text
//! #fn <name> <argtypes...> -> <rettype>
//! <argtype>:<val> [<argtype>:<val>...] -> <rettype>:<val> [<rettype>:<val>...]
//! ```
//!
//! Value encodings:
//! * `i32` / `i64` -- signed decimal
//! * `bool`       -- `true` / `false`
//! * `str`        -- raw text (`<null>` means a Java null)
//! * `f32`        -- `0x%08x` of `Float.floatToRawIntBits`
//! * `f64`        -- `0x%016x` of `Double.doubleToRawLongBits`
//! * `s`          -- comma separated ints (an `IntStream` result)
//!
//! Rows under a `#fn` header are a TRANSCRIPT: for stateful subjects the test walks
//! them in file order and performs the same calls in the same order.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// One typed value from a golden row.
#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
    Bool(bool),
    Str(String),
    Ints(Vec<i32>),
}

impl Val {
    pub fn as_i32(&self) -> i32 {
        match self {
            Val::I32(v) => *v,
            other => panic!("expected i32, got {other:?}"),
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            Val::I64(v) => *v,
            other => panic!("expected i64, got {other:?}"),
        }
    }

    /// Raw IEEE-754 bits -- the whole point: no rounding, no NaN folding.
    pub fn as_f32_bits(&self) -> u32 {
        match self {
            Val::F32(v) => *v,
            other => panic!("expected f32, got {other:?}"),
        }
    }

    pub fn as_f64_bits(&self) -> u64 {
        match self {
            Val::F64(v) => *v,
            other => panic!("expected f64, got {other:?}"),
        }
    }

    pub fn as_f32(&self) -> f32 {
        f32::from_bits(self.as_f32_bits())
    }

    pub fn as_f64(&self) -> f64 {
        f64::from_bits(self.as_f64_bits())
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Val::Bool(v) => *v,
            other => panic!("expected bool, got {other:?}"),
        }
    }

    /// `None` stands in for a Java `null`.
    ///
    /// The oracle escapes strings so a single-space-separated format stays
    /// unambiguous: `\s` = space, `\0` = empty, `\0null` = null, `\n`, `\t`, `\\`.
    /// An empty string is a legal Java value, so it must not be confusable with
    /// "absent".
    pub fn as_opt_str(&self) -> Option<&str> {
        match self {
            Val::Str(s) if s == "\\0null" => None,
            Val::Str(s) => Some(s),
            other => panic!("expected str, got {other:?}"),
        }
    }

    pub fn as_ints(&self) -> &[i32] {
        match self {
            Val::Ints(v) => v.as_slice(),
            other => panic!("expected int list, got {other:?}"),
        }
    }
}

/// One golden row: arguments, then the expected return value(s).
#[derive(Debug, Clone)]
pub struct Row {
    /// Index of the `#fn` group this row belongs to.
    pub line: usize,
    pub args: Vec<Val>,
    pub expect: Vec<Val>,
}

impl Row {
    pub fn arg(&self, i: usize) -> &Val {
        self.args.get(i).unwrap_or_else(|| panic!("row {} has no arg {i}", self.line))
    }

    pub fn exp(&self, i: usize) -> &Val {
        self.expect.get(i).unwrap_or_else(|| panic!("row {} has no expected value {i}", self.line))
    }

    pub fn exp0(&self) -> &Val {
        self.exp(0)
    }
}

/// A whole golden file, grouped by `#fn` name (order preserved within a group).
#[derive(Debug, Clone, Default)]
pub struct Golden {
    pub path: PathBuf,
    groups: HashMap<String, Vec<Row>>,
    order: Vec<String>,
}

impl Golden {
    /// Load a golden file. `name` is the file name inside `_porting/test-data/`.
    pub fn load(name: &str) -> Self {
        let dir = test_data_dir();
        let path = dir.join(name);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read golden file {}: {e}", path.display()));
        let mut g = Golden { path, groups: HashMap::new(), order: Vec::new() };

        let mut current: Option<String> = None;
        for (i, raw) in text.lines().enumerate() {
            let line = raw.trim_end_matches('\r');
            if line.trim().is_empty() {
                continue;
            }
            if let Some(rest) = line.strip_prefix('#') {
                if let Some(header) = rest.trim_start().strip_prefix("fn ") {
                    let name = header.split_whitespace().next().unwrap_or("").to_string();
                    if !g.groups.contains_key(&name) {
                        g.order.push(name.clone());
                    }
                    g.groups.entry(name.clone()).or_default();
                    current = Some(name);
                }
                continue;
            }
            let group = current.clone().unwrap_or_else(|| panic!("data row before any #fn header at line {}", i + 1));
            let (lhs, rhs) = line.split_once(" -> ").unwrap_or_else(|| panic!("malformed row at line {}: {line}", i + 1));
            let row = Row {
                line: i + 1,
                args: split_vals(lhs),
                expect: split_vals(rhs),
            };
            g.groups.get_mut(&group).expect("group").push(row);
        }
        g
    }

    /// Rows of one method group, in file order.
    pub fn rows(&self, method: &str) -> &[Row] {
        self.groups
            .get(method)
            .unwrap_or_else(|| {
                let mut names: Vec<&str> = self.groups.keys().map(|s| s.as_str()).collect();
                names.sort_unstable();
                panic!("golden file {} has no group `{method}`; it has: {names:?}", self.path.display())
            })
            .as_slice()
    }

    /// All method group names, sorted.
    pub fn method_names(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.groups.keys().map(|s| s.as_str()).collect();
        v.sort_unstable();
        v
    }

    /// Index of the only expected value of the single row in `method`.
    pub fn single(&self, method: &str) -> Val {
        let rows = self.rows(method);
        assert_eq!(rows.len(), 1, "`{method}` should have exactly one row, has {}", rows.len());
        rows[0].exp0().clone()
    }
}

/// Split one side of a row into values.
///
/// Values are separated by a single space, but the `s` type (an `IntStream`
/// result) encodes a whole LIST as `s:1,2,3` -- commas, never spaces. So a plain
/// `split(' ')` is correct; the only thing to be careful about is that a `str`
/// value must not contain spaces, which the oracle guarantees.
fn split_vals(s: &str) -> Vec<Val> {
    let s = s.trim();
    if s.is_empty() {
        return Vec::new();
    }
    s.split(' ').filter(|p| !p.is_empty()).map(parse_val).collect()
}

/// Reverse of `Out.str` in the oracle.
fn unescape_str(s: &str) -> String {
    if s == "\\0" {
        return String::new();
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('s') => out.push(' '),
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

fn parse_val(tok: &str) -> Val {
    let (kind, rest) = tok.split_once(':').unwrap_or_else(|| panic!("value `{tok}` has no `type:` prefix"));
    match kind {
        "i32" => Val::I32(rest.parse().unwrap_or_else(|e| panic!("bad i32 `{rest}`: {e}"))),
        "i64" => Val::I64(rest.parse().unwrap_or_else(|e| panic!("bad i64 `{rest}`: {e}"))),
        "bool" => Val::Bool(match rest {
            "true" => true,
            "false" => false,
            other => panic!("bad bool `{other}`"),
        }),
        "f32" => Val::F32(u32::from_str_radix(rest.trim_start_matches("0x"), 16).expect("bad f32 hex")),
        "f64" => Val::F64(u64::from_str_radix(rest.trim_start_matches("0x"), 16).expect("bad f64 hex")),
        "str" => Val::Str(unescape_str(rest)),
        "s" => Val::Ints(rest.split(',').filter(|p| !p.is_empty()).map(|p| p.parse().expect("bad int list item")).collect()),
        other => panic!("unknown golden value type `{other}`"),
    }
}

/// Locate `_porting/test-data/` relative to this file, so the tests work no matter
/// what the current working directory is.
pub fn test_data_dir() -> PathBuf {
    // .../Converted Minecraft in rust/_porting/javacompat/golden.rs
    //   0 = javacompat, 1 = _porting
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join("_porting").join("test-data")
}

// ---------------------------------------------------------------------------
// Assertion helpers -- every failure prints the golden row, the expected raw bits
// and the actual raw bits, because "1.0 vs 1.0000001" is useless in a bug report.
// ---------------------------------------------------------------------------

pub fn assert_i32(method: &str, row: &Row, actual: i32) {
    let expected = row.exp0().as_i32();
    assert_eq!(actual, expected, "{method} (golden line {}) args={:?}: expected {expected}, got {actual}", row.line, row.args);
}

pub fn assert_i64(method: &str, row: &Row, actual: i64) {
    let expected = row.exp0().as_i64();
    assert_eq!(actual, expected, "{method} (golden line {}) args={:?}: expected {expected}, got {actual}", row.line, row.args);
}

pub fn assert_bool(method: &str, row: &Row, actual: bool) {
    let expected = row.exp0().as_bool();
    assert_eq!(actual, expected, "{method} (golden line {}) args={:?}: expected {expected}, got {actual}", row.line, row.args);
}

/// Bit-exact float comparison. Deliberately NOT `==`: that would treat NaN != NaN
/// and +0.0 == -0.0, both of which are real divergences.
pub fn assert_f32_bits(method: &str, row: &Row, actual: f32) {
    let expected = row.exp0().as_f32_bits();
    let actual = actual.to_bits();
    assert_eq!(
        actual, expected,
        "{method} (golden line {}) args={:?}: expected f32 bits 0x{expected:08x} ({}), got 0x{actual:08x} ({})",
        row.line, row.args, f32::from_bits(expected), actual
    );
}

pub fn assert_f64_bits(method: &str, row: &Row, actual: f64) {
    let expected = row.exp0().as_f64_bits();
    let bits = actual.to_bits();
    assert!(
        f64_bits_match(method, expected, bits),
        "{method} (golden line {}) args={:?}: expected f64 bits 0x{expected:016x} ({}), got 0x{bits:016x} ({})",
        row.line, row.args, f64::from_bits(expected), actual
    );
}

/// For rows whose return value is several numbers (e.g. a Vec3 or a UUID).
pub fn assert_multi_f64(method: &str, row: &Row, actual: &[f64]) {
    let expected: Vec<f64> = (0..row.expect.len()).map(|i| row.exp(i).as_f64()).collect();
    assert_eq!(actual.len(), expected.len(), "{method} (golden line {}): arity mismatch", row.line);
    for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
        assert!(
            f64_bits_match(method, e.to_bits(), a.to_bits()),
            "{method} (golden line {}) component {i}: expected {} (0x{:016x}), got {} (0x{:016x})",
            row.line, e, e.to_bits(), a, a.to_bits()
        );
    }
}

pub fn assert_multi_f32(method: &str, row: &Row, actual: &[f32]) {
    let expected: Vec<f32> = (0..row.expect.len()).map(|i| row.exp(i).as_f32()).collect();
    assert_eq!(actual.len(), expected.len(), "{method} (golden line {}): arity mismatch", row.line);
    for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
        assert!(
            f32_bits_match(method, e.to_bits(), a.to_bits()),
            "{method} (golden line {}) component {i}: expected {} (0x{:08x}), got {} (0x{:08x})",
            row.line, e, e.to_bits(), a, a.to_bits()
        );
    }
}

// ============================================================================
// THE NaN POLICY AS IT IS ACTUALLY ENFORCED
// ============================================================================
//
// # WHY THE DEFAULT IS "NaN EQUALS NaN"
//
// A reviewer built this on Linux and got four failures in `parity_random` and a fifth in
// `parity_mth` under `--release`. The causes are worth stating precisely, because they are
// two DIFFERENT problems that look alike:
//
// 1. **The host's libm differs per OS.** glibc's `ln` happens to agree with HotSpot's
//    `_dlog` on the eight inputs where Windows' does not. So `f64::ln` gives different
//    `nextGaussian` values on Windows and Linux. This is why `parity_random` asserted
//    "diverges on exactly rows [12, 13, 140, ...]" and failed on Linux: the divergence is
//    a property of the HOST, not of the port. Fixed long-term by `jvm_math`; in the
//    meantime the required-divergence assertions became ALLOWLISTS (see `parity_random`).
//
// 2. **NaN bits change with the optimisation level.** The same `lerp2` golden row gives
//    `+NaN` in debug and `-NaN` in release. Rust does not guarantee the sign or payload of
//    a NaN produced by arithmetic, and the optimiser is entitled to constant-fold,
//    vectorise or reassociate. So a test that demands exact NaN bits is demanding
//    something the language does not promise.
//
// Point 2 is what this policy fixes. **Where game code cannot observe a NaN's sign or
// payload, the comparison treats any NaN as equal to any other NaN.** Exact bits are still
// required for everything else -- `+0.0` vs `-0.0` is a REAL divergence and is still
// checked, because vanilla's signed-zero behaviour is load-bearing everywhere.
//
// # WHY THE OPT-IN LIST IS CURRENTLY EMPTY
//
// The policy says exact NaN bits matter only where game code can OBSERVE them:
// `Float.floatToRawIntBits` / `Double.doubleToRawLongBits` results, values fed into hashing
// or serialisation, or values compared bit-for-bit against a stored constant.
//
// **No ported golden group observes raw NaN bits.** Java's `hashCode` implementations
// (`Vec2`, `Vec3`, `Rotations`) all go through `floatToIntBits`/`doubleToLongBits`, which
// CANONICALISE every NaN to one value -- so they cannot distinguish payloads, and the
// groups pass without needing exact bits. Nothing else in the ported surface serialises a
// raw float yet; NBT is not ported.
//
// So the list below is empty, and that is a CLAIM worth being able to check rather than an
// accident. `nan_bits_observable_list_has_no_stale_entries` fails if it ever names a group
// the oracle does not emit, and `no_group_claims_raw_nan_observability` (in
// `parity_batch2.rs`) fails if a group is added without a reason.

/// Golden groups where a NaN's exact sign and payload ARE observable by game code.
///
/// **Currently empty**, for the reason documented above: every ported `hashCode`
/// canonicalises NaN, and no ported group serialises a raw float.
///
/// To opt a group in, add its name here AND write down which of the three observable sites
/// applies. Do not add a group just because it "has NaNs" -- `vec3.subtractScalar` has
/// thousands of NaN rows and observes none of them.
pub const NAN_BITS_OBSERVABLE: &[&str] = &[];

/// Does `actual_bits` satisfy `expected_bits` for `method`?
///
/// Equal bits always match. Differing bits match when both sides are NaN **and** the group
/// is not in [`NAN_BITS_OBSERVABLE`]. `+0.0` vs `-0.0` never matches.
#[inline]
pub fn f64_bits_match(method: &str, expected_bits: u64, actual_bits: u64) -> bool {
    if expected_bits == actual_bits {
        return true;
    }
    both_nan_f64(expected_bits, actual_bits) && !NAN_BITS_OBSERVABLE.contains(&method)
}

/// `f32` counterpart of [`f64_bits_match`].
#[inline]
pub fn f32_bits_match(method: &str, expected_bits: u32, actual_bits: u32) -> bool {
    if expected_bits == actual_bits {
        return true;
    }
    both_nan_f32(expected_bits, actual_bits) && !NAN_BITS_OBSERVABLE.contains(&method)
}

#[inline]
fn both_nan_f64(a: u64, b: u64) -> bool {
    const EXP: u64 = 0x7ff0_0000_0000_0000;
    (a & EXP) == EXP && (a & 0x000f_ffff_ffff_ffff) != 0 && (b & EXP) == EXP
        && (b & 0x000f_ffff_ffff_ffff) != 0
}

#[inline]
fn both_nan_f32(a: u32, b: u32) -> bool {
    const EXP: u32 = 0x7f80_0000;
    (a & EXP) == EXP && (a & 0x007f_ffff) != 0 && (b & EXP) == EXP && (b & 0x007f_ffff) != 0
}

/// Guard: every entry in [`NAN_BITS_OBSERVABLE`] must be a group the oracle really emits,
/// and must say why. A stale entry would silently keep NaN bits strict for a group that no
/// longer exists, which is the mirror image of the staleness bugs this project keeps hitting.
#[test]
fn nan_bits_observable_list_has_no_stale_entries() {
    let g = Golden::load("batch2.txt");
    let names = g.method_names();
    for m in NAN_BITS_OBSERVABLE {
        assert!(
            names.iter().any(|n| n == m),
            "NAN_BITS_OBSERVABLE names `{m}`, which batch2.txt does not emit"
        );
    }
}


/// Assert a space-separated list of `i32` results (e.g. a `Vec3i`'s x/y/z).
///
/// Distinct from [`assert_ints`], which expects the `s:1,2,3` "int stream" encoding.
/// Two encodings exist because vanilla has two shapes: `IntStream` results (comma lists)
/// and ordinary multi-value returns (space separated). Mixing them up produces a
/// confusing "expected int list, got I32(0)".
pub fn assert_multi_i32(method: &str, row: &Row, actual: &[i32]) {
    let expected = &row.expect;
    assert_eq!(
        expected.len(),
        actual.len(),
        "{method} (golden line {}): expected {} values, got {}",
        row.line,
        expected.len(),
        actual.len()
    );
    for (i, (want, got)) in expected.iter().zip(actual.iter()).enumerate() {
        let want = want.as_i32();
        assert_eq!(
            *got, want,
            "{method} (golden line {}) value {i}",
            row.line
        );
    }
}
pub fn assert_ints(method: &str, row: &Row, actual: &[i32]) {
    assert_eq!(actual, row.exp0().as_ints(), "{method} (golden line {}) args={:?}", row.line, row.args);
}

pub fn assert_str(method: &str, row: &Row, actual: &str) {
    let expected = match row.exp0() {
        Val::Str(s) => s.clone(),
        other => panic!("expected str, got {other:?}"),
    };
    assert_eq!(actual, expected, "{method} (golden line {}) args={:?}", row.line, row.args);
}
/// Assert a specific EXPECTED value of a multi-value row.
///
/// [`assert_f64_bits`] and friends always compare against `exp(0)`, which is fine for
/// single-value rows but silently re-checks the first value when a row carries several.
/// That failure mode is nasty: you think you are asserting index 4, the test actually
/// asserts index 0, and it either passes vacuously or fails somewhere unrelated.
///
/// The `label` is only used to make the assertion message name which step drifted.
pub fn assert_f64_bits_at(method: &str, row: &Row, index: usize, label: &str, actual: f64) {
    let expected = row.exp(index).as_f64_bits();
    let bits = crate::javacompat::nan_policy::double_to_raw_long_bits(actual) as u64;
    assert!(
        f64_bits_match(method, expected, bits),
        "{method} [{label}] (golden line {}, expected index {index}): expected bits 0x{expected:016x}, got 0x{bits:016x}",
        row.line
    );
}

/// `f32` counterpart of [`assert_f64_bits_at`].
pub fn assert_f32_bits_at(method: &str, row: &Row, index: usize, label: &str, actual: f32) {
    let expected = row.exp(index).as_f32_bits();
    let bits = crate::javacompat::nan_policy::float_to_raw_int_bits(actual) as u32;
    assert!(
        f32_bits_match(method, expected, bits),
        "{method} [{label}] (golden line {}, expected index {index}): expected bits 0x{expected:08x}, got 0x{bits:08x}",
        row.line
    );
}
/// Assert a specific EXPECTED value of a multi-value row against an `i32`.
pub fn assert_i32_at(method: &str, row: &Row, index: usize, label: &str, actual: i32) {
    assert_eq!(actual, row.exp(index).as_i32(), "{method} [{label}] (golden line {}, expected index {index})", row.line);
}
/// Does `actual` equal the EXPECTED value at `index` of `row`, bit for bit?
///
/// This is the non-panicking counterpart of [`assert_f64_bits_at`]. It exists for
/// rows where a divergence is *known, measured and bounded* rather than absent: the
/// caller needs to count the exceptions, assert that the set of exceptions is a SUBSET of
/// the one already documented, and fail loudly if it grows.
///
/// A blanket `assert_eq!` cannot express that, and deleting the assertion to make the
/// suite green would hide the very thing being tracked.
///
/// "Subset", not "equal": a test must never REQUIRE a divergence. If the host libm on
/// this OS happens to agree with HotSpot, that is a good day and the suite should pass.
/// Asserting equality of the mismatch set would make correctness depend on which libc the
/// machine has -- which is exactly the bug this function's callers had.
///
/// Note this applies the same NaN policy as the asserting helpers.
pub fn f64_matches(row: &Row, index: usize, actual: f64) -> bool {
    let bits = crate::javacompat::nan_policy::double_to_raw_long_bits(actual) as u64;
    f64_bits_match("", row.exp(index).as_f64_bits(), bits)
}

/// [`f64_matches`] with the group name supplied, so [`NAN_BITS_OBSERVABLE`] applies.
///
/// Prefer this over [`f64_matches`] in new code: the empty method name above means an
/// opt-in group can never match NaN.
pub fn f64_matches_in(method: &str, row: &Row, index: usize, actual: f64) -> bool {
    let bits = crate::javacompat::nan_policy::double_to_raw_long_bits(actual) as u64;
    f64_bits_match(method, row.exp(index).as_f64_bits(), bits)
}