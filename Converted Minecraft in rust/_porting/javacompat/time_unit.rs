//! Port of: (no Java counterpart -- see module docs)
//! Java class(es): `java.util.concurrent.TimeUnit` (partial: conversion FROM nanoseconds)
//! Status: VERIFIED
//!
//! Session 22. The seven `TimeUnit` constants and `unit.convert(d, TimeUnit.NANOSECONDS)` -- the
//! only conversion `net.minecraft.util.TimeSource` performs.
//!
//! # Converting from nanoseconds is truncating division
//!
//! Every unit is coarser than (or equal to) a nanosecond, so the JDK's `cvt` never takes its
//! saturating multiply branch here: the result is `d / (nanoseconds per unit)` in `long`
//! arithmetic, which TRUNCATES toward zero (`-1500` ns is `-1` us, `-999` ns is `0` us). Rust's
//! `/` on `i64` does the same. `Long.MIN_VALUE` divides without overflow for every divisor > 1.
//!
//! The other directions (to a FINER unit, which saturate at `Long.MIN_VALUE`/`MAX_VALUE`) are not
//! ported, because nothing ported uses them yet.
//!
//! Checked against JDK 25 by the `TIMESRC` group of `_porting/test-data/small_probe.txt`
//! (`_porting/tests/session22.rs`), every unit for ten durations including both `long` extremes.

/// Port of `java.util.concurrent.TimeUnit`. Declaration order is the ordinal order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    Nanoseconds = 0,
    Microseconds = 1,
    Milliseconds = 2,
    Seconds = 3,
    Minutes = 4,
    Hours = 5,
    Days = 6,
}

impl TimeUnit {
    /// `TimeUnit.values()`.
    pub const VALUES: [TimeUnit; 7] = [
        TimeUnit::Nanoseconds,
        TimeUnit::Microseconds,
        TimeUnit::Milliseconds,
        TimeUnit::Seconds,
        TimeUnit::Minutes,
        TimeUnit::Hours,
        TimeUnit::Days,
    ];

    /// `name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            TimeUnit::Nanoseconds => "NANOSECONDS",
            TimeUnit::Microseconds => "MICROSECONDS",
            TimeUnit::Milliseconds => "MILLISECONDS",
            TimeUnit::Seconds => "SECONDS",
            TimeUnit::Minutes => "MINUTES",
            TimeUnit::Hours => "HOURS",
            TimeUnit::Days => "DAYS",
        }
    }

    /// Nanoseconds in one of this unit (the JDK's private `scale`).
    pub const fn nanos_per_unit(&self) -> i64 {
        match self {
            TimeUnit::Nanoseconds => 1,
            TimeUnit::Microseconds => 1_000,
            TimeUnit::Milliseconds => 1_000_000,
            TimeUnit::Seconds => 1_000_000_000,
            TimeUnit::Minutes => 60_000_000_000,
            TimeUnit::Hours => 3_600_000_000_000,
            TimeUnit::Days => 86_400_000_000_000,
        }
    }

    /// Port of `this.convert(sourceDuration, TimeUnit.NANOSECONDS)`: truncating division.
    pub const fn convert_from_nanos(&self, source_duration: i64) -> i64 {
        source_duration / self.nanos_per_unit()
    }
}
