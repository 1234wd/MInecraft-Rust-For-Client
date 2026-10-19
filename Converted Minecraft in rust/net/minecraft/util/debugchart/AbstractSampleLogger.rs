//! Port of: net/minecraft/util/debugchart/AbstractSampleLogger.java
//! Java class(es): net.minecraft.util.debugchart.AbstractSampleLogger
//! Status: VERIFIED
//!
//! Session 24. `public abstract class AbstractSampleLogger implements SampleLogger`: the shared
//! half of the debug-chart loggers. It owns the sample being assembled (`long[] sample`) and the
//! values a sample is reset to (`long[] defaults`), implements the three [`SampleLogger`] methods,
//! and leaves `useSample()` -- what to do with a finished sample -- to the subclass.
//!
//! # Shape of the port
//!
//! Java's template method becomes a trait: a subclass implements [`AbstractSampleLogger`] by
//! exposing its [`SampleBuffers`] (the two `protected final` arrays) and `use_sample`, and the
//! blanket `impl<T: AbstractSampleLogger> SampleLogger for T` supplies `logFullSample`,
//! `logSample` and `logPartialSample` exactly as the abstract class does. `resetSample` is
//! `protected` and overridable in Java; nothing overrides it, and it is the trait's provided
//! method here.
//!
//! # Java's array errors, reproduced
//!
//! * the constructor throws `IllegalArgumentException: defaults have incorrect length of <n>`;
//! * `logFullSample` is `System.arraycopy(sample, 0, this.sample, 0, sample.length)`: a LONGER
//!   array throws HotSpot's `ArrayIndexOutOfBoundsException: arraycopy: last destination index
//!   <n> out of bounds for long[<d>]` before anything is copied, and a SHORTER one overwrites
//!   only its prefix, so the rest of the sample keeps whatever `logPartialSample` put there;
//! * `logSample` writes `sample[0]`, so a zero-dimension logger throws
//!   `ArrayIndexOutOfBoundsException: Index 0 out of bounds for length 0`;
//! * `logPartialSample` accepts dimensions `1..length` only (dimension 0 is `logSample`'s) and
//!   throws `IndexOutOfBoundsException: <d> out of bounds for dimensions <n>` otherwise.
//!
//! # `defaults` is the caller's array in Java
//!
//! The constructor stores `defaults` without copying it, so a caller that later wrote to its
//! array would change what samples reset to. No vanilla caller can: all four construction sites
//! (`DebugScreenOverlay`, `GameTestServer`, `DedicatedServer`) use the one-argument
//! constructors, which allocate a fresh `new long[dimensions]` and keep no reference. The port
//! takes the array by value; the difference is unobservable in the game.
//!
//! Verified through [`LocalSampleLogger`](super::LocalSampleLogger), its only ported subclass, by
//! the `SAMPLELOG` group of `sweep_probe.txt` (`tests/session24.rs`).

use crate::net::minecraft::util::debugchart::SampleLogger::SampleLogger;

/// The two `protected final long[]` fields of `AbstractSampleLogger`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleBuffers {
    /// Port of `protected final long[] defaults`.
    pub defaults: Vec<i64>,
    /// Port of `protected final long[] sample`.
    pub sample: Vec<i64>,
}

impl SampleBuffers {
    /// Port of `protected AbstractSampleLogger(int dimensions, long[] defaults)`.
    pub fn new(dimensions: i32, defaults: Vec<i64>) -> Self {
        if defaults.len() as i64 != i64::from(dimensions) {
            panic!(
                "java.lang.IllegalArgumentException: defaults have incorrect length of {}",
                defaults.len()
            );
        }
        SampleBuffers {
            // `dimensions == defaults.length` here, so it is not negative.
            sample: vec![0; dimensions as usize],
            defaults,
        }
    }
}

/// Port of `AbstractSampleLogger`'s abstract and overridable members, for its subclasses.
pub trait AbstractSampleLogger {
    /// The subclass's view of the inherited fields.
    fn buffers(&self) -> &SampleBuffers;

    /// Mutable access to the inherited fields.
    fn buffers_mut(&mut self) -> &mut SampleBuffers;

    /// Port of `protected abstract void useSample()`: consume `buffers().sample`.
    fn use_sample(&mut self);

    /// Port of `protected void resetSample()`: copy `defaults` over `sample`.
    fn reset_sample(&mut self) {
        let b = self.buffers_mut();
        let n = b.defaults.len();
        b.sample[..n].copy_from_slice(&b.defaults);
    }
}

impl<T: AbstractSampleLogger> SampleLogger for T {
    /// Port of `public void logFullSample(long[] sample)`.
    fn log_full_sample(&mut self, sample: &[i64]) {
        let dims = self.buffers().sample.len();
        if sample.len() > dims {
            // HotSpot checks the destination range before copying anything.
            panic!(
                "java.lang.ArrayIndexOutOfBoundsException: arraycopy: last destination index {} out of bounds for long[{dims}]",
                sample.len()
            );
        }
        self.buffers_mut().sample[..sample.len()].copy_from_slice(sample);
        self.use_sample();
        self.reset_sample();
    }

    /// Port of `public void logSample(long sample)`.
    fn log_sample(&mut self, sample: i64) {
        let b = self.buffers_mut();
        if b.sample.is_empty() {
            panic!("java.lang.ArrayIndexOutOfBoundsException: Index 0 out of bounds for length 0");
        }
        b.sample[0] = sample;
        self.use_sample();
        self.reset_sample();
    }

    /// Port of `public void logPartialSample(long sample, int dimension)`.
    fn log_partial_sample(&mut self, sample: i64, dimension: i32) {
        let b = self.buffers_mut();
        if dimension >= 1 && (dimension as usize) < b.sample.len() {
            b.sample[dimension as usize] = sample;
        } else {
            panic!(
                "java.lang.IndexOutOfBoundsException: {dimension} out of bounds for dimensions {}",
                b.sample.len()
            );
        }
    }
}
