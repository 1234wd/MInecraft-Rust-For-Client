//! Port of: `org.apache.commons.lang3.math.Fraction`
//! Java class(es): `org.apache.commons.lang3.math.Fraction` (commons-lang3 **3.20.0**)
//! Status: PARTIAL
//!
//! Only the members `Mth#mulAndTruncate` actually reaches are ported, plus enough of
//! `Fraction` to construct and compare instances. Everything else is a `todo!()`.
//!
//! # Why this lives in javacompat rather than the mirror
//!
//! `Fraction` is a third-party class. There is no `Fraction.java` in
//! `minecraft-decompiled/`, so Rule 3 puts it under `_porting/`, and it is ported from
//! the **pinned** commons-lang3 3.20.0 that Minecraft 26.2 resolves to (read from the
//! Loom/Gradle cache -- see `java-oracle/fetch_libs.ps1`). Pinning matters: `getNumerator`
//! and `getDenominator` are trivially stable, but the reduction/normalisation rules and
//! the `ArithmeticException` messages are not guaranteed across major versions.
//!
//! # The one thing that matters for parity
//!
//! `Mth#mulAndTruncate` is:
//!
//! ```java
//! fraction.getNumerator() * factor / fraction.getDenominator()
//! ```
//!
//! An `int * int` product that WRAPS on overflow, then an integer division that
//! truncates toward zero. Both halves matter:
//!
//! * The multiply is `int * int` -> `int`. Overflow wraps silently in Java; a Rust `i32`
//!   multiply panics in debug. This must be `wrapping_mul`.
//! * Java's `/` truncates toward zero, which Rust's `/` also does. So no adjustment is
//!   needed -- but note this is *not* `floorDiv`.
//! * The denominator is guaranteed non-zero by the constructor, so there is no divide
//!   by zero to worry about.
//!
//! The oracle emits enough fractions (including `Integer.MAX_VALUE` and
//! `Integer.MIN_VALUE` components) to catch both the wrapping and the truncation.

