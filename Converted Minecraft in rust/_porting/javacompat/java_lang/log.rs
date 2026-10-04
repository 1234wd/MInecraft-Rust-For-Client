//! Port of: (no `.java` counterpart — JDK internals / third-party numerics)
//! Java class(es): `java.lang.Math#log(double)`, `java.lang.StrictMath#log(double)`
//! Status: PORTED
//!
//! # `Math.log` IS NOT ONE FUNCTION. IT IS TWO, AND THEY DISAGREE.
//!
//! This module carries both, because Java does:
//!
//! | Java call | what it actually is |
//! |---|---|
//! | `StrictMath.log(x)` | **fdlibm** `e_log.c`, shipped inside the JDK |
//! | `Math.log(x)` | HotSpot's **`_dlog` intrinsic**, separate hand-written assembly |
//!
//! On x86-64 HotSpot intrinsifies `Math.log(double)` to `_dlog`. That intrinsic is
//! *not* fdlibm and it is *not* the host libm. It is close to fdlibm but not equal —
//! measured over 256 arbitrary doubles drawn from `MarsagliaPolarGaussian`'s
//! `radiusSquared` values:
//!
//! ```text
//! Math.log       == StrictMath.log   : 234 / 256   (22 differ)
//! Math.log       == host f64::ln()    : 255 / 256   ( 1 differs)
//! StrictMath.log == host f64::ln()    : 235 / 256
//! StrictMath.log == strict_log_f64()  : 256 / 256   <- this module, exact
//! ```
//!
//! Every difference is 1 ULP. 22/256 = 8.6% of inputs, which is far too high to
//! ignore and far too low to find by staring at a transcript.
//!
//! Concrete example, `x = 0x3fe81c47b8a72375`:
//!
//! ```text
//! Math.log       0xbfd21e24707c9607
//! host f64::ln() 0xbfd21e24707c9607   (same)
//! StrictMath.log 0xbfd21e24707c9608   (1 ULP higher -- more negative)
//! ```
//!
//! # What this means for the port
//!
//! Vanilla Minecraft calls `Math.log`, not `StrictMath.log`, so
//! [`math_log_f64`] is the function that must be bit-exact. [`math_log_f64`] delegates
//! to the host `ln()`, which is the closest available match at 255/256 — better than
//! fdlibm's 234/256. It is NOT exact, and this module does not pretend otherwise.
//!
//! Closing the last 1-in-256 requires transcribing HotSpot's `generate_math_log`
//! assembly from `stubGenerator_x86_64.cpp`. That is filed as an open question
//! rather than guessed at, because guessing a transcendental's last ULP is exactly
//! the kind of silent divergence this port exists to eliminate.
//!
//! **`MarsagliaPolarGaussian::next_gaussian` is therefore 255/256-exact, not
//! 256/256.** Its golden tests pass today only because the specific seeds in
//! `random.txt` happen not to hit a differing input; the gap is measured and
//! documented rather than hidden.
//!
//! # Why this is the same class of problem as the embedded `Mth` trig tables
//!
//! Vanilla already carries 65536-entry `SIN`/`COS`/`ASIN` lookup tables in `Mth`
//! precisely because host libm disagrees with the JVM. Every transcendental Minecraft
//! relies on for parity needs the same treatment: reproduce the JVM's answer, not
//! the mathematically best one.

/// `2^-54`, used to scale subnormals into the normal range.
const TWO54: f64 = 1.8014398509481984e16;

const LN2_HI: f64 = 6.93147180369123816490e-01;
const LN2_LO: f64 = 1.90821492927058770002e-10;

// fdlibm's minimax polynomial coefficients for log(1+f), f in [sqrt(2)/2-1, sqrt(2)-1].
const LG1: f64 = 6.666666666666735130e-01;
const LG2: f64 = 3.999999999940941908e-01;
const LG3: f64 = 2.857142874366239149e-01;
const LG4: f64 = 2.222219843214978396e-01;
const LG5: f64 = 1.818357216161805012e-01;
const LG6: f64 = 1.531383769920937332e-01;
const LG7: f64 = 1.479819860511658591e-01;

// =============================================================================
// `Math.log` — what vanilla actually calls
// =============================================================================

/// Stand-in for `java.lang.Math#log(double)`.
///
/// Delegates to the host `f64::ln()`, which agrees with HotSpot 25's `_dlog`
/// intrinsic on 255 of the 256 measured inputs and with `StrictMath.log` on 235.
///
/// # Why not just call `x.ln()` at every call site?
///
/// Because "just call `ln()`" is a decision, and a decision that is invisible at
/// every other call site. Naming it here means there is exactly one place to change
/// when HotSpot's `generate_math_log` assembly gets transcribed, and one place to
/// grep to find out how exposed the port is.
///
/// # Accuracy
///
/// 255/256 bit-exact against HotSpot 25 (x86-64). The residual 1/256 is documented in
/// `_porting/OPEN_QUESTIONS.md`; do not treat this function as exact.
#[inline]
pub fn math_log_f64(x: f64) -> f64 {
    x.ln()
}

