//! Port of: net/minecraft/world/flag/FeatureFlag.java
//! Java class(es): net.minecraft.world.flag.FeatureFlag
//! Status: VERIFIED
//!
//! Session 24. `public class FeatureFlag`: one bit of a [`FeatureFlagUniverse`]. Two
//! package-private `final` fields and a package-private constructor; no methods.
//!
//! ```java
//! FeatureFlag(final FeatureFlagUniverse universe, final int bit) {
//!     this.universe = universe;
//!     this.mask = 1L << bit;
//! }
//! ```
//!
//! # The shift is Java's
//!
//! `1L << bit` uses only the low six bits of `bit` (JLS 15.19), so `bit = 64` gives `1`, `bit = -1`
//! gives `1L << 63` (a negative mask) and nothing ever overflows. Rust's `<<` would panic in debug
//! for those, so the port masks explicitly. `FeatureFlagRegistry.Builder.create` never passes a bit
//! of 64 or more (it throws "Too many feature flags" first), but the constructor itself does not
//! check, and the probe pins its behaviour for out-of-range bits.
//!
//! # Identity
//!
//! The class overrides neither `equals` nor `hashCode`, so Java compares flags by reference. No
//! vanilla code depends on that beyond `FeatureFlagRegistry.fromNames`' identity set, whose only
//! effect is de-duplicating flags before their masks are OR-ed -- which is idempotent. The port is
//! therefore a cloneable value holding a shared reference to its universe, and derives no
//! `PartialEq`. `toString()` is `Object`'s, built from the identity hash, and is not ported
//! (OPEN_QUESTIONS #30).
//!
//! The universe is held by `Arc` because [`FeatureFlagSet`](super::FeatureFlagSet) compares
//! universes with Java's `==`: `Arc::ptr_eq` is that comparison.
//!
//! Verified by the `FFLAG` group of `sweep_probe.txt` (`tests/session24.rs`), which reads both
//! fields back by reflection.

use std::sync::Arc;

use crate::net::minecraft::world::flag::FeatureFlagUniverse::FeatureFlagUniverse;

/// Port of `net.minecraft.world.flag.FeatureFlag`.
#[derive(Debug, Clone)]
pub struct FeatureFlag {
    /// Port of `final FeatureFlagUniverse universe` (package-private).
    pub universe: Arc<FeatureFlagUniverse>,
    /// Port of `final long mask` (package-private).
    pub mask: i64,
}

impl FeatureFlag {
    /// Port of the package-private `FeatureFlag(FeatureFlagUniverse universe, int bit)`.
    pub fn new(universe: Arc<FeatureFlagUniverse>, bit: i32) -> Self {
        FeatureFlag {
            universe,
            // Java `1L << bit`: the shift distance is `bit & 0x3f`.
            mask: 1i64 << (bit & 0x3f),
        }
    }
}
