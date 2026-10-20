use crate::sweep_probe::rows;
use minecraft_rust::com::mojang::realmsclient::dto::ReflectionBasedSerialization::ReflectionBasedSerialization;

struct MarkedDto;
impl ReflectionBasedSerialization for MarkedDto {}

fn accepts_marker<T: ReflectionBasedSerialization>() {}

#[test]
fn reflection_serialization_is_an_empty_marker_interface() {
    accepts_marker::<MarkedDto>();
    assert!(!rows("S26_API")
        .iter()
        .any(|row| row[1] == "ReflectionBasedSerialization"));
    let shape = rows("S26_SUP")
        .into_iter()
        .find(|row| row[1] == "ReflectionBasedSerialization")
        .unwrap();
    assert_eq!(&shape[2..], ["-", "interface"]);
}
