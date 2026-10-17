//! Port of: net/minecraft/world/entity/animal/TemperatureVariants.java
//! Java class(es): net.minecraft.world.entity.animal.TemperatureVariants
//! Status: VERIFIED
//!
//! Session 22. `public interface TemperatureVariants`: three `Identifier` constants naming the
//! temperate / warm / cold animal variants, and no methods. Nothing implements it -- it is a
//! constants holder that happens to be declared as an interface.
//!
//! The constants are module-level `LazyLock`s (an `Identifier` owns its strings and cannot be a
//! `const`), and the interface itself is an empty marker trait so the Java type still has a name.
//!
//! Verified by the `IDCONST` group of `small_probe.txt` (`tests/session22.rs`), which lists the
//! interface's static `Identifier` fields by reflection.

use std::sync::LazyLock;

use crate::net::minecraft::resources::Identifier::{self as identifier, Identifier};

/// Port of `net.minecraft.world.entity.animal.TemperatureVariants` (no methods).
pub trait TemperatureVariants {}

/// Port of `TemperatureVariants.TEMPERATE`.
pub static TEMPERATE: LazyLock<Identifier> = LazyLock::new(|| identifier::with_default_namespace("temperate"));
/// Port of `TemperatureVariants.WARM`.
pub static WARM: LazyLock<Identifier> = LazyLock::new(|| identifier::with_default_namespace("warm"));
/// Port of `TemperatureVariants.COLD`.
pub static COLD: LazyLock<Identifier> = LazyLock::new(|| identifier::with_default_namespace("cold"));
