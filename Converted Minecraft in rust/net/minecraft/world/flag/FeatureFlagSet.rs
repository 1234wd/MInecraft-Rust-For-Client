//! Port of: net/minecraft/world/flag/FeatureFlagSet.java
//! Java class(es): net.minecraft.world.flag.FeatureFlagSet
//! Status: VERIFIED
//!
//! Session 24. `public final class FeatureFlagSet`: an immutable set of up to 64
//! [`FeatureFlag`]s of ONE [`FeatureFlagUniverse`], stored as the OR of their masks.
//!
//! # Universe identity
//!
//! Every comparison of universes in the Java source is `==`, by reference. The port holds the
//! universe as `Option<Arc<FeatureFlagUniverse>>` (`null` for the shared `EMPTY` set) and compares
//! with [`Arc::ptr_eq`]. Two universes with the same id are different universes: the probe builds
//! flags in two `new FeatureFlagUniverse("main")` objects and pins every operation across them.
//!
//! # Members
//!
//! | Java | here |
//! |---|---|
//! | `static final FeatureFlagSet EMPTY` (private) | [`of`] returns it; [`FeatureFlagSet::is_empty`] compares with it |
//! | `MAX_CONTAINER_SIZE = 64` | [`MAX_CONTAINER_SIZE`] |
//! | `static create(FeatureFlagUniverse, Collection<FeatureFlag>)` (package-private) | [`create`] |
//! | `of()`, `of(FeatureFlag)`, `of(FeatureFlag, FeatureFlag...)` | [`of`], [`of_flag`], [`of_flags`] |
//! | `computeMask` (private) | `compute_mask` |
//! | `contains`, `isEmpty`, `isSubsetOf`, `intersects`, `join`, `subtract` | the same names |
//! | `equals`, `hashCode` | `PartialEq`/`Eq`, [`FeatureFlagSet::java_hash_code`] |
//! | `toString()` (inherited from `Object`) | `Display` |
//!
//! `hashCode` is `(int) HashCommon.mix(mask)` (fastutil), so -- unusually for a class that does not
//! override `toString` -- `Object.toString()` is reproducible: it is the class name, `@`, and the
//! hex of that hash code. The probe pins it.
//!
//! The two kinds of mismatch throw different exceptions, with different texts: building a set from
//! flags of two universes is an `IllegalStateException` ("Mismatched feature universe, ..."), while
//! `join`/`subtract` of two sets is an `IllegalArgumentException` ("Mismatched set elements: ...").
//! Both are panics here, with Java's `toString` as the text (`DESIGN_DECISIONS.md#panic-not-Result`).
//!
//! Verified by the `FFSET` and `FFSETOP` groups of `sweep_probe.txt` (`tests/session24.rs`):
//! sets built every way (the mismatch exceptions included) and every operation over every ordered
//! pair. `HashCommon.mix` itself is pinned by `HCMIX`.

use std::sync::Arc;

use crate::javacompat::fastutil::hash_common;
use crate::net::minecraft::world::flag::FeatureFlag::FeatureFlag;
use crate::net::minecraft::world::flag::FeatureFlagUniverse::FeatureFlagUniverse;

/// `FeatureFlagSet.class.getName()`.
pub const CLASS_NAME: &str = "net.minecraft.world.flag.FeatureFlagSet";

/// Port of `public static final int MAX_CONTAINER_SIZE = 64`.
pub const MAX_CONTAINER_SIZE: i32 = 64;

/// Port of `net.minecraft.world.flag.FeatureFlagSet`.
#[derive(Debug, Clone)]
pub struct FeatureFlagSet {
    universe: Option<Arc<FeatureFlagUniverse>>,
    mask: i64,
}

/// Port of `private static final FeatureFlagSet EMPTY = new FeatureFlagSet(null, 0L)`.
const EMPTY: FeatureFlagSet = FeatureFlagSet {
    universe: None,
    mask: 0,
};

/// Java's `a == b` on two (nullable) universe references.
fn same_universe(
    a: Option<&Arc<FeatureFlagUniverse>>,
    b: Option<&Arc<FeatureFlagUniverse>>,
) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        _ => false,
    }
}

/// Java's string concatenation of a nullable universe: its `toString()`, or `"null"`.
fn universe_text(u: Option<&Arc<FeatureFlagUniverse>>) -> String {
    u.map_or_else(|| "null".to_string(), |u| u.to_string())
}

/// Port of `private static long computeMask(FeatureFlagUniverse universe, long mask, Iterable<FeatureFlag> flags)`.
fn compute_mask(universe: &Arc<FeatureFlagUniverse>, mut mask: i64, flags: &[&FeatureFlag]) -> i64 {
    for f in flags {
        if !Arc::ptr_eq(universe, &f.universe) {
            panic!(
                "java.lang.IllegalStateException: Mismatched feature universe, expected '{universe}', but got '{}'",
                f.universe
            );
        }
        mask |= f.mask;
    }
    mask
}

