//! Sub-batch C: `RotationSegment`, `DeviceLimits`, `VertexFormatElement`, `BlendFunction`,
//! `GpuTexture`, `BindGroupLayout`, `ProfileResults`, `EmptyProfileResults`,
//! `GraphicsResourceAllocator`, `MultipliedFloats`.
//!
//! Groups: `ROTSEG`, `DEVLIM`, `VFE`, `VFEEQ`, `BLENDFN`, `GPUTEX`, `BINDGL`, `UNIDESC`, `PROFRES`,
//! `GRALLOC`, `MULTFLOAT`.

use std::cell::RefCell;

use crate::sweep_probe::*;

use minecraft_rust::com::mojang::blaze3d::pipeline::BindGroupLayout::{
    self as bgl, BindGroupLayout, UniformDescription,
};
use minecraft_rust::com::mojang::blaze3d::pipeline::BlendEquation::BlendEquation;
use minecraft_rust::com::mojang::blaze3d::pipeline::BlendFunction::{self as blend, BlendFunction};
use minecraft_rust::com::mojang::blaze3d::platform::BlendFactor::BlendFactor;
use minecraft_rust::com::mojang::blaze3d::platform::BlendOp::BlendOp;
use minecraft_rust::com::mojang::blaze3d::resource::GraphicsResourceAllocator::{
    self as gra, GraphicsResourceAllocator,
};
use minecraft_rust::com::mojang::blaze3d::resource::ResourceDescriptor::ResourceDescriptor;
use minecraft_rust::com::mojang::blaze3d::shaders::UniformType::UniformType;
use minecraft_rust::com::mojang::blaze3d::systems::DeviceLimits::DeviceLimits;
use minecraft_rust::com::mojang::blaze3d::textures::GpuTexture::{
    self as gpu_texture, GpuTexture, GpuTextureState,
};
use minecraft_rust::com::mojang::blaze3d::vertex::VertexFormatElement::VertexFormatElement;
use minecraft_rust::com::mojang::blaze3d::GpuFormat::GpuFormat;
use minecraft_rust::net::minecraft::core::Direction;
use minecraft_rust::net::minecraft::util::profiling::EmptyProfileResults;
use minecraft_rust::net::minecraft::util::profiling::ProfileResults::{
    self as profile_results, ProfileResults,
};
use minecraft_rust::net::minecraft::util::profiling::ResultField::ResultField;
use minecraft_rust::net::minecraft::util::valueproviders::MultipliedFloats::{
    DisplaySampledFloat, MultipliedFloats,
};
use minecraft_rust::net::minecraft::util::valueproviders::SampledFloat::SampledFloat;
use minecraft_rust::net::minecraft::util::RandomSource::RandomSource;
use minecraft_rust::net::minecraft::world::level::block::state::properties::RotationSegment as rotation_segment;
use minecraft_rust::net::minecraft::world::level::levelgen::LegacyRandomSource::LegacyRandomSource;

fn gpu_format(name: &str) -> GpuFormat {
    *GpuFormat::VALUES
        .iter()
        .find(|g| g.name() == name)
        .unwrap_or_else(|| panic!("no GpuFormat {name}"))
}

fn uniform_type(name: &str) -> UniformType {
    match name {
        "UNIFORM_BUFFER" => UniformType::UniformBuffer,
        "TEXEL_BUFFER" => UniformType::TexelBuffer,
        other => panic!("no UniformType {other}"),
    }
}

// ---------------------------------------------------------------- ROTSEG

#[test]
fn rotation_segment_matches_the_jar() {
    let rows = rows("ROTSEG");
    assert_eq!(rows.len(), 1 + 6 + 26 + 53);
    for r in rows {
        match r[1] {
            "max" => assert_eq!(rotation_segment::get_max_segment_index(), i32f(r[2])),
            "dir" => {
                // `Direction::name()` is Java's lower-case `getName()`; the probe wrote `Enum.name()`.
                let d = *Direction::VALUES
                    .iter()
                    .find(|d| d.name().to_ascii_uppercase() == r[2])
                    .unwrap();
                assert_eq!(
                    rotation_segment::convert_to_segment_direction(d),
                    i32f(r[3]),
                    "{r:?}"
                );
            }
            "deg" => assert_eq!(
                rotation_segment::convert_to_segment_f32(f32f(r[2])),
                i32f(r[3]),
                "{r:?}"
            ),
            "seg" => {
                let seg = i32f(r[2]);
                let dir = rotation_segment::convert_to_direction(seg)
                    .map_or("empty".to_string(), |d| d.name().to_ascii_uppercase());
                assert_eq!(dir, r[3], "convertToDirection({seg})");
                assert_eq!(
                    fbits(rotation_segment::convert_to_degrees(seg)),
                    r[4],
                    "convertToDegrees({seg})"
                );
            }
            other => panic!("unknown ROTSEG row {other}"),
        }
    }
}

