//! Sub-batch A1: `CowFSCreationException`, `CowFSSymlinkException`, `FeatureFlag`,
//! `FeatureFlagSet`, `Frame`, `SimpleContainerData`, `DataSlot`, `DifficultyInstance`.
//!
//! Groups: `EXC24`, `EXC24FS`, `FFLAG`, `FFSET`, `FFSETOP`, `FRAME`, `FRAMEEQ`, `SCDATA`, `DSLOT`,
//! `DIFFINST`.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::sweep_probe::*;

use minecraft_rust::net::minecraft::commands::execution::Frame::{Frame, FrameControl};
use minecraft_rust::net::minecraft::commands::CommandResultCallback::{self as crc, CommandResultCallback};
use minecraft_rust::net::minecraft::util::filefix::virtualfilesystem::exception::CowFSCreationException::{
    self as creation_mod, CowFSCreationException,
};
use minecraft_rust::net::minecraft::util::filefix::virtualfilesystem::exception::CowFSSymlinkException::{
    self as symlink_mod, CowFSSymlinkException,
};
use minecraft_rust::net::minecraft::world::flag::FeatureFlag::FeatureFlag;
use minecraft_rust::net::minecraft::world::flag::FeatureFlagSet::{self as ffs, FeatureFlagSet};
use minecraft_rust::net::minecraft::world::flag::FeatureFlagUniverse::FeatureFlagUniverse;
use minecraft_rust::net::minecraft::world::inventory::ContainerData::ContainerData;
use minecraft_rust::net::minecraft::world::inventory::DataSlot::{self as data_slot, DataSlot};
use minecraft_rust::net::minecraft::world::inventory::SimpleContainerData::SimpleContainerData;
use minecraft_rust::net::minecraft::world::Difficulty::{self as difficulty, Difficulty};
use minecraft_rust::net::minecraft::world::DifficultyInstance::DifficultyInstance;

/// The probe's `MSGS`, in order.
const MSGS: [Option<&str>; 5] = [
    Some("boom"),
    Some(""),
    None,
    Some("tab\there"),
    Some("\u{e9}\u{4e2d}\u{1F600}"),
];

// ---------------------------------------------------------------- EXC24 / EXC24FS

struct Exc {
    class_name: &'static str,
    superclass: &'static str,
    message: Option<String>,
    to_string: String,
    file: Option<String>,
    other_file: Option<String>,
    reason: Option<String>,
}

fn build_exc(id: &str, msg: Option<&str>) -> Exc {
    match id {
        "CowFSCreationException" => {
            let e = CowFSCreationException::new(msg);
            Exc {
                class_name: creation_mod::CLASS_NAME,
                superclass: creation_mod::SUPERCLASS,
                message: e.get_message().map(str::to_string),
                to_string: e.to_string(),
                file: e.get_file().map(str::to_string),
                other_file: e.get_other_file().map(str::to_string),
                reason: e.get_reason().map(str::to_string),
            }
        }
        "CowFSSymlinkException" => {
            let e = CowFSSymlinkException::new(msg);
            Exc {
                class_name: symlink_mod::CLASS_NAME,
                superclass: symlink_mod::SUPERCLASS,
                message: e.get_message().map(str::to_string),
                to_string: e.to_string(),
                file: e.get_file().map(str::to_string),
                other_file: e.get_other_file().map(str::to_string),
                reason: e.get_reason().map(str::to_string),
            }
        }
        other => panic!("unknown exception id {other}"),
    }
}

#[test]
fn cowfs_creation_and_symlink_exceptions_match_the_jar() {
    for r in rows("EXC24") {
        let e = build_exc(r[1], MSGS[i32f(r[2]) as usize]);
        assert_eq!(e.class_name, r[3], "{r:?}");
        assert_eq!(e.superclass, r[4], "{r:?}");
        assert_eq!(r[5], "true", "both are IOExceptions: {r:?}");
        assert_eq!(e.message, unesc(r[6]), "getMessage {r:?}");
        assert_eq!(Some(e.to_string), unesc(r[7]), "toString {r:?}");
        // The panic a `throw` raises carries the same text.
        if let Some(m) = MSGS[i32f(r[2]) as usize] {
            let text = match r[1] {
                "CowFSCreationException" => panic_text(|| creation_mod::throw(m)),
                _ => panic_text(|| symlink_mod::throw(m)),
            };
            assert_eq!(Some(text), unesc(r[7]), "throw {r:?}");
        }
    }
    for r in rows("EXC24FS") {
        let e = build_exc(r[1], MSGS[i32f(r[2]) as usize]);
        assert_eq!(e.file, unesc(r[3]), "getFile {r:?}");
        assert_eq!(e.other_file, unesc(r[4]), "getOtherFile {r:?}");
        assert_eq!(e.reason, unesc(r[5]), "getReason {r:?}");
    }
}

