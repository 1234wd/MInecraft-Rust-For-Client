//! Sub-batch C: records with float/double, Identifier, enum and OptionalInt components.
//!
//! Groups: `REC`/`RECEQ` (via [`crate::b_records::check_records`]), `GIZMO`, `GIZMOMUL`,
//! `GIZMODEF`, `UVPACK`, `UVUNPACK`, `PPOSE`, `PPOSEMAKE`, `DSDEFAULT`, `LEVELRES`, `WIDGET`,
//! `DISPLAY`. Instance lists are the probe's, in the probe's order.

use crate::b_records::check_records;
use crate::small_probe::*;

use minecraft_rust::com::mojang::blaze3d::pipeline::BlendEquation::BlendEquation;
use minecraft_rust::com::mojang::blaze3d::pipeline::DepthStencilState::DepthStencilState;
use minecraft_rust::com::mojang::blaze3d::platform::BlendFactor;
use minecraft_rust::com::mojang::blaze3d::platform::BlendOp;
use minecraft_rust::com::mojang::blaze3d::platform::CompareOp;
use minecraft_rust::com::mojang::blaze3d::platform::DisplayData::DisplayData;
use minecraft_rust::net::minecraft::client::gui::components::WidgetSprites::WidgetSprites;
use minecraft_rust::net::minecraft::client::model::geom::builders::UVPair::UVPair;
use minecraft_rust::net::minecraft::client::model::geom::ModelLayerLocation::ModelLayerLocation;
use minecraft_rust::net::minecraft::client::model::geom::PartPose::PartPose;
use minecraft_rust::net::minecraft::client::particle::ParticleRenderType::ParticleRenderType;
use minecraft_rust::net::minecraft::client::renderer::entity::state::HitboxRenderState::HitboxRenderState;
use minecraft_rust::net::minecraft::gizmos::GizmoStyle::GizmoStyle;
use minecraft_rust::net::minecraft::resources::Identifier::{self as identifier, Identifier};
use minecraft_rust::net::minecraft::util::parsing::packrat::Atom::Atom;
use minecraft_rust::net::minecraft::world::entity::EntitySpawnReason;
use minecraft_rust::net::minecraft::world::entity::EntitySpawnRequest::EntitySpawnRequest;
use minecraft_rust::net::minecraft::world::level::block::entity::DecoratedPotPattern::DecoratedPotPattern;
use minecraft_rust::net::minecraft::world::level::storage::LevelResource::LevelResource;

const MIN: i32 = i32::MIN;

fn id(path: &str) -> Identifier {
    identifier::with_default_namespace(path)
}

fn nan_payload(bits: u32) -> f32 {
    f32::from_bits(bits)
}

#[test]
fn gizmo_style_matches_the_jar() {
    check_records(
        "GizmoStyle",
        &[
            GizmoStyle::stroke_i32(0xFF00FF00u32 as i32),
            GizmoStyle::stroke_i32(0xFF00FF00u32 as i32),
            GizmoStyle::stroke_i32_f32(1, 0.0),
            GizmoStyle::stroke_i32_f32(1, -0.0),
            GizmoStyle::fill_i32(-1),
            GizmoStyle::stroke_and_fill(5, f32::NAN, 6),
            GizmoStyle::stroke_and_fill(5, nan_payload(0x7fc0_0001), 6),
            GizmoStyle::new(0, 2.5, 0),
        ],
        &|x| x.java_hash_code(),
    );
    for r in rows("GIZMO") {
        let g = GizmoStyle::stroke_and_fill(i32f(r[1]), f32f(r[2]), i32f(r[3]));
        assert_eq!(
            (g.stroke(), g.stroke_width().to_bits(), g.fill()),
            (i32f(r[1]), f32bits(r[2]), i32f(r[3]))
        );
        assert_eq!(g.has_fill(), boolf(r[4]), "{g}.hasFill");
        assert_eq!(g.has_stroke(), boolf(r[5]), "{g}.hasStroke");
    }
    for r in rows("GIZMOMUL") {
        let c = i32f(r[1]);
        let g = GizmoStyle::stroke_and_fill(c, 1.0, !c);
        let a = f32f(r[2]);
        assert_eq!(
            g.multiplied_stroke(a),
            i32f(r[3]),
            "multipliedStroke({c:#x}, {})",
            r[2]
        );
        assert_eq!(
            g.multiplied_fill(a),
            i32f(r[4]),
            "multipliedFill({:#x}, {})",
            !c,
            r[2]
        );
    }
    assert_eq!(
        GizmoStyle::stroke_i32(1).stroke_width().to_bits(),
        f32bits(rows("GIZMODEF")[0][1])
    );
}

