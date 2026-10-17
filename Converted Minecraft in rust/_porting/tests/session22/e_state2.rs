//! Sub-batch D2: the `java_lang` additions (`signum`, `String#compareTo`, `Math.round(float)`) and
//! the classes built on them, plus the remaining small classes.
//!
//! Groups: `JSIGNUM`, `JCMP`, `JROUND`, `SMOOTH`, `RESULTFIELD`, `SEGCTOR`, `SEGDIR`, `SEGDEG`,
//! `SEGANG`, `PISTON`, `TRANSPOV`, `TRANSPOVNEW`, `TRANSPOVEQ`, `AVATAR`, `VISSET`, `INVSET`,
//! `CTXKEY`, `IDCONST`, `TICKTASK`, `GAMETEST`, `NETOUT`, `TEXSLOT`, `TEXSLOTNEW`, `OPTIONULL`,
//! `OPTIONULLNPE`, `CHUNKENT`. Scripted rows are replayed from the row, as in `d_state`.

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use crate::d_state::{debug_int, jfloat, panic_text};
use crate::small_probe::*;

use minecraft_rust::javacompat::java_lang;
use minecraft_rust::net::minecraft::client::data::models::model::TextureSlot::{self as texture_slot, TextureSlot};
use minecraft_rust::net::minecraft::client::entity::ClientAvatarState::ClientAvatarState;
use minecraft_rust::net::minecraft::client::renderer::chunk::TranslucencyPointOfView::TranslucencyPointOfView;
use minecraft_rust::net::minecraft::client::renderer::chunk::VisibilitySet::VisibilitySet;
use minecraft_rust::net::minecraft::commands::arguments::selector::options::InvertableSetOptionState::InvertableSetOptionState;
use minecraft_rust::net::minecraft::core::Direction::{self as direction, Direction};
use minecraft_rust::net::minecraft::core::SectionPos::SectionPos;
use minecraft_rust::net::minecraft::data::AtlasIds;
use minecraft_rust::net::minecraft::gametest::framework::GameTestEvent::GameTestEvent;
use minecraft_rust::net::minecraft::resources::Identifier::{self as identifier, Identifier};
use minecraft_rust::net::minecraft::server::rcon::NetworkDataOutputStream::NetworkDataOutputStream;
use minecraft_rust::net::minecraft::server::TickTask::TickTask;
use minecraft_rust::net::minecraft::util::context::ContextKey::ContextKey;
use minecraft_rust::net::minecraft::util::profiling::ResultField::ResultField;
use minecraft_rust::net::minecraft::util::SegmentedAnglePrecision::SegmentedAnglePrecision;
use minecraft_rust::net::minecraft::util::SmoothDouble::SmoothDouble;
use minecraft_rust::net::minecraft::world::entity::animal::TemperatureVariants;
use minecraft_rust::net::minecraft::world::level::block::piston::PistonMath;
use minecraft_rust::net::minecraft::world::level::entity::ChunkEntities::ChunkEntities;
use minecraft_rust::net::minecraft::world::level::ChunkPos::ChunkPos;
use minecraft_rust::net::minecraft::world::phys::Vec3::Vec3;
use minecraft_rust::net::minecraft::world::phys::AABB::AABB;
use minecraft_rust::net::minecraft::Optionull;

/// The probe's `CMP_STRINGS`.
const CMP_STRINGS: [&str; 16] = [
    "", "a", "b", "ab", "abc", "A", "\u{e9}", "\u{ff}", "\u{ffff}", "\u{1F600}", "zz", "aa", "a\u{0}", "\u{4e2d}\u{6587}", "Aa", "BB",
];

