//! Sub-batch B, continued: session 23's own container groups.
//!
//! * `CTOPS`: a `CompoundTag` through every mutator and every getter (all thirteen tag kinds
//!   under a key, plus a missing key), `merge`, `copy()`/`shallowCopy()` and copy isolation --
//!   with the whole state (literal in iteration order, write bytes, toString, hashCode,
//!   sizeInBytes, size) after each op.
//! * `LTOPS`: a `ListTag` the same way, plus the index errors, `addAndUnwrap` and element-type
//!   transitions.
//! * `TEQ`: `Tag.equals` between 31 tags, and each one's `hashCode`.
//! * `TACC`: `acceptAsRoot` traces over containers the generated trees never hold -- lists with
//!   `END` elements, wrappers, nesting, a list root and an `END` root -- under three policies.
//! * `TLOAD`: `TYPE.load`/`skip`/`parse` of crafted `LIST`/`COMPOUND` payloads under three
//!   accounter limits, including every exception those paths raise.

use crate::b_support::*;
use crate::nbt_probe::*;

use minecraft_rust::javacompat::java_io::DataInputStream;
use minecraft_rust::net::minecraft::nbt::ByteArrayTag::ByteArrayTag;
use minecraft_rust::net::minecraft::nbt::ByteTag::ByteTag;
use minecraft_rust::net::minecraft::nbt::CollectionTag::CollectionTag;
use minecraft_rust::net::minecraft::nbt::CompoundTag::CompoundTag;
use minecraft_rust::net::minecraft::nbt::DoubleTag::DoubleTag;
use minecraft_rust::net::minecraft::nbt::FloatTag::FloatTag;
use minecraft_rust::net::minecraft::nbt::IntArrayTag::IntArrayTag;
use minecraft_rust::net::minecraft::nbt::IntTag::IntTag;
use minecraft_rust::net::minecraft::nbt::ListTag::ListTag;
use minecraft_rust::net::minecraft::nbt::NbtAccounter::NbtAccounter;
use minecraft_rust::net::minecraft::nbt::StreamTagVisitor::ValueResult;
use minecraft_rust::net::minecraft::nbt::StringTag::StringTag;
use minecraft_rust::net::minecraft::nbt::Tag::Tag;
use minecraft_rust::net::minecraft::nbt::TagTypes;

fn int(v: i32) -> Tag {
    IntTag::value_of(v).into_tag()
}

fn opt_str<T: ToString>(o: Option<T>) -> String {
    o.map_or("-".to_string(), |v| v.to_string())
}

fn opt_lit(o: Option<Tag>) -> String {
    o.map_or("\\N".to_string(), |t| lit(&t))
}

/// `hashCode`, or `-` where it folds in `EndTag`'s identity hash.
fn hash_of(t: &Tag) -> String {
    t.java_hash_code()
        .map_or("-".to_string(), |h| h.to_string())
}

/// The probe's `state(Tag)`.
fn state(t: &Tag) -> Vec<String> {
    let (size, ident) = match t {
        Tag::List(l) => (l.size(), l.identify_raw_element_type().to_string()),
        Tag::Compound(c) => (c.size(), "-".to_string()),
        other => panic!("not a container: {other:?}"),
    };
    vec![
        lit(t),
        write_hex(t),
        esc(&t.to_string()),
        hash_of(t),
        t.size_in_bytes().to_string(),
        size.to_string(),
        ident,
    ]
}

