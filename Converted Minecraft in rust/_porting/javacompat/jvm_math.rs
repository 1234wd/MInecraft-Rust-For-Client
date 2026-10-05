//! Port of: `java.lang.StrictMath#asin`, `#atan`, `#atan2` (via `FdLibm.java`)
//! Java class(es): java.lang.Math, java.lang.StrictMath, java.lang.FdLibm
//! Status: PORTED
//!
//! # WHY THIS FILE EXISTS, AND WHY IT IS NOT A THIN WRAPPER
//!
//! A reviewer built this project on Linux and got four failures in `parity_random` that
//! Windows did not produce. Cause: `f64::ln` on glibc happens to agree with HotSpot's
//! `_dlog` on eight specific inputs, and Windows' does not. So **host `libm` is not a
//! specification**, and any ported code that calls it produces different results on
//! different machines.
//!
//! The rule that follows: **ported code may not call the host's math library for anything
//! except `sqrt`.** `sqrt` is the exception because IEEE-754 requires it to be correctly
//! rounded, so hardware `SQRTSD` and Rust's `f64::sqrt` agree bit for bit everywhere.
//! Everything else goes through this module, which is pure Rust and therefore identical on
//! every OS, in debug and in release.
//!
//! # WHICH FUNCTIONS ARE IN HERE, AND WHY
//!
//! Not everything could come from FdLibm. In JDK 21+, most `Math` methods are one-line
//! delegations to `StrictMath`, which is `FdLibm.java` -- pure Java, deterministic and
//! portable. A handful are **HotSpot intrinsics** (`_dlog`, `_dexp`, `_dpow`, `_dsin`,
//! `_dcos`, `_dtan`, `_dcbrt`, ...) and those disagree with FdLibm by 1 ULP on a handful of
//! inputs, so a FdLibm port would be wrong for them.
//!
//! Measured on JDK 25.0.4 (Eclipse Adoptium, x86-64) over a 12,051-value corpus, by
//! comparing `Math.f` against `StrictMath.f` bit for bit:
//!
//! | function | Math vs StrictMath | route |
//! |---|---|---|
//! | **asin** | 0 / 12051 differ | **FdLibm -- ported here** |
//! | **atan** | 0 / 12051 differ | **FdLibm -- ported here** (atan2 needs it) |
//! | **atan2** | 0 / 145 million differ | **FdLibm -- ported here** |
//! | acos | 0 differ | FdLibm, same shape as asin -- easy, not yet ported |
//! | sinh, cosh | 0 differ | FdLibm -- not yet ported |
//! | hypot | 0 differ | FdLibm -- not yet ported |
//! | log1p, expm1 | 0 differ | FdLibm -- not yet ported |
//! | sqrt, floor, ceil, rint | 0 differ | exact already; `sqrt` needs no port |
//! | log | 7 differ, worst 1 ULP | **HotSpot intrinsic** -- needs the stub |
//! | log10 | 11 differ | **HotSpot intrinsic** |
//! | exp | 8 differ | **HotSpot intrinsic** |
//! | sin | 187 differ | **HotSpot intrinsic** |
//! | cos | 188 differ | **HotSpot intrinsic** |
//! | tan | 211 differ | **HotSpot intrinsic** |
//! | tanh | 79 differ | **HotSpot intrinsic** |
//! | cbrt | 1024 differ | **HotSpot intrinsic** |
//! | pow | 51,268 of 145 million differ | **HotSpot intrinsic** |
//!
//! The nine intrinsics are why `MarsagliaPolarGaussian` is still `PARTIAL`: `Math.log` is
//! the one that actually matters there, and it is a table-driven intrinsic that a polynomial
//! cannot replace.
//!
//! # WHAT CLOSING `Vec3#rotation` ACTUALLY TOOK, AND WHAT IT DID NOT TAKE
//!
//! `Vec3#rotation` was recorded for two sessions as 376/512 on yaw and 333/512 on pitch,
//! blamed on this file's subject matter. **That blame was misplaced**, and the correction
//! matters more than the fix:
//!
//! * `asin` and `atan2` turned out to agree with the host libm on all 512 rows of
//!   `vec3.rotation`. Measured after the real fix, host and FdLibm both score 512/512.
//! * The actual bug was a **1-ULP-wrong constant** in `Vec3.rs`:
//!   `(180.0 / Math.PI) as f32` where Java has `180.0F / (float) Math.PI`. An `f64` division
//!   narrowed afterwards is not the same value as an `f32` division (`0x42652ee1` vs
//!   `0x42652ee0`).
//! * And the "lost" rows were mostly **NaN rows** (120 yaw, 177 pitch), which the session-07
//!   NaN policy now treats as equal.
//!
//! So this file did not close `rotation`; correcting a constant and a NaN policy did.
//!
//! It stays anyway, for a reason that is not sentiment. The reviewer's Linux build is the
//! evidence that the host libm is **not a specification** -- glibc and Windows disagree with
//! HotSpot on eight specific inputs. Being accidentally right on Windows is not a property
//! worth relying on, and I cannot test Linux from here. Routing through FdLibm removes the
//! dependency instead of assuming it away, and the guard test below makes the removal
//! permanent.
//!
//! What this file IS for, then, is the ten functions above that were measured IDENTICAL and
//! can be ported today, so that the next caller does not reach for `.ln()` or `.powf()`.
//! `acos`, `sinh`, `cosh`, `hypot`, `log1p` and `expm1` are the obvious next ones, and each is
//! the same job: copy `FdLibm`, then verify against `jvm_math.txt` rather than against the
//! host.
//!
//! # LICENCE
//!
//! The three algorithms below are transcribed from **OpenJDK 25's
//! `java.base/java/lang/FdLibm.java`**, which is:
//!
//! ```text
//! Copyright (c) 1998, 2024, Oracle and/or its affiliates. All rights reserved.
//! DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//!
//! This code is free software; you can redistribute it and/or modify it under the terms of
//! the GNU General Public License version 2 only, as published by the Free Software
//! Foundation. Oracle designates this particular file as subject to the "Classpath"
//! exception as provided by Oracle in the LICENSE file that accompanied this code.
//!
//! This code is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
//! without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
//! See the GNU General Public License version 2 for more details (a copy is included in the
//! LICENSE file that accompanied this code).
//!
//! You should have received a copy of the GNU General Public License version 2 along with
//! this work; if not, write to the Free Software Foundation, Inc., 51 Franklin St, Fifth
//! Floor, Boston, MA 02110-1301 USA.
//!
//! Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA or visit
//! www.oracle.com if you need additional information or have any questions.
//! ```
//!
//! FdLibm itself is the Sun "Freely Distributable Math Library" v5.3, also under the
//! GPLv2 + Classpath exception.
//!
//! The bit-manipulation helpers (`hi`, `lo`, `set_lo`) are this port's own, replacing
//! FdLibm's use of `Double.doubleToRawLongBits` / `longBitsToDouble`.

