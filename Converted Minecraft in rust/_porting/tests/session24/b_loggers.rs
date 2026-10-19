//! Sub-batch B: `AbstractSampleLogger`, `LocalSampleLogger`, `BandwidthDebugMonitor`, and
//! `CollectionTag.stream()` (the last member `CollectionTag` was missing).
//!
//! Groups: `SAMPLELOG`, `BANDWIDTH`, `CSTREAM`.

use std::sync::{Arc, Mutex};

use crate::sweep_probe::*;

use minecraft_rust::net::minecraft::nbt::ByteArrayTag::ByteArrayTag;
use minecraft_rust::net::minecraft::nbt::CollectionTag::CollectionTag;
use minecraft_rust::net::minecraft::nbt::IntArrayTag::IntArrayTag;
use minecraft_rust::net::minecraft::nbt::IntTag::IntTag;
use minecraft_rust::net::minecraft::nbt::ListTag::ListTag;
use minecraft_rust::net::minecraft::nbt::LongArrayTag::LongArrayTag;
use minecraft_rust::net::minecraft::nbt::LongTag::LongTag;
use minecraft_rust::net::minecraft::nbt::StringTag::StringTag;
use minecraft_rust::net::minecraft::nbt::Tag::Tag;
use minecraft_rust::net::minecraft::network::BandwidthDebugMonitor::BandwidthDebugMonitor;
use minecraft_rust::net::minecraft::util::debugchart::LocalSampleLogger::{
    self as local, LocalSampleLogger,
};
use minecraft_rust::net::minecraft::util::debugchart::SampleLogger::SampleLogger;
use minecraft_rust::net::minecraft::util::debugchart::SampleStorage::SampleStorage;

// ---------------------------------------------------------------- SAMPLELOG

/// The probe's `longs(csv)`.
fn longs(csv: &str) -> Vec<i64> {
    if csv.is_empty() {
        Vec::new()
    } else {
        csv.split(',').map(|v| v.parse().unwrap()).collect()
    }
}

/// The probe's `newLogger(desc)`.
fn new_logger(desc: &str) -> LocalSampleLogger {
    let parts: Vec<&str> = desc.split(':').collect();
    let dims: i32 = parts[1].parse().unwrap();
    if parts[0] == "1" {
        LocalSampleLogger::new(dims)
    } else {
        LocalSampleLogger::with_defaults(dims, longs(parts[2]))
    }
}

/// The probe's `runLogger`, op for op.
fn run_logger(lg: &mut LocalSampleLogger, dims: i32, script: &str) -> String {
    let mut out = Vec::new();
    for op in script.split(' ') {
        let (k, arg) = op.split_at(1);
        let r = match k {
            "l" => or_ex(|| lg.log_sample(arg.parse().unwrap()), |()| "ok".into()),
            "p" => {
                let (v, d) = arg.split_once('@').unwrap();
                let (v, d): (i64, i32) = (v.parse().unwrap(), d.parse().unwrap());
                or_ex(|| lg.log_partial_sample(v, d), |()| "ok".into())
            }
            "f" => or_ex(|| lg.log_full_sample(&longs(arg)), |()| "ok".into()),
            "n" => {
                let (n, base) = arg.split_once('@').unwrap();
                let (n, base): (i32, i64) = (n.parse().unwrap(), base.parse().unwrap());
                or_ex(
                    || {
                        for q in 0..n {
                            lg.log_sample(base + i64::from(q));
                        }
                    },
                    |()| "ok".into(),
                )
            }
            "g" => match arg.split_once('@') {
                Some((i, d)) => {
                    let (i, d): (i32, i32) = (i.parse().unwrap(), d.parse().unwrap());
                    or_ex(|| lg.get_i32_i32(i, d), |v| v.to_string())
                }
                None => {
                    let i: i32 = arg.parse().unwrap();
                    or_ex(|| lg.get_i32(i), |v| v.to_string())
                }
            },
            "z" => lg.size().to_string(),
            "c" => lg.capacity().to_string(),
            "r" => {
                lg.reset();
                "ok".into()
            }
            _ => {
                let held: Vec<String> = (0..lg.size())
                    .map(|s| {
                        let dims: Vec<String> = (0..dims)
                            .map(|d| lg.get_i32_i32(s, d).to_string())
                            .collect();
                        dims.join("/")
                    })
                    .collect();
                format!("[{}]", held.join(";"))
            }
        };
        out.push(r);
    }
    out.join(",")
}

