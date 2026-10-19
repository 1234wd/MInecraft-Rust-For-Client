//! Sub-batch D: the eight datafix rename tables (`javacompat::guava::ImmutableStrMap`),
//! `IntersectionIterator`, `MergingUniqueIterator` (`javacompat::guava::AbstractIterator` and
//! `PeekingIterator`), `PotentialCalculator`, `VecDeltaCodec` (`java_lang::round_f64`), `Graph`,
//! `StrictQueue`.
//!
//! Groups: `RENAMEMAP`, `RENAMEMAPE`, `SEARCHIT`, `POTCALC`, `VECDELTA`, `GRAPH`, `STRICTQ`.

use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::sweep_probe::*;

use minecraft_rust::javacompat::guava::{ExactIter, ImmutableStrMap, JavaIterator};
use minecraft_rust::net::minecraft::client::searchtree::IntersectionIterator as intersection;
use minecraft_rust::net::minecraft::client::searchtree::MergingUniqueIterator as merging;
use minecraft_rust::net::minecraft::core::BlockPos::BlockPos;
use minecraft_rust::net::minecraft::network::protocol::game::VecDeltaCodec::{
    self as vdc, VecDeltaCodec,
};
use minecraft_rust::net::minecraft::util::datafix::fixes;
use minecraft_rust::net::minecraft::util::thread::StrictQueue::{
    FixedPriorityQueue, QueueStrictQueue, RunnableWithPriority, StrictQueue, Task,
};
use minecraft_rust::net::minecraft::util::Graph;
use minecraft_rust::net::minecraft::world::level::PotentialCalculator::PotentialCalculator;
use minecraft_rust::net::minecraft::world::phys::Vec3::Vec3;

fn java_double(s: &str) -> f64 {
    match s {
        "NaN" => f64::NAN,
        "Infinity" => f64::INFINITY,
        "-Infinity" => f64::NEG_INFINITY,
        _ => s.parse().unwrap_or_else(|_| panic!("not a double: {s}")),
    }
}

// ---------------------------------------------------------------- RENAMEMAP / RENAMEMAPE

fn rename_map(class: &str, field: &str) -> &'static ImmutableStrMap {
    match (class, field) {
        ("DyeItemRenameFix", "RENAMED_IDS") => &fixes::DyeItemRenameFix::RENAMED_IDS,
        ("RenamedCoralFansFix", "RENAMED_IDS") => &fixes::RenamedCoralFansFix::RENAMED_IDS,
        ("RecipesRenameningFix", "RECIPES") => &fixes::RecipesRenameningFix::RECIPES,
        ("RenamedCoralFix", "RENAMED_IDS") => &fixes::RenamedCoralFix::RENAMED_IDS,
        ("CavesAndCliffsRenames", "RENAMES") => &fixes::CavesAndCliffsRenames::RENAMES,
        ("StonecutterRecipeRenameFix", "RECIPE_RENAMES") => {
            &fixes::StonecutterRecipeRenameFix::RECIPE_RENAMES
        }
        ("StonecutterRecipeRenameFix", "ADVANCEMENT_RENAMES") => {
            &fixes::StonecutterRecipeRenameFix::ADVANCEMENT_RENAMES
        }
        ("BiomeFix", "BIOMES") => &fixes::BiomeFix::BIOMES,
        ("RecipesFix", "RECIPES") => &fixes::RecipesFix::RECIPES,
        other => panic!("unknown table {other:?}"),
    }
}

