//! Sub-batch B: `CompoundTag` and `ListTag` over session 19's tree oracle.
//!
//! * `GEN`: 120 pseudo-random trees from `NbtProbe.genCompound(new Random(seed), 0)`. The
//!   generator is mirrored below call for call (same `java.util.Random` draws, same order), so a
//!   tree built here is the tree the probe built -- the `S` (sorted `toString`) row would show the
//!   first divergence.
//! * `CASE`: the probe's hand-built trees, rebuilt here statement for statement.
//! * `CASEW`, `CASED`, `IDENT`, `LGET`, `CGET`: the special cases around them.
//!
//! Per tree (`treeRows`): `W` the `NbtIo.write` bytes, `S` toString, `Z` sizeInBytes, `H`
//! hashCode, `CW` the bytes of `copy()`, `SW` of `shallowCopy()`, `RW`/`RU`/`RE` the bytes, quota
//! use and `equals` of `NbtIo.read` of `W`, and with traces `TA<p>` (`acceptAsRoot`) and `TP<p>`
//! (`NbtIo.parse`) under three visitor policies.
//!
//! NOT checked, and listed in [`UNCHECKED_SUB_ROWS`]: `PW`/`PU`, `CF<n>` and `SF<n>`, which run
//! `CollectToTag`/`CollectFields`/`SkipFields` from `net.minecraft.nbt.visitors` -- not ported.

use std::collections::HashMap;

use crate::b_support::*;
use crate::nbt_probe::*;

use minecraft_rust::javacompat::java_io::{DataOutput, DataOutputStream};
use minecraft_rust::javacompat::java_lang::float_to_string::{double_to_string, float_to_string};
use minecraft_rust::javacompat::java_random::JavaRandom;
use minecraft_rust::net::minecraft::nbt::ByteArrayTag::ByteArrayTag;
use minecraft_rust::net::minecraft::nbt::ByteTag::ByteTag;
use minecraft_rust::net::minecraft::nbt::CompoundTag::CompoundTag;
use minecraft_rust::net::minecraft::nbt::DoubleTag::DoubleTag;
use minecraft_rust::net::minecraft::nbt::FloatTag::FloatTag;
use minecraft_rust::net::minecraft::nbt::IntArrayTag::IntArrayTag;
use minecraft_rust::net::minecraft::nbt::IntTag::IntTag;
use minecraft_rust::net::minecraft::nbt::ListTag::ListTag;
use minecraft_rust::net::minecraft::nbt::LongArrayTag::LongArrayTag;
use minecraft_rust::net::minecraft::nbt::LongTag::LongTag;
use minecraft_rust::net::minecraft::nbt::NbtAccounter::NbtAccounter;
use minecraft_rust::net::minecraft::nbt::ShortTag::ShortTag;
use minecraft_rust::net::minecraft::nbt::StringTag::StringTag;
use minecraft_rust::net::minecraft::nbt::Tag::Tag;

// ------------------------------------------------------------------ the generator, mirrored

/// `NbtProbe.KEY_POOL`, code point for code point (read off the Java source's UTF-8).
const KEY_POOL: &[&str] = &[
    "",
    "a",
    "b",
    "id",
    "Count",
    "x",
    "y",
    "z",
    "true",
    "FALSE",
    "tRuE",
    "false1",
    "key with space",
    "quote\"d",
    "apo's",
    "both\"'",
    "back\\slash",
    "1abc",
    "_under",
    ".dot",
    "a+b-c",
    "-lead",
    "+plus",
    "\u{e9}",
    "\u{4e2d}\u{6587}",
    "\u{1f600}",
    "Aa",
    "BB",
    "AaAa",
    "BBBB",
    "AaBB",
    "BBAa",
    "minecraft:stone",
    "A",
    "Z",
    "zz",
    "\u{ff}",
    "\u{fffd}",
    "\u{e000}",
    "a\u{80}b",
    "\u{7ff}\u{800}",
];