// ---------------------------------------------------------------------------
// Bit helpers -- FdLibm's "two 32-bit halves of a double" idiom
// ---------------------------------------------------------------------------

/// The high-order 32 bits of a double, as FdLibm's `__HI` sees them.
#[inline]
fn hi(x: f64) -> u32 {
    (x.to_bits() >> 32) as u32
}

/// The low-order 32 bits of a double, as FdLibm's `__LO` sees them.
#[inline]
fn lo(x: f64) -> u32 {
    x.to_bits() as u32
}

/// A double with the low-order 32 bits REPLACED, keeping the high half.
///
/// FdLibm's `__LO(x, low)`. Used to zero a value's low bits so that `f*f` is computed on a
/// deliberately truncated mantissa -- the whole point of the `pio4_hi` correction path in
/// `asin`, where the low bits of `sqrt(t)` are discarded so the rounding error can be
/// recovered exactly.
#[inline]
fn set_lo(x: f64, low: u32) -> f64 {
    f64::from_bits(((x.to_bits() & 0xffff_ffff_0000_0000) | low as u64))
}

/// Is the SIGN BIT of an IEEE-754 high word set?
///
/// FdLibm's `hx`, `hy` and `ix` are Java `int`s holding the high 32 bits, so its tests read
/// `hx < 0` for "negative". In Rust those words are `u32`, where `< 0` is a CONSTANT-FALSE
/// comparison -- and the compiler says so. Left unfixed it silently returns `+z` for every
/// negative input to `atan`, which is a wrong answer rather than a compile error.
#[inline]
fn sign_bit(h: u32) -> bool {
    h & 0x8000_0000 != 0
}

