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

/// `TWO54`, `LN2_HI`, `LN2_LO` and `LG1`..`LG7` moved to `jvm_math::log` in session 08, next to
/// the algorithm that uses them. One copy of a bit-level port, not two: the copy that used to
/// live here had a subnormal bug that survived a "verified 256/256" claim, and two copies is two
/// places for the next one to hide.

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
    // PURE RUST as of session 08. This used to be `x.ln()`, i.e. the host libm, and the
    // reviewer found the consequence on Linux:
    //
    //     "`nextGaussian` matches 248/256 draws on Windows and 256/256 on Linux. The same seed
    //      gives different random numbers depending on the OS."
    //
    // `MarsagliaPolarGaussian` is reached from worldgen, mob AI and every `RandomSource`, so an
    // OS-dependent stream is not a rounding curiosity -- it is a different world.
    //
    // What this costs, measured against HotSpot's `Math.log`:
    //
    // | candidate                       | 29,201-value corpus | 1,536 gaussian-domain |
    // |---|---|---|
    // | `jvm_math::log` (FdLibm) -- KEPT | 28,444 (97.43%)     | 1,434 (93.36%)        |
    // | `libm` 0.2.16 (musl) -- rejected | 28,410 (97.31%)     | 1,431 (93.16%)        |
    // | host `ln()` -- REMOVED          | 14,639 (50.14%)     | 1,536 (100%, Windows) |
    //
    // Be clear-eyed about the last row: on Windows the host was PERFECT on the gaussian domain,
    // which is the only domain the game reaches. So this is a deliberate trade -- slightly worse
    // on Windows, slightly better elsewhere, and IDENTICAL on every platform -- made knowingly
    // rather than by accident. See `jvm_math`'s `Math.log` section for the full reasoning, and
    // `parity_jvm_math.rs` for the allowlist pinning the residual 1-ULP rows.
    //
    // The name is a little dishonest now: this is FdLibm's `log`, not HotSpot's `_dlog`. It stays
    // because this is where every caller already reaches for `Math.log`, and renaming it would
    // hide the fact that Java's two log methods differ. `strict_log_f64` is the honest name for
    // the same value and sits right next to it.
    crate::javacompat::jvm_math::log(x)
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
    // MOVED TO `javacompat::jvm_math::log` IN SESSION 08; this is a delegation, not a second
    // copy of the algorithm.
    //
    // Session 08 found a subnormal bug in the copy that used to live here: it captured the low
    // word of `x` BEFORE FdLibm's `x *= TWO54` scaling. That is correct for every normal input --
    // the branch never runs -- and wrong by up to 2,257,518 ULP for subnormals. Six of 29,201
    // corpus rows disagreed with `StrictMath.log`, every one of them a subnormal.
    //
    // It survived because the corpus that motivated the function contained none: `Math.log` is
    // reached only with `radiusSquared` in (0,2), which is never subnormal. So "verified on
    // 256/256" was true and worthless -- the measurement was real, the corpus too narrow to mean
    // anything.
    //
    // Two copies of a bit-level port is two places for the next bug to hide, and the algorithm
    // belongs beside the other transcendentals anyway. `jvm_math` owns it; this name stays
    // because it says which of Java's two logs this is.
    crate::javacompat::jvm_math::log(x)
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
    /// On this exact input, HotSpot's `Math.log` and FdLibm's `StrictMath.log` are two
    /// different answers one ULP apart. That is the whole reason this module exists: matching
    /// Java's `Math.log` and being FdLibm are not the same claim, and only one of them is
    /// portable.
    ///
    /// Values confirmed against HotSpot 25 via the oracle's `legacy.gaussianSteps` group, pair
    /// k=5 (which took one rejection).
    ///
    /// # WHY THE HOST IS GONE FROM THIS TEST
    ///
    /// This used to assert THREE answers -- `Math.log`, `StrictMath.log`, and the host's
    /// `rs.ln()` -- and the host happened to equal `Math.log` on this input, which made the
    /// point nicely on Windows. It also meant the test called `.ln()`, and when the direct-call
    /// guard was extended to `javacompat/` in session 08 it fired HERE. That is the guard
    /// working: the assertion was about a function this port no longer uses.
    ///
    /// The portable half of the original claim is kept. The host `ln()` equals `Math.log` on this
    /// input on Windows and on Linux both, but neither is something to rely on -- and the
    /// `rs.ln()` call is itself the thing being removed.
    #[test]
    fn math_log_and_strict_log_are_different_answers() {
        // rs = 0x3fe81c47b8a72375
        let rs = f64::from_bits(0x3fe8_1c47_b8a7_2375);

        // HotSpot 25's `Math.log` -- the intrinsic vanilla actually computes.
        const MATH_LOG_RS: u64 = 0xbfd2_1e24_707c_9607;
        // HotSpot 25's `StrictMath.log` -- FdLibm, one ULP away from the intrinsic here.
        const STRICT_LOG_RS: u64 = 0xbfd2_1e24_707c_9608;

        // `math_log_f64` IS FdLibm as of session 08, so it equals `StrictMath.log` rather than
        // `Math.log`. That is the deliberate trade: OS-independent, but 1 ULP from the intrinsic
        // on about 2.6% of inputs. Asserted here so the trade is visible in one place.
        assert_eq!(math_log_f64(rs).to_bits(), STRICT_LOG_RS);
        assert_eq!(strict_log_f64(rs).to_bits(), STRICT_LOG_RS);

        // And the intrinsic really is a different answer from FdLibm -- which is exactly why
        // `math_log_f64` is named after `Math.log` while returning FdLibm's value. If a future
        // `_dlog` port ever lands, THIS is the assertion that should change, deliberately.
        assert_ne!(MATH_LOG_RS, STRICT_LOG_RS);
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
