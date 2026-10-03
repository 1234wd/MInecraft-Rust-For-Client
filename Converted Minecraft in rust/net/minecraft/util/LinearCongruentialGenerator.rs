//! Port of: net/minecraft/util/LinearCongruentialGenerator.java
//! Java class(es): net.minecraft.util.LinearCongruentialGenerator
//! Status: VERIFIED
//!
//! A nested LCG (`rval *= rval * A + C`) used by the noise/decorator pipeline. It
//! is NOT `java.util.Random`: the inner `rval * A + C` is evaluated first and then
//! multiplied by `rval` again. That is easy to "fix" by accident, so keep the shape
//! of the expression identical.

/// Port of `LinearCongruentialGenerator#MULTIPLIER` (`6364136223846793005L`).
pub const MULTIPLIER: i64 = 6364136223846793005i64;
/// Port of `LinearCongruentialGenerator#INCREMENT` (`1442695040888963407L`).
pub const INCREMENT: i64 = 1442695040888963407i64;

/// Port of `LinearCongruentialGenerator#next(long,long)`.
///
/// ```java
/// rval *= rval * 6364136223846793005L + 1442695040888963407L;
/// return rval + c;
/// ```
#[inline]
pub fn next(rval: i64, c: i64) -> i64 {
    let inner = rval.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
    rval.wrapping_mul(inner).wrapping_add(c)
}