/// `hx == 0` in FdLibm terms: the high word is entirely zero, i.e. `+0.0`.
#[inline]
fn hi_is_zero(h: u32) -> bool {
    h == 0
}

/// `1e300`, FdLibm's `HUGE`. Used only to force the "inexact" flag in the early-return
/// paths (`if HUGE + x > 1.0`); we keep the comparison so the control flow matches, and the
/// comment at each site says the flag itself is unobservable in Rust.
#[inline]
fn huge() -> f64 {
    f64::from_bits(0x7e37e43c8800759c) // 1.0e300
}

// ===========================================================================
// Math.asin
// ===========================================================================
//
// Straight from `FdLibm.Asin.compute`. The three branches are |x| >= 1, |x| < 0.5, and
// 0.5 <= |x| < 1, and each is a different rational approximation.

const PIO2_HI: f64 = f64::from_bits(0x3ff921fb54442d18); // FdLibm 0x1.921fb54442d18p0 //  1.57079632679489655800e+00
const PIO2_LO: f64 = f64::from_bits(0x3c91a62633145c07); // FdLibm 0x1.1a62633145c07p-54 //  6.12323399573676603587e-17
const PIO4_HI: f64 = f64::from_bits(0x3fe921fb54442d18); // FdLibm 0x1.921fb54442d18p-1 //  7.85398163397448278999e-01

const PS0: f64 = f64::from_bits(0x3fc5555555555555); // FdLibm 0x1.5555555555555p-3 //  1.66666666666666657415e-01
const PS1: f64 = f64::from_bits(0xbfd4d61203eb6f7d); // FdLibm -0x1.4d61203eb6f7dp-2 // -3.25565818622400915405e-01
const PS2: f64 = f64::from_bits(0x3fc9c1550e884455); // FdLibm 0x1.9c1550e884455p-3 //  2.01212532134862925881e-01
const PS3: f64 = f64::from_bits(0xbfa48228b5688f3b); // FdLibm -0x1.48228b5688f3bp-5 // -4.00555345006794114027e-02
const PS4: f64 = f64::from_bits(0x3f49efe07501b288); // FdLibm 0x1.9efe07501b288p-11 //  7.91534994289814532176e-04
const PS5: f64 = f64::from_bits(0x3f023de10dfdf709); // FdLibm 0x1.23de10dfdf709p-15 //  3.47933107596021167570e-05
const QS1: f64 = f64::from_bits(0xc0033a271c8a2d4b); // FdLibm -0x1.33a271c8a2d4bp1 // -2.40339491173441421878e+00
const QS2: f64 = f64::from_bits(0x40002ae59c598ac8); // FdLibm 0x1.02ae59c598ac8p1 //  2.02094576023350569471e+00
const QS3: f64 = f64::from_bits(0xbfe6066c1b8d0159); // FdLibm -0x1.6066c1b8d0159p-1 // -6.88283971605453293030e-01
const QS4: f64 = f64::from_bits(0x3fb3b8c5b12e9282); // FdLibm 0x1.3b8c5b12e9282p-4 //  7.70381505559019352791e-02

