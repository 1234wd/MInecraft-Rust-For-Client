//! Port of: (no Java file -- see DESIGN_DECISIONS.md #javacompat)
//! Java class(es): java.lang.Math, java.lang.String, java.lang.Float, java.lang.Double
//! Status: VERIFIED
//!
//! These have no `.java` counterpart in `minecraft-decompiled/`, so they live in
//! `_porting/javacompat/` instead of the mirror. They are pulled into the crate by
//! `lib.rs` via `#[path]`.
//!
//! Everything here must reproduce Java semantics EXACTLY -- especially the places
//! where Rust's natural operator differs from Java's:
//!
//! | behaviour                    | Java                      | Rust (what to avoid)      |
//! |------------------------------|---------------------------|---------------------------|
//! | `int` division rounding      | truncates toward zero      | same                      |
//! | `%` with a negative operand  | sign of the dividend       | same                      |
//! | `Math.floorDiv`/`floorMod`   | floor semantics            | `/` and `%` are WRONG      |
//! | `Math.abs(Integer.MIN_VALUE)`| stays `MIN_VALUE`          | `i32::abs` PANICS          |
//! | `Math.abs(NaN)`              | returns the input unchanged | `f32::abs` clears the sign |
//! | `(byte)someFloat`            | saturate to i32, then wrap | `as i8` saturates to i8    |
//! | `double`/`float` -> `int`    | saturate, NaN -> 0        | `as` already matches       |
//! | `String.hashCode()`          | over UTF-16 code units    | over UTF-8 bytes is WRONG |

use std::cmp::Ordering;

// ---------------------------------------------------------------------------
// Arithmetic -- the JVM's invalid-operation NaN is NEGATIVE, Rust's is positive
// ---------------------------------------------------------------------------
//
// x86 raises the *real indefinite* QNaN (`0xfff8...` / `0xffc0...`) for a genuine
// invalid operation, while Rust/LLVM hands back the *default* QNaN
// (`0x7ff8...` / `0x7fc0...`). Measured on HotSpot 21 at runtime:
//
// | expression        | HotSpot 21          | Rust                    |
// |-------------------|---------------------|-------------------------|
// | `Inf + -Inf`      | `0xfff8000000000000`| `0x7ff8000000000000`   |
// | `Inf - Inf`       | `0xfff8000000000000`| `0x7ff8000000000000`   |
// | `Inf * 0`         | `0xfff8000000000000`| `0x7ff8000000000000`   |
// | `sqrt(-1)`        | `0xfff8000000000000`| `0x7ff8000000000000`   |
// | `NaN + 1`         | `0x7ff8000000000000`| payload of the operand  |
//
// Note the asymmetry that makes this unfixable with a blanket rule: an *invalid
// operation* is negative, but a *NaN operand* propagates its own sign (`-NaN + 1`
// is negative). So the helpers below test the INPUTS, not the result: if an
// operand was already NaN we let the platform propagate it, and if it was not but
// the answer is, we substitute the real indefinite.

// The helper below encodes the x86/Intel rule for NaN selection, which HotSpot
// inherits directly and which Rust/LLVM does NOT reproduce:
//
//   "If only one operand is NaN, the SECOND operand is returned. If both are NaN,
//    the SECOND operand is returned."
//
// In Rust terms (`a` is the destination, `b` the source):
//
//   1. if `b` is NaN  -> result is `b`, bit for bit
//   2. else if `a` is NaN -> result is `a`, bit for bit
//   3. else if the result is NaN -> a genuine invalid operation, so the JVM's
//      real-indefinite (NEGATIVE) NaN
//
// Every case below was measured on HotSpot 21 at runtime; see
// `arithmetic_nan_selection_matches_the_jvm`, which pins all of them.
//
// The sign asymmetry that makes this necessary (and which a naive
// "if result is NaN and no operand was, use indefinite" rule gets WRONG):
//
//   -NaN * +NaN = 0x7ff8000000000000   <- b wins, b is positive
//   +NaN * -NaN = 0xfff8000000000000   <- b wins, b is negative
//   +NaN * +NaN = 0x7ff8000000000000
//   -NaN * -NaN = 0xfff8000000000000
//   +NaN + 1.0  = 0x7ff8000000000000   <- b is not NaN, so a wins
//   -NaN + 1.0  = 0xfff8000000000000   <- a wins, keeps the negative sign
//   Inf + -Inf  = 0xfff8000000000000   <- invalid operation -> indefinite

