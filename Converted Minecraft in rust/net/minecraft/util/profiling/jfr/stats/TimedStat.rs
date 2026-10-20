//! Port of: `net/minecraft/util/profiling/jfr/stats/TimedStat.java`
//! Java class(es): `net.minecraft.util.profiling.jfr.stats.TimedStat`
//! Status: VERIFIED
//!
//! The Java interface declares one abstract accessor and no fields, constructors, defaults, or
//! static members. The return type is represented by the JDK 25 `Duration` value subset in
//! `javacompat::java_time`, which preserves the value operations used by timed-stat records.

use crate::javacompat::java_time::Duration;

/// Port of Java's `TimedStat` interface.
pub trait TimedStat {
    /// Port of `TimedStat#duration()`.
    fn duration(&self) -> &Duration;
}