/// `NbtProbe.UNI_POOL`.
const UNI_POOL: &[&str] = &[
    "a",
    "Z",
    "0",
    " ",
    "\"",
    "'",
    "\\",
    "\u{e9}",
    "\u{ff}",
    "\u{100}",
    "\u{7ff}",
    "\u{800}",
    "\u{4e2d}",
    "\u{fffd}",
    "\u{e000}",
    "\u{ffff}",
    "\u{1f600}",
    "\u{10000}",
    "\u{10ffff}",
    "\u{7f}",
    "\u{80}",
];

/// `NbtProbe.IDENT`.
const IDENT: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_.+-";

/// `NbtProbe.SPECIAL_F`, its literals character for character (`2.14748365E9` rounds to the
/// same `f32` as clippy's shorter spelling; keeping the Java text keeps the mirror checkable).
#[allow(clippy::excessive_precision)]
fn special_f() -> [f32; 24] {
    [
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        0.1,
        1e7,
        9999999.0,
        1e-3,
        1e-4,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::from_bits(1),
        f32::MAX,
        f32::MIN_POSITIVE,
        1.0E10,
        -1.5,
        255.9,
        2.14748365E9,
        -2.14748365E9,
        65535.7,
        128.5,
        -129.5,
    ]
}

/// `NbtProbe.SPECIAL_D`.
fn special_d() -> [f64; 27] {
    [
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        0.1,
        1e7,
        9999999.0,
        1e-3,
        1e-4,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(1),
        f64::MAX,
        f64::MIN_POSITIVE,
        1.0E10,
        -1.5,
        255.9,
        2.147483648E9,
        -2.147483649E9,
        9.3E18,
        -9.3E18,
        65535.7,
        128.5,
        -129.5,
        1.0E-300,
    ]
}

fn pick(r: &mut JavaRandom, n: usize) -> usize {
    r.next_int_bounded(n as i32) as usize
}

fn gen_key(r: &mut JavaRandom) -> String {
    match r.next_int_bounded(4) {
        0 => KEY_POOL[pick(r, KEY_POOL.len())].to_string(),
        1 => {
            let len = 1 + r.next_int_bounded(12);
            (0..len)
                .map(|_| IDENT[pick(r, IDENT.len())] as char)
                .collect()
        }
        2 => {
            let len = r.next_int_bounded(6);
            (0..len)
                .map(|_| UNI_POOL[pick(r, UNI_POOL.len())])
                .collect()
        }
        _ => format!("k{}", r.next_int_bounded(1000)),
    }
}

fn gen_string(r: &mut JavaRandom) -> String {
    match r.next_int_bounded(3) {
        0 => gen_key(r),
        1 => {
            let len = r.next_int_bounded(40);
            (0..len)
                .map(|_| UNI_POOL[pick(r, UNI_POOL.len())])
                .collect()
        }
        _ => String::new(),
    }
}

fn gen_float(r: &mut JavaRandom) -> f32 {
    match r.next_int_bounded(3) {
        0 => special_f()[pick(r, 24)],
        1 => f32::from_bits(r.next_int() as u32),
        _ => r.next_float() * 2000.0 - 1000.0,
    }
}

fn gen_double(r: &mut JavaRandom) -> f64 {
    match r.next_int_bounded(3) {
        0 => special_d()[pick(r, 27)],
        1 => f64::from_bits(r.next_long() as u64),
        _ => r.next_double() * 2.0e6 - 1.0e6,
    }
}

