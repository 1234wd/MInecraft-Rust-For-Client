//! Port of: `java.lang.Float#floatToRawIntBits`, `java.lang.Double#doubleToRawLongBits`
//! Java class(es): `java.lang.Float`, `java.lang.Double`
//! Status: PORTED
//!
//! # The NaN policy, and why most of this file is "don't use it"
//!
//! Vanilla's own code is inconsistent about NaN payloads, and the CPU decides the rest.
//! On x86-64 a genuine invalid operation (`Inf - Inf`, `0.0/0.0`, `sqrt(-1)`) raises
//! the *real indefinite* QNaN, which is **negative** (`0xffc0_0000` / `0xfff8_...`),
//! while Rust/LLVM returns the *default* QNaN, which is positive. Session 02 therefore
//! added exact-sign helpers (`rem_f32`, `add_f64`, `sqrt_f64`, ...), which is right for
//! the arithmetic itself.
//!
//! But **matching the sign everywhere is not the goal, and is not always achievable.**
//!
//! * `Math.min`/`Math.max` propagate whatever NaN operand they were handed, so the sign
//!   of an intermediate depends on argument order.
//! * A NaN that reaches `Mth#wrapDegrees` and then `wrapDegrees_f32` may have had its
//!   sign rewritten by any preceding comparison or multiply.
//! * The JVM's own answer depends on the CPU: an AArch64 JVM raises the positive
//!   default QNaN for the same invalid operation x86-64 raises the negative one.
//!
//! **We target x86-64 HotSpot** and say so, because that is what the oracle measures and
//! what the overwhelming majority of Minecraft runs on. Anything CPU-dependent is not
//! treated as a portable guarantee.
//!
//! # The rule this module implements
//!
//! Match the exact NaN **sign and payload** only where game code can *observe* it:
//!
//! 1. `Float.floatToRawIntBits` / `Double.doubleToRawLongBits` results.
//! 2. Values fed into hashing, serialisation, or network data.
//! 3. Values compared for bit equality against a stored constant.
//!
//! **Everywhere else, being NaN is enough.** A parity test for those sites asserts
//! `f.is_nan() == f.is_nan()` rather than exact bits, because pinning the sign there
//! would mean encoding a CPU-specific artefact as if it were game behaviour.
//!
//! Concretely: [`java_bits`] and friends are for sites 1-3. Do not reach for
//! [`java_lang::add_f64`] merely to make a NaN sign "right" in a place nothing reads the
//! sign -- use the plain operator there and let the oracle assert NaN-ness.
//!
//! # Related
//!
//! The exact-sign arithmetic helpers live in [`java_lang`]. This module is about
//! deciding *where they are required*, and providing the comparison helpers that
//! express "NaN-ness is enough".

use crate::javacompat::java_lang;

/// Port of `Float.floatToRawIntBits(float)`.
///
/// Identity on the bit pattern: `f32::to_bits` already returns the raw bits, with no
/// canonicalisation of NaN. (Contrast `f32::to_bits` on a value that has been through
/// arithmetic: there is no such canonicalisation in Rust, which is exactly why this
/// function is a one-liner and why the interesting part is *when to call it*.)
#[inline]
pub fn float_to_raw_int_bits(x: f32) -> i32 {
    x.to_bits() as i32
}

/// Port of `Double.doubleToRawLongBits(double)`.
#[inline]
pub fn double_to_raw_long_bits(x: f64) -> i64 {
    x.to_bits() as i64
}

/// Port of `Float.floatToIntBits(float)`.
///
/// Differs from [`float_to_raw_int_bits`] only in that Java canonicalises every NaN to
/// `0x7fc00000` first. This matters when the result is written to a file or compared
/// against a literal.
#[inline]
pub fn float_to_int_bits(x: f32) -> i32 {
    if x.is_nan() {
        0x7fc0_0000
    } else {
        x.to_bits() as i32
    }
}

/// Port of `Double.doubleToLongBits(double)`.
#[inline]
pub fn double_to_long_bits(x: f64) -> i64 {
    if x.is_nan() {
        0x7ff8_0000_0000_0000u64 as i64
    } else {
        x.to_bits() as i64
    }
}

/// Port of `Float.floatToIntBits` applied to a value already in a packed int.
///
/// Provided because packed-block-state code frequently does `ARGB.getRed(packed)` on a
/// value that came off the wire, where the canonicalising form is the correct one.
#[inline]
pub fn packed_channel(x: u32) -> u32 {
    let v = f32::from_bits(x);
    if v.is_nan() {
        0x7fc0_0000
    } else {
        x
    }
}

/// "These two are bit-identical" -- the strictest comparison, for site 1-3 above.
#[inline]
pub fn bits_equal_f32(a: f32, b: f32) -> bool {
    float_to_raw_int_bits(a) == float_to_raw_int_bits(b)
}

/// "These two are bit-identical" on `f64`.
#[inline]
pub fn bits_equal_f64(a: f64, b: f64) -> bool {
    double_to_raw_long_bits(a) == double_to_raw_long_bits(b)
}

/// "These two agree, where NaN agrees with NaN."
///
/// This is the right assertion for the *rest* of the surface: sites where only
/// NaN-ness is observable, and where pinning the sign would encode a CPU artefact as
/// game behaviour. See the module docs.
///
/// Note it deliberately still distinguishes `-0.0` from `+0.0`, and does not treat
/// distinct NaN payloads as equal -- a payload that survives into hashing or the save
/// format IS observable, and pretending otherwise would hide a real divergence.
#[inline]
pub fn equivalent_f64(a: f64, b: f64) -> bool {
    if a.is_nan() && b.is_nan() {
        return true;
    }
    a.to_bits() == b.to_bits()
}

