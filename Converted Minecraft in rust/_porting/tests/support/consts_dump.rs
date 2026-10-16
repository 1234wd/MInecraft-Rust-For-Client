//! Shared helpers for checking ports against the generic constant dump
//! (`_porting/test-data/consts.txt`, produced by `java-oracle/ConstOracle.java`).
//!
//! Session 19 moved these out of `consts.rs` (which keeps its own copies, unchanged) so that
//! several test targets can use them without editing one shared file. Include with:
//!
//! ```ignore
//! #[path = "support/consts_dump.rs"]
//! mod consts_dump;
//! use consts_dump::*;
//! ```
//!
//! The dump covers top-level classes only. Nested classes (`Direction$Axis`,
//! `ByIdMap$OutOfBoundsStrategy`) have no group, so they cannot be checked here.
//!
//! Field values are the raw `type:value` strings the dump writes (`i32:3`, `str:stone`,
//! `f32:0x3f800000`), so what a test asserts is literally a substring of the golden row.

#![allow(dead_code)]

use minecraft_rust::javacompat::golden::{Golden, Row};
use std::sync::OnceLock;

fn dump() -> &'static Golden {
    static G: OnceLock<Golden> = OnceLock::new();
    G.get_or_init(|| Golden::load("consts.txt"))
}

pub fn const_rows(fq: &str) -> Vec<Row> {
    let group = format!("const.{fq}");
    let rows = dump().rows(&group);
    assert!(
        !rows.is_empty(),
        "consts.txt has no group `{group}`. Either the class is not an enum / has no comparable \
         static finals, or it is missing from the jar sweep."
    );
    rows.to_vec()
}

fn entry(row: &Row, key: &str) -> Option<String> {
    for v in row.expect.iter() {
        let s = v.as_opt_str().unwrap_or("");
        if let Some(rest) = s.strip_prefix(&format!("{key}=")) {
            return Some(rest.to_string());
        }
    }
    None
}

fn ordinal_of(row: &Row) -> i32 {
    entry(row, "ordinal")
        .unwrap_or_else(|| {
            panic!(
                "constant `{}` has no `ordinal=` entry; the dump ran on something that is not an enum",
                row.arg(0).as_opt_str().unwrap_or("?")
            )
        })
        .parse()
        .expect("ordinal is a decimal int")
}

/// Every constant name with its ordinal, exactly as the jar has them (sorted list equality, so
/// an invented or a dropped constant both fail).
pub fn assert_enum(fq: &str, expected: &[(&str, i32)]) {
    let rows = const_rows(fq);
    let mut got: Vec<(String, i32)> = rows
        .iter()
        .filter(|r| entry(r, "ordinal").is_some())
        .map(|r| (r.arg(0).as_opt_str().unwrap_or("").to_string(), ordinal_of(r)))
        .collect();
    got.sort();
    let mut want: Vec<(String, i32)> = expected.iter().map(|(n, o)| (n.to_string(), *o)).collect();
    want.sort();
    assert_eq!(got, want, "{fq}: enum constants/ordinals do not match the jar");
}

/// Declaration-order variant: `names[i]` must be the constant with ordinal `i`.
pub fn assert_enum_order(fq: &str, names: &[&str]) {
    let expected: Vec<(&str, i32)> = names.iter().enumerate().map(|(i, n)| (*n, i as i32)).collect();
    assert_enum(fq, &expected);
}

/// One enum constant's declared field, e.g. `("...Mirror", "NONE", "id", "str:none")`.
pub fn assert_enum_field(fq: &str, constant: &str, field: &str, expected: &str) {
    let rows = const_rows(fq);
    let row = rows
        .iter()
        .find(|r| r.arg(0).as_opt_str() == Some(constant))
        .unwrap_or_else(|| panic!("{fq} has no constant `{constant}` in the jar's dump"));
    let got = entry(row, field).unwrap_or_else(|| {
        panic!("{fq}.{constant} has no `{field}=` entry; expected column was {:?}", row.expect)
    });
    assert_eq!(got, expected, "{fq}.{constant}.{field}: the jar disagrees with the port");
}

/// A `static final` scalar, e.g. `("...QuartPos", "BITS", "i32:2")`.
pub fn assert_const(fq: &str, field: &str, expected: &str) {
    let rows = const_rows(fq);
    let row = rows
        .iter()
        .find(|r| r.arg(0).as_opt_str() == Some(field))
        .unwrap_or_else(|| panic!("{fq} has no static final `{field}` in the jar's dump"));
    let got = entry(row, "value").unwrap_or_else(|| panic!("{fq}.{field} has no `value=` entry"));
    assert_eq!(got, expected, "{fq}.{field}: the jar disagrees with the port");
}

/// Every `static final` scalar the jar dumps for `fq` must be listed in `expected`, and every
/// listed one must match. Catches a constant the port forgot, not just a wrong value.
pub fn assert_all_consts(fq: &str, expected: &[(&str, String)]) {
    let rows = const_rows(fq);
    let mut got: Vec<(String, String)> = rows
        .iter()
        .filter_map(|r| entry(r, "value").map(|v| (r.arg(0).as_opt_str().unwrap_or("").to_string(), v)))
        .collect();
    got.sort();
    let mut want: Vec<(String, String)> =
        expected.iter().map(|(n, v)| (n.to_string(), v.clone())).collect();
    want.sort();
    assert_eq!(got, want, "{fq}: static final constants do not match the jar");
}

/// Format helpers producing the dump's `type:value` spelling.
pub fn i32v(v: i32) -> String {
    format!("i32:{v}")
}
pub fn i64v(v: i64) -> String {
    format!("i64:{v}")
}
pub fn strv(v: &str) -> String {
    format!("str:{v}")
}
pub fn boolv(v: bool) -> String {
    format!("bool:{v}")
}
pub fn f32v(v: f32) -> String {
    format!("f32:0x{:08x}", v.to_bits())
}
pub fn f64v(v: f64) -> String {
    format!("f64:0x{:016x}", v.to_bits())
}