/// The probe's `compoundGetters(c, k)`.
fn compound_getters(c: &CompoundTag, k: &str) -> String {
    [
        format!("B={}", opt_str(c.get_byte(k))),
        format!("BO={}", c.get_byte_or(k, 7)),
        format!("S={}", opt_str(c.get_short(k))),
        format!("SO={}", c.get_short_or(k, 7)),
        format!("I={}", opt_str(c.get_int(k))),
        format!("IO={}", c.get_int_or(k, 7)),
        format!("L={}", opt_str(c.get_long(k))),
        format!("LO={}", c.get_long_or(k, 7)),
        format!("F={}", c.get_float(k).map_or("-".to_string(), hex32)),
        format!("FO={}", hex32(c.get_float_or(k, 7.0))),
        format!("D={}", c.get_double(k).map_or("-".to_string(), hex64)),
        format!("DO={}", hex64(c.get_double_or(k, 7.0))),
        format!("STR={}", c.get_string(k).map_or("-".to_string(), esc)),
        format!("STRO={}", esc(c.get_string_or(k, "d"))),
        format!("BA={}", opt_str(c.get_byte_array(k).map(|a| a.len()))),
        format!("IA={}", opt_str(c.get_int_array(k).map(|a| a.len()))),
        format!("LA={}", opt_str(c.get_long_array(k).map(|a| a.len()))),
        format!(
            "C={}",
            c.get_compound(k)
                .map_or("-".to_string(), |x| lit(&x.clone().into_tag()))
        ),
        format!(
            "CO={}",
            lit(&c.get_compound_or_empty(k).into_owned().into_tag())
        ),
        format!(
            "LI={}",
            c.get_list(k)
                .map_or("-".to_string(), |x| lit(&x.clone().into_tag()))
        ),
        format!(
            "LIO={}",
            lit(&c.get_list_or_empty(k).into_owned().into_tag())
        ),
        format!("BOOL={}", opt_str(c.get_boolean(k))),
        format!("BOOLO0={}", c.get_boolean_or(k, false)),
        format!("BOOLO1={}", c.get_boolean_or(k, true)),
        format!("HAS={}", c.contains(k)),
        format!("GET={}", c.get(k).map_or("-".to_string(), lit)),
    ]
    .join(";")
}

/// The probe's `listGetters(l, i)`.
fn list_getters(l: &ListTag, i: i32) -> String {
    [
        format!(
            "C={}",
            l.get_compound(i)
                .map_or("-".to_string(), |x| lit(&x.clone().into_tag()))
        ),
        format!(
            "CO={}",
            lit(&l.get_compound_or_empty(i).into_owned().into_tag())
        ),
        format!(
            "LI={}",
            l.get_list(i)
                .map_or("-".to_string(), |x| lit(&x.clone().into_tag()))
        ),
        format!(
            "LIO={}",
            lit(&l.get_list_or_empty(i).into_owned().into_tag())
        ),
        format!("S={}", opt_str(l.get_short(i))),
        format!("SO={}", l.get_short_or(i, 7)),
        format!("I={}", opt_str(l.get_int(i))),
        format!("IO={}", l.get_int_or(i, 7)),
        format!("IA={}", opt_str(l.get_int_array(i).map(|a| a.len()))),
        format!("LA={}", opt_str(l.get_long_array(i).map(|a| a.len()))),
        format!("D={}", l.get_double(i).map_or("-".to_string(), hex64)),
        format!("DO={}", hex64(l.get_double_or(i, 7.0))),
        format!("F={}", l.get_float(i).map_or("-".to_string(), hex32)),
        format!("FO={}", hex32(l.get_float_or(i, 7.0))),
        format!("STR={}", l.get_string(i).map_or("-".to_string(), esc)),
        format!("STRO={}", esc(l.get_string_or(i, "d"))),
    ]
    .join(";")
}

/// `"key"=rest` -> (key, rest).
fn split_kv(arg: &str) -> (String, &str) {
    let close = arg[1..].find('"').expect("closing quote") + 1;
    (
        parse_qstring(&arg[..=close]),
        arg[close + 1..].strip_prefix('=').expect("="),
    )
}

fn numbers<T: std::str::FromStr>(s: &str) -> Vec<T> {
    if s.is_empty() {
        return Vec::new();
    }
    s.split(',')
        .map(|x| x.parse().unwrap_or_else(|_| panic!("bad number {x}")))
        .collect()
}