// ---------------------------------------------------------------- DEVLIM

/// The probe's `LIMITS`.
fn limits(k: usize) -> DeviceLimits {
    const L: [[i64; 6]; 12] = [
        [16, 256, 16384, 4294967296, 1, 8],
        [1, 1, 1, 0, 0, 0],
        [
            0,
            0,
            i32::MAX as i64,
            i64::MAX,
            i32::MAX as i64,
            i32::MAX as i64,
        ],
        [-1, -256, -16384, 1 << 40, i32::MIN as i64, -1],
        [8, 64, 32768, -1, 4, 4],
        [8, 64, i32::MIN as i64, 1 << 62, 4, 4],
        [16, 256, 16384, 268435456, 1, 8],
        [16, 256, 16384, 4294967296, 1, 8],
        [16, 256, 16384, 4294967296, 1, 7],
        [16, 256, 16384, 4191028, 1, 8],
        [16, 256, 16384, 1048575 * 16 + 15, 1, 8],
        [16, 256, 4096, 67108863, 1, 8],
    ];
    let v = L[k];
    DeviceLimits::new(
        v[0] as i32,
        v[1] as i32,
        v[2] as i32,
        v[3],
        v[4] as i32,
        v[5] as i32,
    )
}

#[test]
fn device_limits_match_the_jar() {
    let rows = rows("DEVLIM");
    assert_eq!(rows.len(), 12);
    for r in rows {
        let k = i32f(r[1]) as usize;
        let dl = limits(k);
        assert_eq!(dl.max_anisotropy(), i32f(r[2]));
        assert_eq!(dl.min_uniform_offset_alignment(), i32f(r[3]));
        assert_eq!(dl.max_texture_size(), i32f(r[4]));
        assert_eq!(dl.max_memory_allocation_size(), i64f(r[5]));
        assert_eq!(
            dl.max_multi_draw_direct_interleaved_draw_count(),
            i32f(r[6])
        );
        assert_eq!(dl.max_color_attachments(), i32f(r[7]));
        assert_eq!(dl.java_hash_code(), i32f(r[8]), "hashCode {k}");
        assert_eq!(esc(&dl.to_string()), r[9], "toString {k}");
        let sizes: Vec<String> = GpuFormat::VALUES
            .iter()
            .map(|&g| dl.max_texture_size_for_format(g).to_string())
            .collect();
        assert_eq!(sizes.join(","), r[10], "maxTextureSizeForFormat {k}");
        let eq: String = (0..12)
            .map(|o| if dl == limits(o) { '1' } else { '0' })
            .collect();
        assert_eq!(eq, r[11], "equals {k}");
    }
}

// ---------------------------------------------------------------- VFE / VFEEQ

/// The probe's elements, in its order, and their keys.
fn elements() -> Vec<(String, VertexFormatElement)> {
    let names = ["Position", "", "UV0", "\u{e9}\u{1F600}"];
    let offsets = [0, 12, -1, i32::MAX, i32::MIN];
    let formats = [
        GpuFormat::VALUES[0],
        gpu_format("RGB32_FLOAT"),
        gpu_format("RGBA8_UNORM"),
    ];
    let fmts = [formats[1], formats[2], formats[0]];
    let mut out = Vec::new();
    for n in names {
        for o in offsets {
            for g in fmts {
                out.push((
                    format!("{}|{o}|{}", esc(n), g.name()),
                    VertexFormatElement::new(n, o, g),
                ));
            }
        }
    }
    out
}

