//! Sub-batch E: interfaces whose default methods, constants and static factories carry behaviour.
//!
//! Groups: `TABORDER`, `HOLDEROWNER`, `ROTTOWARDS`, `PALRESIZE`, `RESHANDLE`, `GLYPH`, `CHARPRED`,
//! `CHARPREDSURR`, `BOOLOP`, `LIGHTEVT`, `RIDEJUMP`, `HEIGHTACC`, `SELENTRY`, `CRCEMPTY`,
//! `CRCCHAIN`, `CRCCALLS`, `LERPF`, `LERPI`, `LERPCONST`, `BOUNDEDF`, `ABORTCONT`, `ABORTFOR`,
//! `REPEVENT`, `REPEVENTSAME`, `PACKRATCTL`, `TIMESRC`, `SKIPALL`.
//!
//! Each interface is exercised through a Rust implementation built the way the probe built its
//! anonymous Java one: only the abstract methods are written, so every default under test is the
//! port's.

use std::sync::{Arc, Mutex};

use crate::d_state::panic_text;
use crate::small_probe::*;

use minecraft_rust::com::mojang::blaze3d::font::GlyphInfo::{self as glyph_info, GlyphInfo};
use minecraft_rust::com::mojang::blaze3d::resource::ResourceHandle::{
    self as resource_handle, ResourceHandle,
};
use minecraft_rust::javacompat::time_unit::TimeUnit;
use minecraft_rust::net::minecraft::client::gui::components::SelectableEntry::SelectableEntry;
use minecraft_rust::net::minecraft::client::gui::components::TabOrderedElement::TabOrderedElement;
use minecraft_rust::net::minecraft::commands::CommandResultCallback::{
    self as crc, CommandResultCallback,
};
use minecraft_rust::net::minecraft::core::BlockPos::BlockPos;
use minecraft_rust::net::minecraft::core::HolderOwner::HolderOwner;
use minecraft_rust::net::minecraft::core::SectionPos::SectionPos;
use minecraft_rust::net::minecraft::nbt::visitors::SkipAll;
use minecraft_rust::net::minecraft::nbt::StreamTagVisitor::StreamTagVisitor;
use minecraft_rust::net::minecraft::nbt::TagTypes;
use minecraft_rust::net::minecraft::util::parsing::packrat::Control::{
    self as packrat_control, Control as PackratControl,
};
use minecraft_rust::net::minecraft::util::AbortableIterationConsumer::{
    self as abortable, AbortableIterationConsumer, Continuation,
};
use minecraft_rust::net::minecraft::util::BoundedFloatFunction::{
    self as bounded, BoundedFloatFunction,
};
use minecraft_rust::net::minecraft::util::TimeSource::{
    self as time_source, NanoTimeSource, TimeSource,
};
use minecraft_rust::net::minecraft::world::attribute::LerpFunction::{self as lerp, LerpFunction};
use minecraft_rust::net::minecraft::world::entity::ai::control::Control::Control;
use minecraft_rust::net::minecraft::world::entity::ai::village::ReputationEventType::{
    self as reputation, Registered,
};
use minecraft_rust::net::minecraft::world::entity::PlayerRideable::PlayerRideable;
use minecraft_rust::net::minecraft::world::entity::PlayerRideableJumping::PlayerRideableJumping;
use minecraft_rust::net::minecraft::world::level::chunk::PaletteResize::{
    self as palette_resize, PaletteResize,
};
use minecraft_rust::net::minecraft::world::level::lighting::LightEventListener::LightEventListener;
use minecraft_rust::net::minecraft::world::level::ChunkPos::ChunkPos;
use minecraft_rust::net::minecraft::world::level::LevelHeightAccessor::{
    self as level_height, LevelHeightAccessor,
};
use minecraft_rust::net::minecraft::world::phys::shapes::BooleanOp::{
    self as boolean_op, BooleanOp,
};
use minecraft_rust::net::minecraft::CharPredicate::CharPredicate;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