// ---------------------------------------------------------------- FFLAG / FFSET / FFSETOP

/// `mask: <long>` out of a derived `Debug` string (the field is private, as in Java, where the
/// probe read it by reflection).
fn debug_i64(debug: &str, field: &str) -> i64 {
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
        .unwrap_or_else(|_| panic!("`{field}` is not a long in {debug}"))
}

#[test]
fn fastutil_hash_common_mix_matches_the_jar_in_all_64_bits() {
    use minecraft_rust::javacompat::fastutil::hash_common;
    for r in rows("HCMIX") {
        assert_eq!(
            hash_common::mix_i64(i64f(r[1])),
            i64f(r[2]),
            "HashCommon.mix({})",
            r[1]
        );
    }
}

#[test]
fn feature_flag_masks_use_javas_six_bit_shift() {
    let u = Arc::new(FeatureFlagUniverse::new("main"));
    for r in rows("FFLAG") {
        let flag = FeatureFlag::new(Arc::clone(&u), i32f(r[1]));
        assert_eq!(flag.mask, i64f(r[2]), "mask for bit {}", r[1]);
        assert_eq!(
            Arc::ptr_eq(&flag.universe, &u),
            boolf(r[3]),
            "universe {}",
            r[1]
        );
    }
}

/// The probe's universes and flag pool: A and B are both named "main", C is "other".
struct Pool {
    ua: Arc<FeatureFlagUniverse>,
    ub: Arc<FeatureFlagUniverse>,
    uc: Arc<FeatureFlagUniverse>,
    names: Vec<&'static str>,
    flags: HashMap<&'static str, FeatureFlag>,
}

impl Pool {
    fn new() -> Self {
        let ua = Arc::new(FeatureFlagUniverse::new("main"));
        let ub = Arc::new(FeatureFlagUniverse::new("main"));
        let uc = Arc::new(FeatureFlagUniverse::new("other"));
        let names = vec!["A0", "A1", "A5", "A63", "B0", "B1", "C3"];
        let mut flags = HashMap::new();
        for n in &names {
            let u = match &n[..1] {
                "A" => &ua,
                "B" => &ub,
                _ => &uc,
            };
            flags.insert(*n, FeatureFlag::new(Arc::clone(u), n[1..].parse().unwrap()));
        }
        Pool {
            ua,
            ub,
            uc,
            names,
            flags,
        }
    }

    fn universe(&self, letter: &str) -> &Arc<FeatureFlagUniverse> {
        match letter {
            "A" => &self.ua,
            "B" => &self.ub,
            _ => &self.uc,
        }
    }

    /// The probe's `build(desc)`.
    fn build(&self, desc: &str) -> FeatureFlagSet {
        if desc == "E" {
            return ffs::of();
        }
        if let Some(n) = desc.strip_prefix("1:") {
            return ffs::of_flag(&self.flags[n]);
        }
        if let Some(list) = desc.strip_prefix("N:") {
            let names: Vec<&str> = list.split(',').collect();
            let rest: Vec<&FeatureFlag> = names[1..].iter().map(|n| &self.flags[n]).collect();
            return ffs::of_flags(&self.flags[names[0]], &rest);
        }
        let parts: Vec<&str> = desc.splitn(3, ':').collect();
        let list: Vec<&FeatureFlag> = if parts[2].is_empty() {
            Vec::new()
        } else {
            parts[2].split(',').map(|n| &self.flags[n]).collect()
        };
        ffs::create(self.universe(parts[1]), &list)
    }

    /// The probe's `state(s)`. The universe label is the universe of a contained pool flag (every
    /// non-empty set here is built from pool flags), and `null` for an empty one.
    fn state(&self, s: &FeatureFlagSet) -> String {
        let contains: String = self
            .names
            .iter()
            .map(|n| if s.contains(&self.flags[n]) { '1' } else { '0' })
            .collect();
        let label = self
            .names
            .iter()
            .find(|n| s.contains(&self.flags[*n]))
            .map_or("null", |n| &n[..1]);
        format!(
            "{label}|{}|{}|{}|{}|{contains}",
            debug_i64(&format!("{s:?}"), "mask"),
            s.java_hash_code(),
            esc(&s.to_string()),
            if s.is_empty() { '1' } else { '0' }
        )
    }
}

#[test]
fn feature_flag_sets_match_the_jar() {
    let pool = Pool::new();
    for r in rows("FFSET") {
        let got = or_ex(|| pool.build(r[1]), |s| pool.state(&s));
        assert_eq!(got, r[2], "FeatureFlagSet {}", r[1]);
    }
}

