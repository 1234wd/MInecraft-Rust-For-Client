//! Port of: com/mojang/blaze3d/platform/Transparency.java
//! Java class(es): com.mojang.blaze3d.platform.Transparency
//! Status: VERIFIED
//!
//! Session 22. `public record Transparency(boolean hasTransparent, boolean hasTranslucent)` with
//! the four possible values as constants and a factory that always returns one of them.
//!
//! `of(a, b)` maps each of the four combinations to its constant, and `or(other)` is `of` over the
//! component-wise `||` -- so `or` returns a constant even when called on an instance that is not
//! one (the probe has a row for that). Java callers may then compare with `==`; here equality is
//! by value, which agrees with that identity on every value `of` can return.
//!
//! # Record members
//!
//! `equals` is derived, `hashCode` is [`Transparency::java_hash_code`] (the record fold over two
//! `Boolean.hashCode`s), `toString` is `Transparency[hasTransparent=<b>, hasTranslucent=<b>]`.
//!
//! Verified by the `REC`, `RECEQ`, `TROF`, `TROR` and `TROPAQUE` groups of `small_probe.txt`
//! (`tests/session22.rs`): all four `of` inputs, all sixteen `or` pairs and `isOpaque` on each.

use crate::javacompat::java_object;

/// Port of `com.mojang.blaze3d.platform.Transparency`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transparency {
    has_transparent: bool,
    has_translucent: bool,
}

impl Transparency {
    /// Port of `Transparency.NONE` = `(false, false)`.
    pub const NONE: Transparency = Transparency::new(false, false);
    /// Port of `Transparency.TRANSPARENT` = `(true, false)`.
    pub const TRANSPARENT: Transparency = Transparency::new(true, false);
    /// Port of `Transparency.TRANSLUCENT` = `(false, true)`.
    pub const TRANSLUCENT: Transparency = Transparency::new(false, true);
    /// Port of `Transparency.TRANSPARENT_AND_TRANSLUCENT` = `(true, true)`.
    pub const TRANSPARENT_AND_TRANSLUCENT: Transparency = Transparency::new(true, true);

    /// Port of the canonical constructor.
    pub const fn new(has_transparent: bool, has_translucent: bool) -> Self {
        Transparency {
            has_transparent,
            has_translucent,
        }
    }

    /// Port of the accessor `hasTransparent()`.
    pub const fn has_transparent(&self) -> bool {
        self.has_transparent
    }

    /// Port of the accessor `hasTranslucent()`.
    pub const fn has_translucent(&self) -> bool {
        self.has_translucent
    }

    /// Port of `of(boolean hasTransparent, boolean hasTranslucent)`.
    pub const fn of(has_transparent: bool, has_translucent: bool) -> Transparency {
        if has_transparent && has_translucent {
            Self::TRANSPARENT_AND_TRANSLUCENT
        } else if has_transparent {
            Self::TRANSPARENT
        } else if has_translucent {
            Self::TRANSLUCENT
        } else {
            Self::NONE
        }
    }

    /// Port of `or(Transparency other)`.
    pub const fn or(&self, other: &Transparency) -> Transparency {
        Self::of(
            self.has_transparent || other.has_transparent,
            self.has_translucent || other.has_translucent,
        )
    }

    /// Port of `isOpaque()`.
    pub const fn is_opaque(&self) -> bool {
        !self.has_transparent && !self.has_translucent
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            java_object::hash_bool(self.has_transparent),
            java_object::hash_bool(self.has_translucent),
        ])
    }
}

impl std::fmt::Display for Transparency {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transparency[hasTransparent={}, hasTranslucent={}]",
            self.has_transparent, self.has_translucent
        )
    }
}
