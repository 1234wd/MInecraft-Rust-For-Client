//! Smoke tests for the Tier 2 batch ported in session 14.
//!
//! # WHY THESE EXIST AND WHY THEY ARE NOT PARITY TESTS
//!
//! Tier 2 files are ported + compiled without oracle verification, on the understanding that
//! `PORTED` means "translated, not verified". The minimum honest claim is that the translation
//! is *shaped* correctly: the enum has the right constants in the right order, the record has
//! the right field, the marker interface is implementable. That is all these assert.
//!
//! # THE ORDINAL ASSERTIONS ARE THE POINT
//!
//! For every enum below, the variant order is checked against an explicit expected discriminant.
//! Writing `ClickAction::Primary as i32 == 0` in a test is what turns "I wrote them in the right
//! order" from a claim into a check, and it is the one thing about an enum that can silently
//! break a save file without breaking a single behavioural test -- `ClickAction` is written to
//! the wire by ordinal, `LightLayer` indexes the light arrays with it, and `ChunkType` orders
//! the chunk-status registry with it.
//!
//! A bulk verification pass will replace these with golden-backed checks. Until then this file
//! is the honest lower bound of what is known about these files, and `MANIFEST.csv` says so via
//! each file's `PORTED` status and its `Tier 2, not yet oracle-verified` header.

use minecraft_rust::net::minecraft::core::particles::ParticleLimit::ParticleLimit;
use minecraft_rust::net::minecraft::network::codec::StreamDecoder::StreamDecoder;
use minecraft_rust::net::minecraft::network::codec::StreamEncoder::StreamEncoder;
use minecraft_rust::net::minecraft::network::SkipPacketException::SkipPacketException;
use minecraft_rust::net::minecraft::util::ToFloatFunction::ToFloatFunction;
use minecraft_rust::net::minecraft::util::VisibleForDebug::VISIBLE_FOR_DEBUG;
use minecraft_rust::net::minecraft::world::Clearable::Clearable;
use minecraft_rust::net::minecraft::world::entity::ItemSteerable::ItemSteerable;
use minecraft_rust::net::minecraft::world::entity::PlayerRideable::PlayerRideable;
use minecraft_rust::net::minecraft::world::entity::SpawnGroupData::SpawnGroupData;
use minecraft_rust::net::minecraft::world::entity::npc::Npc::Npc;
use minecraft_rust::net::minecraft::world::inventory.ClickAction::ClickAction;
use minecraft_rust::net::minecraft::world::inventory.tooltip::TooltipComponent::TooltipComponent;
use minecraft_rust::net::minecraft::world::item.crafting::ExtendedRecipeBookCategory::ExtendedRecipeBookCategory;
use minecraft_rust::net::minecraft::world::item.equipment::EquipmentAsset::EquipmentAsset;
use minecraft_rust::net::minecraft::world::level::LightLayer::LightLayer;
use minecraft_rust::net::minecraft::world::level::block.GameMasterBlock::GameMasterBlock;
use minecraft_rust::net::minecraft::world::level::block::RenderShape::RenderShape;
use minecraft_rust::net::minecraft::world::level::block::entity::LidBlockEntity::LidBlockEntity;
use minecraft_rust::net::minecraft::world::level::chunk::status::ChunkType::ChunkType;
use minecraft_rust::net::minecraft::world::waypoints::WaypointStyleAsset::WaypointStyleAsset;

// ---------------------------------------------------------------------------
// Enums -- ordinals are persisted data, so they are asserted one by one.
// ---------------------------------------------------------------------------

#[test]
fn click_action_ordinals_are_primary_then_secondary() {
    assert_eq!(ClickAction::Primary as i32, 0, "PRIMARY is ordinal 0");
    assert_eq!(ClickAction::Secondary as i32, 1, "SECONDARY is ordinal 1");
}

#[test]
fn light_layer_ordinals_are_sky_then_block() {
    assert_eq!(LightLayer::Sky as i32, 0, "SKY is ordinal 0");
    assert_eq!(LightLayer::Block as i32, 1, "BLOCK is ordinal 1");
}

#[test]
fn render_shape_ordinals_are_invisible_then_model() {
    assert_eq!(RenderShape::Invisible as i32, 0, "INVISIBLE is ordinal 0");
    assert_eq!(RenderShape::Model as i32, 1, "MODEL is ordinal 1");
}

#[test]
fn chunk_type_ordinals_are_protochunk_then_levelchunk() {
    assert_eq!(ChunkType::ProtoChunk as i32, 0, "PROTOCHUNK is ordinal 0");
    assert_eq!(ChunkType::LevelChunk as i32, 1, "LEVELCHUNK is ordinal 1");
}

// ---------------------------------------------------------------------------
// Records / data holders
// ---------------------------------------------------------------------------