#[test]
fn feature_flag_set_operations_match_the_jar() {
    let pool = Pool::new();
    for r in rows("FFSETOP") {
        let x = pool.build(r[1]);
        let y = pool.build(r[2]);
        assert_eq!(
            x.is_subset_of(&y),
            boolf(r[3]),
            "{} isSubsetOf {}",
            r[1],
            r[2]
        );
        assert_eq!(
            x.intersects(&y),
            boolf(r[4]),
            "{} intersects {}",
            r[1],
            r[2]
        );
        assert_eq!(x == y, boolf(r[5]), "{} equals {}", r[1], r[2]);
        assert_eq!(
            or_ex(|| x.join(&y), |s| pool.state(&s)),
            r[6],
            "{} join {}",
            r[1],
            r[2]
        );
        assert_eq!(
            or_ex(|| x.subtract(&y), |s| pool.state(&s)),
            r[7],
            "{} subtract {}",
            r[1],
            r[2]
        );
    }
}

// ---------------------------------------------------------------- FRAME / FRAMEEQ

fn recorder(log: &Arc<Mutex<String>>) -> Arc<dyn CommandResultCallback> {
    let log = Arc::clone(log);
    Arc::new(move |success: bool, result: i32| {
        let mut l = log.lock().unwrap();
        l.push(if success { 'S' } else { 'F' });
        l.push_str(&result.to_string());
        l.push(' ');
    })
}

fn discarder(log: &Arc<Mutex<String>>, text: &'static str) -> Arc<dyn FrameControl> {
    let log = Arc::clone(log);
    Arc::new(move || log.lock().unwrap().push_str(text))
}

fn run_frame(frame: &Frame, script: &str) {
    for op in script.split(' ') {
        match &op[..1] {
            "S" => frame.return_success(op[1..].parse().unwrap()),
            "F" => frame.return_failure(),
            _ => frame.discard(),
        }
    }
}

#[test]
fn frame_delegates_to_its_callback_and_control_as_the_jar_does() {
    let log = Arc::new(Mutex::new(String::new()));
    let cb_a = recorder(&log);
    let fc_a = discarder(&log, "D ");
    for r in rows("FRAME") {
        log.lock().unwrap().clear();
        let frame = if r[1] == "EMPTY" {
            Frame::new(3, Arc::clone(&*crc::EMPTY), Arc::clone(&fc_a))
        } else {
            Frame::new(i32f(r[1]), Arc::clone(&cb_a), Arc::clone(&fc_a))
        };
        run_frame(&frame, r[2]);
        assert_eq!(
            log.lock().unwrap().trim(),
            s(r[3]),
            "callback log for {r:?}"
        );
        assert_eq!(frame.depth(), i32f(r[4]));
        assert_eq!(
            Arc::ptr_eq(frame.return_value_consumer(), &cb_a),
            boolf(r[5])
        );
        assert_eq!(Arc::ptr_eq(frame.frame_control(), &fc_a), boolf(r[6]));
    }
}

#[test]
fn frame_equality_is_by_value_for_depth_and_by_reference_for_the_rest() {
    let log = Arc::new(Mutex::new(String::new()));
    let cb_a = recorder(&log);
    let cb_b = recorder(&log);
    let fc_a = discarder(&log, "D ");
    let fc_b = discarder(&log, "d");
    let f1 = Frame::new(1, Arc::clone(&cb_a), Arc::clone(&fc_a));
    for r in rows("FRAMEEQ") {
        let other = match r[1] {
            "self" => f1.clone(),
            "same-components" => Frame::new(1, Arc::clone(&cb_a), Arc::clone(&fc_a)),
            "other-depth" => Frame::new(2, Arc::clone(&cb_a), Arc::clone(&fc_a)),
            "other-callback" => Frame::new(1, Arc::clone(&cb_b), Arc::clone(&fc_a)),
            "other-control" => Frame::new(1, Arc::clone(&cb_a), Arc::clone(&fc_b)),
            "both-other" => Frame::new(1, Arc::clone(&cb_b), Arc::clone(&fc_b)),
            label => panic!("unknown FRAMEEQ case {label}"),
        };
        assert_eq!(f1 == other, boolf(r[2]), "{}", r[1]);
    }
}

// ---------------------------------------------------------------- SCDATA

fn run_container(data: &mut dyn ContainerData, script: &str) -> String {
    let mut out = Vec::new();
    for op in script.split(' ') {
        let r = if op == "c" {
            data.get_count().to_string()
        } else if let Some(i) = op.strip_prefix('g') {
            let i: i32 = i.parse().unwrap();
            or_ex(|| data.get(i), |v| v.to_string())
        } else {
            let (k, v) = op[1..].split_once('=').unwrap();
            let (k, v): (i32, i32) = (k.parse().unwrap(), v.parse().unwrap());
            or_ex(|| data.set(k, v), |()| "ok".to_string())
        };
        out.push(r);
    }
    out.join(",")
}

