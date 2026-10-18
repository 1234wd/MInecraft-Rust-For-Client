//! Sub-batch D1: small stateful classes.
//!
//! Groups: `DOTS`, `MATDEF`, `SETONCE`, `FFUNIV`, `SAVEDDATA`, `SPVCDIM`, `THROTTLE`, `SCISSOR`,
//! `BDPSET`, `BDP`, `STD140`, `ANIM`, `WALK`, `CHESTLID`, `CUBEDEF`, `BEAMSEC`.
//!
//! Where a row carries a script (`THROTTLE`, `ANIM`, `WALK`, `CHESTLID`, `STD140`, `SAVEDDATA`),
//! the test REPLAYS that script -- it does not hold its own copy -- so the two sides cannot drift.
//! Private Java fields that the probe read by reflection (`TickThrottler.count`,
//! `AnimationState.startTick`) are read here from the struct's derived `Debug` text, so the ports
//! carry no test-only accessors.

use crate::small_probe::*;

use minecraft_rust::com::mojang::blaze3d::buffers::Std140SizeCalculator::Std140SizeCalculator;
use minecraft_rust::com::mojang::blaze3d::systems::ScissorState::ScissorState;
use minecraft_rust::com::mojang::blaze3d::vulkan::glsl::SpvcUtil;
use minecraft_rust::net::minecraft::client::gui::screens::LoadingDotsText;
use minecraft_rust::net::minecraft::client::model::geom::builders::CubeDeformation::CubeDeformation;
use minecraft_rust::net::minecraft::client::model::geom::builders::MaterialDefinition::MaterialDefinition;
use minecraft_rust::net::minecraft::commands::arguments::selector::options::SetOnceOptionState::SetOnceOptionState;
use minecraft_rust::net::minecraft::core::BlockPos::BlockPos;
use minecraft_rust::net::minecraft::server::level::BlockDestructionProgress::BlockDestructionProgress;
use minecraft_rust::net::minecraft::util::TickThrottler::TickThrottler;
use minecraft_rust::net::minecraft::world::entity::AnimationState::AnimationState;
use minecraft_rust::net::minecraft::world::entity::WalkAnimationState::WalkAnimationState;
use minecraft_rust::net::minecraft::world::flag::FeatureFlagUniverse::FeatureFlagUniverse;
use minecraft_rust::net::minecraft::world::level::block::entity::BeaconBeamOwner::Section;
use minecraft_rust::net::minecraft::world::level::block::entity::ChestLidController::ChestLidController;
use minecraft_rust::net::minecraft::world::level::saveddata::SavedData::SavedData;

/// The text of a panic raised by `f`, for comparing with a Java exception's `class: message`.
pub fn panic_text<R>(f: impl FnOnce() -> R + std::panic::UnwindSafe) -> String {
    let err = std::panic::catch_unwind(f)
        .err()
        .expect("expected the call to panic, as Java throws");
    if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        panic!("panic payload is not a string")
    }
}

/// `field: <int>` out of a derived `Debug` string.
pub fn debug_int(debug: &str, field: &str) -> i32 {
    let key = format!("{field}: ");
    let at = debug
        .find(&key)
        .unwrap_or_else(|| panic!("no `{field}` in {debug}"))
        + key.len();
    let end = debug[at..]
        .find(|c: char| c != '-' && !c.is_ascii_digit())
        .map_or(debug.len(), |e| at + e);
    debug[at..end]
        .parse()
        .unwrap_or_else(|_| panic!("`{field}` is not an int in {debug}"))
}

/// Java's `Float.parseFloat` for the spellings the scripts use (`NaN`, `Infinity`, `1.0E9` ...),
/// all of which Rust's correctly-rounded `parse` reads to the same value.
pub fn jfloat(s: &str) -> f32 {
    s.parse().unwrap_or_else(|_| panic!("not a float: {s:?}"))
}

#[test]
fn loading_dots_text_matches_the_jar_including_its_negative_index() {
    for r in rows("DOTS") {
        let t = i64f(r[1]);
        if let Some(ex) = r[2].strip_prefix("EX:") {
            assert_eq!(
                Some(panic_text(move || LoadingDotsText::get(t))),
                unesc(ex),
                "get({t})"
            );
        } else {
            assert_eq!(
                Some(LoadingDotsText::get(t).to_string()),
                unesc(r[2]),
                "get({t})"
            );
        }
    }
}

#[test]
fn material_definition_and_cube_deformation_fields_match_the_jar() {
    for r in rows("MATDEF") {
        let m = MaterialDefinition::new(i32f(r[1]), i32f(r[2]));
        assert_eq!((m.x_tex_size, m.y_tex_size), (i32f(r[3]), i32f(r[4])));
    }
    let cds = [
        CubeDeformation::NONE,
        CubeDeformation::uniform(0.5),
        CubeDeformation::new(-0.0, f32::NAN, 1e30),
        CubeDeformation::uniform(1.0).extend_f32(0.25),
        CubeDeformation::uniform(1.0).extend_f32_f32_f32(0.1, -2.0, f32::MAX),
        CubeDeformation::NONE.extend_f32(-0.0),
        CubeDeformation::uniform(f32::MAX).extend_f32(f32::MAX),
    ];
    let rows = rows("CUBEDEF");
    assert_eq!(rows.len(), cds.len());
    for (r, c) in rows.iter().zip(cds.iter()) {
        assert_eq!(
            (hex32(c.grow_x), hex32(c.grow_y), hex32(c.grow_z)),
            (r[2].to_string(), r[3].to_string(), r[4].to_string()),
            "deformation[{}]",
            r[1]
        );
    }
}

