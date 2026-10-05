// ============================================================================
// TRANSCENDENTAL SURVEY -- not game code, and not committed.
// ============================================================================
//
// Answers one question per function: does `Math.f` agree with `StrictMath.f` on this JVM?
//
// In JDK 21+ most `Math` methods are one-line delegations to `StrictMath`, which is
// `FdLibm.java` -- pure Java, fully deterministic, portable. A few are HotSpot intrinsics
// (`_dlog`, `_dexp`, `_dpow`, `_dsin`, `_dcos`, `_dtan` on x86-64), and those disagree with
// FdLibm. That distinction decides whether a function can be ported from FdLibm and be
// correct EVERYWHERE, or needs the platform-specific stub.
//
// Method: for each function, sweep a corpus, compare `Math.f(x)` against
// `StrictMath.f(x)` bit for bit, and report the disagreement count and the largest ULP
// gap. Also print a handful of concrete disagreements so a bug in the comparison itself
// would be visible rather than showing up as "0 mismatches, mysteriously".
package p;

import java.util.LinkedHashMap;
import java.util.Map;

public class Survey {

    /** Bit patterns covering every class of double that tends to separate implementations. */
    static final long[] CORPUS = build();

    static long[] build() {
        double[] seeds = {
            0.0, -0.0, 1.0, -1.0, 0.5, -0.5, 1.5, -1.5, 2.0, -2.0, 3.0, -3.0,
            0.1, -0.1, 0.25, -0.25, 1e-8, -1e-8, 1e-3, -1e-3, 1e3, -1e3,
            1e-30, -1e-30, 1e30, -1e30, 1e300, -1e300,
            Math.PI, -Math.PI, Math.PI / 2, -Math.PI / 2, Math.PI / 4,
            0.9999999999999999, 1.0000000000000002, 0.49999999999999994,
            Double.MIN_VALUE, -Double.MIN_VALUE, Double.MIN_NORMAL, -Double.MIN_NORMAL,
            Double.MAX_VALUE, -Double.MAX_VALUE, Math.ulp(1.0), -Math.ulp(1.0),
            Math.nextUp(1.0), Math.nextDown(1.0), Math.nextUp(0.0), Math.nextDown(0.0),
            Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, Double.NaN,
        };
        java.util.List<Long> out = new java.util.ArrayList<>();
        for (double d : seeds) {
            out.add(Double.doubleToRawLongBits(d));
        }
        // A deterministic pseudo-random sweep, so the corpus is not just the hand-picked
        // values above. LCG, so it is reproducible without a Random seed argument.
        long s = 0x2545F4914F6CDD1DL;
        for (int i = 0; i < 4000; i++) {
            s = s * 6364136223846793005L + 1442695040888963407L;
            out.add(s);
            out.add(s >>> 1);           // smaller exponents too
            out.add(s | Long.MIN_VALUE); // negative
        }
        long[] a = new long[out.size()];
        for (int i = 0; i < a.length; i++) {
            a[i] = out.get(i);
        }
        return a;
    }

    interface F2 {
        double apply(double a, double b);
    }

    interface F1 {
        double apply(double a);
    }

    static void one(String name, F1 math, F1 strict) {
        int mismatches = 0;
        long worstUlp = 0;
        String firstBad = null;
        for (long bits : CORPUS) {
            double x = Double.longBitsToDouble(bits);
            double a = math.apply(x);
            double b = strict.apply(x);
            long ab = Double.doubleToRawLongBits(a);
            long bb = Double.doubleToRawLongBits(b);
            if (ab != bb) {
                mismatches++;
                long gap = ulpGap(a, b);
                if (gap > worstUlp) {
                    worstUlp = gap;
                }
                if (firstBad == null) {
                    firstBad = String.format("x=%s Math=%s Strict=%s",
                            trim(x), trim(a), trim(b));
                }
            }
        }
        System.out.printf("%-12s %6d / %d mismatch   worstULP=%-4d  %s%n",
                name, mismatches, CORPUS.length, worstUlp,
                mismatches == 0 ? "IDENTICAL -> FdLibm port" : "DIFFERS -> intrinsic");
        if (firstBad != null) {
            System.out.println("             first: " + firstBad);
        }
    }

