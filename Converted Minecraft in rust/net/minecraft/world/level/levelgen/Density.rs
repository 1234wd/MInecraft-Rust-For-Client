//! Port of: net/minecraft/world/level/levelgen/Density.java
//! Java class(es): net.minecraft.world.level.levelgen.Density
//! Status: VERIFIED
//!
//! Three `double` constants and nothing else -- a namespace class, not instantiable in Java
//! either (it has an implicit public constructor, but nothing ever makes one).
//!
//! # `SURFACE` IS **EXACTLY** `0.0`, AND THE OTHER TWO ARE NOT INTEGERS
//!
//! ```java
//! public static final double SURFACE = 0.0;
//! public static final double UNRECOVERABLY_DENSE = 64.0;
//! public static final double UNRECOVERABLY_THIN = -64.0;
//! ```
//!
//! These are the sentinels the density function is allowed to return. `UNRECOVERABLY_DENSE` and
//! `UNRECOVERABLY_THIN` are **clamped to** by `DensityFunction`, not merely compared against: a
//! density function that exceeds `+64` has its output collapsed to exactly `64.0`, which is why
//! the constant is a specific double rather than a range. Any port that used `>= 64` at a call
//! site instead of clamping at the producer would diverge for values in between.
//!
//! All three are verified against the jar by `_porting/tests/consts.rs`, which records floats
//! and doubles as **raw bits** so a `-0.0`/`0.0` mix-up cannot hide.

/// Java: `Density.SURFACE` = `0.0`.
pub const SURFACE: f64 = 0.0;

/// Java: `Density.UNRECOVERABLY_DENSE` = `64.0`.
pub const UNRECOVERABLY_DENSE: f64 = 64.0;

/// Java: `Density.UNRECOVERABLY_THIN` = `-64.0`.
pub const UNRECOVERABLY_THIN: f64 = -64.0;

/// Port of `net.minecraft.world.level.levelgen.Density`.
///
/// Java's implicit public constructor is not reproduced: nothing instantiates this class, and a
/// unit struct would imply a value users are meant to have.
pub struct Density;