//! Port of: net/minecraft/util/ToFloatFunction.java
//! Java class(es): net.minecraft.util.ToFloatFunction
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A `@FunctionalInterface` mirroring `java.util.function.ToFloatFunction`. It exists because
//! the vanilla particle and biome samplers take a lambda that widens to `float`, and Java will
//! not box a lambda into `Function<T, Float>` without an allocation.
//!
//! # NAMED `APPLY_AS_FLOAT`, NOT `APPLY`
//!
//! `apply` would collide with [`Fn`](crate::javacompat::Fn) in any scope where both are in
//! scope, and the collision would be resolved by the import -- silently binding to the wrong
//! one. `apply_as_float` cannot collide, and it matches the method's own name, so a reader can
//! grep for it.

/// Port of `net.minecraft.util.ToFloatFunction`.
pub trait ToFloatFunction<T> {
    /// Port of `ToFloatFunction#applyAsFloat(T)`.
    fn apply_as_float(&self, value: T) -> f32;
}