#[test]
fn vertex_format_element_matches_the_jar() {
    let all = elements();
    let vfe = rows("VFE");
    assert_eq!(vfe.len(), all.len());
    for (r, (key, e)) in vfe.iter().zip(&all) {
        assert_eq!(r[1], key);
        assert_eq!(esc(&e.to_string()), r[2], "toString {key}");
        assert_eq!(esc(e.name()), r[3]);
        assert_eq!(e.offset(), i32f(r[4]));
        assert_eq!(e.format().name(), r[5]);
    }
    for (k, r) in rows("VFEEQ").iter().enumerate() {
        let (key, e) = &all[k];
        assert_eq!(r[1], key);
        let copy = VertexFormatElement::new(e.name().to_string(), e.offset(), e.format());
        assert_eq!(*e == copy, boolf(r[2]), "equals copy {key}");
        let next = (k + 1) % all.len();
        assert_eq!(r[3], all[next].0);
        assert_eq!(*e == all[next].1, boolf(r[4]), "equals next {key}");
        let far = (k + 7) % all.len();
        assert_eq!(r[5], all[far].0);
        assert_eq!(*e == all[far].1, boolf(r[6]), "equals far {key}");
    }
}

// ---------------------------------------------------------------- BLENDFN

fn blend_factor(n: &str) -> BlendFactor {
    *minecraft_rust::com::mojang::blaze3d::platform::BlendFactor::VALUES
        .iter()
        .find(|f| f.name() == n)
        .unwrap()
}

fn blend_op(n: &str) -> BlendOp {
    *minecraft_rust::com::mojang::blaze3d::platform::BlendOp::VALUES
        .iter()
        .find(|o| o.name() == n)
        .unwrap()
}

/// The probe's functions, in its order.
fn blend_functions() -> Vec<(String, BlendFunction)> {
    let mut out: Vec<(String, BlendFunction)> = vec![
        ("LIGHTNING".into(), blend::LIGHTNING),
        ("GLINT".into(), blend::GLINT),
        ("OVERLAY".into(), blend::OVERLAY),
        ("TRANSLUCENT".into(), blend::TRANSLUCENT),
        (
            "TRANSLUCENT_PREMULTIPLIED_ALPHA".into(),
            blend::TRANSLUCENT_PREMULTIPLIED_ALPHA,
        ),
        ("ADDITIVE".into(), blend::ADDITIVE),
        ("ENTITY_OUTLINE_BLIT".into(), blend::ENTITY_OUTLINE_BLIT),
        ("INVERT".into(), blend::INVERT),
    ];
    let f = blend_factor;
    let o = blend_op;
    out.push((
        "pair:SRC_ALPHA,ONE".into(),
        BlendFunction::of_factor_pair(f("SRC_ALPHA"), f("ONE")),
    ));
    out.push((
        "op:SRC_ALPHA,ONE,MAX".into(),
        BlendFunction::of_factor_op(f("SRC_ALPHA"), f("ONE"), o("MAX")),
    ));
    out.push((
        "eq:DST_COLOR,ZERO,REVERSE_SUBTRACT".into(),
        BlendFunction::of_equation(BlendEquation::new(
            f("DST_COLOR"),
            f("ZERO"),
            o("REVERSE_SUBTRACT"),
        )),
    ));
    out.push((
        "four:SRC_ALPHA,ONE,ONE,ZERO".into(),
        BlendFunction::of_factors(f("SRC_ALPHA"), f("ONE"), f("ONE"), f("ZERO")),
    ));
    out.push((
        "six:SRC_ALPHA,ONE,ADD,ONE,ZERO,MAX".into(),
        BlendFunction::of_factors_ops(
            f("SRC_ALPHA"),
            f("ONE"),
            o("ADD"),
            f("ONE"),
            f("ZERO"),
            o("MAX"),
        ),
    ));
    out.push((
        "canon:SRC_ALPHA,ONE,ADD/ONE,SRC_ALPHA,ADD".into(),
        BlendFunction::new(
            BlendEquation::new(f("SRC_ALPHA"), f("ONE"), o("ADD")),
            BlendEquation::new(f("ONE"), f("SRC_ALPHA"), o("ADD")),
        ),
    ));
    out
}

#[test]
fn blend_functions_match_the_jar() {
    let fns = blend_functions();
    let rows = rows("BLENDFN");
    assert_eq!(rows.len(), fns.len());
    for (r, (label, bf)) in rows.iter().zip(&fns) {
        assert_eq!(r[1], label);
        assert_eq!(esc(&bf.to_string()), r[2], "toString {label}");
        assert_eq!(esc(&bf.color().to_string()), r[3], "color {label}");
        assert_eq!(esc(&bf.alpha().to_string()), r[4], "alpha {label}");
        let eq: String = fns
            .iter()
            .map(|(_, o)| if bf == o { '1' } else { '0' })
            .collect();
        assert_eq!(eq, r[5], "equals {label}");
    }
}

// ---------------------------------------------------------------- GPUTEX

struct TestTexture {
    state: GpuTextureState,
    closed: bool,
}

