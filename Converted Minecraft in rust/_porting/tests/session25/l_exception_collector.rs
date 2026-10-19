//! Session 25 sub-batch L: ExceptionCollector and the throwable suppression behavior it uses.

use crate::sweep_probe::*;
use minecraft_rust::javacompat::throwable::{JavaThrowable, ThrowableRef};
use minecraft_rust::net::minecraft::util::ExceptionCollector::ExceptionCollector;
use std::sync::Arc;

fn same(left: &ThrowableRef, right: &ThrowableRef) -> bool {
    Arc::ptr_eq(left, right)
}

fn state(collector: &ExceptionCollector, expected: Option<&ThrowableRef>) -> bool {
    match (collector.throw_if_present(), expected) {
        (Ok(()), None) => true,
        (Err(actual), Some(expected)) => same(&actual, expected),
        _ => false,
    }
}

#[test]
fn exception_collector_keeps_the_first_object_and_suppresses_later_ones_in_order() {
    for row in rows("S25_EXCCOLL") {
        if row[1] == "empty" {
            let mut collector = ExceptionCollector::new();
            assert!(collector.add(None).is_ok());
            assert!(state(&collector, None));
            assert!(state(&collector, None));
        }
    }

    let mut collector = ExceptionCollector::new();
    let primary = JavaThrowable::new("p.CollectorThrowable", Some("primary".to_string()));
    let first = JavaThrowable::new("p.CollectorThrowable", Some("first suppressed".to_string()));
    let second = JavaThrowable::new(
        "p.CollectorThrowable",
        Some("second suppressed".to_string()),
    );
    assert!(collector.add(Some(primary.clone())).is_ok());
    assert!(collector.add(Some(first.clone())).is_ok());
    assert!(collector.add(Some(second.clone())).is_ok());
    let suppressed = primary.suppressed();
    let row = rows("S25_EXCCOLL")
        .into_iter()
        .find(|row| row[1] == "ordered")
        .unwrap();
    assert_eq!(state(&collector, Some(&primary)).to_string(), row[2]);
    assert_eq!(state(&collector, Some(&primary)).to_string(), row[3]);
    assert_eq!(suppressed.len().to_string(), row[4]);
    assert_eq!(same(&suppressed[0], &first).to_string(), row[5]);
    assert_eq!(same(&suppressed[1], &second).to_string(), row[6]);
    assert_eq!(suppressed[0].message().unwrap(), unesc(row[7]).unwrap());
    assert_eq!(suppressed[1].message().unwrap(), unesc(row[8]).unwrap());
}

