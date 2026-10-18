//! Port of: net/minecraft/util/debugchart/SampleLogger.java
//! Java class(es): net.minecraft.util.debugchart.SampleLogger
//! Status: VERIFIED
//!
//! Session 22. `public interface SampleLogger` -- declaration only.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

/// Port of `net.minecraft.util.debugchart.SampleLogger`.
pub trait SampleLogger {
    /// Port of `void logFullSample(long[] sample)`.
    fn log_full_sample(&mut self, sample: &[i64]);

    /// Port of `void logSample(long sample)`.
    fn log_sample(&mut self, sample: i64);

    /// Port of `void logPartialSample(long sample, int dimension)`.
    fn log_partial_sample(&mut self, sample: i64, dimension: i32);
}