#[test]
fn compound_op_script_matches_the_jar() {
    let rs = rows("CTOPS");
    let mut c = CompoundTag::new();
    let mut copy: Option<CompoundTag> = None;
    let mut grown = CompoundTag::new();
    let mut merged = CompoundTag::new();
    for r in &rs {
        let (name, step, op, arg) = (r[1], r[2], r[3], r[4]);
        let mut ret = "-".to_string();
        let shown: CompoundTag = match (name, op) {
            ("compound", "put") => {
                let (k, v) = split_kv(arg);
                ret = opt_lit(c.put(&k, parse_lit(v)));
                c.clone()
            }
            ("compound", "getters") => {
                ret = esc(&compound_getters(&c, &parse_qstring(arg)));
                c.clone()
            }
            ("compound", "remove") => {
                ret = opt_lit(c.remove(&parse_qstring(arg)));
                c.clone()
            }
            ("compound", "merge") => {
                let src = parse_lit(arg);
                c.merge(src.as_compound().expect("compound"));
                c.clone()
            }
            ("compound", "mutated-original") => {
                copy = Some(c.copy());
                c.get_compound_mut("ec").expect("ec").put_int("after", 1);
                c.get_list_mut("new").expect("new").add(int(4));
                c.clone()
            }
            ("compound", "copy") => {
                let cp = copy.take().expect("copy taken at mutated-original");
                ret = (cp == c).to_string();
                cp
            }
            ("compound", "shallowCopy") => c.shallow_copy(),
            ("compound", typed) => {
                let (k, v) = split_kv(arg);
                match typed {
                    "putByte" => c.put_byte(&k, v.parse().unwrap()),
                    "putShort" => c.put_short(&k, v.parse().unwrap()),
                    "putInt" => c.put_int(&k, v.parse().unwrap()),
                    "putLong" => c.put_long(&k, v.parse().unwrap()),
                    "putFloat" => c.put_float(&k, f32f(v)),
                    "putDouble" => c.put_double(&k, f64f(v)),
                    "putString" => c.put_string(&k, &parse_qstring(v)),
                    "putByteArray" => c.put_byte_array(&k, numbers(v)),
                    "putIntArray" => c.put_int_array(&k, numbers(v)),
                    "putLongArray" => c.put_long_array(&k, numbers(v)),
                    "putBoolean" => c.put_boolean(&k, boolf(v)),
                    other => panic!("unknown CTOPS op {other}"),
                }
                c.clone()
            }
            ("grown", "grow-shrink") => {
                for i in 0..100 {
                    grown.put_int(&format!("g{i}"), i);
                }
                for i in 0..90 {
                    grown.remove(&format!("g{i}"));
                }
                grown.clone()
            }
            ("grown", "copy") => {
                ret = (grown.copy() == grown).to_string();
                grown.copy()
            }
            ("grown", "shallowCopy") => grown.shallow_copy(),
            ("grown", "nested-copy") => {
                let mut outer = CompoundTag::new();
                outer.put("g", grown.clone().into_tag());
                outer.put_int("z", 1);
                outer.copy()
            }
            ("merge2", "base") => {
                merged = parse_lit(arg).as_compound().expect("compound").clone();
                merged.clone()
            }
            ("merge2", "merge") => {
                merged.merge(parse_lit(arg).as_compound().expect("compound"));
                merged.clone()
            }
            other => panic!("unknown CTOPS step {other:?}"),
        };
        assert_eq!(ret, r[5], "CTOPS {name} {step} {op} {arg}: return value");
        assert_eq!(
            state(&shown.into_tag()),
            r[6..].to_vec(),
            "CTOPS {name} {step} {op} {arg}: state"
        );
    }
}

/// One `LTOPS` edit -- `set`/`remove`/`addAt`/`setTag`/`addTag`/`get` -- with Java's return.
fn list_edit(l: &mut ListTag, op: &str, arg: &str) -> String {
    let (idx, lit_arg) = arg.split_once('=').unwrap_or((arg, ""));
    let idx: i32 = idx.parse().unwrap();
    let res = catch(|| match op {
        "set" => lit(&l.set(idx, parse_lit(lit_arg))),
        "remove" => lit(&l.remove(idx)),
        "addAt" => {
            l.add_i32_tag(idx, parse_lit(lit_arg));
            "-".to_string()
        }
        "setTag" => CollectionTag::set_tag(l, idx, &parse_lit(lit_arg)).to_string(),
        "addTag" => CollectionTag::add_tag(l, idx, &parse_lit(lit_arg)).to_string(),
        "get" => lit(l.get(idx)),
        other => panic!("unknown LTOPS edit {other}"),
    });
    res.unwrap_or_else(|panic| ex_text(&panic))
}