#[test]
fn exception_collector_matches_null_self_and_suppression_disabled_edges() {
    let rows = rows("S25_EXCCOLL");
    let mut null_first = ExceptionCollector::new();
    assert!(null_first.add(None).is_ok());
    let later = JavaThrowable::new("p.CollectorThrowable", Some("after null".to_string()));
    assert!(null_first.add(Some(later.clone())).is_ok());
    let row = rows.iter().find(|row| row[1] == "null-first").unwrap();
    assert_eq!(state(&null_first, Some(&later)).to_string(), row[2]);
    assert_eq!(later.suppressed().len().to_string(), row[3]);

    let mut null_suppressed = ExceptionCollector::new();
    let primary = JavaThrowable::new("p.CollectorThrowable", Some("null target".to_string()));
    assert!(null_suppressed.add(Some(primary.clone())).is_ok());
    let error = null_suppressed.add(None).unwrap_err();
    let null_result = format!(
        "{}|{}|{}",
        error.class_name(),
        esc(error.message().unwrap()),
        error.cause().is_none()
    );
    let row = rows.iter().find(|row| row[1] == "null-suppressed").unwrap();
    assert_eq!(null_result, row[2]);
    assert_eq!(primary.suppressed().len().to_string(), row[3]);
    assert_eq!(state(&null_suppressed, Some(&primary)).to_string(), row[4]);

    let mut self_suppressed = ExceptionCollector::new();
    let same_object = JavaThrowable::new("p.CollectorThrowable", Some("self target".to_string()));
    assert!(self_suppressed.add(Some(same_object.clone())).is_ok());
    let error = self_suppressed.add(Some(same_object.clone())).unwrap_err();
    let self_result = format!(
        "{}|{}|{}",
        error.class_name(),
        esc(error.message().unwrap()),
        error
            .cause()
            .as_ref()
            .is_some_and(|c| same(c, &same_object))
    );
    let row = rows.iter().find(|row| row[1] == "self").unwrap();
    assert_eq!(self_result, row[2]);
    assert_eq!(same_object.suppressed().len().to_string(), row[3]);
    assert_eq!(
        state(&self_suppressed, Some(&same_object)).to_string(),
        row[4]
    );

    let mut disabled = ExceptionCollector::new();
    let no_suppression = JavaThrowable::with_options(
        "p.CollectorThrowable",
        Some("disabled".to_string()),
        None,
        false,
    );
    assert!(disabled.add(Some(no_suppression.clone())).is_ok());
    assert!(disabled
        .add(Some(JavaThrowable::new(
            "p.CollectorThrowable",
            Some("ignored".to_string()),
        )))
        .is_ok());
    let row = rows.iter().find(|row| row[1] == "disabled").unwrap();
    assert_eq!("ok", row[2]);
    assert_eq!(no_suppression.suppressed().len().to_string(), row[3]);
    assert_eq!(state(&disabled, Some(&no_suppression)).to_string(), row[4]);

    let mut disabled_self = ExceptionCollector::new();
    let disabled_object = JavaThrowable::with_options(
        "p.CollectorThrowable",
        Some("disabled self".to_string()),
        None,
        false,
    );
    assert!(disabled_self.add(Some(disabled_object.clone())).is_ok());
    let error = disabled_self
        .add(Some(disabled_object.clone()))
        .unwrap_err();
    let got = format!(
        "{}|{}|{}",
        error.class_name(),
        esc(error.message().unwrap()),
        error
            .cause()
            .as_ref()
            .is_some_and(|c| same(c, &disabled_object))
    );
    let row = rows.iter().find(|row| row[1] == "disabled-self").unwrap();
    assert_eq!(got, row[2]);
    assert_eq!(disabled_object.suppressed().len().to_string(), row[3]);
    assert_eq!(
        state(&disabled_self, Some(&disabled_object)).to_string(),
        row[4]
    );
}

#[test]
fn exception_collector_declared_members_match_the_26_2_jar() {
    let mut want: Vec<String> = rows("S25_API")
        .iter()
        .filter(|row| row[1] == "ExceptionCollector")
        .map(|r| format!("{}\t{}\t{}\t{}", r[1], r[2], r[3], r[4]))
        .collect();
    let mut have = MEMBERS
        .iter()
        .map(|m| format!("{}\t{}\t{}\t{}", m.0, m.1, m.2, m.3))
        .collect::<Vec<_>>();
    want.sort();
    have.sort();
    assert_eq!(
        have, want,
        "ExceptionCollector declarations differ from the jar"
    );
    let got: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|row| row[1] == "ExceptionCollector")
        .map(|r| (r[1].to_string(), r[2].to_string(), r[3].to_string()))
        .collect();
    assert_eq!(
        got,
        vec![(
            "ExceptionCollector".to_string(),
            "-".to_string(),
            "class".to_string()
        )]
    );
}

const MEMBERS: &[(&str, &str, &str, &str)] = &[
    ("ExceptionCollector", "ctor", "<init>()", "public"),
    ("ExceptionCollector", "field", "result:T", "private"),
    ("ExceptionCollector", "method", "add(T)void", "public"),
    (
        "ExceptionCollector",
        "method",
        "throwIfPresent()void",
        "public",
    ),
];