#[inline]
fn nan_result_f32(a: f32, b: f32, r: f32) -> f32 {
    if !r.is_nan() {
        return r;
    }
    if b.is_nan() {
        return b;
    }
    if a.is_nan() {
        return a;
    }
    f32::from_bits(REAL_INDEFINITE_F32)
}

#[inline]
fn nan_result_f64(a: f64, b: f64, r: f64) -> f64 {
    if !r.is_nan() {
        return r;
    }
    if b.is_nan() {
        return b;
    }
    if a.is_nan() {
        return a;
    }
    f64::from_bits(REAL_INDEFINITE_F64)
}

/// Port of Java's `+` on `f32`.
#[inline]
pub fn add_f32(a: f32, b: f32) -> f32 {
    nan_result_f32(a, b, a + b)
}

/// Port of Java's `-` on `f32`.
#[inline]
pub fn sub_f32(a: f32, b: f32) -> f32 {
    nan_result_f32(a, b, a - b)
}

/// Port of Java's `*` on `f32`.
#[inline]
pub fn mul_f32(a: f32, b: f32) -> f32 {
    nan_result_f32(a, b, a * b)
}

/// Port of Java's `/` on `f32`. See [`div_f64`] for the `0.0 / 0.0` special case.
#[inline]
pub fn div_f32(a: f32, b: f32) -> f32 {
    if a == 0.0 && b == 0.0 {
        return f32::from_bits(POSITIVE_NAN_F32);
    }
    nan_result_f32(a, b, a / b)
}

/// Port of Java's `+` on `f64`.
#[inline]
pub fn add_f64(a: f64, b: f64) -> f64 {
    nan_result_f64(a, b, a + b)
}

/// Port of Java's `-` on `f64`.
#[inline]
pub fn sub_f64(a: f64, b: f64) -> f64 {
    nan_result_f64(a, b, a - b)
}

/// Port of Java's `*` on `f64`.
#[inline]
pub fn mul_f64(a: f64, b: f64) -> f64 {
    nan_result_f64(a, b, a * b)
}

/// Port of Java's `/` on `f64`.
///
/// SPECIAL CASE: `0.0 / 0.0` is NOT a "real indefinite" on x86 -- it divides by
/// zero with a zero dividend, which raises `DIVIDE_ERROR` and yields the DEFAULT
/// (POSITIVE) QNaN, not the negative one that `Inf/Inf`, `Inf-Inf` and `Inf*0`
/// produce. Measured on HotSpot 21:
///
/// ```text
/// 0.0 / 0.0  -> 0x7ff8000000000000   (default, positive)
/// 1.0 / 0.0  -> Infinity            (no NaN at all)
/// Inf / Inf  -> 0xfff8000000000000   (real indefinite, negative)
/// ```
#[inline]
pub fn div_f64(a: f64, b: f64) -> f64 {
    if a == 0.0 && b == 0.0 {
        return f64::from_bits(POSITIVE_NAN_F64);
    }
    nan_result_f64(a, b, a / b)
}

/// Port of Java's `Math.sqrt(double)`.
///
/// `sqrt` is the only `Math` function here that can *create* an invalid result:
/// `sqrt(-1)` and `sqrt(-Inf)` are both `NaN` on x86, and the JVM raises the
/// real indefinite for them, while Rust returns the default positive QNaN.
/// There is no NaN operand case here -- `sqrt(NaN)` propagates the operand.
#[inline]
pub fn sqrt_f64(x: f64) -> f64 {
    let r = x.sqrt();
    if r.is_nan() && !x.is_nan() {
        f64::from_bits(REAL_INDEFINITE_F64)
    } else {
        r
    }
}

