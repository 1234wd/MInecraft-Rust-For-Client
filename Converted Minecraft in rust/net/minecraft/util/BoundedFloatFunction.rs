//! Port of: net/minecraft/util/BoundedFloatFunction.java
//! Java class(es): net.minecraft.util.BoundedFloatFunction, BoundedFloatFunction::<3 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 22. `public interface BoundedFloatFunction<C>`: a function to `float` that also states
//! its range (`minValue()`/`maxValue()`), with an `IDENTITY`, a `constant` factory and a default
//! `comap` that pre-composes an input mapping.
//!
//! * `IDENTITY` is `BoundedFloatFunction<Float>`: `apply(v)` returns `v` unchanged (NaN and `-0.0`
//!   included), with range `[-Infinity, Infinity]`.
//! * `constant(value)` returns `value` for any input, and `value` as both bounds.
//! * `comap(function)` applies `function` first, then this; the bounds are this function's.
//!
//! The three anonymous classes are [`Identity`], [`Constant`] and [`Comap`]. `comap` consumes
//! `self` (the Java lambda captures `this` by reference; here it owns it).
//!
//! Verified by the `BOUNDEDF` group of `small_probe.txt` (`tests/session22.rs`).

use std::marker::PhantomData;

/// Port of `net.minecraft.util.BoundedFloatFunction<C>`.
pub trait BoundedFloatFunction<C> {
    /// Port of `float apply(C c)`.
    fn apply(&self, c: C) -> f32;

    /// Port of `float minValue()`.
    fn min_value(&self) -> f32;

    /// Port of `float maxValue()`.
    fn max_value(&self) -> f32;

    /// Port of `default <C2> BoundedFloatFunction<C2> comap(Function<C2, C> function)`.
    fn comap<C2, F: Fn(C2) -> C>(self, function: F) -> Comap<Self, F, C>
    where
        Self: Sized,
    {
        Comap {
            outer: self,
            function,
            _input: PhantomData,
        }
    }
}

/// The anonymous class behind `BoundedFloatFunction.IDENTITY`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Identity;

impl BoundedFloatFunction<f32> for Identity {
    fn apply(&self, value: f32) -> f32 {
        value
    }

    fn min_value(&self) -> f32 {
        f32::NEG_INFINITY
    }

    fn max_value(&self) -> f32 {
        f32::INFINITY
    }
}

/// Port of `BoundedFloatFunction.IDENTITY`.
pub const IDENTITY: Identity = Identity;

/// The anonymous class `BoundedFloatFunction.constant(value)` returns.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constant {
    value: f32,
}

impl<C> BoundedFloatFunction<C> for Constant {
    fn apply(&self, _c: C) -> f32 {
        self.value
    }

    fn min_value(&self) -> f32 {
        self.value
    }

    fn max_value(&self) -> f32 {
        self.value
    }
}

/// Port of `static <C> BoundedFloatFunction<C> constant(float value)`.
pub const fn constant(value: f32) -> Constant {
    Constant { value }
}

/// The anonymous class `comap` returns: `outer.apply(function.apply(c2))`, `outer`'s bounds.
pub struct Comap<B, F, C> {
    outer: B,
    function: F,
    _input: PhantomData<fn(C)>,
}

impl<C, C2, B: BoundedFloatFunction<C>, F: Fn(C2) -> C> BoundedFloatFunction<C2> for Comap<B, F, C> {
    fn apply(&self, c2: C2) -> f32 {
        self.outer.apply((self.function)(c2))
    }

    fn min_value(&self) -> f32 {
        self.outer.min_value()
    }

    fn max_value(&self) -> f32 {
        self.outer.max_value()
    }
}