impl GpuTexture for TestTexture {
    fn state(&self) -> &GpuTextureState {
        &self.state
    }

    fn close(&mut self) {
        self.closed = true;
    }

    fn is_closed(&self) -> bool {
        self.closed
    }
}

#[test]
fn gpu_texture_getters_match_the_jar() {
    const MIPS: [i32; 19] = [
        -64,
        -33,
        -32,
        -31,
        -1,
        0,
        1,
        2,
        4,
        7,
        8,
        30,
        31,
        32,
        33,
        63,
        64,
        i32::MAX,
        i32::MIN,
    ];
    let rows = rows("GPUTEX");
    assert_eq!(rows.len(), 8);
    for r in rows {
        if r[1] == "usage" {
            let got = [
                gpu_texture::USAGE_COPY_DST,
                gpu_texture::USAGE_COPY_SRC,
                gpu_texture::USAGE_TEXTURE_BINDING,
                gpu_texture::USAGE_RENDER_ATTACHMENT,
                gpu_texture::USAGE_CUBEMAP_COMPATIBLE,
            ];
            let want: Vec<i32> = r[2..7].iter().map(|v| i32f(v)).collect();
            assert_eq!(got.to_vec(), want);
            continue;
        }
        let mut t = TestTexture {
            state: GpuTextureState::new(
                i32f(r[2]),
                s(r[3]),
                gpu_format(r[4]),
                i32f(r[5]),
                i32f(r[6]),
                i32f(r[7]),
                i32f(r[8]),
            ),
            closed: false,
        };
        let w: Vec<String> = MIPS.iter().map(|&m| t.get_width(m).to_string()).collect();
        let h: Vec<String> = MIPS.iter().map(|&m| t.get_height(m).to_string()).collect();
        assert_eq!(w.join(","), r[9], "getWidth {r:?}");
        assert_eq!(h.join(","), r[10], "getHeight {r:?}");
        assert_eq!(t.get_depth_or_layers(), i32f(r[11]));
        assert_eq!(t.get_mip_levels(), i32f(r[12]));
        assert_eq!(t.get_format().name(), r[13]);
        assert_eq!(t.usage(), i32f(r[14]));
        assert_eq!(esc(t.get_label()), r[15]);
        assert!(!t.is_closed());
        t.close();
        assert!(t.is_closed());
    }
}

// ---------------------------------------------------------------- BINDGL / UNIDESC

fn layout_text(l: &BindGroupLayout) -> String {
    let uniforms: Vec<String> = l.get_uniforms().iter().map(|u| u.to_string()).collect();
    format!(
        "[{}/{}]",
        esc(&l.get_samplers().join("|")),
        esc(&format!("[{}]", uniforms.join(", ")))
    )
}

/// The probe's `runBuilder`, op for op.
fn run_builder(script: &str, built: &mut Vec<BindGroupLayout>) -> String {
    let mut b = bgl::builder();
    let mut out = Vec::new();
    for op in script.split(' ') {
        let (k, arg) = op.split_at(1);
        let p: Vec<&str> = arg.split(':').collect();
        let r = match k {
            "s" => or_ex(
                || {
                    b.with_sampler(p[0]);
                },
                |()| "ok".into(),
            ),
            "u" => or_ex(
                || {
                    b.with_uniform(p[0], uniform_type(p[1]));
                },
                |()| "ok".into(),
            ),
            "t" => or_ex(
                || {
                    b.with_uniform_format(p[0], uniform_type(p[1]), gpu_format(p[2]));
                },
                |()| "ok".into(),
            ),
            _ => {
                built.push(b.build());
                layout_text(built.last().unwrap())
            }
        };
        out.push(r);
    }
    out.join(",")
}

#[test]
fn bind_group_layouts_match_the_jar() {
    let mut built = Vec::new();
    for r in rows("BINDGL") {
        match r[1] {
            "build" => assert_eq!(run_builder(&s(r[2]), &mut built), r[3], "builder {}", r[2]),
            "list" => {
                let sel: Vec<&BindGroupLayout> = if r[2].is_empty() {
                    Vec::new()
                } else {
                    r[2].split(',')
                        .map(|n| &built[n.parse::<usize>().unwrap()])
                        .collect()
                };
                assert_eq!(
                    esc(&bgl::flatten_samplers(&sel).join("|")),
                    r[3],
                    "flattenSamplers {}",
                    r[2]
                );
                let u: Vec<String> = bgl::flatten_uniforms(&sel)
                    .iter()
                    .map(|u| u.to_string())
                    .collect();
                assert_eq!(
                    esc(&format!("[{}]", u.join(", "))),
                    r[4],
                    "flattenUniforms {}",
                    r[2]
                );
                assert_eq!(
                    or_ex(|| bgl::ensure_compatible(&sel), |()| "ok".into()),
                    r[5],
                    "ensureCompatible {}",
                    r[2]
                );
            }
            "count" => assert_eq!(built.len() as i32, i32f(r[2])),
            other => panic!("unknown BINDGL row {other}"),
        }
    }
}