#[test]
fn set_once_and_saved_data_flags_match_the_jar() {
    let mut so = SetOnceOptionState::new();
    let mut bits = String::new();
    bits.push(if so.can_parse() { '1' } else { '0' });
    so.mark_parsed();
    bits.push(if so.can_parse() { '1' } else { '0' });
    so.mark_parsed();
    bits.push(if so.can_parse() { '1' } else { '0' });
    assert_eq!(bits, rows("SETONCE")[0][1]);

    let r = &rows("SAVEDDATA")[0];
    let mut sd = SavedData::new();
    let mut bits = String::new();
    bits.push(if sd.is_dirty() { '1' } else { '0' });
    for op in r[1].split(' ') {
        match op {
            "D" => sd.set_dirty(),
            "t" => sd.set_dirty_bool(true),
            _ => sd.set_dirty_bool(false),
        }
        bits.push(if sd.is_dirty() { '1' } else { '0' });
    }
    assert_eq!(bits, r[2], "SavedData script {}", r[1]);
}

#[test]
fn feature_flag_universe_prints_its_id_and_compares_by_identity() {
    for r in rows("FFUNIV") {
        let id = s(r[1]);
        let a = FeatureFlagUniverse::new(&id);
        let b = FeatureFlagUniverse::new(&id);
        assert_eq!(Some(a.to_string()), unesc(r[2]));
        // Java: two universes with one id are NOT equal; one is equal to itself. Identity.
        assert_eq!(std::ptr::eq(&a, &b), boolf(r[3]));
        assert_eq!(std::ptr::eq(&a, &a), boolf(r[4]));
    }
}

#[test]
fn spvc_image_dimension_names_match_the_jar() {
    for r in rows("SPVCDIM") {
        assert_eq!(
            Some(SpvcUtil::image_dimension_to_string(i32f(r[1]))),
            unesc(r[2]),
            "dimension {}",
            r[1]
        );
    }
}

#[test]
fn tick_throttler_replays_the_jar_script() {
    let rows = rows("THROTTLE");
    assert_eq!(rows.len(), 8);
    for r in rows {
        let mut th = TickThrottler::new(i32f(r[1]), i32f(r[2]));
        let sample = |th: &TickThrottler| {
            format!(
                "{}:{}",
                if th.is_under_threshold() { 1 } else { 0 },
                debug_int(&format!("{th:?}"), "count")
            )
        };
        let mut out = vec![sample(&th)];
        for op in r[3].split(' ') {
            if op == "I" {
                th.increment();
            } else {
                th.tick();
            }
            out.push(sample(&th));
        }
        assert_eq!(out.join(","), r[4], "TickThrottler({}, {})", r[1], r[2]);
    }
}

#[test]
fn scissor_state_matches_the_jar() {
    let show = |s: &ScissorState| {
        format!(
            "{},{},{},{},{}",
            s.enabled(),
            s.x(),
            s.y(),
            s.width(),
            s.height()
        )
    };
    let rows = rows("SCISSOR");
    let find = |k: &str| {
        rows.iter()
            .find(|r| r[1] == k)
            .unwrap_or_else(|| panic!("no SCISSOR {k}"))[2]
            .to_string()
    };
    let mut sc = ScissorState::new();
    assert_eq!(show(&sc), find("new"));
    sc.enable(1, -2, 300, i32::MAX);
    assert_eq!(show(&sc), find("enable"));
    let mut copy = ScissorState::copy_of(&sc);
    assert_eq!(show(&copy), find("copy"));
    sc.disable();
    assert_eq!(show(&sc), find("disable"));
    assert_eq!(show(&copy), find("copy-after-disable"));
    copy.set_from(&sc);
    assert_eq!(show(&copy), find("setFrom"));
}

