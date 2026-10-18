//! Reader for `_porting/test-data/nbt_probe.txt`, written by
//! `_porting/java-oracle/src/p/NbtProbe.java` against the real 26.2 jar on JDK 25.
//!
//! Session 23. A module, not a test target: `tests/session23.rs` includes it with
//! `#[path = "support/nbt_probe.rs"]`. (`tests/session19_g.rs` keeps its own inline reader.)
//!
//! Line format: TAB-separated fields, the first is the group ("kind"). Strings are escaped by the
//! probe's `esc`: a backslash is doubled, every char outside `0x20..=0x7E` is written as `\uXXXX`
//! UTF-16 code units, and a Java `null` is the two characters `\N`. Byte strings are lower-case
//! hex. Lines starting with `#` are comments.

#![allow(dead_code)]

use std::collections::BTreeSet;
use std::sync::OnceLock;

fn text() -> &'static str {
    static T: OnceLock<String> = OnceLock::new();
    T.get_or_init(|| {
        let path = minecraft_rust::javacompat::golden::test_data_dir().join("nbt_probe.txt");
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

/// Every row of group `kind`, fields still escaped. Panics if the group is empty, so a typo in a
/// group name cannot pass as "no rows to check".
pub fn rows(kind: &str) -> Vec<Vec<&'static str>> {
    let rows: Vec<Vec<&'static str>> = all_rows().filter(|f| f[0] == kind).collect();
    assert!(!rows.is_empty(), "nbt_probe.txt has no `{kind}` rows");
    rows
}

/// The distinct group names in the file.
pub fn kinds() -> BTreeSet<&'static str> {
    all_rows().map(|f| f[0]).collect()
}

/// The probe's `esc` run backwards, to UTF-16 code units. `None` is a Java `null`.
pub fn unesc_units(field: &str) -> Option<Vec<u16>> {
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
    Some(units)
}

/// [`unesc_units`] as a `String`.
///
/// # Panics
///
/// On an unpaired surrogate, which no Rust `String` can hold.
pub fn unesc(field: &str) -> Option<String> {
    unesc_units(field).map(|units| {
        String::from_utf16(&units).unwrap_or_else(|_| panic!("unpaired surrogate in {field:?}"))
    })
}

/// [`unesc`] for a field that must not be null.
pub fn s(field: &str) -> String {
    unesc(field).unwrap_or_else(|| panic!("field is a Java null where a value was expected"))
}

/// The probe's `esc`, forwards.
pub fn esc(s: &str) -> String {
    let mut out = String::new();
    for u in s.encode_utf16() {
        if u == '\\' as u16 {
            out.push_str("\\\\");
        } else if (0x20..=0x7e).contains(&u) {
            out.push(u as u8 as char);
        } else {
            out.push_str(&format!("\\u{u:04x}"));
        }
    }
    out
}

/// The probe's `hex`: lower-case, no separator.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The probe's `unhex`.
pub fn unhex(s: &str) -> Vec<u8> {
    (0..s.len() / 2)
        .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).expect("hex byte"))
        .collect()
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

/// A float written as `Integer.toHexString(Float.floatToRawIntBits(v))`.
pub fn f32f(field: &str) -> f32 {
    f32::from_bits(
        u32::from_str_radix(field, 16).unwrap_or_else(|_| panic!("not f32 bits: {field:?}")),
    )
}

/// A double written as `Long.toHexString(Double.doubleToRawLongBits(v))`.
pub fn f64f(field: &str) -> f64 {
    f64::from_bits(
        u64::from_str_radix(field, 16).unwrap_or_else(|_| panic!("not f64 bits: {field:?}")),
    )
}

/// The probe's spelling of a float's bits.
pub fn hex32(v: f32) -> String {
    format!("{:x}", v.to_bits())
}

/// The probe's spelling of a double's bits.
pub fn hex64(v: f64) -> String {
    format!("{:x}", v.to_bits())
}