    static void two(String name, F2 math, F2 strict) {
        int mismatches = 0;
        long worstUlp = 0;
        String firstBad = null;
        for (long ba : CORPUS) {
            for (long bb : CORPUS) {
                double a = Double.longBitsToDouble(ba);
                double b = Double.longBitsToDouble(bb);
                long ab = Double.doubleToRawLongBits(math.apply(a, b));
                long bs = Double.doubleToRawLongBits(strict.apply(a, b));
                if (ab != bs) {
                    mismatches++;
                    long gap = ulpGap(math.apply(a, b), strict.apply(a, b));
                    if (gap > worstUlp) {
                        worstUlp = gap;
                    }
                    if (firstBad == null) {
                        firstBad = String.format("(%s,%s)", trim(a), trim(b));
                    }
                }
            }
        }
        System.out.printf("%-12s %6d / %d mismatch   worstULP=%-4d  %s%n",
                name, mismatches, CORPUS.length * CORPUS.length, worstUlp,
                mismatches == 0 ? "IDENTICAL -> FdLibm port" : "DIFFERS -> intrinsic");
        if (firstBad != null) {
            System.out.println("             first at: " + firstBad);
        }
    }

    /** Signed-magnitude ULP distance between two doubles, NaN-aware (NaN vs NaN = 0). */
    static long ulpGap(double a, double b) {
        if (Double.isNaN(a) && Double.isNaN(b)) {
            return 0;
        }
        if (Double.isNaN(a) || Double.isNaN(b)) {
            return Long.MAX_VALUE / 4;
        }
        return Math.abs(ordered(a) - ordered(b));
    }

    static long ordered(double d) {
        long b = Double.doubleToRawLongBits(d);
        return b < 0 ? Long.MIN_VALUE - b : b; // maps -Inf..+Inf onto a monotone range
    }

    static String trim(double d) {
        if (Double.isNaN(d)) {
            return "NaN";
        }
        if (Double.isInfinite(d)) {
            return d > 0 ? "Inf" : "-Inf";
        }
        return String.format("%s(0x%016x)", d, Double.doubleToRawLongBits(d));
    }

    public static void main(String[] args) {
        System.out.println("java.version    = " + System.getProperty("java.version"));
        System.out.println("java.vendor     = " + System.getProperty("java.vendor"));
        System.out.println("java.vm.name    = " + System.getProperty("java.vm.name"));
        System.out.println("os.name         = " + System.getProperty("os.name"));
        System.out.println("os.arch         = " + System.getProperty("os.arch"));
        System.out.println("corpus size     = " + CORPUS.length);
        System.out.println();
        System.out.printf("%-12s %s%n", "FUNCTION", "Math vs StrictMath over the corpus");
        System.out.println("=".repeat(100));

        one("sqrt", Math::sqrt, StrictMath::sqrt);
        one("cbrt", Math::cbrt, StrictMath::cbrt);
        one("log", Math::log, StrictMath::log);
        one("log10", Math::log10, StrictMath::log10);
        one("log1p", Math::log1p, StrictMath::log1p);
        one("exp", Math::exp, StrictMath::exp);
        one("expm1", Math::expm1, StrictMath::expm1);
        one("sin", Math::sin, StrictMath::sin);
        one("cos", Math::cos, StrictMath::cos);
        one("tan", Math::tan, StrictMath::tan);
        one("asin", Math::asin, StrictMath::asin);
        one("acos", Math::acos, StrictMath::acos);
        one("atan", Math::atan, StrictMath::atan);
        one("sinh", Math::sinh, StrictMath::sinh);
        one("cosh", Math::cosh, StrictMath::cosh);
        one("tanh", Math::tanh, StrictMath::tanh);
        two("hypot", Math::hypot, StrictMath::hypot);
        one("floor", Math::floor, StrictMath::floor);
        one("ceil", Math::ceil, StrictMath::ceil);
        one("rint", Math::rint, StrictMath::rint);
        two("atan2", Math::atan2, StrictMath::atan2);
        two("pow", Math::pow, StrictMath::pow);
    }
}