#[test]
fn uniform_descriptions_match_the_jar() {
    let mut canon = Vec::new();
    for n in ["a", "Fog"] {
        for t in [UniformType::UniformBuffer, UniformType::TexelBuffer] {
            for g in ["null", "R32_FLOAT", "RGBA8_UNORM"] {
                let gf = (g != "null").then(|| gpu_format(g));
                canon.push((
                    format!("canon:{n}:{}:{g}", t.name()),
                    UniformDescription::new(n, t, gf),
                ));
            }
        }
    }
    let mut seen = 0;
    for r in rows("UNIDESC") {
        match r[1] {
            "type" => {
                let got = or_ex(
                    || UniformDescription::of_type(r[2], uniform_type(r[3])),
                    |d| esc(&d.to_string()),
                );
                assert_eq!(got, r[4], "UniformDescription({}, {})", r[2], r[3]);
            }
            "format" => {
                let d = UniformDescription::of_format(r[2], gpu_format(r[3]));
                assert_eq!(esc(&d.to_string()), r[4]);
            }
            "canon" => {
                let (key, d) = &canon[seen];
                seen += 1;
                assert_eq!(r[2], key);
                assert_eq!(esc(&d.to_string()), r[3], "toString {key}");
                assert_eq!(esc(d.name()), r[4]);
                assert_eq!(d.type_().name(), r[5]);
                assert_eq!(d.gpu_format().map_or("null", |g| g.name()), r[6]);
                let eq: String = canon
                    .iter()
                    .map(|(_, o)| if d == o { '1' } else { '0' })
                    .collect();
                assert_eq!(eq, r[7], "equals {key}");
            }
            other => panic!("unknown UNIDESC row {other}"),
        }
    }
    assert_eq!(seen, canon.len());
}

// ---------------------------------------------------------------- PROFRES

struct Times([i64; 4]);

impl ProfileResults for Times {
    fn get_times(&self, _path: &str) -> Vec<ResultField> {
        Vec::new()
    }
    fn save_results(&self, _file: &std::path::Path) -> bool {
        false
    }
    fn get_start_time_nano(&self) -> i64 {
        self.0[0]
    }
    fn get_start_time_ticks(&self) -> i32 {
        self.0[1] as i32
    }
    fn get_end_time_nano(&self) -> i64 {
        self.0[2]
    }
    fn get_end_time_ticks(&self) -> i32 {
        self.0[3] as i32
    }
    fn get_profiler_results(&self) -> String {
        String::new()
    }
}

#[test]
fn profile_results_defaults_and_the_empty_results_match_the_jar() {
    let rows = rows("PROFRES");
    assert_eq!(rows.len(), 16);
    for r in rows {
        match r[1] {
            "empty" => {
                let e = EmptyProfileResults::EMPTY;
                let got = [
                    e.get_times("root").len().to_string(),
                    e.get_times("").len().to_string(),
                    e.save_results(std::path::Path::new("nowhere/x.txt"))
                        .to_string(),
                    e.get_start_time_nano().to_string(),
                    e.get_start_time_ticks().to_string(),
                    e.get_end_time_nano().to_string(),
                    e.get_end_time_ticks().to_string(),
                    e.get_nano_duration().to_string(),
                    e.get_tick_duration().to_string(),
                    esc(&e.get_profiler_results()),
                ];
                assert_eq!(got.to_vec(), r[2..12].to_vec());
            }
            "sep" => assert_eq!(profile_results::PATH_SEPARATOR as i32, i32f(r[2])),
            "demangle" => assert_eq!(
                esc(&profile_results::demangle_path(&s(r[2]))),
                r[3],
                "{r:?}"
            ),
            "durations" => {
                let t = Times([i64f(r[2]), i64f(r[3]), i64f(r[4]), i64f(r[5])]);
                assert_eq!(t.get_nano_duration(), i64f(r[6]), "{r:?}");
                assert_eq!(t.get_tick_duration(), i32f(r[7]), "{r:?}");
            }
            other => panic!("unknown PROFRES row {other}"),
        }
    }
}