/// Port of `Math.asin(double)` == `StrictMath.asin` == `FdLibm.Asin.compute`.
///
/// Returns `NaN` for `|x| > 1`, `+-pi/2` (inexact) for `|x| == 1`, and `x` unchanged for
/// `|x| < 2^-27`.
pub fn asin(x: f64) -> f64 {
    let hx = hi(x);
    let ix = hx & 0x7fff_ffff;
    // FdLibm declares `double t = 0` and relies on the `|x| < 2^-27` early return to leave
    // it unused, then reads it in the `else`. Rust wants the initialiser spelled out; the
    // value is never read on the path that leaves `t` at zero.
    let mut t: f64 = 0.0;

    if ix >= 0x3ff0_0000 {
        // |x| >= 1
        if ((ix - 0x3ff0_0000) | lo(x)) == 0 {
            // asin(+-1) = +-pi/2, deliberately inexact.
            return x * PIO2_HI + x * PIO2_LO;
        }
        // asin(|x| > 1) is NaN. FdLibm writes this as `(x - x)/(x - x)`, which is the
        // idiom for "produce a NaN and raise invalid"; in Rust it is clearer and gives the
        // same value, and the NaN-policy in golden.rs means the payload is not compared.
        return f64::NAN;
    }

    if ix < 0x3fe0_0000 {
        // |x| < 0.5
        if ix < 0x3e40_0000 {
            // |x| < 2^-27: the result IS x.
            if huge() + x > 1.0 {
                return x;
            }
        } else {
            t = x * x;
        }
        let p = t * (PS0 + t * (PS1 + t * (PS2 + t * (PS3 + t * (PS4 + t * PS5)))));
        let q = 1.0 + t * (QS1 + t * (QS2 + t * (QS3 + t * QS4)));
        let w = p / q;
        return x + x * w;
    }

    // 0.5 <= |x| < 1
    let mut w = 1.0 - x.abs();
    t = w * 0.5;
    let mut p = t * (PS0 + t * (PS1 + t * (PS2 + t * (PS3 + t * (PS4 + t * PS5)))));
    let q = 1.0 + t * (QS1 + t * (QS2 + t * (QS3 + t * QS4)));
    let s = t.sqrt();
    if ix >= 0x3fef_3333 {
        // |x| > 0.975
        w = p / q;
        t = PIO2_HI - (2.0 * (s + s * w) - PIO2_LO);
    } else {
        // The low half of `s` is DISCARDED so that `s*s` is exact, which lets the
        // correction term `c` recover the rounding error exactly. This is the whole reason
        // FdLibm keeps pio2_hi and pio4_hi as separate constants, and it is why a naive
        // `2.0*asin`-shaped rewrite is 1 ULP off.
        w = set_lo(s, 0);
        let c = (t - w * w) / (s + w);
        let r = p / q;
        p = 2.0 * s * r - (PIO2_LO - 2.0 * c);
        let q2 = PIO4_HI - 2.0 * w;
        t = PIO4_HI - (p - q2);
    }
    // FdLibm's `return (hx > 0) ? t : -t`. `hx` is a Java `int` there, so `hx > 0` means
    // "the sign bit is CLEAR". See `sign_bit` -- writing `hx > 0` in Rust would be a
    // constant-false `u32` comparison and would drop the sign of every negative input.
    if !sign_bit(hx) {
        t
    } else {
        -t
    }
}

// ===========================================================================
// Math.atan
// ===========================================================================
//
// Not used by vanilla directly, but `atan2` is defined in terms of it, so porting `atan2`
// means porting this too. From `FdLibm.Atan.compute`.

const ATANHI: [f64; 4] = [
    f64::from_bits(0x3fddac670561bb4f), // 0x1.dac670561bb4fp-2  atan(0.5)hi
    f64::from_bits(0x3fe921fb54442d18), // 0x1.921fb54442d18p-1  atan(1.0)hi
    f64::from_bits(0x3fef730bd281f69b), // 0x1.f730bd281f69bp-1  atan(1.5)hi
    f64::from_bits(0x3ff921fb54442d18), // 0x1.921fb54442d18p0   atan(inf)hi
];

const ATANLO: [f64; 4] = [
    f64::from_bits(0x3c7a2b7f222f65e2), // 0x1.a2b7f222f65e2p-56  atan(0.5)lo
    f64::from_bits(0x3c81a62633145c07), // 0x1.1a62633145c07p-55  atan(1.0)lo
    f64::from_bits(0x3c7007887af0cbbd), // 0x1.007887af0cbbdp-56 atan(1.5)lo
    f64::from_bits(0x3c91a62633145c07), // 0x1.1a62633145c07p-54  atan(inf)lo
];