#[test]
fn java_lang_signum_compare_to_and_round_match_the_jdk() {
    for r in rows("JSIGNUM") {
        match r[1] {
            "d" => assert_eq!(hex64(java_lang::signum_f64(f64f(r[2]))), r[3], "Math.signum({}d)", r[2]),
            _ => assert_eq!(hex32(java_lang::signum_f32(f32f(r[2]))), r[3], "Math.signum({}f)", r[2]),
        }
    }
    for r in rows("JCMP") {
        let (a, b) = (CMP_STRINGS[i32f(r[1]) as usize], CMP_STRINGS[i32f(r[2]) as usize]);
        assert_eq!(java_lang::string_compare_to(a, b), i32f(r[3]), "{a:?}.compareTo({b:?})");
    }
    let rounds = rows("JROUND");
    assert!(rounds.len() > 5000, "the round corpus shrank");
    for r in rounds {
        assert_eq!(java_lang::round_f32(f32f(r[1])), i32f(r[2]), "Math.round({}f)", r[1]);
    }
}

#[test]
fn smooth_double_replays_the_jar_scripts() {
    for r in rows("SMOOTH") {
        let mut sm = SmoothDouble::new();
        let mut out = Vec::new();
        for op in r[1].split(' ') {
            if op == "R" {
                sm.reset();
                out.push("R".to_string());
            } else {
                let (a, b) = op.split_once(':').expect("pair");
                let (a, b): (f64, f64) = (a.parse().expect("double"), b.parse().expect("double"));
                out.push(hex64(sm.get_new_delta_value(a, b)));
            }
        }
        assert_eq!(out.join(","), r[2], "script {}", r[1]);
    }
}

#[test]
fn result_field_matches_the_jar() {
    let rows = rows("RESULTFIELD");
    let fields: Vec<ResultField> =
        rows.iter().map(|r| ResultField::new(&s(r[2]), f64f(r[3]), f64f(r[4]), i64f(r[5]))).collect();
    assert_eq!(fields.len(), 9);
    for (a, r) in rows.iter().enumerate() {
        let f = &fields[a];
        assert_eq!((f.percentage.to_bits(), f.global_percentage.to_bits(), f.count), (f64bits(r[3]), f64bits(r[4]), i64f(r[5])));
        assert_eq!(f.get_color(), i32f(r[6]), "{}.getColor()", f.name);
        let cmp: Vec<String> = fields.iter().map(|o| f.compare_to(o).to_string()).collect();
        assert_eq!(cmp.join(","), r[7], "compareTo row for {:?}", f.name);
    }
}

#[test]
fn segmented_angle_precision_matches_the_jar() {
    for r in rows("SEGCTOR") {
        let bits = i32f(r[1]);
        if let Some(ex) = r[2].strip_prefix("EX:") {
            assert_eq!(Some(panic_text(move || SegmentedAnglePrecision::new(bits))), unesc(ex), "new({bits})");
        } else {
            let _ = SegmentedAnglePrecision::new(bits);
        }
    }
    for r in rows("SEGDIR") {
        let sp = SegmentedAnglePrecision::new(i32f(r[1]));
        assert_eq!(sp.get_mask(), i32f(r[2]));
        let dirs: Vec<String> = direction::VALUES.iter().map(|d| sp.from_direction(*d).to_string()).collect();
        assert_eq!(dirs.join(","), r[3], "fromDirection for precision {}", r[1]);
    }
    for r in rows("SEGDEG") {
        let sp = SegmentedAnglePrecision::new(i32f(r[1]));
        let deg = f32f(r[2]);
        assert_eq!(sp.from_degrees_with_turns(deg), i32f(r[3]), "[{}].fromDegreesWithTurns({})", r[1], r[2]);
        assert_eq!(sp.from_degrees(deg), i32f(r[4]), "[{}].fromDegrees({})", r[1], r[2]);
    }
    let angles = [0, 1, -1, 2, 3, 4, 7, 8, 15, 16, 31, 64, 128, 255, 256, 1023, 1024, 65535, 65536, i32::MAX, i32::MIN];
    for r in rows("SEGANG") {
        let sp = SegmentedAnglePrecision::new(i32f(r[1]));
        let an = i32f(r[2]);
        assert_eq!(sp.normalize(an), i32f(r[3]));
        assert_eq!(hex32(sp.to_degrees_with_turns(an)), r[4], "[{}].toDegreesWithTurns({an})", r[1]);
        assert_eq!(hex32(sp.to_degrees(an)), r[5], "[{}].toDegrees({an})", r[1]);
        let axis: String = angles.iter().map(|&o| if sp.is_same_axis(an, o) { '1' } else { '0' }).collect();
        assert_eq!(axis, r[6], "[{}].isSameAxis({an}, *)", r[1]);
    }
}

