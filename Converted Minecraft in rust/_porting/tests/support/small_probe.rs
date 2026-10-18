//! Reader for `_porting/test-data/small_probe.txt`, written by
//! `_porting/java-oracle/src/p/SmallProbe.java` against the real 26.2 jar on JDK 25.
//!
//! Session 22. A module, not a test target: `tests/session22.rs` includes it with
//! `#[path = "support/small_probe.rs"]`.
//!
//! Line format (the same as `nbt_probe.txt`): TAB-separated fields, the first is the group
//! ("kind"). Strings are escaped by the probe's `esc`: a backslash is doubled, every char outside
//! `0x20..=0x7E` is written as `\uXXXX` UTF-16 code units, and a Java `null` is the two
//! characters `\N`. Lines starting with `#` are comments.

#![allow(dead_code)]

use std::collections::BTreeSet;
use std::sync::OnceLock;

fn text() -> &'static str {
    static T: OnceLock<String> = OnceLock::new();
    T.get_or_init(|| {
        let path = minecraft_rust::javacompat::golden::test_data_dir().join("small_probe.txt");
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
    })
}

fn all_rows() -> impl Iterator<Item = Vec<&'static str>> {
    text()
        .lines()
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.split('\t').collect::<Vec<&str>>())
}

/// Every row of group `kind`, fields still escaped. Panics if the group is empty, so a typo in
/// a group name (or a probe section that silently wrote nothing) cannot pass as "no rows to check".
pub fn rows(kind: &str) -> Vec<Vec<&'static str>> {
    let rows: Vec<Vec<&'static str>> = all_rows().filter(|f| f[0] == kind).collect();
    assert!(!rows.is_empty(), "small_probe.txt has no `{kind}` rows");
    rows
}

/// The distinct group names in the file.
pub fn kinds() -> BTreeSet<&'static str> {
    all_rows().map(|f| f[0]).collect()
}

/// The probe's `esc` run backwards. `None` is a Java `null`.
///
/// # Panics
///
/// On an unpaired surrogate, which no Rust `String` can hold; no session-22 row carries one.
pub fn unesc(field: &str) -> Option<String> {
    if field == "\\N" {
        return None;
    }
    let chars: Vec<char> = field.chars().collect();
    let mut units: Vec<u16> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' {
            if chars.get(i + 1) == Some(&'\\') {
                units.push('\\' as u16);
                i += 2;
                continue;
            }
            if chars.get(i + 1) == Some(&'u') && i + 6 <= chars.len() {
                let hex: String = chars[i + 2..i + 6].iter().collect();
                units.push(u16::from_str_radix(&hex, 16).expect("hex escape"));
                i += 6;
                continue;
            }
            panic!("unrecognised escape at {i} in {field:?}");
        }
        let mut buf = [0u16; 2];
        units.extend_from_slice(chars[i].encode_utf16(&mut buf));
        i += 1;
    }
    Some(String::from_utf16(&units).unwrap_or_else(|_| panic!("unpaired surrogate in {field:?}")))
}

/// [`unesc`] for a field that must not be null.
pub fn s(field: &str) -> String {
    unesc(field).unwrap_or_else(|| panic!("field is a Java null where a value was expected"))
}

pub fn i32f(field: &str) -> i32 {
    field
        .parse()
        .unwrap_or_else(|_| panic!("not an int: {field:?}"))
}

pub fn i64f(field: &str) -> i64 {
    field
        .parse()
        .unwrap_or_else(|_| panic!("not a long: {field:?}"))
}

pub fn boolf(field: &str) -> bool {
    match field {
        "true" => true,
        "false" => false,
        _ => panic!("not a boolean: {field:?}"),
    }
}

/// A float written by the probe as `Integer.toHexString(Float.floatToRawIntBits(v))`.
pub fn f32bits(field: &str) -> u32 {
    u32::from_str_radix(field, 16).unwrap_or_else(|_| panic!("not f32 bits: {field:?}"))
}

/// A double written by the probe as `Long.toHexString(Double.doubleToRawLongBits(v))`.
pub fn f64bits(field: &str) -> u64 {
    u64::from_str_radix(field, 16).unwrap_or_else(|_| panic!("not f64 bits: {field:?}"))
}

pub fn f32f(field: &str) -> f32 {
    f32::from_bits(f32bits(field))
}

pub fn f64f(field: &str) -> f64 {
    f64::from_bits(f64bits(field))
}

/// The bit pattern of `v`, in the probe's spelling, for an exact comparison in a message.
pub fn hex32(v: f32) -> String {
    format!("{:x}", v.to_bits())
}

pub fn hex64(v: f64) -> String {
    format!("{:x}", v.to_bits())
}
