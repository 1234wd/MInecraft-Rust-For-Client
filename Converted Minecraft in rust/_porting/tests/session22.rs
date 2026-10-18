//! Session 22: the small-file sweep. Checks ports against `_porting/test-data/small_probe.txt`,
//! written by `_porting/java-oracle/src/p/SmallProbe.java` against the real 26.2 jar on JDK 25.
//!
//! Every expectation here is a probe row, not a value read off the Java source. Where a test
//! builds its own input it builds the SAME input the probe built, case for case, and the probe's
//! javadoc says so from the other side.
//!
//! # The guard
//!
//! [`every_probe_group_is_claimed`] fails if `small_probe.txt` holds a group that no test in this
//! target reads, or if a test reads a group the file does not hold. It finds the reads by scanning
//! this target's own source for `rows("...")`, so a probe section added without a test -- the
//! session-19 `nbt_probe.txt` situation, 5,026 rows that nothing read -- cannot reach a green run.

#[path = "support/small_probe.rs"]
mod small_probe;
use small_probe::*;

#[path = "session22/a_exceptions.rs"]
mod a_exceptions;
#[path = "session22/b_records.rs"]
mod b_records;
#[path = "session22/c_records.rs"]
mod c_records;
#[path = "session22/d_state.rs"]
mod d_state;
#[path = "session22/e_state2.rs"]
mod e_state2;
#[path = "session22/f_interfaces.rs"]
mod f_interfaces;
#[path = "session22/g_api.rs"]
mod g_api;
#[path = "session22/h_misc.rs"]
mod h_misc;
#[path = "session22/i_more.rs"]
mod i_more;
#[path = "session22/j_last.rs"]
mod j_last;

/// The source of every module in this target, for the guard's `rows("...")` scan.
const SOURCES: &[&str] = &[
    include_str!("session22.rs"),
    include_str!("session22/a_exceptions.rs"),
    include_str!("session22/b_records.rs"),
    include_str!("session22/c_records.rs"),
    include_str!("session22/d_state.rs"),
    include_str!("session22/e_state2.rs"),
    include_str!("session22/f_interfaces.rs"),
    include_str!("session22/g_api.rs"),
    include_str!("session22/h_misc.rs"),
    include_str!("session22/i_more.rs"),
    include_str!("session22/j_last.rs"),
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
    // `rows("...")` in this function's own doc text is not a claim.
    claimed.remove("...");
    let present: std::collections::BTreeSet<String> = kinds().into_iter().map(str::to_string).collect();
    let unread: Vec<&String> = present.difference(&claimed).collect();
    let missing: Vec<&String> = claimed.difference(&present).collect();
    assert!(unread.is_empty(), "small_probe.txt groups that no session22 test reads: {unread:?}");
    assert!(missing.is_empty(), "session22 reads groups small_probe.txt does not have: {missing:?}");
}