/// Port of Java's `Math.sqrt`-then-narrow, as used by `Mth#sqrt(float)`.
///
/// Java: `(float) Math.sqrt(x)`. The double sqrt is computed first and only then
/// narrowed, which is NOT the same as `x.sqrt()` in f32 -- so the intermediate has
/// to stay f64.
#[inline]
pub fn sqrt_narrow_f32(x: f32) -> f32 {
    sqrt_f64(x as f64) as f32
}

// ---------------------------------------------------------------------------
// Remainder (`%`) -- NaN canonicalisation
// ---------------------------------------------------------------------------

/// The JVM's "real indefinite" QNaN -- the value x86/SSE raises for an invalid
/// floating-point operation such as `Inf % finite` or `finite / 0`.
///
/// Verified by a runtime probe against HotSpot 21 (see OPEN_QUESTIONS.md). NOTE:
/// probing this with `javac` constants is a TRAP -- `Float.POSITIVE_INFINITY % 360.0F`
/// is a compile-time constant, so javac folds it with its own arithmetic and reports
/// a POSITIVE NaN. The probe must defeat constant folding (read through a
/// `volatile`, or pass the value as an argument).
const REAL_INDEFINITE_F32: u32 = 0xFFC0_0000;
const REAL_INDEFINITE_F64: u64 = 0xFFF8_0000_0000_0000;

/// The JVM's canonical POSITIVE default QNaN, which it returns when a NaN operand
/// is fed to an arithmetic operation or to `Math.min`/`Math.max`.
const POSITIVE_NAN_F32: u32 = 0x7FC0_0000;
const POSITIVE_NAN_F64: u64 = 0x7FF8_0000_0000_0000;

/// Port of Java's `%` on `float` (JLS 15.21.4 / `Float.rem`).
///
/// For finite operands this is plain `fmod`, which is correctly rounded in both
/// languages, so the operator itself is fine. The divergence is the invalid case:
///
/// | expression            | HotSpot 21          | Rust `f32 % f32` |
/// |-----------------------|---------------------|-------------------|
/// | `+Inf % 360.0F`       | `0xffc00000`        | `0x7fc00000`     |
/// | `-Inf % 360.0F`       | `0xffc00000`        | `0x7fc00000`     |
/// | `1.0F % 0.0F`         | `0xffc00000`        | `0x7fc00000`     |
/// | `NaN % 360.0F`        | `0x7fc00000`        | `0x7fc00000`     |
///
/// Java raises the hardware's real-indefinite QNaN, which is NEGATIVE. Rust (via
/// LLVM/libm) hands back the positive default NaN. When a NaN is fed IN, Java
/// propagates it and Rust agrees, so that case is left alone.
///
/// This matters: `Mth#wrapDegrees` reduces with `%`, so every infinite or degenerate
/// angle argument would otherwise produce a different NaN payload than vanilla --
/// which is visible in entity yaw, block rotation and camera angles.
#[inline]
pub fn rem_f32(a: f32, b: f32) -> f32 {
    if a.is_nan() || b.is_nan() {
        // HotSpot canonicalises a NaN operand to the POSITIVE default QNaN.
        return f32::from_bits(POSITIVE_NAN_F32);
    }
    let r = a % b;
    if r.is_nan() {
        // Invalid operation (inf % finite, or finite % 0): negative default.
        f32::from_bits(REAL_INDEFINITE_F32)
    } else {
        r
    }
}

/// Port of Java's `%` on `double`. Same handling as [`rem_f32`].
#[inline]
pub fn rem_f64(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        return f64::from_bits(POSITIVE_NAN_F64);
    }
    let r = a % b;
    if r.is_nan() {
        f64::from_bits(REAL_INDEFINITE_F64)
    } else {
        r
    }
}

// ---------------------------------------------------------------------------
// Integer helpers
// ---------------------------------------------------------------------------

