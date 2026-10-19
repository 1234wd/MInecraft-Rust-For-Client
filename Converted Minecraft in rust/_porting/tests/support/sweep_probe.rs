//! Reader for `_porting/test-data/sweep_probe.txt`, written by
//! `_porting/java-oracle/src/p/SweepProbe.java` against the real 26.2 jar on JDK 25.
//!
//! Session 24. A module, not a test target: `tests/session24.rs` includes it with
//! `#[path = "support/sweep_probe.rs"]`. The line format is `small_probe.txt`'s (session 22), so
//! the field decoders are that module's, re-exported here; only the file differs.
//!
//! An exception is written by the probe as `EX:` followed by its escaped `toString()`, which is
//! exactly the text a port's panic carries.

#![allow(dead_code)]

use std::collections::BTreeSet;
use std::sync::OnceLock;

#[allow(unused_imports)]
pub use crate::small_probe::{
    boolf, f32bits, f32f, f64bits, f64f, hex32, hex64, i32f, i64f, nan_canon, s, unesc,
};

fn text() -> &'static str {
    static T: OnceLock<String> = OnceLock::new();
    T.get_or_init(|| {
        let path = minecraft_rust::javacompat::golden::test_data_dir().join("sweep_probe.txt");
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
/// group name (or a probe section that silently wrote nothing) cannot pass as "no rows to check".
pub fn rows(kind: &str) -> Vec<Vec<&'static str>> {
    let rows: Vec<Vec<&'static str>> = all_rows().filter(|f| f[0] == kind).collect();
    assert!(!rows.is_empty(), "sweep_probe.txt has no `{kind}` rows");
    rows
}

/// The distinct group names in the file.
pub fn kinds() -> BTreeSet<&'static str> {
    all_rows().map(|f| f[0]).collect()
}

/// The text of a panic raised by `f`, for comparing with a Java exception's `toString()`.
pub fn panic_text<R>(f: impl FnOnce() -> R) -> String {
    let err = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f))
        .err()
        .expect("expected the call to panic, as Java throws");
    if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        panic!("panic payload is not a string")
    }
}

/// `f()` rendered the probe's way: its result through `show`, or `EX:` plus the (escaped) panic
/// text when it panics.
pub fn or_ex<R>(f: impl FnOnce() -> R, show: impl FnOnce(R) -> String) -> String {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(v) => show(v),
        Err(err) => {
            let text = if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else {
                panic!("panic payload is not a string")
            };
            format!("EX:{}", esc(&text))
        }
    }
}

/// The probe's `esc`: backslash doubled, chars outside `0x20..=0x7E` as `\uXXXX` UTF-16 units.
pub fn esc(s: &str) -> String {
    let mut out = String::new();
    for unit in s.encode_utf16() {
        if unit == u16::from(b'\\') {
            out.push_str("\\\\");
        } else if (0x20..=0x7e).contains(&unit) {
            out.push(char::from(unit as u8));
        } else {
            out.push_str(&format!("\\u{unit:04x}"));
        }
    }
    out
}

/// [`esc`] of a nullable string: Java's `null` is `\N`.
pub fn esc_opt(s: Option<&str>) -> String {
    s.map_or_else(|| "\\N".to_string(), esc)
}

/// Java's `Float.floatToRawIntBits(v)` in the probe's spelling.
pub fn fbits(v: f32) -> String {
    format!("{:x}", v.to_bits())
}

/// Java's `Double.doubleToRawLongBits(v)` in the probe's spelling.
pub fn dbits(v: f64) -> String {
    format!("{:x}", v.to_bits())
}