#[test]
fn datafix_rename_tables_match_the_jar_entry_for_entry() {
    let mut order = Vec::new();
    for r in rows("RENAMEMAP") {
        if r[1] == "equals" {
            let got: Vec<String> = order
                .iter()
                .map(|a: &&ImmutableStrMap| {
                    order
                        .iter()
                        .map(|b| if *a == *b { '1' } else { '0' })
                        .collect::<String>()
                })
                .collect();
            assert_eq!(got.join(","), r[2], "Map.equals matrix");
            continue;
        }
        let m = rename_map(r[1], r[2]);
        order.push(m);
        assert_eq!(
            r[3], "com.google.common.collect.ImmutableMap",
            "an ImmutableMap in the jar"
        );
        assert_eq!(m.size(), i32f(r[4]), "size {}.{}", r[1], r[2]);
        assert_eq!(m.is_empty(), boolf(r[5]));
        assert_eq!(m.java_hash_code(), i32f(r[6]), "hashCode {}.{}", r[1], r[2]);
        assert_eq!(esc(&m.to_string()), r[7], "toString {}.{}", r[1], r[2]);
        assert_eq!(m.get("minecraft:no_such_id").unwrap_or("null"), r[8]);
        assert_eq!(m.contains_key("minecraft:no_such_id"), boolf(r[9]));
        assert_eq!(m.contains_key(""), boolf(r[10]));
    }
    assert_eq!(order.len(), 9);
    let mut seen: HashMap<(String, String), usize> = HashMap::new();
    for r in rows("RENAMEMAPE") {
        let m = rename_map(r[1], r[2]);
        let k = i32f(r[3]) as usize;
        let n = seen
            .entry((r[1].to_string(), r[2].to_string()))
            .or_default();
        assert_eq!(k, *n, "entries in order");
        *n += 1;
        let (key, value) = m.entries()[k];
        assert_eq!(esc(key), r[4], "{}.{}[{k}] key", r[1], r[2]);
        assert_eq!(esc(value), r[5], "{}.{}[{k}] value", r[1], r[2]);
        assert_eq!(esc(m.get(key).unwrap()), r[6]);
        assert_eq!(m.contains_key(key), boolf(r[7]));
        assert_eq!(m.contains_value(value), boolf(r[8]));
        assert_eq!(
            m.contains_value(key),
            boolf(r[9]),
            "containsValue(key) {key}"
        );
    }
    for ((c, f), n) in seen {
        assert_eq!(
            rename_map(&c, &f).size() as usize,
            n,
            "every entry of {c}.{f} probed"
        );
    }
}

// ---------------------------------------------------------------- SEARCHIT

fn ints(csv: &str) -> Vec<i32> {
    if csv.is_empty() {
        Vec::new()
    } else {
        csv.split(',').map(|v| v.parse().unwrap()).collect()
    }
}

/// The probe's `counting(src, count)`: a Java list iterator that counts what it hands out.
struct Counting {
    inner: ExactIter<std::vec::IntoIter<i32>>,
    count: Rc<Cell<i32>>,
}

impl JavaIterator for Counting {
    type Item = i32;
    fn has_next(&mut self) -> bool {
        self.inner.has_next()
    }
    fn next_elem(&mut self) -> i32 {
        self.count.set(self.count.get() + 1);
        self.inner.next_elem()
    }
}

fn counting(csv: &str) -> (Counting, Rc<Cell<i32>>) {
    let count = Rc::new(Cell::new(0));
    (
        Counting {
            inner: ExactIter(ints(csv).into_iter()),
            count: Rc::clone(&count),
        },
        count,
    )
}

/// The probe's `comparator(kind, calls)`.
fn comparator(kind: &str, calls: Rc<Cell<i32>>) -> impl FnMut(&i32, &i32) -> Ordering {
    let kind = kind.to_string();
    move |x: &i32, y: &i32| {
        calls.set(calls.get() + 1);
        match kind.as_str() {
            "nat" => x.cmp(y),
            "abs" => x.abs().cmp(&y.abs()),
            _ => {
                if *x == 13 || *y == 13 {
                    panic!("java.lang.IllegalArgumentException: boom");
                }
                x.cmp(y)
            }
        }
    }
}

/// The probe's `drain(it)`: elements, or the exception and what `hasNext` does after it.
fn drain<I: JavaIterator<Item = i32>>(it: &mut I) -> String {
    let mut out: Vec<String> = Vec::new();
    loop {
        let step = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if it.has_next() {
                Some(it.next_elem())
            } else {
                None
            }
        }));
        match step {
            Ok(Some(v)) => out.push(v.to_string()),
            Ok(None) => break,
            Err(e) => {
                out.push(format!("EX:{}", esc(&payload(e))));
                let again =
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| it.has_next()));
                out.push(match again {
                    Ok(_) => "hasNext-ok".to_string(),
                    Err(e2) => format!("EX:{}", esc(&payload(e2))),
                });
                break;
            }
        }
    }
    out.join(",")
}