#[test]
fn list_op_script_matches_the_jar() {
    let rs = rows("LTOPS");
    let mut l = ListTag::new();
    let mut tr = ListTag::new();
    let mut copy: Option<ListTag> = None;
    for r in &rs {
        let (name, step, op, arg) = (r[1], r[2], r[3], r[4]);
        let mut ret = "-".to_string();
        let target = if name == "transitions" {
            &mut tr
        } else {
            &mut l
        };
        let shown: ListTag = match op {
            "new" => target.clone(),
            "add" => {
                let added = target.add(parse_lit(arg));
                if name == "list" {
                    ret = added.to_string();
                }
                target.clone()
            }
            "getters" => {
                ret = esc(&list_getters(target, arg.parse().unwrap()));
                target.clone()
            }
            "set" | "addAt" | "setTag" | "addTag" | "get" => {
                ret = list_edit(target, op, arg);
                target.clone()
            }
            "remove" if name == "list" => {
                ret = list_edit(target, op, arg);
                target.clone()
            }
            "remove" => {
                target.remove(arg.parse().unwrap());
                target.clone()
            }
            "mutated-original" => {
                copy = Some(target.copy());
                target
                    .get_compound_mut(4)
                    .expect("compound at 4")
                    .put_int("y", 2);
                target.clone()
            }
            "copy" => {
                let cp = copy.take().expect("copy taken at mutated-original");
                ret = (cp == *target).to_string();
                cp
            }
            "clear" => {
                target.clear();
                target.clone()
            }
            "addAndUnwrap" => {
                target.add_and_unwrap(parse_lit(arg));
                target.clone()
            }
            other => panic!("unknown LTOPS op {other}"),
        };
        assert_eq!(ret, r[5], "LTOPS {name} {step} {op} {arg}: return value");
        assert_eq!(
            state(&shown.into_tag()),
            r[6..].to_vec(),
            "LTOPS {name} {step} {op} {arg}: state"
        );
    }
}

fn compound_of(kv: &[(&str, Tag)]) -> Tag {
    let mut c = CompoundTag::new();
    for (k, v) in kv {
        c.put(k, v.clone());
    }
    c.into_tag()
}

fn list_of(tags: &[Tag]) -> Tag {
    let mut l = ListTag::new();
    for t in tags {
        l.add(t.clone());
    }
    l.into_tag()
}

#[test]
fn tag_equality_and_hashes_match_the_jar() {
    let f = |bits: u32| FloatTag::new(f32::from_bits(bits)).into_tag();
    let d = |bits: u64| DoubleTag::new(f64::from_bits(bits)).into_tag();
    // The probe's `eq` array, entry for entry.
    let tags: Vec<Tag> = vec![
        FloatTag::new(0.0).into_tag(),
        FloatTag::new(-0.0).into_tag(),
        FloatTag::new(f32::NAN).into_tag(),
        f(0x7f80_0001),
        FloatTag::new(1.0).into_tag(),
        DoubleTag::new(0.0).into_tag(),
        DoubleTag::new(-0.0).into_tag(),
        DoubleTag::new(f64::NAN).into_tag(),
        d(0xfff0_0000_0000_0001),
        DoubleTag::new(1.0).into_tag(),
        ByteTag::value_of(1).into_tag(),
        int(1),
        StringTag::value_of("1").into_tag(),
        compound_of(&[("a", FloatTag::new(0.0).into_tag())]),
        compound_of(&[("a", FloatTag::new(-0.0).into_tag())]),
        compound_of(&[("a", FloatTag::new(f32::NAN).into_tag())]),
        compound_of(&[("a", f(0x7fc0_0001))]),
        list_of(&[DoubleTag::new(f64::NAN).into_tag()]),
        list_of(&[d(0x7ff0_0000_0000_0002)]),
        list_of(&[int(1), int(2)]),
        list_of(&[int(2), int(1)]),
        compound_of(&[("a", int(1)), ("b", int(2))]),
        compound_of(&[("b", int(2)), ("a", int(1))]),
        ListTag::new().into_tag(),
        CompoundTag::new().into_tag(),
        ByteArrayTag::new(vec![1]).into_tag(),
        IntArrayTag::new(vec![1]).into_tag(),
        list_of(&[compound_of(&[("", int(1))])]),
        list_of(&[int(1)]),
        Tag::End,
        list_of(&[Tag::End]),
    ];
    let rs = rows("TEQ");
    assert_eq!(rs.len(), tags.len());
    for (r, t) in rs.iter().zip(tags.iter()) {
        assert_eq!(lit(t), r[2], "TEQ {}: the probe's tag", r[1]);
        assert_eq!(hash_of(t), r[3], "TEQ {}: hashCode", r[1]);
        let bits: String = tags
            .iter()
            .map(|o| if t == o { '1' } else { '0' })
            .collect();
        assert_eq!(bits, r[4], "TEQ {}: equals row", r[1]);
    }
}