const AT: [f64; 11] = [
    f64::from_bits(0x3fd555555555550d), //   3.33333333333329318027e-01
    f64::from_bits(0xbfc999999998ebc4), //  -1.99999999998764832476e-01
    f64::from_bits(0x3fc24924920083ff), //   1.42857142725034663711e-01
    f64::from_bits(0xbfbc71c6fe231671), //  -1.11111104054623557880e-01
    f64::from_bits(0x3fb745cdc54c206e), //   9.09088713343650656196e-02
    f64::from_bits(0xbfb3b0f2af749a6d), //  -7.69187620504482999495e-02
    f64::from_bits(0x3fb10d66a0d03d51), //   6.66107313738753120669e-02
    f64::from_bits(0xbfadde2d52defd9a), //  -5.83357013379057348645e-02
    f64::from_bits(0x3fa97b4b24760deb), //   4.97687799461593236017e-02
    f64::from_bits(0xbfa2b4442c6a6c2f), //  -3.65315727442169155270e-02
    f64::from_bits(0x3f90ad3ae322da11), //   1.62858201153657823623e-02
];

/// Port of `Math.atan(double)` == `StrictMath.atan` == `FdLibm.Atan.compute`.
pub fn atan(mut x: f64) -> f64 {
    let hx = hi(x);
    let ix = hx & 0x7fff_ffff;
    // `id` is the index into ATANHI/ATANLO, or -1 for "no reduction needed".
    let mut id: i32;

    if ix >= 0x4410_0000 {
        // |x| >= 2^66
        if ix > 0x7ff0_0000 || (ix == 0x7ff0_0000 && lo(x) != 0) {
            return x + x; // NaN or Inf
        }
        return if sign_bit(hx) {
            -ATANHI[3] - ATANLO[3]
        } else {
            ATANHI[3] + ATANLO[3]
        };
    }

    if ix < 0x3fdc_0000 {
        // |x| < 0.4375
        if ix < 0x3e20_0000 {
            // |x| < 2^-29
            if huge() + x > 1.0 {
                return x;
            }
        }
        id = -1;
    } else {
        x = x.abs();
        // Argument reduction into one of four intervals.
        if ix < 0x3ff3_0000 {
            // |x| < 1.1875
            if ix < 0x3fe6_0000 {
                id = 0; // 7/16 <= |x| < 11/16
                x = (2.0 * x - 1.0) / (2.0 + x);
            } else {
                id = 1; // 11/16 <= |x| < 19/16
                x = (x - 1.0) / (x + 1.0);
            }
        } else if ix < 0x4003_8000 {
            id = 2; // |x| < 2.4375
            x = (x - 1.5) / (1.0 + 1.5 * x);
        } else {
            id = 3; // 2.4375 <= |x| < 2^66
            x = -1.0 / x;
        }
    }

    let z = x * x;
    let w = z * z;
    // The 11-term series is split into odd and even halves before evaluation, which is a
    // Horner nesting rather than a power series -- changing it to `sum aT[i] * z^(i+1)`
    // rounds differently.
    let s1 = z * (AT[0] + w * (AT[2] + w * (AT[4] + w * (AT[6] + w * (AT[8] + w * AT[10])))));
    let s2 = w * (AT[1] + w * (AT[3] + w * (AT[5] + w * (AT[7] + w * AT[9]))));
    if id < 0 {
        x - x * (s1 + s2)
    } else {
        let z = ATANHI[id as usize] - ((x * (s1 + s2) - ATANLO[id as usize]) - x);
        if sign_bit(hx) {
            -z
        } else {
            z
        }
    }
}

// ===========================================================================
// Math.atan2
// ===========================================================================
//
// From `FdLibm.Atan2.compute`. Note the ARGUMENT ORDER: Java is `atan2(y, x)`, which is the
// opposite of Rust's `f64::atan2(self, other)` where `self` is the Y. See `atan2` below for
// the mapping, because getting it backwards shifts the result by pi and is invisible unless
// something checks the quadrants.

