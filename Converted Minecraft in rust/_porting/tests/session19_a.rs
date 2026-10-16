//! Session 19 batch a: block-state property enums. Checks ports against the jar's constant dump and the
//! Java source's behaviour. See the file headers of the ported .rs files for status.

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

use minecraft_rust::net::minecraft::util::StringRepresentable::{
    create_name_lookup, from_enum, NameLookup, StringRepresentable as SR,
};
use minecraft_rust::net::minecraft::world::level::block::state::properties as p;
use std::fmt::{Debug, Display};

/// Every serialized name must resolve back to its own constant through the ported
/// `StringRepresentable.fromEnum(...).byName` (so the names are unique), and the lookup is
/// case-sensitive (Java `String#equals`).
fn round_trip<E: SR + Clone + Display + PartialEq + Debug>(values: &[E]) {
    let codec = from_enum(values);
    for v in values {
        let name = SR::get_serialized_name(v).to_string();
        assert_eq!(codec.by_name(&name), Some(v.clone()), "byName({name:?})");
        let upper = name.to_ascii_uppercase();
        if upper != name {
            assert_eq!(codec.by_name(&upper), None, "byName({upper:?}) must not match");
        }
    }
    assert_eq!(codec.by_name(""), None);
}

/// Expected `toString()` per the Java source: `name()` when the enum does not override it,
/// the serialized name when it does.
macro_rules! java_to_string {
    (name, $v:expr) => {
        $v.name()
    };
    (serialized, $v:expr) => {
        $v.get_serialized_name()
    };
}

/// Enums whose serialized name is a `name` field: names + ordinals + the `name` column against
/// the jar, the `StringRepresentable` impl, `toString()`, and the byName round trip.
macro_rules! name_field_enum {
    ($test:ident, $m:ident, to_string = $disp:ident) => {
        #[test]
        fn $test() {
            use minecraft_rust::net::minecraft::world::level::block::state::properties::$m::{
                $m as E, VALUES,
            };
            let fq = concat!("net.minecraft.world.level.block.state.properties.", stringify!($m));
            assert_enum_order(fq, &VALUES.map(|v| v.name()));
            for v in VALUES {
                assert_eq!(v.ordinal(), v as i32, "{fq}.{}", v.name());
                assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
                assert_eq!(<E as SR>::get_serialized_name(&v), v.get_serialized_name());
                assert_eq!(v.to_string(), java_to_string!($disp, v), "{fq}.{} toString", v.name());
            }
            round_trip(&VALUES);
        }
    };
}

name_field_enum!(attach_face_matches_the_jar, AttachFace, to_string = name);
name_field_enum!(bell_attach_type_matches_the_jar, BellAttachType, to_string = name);
name_field_enum!(comparator_mode_matches_the_jar, ComparatorMode, to_string = serialized);
name_field_enum!(half_matches_the_jar, Half, to_string = serialized);
name_field_enum!(piston_type_matches_the_jar, PistonType, to_string = serialized);
name_field_enum!(bamboo_leaves_matches_the_jar, BambooLeaves, to_string = serialized);
name_field_enum!(creaking_heart_state_matches_the_jar, CreakingHeartState, to_string = serialized);
name_field_enum!(sculk_sensor_phase_matches_the_jar, SculkSensorPhase, to_string = serialized);
name_field_enum!(slab_type_matches_the_jar, SlabType, to_string = serialized);
name_field_enum!(wall_side_matches_the_jar, WallSide, to_string = serialized);
name_field_enum!(bed_part_matches_the_jar, BedPart, to_string = serialized);
name_field_enum!(potent_sulfur_state_matches_the_jar, PotentSulfurState, to_string = serialized);
name_field_enum!(speleothem_thickness_matches_the_jar, SpeleothemThickness, to_string = serialized);
name_field_enum!(stairs_shape_matches_the_jar, StairsShape, to_string = serialized);
name_field_enum!(tilt_matches_the_jar, Tilt, to_string = name);
name_field_enum!(redstone_side_matches_the_jar, RedstoneSide, to_string = serialized);
name_field_enum!(chest_type_matches_the_jar, ChestType, to_string = name);
name_field_enum!(rail_shape_matches_the_jar, RailShape, to_string = serialized);
name_field_enum!(side_chain_part_matches_the_jar, SideChainPart, to_string = serialized);
name_field_enum!(structure_mode_matches_the_jar, StructureMode, to_string = name);
name_field_enum!(test_block_mode_matches_the_jar, TestBlockMode, to_string = name);
name_field_enum!(note_block_instrument_matches_the_jar, NoteBlockInstrument, to_string = name);