/// The probe's `UVS`.
const UVS: [f32; 10] = [
    0.0,
    -0.0,
    1.5,
    -2.25,
    f32::NAN,
    f32::from_bits(0x7f80_0001),
    f32::from_bits(1),
    f32::MAX,
    f32::INFINITY,
    f32::NEG_INFINITY,
];

#[test]
fn uv_pair_matches_the_jar() {
    let mut xs: Vec<UVPair> = (0..UVS.len())
        .map(|k| UVPair::new(UVS[k], UVS[(k + 1) % UVS.len()]))
        .collect();
    xs.push(UVPair::new(UVS[0], UVS[1]));
    check_records("UVPair", &xs, &|x| x.java_hash_code());
    // Accessors hand back the exact bits stored, NaN payload included.
    let probe_pair = UVPair::new(f32::from_bits(0x7f80_0001), -0.0);
    assert_eq!(
        (probe_pair.u().to_bits(), probe_pair.v().to_bits()),
        (0x7f80_0001, 0x8000_0000)
    );
    let rows_pack = rows("UVPACK");
    assert_eq!(rows_pack.len(), UVS.len() * UVS.len());
    for r in rows_pack {
        let (u, v) = (f32f(r[1]), f32f(r[2]));
        let p = UVPair::pack(u, v);
        assert_eq!(p, i64f(r[3]), "pack({}, {})", r[1], r[2]);
        assert_eq!(
            hex32(UVPair::unpack_u(p)),
            r[4],
            "unpackU(pack({}, {}))",
            r[1],
            r[2]
        );
        assert_eq!(
            hex32(UVPair::unpack_v(p)),
            r[5],
            "unpackV(pack({}, {}))",
            r[1],
            r[2]
        );
    }
    for r in rows("UVUNPACK") {
        let raw = i64f(r[1]);
        assert_eq!(hex32(UVPair::unpack_u(raw)), r[2], "unpackU({raw})");
        assert_eq!(hex32(UVPair::unpack_v(raw)), r[3], "unpackV({raw})");
    }
}