/// Port of `Math.floorDiv(int, int)`.
/// Java: `Math.floorDiv(a, b) == Math.floor((double) a / (double) b)`, computed on
/// integers. For `a == i32::MIN` and `b == -1` the mathematical quotient is
/// `2147483648`, which does not fit in an `int`; Java wraps it to `i32::MIN`.
#[inline]
pub fn floor_div_i32(a: i32, b: i32) -> i32 {
    // Rust's `/` truncates toward zero, so correct it only when the signs differ
    // and the division was inexact. `wrapping_*` matters: Java's
    // floorDiv(Integer.MIN_VALUE, -1) is defined as Integer.MIN_VALUE, which plain
    // `/` and `%` would panic on instead.
    let q = a.wrapping_div(b);
    if (a.wrapping_rem(b) != 0) && ((a < 0) != (b < 0)) {
        q - 1
    } else {
        q
    }
}

/// Port of `Math.floorDiv(long, long)`.
#[inline]
pub fn floor_div_i64(a: i64, b: i64) -> i64 {
    let q = a.wrapping_div(b);
    if (a.wrapping_rem(b) != 0) && ((a < 0) != (b < 0)) {
        q - 1
    } else {
        q
    }
}

/// Port of `Math.floorMod(int, int)`.
#[inline]
pub fn floor_mod_i32(a: i32, b: i32) -> i32 {
    // Java: r = a % b; if ((r ^ b) < 0 && r != 0) r += b;
    let r = a.wrapping_rem(b);
    if (r ^ b) < 0 && r != 0 {
        r.wrapping_add(b)
    } else {
        r
    }
}

/// Port of `Math.floorMod(long, long)`.
#[inline]
pub fn floor_mod_i64(a: i64, b: i64) -> i64 {
    let r = a.wrapping_rem(b);
    if (r ^ b) < 0 && r != 0 {
        r.wrapping_add(b)
    } else {
        r
    }
}

/// Port of `Math.abs(int)`. Note `Math.abs(Integer.MIN_VALUE) == Integer.MIN_VALUE`
/// (overflow is preserved), so this must NOT use `i32::abs`, which panics.
#[inline]
pub fn abs_i32(a: i32) -> i32 {
    if a < 0 {
        a.wrapping_neg()
    } else {
        a
    }
}

/// Port of `Math.abs(long)`.
#[inline]
pub fn abs_i64(a: i64) -> i64 {
    if a < 0 {
        a.wrapping_neg()
    } else {
        a
    }
}

/// Port of `Math.abs(float)`.
///
/// The JLS expression is `(a <= 0.0F) ? 0.0F - a : a`. Taken literally that returns
/// a negative NaN unchanged, because `NaN <= 0.0` is false. HotSpot, however,
/// INTRINSIFIES `Math.abs` to `fabsf`, which clears the sign bit unconditionally --
/// so at runtime `Math.abs(-NaN)` is `+NaN`.
///
/// Verified against HotSpot 21: `Math.abs(-Float.NaN)` == `0x7fc00000`.
///
/// `f32::abs` lowers to the same `fabsf`, so the intrinsic behaviour is exactly
/// what we want here. Note this is the OPPOSITE of [`rem_f32`], where the JVM's
/// invalid-operation NaN is negative and Rust's is positive.
#[inline]
pub fn abs_f32(a: f32) -> f32 {
    a.abs()
}

/// Port of `Math.abs(double)`. Same reasoning as [`abs_f32`].
#[inline]
pub fn abs_f64(a: f64) -> f64 {
    a.abs()
}

/// Port of `Math.min(float, float)`.
///
/// Java's source is
/// `if (a != a) return a; if (b != b) return b; return (a <= b) ? a : b;`
///
/// TWO things to copy exactly:
///
/// 1. The NaN operand is returned as-is -- payload AND sign. Measured:
///    `Math.min(floatFromBits(0xffc00000), 45.0F)` == `0xffc00000`.
///    Rust's `f32::min` instead returns the *non*-NaN operand, so it is unusable.
///
/// 3. SIGNED ZERO. HotSpot intrinsifies `Math.min`/`Math.max` to x86 `minss`/
///    `maxss`, whose tie-break is "return the SECOND operand when the two compare
///    equal". Measured:
///    `Math.min(+0.0F, -0.0F)` == `0x80000000` (`-0.0`), and
///    `Math.max(+0.0F, -0.0F)` == `0x00000000` (`+0.0`).
///    Writing this as a plain `if a <= b` returns `+0.0` for min, which is wrong.
#[inline]
pub fn min_f32(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        return a;
    }
    if b.is_nan() {
        return b;
    }
    if a < b {
        a
    } else {
        b
    }
}