fn payload(e: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = e.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = e.downcast_ref::<&str>() {
        s.to_string()
    } else {
        panic!("panic payload is not a string")
    }
}

#[test]
fn search_tree_iterators_match_the_jar_including_what_they_pull() {
    let rows = rows("SEARCHIT");
    assert_eq!(rows.len(), 29);
    for r in rows {
        if r[1] == "peek" {
            let mut it = intersection::new(
                ExactIter(vec![1, 2, 4, 8].into_iter()),
                ExactIter(vec![2, 4, 6, 8].into_iter()),
                |a: &i32, b: &i32| a.cmp(b),
            );
            let mut o: Vec<String> = Vec::new();
            o.push(it.peek().unwrap().to_string());
            o.push(it.peek().unwrap().to_string());
            o.push(it.next_or_throw().to_string());
            o.push(it.has_next().to_string());
            o.push(it.peek().unwrap().to_string());
            o.push(it.next_or_throw().to_string());
            o.push(it.next_or_throw().to_string());
            o.push(it.has_next().to_string());
            o.push(or_ex(|| it.next_or_throw(), |v| v.to_string()));
            assert_eq!(o.join(","), r[2]);
            continue;
        }
        let (a, ca) = counting(r[2]);
        let (b, cb) = counting(r[3]);
        let calls = Rc::new(Cell::new(0));
        let cmp = comparator(r[4], Rc::clone(&calls));
        let (out, after) = if r[1] == "inter" {
            let mut it = intersection::new(a, b, cmp);
            let out = drain(&mut it);
            (
                out,
                or_ex(
                    || it.peek().copied(),
                    |v| v.map_or("null".into(), |v| v.to_string()),
                ),
            )
        } else {
            let mut it = merging::new(a, b, cmp);
            let out = drain(&mut it);
            (
                out,
                or_ex(
                    || it.peek().copied(),
                    |v| v.map_or("null".into(), |v| v.to_string()),
                ),
            )
        };
        let ctx = format!("{} [{}] [{}] {}", r[1], r[2], r[3], r[4]);
        assert_eq!(out, r[5], "elements {ctx}");
        assert_eq!(ca.get(), i32f(r[6]), "taken from the first source {ctx}");
        assert_eq!(cb.get(), i32f(r[7]), "taken from the second source {ctx}");
        assert_eq!(calls.get(), i32f(r[8]), "comparisons {ctx}");
        // Java's `peek()` past the end throws NoSuchElementException; the port's `peek` answers
        // `None` there. A FAILED iterator throws IllegalStateException from both.
        let want_after = if r[9] == "EX:java.util.NoSuchElementException" {
            "null"
        } else {
            r[9]
        };
        assert_eq!(after, want_after, "peek after the end {ctx}");
    }
}

#[test]
fn search_tree_iterators_are_rust_iterators_too() {
    // `FullTextSearchTree` feeds a MergingUniqueIterator into an IntersectionIterator and copies
    // the result with `ImmutableList.copyOf`: both as JavaIterator and as Iterator.
    let merged = merging::new(
        ExactIter(vec![1, 4, 9].into_iter()),
        ExactIter(vec![2, 4, 8].into_iter()),
        |a: &i32, b: &i32| a.cmp(b),
    );
    let both: Vec<i32> = intersection::new(
        ExactIter(vec![2, 3, 4, 8, 9].into_iter()),
        merged,
        |a: &i32, b: &i32| a.cmp(b),
    )
    .collect();
    assert_eq!(both, vec![2, 4, 8, 9]);
}

// ---------------------------------------------------------------- POTCALC

fn block(spec: &[&str]) -> BlockPos {
    BlockPos::new(
        spec[0].parse().unwrap(),
        spec[1].parse().unwrap(),
        spec[2].parse().unwrap(),
    )
}