fn poses() -> [PartPose; 7] {
    [
        PartPose::ZERO,
        PartPose::offset(0.0, 0.0, 0.0),
        PartPose::offset(1.0, 2.0, 3.0),
        PartPose::rotation(0.5, -0.5, f32::NAN),
        PartPose::offset_and_rotation(-0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        PartPose::new(1.1, -2.2, 3.3, 0.1, 0.2, 0.3, 2.0, 0.5, -1.0),
        PartPose::new(
            f32::MAX,
            f32::from_bits(1),
            f32::NEG_INFINITY,
            1e-30,
            1e30,
            7.0,
            f32::NAN,
            0.0,
            -0.0,
        ),
    ]
}

#[test]
fn part_pose_matches_the_jar() {
    let ps = poses();
    check_records("PartPose", &ps, &|x| x.java_hash_code());
    let pp = rows("PPOSE");
    assert_eq!(pp.len(), ps.len() * (1 + 3 * 8));
    for r in pp {
        let p = ps[i32f(r[1]) as usize];
        let (op, arg) = r[2].split_once(':').unwrap_or((r[2], ""));
        let got = match op {
            "translated" => p.translated(0.25, -1.5, f32::MAX),
            "scaled" => {
                let factor = f32f(arg);
                // Java's `s == p` is IDENTITY: true exactly when `factor == 1.0F` took the
                // `return this` branch. The port returns an equal copy there, so the row pins the
                // branch condition, and the copy must be unchanged.
                assert_eq!(
                    factor == 1.0,
                    boolf(r[4]),
                    "pose[{}].scaled({}): which branch",
                    r[1],
                    arg
                );
                let s = p.scaled_f32(factor);
                if factor == 1.0 {
                    assert!(s == p, "pose[{}].scaled(1.0) must be unchanged", r[1]);
                }
                s
            }
            "withScale" => p.with_scale(f32f(arg)),
            "scaled3" => p.scaled_f32_f32_f32(f32f(arg), 0.5, f32f(arg)),
            other => panic!("unknown PPOSE op {other}"),
        };
        assert_eq!(
            Some(got.to_string()),
            unesc(r[3]),
            "pose[{}].{}",
            r[1],
            r[2]
        );
    }
    let make = rows("PPOSEMAKE");
    let find = |k: &str| {
        make.iter()
            .find(|r| r[1] == k)
            .unwrap_or_else(|| panic!("no PPOSEMAKE {k}"))
    };
    assert_eq!(
        Some(PartPose::offset(1.5, -0.0, f32::NAN).to_string()),
        unesc(find("offset")[2])
    );
    assert_eq!(
        Some(PartPose::rotation(1.5, -0.0, f32::NAN).to_string()),
        unesc(find("rotation")[2])
    );
    assert_eq!(
        Some(PartPose::offset_and_rotation(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).to_string()),
        unesc(find("offsetAndRotation")[2])
    );
    let p = ps[5];
    assert_eq!(
        [
            p.x(),
            p.y(),
            p.z(),
            p.x_rot(),
            p.y_rot(),
            p.z_rot(),
            p.x_scale(),
            p.y_scale(),
            p.z_scale()
        ],
        [1.1, -2.2, 3.3, 0.1, 0.2, 0.3, 2.0, 0.5, -1.0]
    );
}

#[test]
fn hitbox_render_state_matches_the_jar() {
    let xs = [
        HitboxRenderState::new_without_offset(0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.5, 0.0),
        HitboxRenderState::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0),
        HitboxRenderState::new(-0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0),
        HitboxRenderState::new(
            0.1,
            -2.5,
            1e300,
            f64::NAN,
            f64::from_bits(1),
            f64::NEG_INFINITY,
            0.25,
            -0.0,
            f32::NAN,
            0.1,
            1e-40,
            f32::MAX,
        ),
        HitboxRenderState::new(
            0.1,
            -2.5,
            1e300,
            f64::from_bits(0x7ff8_0000_0000_0001),
            f64::from_bits(1),
            f64::NEG_INFINITY,
            0.25,
            -0.0,
            nan_payload(0xffc0_0000),
            0.1,
            1e-40,
            f32::MAX,
        ),
    ];
    check_records("HitboxRenderState", &xs, &|x| x.java_hash_code());
    let h = xs[3];
    assert_eq!(
        (h.x0(), h.y0(), h.z0(), h.y1()),
        (0.1, -2.5, 1e300, f64::from_bits(1))
    );
    assert!(h.x1().is_nan() && h.z1() == f64::NEG_INFINITY);
    assert_eq!(
        (
            h.offset_x(),
            h.offset_y().to_bits(),
            h.red(),
            h.green(),
            h.blue()
        ),
        (0.25, 0x8000_0000, 0.1, 1e-40, f32::MAX)
    );
    assert!(h.offset_z().is_nan());
}

