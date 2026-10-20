//! Session 26: java.time value behavior used by Minecraft's JFR tick-time record.

#[path = "support/small_probe.rs"]
mod small_probe;
#[path = "support/sweep_probe.rs"]
mod sweep_probe;

#[path = "session26/a_timed_stat.rs"]
mod a_timed_stat;
#[path = "session26/b_frame_pass.rs"]
mod b_frame_pass;
#[path = "session26/c_reflection_serialization.rs"]
mod c_reflection_serialization;

#[test]
fn every_session26_probe_group_is_claimed() {
    let source = concat!(
        include_str!("session26/a_timed_stat.rs"),
        include_str!("session26/b_frame_pass.rs"),
        include_str!("session26/c_reflection_serialization.rs"),
    );
    let mut claimed = std::collections::BTreeSet::new();
    let mut rest = source;
    while let Some(at) = rest.find("rows(\"") {
        rest = &rest[at + 6..];
        let end = rest.find('"').expect("unterminated rows(\"...\") literal");
        claimed.insert(rest[..end].to_string());
        rest = &rest[end..];
    }
    let present: std::collections::BTreeSet<String> = sweep_probe::kinds()
        .into_iter()
        .filter(|kind| kind.starts_with("S26_"))
        .map(str::to_string)
        .collect();
    let unread: Vec<&String> = present.difference(&claimed).collect();
    let missing: Vec<&String> = claimed.difference(&present).collect();
    assert!(
        unread.is_empty(),
        "unread Session 26 probe groups: {unread:?}"
    );
    assert!(
        missing.is_empty(),
        "Session 26 tests claim absent groups: {missing:?}"
    );
}
