//! Session 25 sub-batch I: fixed Java URIs, UUID formatting, and CommonLinks URL assembly.

use crate::sweep_probe::*;
use minecraft_rust::javacompat::java_util::Uuid;
use minecraft_rust::net::minecraft::util::CommonLinks::CommonLinks;

fn uuid(index: i32) -> Option<Uuid> {
    let bits = [
        (0, 0),
        (-1, -1),
        (i64::MIN, i64::MIN),
        (i64::MAX, i64::MAX),
        (0x0123_4567_89ab_cdef, 0xfedc_ba98_7654_3210u64 as i64),
        (1, 1),
    ];
    (index >= 0).then(|| Uuid::from_bits(bits[index as usize].0, bits[index as usize].1))
}

#[test]
fn common_links_constants_match_java_uri_and_string_fields() {
    for row in rows("S25_LINK") {
        let (java_type, value) = match row[1] {
            "GDPR" => ("java.net.URI", CommonLinks::GDPR.as_str()),
            "EULA" => ("java.net.URI", CommonLinks::EULA.as_str()),
            "PRIVACY_STATEMENT" => ("java.net.URI", CommonLinks::PRIVACY_STATEMENT.as_str()),
            "ATTRIBUTION" => ("java.net.URI", CommonLinks::ATTRIBUTION.as_str()),
            "LICENSES" => ("java.net.URI", CommonLinks::LICENSES.as_str()),
            "BUY_MINECRAFT_JAVA" => ("java.net.URI", CommonLinks::BUY_MINECRAFT_JAVA.as_str()),
            "ACCOUNT_SETTINGS" => ("java.net.URI", CommonLinks::ACCOUNT_SETTINGS.as_str()),
            "SNAPSHOT_FEEDBACK" => ("java.net.URI", CommonLinks::SNAPSHOT_FEEDBACK.as_str()),
            "RELEASE_FEEDBACK" => ("java.net.URI", CommonLinks::RELEASE_FEEDBACK.as_str()),
            "SNAPSHOT_BUGS_FEEDBACK" => {
                ("java.net.URI", CommonLinks::SNAPSHOT_BUGS_FEEDBACK.as_str())
            }
            "GENERAL_HELP" => ("java.net.URI", CommonLinks::GENERAL_HELP.as_str()),
            "ACCESSIBILITY_HELP" => ("java.net.URI", CommonLinks::ACCESSIBILITY_HELP.as_str()),
            "REPORTING_HELP" => ("java.net.URI", CommonLinks::REPORTING_HELP.as_str()),
            "SUSPENSION_HELP" => ("java.net.URI", CommonLinks::SUSPENSION_HELP.as_str()),
            "BLOCKING_HELP" => ("java.net.URI", CommonLinks::BLOCKING_HELP.as_str()),
            "SYMLINK_HELP" => ("java.net.URI", CommonLinks::SYMLINK_HELP.as_str()),
            "PRIVACY_AND_ONLINE_SETTINGS" => (
                "java.net.URI",
                CommonLinks::PRIVACY_AND_ONLINE_SETTINGS.as_str(),
            ),
            "START_REALMS_TRIAL" => ("java.net.URI", CommonLinks::START_REALMS_TRIAL.as_str()),
            "BUY_REALMS" => ("java.net.URI", CommonLinks::BUY_REALMS.as_str()),
            "REALMS_TERMS" => ("java.net.URI", CommonLinks::REALMS_TERMS.as_str()),
            "REALMS_CONTENT_CREATION" => (
                "java.net.URI",
                CommonLinks::REALMS_CONTENT_CREATION.as_str(),
            ),
            "EXTEND_REALMS_LINK" => ("java.lang.String", CommonLinks::EXTEND_REALMS_LINK),
            "INTENTIONAL_GAME_DESIGN_BUG_ID" => (
                "java.lang.String",
                CommonLinks::INTENTIONAL_GAME_DESIGN_BUG_ID,
            ),
            "INTENTIONAL_GAME_DESIGN_BUG" => (
                "java.net.URI",
                CommonLinks::INTENTIONAL_GAME_DESIGN_BUG.as_str(),
            ),
            other => panic!("unhandled CommonLinks constant {other}"),
        };
        assert_eq!(java_type, row[2], "field type for {}", row[1]);
        assert_eq!(
            value,
            unesc(row[3]).expect("constant URI/string is non-null"),
            "constant {}",
            row[1]
        );
    }
}

