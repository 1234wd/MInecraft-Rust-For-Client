//! Sub-batch E (files the earlier sub-batches unblocked): `GpuTextureView`, `GlyphBitmap`,
//! `ColorTargetState`, `DeviceInfo`, `RecordedDeviation`, and the `javacompat::java_time::Instant`
//! value it holds.
//!
//! Groups: `GPUTEXVIEW`, `GLYPHBMP`, `COLORTGT`, `DEVINFO`, `INSTANT`, `RECDEV`.

use std::cell::Cell;
use std::sync::Arc;

use crate::sweep_probe::*;

use minecraft_rust::com::mojang::blaze3d::font::GlyphBitmap::GlyphBitmap;
use minecraft_rust::com::mojang::blaze3d::pipeline::BlendFunction::{self as blend, BlendFunction};
use minecraft_rust::com::mojang::blaze3d::pipeline::ColorTargetState::{
    self as cts, ColorTargetState,
};
use minecraft_rust::com::mojang::blaze3d::systems::DeviceFeatures::DeviceFeatures;
use minecraft_rust::com::mojang::blaze3d::systems::DeviceInfo::DeviceInfo;
use minecraft_rust::com::mojang::blaze3d::systems::DeviceLimits::DeviceLimits;
use minecraft_rust::com::mojang::blaze3d::systems::DeviceType;
use minecraft_rust::com::mojang::blaze3d::systems::HintsAndWorkarounds::HintsAndWorkarounds;
use minecraft_rust::com::mojang::blaze3d::textures::GpuTexture::{GpuTexture, GpuTextureState};
use minecraft_rust::com::mojang::blaze3d::textures::GpuTextureView::{
    GpuTextureView, GpuTextureViewState,
};
use minecraft_rust::com::mojang::blaze3d::GpuFormat::GpuFormat;
use minecraft_rust::javacompat::java_time::Instant;
use minecraft_rust::net::minecraft::util::profiling::metrics::storage::RecordedDeviation::RecordedDeviation;
use minecraft_rust::net::minecraft::util::profiling::EmptyProfileResults;
use minecraft_rust::net::minecraft::util::profiling::ProfileResults::ProfileResults;

fn java_float(s: &str) -> f32 {
    match s {
        "NaN" => f32::NAN,
        "Infinity" => f32::INFINITY,
        "-Infinity" => f32::NEG_INFINITY,
        _ => s.parse().unwrap_or_else(|_| panic!("not a float: {s}")),
    }
}

// ---------------------------------------------------------------- GPUTEXVIEW

struct Tex(GpuTextureState);

impl GpuTexture for Tex {
    fn state(&self) -> &GpuTextureState {
        &self.0
    }
    fn close(&self) {}
    fn is_closed(&self) -> bool {
        false
    }
}

struct View {
    state: GpuTextureViewState,
    closed: Cell<bool>,
}

impl GpuTextureView for View {
    fn state(&self) -> &GpuTextureViewState {
        &self.state
    }
    fn close(&self) {
        self.closed.set(true);
    }
    fn is_closed(&self) -> bool {
        self.closed.get()
    }
}