#[test]
fn depth_stencil_state_matches_the_jar() {
    let ops = CompareOp::VALUES;
    let xs = [
        DepthStencilState::DEFAULT,
        DepthStencilState::new_without_bias(CompareOp::CompareOp::GreaterThanOrEqual, true),
        DepthStencilState::new(CompareOp::CompareOp::GreaterThanOrEqual, true, 0.0, -0.0),
        DepthStencilState::new(ops[0], false, f32::NAN, 1.5),
        DepthStencilState::new(ops[0], false, nan_payload(0x7fc0_0001), 1.5),
        DepthStencilState::new(ops[7], true, -2.0, f32::MAX),
    ];
    check_records("DepthStencilState", &xs, &|_| {
        unreachable!("enum component: hash not portable")
    });
    let r = &rows("DSDEFAULT")[0];
    let d = DepthStencilState::DEFAULT;
    assert_eq!(d.depth_test().name(), r[1]);
    assert_eq!(d.write_depth(), boolf(r[2]));
    assert_eq!(d.depth_bias_scale_factor().to_bits(), f32bits(r[3]));
    assert_eq!(d.depth_bias_constant().to_bits(), f32bits(r[4]));
}

#[test]
fn enum_component_records_match_the_jar() {
    let reasons = EntitySpawnReason::VALUES;
    let xs = [
        EntitySpawnRequest::new(reasons[0], false),
        EntitySpawnRequest::new(reasons[0], false),
        EntitySpawnRequest::new(reasons[0], true),
        EntitySpawnRequest::new(reasons[2], false),
        EntitySpawnRequest::new(reasons[reasons.len() - 1], true),
    ];
    check_records("EntitySpawnRequest", &xs, &|_| {
        unreachable!("enum component: hash not portable")
    });
    assert_eq!((xs[3].reason(), xs[3].ignore_checks()), (reasons[2], false));

    let bf = BlendFactor::VALUES;
    let bo = BlendOp::VALUES;
    let xs = [
        BlendEquation::new(bf[9], bf[4], bo[0]),
        BlendEquation::new(bf[9], bf[4], bo[0]),
        BlendEquation::new(bf[4], bf[9], bo[0]),
        BlendEquation::new(bf[9], bf[4], bo[4]),
        BlendEquation::new(bf[0], bf[14], bo[2]),
    ];
    check_records("BlendEquation", &xs, &|_| {
        unreachable!("enum components: hash not portable")
    });
    assert_eq!(
        (xs[4].source_factor(), xs[4].dest_factor(), xs[4].op()),
        (bf[0], bf[14], bo[2])
    );
}

#[test]
fn identifier_records_match_the_jar() {
    let pots = [
        DecoratedPotPattern::new(id("blank")),
        DecoratedPotPattern::new(identifier::parse("minecraft:blank")),
        DecoratedPotPattern::new(identifier::from_namespace_and_path("mod", "a/b")),
        DecoratedPotPattern::new(id("angler")),
    ];
    check_records("DecoratedPotPattern", &pots, &|x| x.java_hash_code());
    assert_eq!(pots[2].asset_id().to_string(), "mod:a/b");

    let layers = [
        ModelLayerLocation::new(id("player"), "main"),
        ModelLayerLocation::new(id("player"), "main"),
        ModelLayerLocation::new(id("player"), "outer"),
        ModelLayerLocation::new(id("player"), "MAIN"),
        ModelLayerLocation::new(identifier::from_namespace_and_path("mod", "x"), ""),
    ];
    check_records("ModelLayerLocation", &layers, &|x| x.java_hash_code());
    assert_eq!(
        (layers[2].model().to_string(), layers[2].layer()),
        ("minecraft:player".to_string(), "outer")
    );

    let (a, b, c, d) = (id("a"), id("b"), id("c"), id("d"));
    let ws = [
        WidgetSprites::from_sprite(&a),
        WidgetSprites::from_sprite_and_focused(&a, &b),
        WidgetSprites::from_enabled_disabled_focused(&a, &b, &c),
        WidgetSprites::new(a.clone(), b.clone(), c.clone(), d.clone()),
        WidgetSprites::new(a.clone(), a.clone(), a.clone(), a.clone()),
    ];
    check_records("WidgetSprites", &ws, &|x| x.java_hash_code());
    let rows_w = rows("WIDGET");
    assert_eq!(rows_w.len(), ws.len() * 4);
    for r in rows_w {
        let w = &ws[i32f(r[1]) as usize];
        assert_eq!(
            w.get(boolf(r[2]), boolf(r[3])).to_string(),
            r[4],
            "ws[{}].get({}, {})",
            r[1],
            r[2],
            r[3]
        );
    }
    let w = &ws[3];
    assert_eq!(
        [
            w.enabled(),
            w.disabled(),
            w.enabled_focused(),
            w.disabled_focused()
        ]
        .map(|i| i.to_string()),
        ["minecraft:a", "minecraft:b", "minecraft:c", "minecraft:d"]
    );
}

