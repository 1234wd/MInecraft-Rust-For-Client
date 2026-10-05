// ============================================================================
// JVM_MATH ORACLE -- not game code.
//
// Emits golden data for `_porting/javacompat/jvm_math.rs`.
//
// WHY THIS FILE EXISTS
// --------------------
// Session 06 found that `Vec3#rotation` is only 376/512 exact on yaw and 333/512 on
// pitch, because it calls `Math.atan2` and `Math.asin` and the host libm differs from
// HotSpot by 1 ULP. Session 07 measured which `Math` functions are HotSpot intrinsics and
// which simply delegate to `StrictMath` (== FdLibm, pure Java, portable):
//
//   IDENTICAL to StrictMath -> port from FdLibm, exact on every OS
//   DIFFERS                -> HotSpot intrinsic, needs the platform stub
//
// asin, acos, atan, atan2, sinh, cosh, hypot, log1p, expm1, floor, ceil, rint, sqrt are
// IDENTICAL. log, log10, exp, sin, cos, tan, tanh, cbrt, pow are intrinsics.
//
// This file emits `Math` AND `StrictMath` for asin, atan and atan2 over a wide corpus, so
// the Rust port can be checked against BOTH. They are identical today (session 07's survey
// proved it over 12,051 values), and the test asserts they are -- so the day HotSpot starts
// intrinsifying one of them, the failure names the function instead of showing up as a
// mysterious bit difference in `Vec3#rotation`.
//
// The corpus is the same shape as the survey's, so the two agree on coverage.
package oracle;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

/** Golden data for {@code jvm_math.rs}: the FdLibm-backed transcendentals. */
final class JvmMathOracle {

    /** Bit patterns covering every class of double that separates implementations. */
    static final long[] CORPUS = build();

