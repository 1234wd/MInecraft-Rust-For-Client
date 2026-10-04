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
//! # Digit generation
//!
//! Both languages produce the SHORTEST decimal string that round-trips, and since JDK 19
//! Java's is the Raffaello Giulietti algorithm, which is what Rust also implements. So the
//! DIGITS agree; only the LAYOUT differs. This module therefore takes Rust's shortest
//! scientific form (`{:e}`, e.g. `1.5e2`, `1e-5`) and re-lays it out in Java's shape.
//!
//! Caveat worth stating: pre-19 Java produced a shortest-but-not-shortest decimal in some
//! cases. We target 26.2 on JDK 25, so that is not a live concern -- but it is why this
//! says "JDK 25" rather than "Java".

/// `Float.toString(float)`.
pub fn float_to_string(v: f32) -> String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return if v > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    // Rust's `{:e}` gives the shortest round-tripping digits in scientific form, which is
    // exactly the decomposition Java's algorithm needs.
    let sci = format!("{:e}", v);
    layout(&sci)
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
    let sci = format!("{:e}", v);
    layout(&sci)
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
    let (mantissa, exp) = sci.split_once('e').expect("Rust `{:e}` always emits an `e`");
    let exp: i32 = exp.parse().expect("Rust `{:e}` always emits a decimal exponent");

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

    /// Every result must round-trip, or it is not a faithful reproduction.
    #[test]
    fn round_trips() {
        for v in [
            0.0f32, 1.0, -1.0, 0.1, 1.0 / 3.0, 1.0e-4, 1.0e-3, 1.0e7, 1.0e8,
            1.0e30, 1.0e-30, f32::MIN_POSITIVE, f32::MAX, 123456.0, 0.000999,
        ] {
            let s = float_to_string(v);
            let back: f32 = s.parse().expect("java float_to_string must round-trip");
            assert_eq!(back.to_bits(), v.to_bits(), "{s} did not round-trip {v}");
        }
    }
}