//! Port of: net/minecraft/world/level/block/state/properties/Tilt.java
//! Java class(es): net.minecraft.world.level.block.state.properties.Tilt
//! Status: VERIFIED
//!
//! Session 19. Four constants carrying a `String name` and a `boolean causesVibration`. Every
//! member ported; ordinals and both fields of every constant are checked against the jar
//! (`session19_a.rs`).
//!
//! # ONLY `UNSTABLE` IS SILENT
//!
//! ```java
//! NONE("none", true), UNSTABLE("unstable", false), PARTIAL("partial", true), FULL("full", true);
//! ```
//!
//! `NONE` causes a vibration and `UNSTABLE` does not. That reads backwards but it is the jar's
//! value (dump-checked), so it is transcribed, not "fixed".
//!
//! No `toString()` override: `Display` is `name()` (`UNSTABLE`), not `unstable`.

/// Port of `net.minecraft.world.level.block.state.properties.Tilt`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tilt {
    /// `NONE` -- ordinal 0, `"none"`, causes vibration.
    None = 0,
    /// `UNSTABLE` -- ordinal 1, `"unstable"`, does NOT cause vibration.
    Unstable = 1,
    /// `PARTIAL` -- ordinal 2, `"partial"`, causes vibration.
    Partial = 2,
    /// `FULL` -- ordinal 3, `"full"`, causes vibration.
    Full = 3,
}

/// Java: `Tilt.values()`, declaration order.
pub const VALUES: [Tilt; 4] = [Tilt::None, Tilt::Unstable, Tilt::Partial, Tilt::Full];

impl Tilt {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Tilt::None => "NONE",
            Tilt::Unstable => "UNSTABLE",
            Tilt::Partial => "PARTIAL",
            Tilt::Full => "FULL",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Tilt#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            Tilt::None => "none",
            Tilt::Unstable => "unstable",
            Tilt::Partial => "partial",
            Tilt::Full => "full",
        }
    }

    /// Port of `Tilt#causesVibration()` -- the private `causesVibration` field.
    #[inline]
    pub const fn causes_vibration(&self) -> bool {
        match self {
            Tilt::None => true,
            Tilt::Unstable => false,
            Tilt::Partial => true,
            Tilt::Full => true,
        }
    }
}

impl std::fmt::Display for Tilt {
    /// Java's `Enum#toString()` (not overridden), i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for Tilt {
    fn get_serialized_name(&self) -> &str {
        Tilt::get_serialized_name(self)
    }
}
