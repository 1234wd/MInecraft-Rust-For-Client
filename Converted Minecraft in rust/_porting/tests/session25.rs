//! Session 25: fastutil double-list support and three small voxel-shape types.
//!
//! The Java expectations are the `S25_*` rows in `sweep_probe.txt`, generated against the
//! Minecraft 26.2 jar with fastutil 8.5.18. Session 24's target ignores only these prefixed rows;
//! this target owns them and checks that every group is consumed.

#[path = "support/small_probe.rs"]
mod small_probe;
#[path = "support/sweep_probe.rs"]
mod sweep_probe;

#[path = "session25/f_fastutil.rs"]
mod f_fastutil;
#[path = "session25/g_discrete_cube.rs"]
mod g_discrete_cube;
#[path = "session25/h_divisor.rs"]
mod h_divisor;
#[path = "session25/i_common_links.rs"]
mod i_common_links;
#[path = "session25/k_uniquely_identifyable.rs"]
mod k_uniquely_identifyable;
#[path = "session25/l_exception_collector.rs"]
mod l_exception_collector;
#[path = "session25/m_filesystem_capabilities.rs"]
mod m_filesystem_capabilities;
#[path = "session25/n_crash_report_detail.rs"]
mod n_crash_report_detail;
#[path = "session25/o_jfr_stats.rs"]
mod o_jfr_stats;
#[path = "session25/p_uuid_lookup.rs"]
mod p_uuid_lookup;

#[test]
fn every_session25_probe_group_is_claimed() {
    let source = concat!(
        include_str!("session25/f_fastutil.rs"),
        include_str!("session25/g_discrete_cube.rs"),
        include_str!("session25/h_divisor.rs"),
        include_str!("session25/i_common_links.rs"),
        include_str!("session25/k_uniquely_identifyable.rs"),
        include_str!("session25/l_exception_collector.rs"),
        include_str!("session25/m_filesystem_capabilities.rs"),
        include_str!("session25/n_crash_report_detail.rs"),
        include_str!("session25/o_jfr_stats.rs"),
        include_str!("session25/p_uuid_lookup.rs")
    );
    let mut claimed = std::collections::BTreeSet::new();
    let mut rest = source;
    while let Some(at) = rest.find("rows(\"") {
        rest = &rest[at + 6..];
        let end = rest.find('\"').expect("unterminated rows(\"...\") literal");
        claimed.insert(rest[..end].to_string());
        rest = &rest[end..];
    }
    let present: std::collections::BTreeSet<String> = sweep_probe::kinds()
        .into_iter()
        .filter(|kind| kind.starts_with("S25_"))
        .map(str::to_string)
        .collect();
    let unread: Vec<&String> = present.difference(&claimed).collect();
    let missing: Vec<&String> = claimed.difference(&present).collect();
    assert!(
        unread.is_empty(),
        "unread Session 25 probe groups: {unread:?}"
    );
    assert!(
        missing.is_empty(),
        "Session 25 tests claim absent groups: {missing:?}"
    );
}