#[test]
fn piston_movement_area_matches_the_jar() {
    let boxes = [
        AABB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
        AABB::new(-2.5, 64.0, 7.0, 3.25, 65.5, -0.0),
        AABB::new(-0.0, -0.0, -0.0, 0.0, 0.0, 0.0),
    ];
    let rows = rows("PISTON");
    assert_eq!(rows.len(), boxes.len() * 6 * 7);
    for r in rows {
        let dir = Direction::by_name(r[2]).expect("direction");
        let got = PistonMath::get_movement_area(&boxes[i32f(r[1]) as usize], dir, f64f(r[3]));
        let corners = [got.min_x, got.min_y, got.min_z, got.max_x, got.max_y, got.max_z].map(hex64);
        assert_eq!(corners.to_vec(), r[4..10].to_vec(), "box {} {} {}", r[1], r[2], r[3]);
    }
}

fn pov_xyz(p: &TranslucencyPointOfView) -> (i32, i32, i32) {
    let d = format!("{p:?}");
    (debug_int(&d, "x"), debug_int(&d, "y"), debug_int(&d, "z"))
}

#[test]
fn translucency_point_of_view_matches_the_jar() {
    let rows_p = rows("TRANSPOV");
    let mut povs = Vec::new();
    for r in &rows_p {
        let node = i64f(r[1]);
        let cam = Vec3::new(f64f(r[2]), f64f(r[3]), f64f(r[4]));
        let p = TranslucencyPointOfView::of(&cam, node);
        assert_eq!(pov_xyz(&p), (i32f(r[5]), i32f(r[6]), i32f(r[7])), "of({cam}, {node})");
        assert_eq!(p.is_axis_aligned(), boolf(r[8]));
        assert_eq!(p.java_hash_code(), i32f(r[9]));
        povs.push(p);
    }
    assert_eq!(povs.len(), 44);
    // The probe's section nodes are `SectionPos.asLong`s; the first is 0.
    assert_eq!(SectionPos::as_long_xyz(1, 2, 3), i64f(rows_p[11][1]));
    let r = &rows("TRANSPOVNEW")[0];
    let mut fresh = TranslucencyPointOfView::new();
    assert_eq!(pov_xyz(&fresh), (i32f(r[1]), i32f(r[2]), i32f(r[3])));
    assert_eq!(fresh.is_axis_aligned(), boolf(r[4]));
    assert_eq!(fresh.java_hash_code(), i32f(r[5]));
    assert_eq!(fresh == povs[0], boolf(r[6]));
    let p: *const TranslucencyPointOfView = &fresh;
    assert_eq!(std::ptr::eq(fresh.set(&Vec3::new(20.0, 20.0, 20.0), 0), p), boolf(r[7]), "set returns this");
    for r in rows("TRANSPOVEQ") {
        let a = i32f(r[1]) as usize;
        let eq: String = (0..8).map(|c| if povs[a] == povs[c] { '1' } else { '0' }).collect();
        assert_eq!(eq, r[2], "equals row {a}");
    }
}