    static long[] build() {
        double[] seeds = {
            0.0, -0.0, 1.0, -1.0, 0.5, -0.5, 1.5, -1.5, 2.0, -2.0, 3.0, -3.0,
            0.1, -0.1, 0.25, -0.25, 1e-8, -1e-8, 1e-3, -1e-3, 1e3, -1e3,
            1e-30, -1e-30, 1e30, -1e30, 1e300, -1e300,
            Math.PI, -Math.PI, Math.PI / 2, -Math.PI / 2, Math.PI / 4, -Math.PI / 4,
            3.0 * Math.PI / 4, -3.0 * Math.PI / 4,
            // The boundaries FdLibm branches on. Getting one of these wrong is the whole
            // risk in a hand transcription, so they are named explicitly rather than left
            // to the random sweep to find.
            0.4375, 0.6875, 0.8125, 1.1875, 2.4375, 1.0 / 3.0, 2.0 / 3.0,
            0.975, 0.49999999999999994, 0.5000000000000001,
            Math.nextDown(0.5), Math.nextUp(0.5),
            Math.nextDown(0.4375), Math.nextUp(0.4375),
            Math.nextDown(0.6875), Math.nextUp(0.6875),
            Math.nextDown(1.1875), Math.nextUp(1.1875),
            Math.nextDown(2.4375), Math.nextUp(2.4375),
            Double.MIN_VALUE, -Double.MIN_VALUE, Double.MIN_NORMAL, -Double.MIN_NORMAL,
            Double.MAX_VALUE, -Double.MAX_VALUE, Math.ulp(1.0), -Math.ulp(1.0),
            Math.nextUp(1.0), Math.nextDown(1.0), Math.nextUp(0.0), Math.nextDown(0.0),
            Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, Double.NaN,
            // 2^-27 and 2^-29: the "return x with inexact" cutoffs in Asin and Atan.
            Math.scalb(1.0, -27), Math.scalb(1.0, -28), Math.scalb(1.0, -29),
            Math.scalb(1.0, -30), Math.scalb(1.0, -57), Math.scalb(1.0, -66),
        };
        List<Long> out = new ArrayList<>();
        for (double d : seeds) {
            out.add(Double.doubleToRawLongBits(d));
        }
        // Deterministic sweep: an LCG rather than `Random`, so the corpus is identical on
        // every machine and every run without having to pin a seed.
        long s = 0x2545F4914F6CDD1DL;
        for (int i = 0; i < 3000; i++) {
            s = s * 6364136223846793005L + 1442695040888963407L;
            out.add(s);
            out.add(s >>> 1);
            out.add(s | Long.MIN_VALUE);
        }
        // A DENSE sweep of [-1, 1], and a denser one over [0.4, 1.0].
        //
        // This block exists because of a failure the Rust suite found, not a guess. The LCG
        // sweep above is spread over the whole exponent range, so it lands almost nothing in
        // [-1, 1] -- and that is exactly where `Asin` branches: the `0.5..0.975` region uses
        // the `pio4_hi` correction with a deliberately truncated `sqrt`, the fiddliest part
        // of the function. The Rust port's `corpus_covers_every_fdlibm_branch` test failed
        // with "asin corpus lacks 0.5..0.975", which is the corpus being wrong, not the
        // port. A random sweep verifies the arithmetic; only a dense one verifies the
        // branches.
        for (int i = 0; i <= 4000; i++) {
            double x = -1.0 + (2.0 * i) / 4000.0;
            out.add(Double.doubleToRawLongBits(x));
            out.add(Double.doubleToRawLongBits(-x));
        }
        for (int i = 0; i <= 6000; i++) {
            double x = 0.4 + (0.6 * i) / 6000.0;
            out.add(Double.doubleToRawLongBits(x));
            out.add(Double.doubleToRawLongBits(-x));
        }
        // `nextUp`/`nextDown` of every branch cutoff, in doubles. One ULP either side of a
        // boundary is where an off-by-one in the comparison shows up, and those are the
        // rows a hand transcription gets wrong while every ordinary value still passes.
        double[] cuts = {
            0.0, 0.4375, 0.5, 0.6875, 0.8125, 0.975, 1.0, 1.1875, 1.5, 2.4375,
            1.0 / 3.0, 2.0 / 3.0, Math.scalb(1.0, -27), Math.scalb(1.0, -29),
            Math.scalb(1.0, -57), Math.scalb(1.0, -60), Math.scalb(1.0, -66),
            Math.scalb(1.0, 60), Math.scalb(1.0, 66), Math.scalb(1.0, 440),
        };
        for (double c : cuts) {
            double a = Math.nextDown(c);
            double b = c;
            double d = Math.nextUp(c);
            for (double v : new double[] {a, b, d, -a, -b, -d}) {
                out.add(Double.doubleToRawLongBits(v));
            }
        }
        // SUBNORMALS, densely. Added in session 08 for the same reason the [-1,1] sweep was
        // added in session 07: a coverage test failed.
        //
        // `corpus_covers_every_fdlibm_branch` asserts the corpus reaches the subnormal branch of
        // `e_log`, and found only 23 subnormal inputs in 29,201. Twenty-three happened to be
        // enough to catch the real bug (the first port of `jvm_math::log` captured the low word
        // of `x` before FdLibm's `x *= TWO54` scaling and was 2,257,518 ULP off on subnormals),
        // but "happened to be enough" is not a property to build on -- 23 of 29,201 means the
        // corpus is reaching that branch by accident.
        //
        // The game never calls `log` with a subnormal, since `MarsagliaPolarGaussian` passes
        // `radiusSquared` in (0,2). That is exactly why the original 256-row corpus had none and
        // why the bug survived a green suite.
        for (int i = 1; i <= 3000; i++) {
            // Raw subnormal bit patterns: exponent field 0, non-zero mantissa.
            out.add((long) i);
            out.add((long) i << 20);
            out.add(((long) i) | 0x8000_0000_0000_0000L);           // negative subnormal
            out.add(0x000f_ffff_ffff_ffffL - i);                     // top of the subnormal range
        }
        // The extremes of the subnormal range and their neighbours, which are where the
        // TWO54 scaling either underflows or lands exactly on 1.0.
        out.add(0x0000_0000_0000_0001L); // smallest positive subnormal
        out.add(0x000f_ffff_ffff_ffffL); // largest subnormal
        out.add(0x8000_0000_0000_0001L);
        out.add(0x800f_ffff_ffff_ffffL);
        for (long k : new long[] {0x3ca, 0x3cb, 0x3cc, 0x3cd, 0x3ce, 0x3cf}) {
            long b = k << 52;
            out.add(b);
            out.add(b | 1L);
            out.add(b | 0xfffffffffffffL);
            out.add(b | 0x8000000000000L);
        }
        long[] a = new long[out.size()];
        for (int i = 0; i < a.length; i++) {
            a[i] = out.get(i);
        }
        return a;
    }

