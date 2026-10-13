//! Port of: net/minecraft/core/AxisCycle.java
//! Java class(es): net.minecraft.core.AxisCycle
//! Status: VERIFIED
//!
//! Three constants that rotate a triple's components -- the trick levelgen uses to fill a box
//! without writing the six axis orderings by hand.
//!
//! # EACH CONSTANT OVERRIDES EVERY METHOD, SO THIS IS A MATCH ON THE VARIANT
//!
//! In Java the three constants each carry an anonymous subclass overriding `cycle` and
//! `inverse`. Rust has no anonymous subclasses, so the same shape is expressed as a `match` on
//! the discriminant with the per-constant bodies written out. That is not a translation of the
//! *mechanism* but it is a translation of the *behaviour*, and the behaviour is what the golden
//! checks.
//!
//! # `FORWARD` AND `BACKWARD` ARE NOT SYMMETRIC ARGUMENT ORDERS
//!
//! ```java
//! // NONE      -> axis.choose(x, y, z)
//! // FORWARD   -> axis.choose(z, x, y)
//! // BACKWARD  -> axis.choose(y, z, x)
//! ```
//!
//! `FORWARD` reads `(z, x, y)` and `BACKWARD` reads `(y, z, x)`. Note that the inverse of
//! `(z, x, y)` -- read as "output axis gets input component 2, axis 0 gets component 0, axis 1
//! gets component 1" -- is `(y, z, x)`, so they really are inverses. Writing `BACKWARD` as
//! `axis.choose(x, z, y)` would be the intuitive guess and is wrong.
//!
//! # `Math.floorMod` MATTERS FOR THE `Axis` OVERLOAD
//!
//! ```java
//! AXIS_VALUES[Math.floorMod(axis.ordinal() - 1, 3)]
//! ```
//!
//! `floorMod(-1, 3)` is `2`, where `%` would give `-1` and `AXIS_VALUES[-1]` throws
//! `ArrayIndexOutOfBoundsException`. `BACKWARD` applied to `Axis::X` therefore returns `Z`
//! rather than throwing. Rust's `rem_euclid` is the equivalent for a positive divisor, so
//! `ordinal.wrapping_sub(1).rem_euclid(3)` is the translation -- **not** `%`.

use crate::net::minecraft::core::Direction::Axis;

/// Java: `AxisCycle.values()`.
pub const VALUES: [AxisCycle; 3] = [
    AxisCycle::None,
    AxisCycle::Forward,
    AxisCycle::Backward,
];

/// Port of `net.minecraft.core.AxisCycle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AxisCycle {
    /// Port of `AxisCycle#NONE` -- ordinal 0. Identity in every sense.
    None = 0,
    /// Port of `AxisCycle#FORWARD` -- ordinal 1.
    Forward = 1,
    /// Port of `AxisCycle#BACKWARD` -- ordinal 2.
    Backward = 2,
}

impl AxisCycle {
    /// Port of `AxisCycle#cycle(int, int, int, Axis)`.
    #[inline]
    pub fn cycle_int(&self, x: i32, y: i32, z: i32, axis: Axis) -> i32 {
        match self {
            AxisCycle::None => axis.choose_int(x, y, z),
            AxisCycle::Forward => axis.choose_int(z, x, y),
            AxisCycle::Backward => axis.choose_int(y, z, x),
        }
    }

    /// Port of `AxisCycle#cycle(double, double, double, Axis)`.
    ///
    /// A separate method rather than a generic one, because `Axis#choose` is overloaded on `int`,
    /// `double` and `boolean` in Java and Rust has no overloading. The `f64` overload on
    /// `Direction::Axis` kept the name `choose`; the `int` and `bool` ones added alongside it are
    /// `choose_int` and `choose_bool`. Naming them apart prevents the silent integer-division
    /// class of bug that a single generic `choose<T>` would invite.
    #[inline]
    pub fn cycle_f64(&self, x: f64, y: f64, z: f64, axis: Axis) -> f64 {
        match self {
            AxisCycle::None => axis.choose(x, y, z),
            AxisCycle::Forward => axis.choose(z, x, y),
            AxisCycle::Backward => axis.choose(y, z, x),
        }
    }

    /// Port of `AxisCycle#cycle(Axis)`: rotate the axis itself.
    #[inline]
    pub fn cycle_axis(&self, axis: Axis) -> Axis {
        let ordinal = axis.ordinal();
        match self {
            AxisCycle::None => axis,
            AxisCycle::Forward => Axis::from_ordinal(ordinal.wrapping_add(1).rem_euclid(3)),
            AxisCycle::Backward => Axis::from_ordinal(ordinal.wrapping_sub(1).rem_euclid(3)),
        }
    }

    /// Port of `AxisCycle#inverse()`.
    #[inline]
    pub const fn inverse(&self) -> AxisCycle {
        match self {
            AxisCycle::None => AxisCycle::None,
            AxisCycle::Forward => AxisCycle::Backward,
            AxisCycle::Backward => AxisCycle::Forward,
        }
    }
}

/// Port of `AxisCycle#between(Axis, Axis)`.
///
/// The forward cycle taking `from` to `to` -- `Math.floorMod(to - from, 3)`, so `X -> Y` and
/// `Y -> X` are different answers (1 and 2) and `X -> X` is `NONE`.
#[inline]
pub fn between(from: Axis, to: Axis) -> AxisCycle {
    VALUES[to.ordinal().wrapping_sub(from.ordinal()).rem_euclid(3) as usize]
}