#[test]
fn client_avatar_state_replays_the_jar_script() {
    let r = &rows("AVATAR")[0];
    let mut av = ClientAvatarState::new();
    let mut out: Vec<String> = Vec::new();
    for op in r[1].split(' ') {
        let p: Vec<&str> = op.split(':').collect();
        let num = |s: &str| -> f64 { s.parse().expect("double") };
        match p[0] {
            "T" => av.tick(&Vec3::new(num(p[1]), num(p[2]), num(p[3])), &Vec3::new(0.25, 0.0, 0.0)),
            "W" => av.add_walk_distance(jfloat(p[1])),
            "B" => av.update_bob(jfloat(p[1])),
            _ => av.reset_bob(),
        }
        let mut s = av.delta_movement_on_previous_tick().to_string();
        for pt in [0.0f32, 0.5, 1.0] {
            s.push_str(&format!(
                ",{},{},{},{},{},{}",
                hex64(av.get_interpolated_cloak_x(pt)),
                hex64(av.get_interpolated_cloak_y(pt)),
                hex64(av.get_interpolated_cloak_z(pt)),
                hex32(av.get_interpolated_bob(pt)),
                hex32(av.get_backwards_interpolated_walk_distance(pt)),
                hex32(av.get_interpolated_walk_distance(pt))
            ));
        }
        out.push(s);
    }
    assert_eq!(out.join(";"), r[2]);
}

#[test]
fn visibility_set_replays_the_jar_scripts() {
    for r in rows("VISSET") {
        let mut vs = VisibilitySet::new();
        let mut out = Vec::new();
        for op in r[1].split(' ') {
            let p: Vec<&str> = op.split(':').collect();
            match p[0] {
                "A" => {
                    let set: Vec<Direction> =
                        p[1].split(',').filter(|n| !n.is_empty()).map(|n| Direction::by_name(n).expect("direction")).collect();
                    vs.add(&set);
                }
                "S" => vs.set(Direction::by_name(p[1]).unwrap(), Direction::by_name(p[2]).unwrap(), p[3] == "1"),
                _ => vs.set_all(p[1] == "1"),
            }
            let mut m = String::new();
            for a in direction::VALUES {
                for c in direction::VALUES {
                    m.push(if vs.visibility_between(a, c) { '1' } else { '0' });
                }
            }
            out.push(format!("{m}|{}", esc_like_probe(&vs.to_string())));
        }
        assert_eq!(out.join(";"), r[2], "script {}", r[1]);
    }
}

/// The probe's `esc`, forwards, for comparing a Rust string with an escaped row field.
fn esc_like_probe(s: &str) -> String {
    let mut out = String::new();
    for unit in s.encode_utf16() {
        if unit == '\\' as u16 {
            out.push_str("\\\\");
        } else if (0x20..=0x7e).contains(&unit) {
            out.push(unit as u8 as char);
        } else {
            out.push_str(&format!("\\u{unit:04x}"));
        }
    }
    out
}

#[test]
fn invertable_set_option_state_replays_the_jar_scripts() {
    let a = identifier::with_default_namespace("a");
    let b = identifier::with_default_namespace("b");
    let state = |st: &InvertableSetOptionState| -> String {
        [
            st.can_parse_positive_element(),
            st.can_parse_negative_element(),
            st.can_parse_any_tag(),
            st.can_parse_any(),
            st.can_parse_element(false),
            st.can_parse_element(true),
            st.can_parse_tag(&a),
            st.can_parse_tag(&b),
        ]
        .iter()
        .map(|&x| if x { '1' } else { '0' })
        .collect()
    };
    for r in rows("INVSET") {
        let mut st = InvertableSetOptionState::new();
        let mut out = vec![state(&st)];
        for op in r[1].split(' ') {
            let p: Vec<&str> = op.split(':').collect();
            match p[0] {
                "p" => st.mark_parsed_positive_element(),
                "n" => st.mark_parsed_negative_element(),
                "t" => st.mark_parsed_tag(identifier::with_default_namespace(p[1])),
                _ => st.mark_parsed_element(p[1] == "1"),
            }
            out.push(state(&st));
        }
        assert_eq!(out.join(";"), r[2], "script {}", r[1]);
    }
}