#[test]
fn to_string_spot_checks_from_the_java_source() {
    // Overrides vs inherited Enum#toString, literal values from the .java files.
    assert_eq!(p::PistonType::PistonType::Default.to_string(), "normal");
    assert_eq!(p::PistonType::PistonType::Default.name(), "DEFAULT");
    assert_eq!(p::AttachFace::AttachFace::Floor.to_string(), "FLOOR");
    assert_eq!(p::BellAttachType::BellAttachType::SingleWall.to_string(), "SINGLE_WALL");
    assert_eq!(p::Tilt::Tilt::Unstable.to_string(), "UNSTABLE");
    assert_eq!(p::ChestType::ChestType::Single.to_string(), "SINGLE");
    assert_eq!(p::StructureMode::StructureMode::Save.to_string(), "SAVE");
    assert_eq!(p::TestBlockMode::TestBlockMode::Start.to_string(), "START");
    assert_eq!(
        p::NoteBlockInstrument::NoteBlockInstrument::IronXylophone.to_string(),
        "IRON_XYLOPHONE"
    );
    assert_eq!(p::WallSide::WallSide::Tall.to_string(), "tall");
    assert_eq!(p::RedstoneSide::RedstoneSide::Side.to_string(), "side");
    assert_eq!(p::RailShape::RailShape::AscendingNorth.to_string(), "ascending_north");
    assert_eq!(p::SideChainPart::SideChainPart::Unconnected.to_string(), "unconnected");
}

// ---------------------------------------------------------------------------------------------
// Enums whose serialized name is computed (no `name` field in the dump)
// ---------------------------------------------------------------------------------------------

#[test]
fn door_hinge_side_matches_the_jar() {
    use p::DoorHingeSide::{DoorHingeSide, VALUES};
    let fq = "net.minecraft.world.level.block.state.properties.DoorHingeSide";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(<DoorHingeSide as SR>::get_serialized_name(&v), v.get_serialized_name());
        assert_eq!(v.to_string(), v.get_serialized_name());
    }
    // `this == LEFT ? "left" : "right"` -- no field, so from the source.
    assert_eq!(DoorHingeSide::Left.get_serialized_name(), "left");
    assert_eq!(DoorHingeSide::Right.get_serialized_name(), "right");
    assert_eq!(DoorHingeSide::Left.to_string(), "left");
    round_trip(&VALUES);
}

#[test]
fn double_block_half_matches_the_jar() {
    use minecraft_rust::net::minecraft::core::Direction::Direction;
    use p::DoubleBlockHalf::{DoubleBlockHalf, VALUES};
    let fq = "net.minecraft.world.level.block.state.properties.DoubleBlockHalf";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        // The dump records enum-typed fields by `name()`. The Direction port's `name()` is Java's
        // `getName()` (lowercase); for Direction, Java `name()` is exactly that, upper-cased.
        let dir = v.get_direction_to_other();
        assert_enum_field(fq, v.name(), "directionToOther", &strv(&dir.name().to_ascii_uppercase()));
        assert_eq!(<DoubleBlockHalf as SR>::get_serialized_name(&v), v.get_serialized_name());
        assert_eq!(v.to_string(), v.get_serialized_name());
        // The two halves point at each other.
        assert_eq!(v.get_other_half().get_other_half(), v);
        assert_ne!(v.get_other_half(), v);
        assert_eq!(dir.get_opposite(), v.get_other_half().get_direction_to_other());
    }
    assert_eq!(DoubleBlockHalf::Upper.get_direction_to_other(), Direction::Down);
    assert_eq!(DoubleBlockHalf::Lower.get_direction_to_other(), Direction::Up);
    assert_eq!(DoubleBlockHalf::Upper.get_serialized_name(), "upper");
    assert_eq!(DoubleBlockHalf::Lower.get_serialized_name(), "lower");
    assert_eq!(DoubleBlockHalf::Upper.get_other_half(), DoubleBlockHalf::Lower);
    round_trip(&VALUES);
}

