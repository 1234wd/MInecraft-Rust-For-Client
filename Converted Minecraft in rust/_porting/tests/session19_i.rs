//! Session 19 batch i: worldgen noise (levelgen/synth). Every row of
//! `_porting/test-data/noise_probe.txt` -- produced by `_porting/java-oracle/src/p/NoiseProbe.java`
//! running against the real 26.2 jar on JDK 25 -- is replayed against the port and compared
//! bit for bit (doubles are raw IEEE-754 bits in the file).

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

use minecraft_rust::javacompat::golden::test_data_dir;
use minecraft_rust::net::minecraft::util::RandomSource::RandomSource;
use minecraft_rust::net::minecraft::world::level::levelgen::synth::ImprovedNoise::ImprovedNoise;
use minecraft_rust::net::minecraft::world::level::levelgen::synth::NormalNoise::{NoiseParameters, NormalNoise};
use minecraft_rust::net::minecraft::world::level::levelgen::synth::PerlinNoise::{wrap, PerlinNoise};
use minecraft_rust::net::minecraft::world::level::levelgen::synth::PerlinSimplexNoise::PerlinSimplexNoise;
use minecraft_rust::net::minecraft::world::level::levelgen::synth::SimplexNoise::{SimplexNoise, F2, G2, SQRT_3};
use minecraft_rust::net::minecraft::world::level::levelgen::LegacyRandomSource::LegacyRandomSource;
use minecraft_rust::net::minecraft::world::level::levelgen::WorldgenRandom::WorldgenRandom;
use minecraft_rust::net::minecraft::world::level::levelgen::XoroshiroRandomSource::XoroshiroRandomSource;

fn rows() -> Vec<Vec<String>> {
    let path = test_data_dir().join("noise_probe.txt");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| l.split('|').map(str::to_string).collect())
        .collect()
}

fn d(hex: &str) -> f64 {
    f64::from_bits(u64::from_str_radix(hex, 16).unwrap_or_else(|e| panic!("bad hex `{hex}`: {e}")))
}

fn src(id: &str) -> Box<dyn RandomSource> {
    let seed: i64 = id[1..].parse().unwrap();
    match &id[..1] {
        "L" => Box::new(LegacyRandomSource::new(seed)),
        "X" => Box::new(XoroshiroRandomSource::new(seed)),
        "W" => Box::new(WorldgenRandom::new(Box::new(LegacyRandomSource::new(seed)))),
        other => panic!("unknown source kind {other}"),
    }
}

fn same(kind: &str, row: &[String], expected: f64, actual: f64) {
    assert_eq!(
        actual.to_bits(),
        expected.to_bits(),
        "{kind} {row:?}: jar {expected:e} ({:016x}), port {actual:e} ({:016x})",
        expected.to_bits(),
        actual.to_bits()
    );
}

fn of_kind<'a>(all: &'a [Vec<String>], kind: &str) -> Vec<&'a Vec<String>> {
    let v: Vec<_> = all.iter().filter(|r| r[0] == kind).collect();
    assert!(!v.is_empty(), "probe has no `{kind}` rows -- a group that tests nothing must not pass");
    v
}

#[test]
fn simplex_constants() {
    assert_eq!(SQRT_3.to_bits(), 3.0f64.sqrt().to_bits());
    assert_eq!(F2.to_bits(), (0.5 * (3.0f64.sqrt() - 1.0)).to_bits());
    assert_eq!(G2.to_bits(), ((3.0 - 3.0f64.sqrt()) / 6.0).to_bits());
}

#[test]
fn simplex_noise_matches_the_jar() {
    let all = rows();
    let mut n_checked = 0;
    for r in of_kind(&all, "simplex.origin") {
        let n = SimplexNoise::new(&mut *src(&r[1]));
        same("simplex.origin.xo", r, d(&r[2]), n.xo);
        same("simplex.origin.yo", r, d(&r[3]), n.yo);
        same("simplex.origin.zo", r, d(&r[4]), n.zo);
        for q in all.iter().filter(|q| q[1] == r[1]) {
            match q[0].as_str() {
                "simplex.2d" => {
                    same("simplex.2d", q, d(&q[4]), n.get_value_2d(d(&q[2]), d(&q[3])));
                    n_checked += 1;
                }
                "simplex.3d" => {
                    same("simplex.3d", q, d(&q[5]), n.get_value_3d(d(&q[2]), d(&q[3]), d(&q[4])));
                    n_checked += 1;
                }
                _ => {}
            }
        }
    }
    assert!(n_checked > 500, "only {n_checked} simplex samples checked");
}