#[test]
fn gpu_texture_views_read_through_to_the_texture_as_in_the_jar() {
    const MIPS: [i32; 12] = [-33, -2, -1, 0, 1, 2, 5, 30, 31, 32, i32::MAX, i32::MIN];
    let rows = rows("GPUTEXVIEW");
    assert_eq!(rows.len(), 7);
    for r in rows {
        let tex: Arc<dyn GpuTexture> = Arc::new(Tex(GpuTextureState::new(
            15,
            "t",
            GpuFormat::Rgba8Unorm,
            i32f(r[1]),
            i32f(r[2]),
            1,
            8,
        )));
        let v = View {
            state: GpuTextureViewState::new(Arc::clone(&tex), i32f(r[3]), i32f(r[4])),
            closed: Cell::new(false),
        };
        assert_eq!(v.base_mip_level(), i32f(r[5]));
        assert_eq!(v.mip_levels(), i32f(r[6]));
        assert_eq!(Arc::ptr_eq(v.texture(), &tex), boolf(r[7]));
        let w: Vec<String> = MIPS.iter().map(|&m| v.get_width(m).to_string()).collect();
        let h: Vec<String> = MIPS.iter().map(|&m| v.get_height(m).to_string()).collect();
        assert_eq!(w.join(","), r[8], "getWidth {r:?}");
        assert_eq!(h.join(","), r[9], "getHeight {r:?}");
        let before = v.is_closed();
        v.close();
        assert_eq!(before, boolf(r[10]));
        assert_eq!(v.is_closed(), boolf(r[11]));
    }
}

// ---------------------------------------------------------------- GLYPHBMP

/// The probe's `glyph(spec)`.
struct Glyph {
    w: i32,
    h: i32,
    over: f32,
    bearings: Option<(f32, f32)>,
    left: Option<f32>,
}

impl GlyphBitmap for Glyph {
    fn get_pixel_width(&self) -> i32 {
        self.w
    }
    fn get_pixel_height(&self) -> i32 {
        self.h
    }
    fn upload(&self, _x: i32, _y: i32, _texture: &dyn GpuTexture) {}
    fn is_colored(&self) -> bool {
        self.w % 2 == 0
    }
    fn get_oversample(&self) -> f32 {
        self.over
    }
    fn get_bearing_left(&self) -> f32 {
        match self.bearings {
            Some((l, _)) => l,
            None => 0.0,
        }
    }
    fn get_bearing_top(&self) -> f32 {
        match self.bearings {
            Some((_, t)) => t,
            None => 7.0,
        }
    }
    fn get_left(&self) -> f32 {
        self.left.unwrap_or_else(|| self.get_bearing_left())
    }
}

/// A glyph that overrides nothing it need not, so the trait's own defaults run.
struct PlainGlyph(i32, i32, f32);

impl GlyphBitmap for PlainGlyph {
    fn get_pixel_width(&self) -> i32 {
        self.0
    }
    fn get_pixel_height(&self) -> i32 {
        self.1
    }
    fn upload(&self, _x: i32, _y: i32, _texture: &dyn GpuTexture) {}
    fn is_colored(&self) -> bool {
        self.0 % 2 == 0
    }
    fn get_oversample(&self) -> f32 {
        self.2
    }
}

fn glyph_fields(g: &dyn GlyphBitmap) -> String {
    [
        fbits(g.get_left()),
        fbits(g.get_right()),
        fbits(g.get_top()),
        fbits(g.get_bottom()),
        fbits(g.get_bearing_left()),
        fbits(g.get_bearing_top()),
        g.is_colored().to_string(),
        g.get_pixel_width().to_string(),
        g.get_pixel_height().to_string(),
        fbits(g.get_oversample()),
    ]
    .join("\t")
}

#[test]
fn glyph_bitmap_defaults_match_the_jar() {
    let rows = rows("GLYPHBMP");
    assert_eq!(rows.len(), 16);
    for r in rows {
        let (main, left) = match r[1].split_once(';') {
            Some((m, l)) => (m, Some(java_float(&l[5..]))),
            None => (r[1], None),
        };
        let p: Vec<&str> = main.split(',').collect();
        let (w, h, over) = (
            p[0].parse().unwrap(),
            p[1].parse().unwrap(),
            java_float(p[2]),
        );
        let bearings = (p.len() > 3).then(|| (java_float(p[3]), java_float(p[4])));
        let want = r[2..12].join("\t");
        let got = glyph_fields(&Glyph {
            w,
            h,
            over,
            bearings,
            left,
        });
        // Computed NaNs compare after `nan_canon` (OPEN_QUESTIONS #33); everything else exactly.
        assert_eq!(nan_canon(&got), nan_canon(&want), "glyph {}", r[1]);
        if bearings.is_none() && left.is_none() {
            assert_eq!(
                nan_canon(&glyph_fields(&PlainGlyph(w, h, over))),
                nan_canon(&want),
                "trait defaults, glyph {}",
                r[1]
            );
        }
    }
}