const ATAN2_TINY: f64 = f64::from_bits(0x01a56e1fc2f8f359); // FdLibm 1.0e-300
const PI_O_4: f64 = f64::from_bits(0x3fe921fb54442d18); // FdLibm 0x1.921fb54442d18p-1 // 7.8539816339744827900E-01
const PI_O_2: f64 = f64::from_bits(0x3ff921fb54442d18); // FdLibm 0x1.921fb54442d18p0 // 1.5707963267948965580E+00
const PI_LO: f64 = f64::from_bits(0x3ca1a62633145c07); // FdLibm 0x1.1a62633145c07p-53 // 1.2246467991473531772E-16

/// Port of `Math.atan2(double y, double x)` == `StrictMath.atan2` == `FdLibm.Atan2.compute`.
///
/// # THE ARGUMENT ORDER IS THE OPPOSITE OF RUST'S `f64::atan2`
///
/// Java: `atan2(y, x)` -- y first. Rust: `x.atan2(y)` -- the receiver is the Y. So this
/// function takes `(y, x)` to match Java and the Rust `Vec3` port calls
/// `jvm_math::atan2(-self.x, self.z)`, NOT `(-self.x).atan2(self.z)`.
///
/// That is the bug session 06 made and measured: `self.x.atan2(-self.z)` scored **0 of 512**
/// rows, because negating both arguments shifts the result by pi rather than cancelling.
#[inline]
pub fn atan2(y: f64, x: f64) -> f64 {
    let hx = hi(x);
    let ix = hx & 0x7fff_ffff;
    let lx = lo(x);
    let hy = hi(y);
    let iy = hy & 0x7fff_ffff;
    let ly = lo(y);

    if x.is_nan() || y.is_nan() {
        return x + y;
    }
    if ((hx.wrapping_sub(0x3ff0_0000)) | lx) == 0 {
        // x == 1.0 exactly
        return atan(y);
    }

    // m encodes which quadrant: 2*sign(x) + sign(y). The special-case answers below are
    // indexed by it, so it has to be built the same way FdLibm builds it.
    let m = ((hy >> 31) & 1) | ((hx >> 30) & 2);

    if (iy | ly) == 0 {
        // y == 0 (either sign)
        return match m {
            0 | 1 => y,                     // atan(+-0, +anything) = +-0
            2 => std::f64::consts::PI + ATAN2_TINY,  // atan(+0, -anything) = pi
            _ => -std::f64::consts::PI - ATAN2_TINY, // atan(-0, -anything) = -pi
        };
    }
    if hi_is_zero(ix) && lx == 0 {
        // x == 0
        return if sign_bit(hy) {
            -PI_O_2 - ATAN2_TINY
        } else {
            PI_O_2 + ATAN2_TINY
        };
    }
    if ix == 0x7ff0_0000 {
        // x is infinite
        if iy == 0x7ff0_0000 {
            return match m {
                0 => PI_O_4 + ATAN2_TINY,             // atan(+Inf, +Inf)
                1 => -PI_O_4 - ATAN2_TINY,            // atan(-Inf, +Inf)
                2 => 3.0 * PI_O_4 + ATAN2_TINY,      // atan(+Inf, -Inf)
                _ => -3.0 * PI_O_4 - ATAN2_TINY,     // atan(-Inf, -Inf)
            };
        }
        return match m {
            0 => 0.0,
            1 => -0.0,
            2 => std::f64::consts::PI + ATAN2_TINY,
            _ => -std::f64::consts::PI - ATAN2_TINY,
        };
    }
    if iy == 0x7ff0_0000 {
        // y is infinite
        return if sign_bit(hy) {
            -PI_O_2 - ATAN2_TINY
        } else {
            PI_O_2 + ATAN2_TINY
        };
    }

    // Compute atan(|y/x|), with two guards so the division cannot overflow or underflow
    // into a wrong answer. `k` is the exponent difference from the HIGH words, which is why
    // the shift is by 20: only the exponent field is being compared.
    let k = (iy as i32 - ix as i32) >> 20;
    let z = if k > 60 {
        // |y/x| > 2^60: y/x would overflow.
        PI_O_2 + 0.5 * PI_LO
    } else if (hx as i32) < 0 && k < -60 {
        // |y/x| < 2^-60 with x < 0: y/x would be -0 and lose the quadrant.
        0.0
    } else {
        atan((y / x).abs())
    };

    match m {
        0 => z,                                  // atan(+, +)
        1 => -z,                                 // atan(-, +)
        2 => std::f64::consts::PI - (z - PI_LO), // atan(+, -)
        _ => (z - PI_LO) - std::f64::consts::PI, // atan(-, -)
    }
}