/// Port of `Math.min(double, double)`. See [`min_f32`].
/// Port of `Math.min(double, double)`. See [`min_f32`], including the signed-zero
/// tie-break (`Math.min(+0.0, -0.0)` is `-0.0`).
#[inline]
pub fn min_f64(a: f64, b: f64) -> f64 {
    if a.is_nan() {
        return a;
    }
    if b.is_nan() {
        return b;
    }
    if a < b {
        a
    } else {
        b
    }
}

/// Port of `Math.max(float, float)`. See [`min_f32`] for the NaN rule.
///
/// Note `max(+0.0, -0.0)` is `+0.0`, because `+0.0 >= -0.0` is true.
#[inline]
pub fn max_f32(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        return a;
    }
    if b.is_nan() {
        return b;
    }
    if a >= b {
        a
    } else {
        b
    }
}

/// Port of `Math.max(double, double)`. See [`min_f32`] for the NaN rule.
#[inline]
pub fn max_f64(a: f64, b: f64) -> f64 {
    if a.is_nan() {
        return a;
    }
    if b.is_nan() {
        return b;
    }
    if a >= b {
        a
    } else {
        b
    }
}

/// Port of `Long.rotateLeft(long, int)`. The distance is masked to 6 bits by the
/// JVM, matching Rust's `rotate_left` on `i64`.
#[inline]
pub fn rotate_left_i64(v: i64, distance: i32) -> i64 {
    v.rotate_left(distance as u32)
}

// ---------------------------------------------------------------------------
// Narrowing conversions (JLS 5.1.3)
// ---------------------------------------------------------------------------

/// Port of the Java narrowing conversion `(byte) someFloat`.
///
/// Java is TWO steps: first `float -> int` (saturating, NaN -> 0), then
/// `int -> byte` (two's-complement wrap). Rust's `f32 as i8` saturates straight to
/// the `i8` range, which is a DIFFERENT function -- e.g. `(byte) 1e10f` is `0` in
/// Java, not `127`.
#[inline]
pub fn narrow_f32_to_i8(v: f32) -> i8 {
    (v as i32) as i8
}

/// Port of the Java narrowing conversion `(short) someFloat`.
#[inline]
pub fn narrow_f32_to_i16(v: f32) -> i16 {
    (v as i32) as i16
}

/// Port of the Java narrowing conversion `(byte) someDouble`.
#[inline]
pub fn narrow_f64_to_i8(v: f64) -> i8 {
    (v as i32) as i8
}

/// Port of the Java narrowing conversion `(short) someDouble`.
#[inline]
pub fn narrow_f64_to_i16(v: f64) -> i16 {
    (v as i32) as i16
}

/// Port of the Java narrowing conversion `(char) someDouble`: `double -> int`
/// (saturating, NaN -> 0) then `int -> char` (wrap modulo 2^16).
#[inline]
pub fn narrow_f64_to_u16(v: f64) -> u16 {
    (v as i32) as u16
}

/// Port of the Java narrowing conversion `(byte) someInt` / `(short) someInt`.
#[inline]
pub fn narrow_i32_to_i8(v: i32) -> i8 {
    v as i8
}

// ---------------------------------------------------------------------------
// String
// ---------------------------------------------------------------------------

/// Port of `java.lang.String#hashCode()`.
///
/// `s[0]*31^(n-1) + s[1]*31^(n-2) + ... + s[n-1]`, with `int` overflow wrapping.
///
/// The iteration is over **UTF-16 code units**, not UTF-8 bytes, so any string
/// outside the BMP (or containing characters whose UTF-8 form is longer than
/// their UTF-16 form) would differ if we hashed the raw bytes.
/// This is on a seeded world-generation path: `PositionalRandomFactory.fromHashOf`.
pub fn string_hash_code(s: &str) -> i32 {
    let mut h: i32 = 0;
    for unit in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(unit as i32);
    }
    h
}

// ---------------------------------------------------------------------------
// Ordering / sign
// ---------------------------------------------------------------------------

