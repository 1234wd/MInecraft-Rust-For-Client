//! Port of: com/mojang/blaze3d/pipeline/DepthStencilState.java
//! Java class(es): com.mojang.blaze3d.pipeline.DepthStencilState
//! Status: VERIFIED
//!
//! Session 22. `public record DepthStencilState(CompareOp depthTest, boolean writeDepth,
//! float depthBiasScaleFactor, float depthBiasConstant)` with a two-argument constructor (no depth
//! bias) and a `DEFAULT` of `GREATER_THAN_OR_EQUAL`, writing depth.
//!
//! # `hashCode` is not portable, by design of the JVM
//!
//! The record hash folds `depthTest.hashCode()`, and an enum's `hashCode` is `Object`'s identity
//! hash: a different number on every JVM run. There is no Java value to match, so no
//! `java_hash_code` is ported; the Rust `Hash` impl is consistent with `equals`, which is all a
//! `HashMap` needs. The probe writes `-` for these hashes rather than an unreproducible number.
//!
//! # Record members
//!
//! `equals` compares the enum by identity (`==`, i.e. by variant here), the boolean directly and
//! both floats with `Float.compare` (hand-written on [`java_object::equals_f32`]); `toString`
//! prints the enum's `name()` -- `CompareOp` does not override `toString` -- and the floats with
//! Java's `Float.toString`.
//!
//! Verified by the `REC`, `RECEQ` and `DSDEFAULT` groups of `small_probe.txt`
//! (`tests/session22.rs`): `DEFAULT`, both constructors, a `-0.0` bias, and two instances that
//! differ only in a NaN payload.

use crate::com::mojang::blaze3d::platform::CompareOp::CompareOp;
use crate::javacompat::java_lang::float_to_string::float_to_string;
use crate::javacompat::java_object;

/// Port of `com.mojang.blaze3d.pipeline.DepthStencilState`.
#[derive(Debug, Clone, Copy)]
pub struct DepthStencilState {
    depth_test: CompareOp,
    write_depth: bool,
    depth_bias_scale_factor: f32,
    depth_bias_constant: f32,
}

impl DepthStencilState {
    /// Port of `DepthStencilState.DEFAULT` = `new DepthStencilState(GREATER_THAN_OR_EQUAL, true)`.
    pub const DEFAULT: DepthStencilState = DepthStencilState::new_without_bias(CompareOp::GreaterThanOrEqual, true);

    /// Port of the canonical constructor.
    pub const fn new(
        depth_test: CompareOp,
        write_depth: bool,
        depth_bias_scale_factor: f32,
        depth_bias_constant: f32,
    ) -> Self {
        DepthStencilState {
            depth_test,
            write_depth,
            depth_bias_scale_factor,
            depth_bias_constant,
        }
    }

    /// Port of `DepthStencilState(CompareOp depthTest, boolean depthWrite)`: both biases `0.0F`.
    pub const fn new_without_bias(depth_test: CompareOp, depth_write: bool) -> Self {
        DepthStencilState::new(depth_test, depth_write, 0.0, 0.0)
    }

    /// Port of the accessor `depthTest()`.
    pub const fn depth_test(&self) -> CompareOp {
        self.depth_test
    }

    /// Port of the accessor `writeDepth()`.
    pub const fn write_depth(&self) -> bool {
        self.write_depth
    }

    /// Port of the accessor `depthBiasScaleFactor()`.
    pub const fn depth_bias_scale_factor(&self) -> f32 {
        self.depth_bias_scale_factor
    }

    /// Port of the accessor `depthBiasConstant()`.
    pub const fn depth_bias_constant(&self) -> f32 {
        self.depth_bias_constant
    }
}

impl PartialEq for DepthStencilState {
    /// Port of the record's generated `equals`.
    fn eq(&self, other: &Self) -> bool {
        self.depth_test == other.depth_test
            && self.write_depth == other.write_depth
            && java_object::equals_f32(self.depth_bias_scale_factor, other.depth_bias_scale_factor)
            && java_object::equals_f32(self.depth_bias_constant, other.depth_bias_constant)
    }
}

impl Eq for DepthStencilState {}

impl std::hash::Hash for DepthStencilState {
    /// Consistent with `equals`; NOT Java's (identity-based) hash -- see the module docs.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.depth_test as i32).hash(state);
        self.write_depth.hash(state);
        java_object::float_to_int_bits(self.depth_bias_scale_factor).hash(state);
        java_object::float_to_int_bits(self.depth_bias_constant).hash(state);
    }
}

impl std::fmt::Display for DepthStencilState {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DepthStencilState[depthTest={}, writeDepth={}, depthBiasScaleFactor={}, depthBiasConstant={}]",
            self.depth_test.name(),
            self.write_depth,
            float_to_string(self.depth_bias_scale_factor),
            float_to_string(self.depth_bias_constant)
        )
    }
}
