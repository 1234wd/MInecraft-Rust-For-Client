//! Session 25 sub-batch M: FileSystemCapabilities and its narrow JDK NIO option helpers.

use crate::sweep_probe::*;
use minecraft_rust::javacompat::java_nio::{
    CopyOption, CopyOptionValue, LinkOption, OpenOption, StandardCopyOption,
};
use minecraft_rust::net::minecraft::util::filefix::FileSystemCapabilities::FileSystemCapabilities;

#[test]
fn file_system_capabilities_record_matches_constructor_accessors_equality_hash_and_string() {
    for row in rows("S25_FSCAP") {
        let atomic = row[1] == "true";
        let hard_links = row[2] == "true";
        let value = FileSystemCapabilities::new(atomic, hard_links);
        assert_eq!(value.atomic_move().to_string(), row[1]);
        assert_eq!(value.hard_links().to_string(), row[2]);
        assert_eq!(value.hash_code().to_string(), row[3]);
        assert_eq!(esc(&value.to_string()), row[4]);
        assert_eq!(
            (value == FileSystemCapabilities::new(atomic, hard_links)).to_string(),
            row[5]
        );
        assert_eq!(
            (value == FileSystemCapabilities::new(!atomic, hard_links)).to_string(),
            row[6]
        );
    }
}