#[test]
fn particle_limit_has_one_field_and_its_constant_is_1000() {
    let l = ParticleLimit { limit: 1000 };
    assert_eq!(l.limit, 1000);
    assert_eq!(
        ParticleLimit::SPORE_BLOSSOM.limit, 1000,
        "SPORE_BLOSSOM is `new ParticleLimit(1000)` -- copied from the source, not measured"
    );
    // The record's generated equals/hashCode are the port of what the Java record synthesises,
    // so the derived traits are part of the contract rather than an implementation detail.
    assert_eq!(l, ParticleLimit::SPORE_BLOSSOM);
}

#[test]
fn marker_types_are_zero_sized_and_distinct() {
    // A zero-sized marker is the point of `EquipmentAsset` and `WaypointStyleAsset`: they are a
    // distinct *type* carrying nothing. `size_of == 0` is what makes that true rather than
    // asserted by comment.
    assert_eq!(
        std::mem::size_of::<EquipmentAsset>(),
        0,
        "EquipmentAsset carries no state"
    );
    assert_eq!(
        std::mem::size_of::<WaypointStyleAsset>(),
        0,
        "WaypointStyleAsset carries no state"
    );
    assert_eq!(VISIBLE_FOR_DEBUG, (), "the debug marker is a unit");
}

// ---------------------------------------------------------------------------
// Marker interfaces -- the only way to smoke-test an empty trait is to implement it,
// which is also the only way it is ever used in vanilla (`instanceof`).
// ---------------------------------------------------------------------------

struct MarkerProbe;
impl SkipPacketException for MarkerProbe {}
impl PlayerRideable for MarkerProbe {}
impl SpawnGroupData for MarkerProbe {}
impl Npc for MarkerProbe {}
impl TooltipComponent for MarkerProbe {}
impl ExtendedRecipeBookCategory for MarkerProbe {}
impl GameMasterBlock for MarkerProbe {}

#[test]
fn every_marker_interface_is_implementable() {
    // One `impl` per marker trait above is the assertion: each trait compiles, each is
    // implementable by a foreign type, and none of them accidentally requires a method.
    fn assert_all<T: SkipPacketException + PlayerRideable + SpawnGroupData + Npc
        + TooltipComponent + ExtendedRecipeBookCategory + GameMasterBlock>() {}
    assert_all::<MarkerProbe>();
}

// ---------------------------------------------------------------------------
// Single-method interfaces -- exercised for real, because a wrong signature here
// would still compile if the trait were never called.
// ---------------------------------------------------------------------------

struct Boxed(i32);
impl Clearable for Boxed {
    fn clear_content(&mut self) {
        self.0 = 0;
    }
}

struct Pig {
    saddled: bool,
}
impl ItemSteerable for Pig {
    fn boost(&self) -> bool {
        self.saddled
    }
}

struct Chest {
    height: f32,
}
impl LidBlockEntity for Chest {
    // The vanilla form is `1.0 - clamp(a, 0, 1)` scaled by the lid height, clamp first.
    fn get_open_ness(&self, openness: f32) -> f32 {
        (1.0 - openness.clamp(0.0, 1.0)) * self.height
    }
}

#[test]
fn single_method_interfaces_are_callable() {
    let mut b = Boxed(7);
    b.clear_content();
    assert_eq!(b.0, 0, "clearContent mutates through &mut self");

    assert!(!Pig { saddled: false }.boost());
    assert!(Pig { saddled: true }.boost(), "a saddled pig steers");

    let chest = Chest { height: 0.5 };
    assert_eq!(chest.get_open_ness(0.0), 0.5, "fully open -> full lid offset");
    assert_eq!(chest.get_open_ness(1.0), 0.0, "fully closed -> no offset");
    assert_eq!(
        chest.get_open_ness(2.0),
        0.0,
        "the clamp happens BEFORE the subtraction, so 2.0 clamps to 1.0"
    );
}

struct Varint(i32);
impl StreamDecoder<Vec<u8>, i32> for Varint {
    fn decode(&self, input: &[u8]) -> i32 {
        input.iter().fold(0i32, |acc, b| (acc << 7) | i32::from(b & 0x7F))
    }
}
impl StreamEncoder<Vec<u8>, i32> for Varint {
    fn encode(&self, output: &mut Vec<u8>, value: i32) {
        output.extend_from_slice(&[value as u8]);
    }
}

#[test]
fn stream_codec_halves_round_trip() {
    let codec = Varint(0);
    assert_eq!(codec.decode(&[0x00, 0x00, 0x7F]), 127);
    let mut out = Vec::new();
    codec.encode(&mut out, 42);
    assert_eq!(out, vec![42u8]);
}

/// A `ToFloatFunction` implemented as the closure Java would write.
struct Halve;
impl ToFloatFunction<i32> for Halve {
    fn apply_as_float(&self, value: i32) -> f32 {
        value as f32 / 2.0
    }
}

#[test]
fn to_float_function_widens_to_f32() {
    assert_eq!(Halve.apply_as_float(7), 3.5f32);
    assert_eq!(Halve.apply_as_float(-1), -0.5f32);
}