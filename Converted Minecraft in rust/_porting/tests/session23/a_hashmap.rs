//! Sub-batch A: `javacompat::java_hash_map` against JDK 25's `java.util.HashMap`.
//!
//! Groups `HMOP`/`HMRES`/`HMCOPY` (session 19's op logs and final states), `HMINT`/`HMINTRES`
//! (Integer keys) and session 23's `HMSTEP` (the whole state after EVERY op: return value, table
//! length, size, hashCode, iteration order) and `HMEQ` (`AbstractMap.equals`/`hashCode`).
//!
//! Every op and key comes from the row itself, so nothing here re-implements the probe's
//! scenario generators; a test cannot drift from the probe it checks.

use crate::nbt_probe::*;

use minecraft_rust::javacompat::java_hash_map::{JavaHashKey, JavaHashMap};

/// The probe's `esc(key) + "=" + value` spelling of every entry, in iteration order.
fn entries_s(m: &JavaHashMap<String, i32>) -> Vec<String> {
    m.iter().map(|(k, v)| format!("{}={v}", esc(k))).collect()
}

fn entries_i(m: &JavaHashMap<i32, i32>) -> Vec<String> {
    m.iter().map(|(k, v)| format!("{k}={v}")).collect()
}

/// `HashMap<_, Integer>.hashCode()`: `Integer.hashCode` is the value.
fn hash_of<K: JavaHashKey + Eq>(m: &JavaHashMap<K, i32>) -> i32 {
    m.java_hash_code(|v| *v)
}

/// Rows split by case name (field 1), in file order.
fn by_case(rows: Vec<Vec<&'static str>>) -> Vec<(&'static str, Vec<Vec<&'static str>>)> {
    let mut out: Vec<(&'static str, Vec<Vec<&'static str>>)> = Vec::new();
    for r in rows {
        match out.last_mut() {
            Some((name, rs)) if *name == r[1] => rs.push(r),
            _ => out.push((r[1], vec![r])),
        }
    }
    out
}

#[test]
fn hashmap_op_logs_end_in_the_jdk_state() {
    let results = rows("HMRES");
    let copies = rows("HMCOPY");
    let cases = by_case(rows("HMOP"));
    assert_eq!(cases.len(), results.len(), "one HMRES per HMOP case");
    assert_eq!(cases.len(), copies.len(), "one HMCOPY per HMOP case");
    for (name, ops) in &cases {
        let mut m: JavaHashMap<String, i32> = JavaHashMap::new();
        for (idx, op) in ops.iter().enumerate() {
            let key = s(op[3]);
            match op[2] {
                "P" => {
                    m.put(key, idx as i32);
                }
                "R" => {
                    m.remove(key.as_str());
                }
                "C" => m.clear(),
                "K" => m = JavaHashMap::copy_of(&m),
                other => panic!("unknown HMOP op {other}"),
            }
        }
        let res = results.iter().find(|r| r[1] == *name).expect("HMRES row");
        assert_eq!(m.table_length(), i32f(res[2]), "{name}: table length");
        assert_eq!(m.size(), i32f(res[3]), "{name}: size");
        assert_eq!(hash_of(&m), i32f(res[4]), "{name}: hashCode");
        assert_eq!(entries_s(&m), res[5..].to_vec(), "{name}: iteration order");

        let copy = JavaHashMap::copy_of(&m);
        let cp = copies.iter().find(|r| r[1] == *name).expect("HMCOPY row");
        assert_eq!(
            copy.table_length(),
            i32f(cp[2]),
            "{name}: copy's table length"
        );
        let keys: Vec<String> = copy.keys().map(|k| esc(k)).collect();
        assert_eq!(keys, cp[3..].to_vec(), "{name}: copy's iteration order");
        assert!(copy == m, "{name}: a copy equals its source");
    }
}

