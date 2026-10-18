//! Sub-batch G: GpuFormat, BlockDisplayContext, MultiblockChestResources, BoundingBoxRenderable,
//! FieldSelector, SingleKeyCache, FileResourceType, PngInfo.
//!
//! Groups: `GPUCOMP`, `GPUFMT`, `BLOCKDISPCTX`, `MULTICHEST`, `BBMODE`, `FIELDSEL`, `SINGLEKEY`,
//! `FILERES`, `PNGINFO`, `PNGHDR`, and `REC`/`RECEQ` for the two records.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::b_records::check_records;
use crate::small_probe::*;

use minecraft_rust::com::mojang::blaze3d::GpuFormat::{ComponentType, GpuFormat};
use minecraft_rust::javacompat::java_io;
use minecraft_rust::net::minecraft::client::renderer::block::model::BlockDisplayContext::BlockDisplayContext;
use minecraft_rust::net::minecraft::client::renderer::MultiblockChestResources::MultiblockChestResources;
use minecraft_rust::net::minecraft::core::BlockPos::BlockPos;
use minecraft_rust::net::minecraft::core::Vec3i::Vec3i;
use minecraft_rust::net::minecraft::nbt::visitors::FieldSelector::FieldSelector;
use minecraft_rust::net::minecraft::nbt::TagType::TagTypeRef;
use minecraft_rust::net::minecraft::util::filefix::access::FileResourceType::FileResourceType;
use minecraft_rust::net::minecraft::util::PngInfo::PngInfo;
use minecraft_rust::net::minecraft::util::SingleKeyCache::SingleKeyCache;
use minecraft_rust::net::minecraft::world::level::block::entity::BoundingBoxRenderable::{
    Mode, RenderableBox,
};
use minecraft_rust::net::minecraft::world::level::block::state::properties::ChestType::{
    self as chest_type,
};

#[test]
fn gpu_format_and_component_type_match_the_jar() {
    let comps = rows("GPUCOMP");
    assert_eq!(
        comps.len(),
        ComponentType::VALUES.len(),
        "ComponentType constants"
    );
    for (r, c) in comps.iter().zip(ComponentType::VALUES.iter()) {
        assert_eq!(
            (c.name(), c.ordinal(), c.byte_size()),
            (r[1], i32f(r[2]), i32f(r[3]))
        );
    }
    let fmts = rows("GPUFMT");
    assert_eq!(fmts.len(), GpuFormat::VALUES.len(), "GpuFormat constants");
    for (r, g) in fmts.iter().zip(GpuFormat::VALUES.iter()) {
        assert_eq!(
            (g.name(), g.ordinal(), g.component_type().name()),
            (r[1], i32f(r[2]), r[3]),
            "{}",
            r[1]
        );
        assert_eq!(
            (g.component_count(), g.block_size(), g.byte_alignment()),
            (i32f(r[4]), i32f(r[5]), i32f(r[6])),
            "{}",
            r[1]
        );
        assert_eq!(
            (
                g.has_color_aspect(),
                g.has_depth_aspect(),
                g.has_stencil_aspect()
            ),
            (boolf(r[7]), boolf(r[8]), boolf(r[9])),
            "{} aspects",
            r[1]
        );
        assert_eq!(g.to_string(), r[1]);
    }
}

#[test]
fn block_display_context_is_a_fresh_object_per_create() {
    let r = &rows("BLOCKDISPCTX")[0];
    let (c1, c2) = (BlockDisplayContext::create(), BlockDisplayContext::create());
    // Java: two creates are neither `==` nor `equals`; one is equal to itself. Identity -- which
    // the port carries as a per-create number, not an address (a zero-sized type has none).
    assert_eq!(c1 == c2, boolf(r[1]), "c1 == c2");
    assert_eq!(c1 == c2, boolf(r[2]), "c1.equals(c2)");
    assert_eq!(c1 == c1, boolf(r[3]), "c1.equals(c1)");
    let copy = c1;
    assert!(copy == c1, "a copied reference is the same object");
    // Distinct across threads as well: four threads, a hundred creates each, no two equal.
    let handles: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                (0..100)
                    .map(|_| BlockDisplayContext::create())
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    let mut seen = std::collections::HashSet::from([c1, c2]);
    for h in handles {
        for c in h.join().expect("create thread") {
            assert!(seen.insert(c), "{c:?} was created twice");
        }
    }
    assert_eq!(seen.len(), 402);
    assert_eq!(r[4], "private", "the jar's constructor is private");
}

#[test]
fn multiblock_chest_resources_match_the_jar() {
    let m = MultiblockChestResources::new(1, -2, 300);
    let r = &rows("MULTICHEST")[0];
    let sel: Vec<String> = chest_type::VALUES
        .iter()
        .map(|t| format!("{}={}", t.name(), m.select(*t)))
        .collect();
    assert_eq!(sel.join(","), r[1]);
    let calls = RefCell::new(Vec::new());
    let mapped = m.map(|v| {
        calls.borrow_mut().push(v.to_string());
        format!("#{v}")
    });
    assert_eq!(
        calls.borrow().join(","),
        r[2],
        "map evaluates single, left, right in order"
    );
    assert_eq!(Some(mapped.to_string()), unesc(r[3]));
    assert_eq!((*m.single(), *m.left(), *m.right()), (1, -2, 300));
    check_records(
        "MultiblockChestResources",
        &[
            m,
            MultiblockChestResources::new(1, -2, 300),
            MultiblockChestResources::new(1, 300, -2),
            MultiblockChestResources::new(0, 0, 0),
        ],
        &|x| x.java_hash_code_with(|v| *v),
    );
}

