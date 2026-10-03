//! Port of: net/minecraft/world/level/levelgen/MarsagliaPolarGaussian.java
//! Java class(es): net.minecraft.world.level.levelgen.MarsagliaPolarGaussian
//! Status: VERIFIED
//!
//! Cached Marsaglia polar Gaussian. The caching behaviour is load-bearing: the
//! golden transcripts in `_porting/test-data/random.txt` depend on the exact
//! interleaving of `nextDouble()` calls and on the reuse of the spare value.
//!
//! Java holds a `public final RandomSource randomSource` field. Every construction
//! site in vanilla passes `this`, which would make this struct self-referential in
//! Rust. We therefore take the source as a METHOD parameter instead -- the only
//! observable difference is that the field is no longer publicly readable, and
//! nothing in vanilla reads it. See _porting/DESIGN_DECISIONS.md
//! (#no-self-referential-structs).

use crate::net::minecraft::util::RandomSource::RandomSource;

#[derive(Clone, Copy, Debug, Default)]
pub struct MarsagliaPolarGaussian {
    next_next_gaussian: f64,
    have_next_next_gaussian: bool,
}

impl MarsagliaPolarGaussian {
    /// Port of `new MarsagliaPolarGaussian(RandomSource)`. Nothing is read from the
    /// source here, so no source parameter is needed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Port of `MarsagliaPolarGaussian#reset()`.
    #[inline]
    pub fn reset(&mut self) {
        self.have_next_next_gaussian = false;
    }

    /// Port of `MarsagliaPolarGaussian#nextGaussian()`.
    ///
    /// Rejection condition `radiusSquared >= 1.0 || radiusSquared == 0.0` is
    /// verbatim; `Mth.square(double)` is a plain multiply.
    pub fn next_gaussian(&mut self, random_source: &mut dyn RandomSource) -> f64 {
        if self.have_next_next_gaussian {
            self.have_next_next_gaussian = false;
            return self.next_next_gaussian;
        }

        let (x, y, radius_squared) = loop {
            let x = 2.0 * random_source.next_double() - 1.0;
            let y = 2.0 * random_source.next_double() - 1.0;
            let radius_squared = x * x + y * y;
            if !(radius_squared >= 1.0 || radius_squared == 0.0) {
                break (x, y, radius_squared);
            }
        };

        let multiplier = (-2.0 * radius_squared.ln() / radius_squared).sqrt();
        self.next_next_gaussian = y * multiplier;
        self.have_next_next_gaussian = true;
        x * multiplier
    }
}