#[test]
fn block_destruction_progress_matches_the_jar() {
    for r in rows("BDPSET") {
        let mut b = BlockDestructionProgress::new(7, BlockPos::new(1, 2, 3));
        b.set_progress(i32f(r[1]));
        assert_eq!(b.get_progress(), i32f(r[2]), "setProgress({})", r[1]);
    }
    let bdv = [(1, 0), (1, 9), (2, 0), (2, 9), (i32::MIN, 3), (i32::MAX, 3)];
    let bdps: Vec<BlockDestructionProgress> = bdv
        .iter()
        .enumerate()
        .map(|(k, &(id, progress))| {
            let mut b = BlockDestructionProgress::new(id, BlockPos::new(k as i32, -(k as i32), 0));
            b.set_progress(progress);
            b.update_tick(1000 * k as i64 - 1);
            b
        })
        .collect();
    let rows = rows("BDP");
    assert_eq!(rows.len(), bdps.len());
    for (a, r) in rows.iter().enumerate() {
        let x = &bdps[a];
        assert_eq!(x.get_id(), i32f(r[2]));
        assert_eq!(x.get_pos().to_string(), r[3]);
        assert_eq!(x.get_progress(), i32f(r[4]));
        assert_eq!(x.get_updated_render_tick(), i64f(r[5]));
        assert_eq!(x.java_hash_code(), i32f(r[6]), "hashCode of [{a}]");
        let eq: String = bdps
            .iter()
            .map(|y| if x == y { '1' } else { '0' })
            .collect();
        assert_eq!(eq, r[7], "equals row [{a}]");
        let cmp: Vec<String> = bdps.iter().map(|y| x.compare_to(y).to_string()).collect();
        assert_eq!(cmp.join(","), r[8], "compareTo row [{a}]");
    }
}

#[test]
fn std140_size_calculator_replays_the_jar_scripts() {
    for r in rows("STD140") {
        let mut c = Std140SizeCalculator::new();
        let mut out = Vec::new();
        for op in r[1].split(' ') {
            match op {
                "f" => c.put_float(),
                "i" => c.put_int(),
                "v2" => c.put_vec2(),
                "iv2" => c.put_ivec2(),
                "v3" => c.put_vec3(),
                "iv3" => c.put_ivec3(),
                "v4" => c.put_vec4(),
                "iv4" => c.put_ivec4(),
                "m4" => c.put_mat4f(),
                a => c.align(i32f(&a[1..])),
            };
            out.push(c.get().to_string());
        }
        assert_eq!(out.join(","), r[2], "script {}", r[1]);
    }
}

#[test]
fn animation_state_replays_the_jar_scripts() {
    for r in rows("ANIM") {
        let mut st = AnimationState::new();
        let mut other = AnimationState::new();
        other.start(77);
        let mut out: Vec<String> = Vec::new();
        for op in r[1].split(' ') {
            let p: Vec<&str> = op.split(':').collect();
            match p[0] {
                "s" => st.start(i32f(p[1])),
                "i" => st.start_if_stopped(i32f(p[1])),
                "a" => st.animate_when(p[1] == "1", i32f(p[2])),
                "x" => st.stop(),
                "f" => st.fast_forward(i32f(p[1]), jfloat(p[2])),
                "c" => st.copy_from(&other),
                "q" => {
                    let mut calls = 0;
                    st.if_started(|_| calls += 1);
                    out.push(format!(
                        "{}{}@{}",
                        if st.is_started() { "S" } else { "-" },
                        calls,
                        debug_int(&format!("{st:?}"), "start_tick")
                    ));
                }
                "t" => out.push(format!("ms={}", st.get_time_in_millis(jfloat(p[1])))),
                other => panic!("unknown ANIM op {other}"),
            }
        }
        assert_eq!(out.join(","), r[2], "script {}", r[1]);
    }
}

#[test]
fn walk_animation_state_replays_the_jar_scripts() {
    let partials = [0.0f32, 0.5, 1.0, -1.0, 2.0];
    for r in rows("WALK") {
        let mut w = WalkAnimationState::new();
        let mut out: Vec<String> = Vec::new();
        for op in r[1].split(' ') {
            let p: Vec<&str> = op.split(':').collect();
            match p[0] {
                "u" => w.update(jfloat(p[1]), jfloat(p[2]), jfloat(p[3])),
                "p" => w.set_speed(jfloat(p[1])),
                _ => w.stop(),
            }
            let mut s = format!(
                "{},{},{}",
                hex32(w.speed()),
                hex32(w.position()),
                if w.is_moving() { 1 } else { 0 }
            );
            for &pt in &partials {
                s.push_str(&format!(
                    ",{},{}",
                    hex32(w.speed_f32(pt)),
                    hex32(w.position_f32(pt))
                ));
            }
            out.push(s);
        }
        assert_eq!(out.join(";"), r[2], "script {}", r[1]);
    }
}

#[test]
fn chest_lid_controller_replays_the_jar_script() {
    let r = &rows("CHESTLID")[0];
    let mut lid = ChestLidController::new();
    let mut out: Vec<String> = Vec::new();
    for op in r[1].split(' ') {
        match op {
            "o" => lid.should_be_open(true),
            "c" => lid.should_be_open(false),
            _ => lid.tick_lid(),
        }
        out.push(format!(
            "{},{},{}",
            hex32(lid.get_openness(0.0)),
            hex32(lid.get_openness(0.5)),
            hex32(lid.get_openness(1.0))
        ));
    }
    assert_eq!(out.join(";"), r[2]);
}

#[test]
fn beacon_beam_sections_match_the_jar() {
    for r in rows("BEAMSEC") {
        let mut s = Section::new(i32f(r[1]));
        assert_eq!(s.get_color(), i32f(r[2]));
        let mut heights = vec![s.get_height().to_string()];
        for _ in 0..3 {
            s.increase_height();
            heights.push(s.get_height().to_string());
        }
        assert_eq!(heights.join(","), r[3]);
    }
}
