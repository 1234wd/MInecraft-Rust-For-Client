//! Session 25 sub-batch N: the generic CrashReportDetail -> Callable contract.

use crate::sweep_probe::*;
use minecraft_rust::javacompat::java_concurrent::Callable;
use minecraft_rust::javacompat::throwable::{JavaThrowable, ThrowableRef};
use minecraft_rust::net::minecraft::CrashReportDetail::CrashReportDetail;

struct Detail(String);

impl Callable<String> for Detail {
    fn call(&self) -> Result<String, ThrowableRef> {
        Ok(self.0.clone())
    }
}

impl CrashReportDetail<String> for Detail {}

#[test]
fn crash_report_detail_inherits_the_generic_callable_contract() {
    fn invoke<T: CrashReportDetail<V>, V>(detail: &T) -> Result<V, ThrowableRef> {
        Callable::call(detail)
    }

    let report = invoke(&Detail("report detail".to_string()))
        .unwrap_or_else(|error| panic!("unexpected throwable: {}", error.java_to_string()));
    assert_eq!(report, "report detail");
    let failure = JavaThrowable::new(
        "java.lang.IllegalStateException",
        Some("failed".to_string()),
    );
    struct FailingDetail(ThrowableRef);
    impl Callable<()> for FailingDetail {
        fn call(&self) -> Result<(), ThrowableRef> {
            Err(self.0.clone())
        }
    }
    impl CrashReportDetail<()> for FailingDetail {}
    let thrown = invoke(&FailingDetail(failure.clone())).unwrap_err();
    assert!(std::sync::Arc::ptr_eq(&thrown, &failure));
}

#[test]
fn crash_report_detail_declares_no_members_and_extends_callable_v() {
    assert!(rows("S25_API")
        .iter()
        .all(|row| row[1] != "CrashReportDetail"));
    let supertypes: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|row| row[1] == "CrashReportDetail")
        .map(|row| (row[1].to_string(), row[2].to_string(), row[3].to_string()))
        .collect();
    assert_eq!(
        supertypes,
        vec![(
            "CrashReportDetail".to_string(),
            "java.util.concurrent.Callable<V>".to_string(),
            "interface".to_string(),
        )]
    );
}
