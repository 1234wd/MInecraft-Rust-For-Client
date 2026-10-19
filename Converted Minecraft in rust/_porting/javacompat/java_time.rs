//! `java.time` value types, the parts ported files need. Session 24. Pinned to JDK 25: every body
//! below is transcribed from `java.base/java/time/*.java` in the JDK's own `src.zip`.
//!
//! | Java | here | used by |
//! |---|---|---|
//! | `Instant` (value, `ofEpochSecond`, accessors, `equals`/`hashCode`/`compareTo`) | [`Instant`] | `RecordedDeviation` |
//!
//! # What is NOT here
//!
//! Anything that reads a clock (`Instant.now()`), and anything that formats or parses
//! (`Instant.toString()` is ISO-8601 through `DateTimeFormatter.ISO_INSTANT`; `parse`). Those need
//! a real time-zone/formatter model, and a partial one would print plausible but wrong dates. A
//! caller that needs them stays blocked on them.

use std::cmp::Ordering;

/// `Instant.MIN_SECOND`: `-1000000000-01-01T00:00Z`.
pub const MIN_SECOND: i64 = -31_557_014_167_219_200;
/// `Instant.MAX_SECOND`: `1000000000-12-31T23:59:59.999999999Z`.
pub const MAX_SECOND: i64 = 31_556_889_864_403_199;
/// `NANOS_PER_SECOND`.
const NANOS_PER_SECOND: i64 = 1_000_000_000;

/// Port of `java.time.Instant`: seconds since the epoch and a nanosecond in `0..1_000_000_000`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instant {
    seconds: i64,
    nanos: i32,
}

/// Port of `Instant.EPOCH`.
pub const EPOCH: Instant = Instant {
    seconds: 0,
    nanos: 0,
};

impl Instant {
    /// Port of `private static Instant create(long seconds, int nanoOfSecond)`.
    fn create(seconds: i64, nano_of_second: i32) -> Instant {
        if (seconds | i64::from(nano_of_second)) == 0 {
            return EPOCH;
        }
        if !(MIN_SECOND..=MAX_SECOND).contains(&seconds) {
            panic!("java.time.DateTimeException: Instant exceeds minimum or maximum instant");
        }
        Instant {
            seconds,
            nanos: nano_of_second,
        }
    }

    /// Port of `Instant.ofEpochSecond(long epochSecond)`.
    pub fn of_epoch_second(epoch_second: i64) -> Instant {
        Self::create(epoch_second, 0)
    }

    /// Port of `Instant.ofEpochSecond(long epochSecond, long nanoAdjustment)`: the adjustment is
    /// folded into the seconds with `floorDiv`/`floorMod`, and the sum is `Math.addExact`
    /// (`ArithmeticException: long overflow`).
    pub fn of_epoch_second_adjusted(epoch_second: i64, nano_adjustment: i64) -> Instant {
        let secs = epoch_second
            .checked_add(nano_adjustment.div_euclid(NANOS_PER_SECOND))
            .unwrap_or_else(|| panic!("java.lang.ArithmeticException: long overflow"));
        // `floorMod` by a positive divisor is the Euclidean remainder.
        let nos = nano_adjustment.rem_euclid(NANOS_PER_SECOND) as i32;
        Self::create(secs, nos)
    }

    /// Port of `getEpochSecond()`.
    pub fn get_epoch_second(&self) -> i64 {
        self.seconds
    }

    /// Port of `getNano()`.
    pub fn get_nano(&self) -> i32 {
        self.nanos
    }

    /// Port of `compareTo(Instant)`: `Long.compare` of the seconds, then the DIFFERENCE of the
    /// nanos (not just its sign).
    pub fn compare_to(&self, other: &Instant) -> i32 {
        match self.seconds.cmp(&other.seconds) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => self.nanos - other.nanos,
        }
    }

    /// Port of `hashCode()`: `Long.hashCode(seconds) + 51 * nanos`.
    pub fn java_hash_code(&self) -> i32 {
        let long_hash = (self.seconds ^ ((self.seconds as u64) >> 32) as i64) as i32;
        long_hash.wrapping_add(self.nanos.wrapping_mul(51))
    }
}

impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Instant {
    /// The order of `compareTo`.
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_to(other).cmp(&0)
    }
}