#[test]
fn context_keys_and_identifier_constants_match_the_jar() {
    for r in rows("CTXKEY") {
        let k: ContextKey<()> = ContextKey::vanilla(r[1]);
        let k2: ContextKey<()> = ContextKey::vanilla(r[1]);
        assert_eq!(Some(k.to_string()), unesc(r[2]));
        assert_eq!(k.name().to_string(), r[3]);
        // Java: two keys with one name are different keys (identity).
        assert_eq!(std::ptr::eq(&k, &k2), boolf(r[4]));
    }
    let consts: Vec<(&str, &str, &Identifier)> = vec![
        ("AtlasIds", "ARMOR_TRIMS", &AtlasIds::ARMOR_TRIMS),
        ("AtlasIds", "BANNER_PATTERNS", &AtlasIds::BANNER_PATTERNS),
        ("AtlasIds", "BLOCKS", &AtlasIds::BLOCKS),
        ("AtlasIds", "ITEMS", &AtlasIds::ITEMS),
        ("AtlasIds", "CHESTS", &AtlasIds::CHESTS),
        ("AtlasIds", "DECORATED_POT", &AtlasIds::DECORATED_POT),
        ("AtlasIds", "GUI", &AtlasIds::GUI),
        ("AtlasIds", "MAP_DECORATIONS", &AtlasIds::MAP_DECORATIONS),
        ("AtlasIds", "PAINTINGS", &AtlasIds::PAINTINGS),
        ("AtlasIds", "PARTICLES", &AtlasIds::PARTICLES),
        ("AtlasIds", "SHIELD_PATTERNS", &AtlasIds::SHIELD_PATTERNS),
        ("AtlasIds", "SHULKER_BOXES", &AtlasIds::SHULKER_BOXES),
        ("AtlasIds", "CELESTIALS", &AtlasIds::CELESTIALS),
        ("TemperatureVariants", "TEMPERATE", &TemperatureVariants::TEMPERATE),
        ("TemperatureVariants", "WARM", &TemperatureVariants::WARM),
        ("TemperatureVariants", "COLD", &TemperatureVariants::COLD),
    ];
    let mut got: Vec<(String, String, String)> =
        consts.iter().map(|(c, n, v)| (c.to_string(), n.to_string(), v.to_string())).collect();
    let mut want: Vec<(String, String, String)> =
        rows("IDCONST").iter().map(|r| (r[1].to_string(), r[2].to_string(), r[3].to_string())).collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "the jar's static Identifier fields, by reflection");
}