fn gen_tag_of_type(r: &mut JavaRandom, t: i32, depth: i32) -> Tag {
    match t {
        0 => ByteTag::value_of(r.next_int() as i8).into_tag(),
        1 => ShortTag::value_of(r.next_int() as i16).into_tag(),
        2 => {
            let big = r.next_boolean();
            let v = if big {
                r.next_int()
            } else {
                r.next_int_bounded(2000) - 500
            };
            IntTag::value_of(v).into_tag()
        }
        3 => {
            let big = r.next_boolean();
            let v = if big {
                r.next_long()
            } else {
                (r.next_int_bounded(2000) - 500) as i64
            };
            LongTag::value_of(v).into_tag()
        }
        4 => FloatTag::value_of(gen_float(r)).into_tag(),
        5 => DoubleTag::value_of(gen_double(r)).into_tag(),
        6 => StringTag::value_of(gen_string(r)).into_tag(),
        7 => {
            let n = r.next_int_bounded(10);
            ByteArrayTag::new((0..n).map(|_| r.next_int() as i8).collect()).into_tag()
        }
        8 => {
            let n = r.next_int_bounded(10);
            IntArrayTag::new((0..n).map(|_| r.next_int()).collect()).into_tag()
        }
        9 => {
            let n = r.next_int_bounded(10);
            LongArrayTag::new((0..n).map(|_| r.next_long()).collect()).into_tag()
        }
        10 => gen_list(r, depth).into_tag(),
        11 => gen_compound(r, depth).into_tag(),
        _ => gen_compound_list(r, depth).into_tag(),
    }
}

fn gen_tag(r: &mut JavaRandom, depth: i32) -> Tag {
    let t = r.next_int_bounded(if depth >= 4 { 10 } else { 13 });
    gen_tag_of_type(r, t, depth)
}

fn gen_list(r: &mut JavaRandom, depth: i32) -> ListTag {
    let mut l = ListTag::new();
    let n = r.next_int_bounded(6);
    let homogeneous = r.next_int_bounded(3) != 0;
    let fixed_type = r.next_int_bounded(if depth >= 4 { 10 } else { 12 });
    for _ in 0..n {
        let t = if homogeneous {
            gen_tag_of_type(r, fixed_type, depth + 1)
        } else {
            gen_tag(r, depth + 1)
        };
        l.add(t);
    }
    l
}

fn gen_compound_list(r: &mut JavaRandom, depth: i32) -> ListTag {
    let mut l = ListTag::new();
    let n = r.next_int_bounded(5);
    for _ in 0..n {
        let k = r.next_int_bounded(4);
        if k == 0 {
            let mut w = CompoundTag::new();
            w.put("", gen_tag(r, depth + 2));
            l.add(w.into_tag());
        } else if k == 3 {
            l.add(gen_tag(r, depth + 1));
        } else {
            l.add(gen_compound(r, depth + 1).into_tag());
        }
    }
    l
}

fn gen_compound(r: &mut JavaRandom, depth: i32) -> CompoundTag {
    let mut c = CompoundTag::new();
    let n = r.next_int_bounded(if depth == 0 { 40 } else { 8 });
    let mut inserted: Vec<String> = Vec::new();
    for _ in 0..n {
        let key = gen_key(r);
        let value = gen_tag(r, depth + 1);
        c.put(&key, value);
        inserted.push(key);
    }
    let removes = if r.next_int_bounded(4) == 0 {
        r.next_int_bounded(inserted.len() as i32 + 1)
    } else {
        0
    };
    for _ in 0..removes {
        let k = inserted[pick(r, inserted.len())].clone();
        c.remove(&k);
    }
    c
}

/// `NbtProbe.colliding(n)`: same-length concatenations of "Aa"/"BB" share a `String.hashCode`.
fn colliding(n: usize) -> Vec<String> {
    (0..n)
        .map(|i| {
            (0..6)
                .rev()
                .map(|bit| if (i >> bit) & 1 == 0 { "Aa" } else { "BB" })
                .collect()
        })
        .collect()
}

// ------------------------------------------------------------------ treeRows

/// The sub-rows of a tree that need the unported `visitors` package.
pub const UNCHECKED_SUB_ROWS: &[&str] = &["PW", "PU", "CF0", "SF0", "CF1", "SF1", "CF2", "SF2"];

/// The sub-rows that ARE checked.
const CHECKED_SUB_ROWS: &[&str] = &[
    "W", "S", "Z", "H", "CW", "SW", "RW", "RU", "RE", "TA0", "TP0", "TA1", "TP1", "TA2", "TP2",
];