// ---------------------------------------------------------------- COLORTGT

fn color_target(label: &str) -> ColorTargetState {
    match label {
        "DEFAULT" => cts::DEFAULT,
        "blend:TRANSLUCENT" => ColorTargetState::of_blend_function(blend::TRANSLUCENT),
        "blend:ADDITIVE" => ColorTargetState::of_blend_function(blend::ADDITIVE),
        "canon:empty:RGBA8_UNORM:15" => ColorTargetState::new(None, GpuFormat::Rgba8Unorm, 15),
        "canon:TRANSLUCENT:RGBA8_UNORM:15" => {
            ColorTargetState::new(Some(blend::TRANSLUCENT), GpuFormat::Rgba8Unorm, 15)
        }
        other => {
            let mask = other
                .strip_prefix("canon:empty:R32_FLOAT:")
                .unwrap_or_else(|| panic!("unknown COLORTGT label {other}"));
            ColorTargetState::new(None, GpuFormat::R32Float, mask.parse().unwrap())
        }
    }
}

#[test]
fn color_target_states_match_the_jar() {
    let rows = rows("COLORTGT");
    assert_eq!(rows.len(), 18);
    let all: Vec<ColorTargetState> = rows[1..].iter().map(|r| color_target(r[1])).collect();
    for r in rows {
        if r[1] == "consts" {
            let got = [
                cts::WRITE_RED,
                cts::WRITE_GREEN,
                cts::WRITE_BLUE,
                cts::WRITE_ALPHA,
                cts::WRITE_COLOR,
                cts::WRITE_ALL,
                cts::WRITE_NONE,
                cts::MAX_COLOR_TARGETS,
            ];
            assert_eq!(got.map(|v| v.to_string()).to_vec(), r[2..10].to_vec());
            continue;
        }
        let c = color_target(r[1]);
        assert_eq!(esc(&c.to_string()), r[2], "toString {}", r[1]);
        assert_eq!(
            c.blend_function()
                .map_or("none".to_string(), |b: BlendFunction| b.to_string()),
            r[3]
        );
        assert_eq!(c.format().name(), r[4]);
        assert_eq!(c.write_mask(), i32f(r[5]));
        assert_eq!(
            [
                c.write_red(),
                c.write_green(),
                c.write_blue(),
                c.write_alpha()
            ]
            .map(|b| b.to_string())
            .to_vec(),
            r[6..10].to_vec(),
            "write bits {}",
            r[1]
        );
        let eq: String = all
            .iter()
            .map(|o| if c == *o { '1' } else { '0' })
            .collect();
        assert_eq!(eq, r[10], "equals {}", r[1]);
    }
}

// ---------------------------------------------------------------- DEVINFO

fn dev_info(name: &str, period: f32, ext: &[&str], type_ord: usize, zz: bool) -> DeviceInfo {
    DeviceInfo::new(
        name,
        "Vendor \u{e9}",
        "driver 1.2",
        zz,
        "OpenGL",
        period,
        DeviceLimits::new(16, 256, 16384, i64::MAX, 0, 8),
        DeviceFeatures::new(true, false, true, false, true, false, true),
        ext.iter().map(|s| s.to_string()),
        HintsAndWorkarounds::new(false, true),
        DeviceType::VALUES[type_ord],
    )
}

