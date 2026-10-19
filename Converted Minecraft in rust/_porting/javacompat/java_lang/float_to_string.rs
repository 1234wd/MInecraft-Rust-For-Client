//! Port of: (no `.java` counterpart -- JDK internals)
//! Java class(es): `java.lang.Float#toString(float)`, `java.lang.Double#toString(double)`
//! Status: PORTED
//!
//! # WHY RUST'S `Display` IS NOT CLOSE ENOUGH
//!
//! Java's `Float.toString` and Rust's `Display` for floats differ in three ways that all
//! show up in player-visible text -- SNBT, `Rotations.toString`, block-state strings,
//! error messages:
//!
//! | value | Java | Rust `{}` |
//! |---|---|---|
//! | `0.0f32` | `0.0` | `0` |
//! | `1.0f32` | `1.0` | `1` |
//! | `-0.0f32` | `-0.0` | `-0` |
//! | `1.0e-5f32` | `1.0E-5` | `0.00001` |
//! | `1.0e8f32` | `1.0E8` | `100000000` |
//! | `1.0e7f32` | `1.0E7` | `10000000` |
//!
//! So: Java ALWAYS prints a fractional part (or an exponent), and it switches to
//! scientific notation outside `[1e-3, 1e7)` -- a boundary Rust does not share.
//!
//! `0.0` vs `0` is the one that bites immediately: `Rotations(0f,0f,0f).toString()` is
//! `"Rotations[x=0.0, y=0.0, z=0.0]"`, not `"...[x=0, ...]"`. That is a golden-test failure
//! on the very first row.
//!
//! # Digit generation: JAVA HAS A TWO-DIGIT MINIMUM, RUST DOES NOT
//!
//! Both languages produce a shortest round-tripping decimal, and since JDK 19 Java's digit
//! generation is the Raffaello Giulietti algorithm that Rust also implements -- so for
//! almost every value the digits agree and only the LAYOUT differs.
//!
//! **Almost.** Java's spec does not ask for the shortest decimal; it asks for this:
//!
//! > A *decimal* is `s * 10^i` with integers `s > 0` and `i`, where **`s` is not a multiple
//! > of 10**; its *length* is the digit count of `s`. Let `R` be the set of all decimals
//! > that round to `v`, and `p` the minimal length over `R`. **If `p <= 2`, let `T` be the
//! > decimals in `R` of length 1 or 2**; otherwise those of length `p`. Then `d` is the
//! > decimal in `T` closest to `v`.
//!
//! The trap is in the last line. When `p == 1` Java does not take the one-digit decimal --
//! it takes the closest decimal of length **at most** 2, which is usually a different
//! number:
//!
//! | value | shortest (Rust `{:e}`) | padded | Java | same? |
//! |---|---|---|---|---|
//! | `0.5f` | `5e-1` | `0.5` | `0.5` | yes |
//! | `1.0f` | `1e0` | `1.0` | `1.0` | yes |
//! | `Float.MIN_VALUE` | `1e-45` | `1.0E-45` | **`1.4E-45`** | no |
//! | bits `0x2` | `3e-45` | `3.0E-45` | **`2.8E-45`** | no |
//! | `Double.MIN_VALUE` | `5e-324` | `5.0E-324` | **`4.9E-324`** | no |
//!
//! `1e-45` and `3e-45` are both genuinely shortest *and* round-tripping -- Java simply does
//! not want them, because a two-digit decimal sits closer to the real value. Padding with a
//! zero is wrong in a way no round-trip test can catch, since the padded form parses back
//! to the same bits.
//!
//! The "`s` is not a multiple of 10" clause is what keeps `0.5f` as `0.5`: the nearest
//! two-significant-digit rendering is `5.0e-1`, whose significand `50` is a multiple of 10,
//! so its canonical form is `5 * 10^-1` -- length 1, and the string is `0.5`. That single
//! clause is the difference between this rule and "always emit two digits", which is what
//! the first attempt at this fix assumed and which prints `0.50` and `0.0010`.
//!
//! # AND THE TIE RULE DISAGREES TOO
//!
//! The spec finishes: `d` is the decimal in `T` closest to `v`, and **"if there are two
//! such decimals, the one whose least significant digit is even"**. Rust's shortest
//! formatting (`{:e}`) breaks a tie by rounding **up**:
//!
//! | `f32` bits | true value is exactly between | Java | Rust `{:e}` |
//! |---|---|---|---|
//! | `1087348736` | `6.4882812` and `6.4882813` | `6.4882812` | `6.4882813` |
//! | `1114531840` | `59.601562` and `59.601563` | `59.601562` | `59.601563` |
//!
//! Measured: **4,282 of 11,223,706** corpus values hit this, which is 0.04% -- rare enough
//! to survive sixteen sessions of golden tests, common enough that a chunk of
//! float-bearing SNBT will contain one.
//!
//! Rust's *explicit-precision* formatting does round half-to-even (`{:.1e}` of `1.25` is
//! `1.2e0`, of `1.35` is `1.4e0`), so both problems have the same cure.
//!
//! # THE RULE, IMPLEMENTED
//!
//! 1. Ask Rust for the shortest form and count its significant digits -- that is the
//!    spec's `p`.
//! 2. Re-render with **explicit precision** at `max(p, 2)` significant digits. Explicit
//!    precision is correctly rounded and breaks ties to even, so this fixes the tie rule
//!    for every `p` and the two-digit minimum when `p == 1`.
//! 3. Drop a trailing mantissa zero, so the significand is not a multiple of 10 (step 2 can
//!    introduce one only when it forced `p` up to 2).
//! 4. Lay the result out in Java's shape.
//!
//! Step 2 is a no-op for the 99.96% of values that are neither ties nor one-digit, because
//! the nearest `p`-digit decimal is then unique. It cannot change the digit COUNT either:
//! a carry would need a significand that is a multiple of 10, which would contradict `p`
//! being minimal.
//!
//! This was `OPEN_QUESTIONS.md` #18 for sixteen sessions. Session 05 found the symptom
//! (`Float.MIN_VALUE` printing wrongly), session 20 measured its extent (nine float
//! subnormal bit patterns) and session 21 found the rule. It did NOT need
//! `java.lang.FloatingDecimal` -- the thing that looked like a 1,800-line port was a
//! two-line one, which is the argument for measuring before porting.