/// Checks one tree against its `kind`/`id` rows; returns how many rows it compared.
fn check_tree(
    rows: &HashMap<(String, String), Vec<&'static str>>,
    kind: &str,
    id: &str,
    t: &CompoundTag,
    traces: bool,
) -> usize {
    let get = |sub: &str| -> &Vec<&'static str> {
        rows.get(&(id.to_string(), sub.to_string()))
            .unwrap_or_else(|| panic!("no {kind} {id} {sub} row"))
    };
    let field = |sub: &str| get(sub)[3];
    let w = nbt_write(t);
    assert_eq!(hex(&w), field("W"), "{kind} {id}: write bytes");
    assert_eq!(esc(&t.to_string()), field("S"), "{kind} {id}: toString");
    assert_eq!(
        t.size_in_bytes().to_string(),
        field("Z"),
        "{kind} {id}: sizeInBytes"
    );
    assert_eq!(
        t.java_hash_code().expect("no EndTag").to_string(),
        field("H"),
        "{kind} {id}: hashCode"
    );
    assert_eq!(
        hex(&nbt_write(&t.copy())),
        field("CW"),
        "{kind} {id}: copy() bytes"
    );
    assert_eq!(
        hex(&nbt_write(&t.shallow_copy())),
        field("SW"),
        "{kind} {id}: shallowCopy() bytes"
    );
    let mut acc = NbtAccounter::unlimited_heap();
    let back = nbt_read(&w, &mut acc);
    let back = back.as_compound().expect("a compound root");
    assert_eq!(
        hex(&nbt_write(back)),
        field("RW"),
        "{kind} {id}: re-written bytes"
    );
    assert_eq!(
        acc.get_usage().to_string(),
        field("RU"),
        "{kind} {id}: quota used by load"
    );
    assert_eq!(
        (back == t).to_string(),
        field("RE"),
        "{kind} {id}: read-back equals"
    );
    let mut checked = 9;
    if traces {
        for policy in 0..3 {
            let mut ta = Trace::for_policy(policy, id);
            Tag::Compound(t.clone()).accept_as_root(&mut ta);
            assert_eq!(
                ta.log,
                field(&format!("TA{policy}")),
                "{kind} {id}: acceptAsRoot, policy {policy}"
            );
            let mut tp = Trace::for_policy(policy, id);
            let res = nbt_parse(&w, &mut tp, &mut NbtAccounter::unlimited_heap());
            let mut got = tp.log.clone();
            if let Err(e) = res {
                got.push('!');
                got.push_str(minecraft_rust::javacompat::java_io::java_class_name(&e));
            }
            assert_eq!(
                got,
                field(&format!("TP{policy}")),
                "{kind} {id}: NbtIo.parse, policy {policy}"
            );
            checked += 2;
        }
    }
    checked
}

/// `kind`'s rows keyed by (id, sub-row), after checking that every sub-row is known.
fn tree_rows(kind: &str) -> HashMap<(String, String), Vec<&'static str>> {
    let mut out = HashMap::new();
    for r in rows(kind) {
        assert!(
            CHECKED_SUB_ROWS.contains(&r[2]) || UNCHECKED_SUB_ROWS.contains(&r[2]),
            "{kind} {}: unknown sub-row {}",
            r[1],
            r[2]
        );
        out.insert((r[1].to_string(), r[2].to_string()), r);
    }
    out
}

fn checked_rows_in(rows: &HashMap<(String, String), Vec<&'static str>>) -> usize {
    rows.keys()
        .filter(|(_, sub)| CHECKED_SUB_ROWS.contains(&sub.as_str()))
        .count()
}

#[test]
fn generated_trees_match_the_jar() {
    let rows = tree_rows("GEN");
    let mut checked = 0;
    for seed in 0..120 {
        let t = gen_compound(&mut JavaRandom::new(seed), 0);
        checked += check_tree(&rows, "GEN", &seed.to_string(), &t, seed < 60);
    }
    assert_eq!(
        checked,
        checked_rows_in(&rows),
        "every checkable GEN row was compared"
    );
}

fn compound(entries: &[(&str, Tag)]) -> CompoundTag {
    let mut c = CompoundTag::new();
    for (k, v) in entries {
        c.put(k, v.clone());
    }
    c
}