#[test]
fn local_sample_logger_replays_the_jar_scripts() {
    let rows = rows("SAMPLELOG");
    assert_eq!(rows.len(), 12 * 7, "the probe's constructors x scripts");
    for r in rows {
        let dims: i32 = r[1].split(':').nth(1).unwrap().parse().unwrap();
        let got = match std::panic::catch_unwind(|| new_logger(r[1])) {
            Ok(mut lg) => run_logger(&mut lg, dims, r[2]),
            Err(_) => format!("EX:{}", esc(&panic_text(|| new_logger(r[1])))),
        };
        assert_eq!(got, r[3], "LocalSampleLogger {} / {}", r[1], r[2]);
    }
}

#[test]
fn local_sample_logger_capacity_constant() {
    // `CAPACITY` is 240 and `capacity()` reports it: the probe's `c` op reads 240 in every row
    // whose constructor succeeded, so the constant and the ring length must agree.
    assert_eq!(local::CAPACITY, 240);
    let cap_rows = rows("SAMPLELOG")
        .into_iter()
        .filter(|r| r[2].starts_with("z c ") && !r[3].starts_with("EX:"))
        .count();
    assert!(cap_rows > 0);
    assert_eq!(LocalSampleLogger::new(2).capacity(), local::CAPACITY);
}

// ---------------------------------------------------------------- BANDWIDTH

#[test]
fn bandwidth_monitor_logs_and_resets_its_count_each_tick() {
    let rows = rows("BANDWIDTH");
    assert_eq!(rows.len(), 13);
    for r in rows {
        let dims = i32f(r[1]);
        let logger = Arc::new(Mutex::new(LocalSampleLogger::new(dims)));
        let monitor = BandwidthDebugMonitor::new(Arc::clone(&logger));
        let read = if r[2] == "250x(r1000q rq t)" {
            for q in 0..250 {
                monitor.on_receive(q * 1000);
                monitor.on_receive(q);
                monitor.tick();
            }
            "z g0 g239 a"
        } else {
            for op in r[2].split(' ') {
                if op == "t" {
                    monitor.tick();
                } else {
                    monitor.on_receive(op[1..].parse().unwrap());
                }
            }
            "z a"
        };
        let got = run_logger(&mut logger.lock().unwrap(), dims, read);
        assert_eq!(got, r[3], "BandwidthDebugMonitor dims={} {}", r[1], r[2]);
    }
}

#[test]
fn bandwidth_monitor_is_shareable_across_threads_like_the_java_object() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<BandwidthDebugMonitor>();
}

// ---------------------------------------------------------------- CSTREAM

/// `[count, toStrings, ids]` as the probe's `streamRow` writes them.
fn stream_fields<C: CollectionTag + ?Sized>(t: &C) -> [String; 3] {
    let parts: Vec<String> = t.stream().map(|x| esc(&x.to_string())).collect();
    let ids: Vec<String> = t.stream().map(|x| x.get_id().to_string()).collect();
    [
        t.stream().count().to_string(),
        parts.join("|"),
        ids.join("|"),
    ]
}

#[test]
fn collection_tag_stream_matches_the_jar_for_all_four_collection_tags() {
    let rows = rows("CSTREAM");
    assert_eq!(rows.len(), 5 * 5);
    for r in rows {
        let v = longs(r[2]);
        let bytes: Vec<i8> = v.iter().map(|&x| x as i8).collect();
        let ints: Vec<i32> = v.iter().map(|&x| x as i32).collect();
        let got = match r[1] {
            "byte" => stream_fields(&ByteArrayTag::new(bytes)),
            "int" => stream_fields(&IntArrayTag::new(ints)),
            "long" => stream_fields(&LongArrayTag::new(v)),
            "list" => {
                let mut l = ListTag::new();
                for &i in &ints {
                    l.add(Tag::Int(IntTag::value_of(i)));
                }
                stream_fields(&l)
            }
            "mixed" => {
                let mut l = ListTag::new();
                for (k, (&i, &x)) in ints.iter().zip(&v).enumerate() {
                    l.add(match k % 3 {
                        0 => Tag::Int(IntTag::value_of(i)),
                        1 => Tag::String(StringTag::value_of(format!("s{x}"))),
                        _ => Tag::Long(LongTag::new(x)),
                    });
                }
                stream_fields(&l)
            }
            other => panic!("unknown CSTREAM kind {other}"),
        };
        assert_eq!(got[0], r[3], "count {} [{}]", r[1], r[2]);
        assert_eq!(got[1], r[4], "elements {} [{}]", r[1], r[2]);
        assert_eq!(got[2], r[5], "ids {} [{}]", r[1], r[2]);
    }
}