#[test]
fn improved_noise_matches_the_jar() {
    let all = rows();
    let mut n_checked = 0;
    for r in of_kind(&all, "improved.origin") {
        let n = ImprovedNoise::new(&mut *src(&r[1]));
        same("improved.origin.xo", r, d(&r[2]), n.xo);
        same("improved.origin.yo", r, d(&r[3]), n.yo);
        same("improved.origin.zo", r, d(&r[4]), n.zo);
        for q in all.iter().filter(|q| q[1] == r[1]) {
            match q[0].as_str() {
                "improved.parity" => {
                    let mut sb = String::new();
                    n.parity_config_string(&mut sb);
                    assert_eq!(sb, q[2].replace('_', " "), "improved.parity {}", q[1]);
                }
                "improved.noise" => {
                    same("improved.noise", q, d(&q[5]), n.noise(d(&q[2]), d(&q[3]), d(&q[4])));
                    n_checked += 1;
                }
                "improved.fudge" => {
                    let v = n.noise_with_fudge(d(&q[2]), d(&q[3]), d(&q[4]), d(&q[5]), d(&q[6]));
                    same("improved.fudge", q, d(&q[7]), v);
                    n_checked += 1;
                }
                "improved.deriv" => {
                    let mut out = [0.5, -0.25, 1.0];
                    let v = n.noise_with_derivative(d(&q[2]), d(&q[3]), d(&q[4]), &mut out);
                    same("improved.deriv.value", q, d(&q[5]), v);
                    same("improved.deriv.dx", q, d(&q[6]), out[0]);
                    same("improved.deriv.dy", q, d(&q[7]), out[1]);
                    same("improved.deriv.dz", q, d(&q[8]), out[2]);
                    n_checked += 1;
                }
                _ => {}
            }
        }
    }
    assert!(n_checked > 500, "only {n_checked} improved-noise samples checked");
}

#[test]
fn perlin_wrap_matches_the_jar() {
    for r in of_kind(&rows(), "perlin.wrap") {
        same("perlin.wrap", r, d(&r[3]), wrap(d(&r[2])));
    }
}

fn build_perlin(kind: &str, case: &str) -> PerlinNoise {
    let parts: Vec<&str> = case.split(';').collect();
    match kind {
        "perlin.new" => {
            let amps: Vec<f64> = parts[2].split(',').map(|a| a.parse().unwrap()).collect();
            PerlinNoise::create_with_amplitudes(&mut *src(parts[0]), parts[1].parse().unwrap(), amps)
        }
        "perlin.set" => {
            let octs: Vec<i32> = parts[1].split(',').map(|a| a.parse().unwrap()).collect();
            // The probe passed List.of(-5, -3, -1, 0, -3): the duplicate must collapse.
            let mut with_dup = octs.clone();
            with_dup.push(-3);
            PerlinNoise::create_from_octaves(&mut *src(parts[0]), with_dup)
        }
        "perlin.legacyBlended" => PerlinNoise::create_legacy_for_blended_noise(&mut *src(parts[0]), -15..=0),
        "perlin.legacyNether" => {
            let amps: Vec<f64> = parts[2].split(',').map(|a| a.parse().unwrap()).collect();
            PerlinNoise::create_legacy_for_legacy_nether_biome(&mut *src(parts[0]), parts[1].parse().unwrap(), amps)
        }
        other => panic!("unknown perlin kind {other}"),
    }
}

#[test]
fn perlin_noise_matches_the_jar() {
    let all = rows();
    let mut cases: Vec<(String, String)> = Vec::new();
    for r in &all {
        if r[0].starts_with("perlin.") && r[0] != "perlin.wrap" && r[0] != "perlin.error" {
            let key = (r[0].clone(), r[1].clone());
            if !cases.contains(&key) {
                cases.push(key);
            }
        }
    }
    assert_eq!(cases.len(), 7, "four create(..) cases, one octave set, two legacy cases");
    let mut n_checked = 0;
    for (kind, case) in &cases {
        let n = build_perlin(kind, case);
        for q in all.iter().filter(|q| &q[0] == kind && &q[1] == case) {
            match q[2].as_str() {
                "parity" => {
                    let mut sb = String::new();
                    n.parity_config_string(&mut sb);
                    assert_eq!(sb, q[3].replace('_', " "), "{kind} {case} parity");
                }
                "maxBroken" => same("perlin.maxBroken", q, d(&q[4]), n.max_broken_value(d(&q[3]))),
                "value" => {
                    same("perlin.value", q, d(&q[6]), n.get_value(d(&q[3]), d(&q[4]), d(&q[5])));
                    n_checked += 1;
                }
                "valueFudge" => {
                    let v = n.get_value_with_fudge(d(&q[3]), d(&q[4]), d(&q[5]), d(&q[6]), d(&q[7]));
                    same("perlin.valueFudge", q, d(&q[8]), v);
                    n_checked += 1;
                }
                other => panic!("unknown perlin sub-kind {other}"),
            }
        }
    }
    assert!(n_checked > 500, "only {n_checked} perlin samples checked");
}

