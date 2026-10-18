//! Sub-batch B: records with int/boolean/String/long/byte components, and the `java_object`
//! hashing rules they stand on.
//!
//! Groups: `HASH`, `FEQ`, `DEQ`, `RHASH`, `OHASH`, `RMIX` (javacompat), `REC`/`RECEQ` (every
//! record), then one group per record method. Each instance list below is the probe's, in the
//! probe's order -- [`check_records`] fails on a length mismatch, so a case added on one side only
//! cannot slip through.

use crate::small_probe::*;

use minecraft_rust::javacompat::java_lang;
use minecraft_rust::javacompat::java_lang::float_to_string::{double_to_string, float_to_string};
use minecraft_rust::javacompat::java_object;

use minecraft_rust::com::mojang::blaze3d::platform::Transparency::Transparency;
use minecraft_rust::com::mojang::blaze3d::shaders::GpuDebugOptions::GpuDebugOptions;
use minecraft_rust::com::mojang::blaze3d::systems::DeviceFeatures::DeviceFeatures;
use minecraft_rust::com::mojang::blaze3d::systems::HintsAndWorkarounds::HintsAndWorkarounds;
use minecraft_rust::com::mojang::blaze3d::vulkan::glsl::SpvSampler::SpvSampler;
use minecraft_rust::com::mojang::blaze3d::vulkan::glsl::SpvUniformBuffer::SpvUniformBuffer;
use minecraft_rust::com::mojang::blaze3d::vulkan::glsl::SpvVariable::SpvVariable;
use minecraft_rust::com::mojang::realmsclient::gui::screens::UploadResult::{
    self as upload_mod, UploadResult,
};
use minecraft_rust::net::minecraft::client::gui::navigation::ScreenAxis::ScreenAxis;
use minecraft_rust::net::minecraft::client::gui::navigation::ScreenDirection::ScreenDirection;
use minecraft_rust::net::minecraft::client::gui::navigation::ScreenPosition::ScreenPosition;
use minecraft_rust::net::minecraft::client::resources::metadata::animation::FrameSize::FrameSize;
use minecraft_rust::net::minecraft::commands::execution::ChainModifiers::ChainModifiers;
use minecraft_rust::net::minecraft::gametest::framework::RetryOptions::RetryOptions;
use minecraft_rust::net::minecraft::server::jsonrpc::methods::ClientInfo::ClientInfo;
use minecraft_rust::net::minecraft::server::level::ColumnPos::ColumnPos;
use minecraft_rust::net::minecraft::world::Stopwatch::Stopwatch;

const MIN: i32 = i32::MIN;
const MAX: i32 = i32::MAX;

// =====================================================================================
// The generic record check
// =====================================================================================

/// Checks `xs` against the probe's `REC`/`RECEQ` rows for `id`: same number of instances, and for
/// each one the `toString`, the `hashCode` (unless the probe wrote `-`), and its whole row of the
/// `equals` matrix.
pub fn check_records<T: std::fmt::Display + PartialEq>(
    id: &str,
    xs: &[T],
    hash: &dyn Fn(&T) -> i32,
) {
    let recs: Vec<_> = rows("REC").into_iter().filter(|r| r[1] == id).collect();
    let eqs: Vec<_> = rows("RECEQ").into_iter().filter(|r| r[1] == id).collect();
    assert_eq!(
        recs.len(),
        xs.len(),
        "{id}: the probe built {} instances, the test {}",
        recs.len(),
        xs.len()
    );
    assert_eq!(eqs.len(), xs.len(), "{id}: RECEQ row count");
    for (a, x) in xs.iter().enumerate() {
        let r = &recs[a];
        assert_eq!(i32f(r[2]) as usize, a, "{id}: REC rows out of order");
        assert_eq!(Some(x.to_string()), unesc(r[3]), "{id}[{a}]: toString()");
        if r[4] != "-" {
            assert_eq!(hash(x), i32f(r[4]), "{id}[{a}]: hashCode() of {x}");
        }
        let bits: Vec<char> = eqs[a][3].chars().collect();
        assert_eq!(bits.len(), xs.len(), "{id}[{a}]: RECEQ width");
        for (k, y) in xs.iter().enumerate() {
            assert_eq!(
                x == y,
                bits[k] == '1',
                "{id}: [{a}].equals([{k}]) for {x} vs {y}"
            );
        }
    }
}