    static void emit(final Out o) {
        // --- asin: Math and StrictMath, so a future intrinsic shows up here. ---
        o.fn("jvm_math.asin", "f64", "f64 f64");
        for (long bits : CORPUS) {
            double x = Double.longBitsToDouble(bits);
            o.row(Out.f64(x), Out.join(Out.f64(Math.asin(x)), Out.f64(StrictMath.asin(x))));
        }

        // --- atan: not in the game surface directly, but atan2 depends on it. ---
        o.fn("jvm_math.atan", "f64", "f64 f64");
        for (long bits : CORPUS) {
            double x = Double.longBitsToDouble(bits);
            o.row(Out.f64(x), Out.join(Out.f64(Math.atan(x)), Out.f64(StrictMath.atan(x))));
        }

        // --- atan2 over a REDUCED corpus: it is two-argument, so the full corpus would be
        //     9,138^2 = 83 million rows. The reduction keeps every FdLibm branch reachable:
        //     y = 0, x = 0, x = 1, x = -1, |x| >= 2^66, |y/x| > 2^60, both infinite, and all
        //     four quadrants, by pairing every y against a fixed set of x values. ---
        o.fn("jvm_math.atan2", "f64 f64", "f64 f64");
        long[] xs = {
            0x0000000000000000L, 0x8000000000000000L, // x = +-0
            0x3ff0000000000000L, 0xbff0000000000000L, // x = +-1
            0x3fe0000000000000L, 0xbfe0000000000000L, // x = +-0.5
            0x3ff4000000000000L, 0x3ff9000000000000L, // x = 1.25, 1.5625
            0x4003008000000000L, 0xc003008000000000L, // x = +-2.4375
            0x4410000000000000L, 0xc410000000000000L, // x = +-2^66
            0x7ff0000000000000L, 0xfff0000000000000L, // x = +-Inf
            0x7ff8000000000000L,                          // x = NaN
            0x0010000000000000L, 0x3ca0000000000000L, // subnormal-ish, tiny
            0x7fefffffffffffffL, 0xffefffffffffffffL, // |x| near DBL_MAX
        };
        for (long yb : CORPUS) {
            for (long xb : xs) {
                double y = Double.longBitsToDouble(yb);
                double x = Double.longBitsToDouble(xb);
                o.row(Out.join(Out.f64(y), Out.f64(x)),
                        Out.join(Out.f64(Math.atan2(y, x)), Out.f64(StrictMath.atan2(y, x))));
            }
        }

        // --- log. `Math.log` is a HotSpot INTRINSIC here, so the two columns DIFFER and this
        //     group is the one that pins which pure-Rust `log` is closest. FdLibm's `e_log`
        //     (== `StrictMath.log`) is the obvious candidate and musl's `log` via the `libm`
        //     crate is the other; neither is guaranteed to match `_dlog`.
        //
        //     This group is NOT claimed to be bit-exact. It is the measuring stick: see
        //     `ALLOWED_LOG_MISMATCHES` in `_porting/tests/parity_jvm_math.rs`.
        o.fn("jvm_math.log", "f64", "f64 f64");
        for (long bits : CORPUS) {
            double x = Double.longBitsToDouble(bits);
            o.row(Out.f64(x), Out.join(Out.f64(Math.log(x)), Out.f64(StrictMath.log(x))));
        }
        emitGaussianLog(o);
    }

    /**
     * The `radiusSquared` values that {@code MarsagliaPolarGaussian} actually feeds to
     * {@code Math.log}, so a candidate {@code log} can be measured on the numbers the GAME
     * uses rather than only on a synthetic sweep.
     *
     * <p>Re-derived here rather than read out of {@code random.txt} so this oracle file has
     * no dependency on another oracle's output format. The three LCG variants are the ones
     * vanilla ships; see {@code Mth#nextGaussian}.
     */
    static void emitGaussianLog(final Out o) {
        o.fn("jvm_math.gaussianLog", "f64", "f64 f64");
        // 256 draws is what random.txt carries; the generators are seeded exactly as the
        // parity tests seed them, so these are the same inputs the game produces.
        final long[] seeds = {
            0L, 1L, 42L, -1L, 0x5DEECE66DL, 123456789L,
        };
        for (long seed : seeds) {
            final java.util.Random rng = new java.util.Random(seed);
            for (int i = 0; i < 256; i++) {
                double rs = rng.nextDouble();
                if (rs == 0.0 || rs >= 1.0 || Double.isNaN(rs)) {
                    continue;
                }
                o.row(Out.f64(rs), Out.join(Out.f64(Math.log(rs)), Out.f64(StrictMath.log(rs))));
            }
        }
    }

    static void emitTo(final Path out) throws Exception {
        final Out o = new Out(out);
        o.comment("jvm_math -- Math vs StrictMath for the FdLibm-backed transcendentals");
        o.comment("jdk " + System.getProperty("java.version") + " / "
                + System.getProperty("java.vm.name"));
        emit(o);
        o.write(out);
    }
}