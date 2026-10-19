//! Session 25 sub-batch P: UUIDLookup's generic nullable result contract.

use crate::sweep_probe::*;
use minecraft_rust::javacompat::java_util::Uuid;
use minecraft_rust::net::minecraft::world::level::entity::UUIDLookup::UUIDLookup;
use minecraft_rust::net::minecraft::world::level::entity::UniquelyIdentifyable::UniquelyIdentifyable;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Identified {
    uuid: Uuid,
    removed: bool,
}

impl UniquelyIdentifyable for Identified {
    fn get_uuid(&self) -> Option<Uuid> {
        Some(self.uuid)
    }

    fn is_removed(&self) -> bool {
        self.removed
    }
}

struct Lookup;

impl UUIDLookup<Identified> for Lookup {
    fn lookup(&self, uuid: Uuid) -> Option<Identified> {
        let expected = Uuid::from_bits(0x0123_4567_89ab_cdef, -0x0123_4567_89ab_cdf0);
        (uuid == expected).then_some(Identified {
            uuid: expected,
            removed: false,
        })
    }
}

#[test]
fn uuid_lookup_returns_the_matching_value_and_preserves_nullable_misses() {
    let lookup = Lookup;
    let expected = Uuid::from_bits(0x0123_4567_89ab_cdef, -0x0123_4567_89ab_cdf0);
    let hit = lookup.lookup(expected).unwrap();
    assert_eq!(hit.get_uuid(), Some(expected));
    assert!(!hit.is_removed());
    assert!(lookup.lookup(Uuid::from_bits(0, 0)).is_none());

    let rows = rows("S25_UUIDLOOKUP");
    assert_eq!(rows.iter().find(|row| row[1] == "hit").unwrap()[2], "true");
    assert_eq!(rows.iter().find(|row| row[1] == "miss").unwrap()[2], "true");
}

#[test]
fn uuid_lookup_generic_bound_and_method_declarations_match_the_jar() {
    let api = rows("S25_API");
    let members: Vec<String> = api
        .iter()
        .filter(|row| row[1] == "UUIDLookup")
        .map(|row| format!("{}\t{}\t{}\t{}", row[1], row[2], row[3], row[4]))
        .collect();
    assert_eq!(
        members,
        vec!["UUIDLookup\tmethod\tlookup(java.util.UUID)IdentifiedType\tpublic abstract"]
    );
    let supers: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|row| row[1] == "UUIDLookup")
        .map(|row| (row[1].to_string(), row[2].to_string(), row[3].to_string()))
        .collect();
    assert_eq!(
        supers,
        vec![(
            "UUIDLookup".to_string(),
            "-".to_string(),
            "interface".to_string()
        )]
    );
}