#[test]
fn java_uuid_bits_and_undashed_spelling_match_authlib() {
    for row in rows("S25_UUID") {
        let value = Uuid::from_bits(i64f(row[1]), i64f(row[2]));
        assert_eq!(value.to_undashed_string(), row[4]);
        assert_eq!(row[3].replace('-', ""), row[4]);
    }
}

#[test]
fn common_links_url_overloads_preserve_null_short_circuit_and_literal_concatenation() {
    let subscriptions = [
        None,
        Some(""),
        Some("realm-123"),
        Some("id &ref?=x"),
        Some("é realm/一"),
    ];
    for row in rows("S25_EXTREALM") {
        let sub_index = i32f(row[1]) as usize;
        let (subscription, profile_id) = (subscriptions[sub_index], uuid(i32f(row[2])));
        let got = [
            or_ex(
                || CommonLinks::extend_realms_basic(subscription, profile_id),
                |value| value,
            ),
            or_ex(
                || CommonLinks::extend_realms(subscription, profile_id, true),
                |value| value,
            ),
            or_ex(
                || CommonLinks::extend_realms(subscription, profile_id, false),
                |value| value,
            ),
        ];
        for index in 0..3 {
            let want = if row[index + 3].starts_with("EX:") {
                row[index + 3].to_string()
            } else {
                unesc(row[index + 3]).expect("URL result is non-null")
            };
            assert_eq!(
                got[index], want,
                "subscription {sub_index}, UUID {}, overload {index}",
                row[2]
            );
        }
    }
}

#[test]
fn common_links_declared_members_and_supertypes_match_the_26_2_jar() {
    let mut want: Vec<String> = rows("S25_API")
        .iter()
        .filter(|row| row[1] == "CommonLinks")
        .map(|r| format!("{}\t{}\t{}\t{}", r[1], r[2], r[3], r[4]))
        .collect();
    let mut have = MEMBERS
        .iter()
        .map(|m| format!("{}\t{}\t{}\t{}", m.0, m.1, m.2, m.3))
        .collect::<Vec<_>>();
    want.sort();
    have.sort();
    assert_eq!(have, want, "CommonLinks declarations differ from the jar");
    let got: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|row| row[1] == "CommonLinks")
        .map(|r| (r[1].to_string(), r[2].to_string(), r[3].to_string()))
        .collect();
    assert_eq!(
        got,
        vec![(
            "CommonLinks".to_string(),
            "-".to_string(),
            "class".to_string()
        )]
    );
}

const MEMBERS: &[(&str, &str, &str, &str)] = &[
    ("CommonLinks", "ctor", "<init>()", "public"),
    (
        "CommonLinks",
        "field",
        "ACCESSIBILITY_HELP:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "ACCOUNT_SETTINGS:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "ATTRIBUTION:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "BLOCKING_HELP:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "BUY_MINECRAFT_JAVA:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "BUY_REALMS:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "EULA:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "EXTEND_REALMS_LINK:java.lang.String",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "GDPR:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "GENERAL_HELP:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "INTENTIONAL_GAME_DESIGN_BUG:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "INTENTIONAL_GAME_DESIGN_BUG_ID:java.lang.String",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "LICENSES:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "PRIVACY_AND_ONLINE_SETTINGS:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "PRIVACY_STATEMENT:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "REALMS_CONTENT_CREATION:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "REALMS_TERMS:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "RELEASE_FEEDBACK:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "REPORTING_HELP:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "SNAPSHOT_BUGS_FEEDBACK:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "SNAPSHOT_FEEDBACK:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "START_REALMS_TRIAL:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "SUSPENSION_HELP:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "field",
        "SYMLINK_HELP:java.net.URI",
        "public static final",
    ),
    (
        "CommonLinks",
        "method",
        "extendRealms(java.lang.String,java.util.UUID)java.lang.String",
        "public static",
    ),
    (
        "CommonLinks",
        "method",
        "extendRealms(java.lang.String,java.util.UUID,boolean)java.lang.String",
        "public static",
    ),
];
