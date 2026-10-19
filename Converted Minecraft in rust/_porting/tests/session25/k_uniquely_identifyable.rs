//! Session 25 sub-batch K: the UUID-backed identity interface.

use crate::sweep_probe::*;
use minecraft_rust::javacompat::java_util::Uuid;
use minecraft_rust::net::minecraft::world::level::entity::UniquelyIdentifyable::UniquelyIdentifyable;

struct Entity {
    uuid: Option<Uuid>,
    removed: bool,
}

impl UniquelyIdentifyable for Entity {
    fn get_uuid(&self) -> Option<Uuid> {
        self.uuid
    }

    fn is_removed(&self) -> bool {
        self.removed
    }
}

#[test]
fn uniquely_identifyable_declaration_matches_the_jar_and_allows_nullable_uuid_results() {
    let mut want: Vec<String> = rows("S25_API")
        .iter()
        .filter(|row| row[1] == "UniquelyIdentifyable")
        .map(|r| format!("{}\t{}\t{}\t{}", r[1], r[2], r[3], r[4]))
        .collect();
    let have = vec![
        "UniquelyIdentifyable\tmethod\tgetUUID()java.util.UUID\tpublic abstract".to_string(),
        "UniquelyIdentifyable\tmethod\tisRemoved()boolean\tpublic abstract".to_string(),
    ];
    want.sort();
    assert_eq!(
        have, want,
        "UniquelyIdentifyable declarations differ from the jar"
    );
    let got: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|row| row[1] == "UniquelyIdentifyable")
        .map(|r| (r[1].to_string(), r[2].to_string(), r[3].to_string()))
        .collect();
    assert_eq!(
        got,
        vec![(
            "UniquelyIdentifyable".to_string(),
            "-".to_string(),
            "interface".to_string()
        )]
    );

    let entity = Entity {
        uuid: Some(Uuid::from_bits(
            0x0123_4567_89ab_cdef,
            0xfedc_ba98_7654_3210u64 as i64,
        )),
        removed: true,
    };
    assert_eq!(
        entity.get_uuid().unwrap().to_undashed_string(),
        "0123456789abcdeffedcba9876543210"
    );
    assert!(entity.is_removed());
    let nullable = Entity {
        uuid: None,
        removed: false,
    };
    assert!(nullable.get_uuid().is_none());
    assert!(!nullable.is_removed());
}