// =============================================================================
// `StrictMath.log` — fdlibm, transcribed exactly
// =============================================================================

/// Port of `java.lang.StrictMath#log(double)`, i.e. **fdlibm's `e_log.c`** as shipped
/// in the JDK.
///
/// Verified bit-exact against HotSpot 25's `StrictMath.log` on 256/256 arbitrary
/// inputs (see the module docs for the measurement). Vanilla does not currently call
/// `StrictMath.log` in the ported surface, but this exists for two reasons:
///
/// 1. It is the **proof** that `Math.log` and `StrictMath.log` really are different
///    algorithms, which is the whole basis of the divergence documented above.
/// 2. If a future file uses `StrictMath.log`, the exact implementation is already here
///    rather than being rediscovered later.
///
/// # Faithfulness
///
/// The bit-level structure of `e_log.c` is preserved exactly — the signed exponent
/// arithmetic, the `i`/`j` signed test that selects between three different final
/// corrections, and the `k == 0` special cases. Every one of those details changes
/// rounding, so none of them may be "simplified".
///
/// # Panics
///
/// Never. NaN, infinities, zeros, negatives and subnormals all return, matching Java.
pub fn strict_log_f64(x: f64) -> f64 {
    let bits = x.to_bits();
    // fdlibm's `hx` is a SIGNED Int32. That matters: it is what routes -0.0 and every
    // negative value into the first branch, and what makes `log(-0.0) == -Inf` instead
    // of -0.0. Treating the high word as unsigned silently breaks both.
    let mut hx = (bits >> 32) as u32;
    let lx = bits as u32;
    let mut k: i32 = 0;
    let mut x = x;

    // ---- extract exponent, handling subnormals ----
    if (hx as i32) < 0x0010_0000 {
        // x < 2^-1022, or negative, or +-0.
        // NOTE the explicit parens: Rust's `==` binds tighter than `|`, so
        // `(hx & mask) | lx == 0` would parse as `(hx & mask) | (lx == 0)`.
        if ((hx & 0x7fff_ffff) | lx) == 0 {
            // x == +-0  ->  log(+-0) = -Inf
            return f64::NEG_INFINITY;
        }
        if (hx as i32) < 0 {
            // x < 0  ->  NaN, produced as (x-x)/0.0 exactly as fdlibm does so the
            // payload and sign behave identically.
            return (x - x) / 0.0;
        }
        // Subnormal: scale up and remember it.
        k -= 54;
        x *= TWO54;
        hx = (x.to_bits() >> 32) as u32;
    }

    if hx >= 0x7ff0_0000 {
        // NaN or +Inf. fdlibm returns x+x: +Inf for infinity, and for NaN it quiets a
        // signalling NaN and propagates the payload.
        return x + x;
    }

    // ---- extract exponent and mantissa ----
    k += ((hx >> 20) as i32) - 1023;
    hx &= 0x000f_ffff;
    // Round to nearest: if the top two mantissa bits are 01 or 10, pre-scale by 2 so
    // |f| lands in the polynomial's accurate range.
    let i = (hx + 0x9_5f64) & 0x0010_0000;
    // fdlibm: SET_HIGH_WORD(x, hx | (i ^ 0x3ff00000)). The OR with the retained
    // mantissa bits is load-bearing -- writing just `i ^ 0x3ff00000` discards the
    // mantissa and gives wildly wrong results for most inputs.
    x = f64::from_bits((((hx | (i ^ 0x3ff0_0000)) as u64) << 32) | lx as u64);
    k += (i >> 20) as i32;

    let f = x - 1.0;
    let dk = k as f64;

    if (0x000f_ffff & (2 + hx)) < 3 {
        // |f| < 2^-20: log(1+f) ~ f - f*f/2, no polynomial needed.
        if f == 0.0 {
            if k == 0 {
                return 0.0;
            }
            return dk * LN2_HI + dk * LN2_LO;
        }
        let r = f * f * (0.5 - 0.333_333_333_333_333_33 * f);
        if k == 0 {
            return f - r;
        }
        return dk * LN2_HI - ((r - dk * LN2_LO) - f);
    }

    let s = f / (2.0 + f);
    let z = s * s;
    let w = z * z;
    // fdlibm uses SIGNED ints here and tests `i > 0`, not `i != 0`. Doing this with
    // unsigned wrapping arithmetic silently picks the wrong correction path for about
    // a quarter of all inputs, because a negative `i` wraps to a large positive value.
    // That is not a rounding difference, it is a different formula.
    let mut i = (hx as i32).wrapping_sub(0x0006_147a);
    let j = 0x0006_b851i32.wrapping_sub(hx as i32);
    let t1 = w * (LG2 + w * (LG4 + w * LG6));
    let t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    let r = t2 + t1;
    i |= j;

    if i > 0 {
        // |f| >= 2^-20
        let hfsq = 0.5 * f * f;
        if k == 0 {
            return f - (hfsq - s * (hfsq + r));
        }
        return dk * LN2_HI - ((hfsq - (s * (hfsq + r) + dk * LN2_LO)) - f);
    }
    if k == 0 {
        return f - s * (f - r);
    }
    dk * LN2_HI - ((s * (f - r) - dk * LN2_LO) - f)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Values where every implementation agrees. If a "simplification" breaks one of
    /// these, it broke something.
    #[test]
    fn agrees_with_glibc_on_round_numbers() {
        for (x, want) in [
            (1.0f64, 0.0f64),
            (2.0f64, 0.693_147_180_559_945_3),
            (0.5f64, -0.693_147_180_559_945_3),
            (10.0f64, 2.302_585_092_994_046),
            (std::f64::consts::E, 1.0),
        ] {
            assert_eq!(strict_log_f64(x).to_bits(), want.to_bits(), "log({x})");
        }
    }

    /// The evidence behind the whole module, pinned.
    ///
    /// On this exact input the three implementations form three different answers,
    /// which is the cheapest possible demonstration that "use `f64::ln()`" and "match
    /// Java" are different claims.
    ///
    /// Values confirmed against HotSpot 25 via the oracle's `legacy.gaussianSteps`
    /// group, pair k=5 (which took one rejection).
    #[test]
    fn math_log_and_strict_log_and_ln_are_three_different_answers() {
        // rs = 0x3fe81c47b8a72375
        let rs = f64::from_bits(0x3fe8_1c47_b8a7_2375);

        // Math.log -- HotSpot's intrinsic. This is what vanilla computes. The host
        // ln() happens to agree with it on this input.
        assert_eq!(rs.ln().to_bits(), 0xbfd2_1e24_707c_9607);
        assert_eq!(math_log_f64(rs).to_bits(), 0xbfd2_1e24_707c_9607);

        // StrictMath.log -- fdlibm, one ULP away from the intrinsic on this input.
        assert_eq!(strict_log_f64(rs).to_bits(), 0xbfd2_1e24_707c_9608);

        // And the two really are different.
        assert_ne!(strict_log_f64(rs).to_bits(), math_log_f64(rs).to_bits());
    }

    #[test]
    fn special_cases_match_java() {
        assert_eq!(strict_log_f64(f64::INFINITY), f64::INFINITY);
        assert!(strict_log_f64(f64::NAN).is_nan());
        // Both zeros give -Inf. This is the case that catches treating the high word
        // as unsigned, where -0.0 falls through to `x + x` and returns -0.0.
        assert_eq!(strict_log_f64(0.0), f64::NEG_INFINITY);
        assert_eq!(strict_log_f64(-0.0), f64::NEG_INFINITY);
        assert_eq!(strict_log_f64(0.0).to_bits(), (-f64::INFINITY).to_bits());
        assert_eq!(strict_log_f64(-0.0).to_bits(), (-f64::INFINITY).to_bits());
        assert!(strict_log_f64(-1.0).is_nan());
        assert!(strict_log_f64(-5.0).is_nan());
        // log(1.0) is exactly +0.0 -- never -0.0. fdlibm's `f == 0.0` branch returns a
        // literal `zero`, so the sign of zero here is real behaviour worth pinning.
        assert_eq!(strict_log_f64(1.0).to_bits(), 0.0f64.to_bits());
        assert_ne!(strict_log_f64(1.0).to_bits(), (-0.0f64).to_bits());
    }

    #[test]
    fn handles_subnormals() {
        let tiny = f64::from_bits(1); // smallest positive subnormal
        let r = strict_log_f64(tiny);
        assert!(r.is_finite());
        // log(2^-1074) ~= -744.44
        assert!((r + 744.44).abs() < 0.01, "log(min subnormal) = {r}");
        // Another subnormal: finite, and in the right ballpark (log(5e-320) ~= -735).
        let r2 = strict_log_f64(5e-320);
        assert!(r2.is_finite() && r2 < -700.0, "log(5e-320) = {r2}");
    }

    #[test]
    fn strict_log_is_close_to_true_log() {
        // log(1+x) = x - x^2/2 + O(x^3), so the series deviation is x^2/2. But the
        // ARGUMENT `1.0 + x` is itself rounded, by up to half an ulp of 1.0, and log
        // amplifies that by ~1. So the observable deviation from `x` is
        //
        //     x^2 / 2  +  eps(1.0)
        //
        // Bound by exactly that. Asserting against a hand-picked constant gets it
        // wrong in both directions -- too tight and ordinary argument rounding trips
        // it (measured 1.108e-16 against an x^2/2 of 5e-17), too loose and a broken
        // polynomial sails through.
        for k in 1..20 {
            let x = k as f64 * 1e-8;
            let l = strict_log_f64(1.0 + x);
            let bound = 0.5 * x * x + f64::EPSILON;
            assert!(
                (l - x).abs() <= bound,
                "log(1+{x}) = {l}, dev {} exceeds {bound}",
                (l - x).abs()
            );
        }
    }
}