// ---------------------------------------------------------------------------------------------
// Extra fields and behaviour
// ---------------------------------------------------------------------------------------------

#[test]
fn tilt_causes_vibration_matches_the_jar() {
    use p::Tilt::{Tilt, VALUES};
    let fq = "net.minecraft.world.level.block.state.properties.Tilt";
    for v in VALUES {
        assert_enum_field(fq, v.name(), "causesVibration", &boolv(v.causes_vibration()));
    }
    // Only UNSTABLE is silent; NONE does vibrate.
    let silent: Vec<Tilt> = VALUES.into_iter().filter(|t| !t.causes_vibration()).collect();
    assert_eq!(silent, vec![Tilt::Unstable]);
    assert!(Tilt::None.causes_vibration());
}

#[test]
fn chest_type_get_opposite() {
    use p::ChestType::{ChestType, VALUES};
    assert_eq!(ChestType::Single.get_opposite(), ChestType::Single);
    assert_eq!(ChestType::Left.get_opposite(), ChestType::Right);
    assert_eq!(ChestType::Right.get_opposite(), ChestType::Left);
    for v in VALUES {
        assert_eq!(v.get_opposite().get_opposite(), v);
    }
}

#[test]
fn redstone_side_is_connected() {
    use p::RedstoneSide::RedstoneSide;
    assert!(RedstoneSide::Up.is_connected());
    assert!(RedstoneSide::Side.is_connected());
    assert!(!RedstoneSide::None.is_connected());
    assert_eq!(RedstoneSide::None.ordinal(), 2);
}

#[test]
fn rail_shape_get_name_and_is_slope() {
    use p::RailShape::{RailShape, VALUES};
    for v in VALUES {
        assert_eq!(v.get_name(), v.get_serialized_name());
        assert_eq!(v.to_string(), v.get_name());
        let expected_slope = matches!(
            v,
            RailShape::AscendingEast
                | RailShape::AscendingWest
                | RailShape::AscendingNorth
                | RailShape::AscendingSouth
        );
        assert_eq!(v.is_slope(), expected_slope, "{}", v.name());
        // In declaration order the four slopes are exactly ordinals 2..=5.
        assert_eq!(v.is_slope(), (2..=5).contains(&v.ordinal()));
        assert_eq!(v.is_slope(), v.get_name().starts_with("ascending_"));
    }
}

#[test]
fn side_chain_part_transitions() {
    use p::SideChainPart::{SideChainPart as S, VALUES};
    // isConnected: this != UNCONNECTED
    assert_eq!(VALUES.map(|v| v.is_connected()), [false, true, true, true]);
    // isChainEnd: this != CENTER (UNCONNECTED counts as an end)
    assert_eq!(VALUES.map(|v| v.is_chain_end()), [true, true, false, true]);
    // isConnectionTowards: this == CENTER || this == endPart
    for a in VALUES {
        for b in VALUES {
            assert_eq!(a.is_connection_towards(b), a == S::Center || a == b, "{a}.isConnectionTowards({b})");
        }
    }
    assert!(S::Unconnected.is_connection_towards(S::Unconnected));
    assert!(!S::Left.is_connection_towards(S::Right));
    // VALUES order: UNCONNECTED, RIGHT, CENTER, LEFT
    assert_eq!(
        VALUES.map(|v| v.when_connected_to_the_right()),
        [S::Left, S::Center, S::Center, S::Left]
    );
    assert_eq!(
        VALUES.map(|v| v.when_connected_to_the_left()),
        [S::Right, S::Right, S::Center, S::Center]
    );
    assert_eq!(
        VALUES.map(|v| v.when_disconnected_from_the_right()),
        [S::Unconnected, S::Right, S::Right, S::Unconnected]
    );
    assert_eq!(
        VALUES.map(|v| v.when_disconnected_from_the_left()),
        [S::Unconnected, S::Unconnected, S::Left, S::Left]
    );
}

#[test]
fn test_block_mode_id_and_by_id() {
    use p::TestBlockMode::{TestBlockMode, BY_ID, VALUES};
    let fq = "net.minecraft.world.level.block.state.properties.TestBlockMode";
    for v in VALUES {
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_eq!(BY_ID(v.id()), v);
    }
    // OutOfBoundsStrategy.ZERO: anything unknown is START (id 0).
    for bad in [-1, 4, 5, 100, i32::MIN, i32::MAX] {
        assert_eq!(BY_ID(bad), TestBlockMode::Start, "BY_ID({bad})");
    }
}

