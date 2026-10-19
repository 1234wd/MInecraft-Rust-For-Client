//! Port of: net/minecraft/util/profiling/EmptyProfileResults.java
//! Java class(es): net.minecraft.util.profiling.EmptyProfileResults
//! Status: VERIFIED
//!
//! Session 24. `public class EmptyProfileResults implements ProfileResults`: the result of a
//! profiler that recorded nothing. One shared instance, [`EMPTY`] (the constructor is private);
//! every method returns its type's empty value -- no times, `saveResults` false without touching
//! the path, every start/end 0, the report "". The two inherited defaults are therefore 0.
//!
//! Verified by the `PROFRES` group of `sweep_probe.txt` (`tests/session24.rs`).

use crate::net::minecraft::util::profiling::ProfileResults::ProfileResults;
use crate::net::minecraft::util::profiling::ResultField::ResultField;

/// Port of `net.minecraft.util.profiling.EmptyProfileResults`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyProfileResults {
    _private: (),
}

/// Port of `public static final EmptyProfileResults EMPTY = new EmptyProfileResults()`.
pub const EMPTY: EmptyProfileResults = EmptyProfileResults { _private: () };

impl ProfileResults for EmptyProfileResults {
    /// Port of `getTimes(String path)`: `Collections.emptyList()`.
    fn get_times(&self, _path: &str) -> Vec<ResultField> {
        Vec::new()
    }

    /// Port of `saveResults(Path file)`: `false`, and nothing is written.
    fn save_results(&self, _file: &std::path::Path) -> bool {
        false
    }

    /// Port of `getStartTimeNano()`: `0L`.
    fn get_start_time_nano(&self) -> i64 {
        0
    }

    /// Port of `getStartTimeTicks()`: `0`.
    fn get_start_time_ticks(&self) -> i32 {
        0
    }

    /// Port of `getEndTimeNano()`: `0L`.
    fn get_end_time_nano(&self) -> i64 {
        0
    }

    /// Port of `getEndTimeTicks()`: `0`.
    fn get_end_time_ticks(&self) -> i32 {
        0
    }

    /// Port of `getProfilerResults()`: `""`.
    fn get_profiler_results(&self) -> String {
        String::new()
    }
}