#[test]
fn bounding_box_renderable_matches_the_jar() {
    let modes = rows("BBMODE");
    assert_eq!(modes.len(), Mode::VALUES.len());
    for (r, m) in modes.iter().zip(Mode::VALUES.iter()) {
        assert_eq!((m.name(), m.ordinal()), (r[1], i32f(r[2])));
    }
    let corners = [
        [0, 0, 0, 1, 2, 3],
        [5, -5, 10, -5, 5, 0],
        [i32::MIN, 0, 0, i32::MAX, 0, 0],
        [7, 7, 7, 7, 7, 7],
        [-1, -2, -3, -4, -5, -6],
    ];
    let mut boxes: Vec<RenderableBox> = corners
        .iter()
        .map(|c| RenderableBox::from_corners(c[0], c[1], c[2], c[3], c[4], c[5]))
        .collect();
    boxes.push(RenderableBox::new(
        BlockPos::new(0, 0, 0),
        Vec3i::new(1, 2, 3),
    ));
    check_records("RenderableBox", &boxes, &|x| x.java_hash_code());
    assert_eq!(boxes[2].size(), Vec3i::new(-1, 0, 0), "MAX - MIN wraps");
    assert_eq!(boxes[4].local_pos(), BlockPos::new(-4, -5, -6));
}

#[test]
fn field_selector_matches_the_jar() {
    let int_tag = TagTypeRef::of_id(3);
    let string_tag = TagTypeRef::of_id(8);
    let fs = [
        FieldSelector::of_name(int_tag, "x"),
        FieldSelector::in_parent("a", int_tag, "x"),
        FieldSelector::in_grandparent("g", "a", int_tag, "x"),
        FieldSelector::new(&["g", "a"], int_tag, "x"),
        FieldSelector::in_grandparent("g", "a", string_tag, "x"),
        FieldSelector::in_grandparent("g", "a", int_tag, "y"),
    ];
    let rows = rows("FIELDSEL");
    assert_eq!(rows.len(), fs.len());
    for (r, f) in rows.iter().zip(fs.iter()) {
        assert_eq!(f.path().join("/"), r[2], "path of [{}]", r[1]);
        assert_eq!(
            f.tag_type(),
            TagTypeRef::of_id(i32f(r[3]) as i8),
            "type of [{}]",
            r[1]
        );
        assert_eq!(f.name(), r[4]);
        let eq: String = fs.iter().map(|o| if f == o { '1' } else { '0' }).collect();
        assert_eq!(eq, r[5], "equals row [{}]", r[1]);
        // Java's path is `List.of`, immutable; `path()` here is a read-only slice.
        assert!(boolf(r[6]));
    }
}

#[test]
fn single_key_cache_replays_the_jar_script() {
    let r = &rows("SINGLEKEY")[0];
    let computes = Rc::new(Cell::new(0));
    let c = computes.clone();
    let mut cache = SingleKeyCache::new(move |k: &String| {
        c.set(c.get() + 1);
        k.chars().count() as i32
    });
    let mut out = Vec::new();
    for key in r[1].split(' ') {
        let v = *cache.get_value(&key.to_string());
        out.push(format!("{v}/{}", computes.get()));
    }
    assert_eq!(out.join(","), r[2]);
}

#[test]
fn file_resource_type_forwards_to_its_factory() {
    let calls = Rc::new(RefCell::new(Vec::<String>::new()));
    let c = calls.clone();
    let frt = FileResourceType::new(move |p: &std::path::Path, dv: i32| {
        c.borrow_mut().push(format!("{}@{dv}", p.display()));
    });
    frt.create(std::path::Path::new("level.dat"), 4903);
    frt.create(std::path::Path::new("a"), -1);
    assert_eq!(calls.borrow().join(";"), rows("FILERES")[0][1]);
}

fn unhex(h: &str) -> Vec<u8> {
    (0..h.len() / 2)
        .map(|i| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap())
        .collect()
}

#[test]
fn png_info_matches_the_jar() {
    for r in rows("PNGINFO") {
        let bytes = if r[1] == "-" { Vec::new() } else { unhex(r[1]) };
        match PngInfo::from_bytes(&bytes) {
            Ok(pi) => {
                assert_eq!(Some(pi.to_string()), unesc(r[2]), "fromBytes({})", r[1]);
                assert_eq!(pi.java_hash_code(), i32f(r[3]));
            }
            Err(e) => {
                let class = r[2].strip_prefix("EX:").unwrap_or_else(|| {
                    panic!("Java succeeded on {} but the port failed: {e}", r[1])
                });
                assert_eq!(java_io::java_class_name(&e), class, "fromBytes({})", r[1]);
                assert_eq!(
                    java_io::java_message(&e),
                    unesc(r[3]),
                    "fromBytes({})",
                    r[1]
                );
            }
        }
    }
    for r in rows("PNGHDR") {
        let result = PngInfo::validate_header(&unhex(r[1]));
        if r[2] == "ok" {
            assert!(result.is_ok(), "validateHeader({})", r[1]);
        } else {
            let e = result.expect_err("Java threw");
            assert_eq!(
                java_io::java_class_name(&e),
                r[2].strip_prefix("EX:").unwrap()
            );
            assert_eq!(
                java_io::java_message(&e),
                unesc(r[3]),
                "validateHeader({})",
                r[1]
            );
        }
    }
    let p = PngInfo::new(16, 32);
    assert_eq!((p.width(), p.height()), (16, 32));
}
