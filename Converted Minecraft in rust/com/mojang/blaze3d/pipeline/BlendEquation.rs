//! Port of: com/mojang/blaze3d/pipeline/BlendEquation.java
//! Java class(es): com.mojang.blaze3d.pipeline.BlendEquation
//! Status: VERIFIED
//!
//! Session 22. `public record BlendEquation(BlendFactor sourceFactor, BlendFactor destFactor,
//! BlendOp op)` -- components only, no body.
//!
//! # `hashCode` is not portable, by design of the JVM
//!
//! All three components are enums, whose `hashCode` is the per-run identity hash, so the record's
//! hash differs on every JVM run. No `java_hash_code` is ported; the derived Rust `Hash` is
//! consistent with `equals`. The probe writes `-` for these hashes.
//!
//! `equals` (each enum by identity, i.e. by variant) is the derived one; `toString` prints each
//! enum's `name()` -- neither enum overrides `toString`.
//!
//! Verified by the `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`),
//! including a pair that differs only in which factor is source and which destination.

use crate::com::mojang::blaze3d::platform::BlendFactor::BlendFactor;
use crate::com::mojang::blaze3d::platform::BlendOp::BlendOp;

/// Port of `com.mojang.blaze3d.pipeline.BlendEquation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendEquation {
    source_factor: BlendFactor,
    dest_factor: BlendFactor,
    op: BlendOp,
}

impl BlendEquation {
    /// Port of the canonical constructor.
    pub const fn new(source_factor: BlendFactor, dest_factor: BlendFactor, op: BlendOp) -> Self {
        BlendEquation {
            source_factor,
            dest_factor,
            op,
        }
    }

    /// Port of the accessor `sourceFactor()`.
    pub const fn source_factor(&self) -> BlendFactor {
        self.source_factor
    }

    /// Port of the accessor `destFactor()`.
    pub const fn dest_factor(&self) -> BlendFactor {
        self.dest_factor
    }

    /// Port of the accessor `op()`.
    pub const fn op(&self) -> BlendOp {
        self.op
    }
}

impl std::fmt::Display for BlendEquation {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BlendEquation[sourceFactor={}, destFactor={}, op={}]",
            self.source_factor.name(),
            self.dest_factor.name(),
            self.op.name()
        )
    }
}