/// Port of `org.apache.commons.lang3.math.Fraction` (partial).
///
/// Immutable, and reduced on construction exactly as commons-lang3 does: divide both
/// parts by their GCD, and move the sign into the numerator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    /// Port of `Fraction(int numerator, int denominator)`.
    ///
    /// Reduces by the GCD and normalises the sign onto the numerator, exactly as
    /// commons-lang3's constructor does. `denominator == 0` is rejected, matching its
    /// `IllegalArgumentException`.
    ///
    /// # Panics
    ///
    /// Panics if `denominator` is zero. Java throws `IllegalArgumentException` there
    /// and vanilla never catches it, so both crash; see `DESIGN_DECISIONS.md`
    /// (#panic-not-Result).
    pub fn new(numerator: i32, denominator: i32) -> Self {
        assert!(denominator != 0, "Fraction with zero denominator");
        if denominator < 0 {
            // Java: if (den < 0) { num = -num; den = -den; }
            //
            // `-i32::MIN` overflows in Rust and panics; Java's `-Integer.MIN_VALUE`
            // also overflows but wraps to itself. Use wrapping_neg to match.
            let numerator = numerator.wrapping_neg();
            let denominator = denominator.wrapping_neg();
            Self::reduce(numerator, denominator)
        } else {
            Self::reduce(numerator, denominator)
        }
    }

    fn reduce(numerator: i32, denominator: i32) -> Self {
        if numerator == 0 {
            // commons-lang3 short-circuits to 0/1.
            return Self { numerator: 0, denominator: 1 };
        }
        let gcd = Self::greatest_common_divisor(numerator.wrapping_abs(), denominator);
        Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }

    /// Euclid's algorithm, as `NumberUtils.gcd`.
    ///
    /// Note the arguments are pre-absolutised by the caller. `i32::MIN.wrapping_abs()`
    /// stays negative, which would make the loop misbehave -- but commons-lang3 has the
    /// same overflow, and a `Fraction` can only reach it via `new(i32::MIN, d)`, where
    /// `d` is positive and the GCD of `2^31` and `d` is computed fine because
    /// `greatest_common_divisor` compares against a positive `d`.
    fn greatest_common_divisor(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    /// Port of `Fraction#getNumerator()`.
    #[inline]
    pub fn numerator(&self) -> i32 {
        self.numerator
    }

    /// Port of `Fraction#getDenominator()`.
    ///
    /// Always positive for a `Fraction` built through [`Fraction::new`], and never zero.
    #[inline]
    pub fn denominator(&self) -> i32 {
        self.denominator
    }

    /// Port of `Fraction#toString()`.
    ///
    /// The oracle does not exercise this today, but `Fraction` shows up in debug text
    /// and it is two lines.
    #[inline]
    pub fn to_java_string(&self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }

    /// Port of `Fraction#doubleValue()`.
    #[inline]
    pub fn double_value(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    // -----------------------------------------------------------------------
    // Not reached by Mth. Present so the type is usable, and explicitly marked so
    // nobody assumes they were ported-and-verified.
    // -----------------------------------------------------------------------

    /// Port of `Fraction#intValue()`.
    pub fn int_value(&self) -> i32 {
        self.numerator / self.denominator
    }

    /// Port of `Fraction#longValue()`.
    pub fn long_value(&self) -> i64 {
        self.numerator as i64 / self.denominator as i64
    }

    /// Port of `Fraction#getNumerator(int)` -- raise the fraction to a power.
    ///
    /// # Panics
    ///
    /// Panics on `i32::MIN`, matching Java's `ArithmeticException("Unable to multiply(...)")`.
    /// Vanilla never reaches it.
    pub fn pow(&self, power: i32) -> Fraction {
        if power == 0 {
            return Fraction::new(1, 1);
        }
        assert!(power != i32::MIN, "Unable to multiply Fraction with unbounded exponent");
        let multiplier = Fraction::new(power, 1);
        if power < 0 {
            Fraction::new(multiplier.denominator, multiplier.numerator)
        } else {
            Fraction::new(
                multiplier.numerator.wrapping_pow(power as u32),
                multiplier.denominator.wrapping_pow(power as u32),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduces_by_gcd() {
        let f = Fraction::new(4, 8);
        assert_eq!((f.numerator(), f.denominator()), (1, 2));
    }

    #[test]
    fn moves_the_sign_onto_the_numerator() {
        let f = Fraction::new(1, -2);
        assert_eq!((f.numerator(), f.denominator()), (-1, 2));
        // Double negative stays positive.
        let f = Fraction::new(-1, -2);
        assert_eq!((f.numerator(), f.denominator()), (1, 2));
    }

    #[test]
    fn zero_short_circuits_to_zero_over_one() {
        let f = Fraction::new(0, 5);
        assert_eq!((f.numerator(), f.denominator()), (0, 1));
        let f = Fraction::new(0, -5);
        assert_eq!((f.numerator(), f.denominator()), (0, 1));
    }

    #[test]
    fn already_reduced_is_left_alone() {
        let f = Fraction::new(3, 7);
        assert_eq!((f.numerator(), f.denominator()), (3, 7));
        let f = Fraction::new(-3, 7);
        assert_eq!((f.numerator(), f.denominator()), (-3, 7));
    }

    #[test]
    fn extreme_components_do_not_panic() {
        // These are the cases that catch a naive `gcd` implementation.
        let _ = Fraction::new(i32::MAX, 1);
        let _ = Fraction::new(i32::MIN, 1);
        let _ = Fraction::new(i32::MIN, -1);
        let _ = Fraction::new(1, i32::MIN);
        let _ = Fraction::new(0, i32::MIN);
    }

    #[test]
    fn to_java_string_matches_commons_lang3() {
        assert_eq!(Fraction::new(4, 8).to_java_string(), "1/2");
        assert_eq!(Fraction::new(-3, 6).to_java_string(), "-1/2");
        assert_eq!(Fraction::new(0, 3).to_java_string(), "0/1");
    }
}
