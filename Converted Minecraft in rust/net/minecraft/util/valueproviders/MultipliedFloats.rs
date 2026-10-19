//! Port of: net/minecraft/util/valueproviders/MultipliedFloats.java
//! Java class(es): net.minecraft.util.valueproviders.MultipliedFloats
//! Status: VERIFIED
//!
//! Session 24. `public class MultipliedFloats implements SampledFloat`: the product of several
//! [`SampledFloat`]s.
//!
//! ```java
//! float result = 1.0F;
//! for (SampledFloat value : this.values) result *= value.sample(random);
//! return result;
//! ```
//!
//! * the factors are sampled in array order, each drawing from the same `random`, so the order
//!   decides which draws each factor gets;
//! * the product is `float` multiplication, left to right, from `1.0F`: no factors gives `1.0`,
//!   and a NaN or an overflow propagates exactly as Java's `fmul` does;
//! * `toString()` is `"MultipliedFloats" + Arrays.toString(values)`: `[a, b]`, each element by its
//!   own `toString()`, and `[]` for none.
//!
//! # `toString` of the factors
//!
//! Every Java object has a `toString`; a Rust trait object does not. Rather than add a method to
//! the session-22 `SampledFloat` trait (whose shape is verified against the jar), this file takes
//! factors that are `SampledFloat + Display` -- the [`DisplaySampledFloat`] blanket trait -- so
//! `Arrays.toString` has each element's text. Vanilla's factors are `FloatProvider`s, whose
//! `toString` is part of their own ports.
//!
//! The varargs array is the caller's in Java (`this.values = values`, no copy); here the factors
//! are moved in. Nothing in vanilla writes to the array afterwards.
//!
//! Verified by the `MULTFLOAT` group of `sweep_probe.txt` (`tests/session24.rs`): products over
//! factors that draw from a seeded `LegacyRandomSource`, special values, and `toString`.

use crate::net::minecraft::util::valueproviders::SampledFloat::SampledFloat;
use crate::net::minecraft::util::RandomSource::RandomSource;

/// A [`SampledFloat`] with a Java `toString()`: what `MultipliedFloats` needs of each factor.
pub trait DisplaySampledFloat: SampledFloat + std::fmt::Display {}

impl<T: SampledFloat + std::fmt::Display> DisplaySampledFloat for T {}

/// Port of `net.minecraft.util.valueproviders.MultipliedFloats`.
pub struct MultipliedFloats {
    values: Vec<Box<dyn DisplaySampledFloat>>,
}

impl MultipliedFloats {
    /// Port of `public MultipliedFloats(SampledFloat... values)`.
    pub fn new(values: Vec<Box<dyn DisplaySampledFloat>>) -> Self {
        MultipliedFloats { values }
    }
}

impl SampledFloat for MultipliedFloats {
    /// Port of `public float sample(RandomSource random)`.
    fn sample(&self, random: &mut dyn RandomSource) -> f32 {
        let mut result = 1.0f32;
        for value in &self.values {
            result *= value.sample(random);
        }
        result
    }
}

impl std::fmt::Display for MultipliedFloats {
    /// Port of `toString()`: `"MultipliedFloats" + Arrays.toString(this.values)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MultipliedFloats[")?;
        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{value}")?;
        }
        f.write_str("]")
    }
}