#[test]
fn device_infos_match_the_jar() {
    let infos = [
        dev_info("GPU", 1.0, &["GL_ARB_a", "GL_ARB_b"], 2, true),
        dev_info("GPU", 1.0, &["GL_ARB_b", "GL_ARB_a"], 2, true),
        dev_info("GPU", 1.0, &["GL_ARB_a"], 2, true),
        dev_info("GPU", -0.0, &[], 0, false),
        dev_info("GPU", 0.0, &[], 0, false),
        dev_info("GPU", f32::NAN, &["x"], 4, true),
        dev_info("GPU", f32::NAN, &["x"], 4, true),
        dev_info("Other \u{1F600}", 52.083332, &["e1", "e2", "e3"], 1, true),
        dev_info("GPU", 1.0e-7, &["GL_ARB_a", "GL_ARB_b"], 3, true),
    ];
    let rows = rows("DEVINFO");
    assert_eq!(rows.len(), infos.len());
    for (r, d) in rows.iter().zip(&infos) {
        assert_eq!(esc(&d.to_string()), r[2], "toString {}", r[1]);
        assert_eq!(fbits(d.timestamp_period()), r[3]);
        assert_eq!(d.type_().name(), r[4]);
        assert_eq!(d.is_z_zero_to_one(), boolf(r[5]));
        assert_eq!(esc(&d.underlying_extensions().join("|")), r[6]);
        let eq: String = infos
            .iter()
            .map(|o| if d == o { '1' } else { '0' })
            .collect();
        assert_eq!(eq, r[7], "equals {}", r[1]);
    }
}

// ---------------------------------------------------------------- INSTANT / RECDEV

#[test]
fn instant_values_match_the_jdk() {
    let mut ok: Vec<(String, Instant)> = Vec::new();
    for r in rows("INSTANT") {
        match r[1] {
            "of" => {
                let (s, n) = (i64f(r[2]), i64f(r[3]));
                let got = or_ex(
                    || Instant::of_epoch_second_adjusted(s, n),
                    |t| {
                        ok.push((format!("{s}:{n}"), t));
                        format!(
                            "{}|{}|{}|{}",
                            t.get_epoch_second(),
                            t.get_nano(),
                            t.java_hash_code(),
                            t == Instant::of_epoch_second(t.get_epoch_second())
                        )
                    },
                );
                assert_eq!(got, r[4], "Instant.ofEpochSecond({s}, {n})");
            }
            "of1" => {
                let s = i64f(r[2]);
                let got = or_ex(
                    || Instant::of_epoch_second(s),
                    |t| format!("{}|{}", t.get_epoch_second(), t.get_nano()),
                );
                assert_eq!(got, r[3], "Instant.ofEpochSecond({s})");
            }
            "cmp" => {
                let (_, a) = ok
                    .iter()
                    .find(|(l, _)| l == r[2])
                    .unwrap_or_else(|| panic!("no instant {}", r[2]));
                let c: Vec<String> = ok
                    .iter()
                    .map(|(_, b)| a.compare_to(b).to_string())
                    .collect();
                assert_eq!(c.join(","), r[3], "compareTo row {}", r[2]);
            }
            other => panic!("unknown INSTANT row {other}"),
        }
    }
    assert_eq!(ok.len(), 16);
}

#[test]
fn recorded_deviation_keeps_what_it_is_given() {
    let res: Arc<dyn ProfileResults> = Arc::new(EmptyProfileResults::EMPTY);
    let rows = rows("RECDEV");
    assert_eq!(rows.len(), 12);
    for r in rows {
        let t = Instant::of_epoch_second_adjusted(i64f(r[1]), i64f(r[2]));
        let d = RecordedDeviation::new(t, i32f(r[3]), Arc::clone(&res));
        assert_eq!(d.timestamp.get_epoch_second(), i64f(r[4]));
        assert_eq!(d.timestamp.get_nano(), i32f(r[5]));
        assert_eq!(d.timestamp == t, boolf(r[6]));
        assert_eq!(d.tick, i32f(r[7]));
        assert_eq!(Arc::ptr_eq(&d.profiler_result_at_tick, &res), boolf(r[8]));
    }
}