#[test]
fn tick_task_and_game_test_event_match_the_jar() {
    for r in rows("TICKTASK") {
        let calls = Arc::new(AtomicI32::new(0));
        let c = calls.clone();
        let mut task = TickTask::new(i32f(r[1]), move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
        task.run();
        task.run();
        assert_eq!(task.get_tick(), i32f(r[2]));
        assert_eq!(calls.load(Ordering::SeqCst), i32f(r[3]), "run() runs the runnable once per call");
    }
    let rows = rows("GAMETEST");
    let calls = Arc::new(AtomicI32::new(0));
    let make = |c: &Arc<AtomicI32>| {
        let c = c.clone();
        move || {
            c.fetch_add(1, Ordering::SeqCst);
        }
    };
    let mut events = [
        ("create", GameTestEvent::create(make(&calls))),
        ("create-long", GameTestEvent::create_i64(-5, make(&calls))),
        ("createWithMinimumDelay", GameTestEvent::create_with_minimum_delay(i64::MAX, make(&calls))),
    ];
    for (name, ev) in events.iter_mut() {
        let r = rows.iter().find(|r| r[1] == *name).expect("GAMETEST row");
        assert_eq!(ev.expected_delay.map(|v| v.to_string()), unesc(r[2]), "{name}: expectedDelay");
        assert_eq!(ev.minimum_delay.map(|v| v.to_string()), unesc(r[3]), "{name}: minimumDelay");
        // Java: the stored assertion IS the runnable passed in. Here: running it runs ours.
        let before = calls.load(Ordering::SeqCst);
        (ev.assertion)();
        assert_eq!(calls.load(Ordering::SeqCst), before + 1);
        assert!(boolf(r[4]));
    }
}

#[test]
fn network_data_output_stream_replays_the_jar_script() {
    let r = &rows("NETOUT")[0];
    let script = s(r[1]);
    let mut nd = NetworkDataOutputStream::new(4);
    let mut out = Vec::new();
    for op in script.split(' ') {
        let (k, v) = op.split_once(':').unwrap_or((op, ""));
        match k {
            "b" => {
                let bytes: Vec<u8> = (0..v.len() / 2).map(|i| u8::from_str_radix(&v[2 * i..2 * i + 2], 16).unwrap()).collect();
                nd.write_bytes(&bytes);
            }
            "i" => nd.write_int(v.parse().unwrap()),
            "s" => nd.write_short(v.parse().unwrap()),
            "f" => nd.write_float(jfloat(v)),
            // A NaN with a payload (or the negative quiet NaN): `floatToIntBits` must canonicalise it.
            "F" => nd.write_float(f32::from_bits(u32::from_str_radix(v, 16).unwrap())),
            "w" => nd.write(v.parse().unwrap()),
            "S" => nd.write_string(v),
            _ => nd.reset(),
        }
        let hex: String = nd.to_byte_array().iter().map(|b| format!("{b:02x}")).collect();
        out.push(if hex.is_empty() { "-".to_string() } else { hex });
    }
    assert_eq!(out.join(","), r[2]);
}

#[test]
fn texture_slots_match_the_jar() {
    let all: Vec<(&str, &TextureSlot)> = vec![
        ("ALL", &texture_slot::ALL), ("TEXTURE", &texture_slot::TEXTURE), ("PARTICLE", &texture_slot::PARTICLE),
        ("END", &texture_slot::END), ("BOTTOM", &texture_slot::BOTTOM), ("TOP", &texture_slot::TOP),
        ("FRONT", &texture_slot::FRONT), ("BACK", &texture_slot::BACK), ("SIDE", &texture_slot::SIDE),
        ("NORTH", &texture_slot::NORTH), ("SOUTH", &texture_slot::SOUTH), ("EAST", &texture_slot::EAST),
        ("WEST", &texture_slot::WEST), ("UP", &texture_slot::UP), ("DOWN", &texture_slot::DOWN),
        ("CROSS", &texture_slot::CROSS), ("CROSS_EMISSIVE", &texture_slot::CROSS_EMISSIVE), ("PLANT", &texture_slot::PLANT),
        ("WALL", &texture_slot::WALL), ("RAIL", &texture_slot::RAIL), ("WOOL", &texture_slot::WOOL),
        ("PATTERN", &texture_slot::PATTERN), ("PANE", &texture_slot::PANE), ("EDGE", &texture_slot::EDGE),
        ("FAN", &texture_slot::FAN), ("STEM", &texture_slot::STEM), ("UPPER_STEM", &texture_slot::UPPER_STEM),
        ("CROP", &texture_slot::CROP), ("DIRT", &texture_slot::DIRT), ("FIRE", &texture_slot::FIRE),
        ("LANTERN", &texture_slot::LANTERN), ("PLATFORM", &texture_slot::PLATFORM), ("UNSTICKY", &texture_slot::UNSTICKY),
        ("TORCH", &texture_slot::TORCH), ("LAYER0", &texture_slot::LAYER0), ("LAYER1", &texture_slot::LAYER1),
        ("LAYER2", &texture_slot::LAYER2), ("LIT_LOG", &texture_slot::LIT_LOG), ("CANDLE", &texture_slot::CANDLE),
        ("INSIDE", &texture_slot::INSIDE), ("CONTENT", &texture_slot::CONTENT), ("INNER_TOP", &texture_slot::INNER_TOP),
        ("FLOWERBED", &texture_slot::FLOWERBED), ("TENTACLES", &texture_slot::TENTACLES), ("BARS", &texture_slot::BARS),
    ];
    let rows_t = rows("TEXSLOT");
    assert_eq!(rows_t.len(), all.len(), "the jar has a different number of TextureSlot constants");
    for (r, (name, slot)) in rows_t.iter().zip(all.iter()) {
        assert_eq!(r[1], *name);
        assert_eq!(slot.get_id(), r[2], "{name}.getId()");
        assert_eq!(slot.get_parent().map(|p| p.get_id().to_string()), unesc(r[3]), "{name}.getParent()");
        assert_eq!(slot.to_string(), r[4], "{name}.toString()");
    }
    // The parent is the very constant, not a copy: NORTH's parent IS SIDE.
    assert!(std::ptr::eq(texture_slot::NORTH.get_parent().unwrap(), &texture_slot::SIDE));
    let r = &rows("TEXSLOTNEW")[0];
    assert_eq!(r[1], "#all");
    assert!(!boolf(r[2]) && !boolf(r[3]), "a fresh create(\"all\") is neither == nor equals ALL");
    assert_eq!(r[4], "side");
    assert_eq!((r[5], r[6]), ("private static", "private static"), "create is private in the jar");
}

#[test]
#[allow(deprecated)]
fn optionull_matches_the_jar() {
    let opt = rows("OPTIONULL");
    let find = |k: &str| opt.iter().find(|r| r[1] == k).unwrap_or_else(|| panic!("no OPTIONULL {k}"));
    let some = vec!["first", "second"];
    let none: Vec<&str> = vec![];
    let null_str = |v: Option<String>| v.unwrap_or_else(|| "null".to_string());

    let r = find("orElse");
    assert_eq!((Optionull::or_else(None, "d"), Optionull::or_else(Some("v"), "d")), (r[2], r[3]));
    let r = find("map");
    assert_eq!(
        (null_str(Optionull::map(None::<&str>, |s| s.len().to_string())), null_str(Optionull::map(Some("abc"), |s| s.len().to_string()))),
        (r[2].to_string(), r[3].to_string())
    );
    let r = find("mapOrDefault");
    assert_eq!(
        (Optionull::map_or_default(None::<&str>, |s| s.len() as i32, -1), Optionull::map_or_default(Some("abcd"), |s| s.len() as i32, -1)),
        (i32f(r[2]), i32f(r[3]))
    );
    let r = find("mapOrElse");
    assert_eq!(
        (Optionull::map_or_else(None::<&str>, |s| s.len() as i32, || -2), Optionull::map_or_else(Some("ab"), |s| s.len() as i32, || -2)),
        (i32f(r[2]), i32f(r[3]))
    );
    let r = find("first");
    assert_eq!(
        (null_str(Optionull::first(none.iter()).map(|s| s.to_string())), null_str(Optionull::first(some.iter()).map(|s| s.to_string()))),
        (r[2].to_string(), r[3].to_string())
    );
    let r = find("firstOrDefault");
    assert_eq!((Optionull::first_or_default(none.clone(), "d"), Optionull::first_or_default(some.clone(), "d")), (r[2], r[3]));
    let r = find("firstOrElse");
    assert_eq!((Optionull::first_or_else(none.clone(), || "e"), Optionull::first_or_else(some.clone(), || "e")), (r[2], r[3]));
    let r = find("isNullOrEmpty");
    assert_eq!(
        [
            Optionull::is_null_or_empty::<i32>(None),
            Optionull::is_null_or_empty::<i32>(Some(&[])),
            Optionull::is_null_or_empty(Some(&[1])),
            Optionull::is_null_or_empty::<String>(Some(&[])),
            Optionull::is_null_or_empty(Some(&[0.0f64])),
        ]
        .to_vec(),
        r[2..7].iter().map(|v| boolf(v)).collect::<Vec<bool>>()
    );
    assert_eq!(rows("OPTIONULLNPE")[0][1], "java.lang.NullPointerException: defaultObj");
}

#[test]
fn chunk_entities_match_the_jar() {
    let r = &rows("CHUNKENT")[0];
    let ce = ChunkEntities::new(ChunkPos::new(3, -4), vec!["a", "b"]);
    let empty: ChunkEntities<&str> = ChunkEntities::new(ChunkPos::new(0, 0), vec![]);
    assert_eq!(ce.get_pos().to_string(), r[1]);
    assert_eq!(ce.get_entities().copied().collect::<Vec<&str>>().join(","), r[2]);
    assert_eq!(ce.is_empty(), boolf(r[3]));
    assert_eq!(empty.is_empty(), boolf(r[4]));
}