#[test]
fn integer_keyed_hashmap_matches_the_jdk() {
    let put = &rows("HMINT")[0];
    let res = &rows("HMINTRES")[0];
    let mut m: JavaHashMap<i32, i32> = JavaHashMap::new();
    for (i, k) in put[1..].iter().enumerate() {
        m.put(i32f(k), i as i32);
    }
    assert_eq!(m.table_length(), i32f(res[1]), "table length");
    let keys: Vec<String> = m.keys().map(|k| k.to_string()).collect();
    assert_eq!(keys, res[2..].to_vec(), "iteration order");
}

/// Replays one `HMSTEP` case, checking every step.
fn replay_steps<K, F, E>(name: &str, steps: &[Vec<&'static str>], parse: F, entries: E) -> usize
where
    K: JavaHashKey + Eq + Clone,
    F: Fn(&str) -> K,
    E: Fn(&JavaHashMap<K, i32>) -> Vec<String>,
{
    let mut m: JavaHashMap<K, i32> = JavaHashMap::new();
    let mut checked = 0;
    for r in steps {
        let (step, op, value, want_ret) = (r[3], r[4], i32f(r[6]), r[7]);
        let ret = match op {
            "P" => m.put(parse(r[5]), value),
            "R" => m.remove(&parse(r[5])),
            "G" => m.get(&parse(r[5])).copied(),
            "C" => {
                m.clear();
                None
            }
            "K" => {
                m = JavaHashMap::copy_of(&m);
                None
            }
            other => panic!("unknown HMSTEP op {other}"),
        };
        let ret = ret.map_or("\\N".to_string(), |v| v.to_string());
        assert_eq!(
            ret, want_ret,
            "{name} step {step} ({op} {}): return value",
            r[5]
        );
        assert_eq!(
            m.table_length(),
            i32f(r[8]),
            "{name} step {step}: table length"
        );
        assert_eq!(m.size(), i32f(r[9]), "{name} step {step}: size");
        assert_eq!(hash_of(&m), i32f(r[10]), "{name} step {step}: hashCode");
        assert_eq!(
            entries(&m),
            r[11..].to_vec(),
            "{name} step {step} ({op} {}): iteration order",
            r[5]
        );
        checked += 1;
    }
    checked
}

#[test]
fn hashmap_matches_the_jdk_after_every_step() {
    let cases = by_case(rows("HMSTEP"));
    let names: Vec<&str> = cases.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        names,
        [
            "tree_grow",
            "same_hash",
            "int_keys",
            "split_boundary_0",
            "split_boundary_1",
            "split_boundary_2",
            "split_boundary_3",
            "split_boundary_4",
            "split_boundary_5"
        ],
        "the probe's HMSTEP cases"
    );
    let mut checked = 0;
    for (name, steps) in &cases {
        checked += match steps[0][2] {
            "S" => replay_steps(name, steps, s, entries_s),
            "I" => replay_steps(name, steps, i32f, entries_i),
            other => panic!("unknown key type {other}"),
        };
    }
    assert_eq!(checked, rows("HMSTEP").len());
}

/// Builds a map from one side of an `HMEQ` script: `P:k=v` / `R:k`, joined by `,`.
fn script(s: &str) -> JavaHashMap<String, i32> {
    let mut m = JavaHashMap::new();
    if s == "-" {
        return m;
    }
    for op in s.split(',') {
        if let Some(kv) = op.strip_prefix("P:") {
            let (k, v) = kv.split_once('=').expect("k=v");
            m.put(k.to_string(), i32f(v));
        } else {
            m.remove(op.strip_prefix("R:").expect("R:key"));
        }
    }
    m
}

#[test]
fn hashmap_equality_is_by_content_not_history() {
    for r in rows("HMEQ") {
        let (a, b) = (script(r[2]), script(r[3]));
        assert_eq!(a == b, boolf(r[4]), "{}: a.equals(b)", r[1]);
        assert_eq!(b == a, boolf(r[5]), "{}: b.equals(a)", r[1]);
        assert_eq!(hash_of(&a), i32f(r[6]), "{}: a.hashCode()", r[1]);
        assert_eq!(hash_of(&b), i32f(r[7]), "{}: b.hashCode()", r[1]);
    }
}