/// Java's `Double.compare`: -0.0 sorts before +0.0 and NaN sorts after everything.
#[inline]
pub fn compare_f64(a: f64, b: f64) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        // Both equal (or both NaN): distinguish the bit patterns so that -0.0 < +0.0.
        let ab = if a.is_nan() { 0x7ff8_0000_0000_0000u64 } else { a.to_bits() };
        let bb = if b.is_nan() { 0x7ff8_0000_0000_0000u64 } else { b.to_bits() };
        ab.cmp(&bb)
    }
}

/// Java's `Float.compare`, NaN-normalised.
#[inline]
pub fn compare_f32(a: f32, b: f32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        let ab = if a.is_nan() { 0x7fc0_0000u32 } else { a.to_bits() };
        let bb = if b.is_nan() { 0x7fc0_0000u32 } else { b.to_bits() };
        ab.cmp(&bb)
    }
}

/// Port of `Math.max(int, int)`.
///
/// `>=` rather than `>`, matching Java: `Math.max(0, -0)` is `0` (identical for ints),
/// and for equal values the choice is unobservable.
#[inline]
pub fn max_i32(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}

/// Port of `Math.min(int, int)`.
#[inline]
pub fn min_i32(a: i32, b: i32) -> i32 {
    if a <= b {
        a
    } else {
        b
    }
}

/// Port of `Math.max(long, long)`.
#[inline]
pub fn max_i64(a: i64, b: i64) -> i64 {
    if a >= b {
        a
    } else {
        b
    }
}

