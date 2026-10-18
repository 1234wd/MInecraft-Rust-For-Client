//! Port of: net/minecraft/world/attribute/LerpFunction.java
//! Java class(es): net.minecraft.world.attribute.LerpFunction
//! Status: VERIFIED
//!
//! Session 22. `public interface LerpFunction<T>`: how an environment attribute blends between two
//! values, `T apply(float alpha, T from, T to)`, with six static factories and a `CONSTANT`.
//!
//! | factory | behaviour |
//! |---|---|
//! | `ofFloat()` | `Mth.lerp(alpha, from, to)` |
//! | `ofInteger()` | `Mth.lerpInt(alpha, from, to)` |
//! | `ofDegrees(maxDelta)` | `delta = Mth.wrapDegrees(to - from)`; `Math.abs(delta) >= maxDelta ? to : from + alpha * delta` |
//! | `ofStep(threshold)` | `alpha >= threshold ? to : from` |
//! | `ofConstant()` | `CONSTANT`, which is `ofStep(1.0F)` |
//! | `ofColor()` | `ARGB.srgbLerp(alpha, from, to)` |
//!
//! `ofDegrees` SNAPS to `to` once the gap is at least `maxDelta`, rather than clamping the step;
//! `Math.abs` is Java's (`javacompat::java_lang::abs_f32`). `ofStep` with a NaN alpha returns `from`
//! (`NaN >= t` is false). Any `Fn(f32, T, T) -> T` closure is a `LerpFunction<T>`; the step
//! function is [`Step`], so `CONSTANT` is a `const` and `ofConstant()` returns it.
//!
//! Verified by the `LERPF`, `LERPI` and `LERPCONST` groups of `small_probe.txt`
//! (`tests/session22.rs`): every factory over eight alphas (including `-0.5`, `1.5`, NaN) and
//! seven float / six int pairs (wraps past 360, `int` extremes, ARGB colours).

use crate::javacompat::java_lang;
use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::util::ARGB;

/// Port of `net.minecraft.world.attribute.LerpFunction<T>`.
pub trait LerpFunction<T> {
    /// Port of `T apply(float alpha, T from, T to)`.
    fn apply(&self, alpha: f32, from: T, to: T) -> T;
}

impl<T, F: Fn(f32, T, T) -> T> LerpFunction<T> for F {
    fn apply(&self, alpha: f32, from: T, to: T) -> T {
        self(alpha, from, to)
    }
}

/// The lambda `ofStep(threshold)` returns: `alpha >= threshold ? to : from`, for any `T`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step {
    threshold: f32,
}

impl<T> LerpFunction<T> for Step {
    fn apply(&self, alpha: f32, from: T, to: T) -> T {
        if alpha >= self.threshold {
            to
        } else {
            from
        }
    }
}

/// Port of `LerpFunction.CONSTANT` = `ofStep(1.0F)`.
pub const CONSTANT: Step = of_step(1.0);

/// Port of `static LerpFunction<Float> ofFloat()`: `Mth::lerp`.
pub fn of_float() -> impl LerpFunction<f32> {
    |alpha: f32, from: f32, to: f32| Mth::lerp_f32(alpha, from, to)
}

/// Port of `static LerpFunction<Integer> ofInteger()`: `Mth::lerpInt`.
pub fn of_integer() -> impl LerpFunction<i32> {
    |alpha: f32, from: i32, to: i32| Mth::lerp_int(alpha, from, to)
}

/// Port of `static LerpFunction<Float> ofDegrees(float maxDelta)`.
pub fn of_degrees(max_delta: f32) -> impl LerpFunction<f32> {
    move |alpha: f32, from: f32, to: f32| {
        let delta = Mth::wrap_degrees_f32(to - from);
        if java_lang::abs_f32(delta) >= max_delta {
            to
        } else {
            from + alpha * delta
        }
    }
}

/// Port of `static <T> LerpFunction<T> ofConstant()`: `CONSTANT`.
pub const fn of_constant() -> Step {
    CONSTANT
}

/// Port of `static <T> LerpFunction<T> ofStep(float threshold)`.
pub const fn of_step(threshold: f32) -> Step {
    Step { threshold }
}

/// Port of `static LerpFunction<Integer> ofColor()`: `ARGB::srgbLerp`.
pub fn of_color() -> impl LerpFunction<i32> {
    |alpha: f32, from: i32, to: i32| ARGB::srgb_lerp(alpha, from, to)
}