fn list(tags: Vec<Tag>) -> ListTag {
    let mut l = ListTag::new();
    for t in tags {
        l.add(t);
    }
    l
}

fn int(v: i32) -> Tag {
    IntTag::value_of(v).into_tag()
}

/// `NbtProbe.caseSection`'s nested_deep tree: `d<i>` is a one-compound list, `v` is `i`.
fn nested_deep(i: i32) -> CompoundTag {
    let mut cur = CompoundTag::new();
    if i < 10 {
        let next = nested_deep(i + 1);
        cur.put(&format!("d{i}"), list(vec![next.into_tag()]).into_tag());
        cur.put_int("v", i);
    }
    cur
}

#[test]
fn hand_built_trees_match_the_jar() {
    let rows = tree_rows("CASE");
    let mut checked = 0;
    let mut check = |id: &str, t: &CompoundTag, traces: bool| {
        checked += check_tree(&rows, "CASE", id, t, traces);
    };

    check("empty", &CompoundTag::new(), true);

    let mut all = CompoundTag::new();
    all.put_byte("b", 1);
    all.put_short("s", 2);
    all.put_int("i", 3);
    all.put_long("l", 4);
    all.put_float("f", 5.5);
    all.put_double("d", 6.25);
    all.put_string("str", "hello");
    all.put_byte_array("ba", vec![1, -2, 3]);
    all.put_int_array("ia", vec![1, -2, i32::MAX, i32::MIN]);
    all.put_long_array("la", vec![1, -2, i64::MAX, i64::MIN]);
    all.put("list", list(vec![int(1), int(2)]).into_tag());
    let mut inner = CompoundTag::new();
    inner.put_string("name", "inner");
    all.put("cmp", inner.into_tag());
    all.put("emptylist", ListTag::new().into_tag());
    all.put_boolean("bool", true);
    check("all_types", &all, true);

    let mut many = CompoundTag::new();
    for i in 0..200 {
        many.put_int(&format!("k{i}"), i);
    }
    check("many_keys", &many, false);
    for i in 0..150 {
        many.remove(&format!("k{i}"));
    }
    check("many_keys_removed", &many, false);

    let mut uni = CompoundTag::new();
    uni.put_string("\u{e9}", "\u{4e2d}\u{6587}");
    uni.put_string("\u{1f600}", "x\u{1f600}y");
    uni.put_string("max", "\u{ffff}\u{800}\u{7ff}\u{80}\u{7f}");
    uni.put_string("\u{fffd}", "\u{e000}");
    check("unicode", &uni, true);

    let mut hetero = CompoundTag::new();
    let hl = list(vec![
        int(1),
        StringTag::value_of("a").into_tag(),
        compound(&[("x", int(1))]).into_tag(),
        compound(&[("", ByteTag::value_of(5).into_tag())]).into_tag(),
        ListTag::new().into_tag(),
    ]);
    hetero.put("hetero", hl.into_tag());
    let mut w2 = CompoundTag::new();
    w2.put_int("", 2);
    w2.put_int("other", 3);
    let wl = list(vec![compound(&[("", int(1))]).into_tag(), w2.into_tag()]);
    hetero.put("wrappers", wl.into_tag());
    check("hetero_list", &hetero, true);

    check("nested_deep", &nested_deep(0), true);

    let mut nz = CompoundTag::new();
    nz.put_float("f", -0.0);
    nz.put_double("d", -0.0);
    nz.put("f2", FloatTag::new(-0.0).into_tag());
    nz.put("d2", DoubleTag::new(-0.0).into_tag());
    nz.put("nan", FloatTag::new(f32::from_bits(0x7f80_0001)).into_tag());
    nz.put(
        "dnan",
        DoubleTag::new(f64::from_bits(0xfff0_0000_0000_0001)).into_tag(),
    );
    check("neg_zero_nan", &nz, true);

    let mut sel_a = CompoundTag::new();
    sel_a.put_int("id", 5);
    let mut sx = CompoundTag::new();
    sx.put_long("y", 7);
    sx.put("z", compound(&[("q", int(1))]).into_tag());
    sx.put_int("w", 2);
    sel_a.put("x", sx.into_tag());
    let mut sa = CompoundTag::new();
    sa.put_byte("b", 1);
    sa.put_int("c", 2);
    sel_a.put("a", sa.into_tag());
    sel_a.put_double("Count", 3.5);
    sel_a.put_short("", 4);
    sel_a.put_string("other", "o");
    sel_a.put(
        "lst",
        list(vec![compound(&[("id", int(1))]).into_tag()]).into_tag(),
    );
    check("sel_a", &sel_a, true);

    let mut sel_b = CompoundTag::new();
    let mut bb = CompoundTag::new();
    bb.put("x", list(vec![int(1), int(2)]).into_tag());
    bb.put_int("y", 3);
    let mut ba = CompoundTag::new();
    ba.put("b", bb.into_tag());
    ba.put_int("c", 4);
    sel_b.put("a", ba.into_tag());
    sel_b.put("y", list(vec![int(5)]).into_tag());
    sel_b.put_string("x", "str");
    sel_b.put_string("id", "notint");
    check("sel_b", &sel_b, true);

    for n in [8, 9, 10, 11, 12, 20, 40] {
        let mut col = CompoundTag::new();
        for (i, k) in colliding(n).iter().enumerate() {
            col.put_int(k, i as i32);
        }
        check(&format!("colliding_{n}"), &col, false);
    }
    assert_eq!(
        checked,
        checked_rows_in(&rows),
        "every checkable CASE row was compared"
    );
}

