//! Port of: (no Java counterpart -- see module docs)
//! Java class(es): boxed `hashCode`s, `Objects.hash`, `java.lang.runtime.ObjectMethods` (records)
//! Status: VERIFIED
//!
//! Session 22. The `equals`/`hashCode` conventions a Java `record` gets for free and a Rust
//! struct does not, kept in one place so thirty small record ports cannot each get one wrong.
//!
//! Checked against JDK 25 by the `HASH` and `REC` groups of `_porting/test-data/small_probe.txt`
//! (`_porting/tests/session22.rs`).
//!
//! # A record's `hashCode` is a fold from ZERO
//!
//! `javac` does not write a record's `hashCode`; it emits an `invokedynamic` to
//! `java.lang.runtime.ObjectMethods`, whose combiner is `(acc, h) -> acc * 31 + h` starting from
//! `0`. So a one-component record hashes to its component's hash, and `record P(int a, int b)`
//! hashes to `31 * a + b`. That is NOT `Objects.hash(a, b)`, which starts from `1` and gives
//! `31 * (31 + a) + b` -- the two differ by `31^n` for `n` components, and [`record_hash`] and
//! [`objects_hash`] are separate functions for that reason. The fold is a JDK implementation
//! detail rather than a spec guarantee, which is exactly why it is probed rather than assumed.
//!
//! # `equals` on `float`/`double` components is `Float.compare == 0`
//!
//! A record compares primitive float components with `Float.compare`, not `==`, so:
//!
//! * `NaN` EQUALS `NaN` -- every NaN, whatever its payload, because `compare` goes through
//!   `floatToIntBits`, which canonicalises;
//! * `0.0` does NOT equal `-0.0`.
//!
//! Both are the opposite of Rust's derived `PartialEq` on `f32`. [`equals_f32`] / [`equals_f64`]
//! are the Java rule. Because it is reflexive, a Rust `Eq` impl built on it is sound.
//!
//! # Boxed hash codes
//!
//! | Java | rule |
//! |---|---|
//! | `Boolean.hashCode` | `1231` for true, `1237` for false |
//! | `Long.hashCode` | `(int) (v ^ (v >>> 32))` |
//! | `Float.hashCode` | `floatToIntBits(v)` -- canonical NaN `0x7fc00000`, `-0.0` != `0.0` |
//! | `Double.hashCode` | `Long.hashCode(doubleToLongBits(v))` -- canonical NaN |
//! | `Byte`/`Short`/`Integer`/`Character` | the value, widened |
//! | `Objects.hashCode(null)` | `0` |

/// `Boolean.hashCode(b)`.
#[inline]
pub const fn hash_bool(b: bool) -> i32 {
    if b {
        1231
    } else {
        1237
    }
}

/// `Long.hashCode(v)`: `(int) (v ^ (v >>> 32))`.
#[inline]
pub const fn hash_i64(v: i64) -> i32 {
    (v ^ ((v as u64) >> 32) as i64) as i32
}

/// `Float.floatToIntBits(v)`: the raw bits, except every NaN becomes `0x7fc00000`.
#[inline]
pub fn float_to_int_bits(v: f32) -> i32 {
    if v.is_nan() {
        0x7fc0_0000
    } else {
        v.to_bits() as i32
    }
}

/// `Double.doubleToLongBits(v)`: the raw bits, except every NaN becomes `0x7ff8000000000000`.
#[inline]
pub fn double_to_long_bits(v: f64) -> i64 {
    if v.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        v.to_bits() as i64
    }
}

/// `Float.hashCode(v)`.
#[inline]
pub fn hash_f32(v: f32) -> i32 {
    float_to_int_bits(v)
}

/// `Double.hashCode(v)`.
#[inline]
pub fn hash_f64(v: f64) -> i32 {
    hash_i64(double_to_long_bits(v))
}

/// `Float.compare(a, b) == 0`: NaN equals NaN, `0.0` does not equal `-0.0`.
#[inline]
pub fn equals_f32(a: f32, b: f32) -> bool {
    float_to_int_bits(a) == float_to_int_bits(b)
}

/// `Double.compare(a, b) == 0`: NaN equals NaN, `0.0` does not equal `-0.0`.
#[inline]
pub fn equals_f64(a: f64, b: f64) -> bool {
    double_to_long_bits(a) == double_to_long_bits(b)
}

/// A record's generated `hashCode`: `acc * 31 + h` over the components' hashes, from `0`.
#[inline]
pub fn record_hash(component_hashes: &[i32]) -> i32 {
    component_hashes
        .iter()
        .fold(0i32, |acc, &h| acc.wrapping_mul(31).wrapping_add(h))
}

/// `Objects.hash(values...)` (`Arrays.hashCode`): the same fold, but from `1`.
#[inline]
pub fn objects_hash(hashes: &[i32]) -> i32 {
    hashes
        .iter()
        .fold(1i32, |acc, &h| acc.wrapping_mul(31).wrapping_add(h))
}

/// `Objects.hashCode(o)` for an optional component: `0` for null.
#[inline]
pub fn hash_opt<T>(v: Option<&T>, hash: impl Fn(&T) -> i32) -> i32 {
    v.map_or(0, hash)
}

/// `String.valueOf(Object)` for a nullable `String` component: `"null"` for null.
#[inline]
pub fn str_or_null(s: Option<&str>) -> &str {
    s.unwrap_or("null")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_hash_is_not_objects_hash() {
        assert_eq!(record_hash(&[]), 0);
        assert_eq!(objects_hash(&[]), 1);
        assert_eq!(record_hash(&[7]), 7);
        assert_eq!(record_hash(&[1, 2]), 31 + 2);
        assert_eq!(objects_hash(&[1, 2]), 31 * (31 + 1) + 2);
    }

    #[test]
    fn float_equality_is_bitwise_after_nan_canonicalisation() {
        assert!(equals_f32(f32::NAN, f32::from_bits(0x7f80_0001)));
        assert!(!equals_f32(0.0, -0.0));
        assert!(equals_f64(f64::NAN, -f64::NAN));
        assert!(!equals_f64(0.0, -0.0));
    }
}
