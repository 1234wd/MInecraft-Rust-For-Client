//! Port of: net/minecraft/world/entity/EntityAttachment.java
//! Java class(es): net.minecraft.world.entity.EntityAttachment, net.minecraft.world.entity.Fallback
//! Status: VERIFIED
//!
//! Session 19. Four constants, each holding one of the three `Fallback` lambdas, plus
//! `createFallbackPoints` and the nested `Fallback` interface with its `ZERO`, `AT_FEET`,
//! `AT_HEIGHT` and `AT_CENTER` constants. Names and ordinals are checked against the jar in
//! `_porting/tests/session19_b.rs`, as is which constants share a lambda (the dump records the
//! lambda's identity string, so `PASSENGER` and `NAME_TAG` must hold the same one). The points
//! each lambda produces are tested from the source.
//!
//! # `Fallback` IS A FUNCTIONAL INTERFACE; ITS CONSTANTS ARE LAMBDAS
//!
//! Ported as the trait [`Fallback`] plus [`FallbackConstant`], an enum of the three lambdas the
//! interface declares. Nothing in vanilla implements `Fallback` any other way.
//!
//! # `AT_CENTER` DIVIDES IN `double`
//!
//! `height / 2.0` is `float / double`, so `height` is widened first and the halving is done in
//! `f64`. For ordinary heights that equals halving in `f32`, but not for the smallest
//! subnormals (`f32` halving would round to `0`). The port widens first, like Java.

use crate::net::minecraft::world::phys::Vec3::Vec3;

/// Port of `net.minecraft.world.entity.EntityAttachment`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityAttachment {
    /// Port of `EntityAttachment#PASSENGER` -- ordinal 0, fallback `AT_HEIGHT`.
    Passenger = 0,
    /// Port of `EntityAttachment#VEHICLE` -- ordinal 1, fallback `AT_FEET`.
    Vehicle = 1,
    /// Port of `EntityAttachment#NAME_TAG` -- ordinal 2, fallback `AT_HEIGHT`.
    NameTag = 2,
    /// Port of `EntityAttachment#WARDEN_CHEST` -- ordinal 3, fallback `AT_CENTER`.
    WardenChest = 3,
}

/// Java: `EntityAttachment.values()` -- declaration order.
pub const VALUES: [EntityAttachment; 4] = [
    EntityAttachment::Passenger,
    EntityAttachment::Vehicle,
    EntityAttachment::NameTag,
    EntityAttachment::WardenChest,
];

impl EntityAttachment {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            EntityAttachment::Passenger => "PASSENGER",
            EntityAttachment::Vehicle => "VEHICLE",
            EntityAttachment::NameTag => "NAME_TAG",
            EntityAttachment::WardenChest => "WARDEN_CHEST",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// The private `fallback` field (no Java getter; exposed read-only for parity tests).
    #[inline]
    pub const fn fallback(&self) -> FallbackConstant {
        match self {
            EntityAttachment::Passenger => AT_HEIGHT,
            EntityAttachment::Vehicle => AT_FEET,
            EntityAttachment::NameTag => AT_HEIGHT,
            EntityAttachment::WardenChest => AT_CENTER,
        }
    }

    /// Port of `EntityAttachment#createFallbackPoints(float, float)`:
    /// `this.fallback.create(width, height)`.
    #[inline]
    pub fn create_fallback_points(&self, width: f32, height: f32) -> Vec<Vec3> {
        self.fallback().create(width, height)
    }
}

impl std::fmt::Display for EntityAttachment {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Port of the nested interface `EntityAttachment.Fallback`.
pub trait Fallback {
    /// Java: `List<Vec3> create(float width, float height)`.
    fn create(&self, width: f32, height: f32) -> Vec<Vec3>;
}

/// The three lambdas declared as constants of `EntityAttachment.Fallback`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FallbackConstant {
    /// `AT_FEET = (width, height) -> ZERO`.
    AtFeet,
    /// `AT_HEIGHT = (width, height) -> List.of(new Vec3(0.0, height, 0.0))`.
    AtHeight,
    /// `AT_CENTER = (width, height) -> List.of(new Vec3(0.0, height / 2.0, 0.0))`.
    AtCenter,
}

impl Fallback for FallbackConstant {
    /// `width` is ignored by all three lambdas.
    fn create(&self, _width: f32, height: f32) -> Vec<Vec3> {
        match self {
            FallbackConstant::AtFeet => ZERO(),
            FallbackConstant::AtHeight => vec![Vec3::new(0.0, height as f64, 0.0)],
            FallbackConstant::AtCenter => vec![Vec3::new(0.0, height as f64 / 2.0, 0.0)],
        }
    }
}

/// Java: `Fallback.ZERO = List.of(Vec3.ZERO)` (an immutable one-element list; a fresh `Vec` here).
pub fn ZERO() -> Vec<Vec3> {
    vec![Vec3::ZERO]
}

/// Java: `Fallback.AT_FEET`.
pub const AT_FEET: FallbackConstant = FallbackConstant::AtFeet;
/// Java: `Fallback.AT_HEIGHT`.
pub const AT_HEIGHT: FallbackConstant = FallbackConstant::AtHeight;
/// Java: `Fallback.AT_CENTER`.
pub const AT_CENTER: FallbackConstant = FallbackConstant::AtCenter;