#[test]
fn potential_calculator_matches_the_jar() {
    let rows = rows("POTCALC");
    assert_eq!(rows.len(), 9);
    for r in rows {
        let mut pc = PotentialCalculator::new();
        if !r[1].is_empty() {
            for ch in r[1].split(';') {
                let p: Vec<&str> = ch.split(',').collect();
                pc.add_charge(block(&p), java_double(p[3]));
            }
        }
        let got: Vec<String> = r[2]
            .split(' ')
            .map(|q| {
                let p: Vec<&str> = q.split(',').collect();
                dbits(pc.get_potential_energy_change(&block(&p), java_double(p[3])))
            })
            .collect();
        // Computed NaNs are compared after `nan_canon` (OPEN_QUESTIONS #33); every other value
        // bit for bit.
        assert_eq!(
            nan_canon(&got.join(",")),
            nan_canon(r[3]),
            "charges {}",
            r[1]
        );
    }
}

// ---------------------------------------------------------------- VECDELTA

fn vec(v: &Vec3) -> String {
    format!("{}/{}/{}", dbits(v.x), dbits(v.y), dbits(v.z))
}

#[test]
fn vec_delta_codec_matches_the_jar() {
    const BASES: [[f64; 3]; 5] = [
        [0.0, 0.0, 0.0],
        [10.3, 64.0, -5.75],
        [1.0e-5, -1.0e-5, 123456.789],
        [f64::NAN, f64::INFINITY, -0.0],
        [1.0e300, -1.0e300, 0.25],
    ];
    const DELTAS: [[i64; 3]; 7] = [
        [0, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 0, 4096],
        [100, -200, 300],
        [i64::MAX, i64::MIN, 1],
        [-1, -1, -1],
    ];
    let poss: [[f64; 3]; 6] = [
        [0.0, 0.0, 0.0],
        [10.3, 64.0, -5.75],
        [10.30001, 63.9999, -5.7501],
        [-1.0e6, 2.0e6, 0.5 / 4096.0],
        [f64::NAN, 1.0, -1.0],
        [f64::INFINITY, f64::NEG_INFINITY, 1.0e300],
    ];
    let mut codecs = 0;
    for r in rows("VECDELTA") {
        match r[1] {
            "encode" => assert_eq!(
                vdc::encode(f64bits(r[2]).pipe()),
                i64f(r[3]),
                "encode({})",
                r[2]
            ),
            "decode" => assert_eq!(dbits(vdc::decode_i64(i64f(r[2]))), r[3], "decode({})", r[2]),
            "codec" => {
                let bs = BASES[codecs];
                codecs += 1;
                let mut c = VecDeltaCodec::new();
                assert_eq!(vec(&c.get_base()), r[2], "initial base");
                c.set_base(Vec3::new(bs[0], bs[1], bs[2]));
                assert_eq!(vec(&c.get_base()), r[3]);
                let dd: Vec<String> = DELTAS
                    .iter()
                    .map(|d| vec(&c.decode_i64_i64_i64(d[0], d[1], d[2])))
                    .collect();
                assert_eq!(
                    nan_canon(&dd.join(";")),
                    nan_canon(r[4]),
                    "decode deltas, base {bs:?}"
                );
                let ee: Vec<String> = poss
                    .iter()
                    .map(|p| {
                        let v = Vec3::new(p[0], p[1], p[2]);
                        format!(
                            "{}/{}/{}/{}",
                            c.encode_x(&v),
                            c.encode_y(&v),
                            c.encode_z(&v),
                            vec(&c.delta(&v))
                        )
                    })
                    .collect();
                assert_eq!(
                    nan_canon(&ee.join(";")),
                    nan_canon(r[5]),
                    "encode/delta, base {bs:?}"
                );
            }
            other => panic!("unknown VECDELTA row {other}"),
        }
    }
    assert_eq!(codecs, BASES.len());
}

/// `f64::from_bits` of the probe's hex, as a method so the call above reads left to right.
trait Pipe {
    fn pipe(self) -> f64;
}

impl Pipe for u64 {
    fn pipe(self) -> f64 {
        f64::from_bits(self)
    }
}

// ---------------------------------------------------------------- GRAPH