// =====================================================================================
// javacompat::java_object against the JDK
// =====================================================================================

#[test]
fn boxed_hash_codes_match_the_jdk() {
    for r in rows("HASH") {
        let got = match r[1] {
            "bool" => java_object::hash_bool(boolf(r[2])),
            "long" => java_object::hash_i64(i64f(r[2])),
            "float" => java_object::hash_f32(f32f(r[2])),
            "double" => java_object::hash_f64(f64f(r[2])),
            other => panic!("unexpected HASH kind {other}"),
        };
        assert_eq!(got, i32f(r[3]), "{}.hashCode({})", r[1], r[2]);
    }
}

#[test]
fn float_and_double_equality_is_float_compare() {
    for r in rows("FEQ") {
        assert_eq!(
            java_object::equals_f32(f32f(r[1]), f32f(r[2])),
            boolf(r[3]),
            "Float.compare({}, {})",
            r[1],
            r[2]
        );
    }
    for r in rows("DEQ") {
        assert_eq!(
            java_object::equals_f64(f64f(r[1]), f64f(r[2])),
            boolf(r[3]),
            "Double.compare({}, {})",
            r[1],
            r[2]
        );
    }
}

fn csv_ints(field: &str) -> Vec<i32> {
    if field.is_empty() {
        Vec::new()
    } else {
        field.split(',').map(i32f).collect()
    }
}

#[test]
fn record_hash_folds_from_zero_and_objects_hash_from_one() {
    for r in rows("RHASH") {
        let v = csv_ints(r[2]);
        assert_eq!(v.len(), i32f(r[1]) as usize);
        assert_eq!(
            java_object::record_hash(&v),
            i32f(r[3]),
            "record hash of {v:?}"
        );
    }
    for r in rows("OHASH") {
        let v = csv_ints(r[1]);
        assert_eq!(
            java_object::objects_hash(&v),
            i32f(r[2]),
            "Objects.hash{v:?}"
        );
    }
}

