//! Port of: net/minecraft/util/debugchart/LocalSampleLogger.java
//! Java class(es): net.minecraft.util.debugchart.LocalSampleLogger
//! Status: VERIFIED
//!
//! Session 24. `public class LocalSampleLogger extends AbstractSampleLogger implements
//! SampleStorage`: the debug charts' ring buffer of the last 240 samples, each `dimensions` longs.
//!
//! `useSample()` copies the finished sample into slot `(start + size) % 240`; once 240 samples
//! are held, every new one overwrites the oldest and advances `start`. `get(index, dimension)`
//! counts from the oldest held sample.
//!
//! Errors, all with Java's texts:
//!
//! * `new LocalSampleLogger(d)` with `d < 0` throws `NegativeArraySizeException: <d>` from its
//!   own `new long[dimensions]`, before the superclass check;
//! * the two-argument constructor throws the superclass's `IllegalArgumentException` when the
//!   defaults do not have `dimensions` entries;
//! * `get` throws `IndexOutOfBoundsException: <i> out of bounds for length <size>` for an index
//!   outside the held samples, and `<d> out of bounds for dimensions <n>` for a bad dimension --
//!   the index is checked first.
//!
//! Verified by the `SAMPLELOG` group of `sweep_probe.txt` (`tests/session24.rs`): scripts of
//! `logSample`, `logPartialSample`, `logFullSample` (shorter, equal and longer arrays), `get`,
//! `reset` and the wrap-around past 240 samples, for several dimension counts and defaults.

use crate::net::minecraft::util::debugchart::AbstractSampleLogger::{
    AbstractSampleLogger, SampleBuffers,
};
use crate::net::minecraft::util::debugchart::SampleStorage::SampleStorage;

/// Port of `public static final int CAPACITY = 240`.
pub const CAPACITY: i32 = 240;

/// Port of `net.minecraft.util.debugchart.LocalSampleLogger`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalSampleLogger {
    base: SampleBuffers,
    samples: Vec<Vec<i64>>,
    start: i32,
    size: i32,
}

impl LocalSampleLogger {
    /// Port of `public LocalSampleLogger(int dimensions)`: `this(dimensions, new long[dimensions])`.
    pub fn new(dimensions: i32) -> Self {
        if dimensions < 0 {
            panic!("java.lang.NegativeArraySizeException: {dimensions}");
        }
        Self::with_defaults(dimensions, vec![0; dimensions as usize])
    }

    /// Port of `public LocalSampleLogger(int dimensions, long[] defaults)`.
    pub fn with_defaults(dimensions: i32, defaults: Vec<i64>) -> Self {
        let base = SampleBuffers::new(dimensions, defaults);
        LocalSampleLogger {
            base,
            // `new long[240][dimensions]`; `dimensions` passed the superclass check, so it is >= 0.
            samples: vec![vec![0; dimensions as usize]; CAPACITY as usize],
            start: 0,
            size: 0,
        }
    }

    /// Port of `private int wrapIndex(int index)`: `index % 240`.
    fn wrap_index(index: i32) -> i32 {
        index % CAPACITY
    }
}

impl AbstractSampleLogger for LocalSampleLogger {
    fn buffers(&self) -> &SampleBuffers {
        &self.base
    }

    fn buffers_mut(&mut self) -> &mut SampleBuffers {
        &mut self.base
    }

    /// Port of `protected void useSample()`.
    fn use_sample(&mut self) {
        let next_index = Self::wrap_index(self.start + self.size) as usize;
        let n = self.base.sample.len();
        self.samples[next_index][..n].copy_from_slice(&self.base.sample);
        if self.size < CAPACITY {
            self.size += 1;
        } else {
            self.start = Self::wrap_index(self.start + 1);
        }
    }
}

impl SampleStorage for LocalSampleLogger {
    /// Port of `public int capacity()`: `samples.length`, always 240.
    fn capacity(&self) -> i32 {
        self.samples.len() as i32
    }

    /// Port of `public int size()`.
    fn size(&self) -> i32 {
        self.size
    }

    /// Port of `public long get(int index)`: `get(index, 0)`.
    fn get_i32(&self, index: i32) -> i64 {
        self.get_i32_i32(index, 0)
    }

    /// Port of `public long get(int index, int dimension)`.
    fn get_i32_i32(&self, index: i32, dimension: i32) -> i64 {
        if index >= 0 && index < self.size {
            let sample_array = &self.samples[Self::wrap_index(self.start + index) as usize];
            if dimension >= 0 && (dimension as usize) < sample_array.len() {
                sample_array[dimension as usize]
            } else {
                panic!(
                    "java.lang.IndexOutOfBoundsException: {dimension} out of bounds for dimensions {}",
                    sample_array.len()
                );
            }
        } else {
            panic!(
                "java.lang.IndexOutOfBoundsException: {index} out of bounds for length {}",
                self.size
            );
        }
    }

    /// Port of `public void reset()`: forget every held sample (the storage is not cleared).
    fn reset(&mut self) {
        self.start = 0;
        self.size = 0;
    }
}
