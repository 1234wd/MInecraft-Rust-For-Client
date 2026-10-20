//! `java.time` value types, the parts ported files need. Session 24. Pinned to JDK 25: every body
//! below is transcribed from `java.base/java/time/*.java` in the JDK's own `src.zip`.
//!
//! | Java | here | used by |
//! |---|---|---|
//! | `Instant` (value, `ofEpochSecond`, accessors, `equals`/`hashCode`/`compareTo`/`toString`) | [`Instant`] | `RecordedDeviation`, `TickTimeStat` |
//! | `Duration` (`ofNanos`, `ofSeconds`, accessors, value methods and ISO `toString`) | [`Duration`] | `TickTimeStat` |
//!
//! # What is NOT here
//!
//! Anything that reads a clock (`Instant.now()`), parses temporal strings, formats calendar zones,
//! or uses the broader temporal adjuster/unit APIs. `Instant.toString()` and the duration string
//! form needed by `TickTimeStat` are implemented directly from JDK 25 behavior; this is not a full
//! `java.time` port.

use std::cmp::Ordering;

/// `Instant.MIN_SECOND`: `-1000000000-01-01T00:00Z`.
pub const MIN_SECOND: i64 = -31_557_014_167_219_200;
/// `Instant.MAX_SECOND`: `1000000000-12-31T23:59:59.999999999Z`.
pub const MAX_SECOND: i64 = 31_556_889_864_403_199;
/// `NANOS_PER_SECOND`.
const NANOS_PER_SECOND: i64 = 1_000_000_000;
const SECONDS_PER_DAY: i64 = 86_400;

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

    /// Port of `Instant.toString()` through `DateTimeFormatter.ISO_INSTANT`.
    pub fn java_to_string(&self) -> String {
        let days = self.seconds.div_euclid(SECONDS_PER_DAY);
        let second_of_day = self.seconds.rem_euclid(SECONDS_PER_DAY);
        let (year, month, day) = civil_from_days(days);
        let hour = second_of_day / 3_600;
        let minute = (second_of_day % 3_600) / 60;
        let second = second_of_day % 60;
        let mut result = format!(
            "{}-{:02}-{:02}T{:02}:{:02}:{:02}",
            format_year(year),
            month,
            day,
            hour,
            minute,
            second
        );
        if self.nanos != 0 {
            if self.nanos % 1_000_000 == 0 {
                result.push_str(&format!(".{:03}", self.nanos / 1_000_000));
            } else if self.nanos % 1_000 == 0 {
                result.push_str(&format!(".{:06}", self.nanos / 1_000));
            } else {
                result.push_str(&format!(".{:09}", self.nanos));
            }
        }
        result.push('Z');
        result
    }
}

impl std::fmt::Display for Instant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.java_to_string())
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

/// The value-semantic subset of `java.time.Duration` needed by Minecraft's timed-stat records.
/// Its representation follows Java's normalized form: nanos is always in `0..1_000_000_000`,
/// including for negative values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Duration {
    seconds: i64,
    nanos: i32,
}

/// Port of `Duration.ZERO`.
pub const DURATION_ZERO: Duration = Duration {
    seconds: 0,
    nanos: 0,
};

impl Duration {
    /// Port of `Duration.ofNanos(long)`. The input is a signed total nanosecond count.
    pub fn of_nanos(nanos: i64) -> Duration {
        Duration {
            seconds: nanos.div_euclid(NANOS_PER_SECOND),
            nanos: nanos.rem_euclid(NANOS_PER_SECOND) as i32,
        }
    }

    /// Convert the JDK 25 `RecordedObject.getDuration` interpretation of a signed
    /// `@Timespan(NANOSECONDS)` long. JFR reserves both long extremes as sentinels.
    pub fn from_jfr_nanoseconds(nanos: i64) -> Duration {
        match nanos {
            0 => DURATION_ZERO,
            i64::MIN => Self::of_seconds_adjusted(i64::MIN, 0),
            i64::MAX => Self::of_seconds_adjusted(i64::MAX, 999_999_999),
            value => Self::of_nanos(value),
        }
    }

