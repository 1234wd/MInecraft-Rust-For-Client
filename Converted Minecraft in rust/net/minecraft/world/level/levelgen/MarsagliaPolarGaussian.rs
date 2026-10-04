//! Port of: net/minecraft/world/level/levelgen/MarsagliaPolarGaussian.java
//! Java class(es): net.minecraft.world.level.levelgen.MarsagliaPolarGaussian
//! Status: PARTIAL
//!
//! PARTIAL because of exactly one thing, stated up front rather than buried in a
//! method doc: `next_gaussian` is **255/256-exact, not 256/256**. It is the only
//! method in the ported surface that calls `Math.log`, and HotSpot's `Math.log` is a
//! table-driven `_dlog` intrinsic that no available implementation reproduces
//! exactly. Everything else here is oracle-verified bit-for-bit.
//!
//! Concretely, the golden transcript in `random.txt` matches on every draw except
//! eight, which are pinned by index in `_porting/tests/parity_random.rs`. Those eight
//! are not sloppiness -- they are the price of an unported JVM intrinsic, and the
//! alternative (asserting equality, or deleting the assertion) would have hidden it.
//! See `javacompat::java_lang::log` and OPEN_QUESTIONS.md.
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

use crate::javacompat::java_lang::log::math_log_f64;
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
    ///
    /// # The rejection loop is NOT optional
    ///
    /// About one pair in eight has `radiusSquared >= 1` and is discarded, costing two
    /// MORE `nextDouble` draws. This is easy to miss because it looks like a rare edge
    /// case rather than part of the algorithm, and omitting it desynchronises the
    /// stream for the rest of the source's life while still producing plausible-looking
    /// output. The oracle's `gaussianSteps` group emits the rejection count for exactly
    /// this reason.
    ///
    /// # `Math.log` IS NOT EXACT HERE
    ///
    /// This is the only method in the ported surface that calls `Math.log`.
    /// `multiplier = sqrt(-2.0 * log(rs) / rs)`, and HotSpot evaluates that `Math.log`
    /// with its `_dlog` intrinsic -- a table-driven stub
    /// (`stubGenerator_x86_64_log.cpp::generate_libmLog`) built on a 128-entry double
    /// table and six polynomial coefficients. That is neither fdlibm nor the host
    /// libm, and it differs from the host's `ln` by 1 ULP on 1 of the 256 arbitrary
    /// doubles measured against it.
    ///
    /// So this method is **255/256-exact, not 256/256**. `javacompat::java_lang::log`
    /// holds the full measurement and the reasoning; transcribing the stub is an open
    /// question.
    ///
    /// The error stays LOCAL, which is the one redeeming consequence: the acceptance
    /// test compares `radiusSquared`, never `log`, so a wrong multiplier cannot change
    /// how many doubles are consumed. Each affected pair diverges in isolation -- its
    /// own two draws and nothing else. `parity_random` pins the exact diverging draw
    /// indices so the gap stays visible and cannot quietly widen.
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

        // `math_log_f64`, not `radius_squared.ln()`, so that the single place to change
        // when `Math.log` becomes exact is findable by grep.
        let multiplier = (-2.0 * math_log_f64(radius_squared) / radius_squared).sqrt();
        self.next_next_gaussian = y * multiplier;
        self.have_next_next_gaussian = true;
        x * multiplier
    }
}