/// `Float.toString(float)`.
pub fn float_to_string(v: f32) -> String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return if v > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    layout(&java_digits_f32(v))
}

/// Java's digit choice for an `f32`: `max(p, 2)` significant digits, ties to even.
///
/// See the module docs for the spec clauses this implements and the measurements behind
/// each one.
fn java_digits_f32(v: f32) -> String {
    let digits = significant_digits(&format!("{:e}", v)).max(2);
    canonicalise_significand(&format!("{:.*e}", digits - 1, v))
}

/// `Double.toString(double)`.
///
/// Same layout rules as the `f32` version.
pub fn double_to_string(v: f64) -> String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return if v > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    layout(&java_digits_f64(v))
}

/// [`java_digits_f32`] for `f64`.
fn java_digits_f64(v: f64) -> String {
    let digits = significant_digits(&format!("{:e}", v)).max(2);
    canonicalise_significand(&format!("{:.*e}", digits - 1, v))
}

/// Drop a trailing mantissa zero, so the significand is not a multiple of 10.
///
/// `5.0e-1` -> `5e-1`, `1.4e-45` -> unchanged. This is the spec's definition of a decimal
/// doing real work: without it the nearest two-digit rendering of `0.5f` prints as `0.50`.
fn canonicalise_significand(sci: &str) -> String {
    match sci.split_once('e') {
        Some((mantissa, exp)) => match mantissa.strip_suffix(".0") {
            Some(trimmed) => format!("{trimmed}e{exp}"),
            None => sci.to_string(),
        },
        None => sci.to_string(),
    }
}

/// The spec's "length" of a decimal: mantissa digits, **keeping** trailing zeros and
/// ignoring the sign, the point and the exponent.
///
/// `1e-45` is 1, `1.4e-45` is 2, `1.0e0` is 2. Leading zeros cannot occur in `{:e}` output
/// (the mantissa is always `d` or `d.ddd` with `d` in 1..=9), except for zero itself, whose
/// mantissa is the single digit `0`.
fn significant_digits(sci: &str) -> usize {
    let mantissa = sci.split('e').next().unwrap_or(sci);
    mantissa.chars().filter(char::is_ascii_digit).count()
}