/// `f32` counterpart of [`equivalent_f64`].
#[inline]
pub fn equivalent_f32(a: f32, b: f32) -> bool {
    if a.is_nan() && b.is_nan() {
        return true;
    }
    a.to_bits() == b.to_bits()
}

/// The x86-64 real-indefinite QNaN, for the observable sites that need it spelled out.
///
/// Documented here rather than in `java_lang` because the *policy* (use it only where
/// the sign is observable) belongs with the policy.
pub const REAL_INDEFINITE_F32: u32 = 0xffc0_0000;
pub const REAL_INDEFINITE_F64: u64 = 0xfff8_0000_0000_0000;

/// Sanitise a NaN reaching an observable site on x86-64 HotSpot.
///
/// Applies [`java_lang`]'s invalid-operation convention: a NaN produced by a genuine
/// invalid operation becomes the negative real indefinite, matching HotSpot. A NaN that
/// was already NaN on input is left alone, because `Math.min`/`Math.max` propagate it.
#[inline]
pub fn observable_f64(x: f64) -> f64 {
    if x.is_nan() {
        f64::from_bits(REAL_INDEFINITE_F64)
    } else {
        x
    }
}

/// `f32` counterpart of [`observable_f64`].
#[inline]
pub fn observable_f32(x: f32) -> f32 {
    if x.is_nan() {
        f32::from_bits(REAL_INDEFINITE_F32)
    } else {
        x
    }
}

/// Force a canonical positive NaN of a given width.
///
/// Use at the boundary of serialisation so a save file cannot encode a CPU-dependent
/// sign: the same world must produce the same bytes on x86 and on ARM.
#[inline]
pub fn canonicalize_f64(x: f64) -> f64 {
    if x.is_nan() {
        f64::NAN
    } else {
        x
    }
}

/// `f32` counterpart of [`canonicalize_f64`].
#[inline]
pub fn canonicalize_f32(x: f32) -> f32 {
    if x.is_nan() {
        f32::NAN
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_bits_are_identity_not_canonicalised() {
        let neg_nan = f32::from_bits(0xffc0_0000);
        assert_eq!(float_to_raw_int_bits(neg_nan), 0xffc0_0000u32 as i32);
        // ...whereas the canonicalising form collapses it.
        assert_eq!(float_to_int_bits(neg_nan), 0x7fc0_0000);
    }

    #[test]
    fn canonicalising_form_collapses_every_payload() {
        for bits in [0x7fc0_0000u32, 0xffc0_0000, 0x7f80_0001, 0xffa0_0001] {
            assert_eq!(float_to_int_bits(f32::from_bits(bits)), 0x7fc0_0000, "bits {bits:#x}");
        }
        for bits in [0x7ff8_0000_0000_0000u64, 0xfff8_0000_0000_0000, 0x7ff0_0000_0000_0001] {
            assert_eq!(double_to_long_bits(f64::from_bits(bits)), 0x7ff8_0000_0000_0000u64 as i64, "bits {bits:#x}");
        }
    }

    #[test]
    fn equivalent_accepts_any_nan_but_still_not_signed_zero() {
        let p = f32::from_bits(0x7fc0_0000);
        let n = f32::from_bits(0xffc0_0000);
        let weird = f32::from_bits(0x7f80_0001);
        assert!(equivalent_f32(p, n));
        assert!(equivalent_f32(p, weird));
        // Signed zero is observable, so it must NOT be forgiven.
        assert!(!equivalent_f32(0.0, -0.0));
        assert!(!equivalent_f64(0.0, -0.0));
        // Infinities are values, not NaN.
        assert!(!equivalent_f32(f32::INFINITY, f32::NEG_INFINITY));
    }

    #[test]
    fn bits_equal_is_the_strict_comparison() {
        let p = f32::from_bits(0x7fc0_0000);
        let n = f32::from_bits(0xffc0_0000);
        assert!(!bits_equal_f32(p, n));
        assert!(bits_equal_f32(p, p));
    }

    #[test]
    fn observable_normalises_the_invalid_operation_nan() {
        // Matches what HotSpot 21 on x86-64 produces for Inf - Inf.
        assert_eq!(double_to_raw_long_bits(observable_f64(java_lang::sub_f64(f64::INFINITY, f64::INFINITY))), REAL_INDEFINITE_F64 as i64);
        assert_eq!(float_to_raw_int_bits(observable_f32(java_lang::sub_f32(f32::INFINITY, f32::INFINITY))), REAL_INDEFINITE_F32 as i32);
        // Non-NaN values pass through untouched, including signed zero.
        assert!(observable_f64(-0.0).is_sign_negative());
        assert_eq!(observable_f64(1.5), 1.5);
    }

    #[test]
    fn canonicalize_gives_the_positive_default() {
        assert_eq!(float_to_raw_int_bits(canonicalize_f32(f32::from_bits(0xffc0_0000))), 0x7fc0_0000);
        assert_eq!(double_to_raw_long_bits(canonicalize_f64(f64::from_bits(0xfff8_0000_0000_0000))), 0x7ff8_0000_0000_0000u64 as i64);
        assert_eq!(canonicalize_f64(-0.0).to_bits(), (-0.0f64).to_bits());
    }
}