/// Port of the package-private `static FeatureFlagSet create(FeatureFlagUniverse, Collection<FeatureFlag>)`.
///
/// `flags` is in the collection's iteration order, which decides which mismatched flag is
/// reported first.
pub fn create(universe: &Arc<FeatureFlagUniverse>, flags: &[&FeatureFlag]) -> FeatureFlagSet {
    if flags.is_empty() {
        return EMPTY;
    }
    let mask = compute_mask(universe, 0, flags);
    FeatureFlagSet {
        universe: Some(Arc::clone(universe)),
        mask,
    }
}

/// Port of `public static FeatureFlagSet of()`: the shared empty set.
pub fn of() -> FeatureFlagSet {
    EMPTY
}

/// Port of `public static FeatureFlagSet of(FeatureFlag flag)`.
pub fn of_flag(flag: &FeatureFlag) -> FeatureFlagSet {
    FeatureFlagSet {
        universe: Some(Arc::clone(&flag.universe)),
        mask: flag.mask,
    }
}

/// Port of `public static FeatureFlagSet of(FeatureFlag flag, FeatureFlag... flags)`.
pub fn of_flags(flag: &FeatureFlag, flags: &[&FeatureFlag]) -> FeatureFlagSet {
    let mask = if flags.is_empty() {
        flag.mask
    } else {
        compute_mask(&flag.universe, flag.mask, flags)
    };
    FeatureFlagSet {
        universe: Some(Arc::clone(&flag.universe)),
        mask,
    }
}

impl FeatureFlagSet {
    /// Port of `public boolean contains(FeatureFlag flag)`.
    pub fn contains(&self, flag: &FeatureFlag) -> bool {
        if !same_universe(self.universe.as_ref(), Some(&flag.universe)) {
            false
        } else {
            (self.mask & flag.mask) != 0
        }
    }

    /// Port of `public boolean isEmpty()`: `this.equals(EMPTY)`.
    pub fn is_empty(&self) -> bool {
        *self == EMPTY
    }

    /// Port of `public boolean isSubsetOf(FeatureFlagSet set)`.
    pub fn is_subset_of(&self, set: &FeatureFlagSet) -> bool {
        if self.universe.is_none() {
            true
        } else if !same_universe(self.universe.as_ref(), set.universe.as_ref()) {
            false
        } else {
            (self.mask & !set.mask) == 0
        }
    }

    /// Port of `public boolean intersects(FeatureFlagSet set)`.
    pub fn intersects(&self, set: &FeatureFlagSet) -> bool {
        if self.universe.is_some()
            && set.universe.is_some()
            && same_universe(self.universe.as_ref(), set.universe.as_ref())
        {
            (self.mask & set.mask) != 0
        } else {
            false
        }
    }

    /// Port of `public FeatureFlagSet join(FeatureFlagSet other)`.
    ///
    /// Returns `other` or `this` unchanged when either side has no universe -- in Java the same
    /// object, here a copy of it, which compares equal.
    pub fn join(&self, other: &FeatureFlagSet) -> FeatureFlagSet {
        if self.universe.is_none() {
            other.clone()
        } else if other.universe.is_none() {
            self.clone()
        } else if !same_universe(self.universe.as_ref(), other.universe.as_ref()) {
            panic!(
                "java.lang.IllegalArgumentException: Mismatched set elements: '{}' != '{}'",
                universe_text(self.universe.as_ref()),
                universe_text(other.universe.as_ref())
            );
        } else {
            FeatureFlagSet {
                universe: self.universe.clone(),
                mask: self.mask | other.mask,
            }
        }
    }

    /// Port of `public FeatureFlagSet subtract(FeatureFlagSet other)`.
    pub fn subtract(&self, other: &FeatureFlagSet) -> FeatureFlagSet {
        if self.universe.is_none() || other.universe.is_none() {
            return self.clone();
        }
        if !same_universe(self.universe.as_ref(), other.universe.as_ref()) {
            panic!(
                "java.lang.IllegalArgumentException: Mismatched set elements: '{}' != '{}'",
                universe_text(self.universe.as_ref()),
                universe_text(other.universe.as_ref())
            );
        }
        let new_mask = self.mask & !other.mask;
        if new_mask == 0 {
            EMPTY
        } else {
            FeatureFlagSet {
                universe: self.universe.clone(),
                mask: new_mask,
            }
        }
    }

    /// Port of `public int hashCode()`: `(int) HashCommon.mix(this.mask)`.
    pub fn java_hash_code(&self) -> i32 {
        hash_common::mix_i64(self.mask) as i32
    }
}

impl PartialEq for FeatureFlagSet {
    /// Port of `public boolean equals(Object o)`: the same universe REFERENCE and the same mask.
    fn eq(&self, other: &Self) -> bool {
        same_universe(self.universe.as_ref(), other.universe.as_ref()) && self.mask == other.mask
    }
}

impl Eq for FeatureFlagSet {}

impl std::hash::Hash for FeatureFlagSet {
    /// Consistent with `eq`: the universe's address and the mask.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.universe.as_ref().map(Arc::as_ptr).hash(state);
        self.mask.hash(state);
    }
}

impl std::fmt::Display for FeatureFlagSet {
    /// Port of the inherited `Object#toString()`: `getClass().getName() + "@" +
    /// Integer.toHexString(hashCode())`, reproducible here because `hashCode` is overridden.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{CLASS_NAME}@{:x}", self.java_hash_code() as u32)
    }
}
