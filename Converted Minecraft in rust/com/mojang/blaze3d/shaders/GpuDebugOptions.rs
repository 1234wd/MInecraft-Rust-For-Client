//! Port of: com/mojang/blaze3d/shaders/GpuDebugOptions.java
//! Java class(es): com.mojang.blaze3d.shaders.GpuDebugOptions
//! Status: VERIFIED
//!
//! Session 22. `public record GpuDebugOptions(int logLevel, boolean synchronousLogs, boolean useLabels, boolean useValidationLayers)` -- components only, no body.
//!
//! # What the record generates, ported explicitly
//!
//! * the canonical constructor and one accessor per component;
//! * `equals` -- component-wise, which `#[derive(PartialEq, Eq)]` is for these component types;
//! * `hashCode` -- [`GpuDebugOptions::java_hash_code`], the record fold `acc * 31 + h` from `0` over the
//!   components' boxed hashes (see [`crate::javacompat::java_object`]);
//! * `toString` -- the `Display` impl, `GpuDebugOptions[logLevel=..., synchronousLogs=..., useLabels=..., useValidationLayers=...]`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`):
//! `toString`, `hashCode` and the full `equals` matrix, over five to nine instances per record that
//! include an equal-but-distinct duplicate and the components' extreme values.

use crate::javacompat::java_object;

/// Port of `com.mojang.blaze3d.shaders.GpuDebugOptions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GpuDebugOptions {
    log_level: i32,
    synchronous_logs: bool,
    use_labels: bool,
    use_validation_layers: bool,
}

impl GpuDebugOptions {
    /// Port of the canonical constructor.
    pub fn new(
        log_level: i32,
        synchronous_logs: bool,
        use_labels: bool,
        use_validation_layers: bool,
    ) -> Self {
        GpuDebugOptions {
            log_level,
            synchronous_logs,
            use_labels,
            use_validation_layers,
        }
    }

    /// Port of the accessor `logLevel()`.
    pub fn log_level(&self) -> i32 {
        self.log_level
    }

    /// Port of the accessor `synchronousLogs()`.
    pub fn synchronous_logs(&self) -> bool {
        self.synchronous_logs
    }

    /// Port of the accessor `useLabels()`.
    pub fn use_labels(&self) -> bool {
        self.use_labels
    }

    /// Port of the accessor `useValidationLayers()`.
    pub fn use_validation_layers(&self) -> bool {
        self.use_validation_layers
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            self.log_level,
            java_object::hash_bool(self.synchronous_logs),
            java_object::hash_bool(self.use_labels),
            java_object::hash_bool(self.use_validation_layers),
        ])
    }
}

impl std::fmt::Display for GpuDebugOptions {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GpuDebugOptions[logLevel={}, synchronousLogs={}, useLabels={}, useValidationLayers={}]", self.log_level, self.synchronous_logs, self.use_labels, self.use_validation_layers)
    }
}