#[test]
fn perlin_noise_construction_errors_match_the_jar() {
    for r in of_kind(&rows(), "perlin.error") {
        let got = match r[1].as_str() {
            "positive" => std::panic::catch_unwind(|| {
                PerlinNoise::create_legacy_for_legacy_nether_biome(&mut *src("L1"), 0, vec![1.0, 1.0]);
            }),
            "empty" => std::panic::catch_unwind(|| {
                PerlinNoise::create_from_octaves(&mut *src("X1"), Vec::<i32>::new());
            }),
            other => panic!("unknown error case {other}"),
        };
        let msg = got.expect_err("the jar threw here");
        let msg = msg.downcast_ref::<String>().cloned().unwrap();
        assert_eq!(msg, r[2], "perlin.error {}", r[1]);
    }
}

#[test]
fn perlin_noise_octave_lookup_counts_from_the_high_end() {
    let n = PerlinNoise::create_with_amplitudes(&mut *src("X5"), -2, vec![1.0, 0.0, 1.0]);
    assert!(n.get_octave_noise(0).is_some(), "index 0 is the LAST (highest-frequency) level");
    assert!(n.get_octave_noise(1).is_none());
    assert!(n.get_octave_noise(2).is_some());
    let err = std::panic::catch_unwind(|| {
        n.get_octave_noise(3);
    })
    .expect_err("out of range must throw like Java");
    assert_eq!(
        err.downcast_ref::<String>().unwrap(),
        "java.lang.ArrayIndexOutOfBoundsException: Index -1 out of bounds for length 3"
    );
}

#[test]
fn perlin_simplex_noise_matches_the_jar() {
    let all = rows();
    let mut n_checked = 0;
    let mut built: Vec<(String, PerlinSimplexNoise)> = Vec::new();
    for q in of_kind(&all, "perlinSimplex.value") {
        if !built.iter().any(|(k, _)| k == &q[1]) {
            let (seed, octs) = q[1].split_once(';').unwrap();
            let octs: Vec<i32> = octs.trim_matches(|c| c == '[' || c == ']').split(',').map(|a| a.parse().unwrap()).collect();
            built.push((q[1].clone(), PerlinSimplexNoise::new(&mut *src(seed), octs)));
        }
        let n = &built.iter().find(|(k, _)| k == &q[1]).unwrap().1;
        let use_start = q[4] == "true";
        same("perlinSimplex.value", q, d(&q[5]), n.get_value(d(&q[2]), d(&q[3]), use_start));
        n_checked += 1;
    }
    assert_eq!(built.len(), 5);
    assert!(n_checked > 300, "only {n_checked} perlin-simplex samples checked");
}

fn build_normal(kind: &str, case: &str) -> NormalNoise {
    let parts: Vec<&str> = case.split(';').collect();
    let amps: Vec<f64> = parts[2].split(',').map(|a| a.parse().unwrap()).collect();
    let first: i32 = parts[1].parse().unwrap();
    match kind {
        "normal.new" => NormalNoise::create(&mut *src(parts[0]), first, &amps),
        "normal.legacy" => NormalNoise::create_legacy_nether_biome(
            &mut *src(parts[0]),
            NoiseParameters::with_first(first, amps[0], &amps[1..]),
        ),
        other => panic!("unknown normal kind {other}"),
    }
}

#[test]
fn normal_noise_matches_the_jar() {
    let all = rows();
    let mut cases: Vec<(String, String)> = Vec::new();
    for r in all.iter().filter(|r| r[0].starts_with("normal.")) {
        let key = (r[0].clone(), r[1].clone());
        if !cases.contains(&key) {
            cases.push(key);
        }
    }
    assert_eq!(cases.len(), 4);
    let mut n_checked = 0;
    for (kind, case) in &cases {
        let n = build_normal(kind, case);
        for q in all.iter().filter(|q| &q[0] == kind && &q[1] == case) {
            match q[2].as_str() {
                "parity" => {
                    let mut sb = String::new();
                    n.parity_config_string(&mut sb);
                    assert_eq!(sb, q[3].replace('_', " "), "{kind} {case} parity");
                }
                "max" => same("normal.max", q, d(&q[3]), n.max_value()),
                "value" => {
                    same("normal.value", q, d(&q[6]), n.get_value(d(&q[3]), d(&q[4]), d(&q[5])));
                    n_checked += 1;
                }
                other => panic!("unknown normal sub-kind {other}"),
            }
        }
    }
    assert!(n_checked > 100, "only {n_checked} normal-noise samples checked");
}

#[test]
fn normal_noise_parameters_record() {
    let a = NoiseParameters::with_first(-3, 1.0, &[0.5, 0.0]);
    assert_eq!(a.first_octave(), -3);
    assert_eq!(a.amplitudes(), &[1.0, 0.5, 0.0]);
    assert_eq!(a, NoiseParameters::new(-3, vec![1.0, 0.5, 0.0]));
    assert_eq!(NoiseParameters::new(0, vec![0.0]), NoiseParameters::new(0, vec![-0.0]), "primitive == on doubles");
    assert_ne!(NoiseParameters::new(0, vec![f64::NAN]), NoiseParameters::new(0, vec![f64::NAN]));
}
