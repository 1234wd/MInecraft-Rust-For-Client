//! Session 24: the small-file conversion sweep. Checks ports against
//! `_porting/test-data/sweep_probe.txt`, written by `_porting/java-oracle/src/p/SweepProbe.java`
//! against the real 26.2 jar on JDK 25.
//!
//! Every expectation here is a probe row, not a value read off the Java source. Where a test
//! builds its own input it builds the SAME input the probe built, and the probe says so from the
//! other side. Scripts (sequences of operations) are carried IN the rows and replayed from them,
//! so the two sides cannot drift.
//!
//! # The guard
//!
//! [`every_probe_group_is_claimed`] fails if `sweep_probe.txt` holds a group that no test in this
//! target reads, or if a test reads a group the file does not hold. It finds the reads by scanning
//! this target's own source for `rows("...")`.

#[path = "support/small_probe.rs"]
mod small_probe;
#[path = "support/sweep_probe.rs"]
mod sweep_probe;

#[path = "session24/a_first.rs"]
mod a_first;
#[path = "session24/b_loggers.rs"]
mod b_loggers;
#[path = "session24/c_small.rs"]
mod c_small;
#[path = "session24/d_guava.rs"]
mod d_guava;
#[path = "session24/e_blaze.rs"]
mod e_blaze;
#[path = "session24/z_api.rs"]
mod z_api;

/// The source of every module in this target, for the guard's `rows("...")` scan.
const SOURCES: &[&str] = &[
    include_str!("session24.rs"),
    include_str!("session24/a_first.rs"),
    include_str!("session24/b_loggers.rs"),
    include_str!("session24/c_small.rs"),
    include_str!("session24/d_guava.rs"),
    include_str!("session24/e_blaze.rs"),
    include_str!("session24/z_api.rs"),
];

#[test]
fn every_probe_group_is_claimed() {
    let mut claimed = std::collections::BTreeSet::new();
    for src in SOURCES {
        let mut rest = *src;
        while let Some(at) = rest.find("rows(\"") {
            rest = &rest[at + 6..];
            let end = rest.find('"').expect("unterminated rows(\"...\") literal");
            claimed.insert(rest[..end].to_string());
            rest = &rest[end..];
        }
    }
    // `rows("...")` in this file's own doc text is not a claim.
    claimed.remove("...");
    let present: std::collections::BTreeSet<String> = sweep_probe::kinds()
        .into_iter()
        .map(str::to_string)
        .collect();
    let unread: Vec<&String> = present.difference(&claimed).collect();
    let missing: Vec<&String> = claimed.difference(&present).collect();
    assert!(
        unread.is_empty(),
        "sweep_probe.txt groups that no session24 test reads: {unread:?}"
    );
    assert!(
        missing.is_empty(),
        "session24 reads groups sweep_probe.txt does not have: {missing:?}"
    );
}

#[test]
fn esc_matches_the_probe_spelling() {
    use sweep_probe::{esc, esc_opt};
    assert_eq!(esc("a\\b"), "a\\\\b");
    assert_eq!(esc("tab\there"), "tab\\u0009here");
    assert_eq!(esc("\u{e9}\u{1F600}"), "\\u00e9\\ud83d\\ude00");
    assert_eq!(esc_opt(None), "\\N");
}