#[test]
fn a_record_mixing_every_component_kind() {
    // The probe's `RMix(boolean z, long j, float f, double dd, String s, byte by)` instances.
    struct Mix(bool, i64, f32, f64, Option<&'static str>, i8);
    let mixes = [
        Mix(true, 5, 1.5, -0.0, Some("s"), -3),
        Mix(false, i64::MIN, f32::NAN, f64::NAN, None, 127),
    ];
    let rows = rows("RMIX");
    assert_eq!(rows.len(), mixes.len());
    for (k, m) in mixes.iter().enumerate() {
        let text = format!(
            "RMix[z={}, j={}, f={}, dd={}, s={}, by={}]",
            m.0,
            m.1,
            float_to_string(m.2),
            double_to_string(m.3),
            java_object::str_or_null(m.4),
            m.5
        );
        let hash = java_object::record_hash(&[
            java_object::hash_bool(m.0),
            java_object::hash_i64(m.1),
            java_object::hash_f32(m.2),
            java_object::hash_f64(m.3),
            java_object::hash_opt(m.4.as_ref(), |s| java_lang::string_hash_code(s)),
            m.5 as i32,
        ]);
        assert_eq!(Some(text), unesc(rows[k][2]), "RMix[{k}] toString");
        assert_eq!(hash, i32f(rows[k][3]), "RMix[{k}] hashCode");
    }
}

// =====================================================================================
// The records
// =====================================================================================

#[test]
fn plain_records_match_the_jar() {
    check_records(
        "GpuDebugOptions",
        &[
            GpuDebugOptions::new(0, false, false, false),
            GpuDebugOptions::new(0, false, false, false),
            GpuDebugOptions::new(1, true, false, true),
            GpuDebugOptions::new(-1, false, true, false),
            GpuDebugOptions::new(MIN, true, true, true),
            GpuDebugOptions::new(MAX, false, false, true),
        ],
        &|x| x.java_hash_code(),
    );
    check_records(
        "HintsAndWorkarounds",
        &[
            HintsAndWorkarounds::new(false, false),
            HintsAndWorkarounds::new(false, false),
            HintsAndWorkarounds::new(true, false),
            HintsAndWorkarounds::new(false, true),
            HintsAndWorkarounds::new(true, true),
        ],
        &|x| x.java_hash_code(),
    );
    check_records(
        "SpvSampler",
        &[
            SpvSampler::new("tex", 0, 1),
            SpvSampler::new("tex", 0, 1),
            SpvSampler::new("", -1, 6),
            SpvSampler::new("\u{e9}\u{4e2d}", 7, MIN),
            SpvSampler::new("tex", 0, 2),
            SpvSampler::new("tex", 1, 1),
        ],
        &|x| x.java_hash_code(),
    );
    check_records(
        "SpvUniformBuffer",
        &[
            SpvUniformBuffer::new("Globals", 0),
            SpvUniformBuffer::new("Globals", 0),
            SpvUniformBuffer::new("", MAX),
            SpvUniformBuffer::new("Globals", 1),
            SpvUniformBuffer::new("globals", 0),
        ],
        &|x| x.java_hash_code(),
    );
    check_records(
        "SpvVariable",
        &[
            SpvVariable::new("Position", 0),
            SpvVariable::new("Position", 0),
            SpvVariable::new("UV0", -1),
            SpvVariable::new("\u{1F600}", MIN),
            SpvVariable::new("Position", 3),
        ],
        &|x| x.java_hash_code(),
    );
    check_records(
        "FrameSize",
        &[
            FrameSize::new(16, 16),
            FrameSize::new(16, 16),
            FrameSize::new(0, 0),
            FrameSize::new(-1, 5),
            FrameSize::new(5, -1),
            FrameSize::new(MIN, MAX),
        ],
        &|x| x.java_hash_code(),
    );
    let df = [
        [false; 7],
        [false; 7],
        [true; 7],
        [true, false, true, false, true, false, true],
        [false, false, false, true, false, false, false],
        [false, false, false, false, false, false, true],
    ];
    let dfs: Vec<DeviceFeatures> = df
        .iter()
        .map(|v| DeviceFeatures::new(v[0], v[1], v[2], v[3], v[4], v[5], v[6]))
        .collect();
    check_records("DeviceFeatures", &dfs, &|x| x.java_hash_code());
}

#[test]
fn plain_record_accessors_return_their_components() {
    let o = GpuDebugOptions::new(-1, true, false, true);
    assert_eq!(
        (
            o.log_level(),
            o.synchronous_logs(),
            o.use_labels(),
            o.use_validation_layers()
        ),
        (-1, true, false, true)
    );
    let s = SpvSampler::new("tex", 2, 5);
    assert_eq!(
        (s.name(), s.binding_offset(), s.dimensions()),
        ("tex", 2, 5)
    );
    let d = DeviceFeatures::new(true, false, false, false, false, false, true);
    assert!(d.shader_draw_parameters() && d.persistent_mapping() && !d.draw_indirect());
    assert_eq!(
        (FrameSize::new(3, 4).width(), FrameSize::new(3, 4).height()),
        (3, 4)
    );
    let h = HintsAndWorkarounds::new(true, false);
    assert!(h.write_to_buffer_is_slow() && !h.anisotropy_has_known_issues());
    assert_eq!(SpvUniformBuffer::new("U", 9).binding_offset(), 9);
    assert_eq!(SpvVariable::new("V", 4).location_offset(), 4);
}

const CHAIN_FLAGS: [i8; 9] = [0, 0, 1, 2, 3, 4, -128, 127, -1];

#[test]
fn chain_modifiers_match_the_jar() {
    let xs: Vec<ChainModifiers> = CHAIN_FLAGS
        .iter()
        .map(|&f| ChainModifiers::new(f))
        .collect();
    check_records("ChainModifiers", &xs, &|x| x.java_hash_code());
    assert_eq!(
        ChainModifiers::DEFAULT.flags() as i32,
        i32f(rows("CHAINDEF")[0][1])
    );
    let rows = rows("CHAIN");
    assert_eq!(rows.len(), CHAIN_FLAGS.len());
    for (r, &fl) in rows.iter().zip(CHAIN_FLAGS.iter()) {
        let c = ChainModifiers::new(fl);
        assert_eq!(fl as i32, i32f(r[1]));
        assert_eq!(c.is_forked(), boolf(r[2]), "{fl}: isForked");
        assert_eq!(c.is_return(), boolf(r[3]), "{fl}: isReturn");
        assert_eq!(c.set_forked().flags() as i32, i32f(r[4]), "{fl}: setForked");
        assert_eq!(c.set_return().flags() as i32, i32f(r[5]), "{fl}: setReturn");
        // Java returns `this` exactly when the flag was already set; the port returns an equal copy.
        assert_eq!(
            c.set_forked() == c,
            boolf(r[6]),
            "{fl}: setForked() == this"
        );
        assert_eq!(
            c.set_return() == c,
            boolf(r[7]),
            "{fl}: setReturn() == this"
        );
    }
}

#[test]
fn retry_options_match_the_jar() {
    check_records(
        "RetryOptions",
        &[
            RetryOptions::no_retries(),
            RetryOptions::new(1, true),
            RetryOptions::new(0, false),
            RetryOptions::new(-5, true),
            RetryOptions::new(3, false),
            RetryOptions::new(MAX, true),
        ],
        &|x| x.java_hash_code(),
    );
    let attempts = [0, 1, 2, 3, 4, MAX];
    let successes = [0, 1, 2, 3];
    let rows = rows("RETRY");
    assert_eq!(rows.len(), 14);
    for r in rows {
        let o = RetryOptions::new(i32f(r[1]), boolf(r[2]));
        assert_eq!(
            (o.number_of_tries(), o.halt_on_failure()),
            (i32f(r[1]), boolf(r[2]))
        );
        assert_eq!(o.unlimited_tries(), boolf(r[3]), "{o}: unlimitedTries");
        assert_eq!(o.has_retries(), boolf(r[4]), "{o}: hasRetries");
        let mut bits = String::new();
        for &at in &attempts {
            for &su in &successes {
                bits.push(if o.has_tries_left(at, su) { '1' } else { '0' });
            }
        }
        assert_eq!(bits, r[5], "{o}: hasTriesLeft grid");
    }
}

const TIMES: [i64; 7] = [0, 1, -1, 1000, 123_456_789_012, i64::MIN, i64::MAX];

#[test]
fn stopwatch_matches_the_jar() {
    let mut xs: Vec<Stopwatch> = (0..TIMES.len())
        .map(|k| Stopwatch::new(TIMES[k], TIMES[(k + 3) % TIMES.len()]))
        .collect();
    xs.push(Stopwatch::starting_at(42));
    xs.push(Stopwatch::new(42, 0));
    check_records("Stopwatch", &xs, &|x| x.java_hash_code());
    let rows = rows("WATCH");
    assert_eq!(rows.len(), TIMES.len().pow(3));
    for r in rows {
        let w = Stopwatch::new(i64f(r[1]), i64f(r[2]));
        assert_eq!(
            (w.creation_time(), w.accumulated_elapsed_time()),
            (i64f(r[1]), i64f(r[2]))
        );
        let now = i64f(r[3]);
        assert_eq!(
            w.elapsed_milliseconds(now),
            i64f(r[4]),
            "{w}.elapsedMilliseconds({now})"
        );
        assert_eq!(
            hex64(w.elapsed_seconds(now)),
            r[5],
            "{w}.elapsedSeconds({now})"
        );
    }
}

fn axis(name: &str) -> ScreenAxis {
    match name {
        "HORIZONTAL" => ScreenAxis::Horizontal,
        "VERTICAL" => ScreenAxis::Vertical,
        other => panic!("unknown ScreenAxis {other}"),
    }
}

fn direction(name: &str) -> ScreenDirection {
    match name {
        "UP" => ScreenDirection::Up,
        "DOWN" => ScreenDirection::Down,
        "LEFT" => ScreenDirection::Left,
        "RIGHT" => ScreenDirection::Right,
        other => panic!("unknown ScreenDirection {other}"),
    }
}

#[test]
fn screen_position_matches_the_jar() {
    check_records(
        "ScreenPosition",
        &[
            ScreenPosition::new(0, 0),
            ScreenPosition::new(0, 0),
            ScreenPosition::new(1, 2),
            ScreenPosition::new(2, 1),
            ScreenPosition::new(-7, 3),
            ScreenPosition::new(MIN, MAX),
        ],
        &|x| x.java_hash_code(),
    );
    for r in rows("SPOF") {
        let p = ScreenPosition::of(axis(r[1]), i32f(r[2]), i32f(r[3]));
        assert_eq!(
            (p.x(), p.y()),
            (i32f(r[4]), i32f(r[5])),
            "of({}, {}, {})",
            r[1],
            r[2],
            r[3]
        );
    }
    for r in rows("SPSTEP") {
        let p = ScreenPosition::new(i32f(r[1]), i32f(r[2])).step(direction(r[3]));
        assert_eq!(
            (p.x(), p.y()),
            (i32f(r[4]), i32f(r[5])),
            "({}, {}).step({})",
            r[1],
            r[2],
            r[3]
        );
    }
    for r in rows("SPCOORD") {
        let p = ScreenPosition::new(i32f(r[1]), i32f(r[2]));
        assert_eq!(
            p.get_coordinate(axis(r[3])),
            i32f(r[4]),
            "{p}.getCoordinate({})",
            r[3]
        );
    }
}

#[test]
fn transparency_matches_the_jar() {
    let tc = [
        Transparency::NONE,
        Transparency::TRANSPARENT,
        Transparency::TRANSLUCENT,
        Transparency::TRANSPARENT_AND_TRANSLUCENT,
    ];
    check_records(
        "Transparency",
        &[tc[0], tc[1], tc[2], tc[3], Transparency::new(false, false)],
        &|x| x.java_hash_code(),
    );
    let index = |t: Transparency| tc.iter().position(|c| *c == t).map_or(-1, |p| p as i32);
    for r in rows("TROF") {
        let t = Transparency::of(boolf(r[1]), boolf(r[2]));
        assert_eq!(
            (t.has_transparent(), t.has_translucent()),
            (boolf(r[1]), boolf(r[2]))
        );
        assert_eq!(index(t), i32f(r[3]), "of({}, {})", r[1], r[2]);
    }
    for r in rows("TROR") {
        let a = if r[1] == "fresh" {
            Transparency::new(false, false)
        } else {
            tc[i32f(r[1]) as usize]
        };
        assert_eq!(
            index(a.or(&tc[i32f(r[2]) as usize])),
            i32f(r[3]),
            "[{}].or([{}])",
            r[1],
            r[2]
        );
    }
    for r in rows("TROPAQUE") {
        assert_eq!(
            tc[i32f(r[1]) as usize].is_opaque(),
            boolf(r[2]),
            "[{}].isOpaque",
            r[1]
        );
    }
}

#[test]
fn column_pos_matches_the_jar() {
    check_records(
        "ColumnPos",
        &[
            ColumnPos::new(0, 0),
            ColumnPos::new(0, 0),
            ColumnPos::new(1, 2),
            ColumnPos::new(2, 1),
            ColumnPos::new(-1, -1),
            ColumnPos::new(31, -17),
            ColumnPos::new(MIN, MAX),
        ],
        &|x| x.java_hash_code(),
    );
    for r in rows("COLPOS") {
        let (x, z) = (i32f(r[1]), i32f(r[2]));
        let c = ColumnPos::new(x, z);
        assert_eq!((c.x(), c.z()), (x, z));
        let ch = c.to_chunk_pos();
        assert_eq!((ch.x, ch.z), (i32f(r[3]), i32f(r[4])), "{c}.toChunkPos()");
        assert_eq!(c.to_long(), i64f(r[5]), "{c}.toLong()");
        let packed = ColumnPos::as_long(x, z);
        assert_eq!(packed, i64f(r[6]), "asLong({x}, {z})");
        assert_eq!(
            (ColumnPos::get_x(packed), ColumnPos::get_z(packed)),
            (i32f(r[7]), i32f(r[8])),
            "unpack {packed}"
        );
    }
    for r in rows("COLPOSRAW") {
        let raw = i64f(r[1]);
        assert_eq!(
            (ColumnPos::get_x(raw), ColumnPos::get_z(raw)),
            (i32f(r[2]), i32f(r[3])),
            "getX/getZ({raw})"
        );
    }
}

#[test]
fn client_info_matches_the_jar() {
    let ids = [None, None, Some(0), Some(5), Some(-1), Some(MIN)];
    let xs: Vec<ClientInfo> = ids.iter().map(|&id| ClientInfo::new(id)).collect();
    check_records("ClientInfo", &xs, &|x| x.java_hash_code());
    let rows = rows("CLIENTINFO");
    assert_eq!(rows.len(), ids.len());
    for (r, &id) in rows.iter().zip(ids.iter()) {
        assert_eq!(id.map(|v: i32| v.to_string()), unesc(r[1]));
        assert_eq!(ClientInfo::of(id) == ClientInfo::new(id), boolf(r[2]));
        assert_eq!(ClientInfo::of(id).connection_id(), id);
    }
}

#[test]
fn upload_result_matches_the_jar() {
    check_records(
        "UploadResult",
        &[
            UploadResult::new(200, None),
            UploadResult::new(200, None),
            UploadResult::new(400, Some("bad")),
            UploadResult::new(400, Some("")),
            UploadResult::new(-1, None),
            UploadResult::new(500, Some("x")),
        ],
        &|x| x.java_hash_code(),
    );
    let errs = [None, Some(""), Some("bad request")];
    for r in rows("UPLOAD") {
        let u = UploadResult::new(i32f(r[1]), errs[i32f(r[2]) as usize]);
        assert_eq!(
            (u.status_code(), u.error_message()),
            (i32f(r[1]), errs[i32f(r[2]) as usize])
        );
        assert_eq!(
            u.get_simplified_error_message(),
            unesc(r[3]),
            "{u}.getSimplifiedErrorMessage()"
        );
    }
    let up = rows("UPBUILD");
    let find = |case: &str| {
        up.iter()
            .find(|r| r[1] == case)
            .unwrap_or_else(|| panic!("no UPBUILD `{case}`"))
    };
    // The probe's script, on ONE builder.
    let mut b = upload_mod::Builder::new();
    assert_eq!(Some(b.build().to_string()), unesc(find("default")[2]));
    assert_eq!(
        Some(b.with_status_code(404).build().to_string()),
        unesc(find("status")[2])
    );
    assert_eq!(
        Some(b.with_error_message(Some("gone")).build().to_string()),
        unesc(find("both")[2])
    );
    assert_eq!(
        Some(b.with_error_message(None).build().to_string()),
        unesc(find("reset-message")[2])
    );
    // `withStatusCode` / `withErrorMessage` return the same builder: `&mut Self` is that by type.
    let r = find("chain-identity");
    assert!(boolf(r[2]) && boolf(r[3]));
    let p: *const upload_mod::Builder = &b;
    assert!(std::ptr::eq(b.with_status_code(1), p));
}