#[test]
fn malformed_and_special_trees_match_the_jar() {
    let rw = rows("CASEW");
    let row = |name: &str| {
        rw.iter()
            .find(|r| r[1] == name)
            .unwrap_or_else(|| panic!("no CASEW {name}"))
    };
    let check = |name: &str, c: &CompoundTag| {
        let r = row(name);
        assert_eq!(hex(&nbt_write(c)), r[2], "CASEW {name}: write bytes");
        assert_eq!(esc(&c.to_string()), r[3], "CASEW {name}: toString");
        assert_eq!(
            c.size_in_bytes().to_string(),
            r[4],
            "CASEW {name}: sizeInBytes"
        );
    };
    check(
        "end_list",
        &compound(&[("l", list(vec![Tag::End, Tag::End]).into_tag())]),
    );
    check(
        "end_then_int_list",
        &compound(&[("l", list(vec![Tag::End, int(7)]).into_tag())]),
    );
    let mut end_val = CompoundTag::new();
    end_val.put("a", Tag::End);
    end_val.put_int("b", 1);
    check("end_value", &end_val);
    let mut ctl = CompoundTag::new();
    ctl.put_string(
        "tab\t",
        "nul\u{0} line\n cr\r bell\u{7} esc\u{1b} bs\u{8} ff\u{c}",
    );
    check("control_chars", &ctl);
    // `long_string_fallback`: NbtIo.write's StringFallbackDataOutput rewrites a string over
    // 65535 bytes -- NbtIo, not CompoundTag, so only the size (a CompoundTag method) is checked.
    let r = row("long_string_fallback");
    let mut long = CompoundTag::new();
    long.put_string("s", &"a".repeat(65536));
    long.put_string("t", &"\u{4e2d}".repeat(21845));
    assert_eq!(
        long.size_in_bytes().to_string(),
        r[4],
        "CASEW long_string_fallback: sizeInBytes"
    );
    assert_eq!(rw.len(), 5, "every CASEW row is checked");
}