// ---------------------------------------------------------------- GRALLOC

struct Recording<'a> {
    log: &'a RefCell<Vec<String>>,
    counter: RefCell<i32>,
    with_prepare: bool,
}

impl ResourceDescriptor<i32> for Recording<'_> {
    fn allocate(&self) -> i32 {
        if !self.with_prepare {
            self.log.borrow_mut().push("allocate".into());
            return 7;
        }
        *self.counter.borrow_mut() += 1;
        let n = *self.counter.borrow();
        self.log.borrow_mut().push(format!("allocate#{n}"));
        n
    }

    fn prepare(&self, resource: &i32) {
        if self.with_prepare {
            self.log.borrow_mut().push(format!("prepare#{resource}"));
        }
    }

    fn free(&self, resource: i32) {
        if self.with_prepare {
            self.log.borrow_mut().push(format!("free#{resource}"));
        } else {
            self.log.borrow_mut().push(format!("free{resource}"));
        }
    }
}

#[test]
fn unpooled_allocator_calls_the_descriptor_as_the_jar_does() {
    let rows = rows("GRALLOC");
    assert_eq!(rows.len(), 2);
    let a = gra::UNPOOLED;
    let log = RefCell::new(Vec::new());
    let desc = Recording {
        log: &log,
        counter: RefCell::new(0),
        with_prepare: true,
    };
    let r1 = a.acquire(&desc);
    let r2 = a.acquire(&desc);
    a.release(&desc, r1);
    let r3 = a.acquire(&desc);
    a.release(&desc, r3);
    a.release(&desc, r2);
    assert_eq!(rows[0][1], "unpooled");
    assert_eq!(log.borrow().join(" "), rows[0][2]);
    assert_eq!(
        [r1, r2, r3].map(|v| v.to_string()).to_vec(),
        rows[0][3..6].to_vec()
    );

    // The default `prepare()` does nothing, so acquire is allocate alone.
    let log = RefCell::new(Vec::new());
    let plain = Recording {
        log: &log,
        counter: RefCell::new(0),
        with_prepare: false,
    };
    a.release(&plain, a.acquire(&plain));
    assert_eq!(rows[1][1], "default-prepare");
    assert_eq!(log.borrow().join(" "), rows[1][2]);
}

// ---------------------------------------------------------------- MULTFLOAT

/// The probe's `factor(spec)`.
struct Factor(String);

fn parse_java_float(s: &str) -> f32 {
    match s {
        "NaN" => f32::NAN,
        "Infinity" => f32::INFINITY,
        "-Infinity" => f32::NEG_INFINITY,
        _ => s.parse().unwrap(),
    }
}

impl SampledFloat for Factor {
    fn sample(&self, r: &mut dyn RandomSource) -> f32 {
        let spec = &self.0;
        if let Some(v) = spec.strip_prefix("C(") {
            return parse_java_float(&v[..v.len() - 1]);
        }
        if spec == "N" {
            return r.next_float() * 2.0 - 1.0;
        }
        let inner = &spec[2..spec.len() - 1];
        let (lo, hi) = inner.split_once(';').unwrap();
        let (lo, hi) = (parse_java_float(lo), parse_java_float(hi));
        lo + r.next_float() * (hi - lo)
    }
}

impl std::fmt::Display for Factor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[test]
fn multiplied_floats_match_the_jar() {
    let rows = rows("MULTFLOAT");
    assert_eq!(rows.len(), 30);
    for r in rows {
        let spec = s(r[1]);
        let factors: Vec<Box<dyn DisplaySampledFloat>> = if spec.is_empty() {
            Vec::new()
        } else {
            spec.split(' ')
                .map(|f| Box::new(Factor(f.to_string())) as Box<dyn DisplaySampledFloat>)
                .collect()
        };
        let m = MultipliedFloats::new(factors);
        let mut random = LegacyRandomSource::new(i64f(r[2]));
        let got: Vec<String> = (0..6).map(|_| fbits(m.sample(&mut random))).collect();
        assert_eq!(
            nan_canon(&got.join(",")),
            nan_canon(r[3]),
            "samples {spec} seed {}",
            r[2]
        );
        assert_eq!(esc(&m.to_string()), r[4], "toString {spec}");
        assert_eq!(random.next_long(), i64f(r[5]), "draws consumed {spec}");
    }
}
