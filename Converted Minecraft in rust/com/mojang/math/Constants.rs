//! Port of: com/mojang/math/Constants.java
//! Java class(es): com.mojang.math.Constants
//! Status: PORTED
//!
//! Four `float` constants. `com.mojang.*` is outside the jar sweep that produced `consts.txt`,
//! so there is no dump group; the raw bits were instead cross-checked against a one-off run of
//! the 26.2 jar (values pinned in `_porting/tests/session19_e.rs`).
//!
//! # The three spellings are three different roundings
//!
//! | constant | Java expression | evaluated as | bits |
//! |---|---|---|---|
//! | `PI` | `(float) Math.PI` | double -> float | `0x40490fdb` |
//! | `RAD_TO_DEG` | `180.0F / (float) Math.PI` | **float** division | `0x42652ee0` |
//! | `DEG_TO_RAD` | `(float) (Math.PI / 180.0)` | **double** division, then narrow | `0x3c8efa35` |
//!
//! `RAD_TO_DEG` divides by the already-rounded float pi, so it is *not* the float nearest
//! `180 / pi` (that would be `0x42652ee1`); computing it in `f64` and narrowing would be one
//! ulp off. Each Rust expression below mirrors its Java expression's width exactly. Rust's
//! `f64 as f32` rounds to nearest-even like Java's `d2f`, and const evaluation uses IEEE
//! arithmetic, so the folded values are the ones javac folded.

/// Java: `Constants.PI` = `(float) Math.PI`.
pub const PI: f32 = std::f64::consts::PI as f32;
/// Java: `Constants.RAD_TO_DEG` = `180.0F / (float) Math.PI` (float division).
pub const RAD_TO_DEG: f32 = 180.0_f32 / (std::f64::consts::PI as f32);
/// Java: `Constants.DEG_TO_RAD` = `(float) (Math.PI / 180.0)` (double division, then narrowed).
pub const DEG_TO_RAD: f32 = (std::f64::consts::PI / 180.0_f64) as f32;
/// Java: `Constants.EPSILON` = `1.0E-6F`.
pub const EPSILON: f32 = 1.0E-6_f32;