#[test]
fn simple_container_data_matches_the_jar_including_array_errors() {
    for r in rows("SCDATA") {
        let count = i32f(r[1]);
        let got = match std::panic::catch_unwind(|| SimpleContainerData::new(count)) {
            Ok(mut data) => run_container(&mut data, r[2]),
            Err(_) => format!(
                "EX:{}",
                esc(&panic_text(|| SimpleContainerData::new(count)))
            ),
        };
        assert_eq!(got, r[3], "SimpleContainerData({count})");
    }
}

// ---------------------------------------------------------------- DSLOT

fn run_slot(slot: &mut DataSlot, mut backdoor: impl FnMut(i32, i32), script: &str) -> String {
    let mut out = Vec::new();
    for op in script.split(' ') {
        let r = if op == "g" {
            or_ex(|| slot.get(), |v| v.to_string())
        } else if op == "k" {
            or_ex(|| slot.check_and_clear_update_flag(), |v| v.to_string())
        } else if let Some(v) = op.strip_prefix('s') {
            let v: i32 = v.parse().unwrap();
            or_ex(|| slot.set(v), |()| "ok".to_string())
        } else {
            let (k, v) = op[1..].split_once('=').unwrap();
            backdoor(k.parse().unwrap(), v.parse().unwrap());
            "ok".to_string()
        };
        out.push(r);
    }
    out.join(",")
}

/// Java's `Arrays.toString(int[])`.
fn arrays_to_string(v: &[i32]) -> String {
    let parts: Vec<String> = v.iter().map(i32::to_string).collect();
    format!("[{}]", parts.join(", "))
}

#[test]
fn data_slots_see_writes_through_their_shared_storage_as_in_the_jar() {
    for r in rows("DSLOT") {
        let script = r[3];
        let got = match r[1] {
            "standalone" => run_slot(&mut data_slot::standalone(), |_, _| {}, script),
            "shared" => {
                let storage = Rc::new(RefCell::new(vec![0i32; 3]));
                let mut slot = data_slot::shared(Rc::clone(&storage), i32f(r[2]));
                let back = Rc::clone(&storage);
                let out = run_slot(
                    &mut slot,
                    move |k, v| back.borrow_mut()[k as usize] = v,
                    script,
                );
                format!("{out}|{}", arrays_to_string(&storage.borrow()))
            }
            "container" => {
                let data = Rc::new(RefCell::new(SimpleContainerData::new(3)));
                let shared: Rc<RefCell<dyn ContainerData>> = data.clone();
                let mut slot = data_slot::for_container(shared, i32f(r[2]));
                let back = Rc::clone(&data);
                let out = run_slot(&mut slot, move |k, v| back.borrow_mut().set(k, v), script);
                let d = data.borrow();
                format!("{out}|{},{},{}", d.get(0), d.get(1), d.get(2))
            }
            other => panic!("unknown DataSlot factory {other}"),
        };
        assert_eq!(got, r[4], "DataSlot.{}({})", r[1], r[2]);
    }
}

// ---------------------------------------------------------------- DIFFINST

/// The probe's `thresholds` for `isHarderThan`, in order.
const THRESHOLDS: [f32; 8] = [-1.0, 0.0, 1.0, 1.5, 2.25, 3.0, 4.5, f32::NAN];

fn difficulty_named(name: &str) -> Difficulty {
    *difficulty::VALUES
        .iter()
        .find(|d| d.to_string() == name)
        .unwrap_or_else(|| panic!("no Difficulty {name}"))
}

#[test]
fn difficulty_instance_matches_the_jar_over_the_whole_grid() {
    let rows = rows("DIFFINST");
    assert_eq!(rows.len(), 4 * 12 * 8 * 11, "the probe's grid");
    for r in rows {
        let base = difficulty_named(r[1]);
        let di = DifficultyInstance::new(base, i64f(r[2]), i64f(r[3]), f32f(r[4]));
        let ctx = format!("{} t={} local={} moon={}", r[1], r[2], r[3], r[4]);
        assert_eq!(di.get_difficulty().to_string(), r[5], "{ctx}");
        assert_eq!(
            nan_canon(&fbits(di.get_effective_difficulty())),
            nan_canon(r[6]),
            "effective {ctx}"
        );
        assert_eq!(di.is_hard(), boolf(r[7]), "isHard {ctx}");
        let harder: String = THRESHOLDS
            .iter()
            .map(|&t| if di.is_harder_than(t) { '1' } else { '0' })
            .collect();
        assert_eq!(harder, r[8], "isHarderThan {ctx}");
        assert_eq!(
            nan_canon(&fbits(di.get_special_multiplier())),
            nan_canon(r[9]),
            "special {ctx}"
        );
    }
}