#[test]
fn accept_as_root_traces_match_the_jar() {
    for r in rows("TACC") {
        let (name, policy) = (r[1], i32f(r[2]));
        let t = parse_lit(r[3]);
        let mut trace = Trace::for_policy(policy, name);
        t.accept_as_root(&mut trace);
        assert_eq!(esc(&trace.log), r[4], "TACC {name}, policy {policy}");
    }
}

/// One `TLOAD` result: `OK` + fields, or the probe's `exc` spelling.
fn tload_fields(
    run: impl FnOnce(&mut DataInputStream<&[u8]>, &mut NbtAccounter) -> std::io::Result<Vec<String>>,
    bytes: &[u8],
    quota: i64,
    depth: i32,
) -> Vec<String> {
    let mut input = DataInputStream::new(bytes);
    let mut acc = NbtAccounter::new(quota, depth);
    match catch(|| run(&mut input, &mut acc)) {
        Ok(Ok(mut fields)) => {
            fields.push(acc.get_usage().to_string());
            fields.push(acc.get_depth().to_string());
            fields
        }
        Ok(Err(e)) => exc_from_io(&e),
        Err(panic) => exc_from_panic(&panic),
    }
}

#[test]
fn container_type_load_skip_parse_match_the_jar() {
    for r in rows("TLOAD") {
        let (name, type_id, quota, depth) = (r[1], i32f(r[2]), i64f(r[3]), i32f(r[4]));
        let bytes = unhex(r[5]);
        let ty = TagTypes::get_type(type_id);
        let load = tload_fields(
            |input, acc| {
                let t = ty.load(input, acc)?;
                Ok(vec!["OK".to_string(), lit(&t)])
            },
            &bytes,
            quota,
            depth,
        );
        let skip = tload_fields(
            |input, acc| {
                ty.skip(input, acc)?;
                Ok(vec!["OK".to_string(), input.get_ref().len().to_string()])
            },
            &bytes,
            quota,
            depth,
        );
        let mut tv = Trace::new(None);
        let parse = {
            let mut input = DataInputStream::new(bytes.as_slice());
            let mut acc = NbtAccounter::new(quota, depth);
            match catch(|| ty.parse(&mut input, &mut tv, &mut acc)) {
                Ok(Ok(res)) => vec![
                    "OK".to_string(),
                    esc(&tv.log),
                    match res {
                        ValueResult::Continue => "CONTINUE",
                        ValueResult::Break => "BREAK",
                        ValueResult::Halt => "HALT",
                    }
                    .to_string(),
                    input.get_ref().len().to_string(),
                    acc.get_usage().to_string(),
                    acc.get_depth().to_string(),
                ],
                Ok(Err(e)) => {
                    let mut f = exc_from_io(&e);
                    f.push(esc(&tv.log));
                    f
                }
                Err(panic) => {
                    let mut f = exc_from_panic(&panic);
                    f.push(esc(&tv.log));
                    f
                }
            }
        };
        let mut got = load;
        got.extend(skip);
        got.extend(parse);
        assert_eq!(
            got,
            r[6..].to_vec(),
            "TLOAD {name} (type {type_id}, quota {quota}, depth {depth})"
        );
    }
}