    /// Port of `Duration.ofSeconds(long, long)`, including floor normalization and checked
    /// seconds addition. The only overflow is the same `ArithmeticException` Java raises.
    pub fn of_seconds_adjusted(seconds: i64, nano_adjustment: i64) -> Duration {
        Self::try_of_seconds_adjusted(seconds, nano_adjustment)
            .unwrap_or_else(|| panic!("java.lang.ArithmeticException: long overflow"))
    }

    /// Checked normalization used to test the overflow boundary without relying on a Rust panic.
    pub fn try_of_seconds_adjusted(seconds: i64, nano_adjustment: i64) -> Option<Duration> {
        let seconds = seconds.checked_add(nano_adjustment.div_euclid(NANOS_PER_SECOND))?;
        Some(Duration {
            seconds,
            nanos: nano_adjustment.rem_euclid(NANOS_PER_SECOND) as i32,
        })
    }

    pub const fn get_seconds(&self) -> i64 {
        self.seconds
    }

    pub const fn get_nano(&self) -> i32 {
        self.nanos
    }

    /// Port of `Duration.compareTo(Duration)` over the normalized seconds/nanos pair.
    pub fn compare_to(&self, other: &Duration) -> i32 {
        match self.seconds.cmp(&other.seconds) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => self.nanos - other.nanos,
        }
    }

    /// Port of `Duration.hashCode()`: `Long.hashCode(seconds) + 51 * nanos`.
    pub fn java_hash_code(&self) -> i32 {
        let long_hash = (self.seconds ^ ((self.seconds as u64) >> 32) as i64) as i32;
        long_hash.wrapping_add(self.nanos.wrapping_mul(51))
    }

    /// Port of JDK 25 `Duration.toString()` (ISO-8601 seconds form).
    pub fn java_to_string(&self) -> String {
        if self.seconds == 0 && self.nanos == 0 {
            return "PT0S".to_string();
        }
        let mut effective_total_seconds = self.seconds;
        if self.seconds < 0 && self.nanos > 0 {
            effective_total_seconds += 1;
        }
        let hours = effective_total_seconds / 3_600;
        let minutes = (effective_total_seconds % 3_600) / 60;
        let seconds = effective_total_seconds % 60;
        let mut result = String::from("PT");
        if hours != 0 {
            result.push_str(&format!("{hours}H"));
        }
        if minutes != 0 {
            result.push_str(&format!("{minutes}M"));
        }
        if seconds == 0 && self.nanos == 0 && result.len() > 2 {
            return result;
        }
        if self.seconds < 0 && self.nanos > 0 {
            if seconds == 0 {
                result.push_str("-0");
            } else {
                result.push_str(&seconds.to_string());
            }
        } else {
            result.push_str(&seconds.to_string());
        }
        if self.nanos > 0 {
            let position = result.len();
            let fractional = if self.seconds < 0 {
                2_000_000_000 - i64::from(self.nanos)
            } else {
                i64::from(self.nanos) + NANOS_PER_SECOND
            };
            result.push_str(&fractional.to_string());
            while result.ends_with('0') {
                result.pop();
            }
            result.replace_range(position..position + 1, ".");
        }
        result.push('S');
        result
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_to(other).cmp(&0)
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.java_to_string())
    }
}

/// Convert epoch-day number to the proleptic Gregorian date used by `ISO_INSTANT`.
fn civil_from_days(epoch_days: i64) -> (i64, i64, i64) {
    let z = epoch_days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    if month <= 2 {
        year += 1;
    }
    (year, month, day)
}

fn format_year(year: i64) -> String {
    if (0..=9_999).contains(&year) {
        format!("{year:04}")
    } else if year > 9_999 {
        format!("+{year:04}")
    } else {
        format!("-{:04}", -year)
    }
}