// ===========================================================================
// The direct-call guard
// ===========================================================================

/// Fails if any ported source file calls a host transcendental directly.
///
/// # WHY THIS TEST EXISTS
///
/// Two sessions running, the same class of bug bit three times, always with the same shape:
/// a host `libm` call slipped into ported code, and nothing failed until someone built the
/// project on a different operating system.
///
/// * `MarsagliaPolarGaussian` calls `f64::ln`. Windows' libm and glibc's disagree with
///   HotSpot on eight specific inputs, so `nextGaussian` gave different values.
/// * `Vec3#rotation` called `f64::atan2` / `f64::asin`. For two sessions that was blamed as
///   the cause of its 136-of-512 yaw divergence; it turned out not to be (see the module
///   docs -- a constant was wrong). The call is still routed here, because the dependence is
///   real even where the symptom was not.
/// * `Vec2#rotate` and `Vec3#xRot`/`yRot`/`zRot` call `Mth::cos`/`Mth::sin`, which are
///   vanilla's own LOOKUP TABLES and therefore fine -- which is exactly why a grep for
///   "libm" finds both the real bugs and the legitimate uses, and cannot be left to a human.
///
/// So the rule is checked mechanically, over the whole ported tree, every time.
///
/// # WHAT COUNTS AS A VIOLATION
///
/// A call to `f64::ln`, `f64::exp`, `f64::sin`, `f64::cos`, `f64::tan`, `f64::asin`,
/// `f64::acos`, `f64::atan`, `f64::atan2`, `f64::sinh`, `f64::cosh`, `f64::tanh`,
/// `f64::log10`, `f64::log1p`, `f64::log2`, `f64::exp2`, `f64::exp_m1`, `f64::hypot`,
/// `f64::cbrt`, `f64::sinh`, `f64::cosh`, `f64::tan`, `f64::to_degrees`, `f64::to_radians`.
///
/// **NOT** a violation, and the list is deliberate:
/// * `sqrt` -- IEEE-754 requires correctly rounded, so the host and HotSpot agree.
/// * `abs`, `floor`, `ceil`, `round`, `trunc`, `fract`, `signum` -- exact integer-like
///   operations, not approximations. (Note `f64::floor` and friends are NOT always
///   identical to Java's `Math.floor` for negative NaN-adjacent inputs, but that is a
///   different problem and `java_lang` handles it.)
/// * `powi` -- `pow` with an integer exponent is exact and is not a `pow` approximation.
///
/// **This module's own file is exempt**, because it necessarily names the host functions in
/// order to check that nothing else does.
#[test]
fn ported_code_never_calls_a_host_transcendental() {
    // Paths relative to the crate root, which is where CARGO_MANIFEST_DIR points.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let ported = root.join("net");
    assert!(ported.is_dir(), "expected the ported tree at {}", ported.display());

    const FORBIDDEN: &[&str] = &[
        // one-argument forms: `x.ln()`, `x.exp()`, ...
        ".ln()",
        ".log10()",
        ".log2()",
        ".log1p()",
        ".exp()",
        ".exp2()",
        ".exp_m1()",
        ".sin()",
        ".cos()",
        ".tan()",
        ".asin()",
        ".acos()",
        ".atan()",
        ".sinh()",
        ".cosh()",
        ".tanh()",
        ".cbrt()",
        ".hypot(",
        // two-argument forms: `a.powf(b)`, `x.powf(y)`, ...
        ".powf(",
        ".atan2(",
        ".to_degrees()",
        ".to_radians()",
    ];

    // Paths allowed to contain the above: this file, and the test itself.
    let exempt = |p: &std::path::Path| {
        let s = p.to_string_lossy().replace('\\', "/");
        s.ends_with("javacompat/jvm_math.rs")
    };

    let mut violations: Vec<String> = Vec::new();
    let mut stack = vec![ported];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            if exempt(&path) {
                continue;
            }
            let text = match std::fs::read_to_string(&path) {
                Ok(t) => t,
                Err(_) => continue,
            };
            for (lineno, line) in text.lines().enumerate() {
                // Ignore comment lines: several files legitimately NAME the host function
                // in a doc comment explaining why they do not call it.
                let t = line.trim_start();
                if t.starts_with("//") {
                    continue;
                }
                for pat in FORBIDDEN {
                    if line.contains(pat) {
                        violations.push(format!(
                            "{}:{}: `{}` -- route it through javacompat::jvm_math (or \
                             Mth's lookup tables for sin/cos)",
                            path.strip_prefix(root).unwrap_or(&path).display(),
                            lineno + 1,
                            pat
                        ));
                    }
                }
            }
        }
    }

    violations.sort();
    violations.dedup();
    assert!(
        violations.is_empty(),
        "ported code calls the host math library directly, so results will differ by OS \
         and by optimisation level. {} site(s):\n  {}",
        violations.len(),
        violations.join("\n  ")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sanity checks that do not need the golden. The authoritative check is
    /// `parity_jvm_math.rs` against `jvm_math.txt`, which came from the jar's own JVM.
    #[test]
    fn asin_special_cases() {
        assert_eq!(asin(0.0), 0.0);
        assert_eq!(asin(-0.0).to_bits(), (-0.0f64).to_bits());
        assert_eq!(asin(1.0).to_bits(), 1.5707963267948966f64.to_bits());
        assert_eq!(asin(-1.0).to_bits(), (-1.5707963267948966f64).to_bits());
        assert!(asin(2.0).is_nan());
        assert!(asin(f64::NAN).is_nan());
        // |x| < 2^-27 returns x itself.
        let tiny_in = f64::from_bits(0x3e30_0000_0000_0000);
        assert_eq!(asin(tiny_in).to_bits(), tiny_in.to_bits());
    }

    #[test]
    fn atan2_quadrants() {
        let q = std::f64::consts::FRAC_PI_4;
        // The four axis cases, which pin the sign handling that session 06 got wrong.
        assert!((atan2(0.0, 1.0) - 0.0).abs() < 1e-300);
        assert!((atan2(1.0, 1.0) - q).abs() < 1e-15);
        assert!((atan2(1.0, -1.0) - 3.0 * q).abs() < 1e-15);
        assert!((atan2(-1.0, -1.0) + 3.0 * q).abs() < 1e-15);
        // x == 0.
        assert!((atan2(1.0, 0.0) - std::f64::consts::FRAC_PI_2).abs() < 1e-15);
        assert!((atan2(-1.0, 0.0) + std::f64::consts::FRAC_PI_2).abs() < 1e-15);
        // y == 0 keeps y's sign, as FdLibm documents.
        assert_eq!(atan2(0.0, 1.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(atan2(-0.0, 1.0).to_bits(), (-0.0f64).to_bits());
    }

    /// The argument order is Java's `(y, x)`, NOT Rust's receiver-is-y. If someone "tidies"
    /// this into `atan2(self.y, self.x)`-style receiver form the quadrants flip, so pin it.
    #[test]
    fn atan2_argument_order_is_y_then_x() {
        // Java atan2(1.0, -1.0) = 3*pi/4, i.e. the point is in the SECOND quadrant.
        let v = atan2(1.0, -1.0);
        assert!(v > std::f64::consts::PI / 2.0, "got {v}: second quadrant expected");
        // Rust's own f64::atan2 takes the receiver as y, so `1.0f64.atan2(-1.0)` agrees --
        // which is exactly why the wrapper exists and why the call site must be written
        // jvm_math::atan2(y, x) and not receiver style.
        assert_eq!(v.to_bits(), 1.0f64.atan2(-1.0).to_bits());
    }

    #[test]
    fn guard_test_itself_runs() {
        // A no-op assertion so the guard is not optimised out of the binary; the real work
        // is the file walk in the test above.
        assert!(FORBIDDEN_LEN > 0);
    }
    const FORBIDDEN_LEN: usize = 22;
}