/// Port of `Math.min(long, long)`.
#[inline]
pub fn min_i64(a: i64, b: i64) -> i64 {
    if a <= b {
        a
    } else {
        b
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_div_matches_java() {
        assert_eq!(floor_div_i32(7, 2), 3);
        assert_eq!(floor_div_i32(-7, 2), -4);
        assert_eq!(floor_div_i32(7, -2), -4);
        assert_eq!(floor_div_i32(-7, -2), 3);
        // Java: Math.floorDiv(Integer.MIN_VALUE, -1) == Integer.MIN_VALUE (wraps).
        assert_eq!(floor_div_i32(i32::MIN, -1), i32::MIN);
        assert_eq!(floor_mod_i32(i32::MIN, -1), 0);
    }

    #[test]
    fn floor_mod_matches_java() {
        assert_eq!(floor_mod_i32(7, 3), 1);
        assert_eq!(floor_mod_i32(-7, 3), 2);
        assert_eq!(floor_mod_i32(7, -3), -2);
        assert_eq!(floor_mod_i32(-7, -3), -1);
        assert_eq!(floor_div_i64(-7_000_000_000, 3), -2_333_333_334);
        assert_eq!(floor_mod_i64(-7_000_000_000, 3), 2);
    }

    #[test]
    fn abs_does_not_panic_on_min() {
        assert_eq!(abs_i32(i32::MIN), i32::MIN);
        assert_eq!(abs_i64(i64::MIN), i64::MIN);
        assert_eq!(abs_f32(-0.0).to_bits(), 0.0f32.to_bits());
    }

    #[test]
    fn narrowing_float_to_byte_saturates_then_wraps() {
        // 1.0e10f -> int saturates to Integer.MAX_VALUE -> byte wraps to -1.
        assert_eq!(narrow_f32_to_i8(1.0e10), -1);
        assert_eq!(narrow_f32_to_i8(i32::MAX as f32), -1);
        assert_eq!(narrow_f32_to_i8(-1.0e10), 0); // -> i32::MIN(0x80000000) -> byte 0x00
        assert_eq!(narrow_f32_to_i8(f32::NAN), 0);
        assert_eq!(narrow_f32_to_i8(200.0), -56); // 200 as i8
    }

    #[test]
    fn string_hash_is_utf16_based() {
        assert_eq!(string_hash_code(""), 0);
        assert_eq!(string_hash_code("a"), 97);
        assert_eq!(string_hash_code("hello"), 99162322);
        // U+4F60 is in the BMP: one UTF-16 unit 0x4F60 = 20320.
        assert_eq!(string_hash_code("\u{4f60}"), 20320);
        // U+1F600 is a surrogate PAIR in UTF-16 (0xD83D, 0xDE00) but 4 bytes in
        // UTF-8, so a byte-wise hash would disagree.
        assert_eq!(string_hash_code("\u{1f600}"), 0xD83Du32.wrapping_mul(31) as i32 + 0xDE00);
    }
    /// The JVM raises the hardware's real-indefinite QNaN (NEGATIVE) for
    /// `Inf % finite` and `finite % 0`; Rust hands back the POSITIVE default NaN.
    ///
    /// Measured from HotSpot 21 AT RUNTIME. Probing this with `javac` constants is
    /// a trap: `Float.POSITIVE_INFINITY % 360.0F` is a compile-time constant, so
    /// javac folds it with its own arithmetic and reports a POSITIVE NaN. The probe
    /// has to defeat constant folding (read through a `volatile`, or pass it in).
    #[test]
    fn rem_matches_the_jvm_on_invalid_operands() {
        // Invalid operation -> real indefinite, negative.
        assert_eq!(rem_f32(f32::INFINITY, 360.0).to_bits(), 0xffc0_0000);
        assert_eq!(rem_f32(f32::NEG_INFINITY, 360.0).to_bits(), 0xffc0_0000);
        assert_eq!(rem_f32(1.0, 0.0).to_bits(), 0xffc0_0000);
        assert_eq!(rem_f64(f64::INFINITY, 360.0).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(rem_f64(f64::NEG_INFINITY, 360.0).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(rem_f64(1.0, 0.0).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(rem_f64(-1.0, 0.0).to_bits(), 0xfff8_0000_0000_0000);

        // NaN operand -> propagate, exactly like the raw operator.
        assert_eq!(rem_f32(f32::NAN, 360.0).to_bits(), (f32::NAN % 360.0).to_bits());
        assert_eq!(rem_f64(f64::NAN, 360.0).to_bits(), (f64::NAN % 360.0).to_bits());

        // Finite results are untouched, including the sign of -0.0.
        assert_eq!(rem_f32(7.0, 3.0), 1.0);
        assert_eq!(rem_f32(-7.0, 3.0), -1.0);
        assert_eq!(rem_f64(7.0, 3.0), 1.0);
        assert_eq!(rem_f32(-0.0, 360.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(rem_f32(361.0, 360.0), 1.0);
        assert_eq!(rem_f32(-361.0, 360.0), -1.0);
    }

    /// `Math.abs(NaN)` is `a <= 0 ? 0.0 - a : a`; `NaN <= 0` is false, so NaN comes
    /// back untouched and HotSpot hands back the POSITIVE canonical NaN. Rust's
    /// `f32::abs` is a `fabsf` that also clears the sign bit, so both agree here --
    /// but the reason they agree is not obvious, so it is pinned by a test.
    #[test]
    fn abs_matches_the_jvm_on_nan() {
        assert_eq!(abs_f32(f32::NAN).to_bits(), 0x7fc0_0000);
        assert_eq!(abs_f32(-f32::NAN).to_bits(), 0x7fc0_0000);
        assert_eq!(abs_f64(f64::NAN).to_bits(), 0x7ff8_0000_0000_0000);
        assert_eq!(abs_f64(-f64::NAN).to_bits(), 0x7ff8_0000_0000_0000);
    }
    /// The x86/Intel NaN-selection rule, which HotSpot inherits and Rust does not.
    /// Every expected value below was measured on HotSpot 21 AT RUNTIME.
    #[test]
    fn arithmetic_nan_selection_matches_the_jvm() {
        let p = f64::from_bits(0x7ff8_0000_0000_0000); // +NaN
        let n = f64::from_bits(0xfff8_0000_0000_0000); // -NaN
        let inf = f64::INFINITY;

        // Second operand wins when it is NaN.
        assert_eq!(mul_f64(n, p).to_bits(), 0x7ff8_0000_0000_0000);
        assert_eq!(mul_f64(p, n).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(mul_f64(p, p).to_bits(), 0x7ff8_0000_0000_0000);
        assert_eq!(mul_f64(n, n).to_bits(), 0xfff8_0000_0000_0000);

        // Otherwise the first operand wins, keeping its sign.
        assert_eq!(add_f64(p, 1.0).to_bits(), 0x7ff8_0000_0000_0000);
        assert_eq!(add_f64(n, 1.0).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(add_f64(p, inf).to_bits(), 0x7ff8_0000_0000_0000);
        assert_eq!(sub_f64(p, inf).to_bits(), 0x7ff8_0000_0000_0000);

        // Genuine invalid operations raise the real indefinite (negative) NaN.
        assert_eq!(add_f64(inf, -inf).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(sub_f64(inf, inf).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(mul_f64(inf, 0.0).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(div_f64(inf, inf).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(sqrt_f64(-1.0).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(sqrt_f64(f64::NEG_INFINITY).to_bits(), 0xfff8_0000_0000_0000);

        // ...but 0.0/0.0 is measured as the POSITIVE default NaN on HotSpot.
        assert_eq!(div_f64(0.0, 0.0).to_bits(), 0x7ff8_0000_0000_0000);

        // The mixed-sign square sum that `Mth#lengthSquared` hits in practice.
        let s1 = mul_f64(p, p);
        let s2 = mul_f64(n, n);
        assert_eq!(add_f64(s1, s2).to_bits(), 0xfff8_0000_0000_0000);

        // Finite results are bit-identical to the plain operator.
        assert_eq!(add_f64(1.0, 2.0).to_bits(), (1.0f64 + 2.0).to_bits());
        assert_eq!(mul_f64(1.0, 2.0).to_bits(), (1.0f64 * 2.0).to_bits());
        assert_eq!(div_f64(1.0, 2.0).to_bits(), (1.0f64 / 2.0).to_bits());
        assert_eq!(sub_f64(1.0, 2.0).to_bits(), (1.0f64 - 2.0).to_bits());
        assert_eq!(sqrt_f64(2.0).to_bits(), 2.0f64.sqrt().to_bits());
    }

    #[test]
    fn arithmetic_nan_selection_f32() {
        let p = f32::from_bits(0x7fc0_0000);
        let n = f32::from_bits(0xffc0_0000);
        let inf = f32::INFINITY;
        assert_eq!(mul_f32(n, p).to_bits(), 0x7fc0_0000);
        assert_eq!(mul_f32(p, n).to_bits(), 0xffc0_0000);
        assert_eq!(add_f32(p, 1.0).to_bits(), 0x7fc0_0000);
        assert_eq!(add_f32(n, 1.0).to_bits(), 0xffc0_0000);
        assert_eq!(add_f32(inf, -inf).to_bits(), 0xffc0_0000);
        assert_eq!(sub_f32(inf, inf).to_bits(), 0xffc0_0000);
        assert_eq!(mul_f32(inf, 0.0).to_bits(), 0xffc0_0000);
        assert_eq!(div_f32(0.0, 0.0).to_bits(), 0x7fc0_0000);
        assert_eq!(sqrt_narrow_f32(-1.0).to_bits(), 0xffc0_0000);
    }

    /// `Math.min`/`Math.max` return the actual NaN operand, payload and sign
    /// intact. Rust's `f32::min` instead returns the non-NaN operand.
    #[test]
    fn min_max_propagate_the_nan_operand() {
        let p = f32::from_bits(0x7fc0_0000);
        let n = f32::from_bits(0xffc0_0000);
        assert_eq!(min_f32(n, 45.0).to_bits(), 0xffc0_0000);
        assert_eq!(min_f32(45.0, n).to_bits(), 0xffc0_0000);
        assert_eq!(max_f32(n, 45.0).to_bits(), 0xffc0_0000);
        assert_eq!(min_f32(p, 45.0).to_bits(), 0x7fc0_0000);
        // Finite behaviour, including the signed-zero asymmetry.
        assert_eq!(min_f32(1.0, 2.0), 1.0);
        assert_eq!(max_f32(1.0, 2.0), 2.0);
        assert!(min_f32(0.0, -0.0).is_sign_negative());
        assert!(max_f32(0.0, -0.0).is_sign_positive());
    }
}