#[test]
fn graph_depth_first_search_matches_the_jar() {
    let rows = rows("GRAPH");
    assert_eq!(rows.len(), 11);
    for r in rows {
        // node -> successors, in the probe's LinkedHashSet order
        let mut edges: HashMap<String, Vec<String>> = HashMap::new();
        if !r[1].is_empty() {
            for e in r[1].split(';') {
                let (k, v) = e.split_once(':').unwrap();
                let succ = if v.is_empty() {
                    Vec::new()
                } else {
                    v.split(',').map(str::to_string).collect()
                };
                edges.insert(k.to_string(), succ);
            }
        }
        let successors = |n: &String| edges.get(n).cloned().unwrap_or_default();
        let mut discovered = HashSet::new();
        let mut visiting = HashSet::new();
        let order = RefCell::new(Vec::<String>::new());
        let mut out = Vec::new();
        for start in r[2].split(' ') {
            let cyc = Graph::depth_first_search(
                &successors,
                &mut discovered,
                &mut visiting,
                &mut |n: &String| order.borrow_mut().push(n.clone()),
                &start.to_string(),
            );
            let d: BTreeSet<&String> = discovered.iter().collect();
            let v: BTreeSet<&String> = visiting.iter().collect();
            out.push(format!(
                "{start}={cyc}[{}]{{{}}}{{{}}}",
                order.borrow().join(","),
                d.into_iter().cloned().collect::<Vec<_>>().join(","),
                v.into_iter().cloned().collect::<Vec<_>>().join(",")
            ));
        }
        assert_eq!(out.join(" "), r[3], "edges {} from {}", r[1], r[2]);
    }
}

// ---------------------------------------------------------------- STRICTQ

/// The probe's `runQueue`. Tasks append their label to a shared log.
fn run_queue(
    push: &dyn Fn(i32, Task) -> bool,
    pop: &dyn Fn() -> Option<Task>,
    size: &dyn Fn() -> i32,
    is_empty: &dyn Fn() -> bool,
    script: &str,
) -> String {
    let log = Arc::new(Mutex::new(String::new()));
    let mut out = Vec::new();
    for op in script.split(' ') {
        let r = match &op[..1] {
            "p" => {
                let (p, label) = op[1..].split_once(':').unwrap();
                let (p, label) = (p.parse::<i32>().unwrap(), label.to_string());
                let log = Arc::clone(&log);
                let task: Task = Box::new(move || log.lock().unwrap().push_str(&label));
                or_ex(|| push(p, task), |ok| ok.to_string())
            }
            "o" => match pop() {
                None => "null".to_string(),
                Some(mut t) => {
                    log.lock().unwrap().clear();
                    t();
                    log.lock().unwrap().clone()
                }
            },
            "z" => size().to_string(),
            _ => is_empty().to_string(),
        };
        out.push(r);
    }
    out.join(",")
}

#[test]
fn strict_queues_match_the_jar() {
    let rows = rows("STRICTQ");
    assert_eq!(rows.len(), 7);
    for r in rows {
        match r[1] {
            "fixed" => {
                let n = i32f(r[2]);
                let got = match std::panic::catch_unwind(|| FixedPriorityQueue::new(n)) {
                    Err(_) => format!("EX:{}", esc(&panic_text(|| FixedPriorityQueue::new(n)))),
                    Ok(q) => run_queue(
                        &|p, t| q.push(RunnableWithPriority::new(p, t)),
                        &|| q.pop(),
                        &|| q.size(),
                        &|| q.is_empty(),
                        r[3],
                    ),
                };
                assert_eq!(got, r[4], "FixedPriorityQueue({n})");
            }
            "plain" => {
                let q = QueueStrictQueue::new();
                let got = run_queue(
                    &|_, t| q.push(t),
                    &|| q.pop(),
                    &|| q.size(),
                    &|| q.is_empty(),
                    r[3],
                );
                assert_eq!(got, r[4], "QueueStrictQueue");
            }
            "record" => {
                let log = Arc::new(Mutex::new(String::new()));
                let l2 = Arc::clone(&log);
                let mut rp =
                    RunnableWithPriority::new(7, Box::new(move || l2.lock().unwrap().push('r')));
                rp.run();
                rp.run();
                assert_eq!(rp.priority(), i32f(r[2]));
                assert_eq!(*log.lock().unwrap(), r[3]);
            }
            other => panic!("unknown STRICTQ row {other}"),
        }
    }
}

#[test]
fn strict_queues_are_shareable_across_threads() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FixedPriorityQueue>();
    assert_send_sync::<QueueStrictQueue>();
}