#[test]
fn move_options_match_jdk_order_and_each_call_allocates_a_fresh_array() {
    for row in rows("S25_FSMOVE") {
        let value = FileSystemCapabilities::new(row[1] == "true", row[2] == "true");
        let first = value.get_move_options();
        let second = value.get_move_options();
        let got = first
            .iter()
            .map(|option| match option {
                CopyOptionValue::Link(value) => {
                    format!("java.nio.file.LinkOption:{value}")
                }
                CopyOptionValue::Standard(value) => {
                    format!("java.nio.file.StandardCopyOption:{value}")
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        assert_eq!(got, row[3]);
        assert_ne!(
            first.as_ptr(),
            second.as_ptr(),
            "Java returns a fresh array each call"
        );
        assert_eq!(first.len().to_string(), row[5]);
        let typed: Vec<CopyOptionValue> = if row[1] == "true" {
            vec![
                StandardCopyOption::ATOMIC_MOVE.into(),
                LinkOption::NOFOLLOW_LINKS.into(),
            ]
        } else {
            vec![LinkOption::NOFOLLOW_LINKS.into()]
        };
        assert_eq!(first, typed);
    }
}

#[test]
fn nio_option_constants_enum_order_and_value_of_edges_match_jdk_25() {
    fn require_copy_option<T: CopyOption>() {}
    fn require_open_option<T: OpenOption>() {}
    require_copy_option::<LinkOption>();
    require_copy_option::<StandardCopyOption>();
    require_open_option::<LinkOption>();

    let nio_rows = rows("S25_NIOENUM");
    let link_values = LinkOption::values()
        .iter()
        .map(|value| format!("{}:{}", value.name(), value.ordinal()))
        .collect::<Vec<_>>()
        .join(",");
    let copy_values = StandardCopyOption::values()
        .iter()
        .map(|value| format!("{}:{}", value.name(), value.ordinal()))
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(
        link_values,
        nio_rows.iter().find(|row| row[1] == "link-values").unwrap()[2]
    );
    assert_eq!(
        copy_values,
        nio_rows.iter().find(|row| row[1] == "copy-values").unwrap()[2]
    );

    for row in nio_rows.iter().filter(|row| row[1] == "link-valueOf") {
        let name = (row[2] != "\\N").then_some(row[2]);
        let got = LinkOption::value_of(name)
            .map(|value| value.name().to_string())
            .unwrap_or_else(|error| format!("EX:{}", esc(&error.java_to_string())));
        assert_eq!(got, row[3]);
    }
    for row in nio_rows.iter().filter(|row| row[1] == "copy-valueOf") {
        let name = (row[2] != "\\N").then_some(row[2]);
        let got = StandardCopyOption::value_of(name)
            .map(|value| value.name().to_string())
            .unwrap_or_else(|error| format!("EX:{}", esc(&error.java_to_string())));
        assert_eq!(got, row[3]);
    }
}

#[test]
fn file_system_capabilities_and_jdk_option_declarations_match_the_jar() {
    let want: Vec<String> = rows("S25_API")
        .iter()
        .filter(|row| {
            matches!(
                row[1],
                "FileSystemCapabilities" | "CopyOption" | "LinkOption" | "StandardCopyOption"
            )
        })
        .map(|row| format!("{}\t{}\t{}\t{}", row[1], row[2], row[3], row[4]))
        .collect();
    let mut want = want;
    want.sort();
    let mut have = MEMBERS
        .iter()
        .map(|member| format!("{}\t{}\t{}\t{}", member.0, member.1, member.2, member.3))
        .collect::<Vec<_>>();
    have.sort();
    assert_eq!(
        have, want,
        "FileSystemCapabilities/NIO declarations differ from the jar"
    );

    let mut got: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|row| {
            matches!(
                row[1],
                "FileSystemCapabilities" | "CopyOption" | "LinkOption" | "StandardCopyOption"
            )
        })
        .map(|row| (row[1].to_string(), row[2].to_string(), row[3].to_string()))
        .collect();
    got.sort();
    let mut expected = SUPERTYPES
        .iter()
        .map(|entry| {
            (
                entry.0.to_string(),
                entry.1.to_string(),
                entry.2.to_string(),
            )
        })
        .collect::<Vec<_>>();
    expected.sort();
    assert_eq!(got, expected);
}

const MEMBERS: &[(&str, &str, &str, &str)] = &[
    (
        "FileSystemCapabilities",
        "ctor",
        "<init>(boolean,boolean)",
        "public",
    ),
    (
        "FileSystemCapabilities",
        "field",
        "atomicMove:boolean",
        "private final",
    ),
    (
        "FileSystemCapabilities",
        "field",
        "hardLinks:boolean",
        "private final",
    ),
    (
        "FileSystemCapabilities",
        "method",
        "atomicMove()boolean",
        "public",
    ),
    (
        "FileSystemCapabilities",
        "method",
        "equals(java.lang.Object)boolean",
        "public",
    ),
    (
        "FileSystemCapabilities",
        "method",
        "getMoveOptions()java.nio.file.CopyOption[]",
        "public",
    ),
    (
        "FileSystemCapabilities",
        "method",
        "hardLinks()boolean",
        "public",
    ),
    (
        "FileSystemCapabilities",
        "method",
        "hashCode()int",
        "public",
    ),
    (
        "FileSystemCapabilities",
        "method",
        "toString()java.lang.String",
        "public",
    ),
    ("LinkOption", "ctor", "<init>()", "private"),
    (
        "LinkOption",
        "field",
        "NOFOLLOW_LINKS:java.nio.file.LinkOption",
        "public static final",
    ),
    (
        "LinkOption",
        "method",
        "valueOf(java.lang.String)java.nio.file.LinkOption",
        "public static",
    ),
    (
        "LinkOption",
        "method",
        "values()java.nio.file.LinkOption[]",
        "public static",
    ),
    ("StandardCopyOption", "ctor", "<init>()", "private"),
    (
        "StandardCopyOption",
        "field",
        "ATOMIC_MOVE:java.nio.file.StandardCopyOption",
        "public static final",
    ),
    (
        "StandardCopyOption",
        "field",
        "COPY_ATTRIBUTES:java.nio.file.StandardCopyOption",
        "public static final",
    ),
    (
        "StandardCopyOption",
        "field",
        "REPLACE_EXISTING:java.nio.file.StandardCopyOption",
        "public static final",
    ),
    (
        "StandardCopyOption",
        "method",
        "valueOf(java.lang.String)java.nio.file.StandardCopyOption",
        "public static",
    ),
    (
        "StandardCopyOption",
        "method",
        "values()java.nio.file.StandardCopyOption[]",
        "public static",
    ),
];

const SUPERTYPES: &[(&str, &str, &str)] = &[
    ("CopyOption", "-", "interface"),
    ("FileSystemCapabilities", "extends java.lang.Record", "record"),
    (
        "LinkOption",
        "java.nio.file.OpenOption,java.nio.file.CopyOption,extends java.lang.Enum<java.nio.file.LinkOption>",
        "enum",
    ),
    (
        "StandardCopyOption",
        "java.nio.file.CopyOption,extends java.lang.Enum<java.nio.file.StandardCopyOption>",
        "enum",
    ),
];
