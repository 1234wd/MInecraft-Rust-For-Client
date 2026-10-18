//! Port of: net/minecraft/util/TimeSource.java
//! Java class(es): net.minecraft.util.TimeSource, net.minecraft.util.NanoTimeSource
//! Status: VERIFIED
//!
//! Session 22. `@FunctionalInterface public interface TimeSource`: `long get(TimeUnit)`, a clock read
//! in a requested unit -- with its nested `NanoTimeSource` (a `LongSupplier` of nanoseconds whose
//! default `get(unit)` converts) and `constant(value)`.
//!
//! # Conversion
//!
//! `NanoTimeSource.get(unit)` is `unit.convert(getAsLong(), TimeUnit.NANOSECONDS)`: truncating
//! division toward zero (`-1500` ns is `-1` us), via `javacompat::time_unit`. Every
//! `NanoTimeSource` is a `TimeSource` here through a blanket impl, and any `Fn() -> i64` closure is
//! a `NanoTimeSource`, as a Java lambda would be.
//!
//! `constant(value)` returns a `NanoTimeSource` whose reading is always `value`.
//!
//! Verified by the `TIMESRC` group of `small_probe.txt` (`tests/session22.rs`): ten values (both
//! `long` extremes, negatives, a day) in all seven `TimeUnit`s.

use crate::javacompat::time_unit::TimeUnit;

/// Port of `net.minecraft.util.TimeSource`.
pub trait TimeSource {
    /// Port of `long get(TimeUnit timeUnit)`.
    fn get(&self, time_unit: TimeUnit) -> i64;
}

/// Port of `TimeSource.NanoTimeSource` (`extends LongSupplier, TimeSource`).
pub trait NanoTimeSource {
    /// Port of `LongSupplier#getAsLong()`: the reading, in nanoseconds.
    fn get_as_long(&self) -> i64;
}

impl<N: NanoTimeSource> TimeSource for N {
    /// Port of `NanoTimeSource`'s `default long get(TimeUnit timeUnit)`.
    fn get(&self, time_unit: TimeUnit) -> i64 {
        time_unit.convert_from_nanos(self.get_as_long())
    }
}

impl<F: Fn() -> i64> NanoTimeSource for F {
    fn get_as_long(&self) -> i64 {
        self()
    }
}

/// The lambda `TimeSource.constant(value)` returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Constant {
    value: i64,
}

impl NanoTimeSource for Constant {
    fn get_as_long(&self) -> i64 {
        self.value
    }
}

/// Port of `static TimeSource.NanoTimeSource constant(long value)`.
pub const fn constant(value: i64) -> Constant {
    Constant { value }
}