/// Re-lay Rust's `d.dddde<exp>` shortest form into Java's `Float.toString` shape.
///
/// # Layout rules, from the JavaDoc of `Float.toString`
///
/// * `m >= 1e-3 && m < 1e7`: plain decimal, with at least one digit after the point.
/// * otherwise: `d.dddEn` -- one digit before the point, at least one after.
/// * there is always at least one digit after the point, so `1.0` never prints as `1`.
///
/// # Panics
///
/// Never for input produced by Rust's `{:e}`. The `expect` documents the invariant
/// rather than papering over it.
fn layout(sci: &str) -> String {
    let (mantissa, exp) = sci
        .split_once('e')
        .expect("Rust `{:e}` always emits an `e`");
    let exp: i32 = exp
        .parse()
        .expect("Rust `{:e}` always emits a decimal exponent");

    let (neg, digits) = match mantissa.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, mantissa),
    };

    // `digits` is `d` followed by an optional `.ddd`. Drop the point and remember the
    // position of the decimal relative to the first significant digit.
    let (int_part, frac_part) = match digits.split_once('.') {
        Some((a, b)) => (a, b),
        None => (digits, ""),
    };
    let all: String = format!("{int_part}{frac_part}");
    debug_assert!(!all.is_empty(), "mantissa had no digits: {sci}");

    // The value is `d.ddd * 10^exp` with `d` in 1..=9, so Java's "plain decimal when
    // 1e-3 <= m < 1e7" is EXACTLY `-3 <= exp <= 6`. Deriving it from the exponent rather
    // than from the digit count matters: my first attempt switched on the number of digits
    // before the point, which printed 0.5 as "5.0E-1" and 0.001 as "1.0E-3".
    let plain = (-3..=6).contains(&exp);

    let mut out = String::new();
    if neg {
        out.push('-');
    }

    if plain {
        // Count of digits before the point in the plain rendering.
        let n = exp + 1;
        if n <= 0 {
            out.push_str("0.");
            for _ in 0..(-n) {
                out.push('0');
            }
            out.push_str(&all);
        } else {
            let n = n as usize;
            if all.len() <= n {
                out.push_str(&all);
                for _ in all.len()..n {
                    out.push('0');
                }
                out.push_str(".0");
            } else {
                out.push_str(&all[..n]);
                out.push('.');
                out.push_str(&all[n..]);
            }
        }
    } else {
        // Scientific: one digit before the point, at least one after.
        out.push_str(&all[..1]);
        out.push('.');
        if all.len() > 1 {
            out.push_str(&all[1..]);
        } else {
            out.push('0');
        }
        out.push('E');
        out.push_str(&exp.to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The cases the module docs table promises. These are ASSERTIONS ABOUT THE JAVA
    /// SPEC (documented above), not golden values -- the authoritative check for the
    /// actual float corpus is `rotations.toString` in `parity_batch2.rs`, which compares
    /// against the jar for every value.
    #[test]
    fn always_prints_a_fractional_part() {
        assert_eq!(float_to_string(0.0), "0.0");
        assert_eq!(float_to_string(1.0), "1.0");
        assert_eq!(float_to_string(-1.0), "-1.0");
        // Rust's `Display` gives "0" and "-0" here, which is the whole reason this module
        // exists.
        assert_eq!(format!("{}", 0.0f32), "0");
        assert_ne!(format!("{}", 0.0f32), float_to_string(0.0));
    }

    #[test]
    fn negative_zero_keeps_its_sign() {
        assert_eq!(float_to_string(-0.0), "-0.0");
        assert_eq!(format!("{}", -0.0f32), "-0");
    }

    #[test]
    fn scientific_boundaries_match_java() {
        // Java switches to scientific OUTSIDE [1e-3, 1e7). Note 1e7 itself is scientific.
        assert_eq!(float_to_string(1.0e-4), "1.0E-4");
        assert_eq!(float_to_string(1.0e-3), "0.001");
        assert_eq!(float_to_string(9999999.0), "9999999.0");
        assert_eq!(float_to_string(1.0e7), "1.0E7");
        assert_eq!(float_to_string(1.0e8), "1.0E8");
    }

    #[test]
    fn non_finite_spellings() {
        assert_eq!(float_to_string(f32::NAN), "NaN");
        assert_eq!(float_to_string(f32::INFINITY), "Infinity");
        assert_eq!(float_to_string(f32::NEG_INFINITY), "-Infinity");
        assert_eq!(double_to_string(f64::NAN), "NaN");
        assert_eq!(double_to_string(f64::INFINITY), "Infinity");
    }

    #[test]
    fn plain_values() {
        assert_eq!(float_to_string(0.5), "0.5");
        assert_eq!(float_to_string(-90.0), "-90.0");
        assert_eq!(float_to_string(370.0), "370.0");
        assert_eq!(float_to_string(0.1), "0.1");
        assert_eq!(float_to_string(1.0 / 3.0), "0.33333334");
    }

    /// The two-digit minimum: when the shortest decimal has ONE significant digit, Java
    /// emits the closest decimal of length at most two instead.
    ///
    /// Oracle: `Float.toString` / `Double.toString` on JDK 21.0.12. The left column is what
    /// Rust's `{:e}` alone would have produced, and it round-trips -- which is why no
    /// round-trip test ever caught this.
    #[test]
    fn one_digit_values_use_the_nearest_two_digit_decimal() {
        // bits -> Java. The nine float patterns where padding with a zero is WRONG.
        for (bits, want) in [
            (1u32, "1.4E-45"),
            (2, "2.8E-45"),
            (3, "4.2E-45"),
            (4, "5.6E-45"),
            (6, "8.4E-45"),
            (7, "9.8E-45"),
            (21, "2.9E-44"),
            (29, "4.1E-44"),
            (71, "9.9E-44"),
        ] {
            assert_eq!(
                float_to_string(f32::from_bits(bits)),
                want,
                "f32 bits {bits}"
            );
        }

        // And the ones where the significand IS a multiple of 10, so the canonical decimal
        // has length 1 and the padded spelling is the right one after all.
        assert_eq!(float_to_string(f32::from_bits(5)), "7.0E-45");
        assert_eq!(float_to_string(0.5), "0.5");
        assert_eq!(float_to_string(1.0), "1.0");
        assert_eq!(float_to_string(100.0), "100.0");
        assert_eq!(float_to_string(0.001), "0.001");

        // `Double.MIN_VALUE` is the double-side case, and it is what kept
        // `aabb.toString` PORT-BLOCKED for fifteen golden rows.
        assert_eq!(double_to_string(f64::from_bits(1)), "4.9E-324");
        assert_eq!(double_to_string(f64::from_bits(2)), "9.9E-324");
        assert_eq!(double_to_string(f64::from_bits(10)), "4.9E-323");
        assert_eq!(double_to_string(f64::from_bits(12)), "5.9E-323");
    }

    /// Ties go to the EVEN last digit, where Rust's shortest formatting rounds up.
    ///
    /// Oracle: `Float.toString` on JDK 21.0.12. Each value sits exactly between two
    /// 8-digit decimals; Java takes the even one. 4,282 of the 11.2M-value verification
    /// corpus hit this.
    #[test]
    fn ties_round_to_the_even_digit() {
        for (bits, want) in [
            (1087348736u32, "6.4882812"),
            (1099894784, "17.882812"),
            (1114531840, "59.601562"),
            (1126032384, "157.89062"),
            (1149294848, "1030.7812"),
        ] {
            let v = f32::from_bits(bits);
            assert_eq!(float_to_string(v), want, "f32 bits {bits}");
            // The thing that makes this a TIE and not a rounding error: the digit Rust
            // would have chosen also round-trips.
            let rust_shortest = format!("{:e}", v);
            assert_ne!(
                rust_shortest.split('e').next().unwrap().replace('.', ""),
                want.replace('.', ""),
                "bits {bits} was supposed to be a tie Rust gets wrong"
            );
        }
    }

    /// Every result must round-trip, or it is not a faithful reproduction.
    #[test]
    fn round_trips() {
        for v in [
            0.0f32,
            1.0,
            -1.0,
            0.1,
            1.0 / 3.0,
            1.0e-4,
            1.0e-3,
            1.0e7,
            1.0e8,
            1.0e30,
            1.0e-30,
            f32::MIN_POSITIVE,
            f32::MAX,
            123456.0,
            0.000999,
        ] {
            let s = float_to_string(v);
            let back: f32 = s.parse().expect("java float_to_string must round-trip");
            assert_eq!(back.to_bits(), v.to_bits(), "{s} did not round-trip {v}");
        }
    }
}
