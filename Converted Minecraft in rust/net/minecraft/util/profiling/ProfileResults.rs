//! Port of: net/minecraft/util/profiling/ProfileResults.java
//! Java class(es): net.minecraft.util.profiling.ProfileResults
//! Status: VERIFIED
//!
//! Session 24. `public interface ProfileResults`: what a finished profiler run reports. Seven
//! abstract methods, two default ones, one static helper and one constant.
//!
//! * `PATH_SEPARATOR = '\u001e'` (ASCII Record Separator) joins the sections of a profiler path;
//!   [`demangle_path`] turns it into `.` for display -- every occurrence, nothing else touched;
//! * `getNanoDuration()` and `getTickDuration()` are `end - start` in `long` and `int`
//!   arithmetic, which WRAP in Java; here `wrapping_sub`.
//!
//! `saveResults(Path)` takes `&std::path::Path`, as `Dumpable` (session 22) does: an interface
//! signature, with no `Path` value semantics involved. `List<ResultField>` is a `Vec`.
//!
//! Verified by the `PROFRES` group of `sweep_probe.txt` (`tests/session24.rs`): the defaults and
//! `demanglePath` against an implementation in the probe, plus the `API24` reflection shape.

use crate::net::minecraft::util::profiling::ResultField::ResultField;

/// Port of `char PATH_SEPARATOR = '\u001e'`.
pub const PATH_SEPARATOR: char = '\u{1e}';

/// Port of `net.minecraft.util.profiling.ProfileResults`.
pub trait ProfileResults {
    /// Port of `List<ResultField> getTimes(String path)`.
    fn get_times(&self, path: &str) -> Vec<ResultField>;

    /// Port of `boolean saveResults(Path file)`.
    fn save_results(&self, file: &std::path::Path) -> bool;

    /// Port of `long getStartTimeNano()`.
    fn get_start_time_nano(&self) -> i64;

    /// Port of `int getStartTimeTicks()`.
    fn get_start_time_ticks(&self) -> i32;

    /// Port of `long getEndTimeNano()`.
    fn get_end_time_nano(&self) -> i64;

    /// Port of `int getEndTimeTicks()`.
    fn get_end_time_ticks(&self) -> i32;

    /// Port of `default long getNanoDuration()`: `getEndTimeNano() - getStartTimeNano()`.
    fn get_nano_duration(&self) -> i64 {
        self.get_end_time_nano()
            .wrapping_sub(self.get_start_time_nano())
    }

    /// Port of `default int getTickDuration()`: `getEndTimeTicks() - getStartTimeTicks()`.
    fn get_tick_duration(&self) -> i32 {
        self.get_end_time_ticks()
            .wrapping_sub(self.get_start_time_ticks())
    }

    /// Port of `String getProfilerResults()`.
    fn get_profiler_results(&self) -> String;
}

/// Port of `static String demanglePath(String path)`: `path.replace('\u001e', '.')`.
pub fn demangle_path(path: &str) -> String {
    path.replace(PATH_SEPARATOR, ".")
}
