//! Port of: net/minecraft/data/AtlasIds.java
//! Java class(es): net.minecraft.data.AtlasIds
//! Status: VERIFIED
//!
//! Session 22. `public class AtlasIds`: the identifiers of the thirteen texture atlases, as
//! `public static final Identifier` constants, and nothing else.
//!
//! `Identifier` owns its strings, so it cannot be a `const`; each constant is a `LazyLock`, built
//! on first use exactly as the Java static initialiser builds it, with
//! `Identifier.withDefaultNamespace(path)`.
//!
//! Verified by the `IDCONST` group of `small_probe.txt` (`tests/session22.rs`), which lists the
//! class's static `Identifier` fields BY REFLECTION -- so a constant added to or dropped from the
//! jar fails the test, not just a misspelled one.

use std::sync::LazyLock;

use crate::net::minecraft::resources::Identifier::{self as identifier, Identifier};

/// Port of `AtlasIds.ARMOR_TRIMS`.
pub static ARMOR_TRIMS: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("armor_trims"));
/// Port of `AtlasIds.BANNER_PATTERNS`.
pub static BANNER_PATTERNS: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("banner_patterns"));
/// Port of `AtlasIds.BLOCKS`.
pub static BLOCKS: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("blocks"));
/// Port of `AtlasIds.ITEMS`.
pub static ITEMS: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("items"));
/// Port of `AtlasIds.CHESTS`.
pub static CHESTS: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("chests"));
/// Port of `AtlasIds.DECORATED_POT`.
pub static DECORATED_POT: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("decorated_pot"));
/// Port of `AtlasIds.GUI`.
pub static GUI: LazyLock<Identifier> = LazyLock::new(|| identifier::with_default_namespace("gui"));
/// Port of `AtlasIds.MAP_DECORATIONS`.
pub static MAP_DECORATIONS: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("map_decorations"));
/// Port of `AtlasIds.PAINTINGS`.
pub static PAINTINGS: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("paintings"));
/// Port of `AtlasIds.PARTICLES`.
pub static PARTICLES: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("particles"));
/// Port of `AtlasIds.SHIELD_PATTERNS`.
pub static SHIELD_PATTERNS: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("shield_patterns"));
/// Port of `AtlasIds.SHULKER_BOXES`.
pub static SHULKER_BOXES: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("shulker_boxes"));
/// Port of `AtlasIds.CELESTIALS`.
pub static CELESTIALS: LazyLock<Identifier> =
    LazyLock::new(|| identifier::with_default_namespace("celestials"));