#[test]
fn direct_string_writes_match_the_jar() {
    // `CASED`: `StringTag.write` straight to a `DataOutputStream` -- no NbtIo fallback.
    for r in rows("CASED") {
        let s = match r[1] {
            "long_string_direct" => "a".repeat(65536),
            "long_string_direct_cjk" => "\u{4e2d}".repeat(21846),
            "max_string_direct" => "\u{4e2d}".repeat(21845),
            other => panic!("unknown CASED {other}"),
        };
        let mut out = DataOutputStream::new(Vec::new());
        let got = match StringTag::value_of(s).into_tag().write(&mut out) {
            Ok(()) => vec!["OK".to_string(), hex(&out.into_inner())],
            Err(e) => exc_from_io(&e),
        };
        assert_eq!(got, r[2..].to_vec(), "CASED {}", r[1]);
    }
}

#[test]
fn list_element_type_identification_matches_the_jar() {
    let ident = rows("IDENT");
    let want = |name: &str| {
        ident.iter().find(|r| r[1] == name).unwrap()[2]
            .parse::<i8>()
            .unwrap()
    };
    let mut l = ListTag::new();
    assert_eq!(l.identify_raw_element_type(), want("empty"));
    l.add(int(1));
    assert_eq!(l.identify_raw_element_type(), want("int"));
    l.add(int(2));
    assert_eq!(l.identify_raw_element_type(), want("int_int"));
    l.add(StringTag::value_of("x").into_tag());
    assert_eq!(l.identify_raw_element_type(), want("int_int_string"));
    let l2 = list(vec![Tag::End, LongTag::value_of(5).into_tag()]);
    assert_eq!(l2.identify_raw_element_type(), want("end_long"));
    assert_eq!(ident.len(), 5);
}

fn opt<T: ToString>(o: Option<T>) -> String {
    o.map_or("\\N".to_string(), |v| v.to_string())
}

#[test]
fn list_getters_match_the_jar() {
    let l = list(vec![
        ByteTag::value_of(-5).into_tag(),
        FloatTag::value_of(3.75).into_tag(),
        StringTag::value_of("s").into_tag(),
        IntArrayTag::new(vec![9]).into_tag(),
        DoubleTag::value_of(-2.5).into_tag(),
    ]);
    let rs = rows("LGET");
    assert_eq!(rs.len(), 7);
    for r in rs {
        let i = i32f(r[1]);
        let got = vec![
            l.get_short_or(i, 77).to_string(),
            l.get_int_or(i, 77).to_string(),
            hex64(l.get_double_or(i, 77.0)),
            hex32(l.get_float_or(i, 77.0)),
            esc(l.get_string_or(i, "dflt")),
            opt(l.get_int(i)),
            l.get_string(i).map_or("\\N".to_string(), esc),
            l.get_double(i).map_or("\\N".to_string(), double_to_string),
            opt(l.get_int_array(i).map(|a| a.len())),
        ];
        assert_eq!(got, r[2..].to_vec(), "LGET index {i}");
    }
}

#[test]
fn compound_getters_match_the_jar() {
    let mut c = CompoundTag::new();
    c.put_float("f", -1.5);
    c.put_double("d", 1e10);
    c.put_string("s", "str");
    c.put_long("l", 0x1_0000_0080);
    c.put_boolean("t", true);
    c.put_byte("two", 2);
    let rs = rows("CGET");
    assert_eq!(rs.len(), 7);
    for r in rs {
        let k = r[1];
        let got = vec![
            c.get_byte_or(k, 9).to_string(),
            c.get_short_or(k, 9).to_string(),
            c.get_int_or(k, 9).to_string(),
            c.get_long_or(k, 9).to_string(),
            hex32(c.get_float_or(k, 9.0)),
            hex64(c.get_double_or(k, 9.0)),
            esc(c.get_string_or(k, "dflt")),
            c.get_boolean_or(k, false).to_string(),
            c.get_boolean_or(k, true).to_string(),
            opt(c.get_boolean(k)),
            opt(c.get_byte(k)),
            opt(c.get_long(k)),
            c.get_float(k).map_or("\\N".to_string(), float_to_string),
        ];
        assert_eq!(got, r[2..].to_vec(), "CGET {k}");
    }
}

/// Keeps the two imports above honest about what this module writes with.
#[allow(dead_code)]
fn _uses(out: &mut DataOutputStream<Vec<u8>>) {
    let _ = out.write_byte(0);
}
