//! Session 23: the JDK-order `HashMap`, `CompoundTag` and `ListTag`. Checks ports against
//! `_porting/test-data/nbt_probe.txt`, written by `_porting/java-oracle/src/p/NbtProbe.java`
//! against the real 26.2 jar on JDK 25.
//!
//! Every expectation here is a probe row, not a value read off the Java source. Op scripts are
//! replayed from the rows themselves, so the tests carry no copy of the probe's generators.
//!
//! # The guard
//!
//! `session19_g.rs`'s `unstarted_groups_are_declared` owns the list of `nbt_probe.txt` groups. A
//! group it records as covered by session 23 must be read here: it scans [`SOURCES`] for
//! `rows("...")`, and [`claimed_groups_exist`] checks the other direction.

#[path = "support/nbt_probe.rs"]
mod nbt_probe;
use nbt_probe::*;

#[path = "session23/a_hashmap.rs"]
mod a_hashmap;

/// The source of every module in this target, for the `rows("...")` scans.
pub const SOURCES: &[&str] = &[
    include_str!("session23.rs"),
    include_str!("session23/a_hashmap.rs"),
];

/// Every group this target reads, found by scanning [`SOURCES`] for `rows("...")`.
pub fn claimed() -> std::collections::BTreeSet<String> {
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
    claimed
}

#[test]
fn claimed_groups_exist() {
    let present: std::collections::BTreeSet<String> =
        kinds().into_iter().map(str::to_string).collect();
    let missing: Vec<String> = claimed().difference(&present).cloned().collect();
    assert!(
        missing.is_empty(),
        "session23 reads groups nbt_probe.txt does not have: {missing:?}"
    );
}