fn bits(v: impl IntoIterator<Item = bool>) -> String {
    v.into_iter().map(|b| if b { '1' } else { '0' }).collect()
}

#[test]
fn tab_order_holder_owner_and_rotate_towards_match_the_jar() {
    struct Tab;
    impl TabOrderedElement for Tab {}
    assert_eq!(Tab.get_tab_order_group(), i32f(rows("TABORDER")[0][1]));

    struct Owner(#[allow(dead_code)] u8);
    impl HolderOwner<()> for Owner {}
    let (a, b) = (Owner(1), Owner(2));
    let r = &rows("HOLDEROWNER")[0];
    assert_eq!(a.can_serialize_in(&a), boolf(r[1]));
    assert_eq!(a.can_serialize_in(&b), boolf(r[2]));
    assert_eq!(b.can_serialize_in(&a), boolf(r[3]));

    struct Ctl;
    impl Control for Ctl {}
    let max_rots = [0.0f32, 1.0, 10.0, 30.0, 180.0, -5.0, f32::NAN];
    let rows = rows("ROTTOWARDS");
    assert_eq!(rows.len(), 17 * 17);
    for r in rows {
        let (from, to) = (f32f(r[1]), f32f(r[2]));
        let got: Vec<String> = max_rots
            .iter()
            .map(|&m| hex32(Ctl.rotate_towards(from, to, m)))
            .collect();
        assert_eq!(
            nan_canon(&got.join(",")),
            nan_canon(r[3]),
            "rotateTowards({}, {}, *)",
            r[1],
            r[2]
        );
    }
}

#[test]
fn palette_resize_and_resource_handle_throw_the_jar_messages() {
    let cases: [(i32, Box<dyn Fn(i32) -> String + std::panic::RefUnwindSafe>); 4] = [
        (
            0,
            Box::new(|b| panic_text(move || palette_resize::no_resize_expected().on_resize(b, 5))),
        ),
        (
            4,
            Box::new(|b| {
                panic_text(move || palette_resize::no_resize_expected().on_resize(b, "stone"))
            }),
        ),
        (
            -1,
            Box::new(|b| {
                panic_text(move || palette_resize::no_resize_expected().on_resize(b, "null"))
            }),
        ),
        (
            MAX,
            Box::new(|b| {
                panic_text(move || palette_resize::no_resize_expected().on_resize(b, -1.5))
            }),
        ),
    ];
    let rows_p = rows("PALRESIZE");
    assert_eq!(rows_p.len(), cases.len());
    for (r, (bits, case)) in rows_p.iter().zip(cases.iter()) {
        assert_eq!(*bits, i32f(r[1]));
        assert_eq!(Some(case(*bits)), unesc(r[3]), "onResize({bits}, {})", r[2]);
    }
    let r = &rows("RESHANDLE")[0];
    let text = panic_text(|| {
        let h = resource_handle::invalid();
        let _: &u32 = ResourceHandle::<u32>::get(&h);
    });
    assert_eq!(Some(text), unesc(r[1]));
    assert_eq!(
        resource_handle::invalid() == resource_handle::INVALID_HANDLE,
        boolf(r[2])
    );
}

#[test]
fn glyph_info_defaults_match_the_jar() {
    for r in rows("GLYPH") {
        let g = glyph_info::simple(f32f(r[1]));
        let got = [
            g.get_advance(),
            g.get_advance_bool(false),
            g.get_advance_bool(true),
            g.get_bold_offset(),
            g.get_shadow_offset(),
        ]
        .map(hex32);
        assert_eq!(
            nan_canon(&got.join(" ")),
            nan_canon(&r[2..7].join(" ")),
            "simple({})",
            r[1]
        );
    }
}

#[test]
fn char_predicate_combinators_match_the_jar() {
    let p = |c: u16| c < b'm' as u16;
    let q = |c: u16| c % 2 == 0;
    let r = &rows("CHARPRED")[0];
    let range = 0..0x300u16;
    assert_eq!(bits(range.clone().map(|c| p.and(q).test(c))), r[1], "and");
    assert_eq!(bits(range.clone().map(|c| p.or(q).test(c))), r[2], "or");
    assert_eq!(
        bits(range.clone().map(|c| p.negate().test(c))),
        r[3],
        "negate"
    );
    assert_eq!(
        bits(range.map(|c| p.negate().or(q.and(p)).negate().test(c))),
        r[4],
        "nested"
    );
    let r = &rows("CHARPREDSURR")[0];
    assert_eq!(q.test(0xd800), boolf(r[1]));
    assert_eq!(p.negate().test(0xdfff), boolf(r[2]));
    assert_eq!(p.and(q).test(0xffff), boolf(r[3]));
}

#[test]
fn boolean_op_constants_match_the_jar() {
    let ours = [
        ("FALSE", boolean_op::FALSE),
        ("NOT_OR", boolean_op::NOT_OR),
        ("ONLY_SECOND", boolean_op::ONLY_SECOND),
        ("NOT_FIRST", boolean_op::NOT_FIRST),
        ("ONLY_FIRST", boolean_op::ONLY_FIRST),
        ("NOT_SECOND", boolean_op::NOT_SECOND),
        ("NOT_SAME", boolean_op::NOT_SAME),
        ("NOT_AND", boolean_op::NOT_AND),
        ("AND", boolean_op::AND),
        ("SAME", boolean_op::SAME),
        ("SECOND", boolean_op::SECOND),
        ("CAUSES", boolean_op::CAUSES),
        ("FIRST", boolean_op::FIRST),
        ("CAUSED_BY", boolean_op::CAUSED_BY),
        ("OR", boolean_op::OR),
        ("TRUE", boolean_op::TRUE),
    ];
    let rows = rows("BOOLOP");
    assert_eq!(
        rows.len(),
        ours.len(),
        "the jar has a different number of BooleanOp constants"
    );
    for (r, (name, op)) in rows.iter().zip(ours.iter()) {
        assert_eq!(r[1], *name, "declaration order");
        let got = [
            op.apply(false, false),
            op.apply(false, true),
            op.apply(true, false),
            op.apply(true, true),
        ];
        assert_eq!(
            got.to_vec(),
            r[2..6].iter().map(|v| boolf(v)).collect::<Vec<bool>>(),
            "{name}"
        );
    }
    for (k, (_, a)) in ours.iter().enumerate() {
        for (_, b) in ours.iter().skip(k + 1) {
            assert_ne!(a, b, "constants are distinct");
        }
    }
    let closure = |a: bool, b: bool| a ^ b;
    assert!(closure.apply(true, false) && !closure.apply(true, true));
}

#[test]
fn light_event_listener_forwards_the_section_of_a_block() {
    struct Recorder(Vec<String>);
    impl LightEventListener for Recorder {
        fn check_block(&mut self, _pos: &BlockPos) {}
        fn has_light_work(&self) -> bool {
            false
        }
        fn run_light_updates(&mut self) -> i32 {
            0
        }
        fn update_section_status_section_pos(&mut self, pos: &SectionPos, section_empty: bool) {
            self.0
                .push(format!("{},{},{}:{}", pos.x, pos.y, pos.z, section_empty));
        }
        fn set_light_enabled(&mut self, _pos: &ChunkPos, _enable: bool) {}
        fn propagate_light_sources(&mut self, _pos: &ChunkPos) {}
    }
    let mut rec = Recorder(Vec::new());
    for [x, y, z] in [[0, 0, 0], [15, 16, -1], [-16, -17, 31], [MAX, MIN, 7]] {
        rec.update_section_status_block_pos(&BlockPos::new(x, y, z), x % 2 == 0);
    }
    assert_eq!(rec.0.join(";"), rows("LIGHTEVT")[0][1]);
}

#[test]
fn player_rideable_jumping_defaults_match_the_jar() {
    struct Mount;
    impl PlayerRideable for Mount {}
    impl PlayerRideableJumping for Mount {
        fn on_player_jump(&mut self, _jump_amount: i32) {}
        fn can_jump(&self) -> bool {
            true
        }
        fn handle_start_jump(&mut self, _jump_scale: i32) {}
        fn handle_stop_jump(&mut self) {}
    }
    let r = &rows("RIDEJUMP")[0];
    assert_eq!(Mount.get_jump_cooldown(), i32f(r[1]));
    let got: Vec<String> = [MIN, -100, -1, 0, 1, 45, 89, 90, 91, 100, MAX]
        .iter()
        .map(|&j| format!("{j}={}", hex32(Mount.get_player_jump_pending_scale(j))))
        .collect();
    assert_eq!(got.join(","), r[2]);
}

#[test]
fn level_height_accessor_defaults_match_the_jar() {
    let ys = [
        -2048, -65, -64, -1, 0, 1, 15, 16, 255, 256, 319, 320, MIN, MAX,
    ];
    let rows = rows("HEIGHTACC");
    assert_eq!(rows.len(), 8);
    for r in rows {
        let la = level_height::create(i32f(r[1]), i32f(r[2]));
        let header = [
            la.get_height(),
            la.get_min_y(),
            la.get_max_y(),
            la.get_sections_count(),
            la.get_min_section_y(),
            la.get_max_section_y(),
        ];
        assert_eq!(
            header.to_vec(),
            r[3..9].iter().map(|v| i32f(v)).collect::<Vec<i32>>(),
            "create({}, {})",
            r[1],
            r[2]
        );
        assert_eq!(
            bits(ys.iter().map(|&y| la.is_inside_build_height_i32(y))),
            r[9]
        );
        assert_eq!(
            bits(ys.iter().map(|&y| la.is_outside_build_height_i32(y))),
            r[10]
        );
        let pos: String = ys
            .iter()
            .map(|&y| {
                let p = BlockPos::new(3, y, -3);
                bits([
                    la.is_inside_build_height_block_pos(&p),
                    la.is_outside_build_height_block_pos(&p),
                ])
            })
            .collect();
        assert_eq!(pos, r[11]);
        let idx: Vec<String> = ys
            .iter()
            .map(|&y| {
                format!(
                    "{}/{}/{}",
                    la.get_section_index(y),
                    la.get_section_index_from_section_y(y),
                    la.get_section_y_from_section_index(y)
                )
            })
            .collect();
        assert_eq!(
            idx.join(","),
            r[12],
            "create({}, {}) index helpers",
            r[1],
            r[2]
        );
    }
}

#[test]
fn selectable_entry_hit_tests_match_the_jar() {
    struct Entry;
    impl SelectableEntry for Entry {}
    let rel = [-1, 0, 1, 7, 8, 15, 16, 17, MIN, MAX];
    for r in rows("SELENTRY") {
        let size = i32f(r[1]);
        let mut cells = Vec::new();
        for &x in &rel {
            for &y in &rel {
                cells.push(bits([
                    Entry.mouse_over_icon(x, y, size),
                    Entry.mouse_over_left_half(x, y, size),
                    Entry.mouse_over_right_half(x, y, size),
                    Entry.mouse_over_top_right_quarter(x, y, size),
                    Entry.mouse_over_bottom_right_quarter(x, y, size),
                    Entry.mouse_over_top_left_quarter(x, y, size),
                    Entry.mouse_over_bottom_left_quarter(x, y, size),
                ]));
            }
        }
        assert_eq!(cells.join(" "), r[2], "size {size}");
    }
}

#[test]
fn command_result_callback_chains_by_identity_like_the_jar() {
    assert_eq!(crc::Empty.to_string(), rows("CRCEMPTY")[0][1]);
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let la = log.clone();
    let lb = log.clone();
    let a: Arc<dyn CommandResultCallback> =
        Arc::new(move |s: bool, r: i32| la.lock().unwrap().push(format!("a:{s}:{r}")));
    let b: Arc<dyn CommandResultCallback> =
        Arc::new(move |s: bool, r: i32| lb.lock().unwrap().push(format!("b:{s}:{r}")));
    let e: Arc<dyn CommandResultCallback> = crc::EMPTY.clone();
    let r = &rows("CRCCHAIN")[0];
    assert_eq!(
        Arc::ptr_eq(&crc::chain(e.clone(), a.clone()), &a),
        boolf(r[1])
    );
    assert_eq!(
        Arc::ptr_eq(&crc::chain(a.clone(), e.clone()), &a),
        boolf(r[2])
    );
    assert_eq!(
        Arc::ptr_eq(&crc::chain(e.clone(), e.clone()), &e),
        boolf(r[3])
    );
    let ab = crc::chain(a.clone(), b.clone());
    ab.on_success(7);
    ab.on_failure();
    a.on_result(true, -3);
    e.on_success(1);
    assert_eq!(log.lock().unwrap().join(";"), rows("CRCCALLS")[0][1]);
}

#[test]
fn lerp_functions_match_the_jar() {
    for r in rows("LERPF") {
        let (al, from, to) = (f32f(r[1]), f32f(r[2]), f32f(r[3]));
        let got = [
            lerp::of_float().apply(al, from, to),
            lerp::of_degrees(30.0).apply(al, from, to),
            lerp::of_degrees(181.0).apply(al, from, to),
            LerpFunction::<f32>::apply(&lerp::of_step(0.5), al, from, to),
            LerpFunction::<f32>::apply(&lerp::of_constant(), al, from, to),
        ]
        .map(hex32);
        assert_eq!(
            nan_canon(&got.join(" ")),
            nan_canon(&r[4..9].join(" ")),
            "alpha {} from {} to {}",
            r[1],
            r[2],
            r[3]
        );
    }
    for r in rows("LERPI") {
        let (al, from, to) = (f32f(r[1]), i32f(r[2]), i32f(r[3]));
        assert_eq!(
            lerp::of_integer().apply(al, from, to),
            i32f(r[4]),
            "ofInteger alpha {} {from}..{to}",
            r[1]
        );
        assert_eq!(
            lerp::of_color().apply(al, from, to),
            i32f(r[5]),
            "ofColor alpha {} {from:#x}..{to:#x}",
            r[1]
        );
    }
    assert_eq!(
        lerp::of_constant() == lerp::CONSTANT,
        boolf(rows("LERPCONST")[0][1])
    );
}

#[test]
fn bounded_float_functions_match_the_jar() {
    let r = &rows("BOUNDEDF")[0];
    let ident = bounded::IDENTITY;
    assert_eq!(
        (hex32(ident.min_value()), hex32(ident.max_value())),
        (r[1].to_string(), r[2].to_string())
    );
    let applied: Vec<String> = [0.0f32, -0.0, 1.5, f32::NAN, f32::NEG_INFINITY]
        .iter()
        .map(|&v| hex32(ident.apply(v)))
        .collect();
    assert_eq!(applied.join(","), r[3]);
    let cst = bounded::constant(2.5);
    assert_eq!(
        (
            hex32(BoundedFloatFunction::<&str>::apply(&cst, "x")),
            hex32(BoundedFloatFunction::<&str>::min_value(&cst)),
            hex32(BoundedFloatFunction::<&str>::max_value(&cst))
        ),
        (r[4].to_string(), r[5].to_string(), r[6].to_string())
    );
    let com = ident.comap(|s: &str| s.parse::<f32>().unwrap());
    assert_eq!(
        (
            hex32(com.apply("-3.25")),
            hex32(com.min_value()),
            hex32(com.max_value())
        ),
        (r[7].to_string(), r[8].to_string(), r[9].to_string())
    );
}

#[test]
fn abortable_iteration_consumer_matches_the_jar() {
    let rows_c = rows("ABORTCONT");
    assert_eq!(rows_c.len(), Continuation::VALUES.len());
    for (r, c) in rows_c.iter().zip(Continuation::VALUES.iter()) {
        assert_eq!(
            (c.name(), c.ordinal(), c.should_abort()),
            (r[1], i32f(r[2]), boolf(r[3]))
        );
    }
    let mut seen = Vec::new();
    let r = &rows("ABORTFOR")[0];
    {
        let mut fc = abortable::for_consumer(|s: &str| seen.push(s.to_string()));
        assert_eq!(fc.accept("x").name(), r[1]);
        assert_eq!(fc.accept("y").name(), r[2]);
    }
    assert_eq!(seen.join(","), r[3]);
}

#[test]
fn reputation_event_types_and_packrat_control_match_the_jar() {
    let ours: [(&str, &Registered); 5] = [
        ("ZOMBIE_VILLAGER_CURED", &reputation::ZOMBIE_VILLAGER_CURED),
        ("GOLEM_KILLED", &reputation::GOLEM_KILLED),
        ("VILLAGER_HURT", &reputation::VILLAGER_HURT),
        ("VILLAGER_KILLED", &reputation::VILLAGER_KILLED),
        ("TRADE", &reputation::TRADE),
    ];
    let rows_r = rows("REPEVENT");
    assert_eq!(rows_r.len(), ours.len());
    for (r, (name, ev)) in rows_r.iter().zip(ours.iter()) {
        assert_eq!((r[1], Some(ev.to_string())), (*name, unesc(r[2])));
    }
    let r = &rows("REPEVENTSAME")[0];
    assert_eq!(
        std::ptr::eq(&reputation::TRADE, &reputation::TRADE),
        boolf(r[1])
    );
    assert_eq!(
        std::ptr::eq(&reputation::TRADE, &reputation::GOLEM_KILLED),
        boolf(r[2])
    );

    let r = &rows("PACKRATCTL")[0];
    let mut unb = packrat_control::UNBOUND;
    let before = unb.has_cut();
    PackratControl::cut(&mut unb);
    assert_eq!((before, unb.has_cut()), (boolf(r[1]), boolf(r[2])));
}

#[test]
fn time_source_constant_converts_like_the_jar() {
    for r in rows("TIMESRC") {
        let ts = time_source::constant(i64f(r[1]));
        assert_eq!(ts.get_as_long(), i64f(r[2]));
        let got: Vec<String> = TimeUnit::VALUES
            .iter()
            .map(|&u| format!("{}={}", u.name(), ts.get(u)))
            .collect();
        assert_eq!(got.join(","), r[3], "constant({})", r[1]);
    }
    // A closure is a NanoTimeSource, and through it a TimeSource, as a Java lambda is.
    let clock = || 2_500_000_000i64;
    assert_eq!(clock.get(TimeUnit::Seconds), 2);
}

#[test]
fn skip_all_answers_match_the_jar() {
    let r = &rows("SKIPALL")[0];
    let mut sa = SkipAll::INSTANCE;
    let tt = TagTypes::get_type(1);
    let got = [
        format!("{:?}", sa.visit_end()),
        format!("{:?}", sa.visit_string("s")),
        format!("{:?}", sa.visit_byte(1)),
        format!("{:?}", sa.visit_short(1)),
        format!("{:?}", sa.visit_int(1)),
        format!("{:?}", sa.visit_long(1)),
        format!("{:?}", sa.visit_float(1.0)),
        format!("{:?}", sa.visit_double(1.0)),
        format!("{:?}", sa.visit_byte_array(&[])),
        format!("{:?}", sa.visit_int_array(&[])),
        format!("{:?}", sa.visit_long_array(&[])),
        format!("{:?}", sa.visit_list(&tt, 3)),
        format!("{:?}", sa.visit_element(&tt, 0)),
        format!("{:?}", sa.visit_entry(&tt)),
        format!("{:?}", sa.visit_entry_named(&tt, "k")),
        format!("{:?}", sa.visit_container_end()),
        format!("{:?}", sa.visit_root_entry(&tt)),
    ];
    let want: Vec<String> = r[1..18].iter().map(|v| v.to_string()).collect();
    let got: Vec<String> = got.iter().map(|g| g.to_uppercase()).collect();
    assert_eq!(got, want);
}