#[test]
fn string_records_match_the_jar() {
    let atoms: [Atom<i32>; 5] = [
        Atom::of("a"),
        Atom::new("a"),
        Atom::of(""),
        Atom::of("\u{e9} \u{4e2d}"),
        Atom::of("b"),
    ];
    check_records("Atom", &atoms, &|x| x.java_hash_code());
    assert_eq!(atoms[3].name(), "\u{e9} \u{4e2d}");

    let lrs = [
        LevelResource::PLAYER_ADVANCEMENTS_DIR,
        LevelResource::PLAYER_STATS_DIR,
        LevelResource::PLAYER_DATA_DIR,
        LevelResource::PLAYER_OLD_DATA_DIR,
        LevelResource::LEVEL_DATA_FILE,
        LevelResource::OLD_LEVEL_DATA_FILE,
        LevelResource::ICON_FILE,
        LevelResource::LOCK_FILE,
        LevelResource::GENERATED_DIR,
        LevelResource::DATAPACK_DIR,
        LevelResource::MAP_RESOURCE_FILE,
        LevelResource::DATA,
        LevelResource::ROOT,
        LevelResource::new("level.dat"),
        LevelResource::new(""),
    ];
    check_records("LevelResource", &lrs, &|x| x.java_hash_code());
    let ids = rows("LEVELRES");
    assert_eq!(ids.len(), lrs.len());
    for (r, lr) in ids.iter().zip(lrs.iter()) {
        assert_eq!(Some(lr.id().to_string()), unesc(r[1]));
    }

    let prts = [
        ParticleRenderType::SINGLE_QUADS,
        ParticleRenderType::ITEM_PICKUP,
        ParticleRenderType::ELDER_GUARDIANS,
        ParticleRenderType::NO_RENDER,
        ParticleRenderType::new("SINGLE_QUADS", "SQ"),
    ];
    check_records("ParticleRenderType", &prts, &|x| x.java_hash_code());
    assert_eq!(
        (prts[2].name(), prts[2].shorthand()),
        ("ELDER_GUARDIANS", "EG")
    );
}

#[test]
fn display_data_matches_the_jar() {
    let dd = [
        DisplayData::new(800, 600, None, None, false),
        DisplayData::new(800, 600, None, None, false),
        DisplayData::new(1, 2, Some(3), Some(4), true),
        DisplayData::new(1, 2, Some(0), None, false),
        DisplayData::new(MIN, i32::MAX, Some(MIN), Some(-1), true),
    ];
    check_records("DisplayData", &dd, &|x| x.java_hash_code());
    let shown = rows("DISPLAY");
    assert_eq!(shown.len(), dd.len());
    for (r, d) in shown.iter().zip(dd.iter()) {
        assert_eq!(
            Some(d.with_size(-5, 7).to_string()),
            unesc(r[2]),
            "{d}.withSize(-5, 7)"
        );
        assert_eq!(
            Some(d.with_fullscreen(!d.is_fullscreen()).to_string()),
            unesc(r[3]),
            "{d}.withFullscreen(!)"
        );
    }
    let d = dd[4];
    assert_eq!(
        (
            d.width(),
            d.height(),
            d.fullscreen_width(),
            d.fullscreen_height()
        ),
        (MIN, i32::MAX, Some(MIN), Some(-1))
    );
}