#[test]
fn note_block_instrument_type_matches_the_jar() {
    use p::NoteBlockInstrument::{NoteBlockInstrument as N, Type, TYPE_VALUES, VALUES};
    let fq = "net.minecraft.world.level.block.state.properties.NoteBlockInstrument";
    for v in VALUES {
        assert_enum_field(fq, v.name(), "type", &strv(v.get_type().name()));
        assert_eq!(v.is_tunable(), v.get_type() == Type::BaseBlock);
        assert_eq!(v.has_custom_sound(), v.get_type() == Type::Custom);
        assert_eq!(v.works_above_note_block(), v.get_type() != Type::BaseBlock);
    }
    let count = |t: Type| VALUES.iter().filter(|v| v.get_type() == t).count();
    assert_eq!((count(Type::BaseBlock), count(Type::MobHead), count(Type::Custom)), (20, 6, 1));
    assert!(N::Harp.is_tunable() && !N::Harp.works_above_note_block());
    assert!(N::Zombie.works_above_note_block() && !N::Zombie.is_tunable() && !N::Zombie.has_custom_sound());
    assert!(N::CustomHead.has_custom_sound() && N::CustomHead.works_above_note_block());
    // Nested Type (private in Java, not in the dump): declaration order from the source.
    assert_eq!(TYPE_VALUES.map(|t| t.name()), ["BASE_BLOCK", "MOB_HEAD", "CUSTOM"]);
    for (i, t) in TYPE_VALUES.iter().enumerate() {
        assert_eq!(t.ordinal(), i as i32);
        assert_eq!(t.to_string(), t.name());
    }
}

#[test]
fn name_lookup_threshold_branch() {
    // 27 instruments > PRE_BUILT_MAP_THRESHOLD (16): the HashMap branch; 10 rail shapes: linear.
    let big = create_name_lookup(&p::NoteBlockInstrument::VALUES);
    assert!(matches!(big, NameLookup::Map(_)));
    assert_eq!(big.apply("didgeridoo"), Some(p::NoteBlockInstrument::NoteBlockInstrument::Didgeridoo));
    assert_eq!(big.apply("DIDGERIDOO"), None);
    let small = create_name_lookup(&p::RailShape::VALUES);
    assert!(matches!(small, NameLookup::Linear(_)));
    assert_eq!(small.apply("north_east"), Some(p::RailShape::RailShape::NorthEast));
}

// ---------------------------------------------------------------------------------------------
// PORT-BLOCKED members must fail loudly, never return a placeholder
// ---------------------------------------------------------------------------------------------

fn panic_text(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let err = std::panic::catch_unwind(f).expect_err("PORT-BLOCKED member returned instead of panicking");
    err.downcast_ref::<String>()
        .cloned()
        .or_else(|| err.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default()
}

#[test]
fn port_blocked_members_panic() {
    let cases: Vec<(&str, String)> = vec![
        ("BedPart.CODEC", panic_text(|| p::BedPart::CODEC())),
        ("ChestType.CODEC", panic_text(|| p::ChestType::CODEC())),
        ("StructureMode.LEGACY_CODEC", panic_text(|| p::StructureMode::LEGACY_CODEC())),
        (
            "StructureMode.getDisplayName",
            panic_text(|| p::StructureMode::StructureMode::Save.get_display_name()),
        ),
        ("TestBlockMode.CODEC", panic_text(|| p::TestBlockMode::CODEC())),
        ("TestBlockMode.STREAM_CODEC", panic_text(|| p::TestBlockMode::STREAM_CODEC())),
        (
            "TestBlockMode.getDisplayName",
            panic_text(|| p::TestBlockMode::TestBlockMode::Log.get_display_name()),
        ),
        (
            "TestBlockMode.getDetailedMessage",
            panic_text(|| p::TestBlockMode::TestBlockMode::Log.get_detailed_message()),
        ),
        (
            "NoteBlockInstrument.getSoundEvent",
            panic_text(|| p::NoteBlockInstrument::NoteBlockInstrument::Harp.get_sound_event()),
        ),
    ];
    for (what, text) in cases {
        assert!(text.contains("PORT-BLOCKED"), "{what}: {text:?}");
    }
}
