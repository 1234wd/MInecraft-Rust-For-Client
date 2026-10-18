//! Sub-batch C: the `net.minecraft.nbt.visitors` package -- `FieldTree`, `CollectToTag`,
//! `CollectFields`, `SkipFields`.
//!
//! Besides the `PW`/`PU`/`CF<n>`/`SF<n>` sub-rows of every `GEN`/`CASE` tree (checked in
//! `c_trees.rs`), three session-23 groups pin them directly:
//!
//! * `FTREE`: the `FieldTree` each selector set builds (the probe's three and four more: a name
//!   selected twice with different types, a two-level path, no selectors, whole containers), in a
//!   portable dump, and `isSelected` over an 8-type x 12-id grid at the root;
//! * `CFX`: `CollectFields` (result and missing count) and `SkipFields` over four crafted trees
//!   for those four extra sets;
//! * `CFXRAW`: the same over hand-written streams with a repeated key, a list root, a repeated
//!   selected field.

use crate::b_support::*;
use crate::nbt_probe::*;

use minecraft_rust::javacompat::java_lang;
use minecraft_rust::net::minecraft::nbt::visitors::CollectFields::CollectFields;
use minecraft_rust::net::minecraft::nbt::visitors::FieldTree::FieldTree;
use minecraft_rust::net::minecraft::nbt::visitors::SkipFields::SkipFields;
use minecraft_rust::net::minecraft::nbt::NbtAccounter::NbtAccounter;
use minecraft_rust::net::minecraft::nbt::TagType::TagTypeRef;
use minecraft_rust::net::minecraft::nbt::{
    ByteTag, CompoundTag, DoubleTag, IntTag, ListTag, LongTag, ShortTag, StringTag,
};

/// `java.util.Collections.sort` of strings (`String.compareTo`), then `List.toString`.
fn sorted_list(mut items: Vec<String>) -> String {
    items.sort_by(|a, b| java_lang::string_compare_to(a, b).cmp(&0));
    format!("[{}]", items.join(", "))
}

/// The probe's `ftree(FieldTree)`.
fn ftree(t: &FieldTree) -> String {
    let sel: Vec<String> = t
        .selected_fields()
        .iter()
        .map(|(k, ty)| format!("{}:{}", esc(k), ty.get_name()))
        .collect();
    let rec: Vec<String> = t
        .fields_to_recurse()
        .iter()
        .map(|(k, child)| format!("{}->{}", esc(k), ftree(child)))
        .collect();
    format!(
        "(d={} sel={} rec={})",
        t.depth(),
        sorted_list(sel),
        sorted_list(rec)
    )
}

#[test]
fn field_trees_match_the_jar() {
    let mut all = selectors();
    all.extend(extra_selectors());
    let types: [TagTypeRef; 8] = [
        IntTag::TYPE,
        StringTag::TYPE,
        ByteTag::TYPE,
        ListTag::TYPE,
        CompoundTag::TYPE,
        LongTag::TYPE,
        DoubleTag::TYPE,
        ShortTag::TYPE,
    ];
    let ids = [
        "id", "x", "b", "a", "y", "z", "Count", "", "c", "d", "l", "missing",
    ];
    let rs = rows("FTREE");
    assert_eq!(rs.len(), all.len());
    for (r, set) in rs.iter().zip(all.iter()) {
        let mut root = FieldTree::create_root();
        for f in set {
            root.add_entry(f);
        }
        assert_eq!(ftree(&root), r[2], "FTREE {}: tree", r[1]);
        let grid: String = types
            .iter()
            .flat_map(|t| ids.iter().map(move |id| (t, id)))
            .map(|(t, id)| if root.is_selected(t, id) { '1' } else { '0' })
            .collect();
        assert_eq!(grid, r[3], "FTREE {}: isSelected grid", r[1]);
    }
}

#[test]
fn collect_and_skip_fields_on_crafted_trees_match_the_jar() {
    let sets = extra_selectors();
    let rs = rows("CFX");
    assert_eq!(rs.len(), sets.len() * 4);
    for r in rs {
        let (set, name) = (i32f(r[1]) as usize, r[2]);
        let tree = parse_lit(r[3]);
        let w = nbt_write(tree.as_compound().expect("a compound tree"));
        let mut cf = CollectFields::new(&sets[set]);
        nbt_parse(&w, &mut cf, &mut NbtAccounter::unlimited_heap()).expect("parse");
        let mut sf = SkipFields::new(&sets[set]);
        nbt_parse(&w, &mut sf, &mut NbtAccounter::unlimited_heap()).expect("parse");
        assert_eq!(
            vec![
                any_hex(cf.get_result()),
                cf.get_missing_field_count().to_string(),
                any_hex(sf.get_result())
            ],
            r[4..].to_vec(),
            "CFX set {set} tree {name}"
        );
    }
}

#[test]
fn collect_and_skip_fields_on_hand_written_streams_match_the_jar() {
    // `CFXRAW`: streams `CompoundTag` cannot write -- a path key repeated after every field was
    // found (`CollectFields` must already have answered `BREAK`), a list root (`CollectFields`
    // answers `HALT` at the root; `SkipFields` does not override `visitRootEntry`), and a selected
    // field repeated.
    let sets = extra_selectors();
    let rs = rows("CFXRAW");
    assert_eq!(rs.len(), sets.len() * 3);
    for r in rs {
        let (set, name) = (i32f(r[1]) as usize, r[2]);
        let bytes = unhex(r[3]);
        let mut cf = CollectFields::new(&sets[set]);
        nbt_parse(&bytes, &mut cf, &mut NbtAccounter::unlimited_heap()).expect("parse");
        let mut sf = SkipFields::new(&sets[set]);
        nbt_parse(&bytes, &mut sf, &mut NbtAccounter::unlimited_heap()).expect("parse");
        assert_eq!(
            vec![
                any_hex(cf.get_result()),
                cf.get_missing_field_count().to_string(),
                any_hex(sf.get_result())
            ],
            r[4..].to_vec(),
            "CFXRAW set {set} stream {name}"
        );
    }
}
