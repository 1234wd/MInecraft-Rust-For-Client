//! Sub-batch H: ChunkResult, StaticCache2D, CarvingMask and the `BitSet` subset under it.
//!
//! Groups: `CHUNKRES`, `CHUNKRESERR`, `CHUNKRESFAILEQ`, `REC`/`RECEQ` (`ChunkResult$Success`),
//! `STATICCACHE`, `STATICCACHENASE`, `CARVING`.

use std::cell::RefCell;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

use crate::b_records::check_records;
use crate::d_state::panic_text;
use crate::small_probe::*;

use minecraft_rust::net::minecraft::server::level::ChunkResult::{self as chunk_result, ChunkResult, ErrorSupplier};
use minecraft_rust::net::minecraft::util::StaticCache2D::StaticCache2D;
use minecraft_rust::net::minecraft::world::level::chunk::CarvingMask::CarvingMask;
use minecraft_rust::net::minecraft::world::level::ChunkPos::ChunkPos;

fn opt_i32(v: Option<&i32>) -> String {
    v.map_or("null".to_string(), |x| x.to_string())
}

#[test]
fn chunk_result_matches_the_jar() {
    let ok: ChunkResult<i32> = ChunkResult::of(5);
    let supplied = Arc::new(AtomicI32::new(0));
    let s = supplied.clone();
    let sup: ErrorSupplier = Arc::new(move || format!("err{}", s.fetch_add(1, Ordering::SeqCst) + 1));
    let bad: ChunkResult<i32> = ChunkResult::error(sup.clone());
    let bad_str: ChunkResult<i32> = ChunkResult::error_string("boom");
    let seen = RefCell::new(Vec::<String>::new());
    let rows_r = rows("CHUNKRES");

    let r = rows_r.iter().find(|r| r[1] == "of").unwrap();
    let minus_one = -1;
    let minus_seven = -7;
    assert_eq!(ok.is_success(), boolf(r[2]));
    assert_eq!(opt_i32(ok.or_else(Some(&minus_one))), r[3]);
    assert_eq!(ok.get_error(), unesc(r[4]));
    assert_eq!(std::ptr::eq(ok.if_success(|v| seen.borrow_mut().push(format!("ok:{v}"))), &ok), boolf(r[5]));
    assert_eq!(opt_i32(ok.map(|v| v * 10).or_else(None)), r[6]);
    assert_eq!(opt_i32(chunk_result::or_else_of(&ok, Some(&minus_seven))), r[7]);
    assert_eq!(ChunkResult::of(5).or_else_throw(|| "unused").unwrap().to_string(), r[8]);

    let r = rows_r.iter().find(|r| r[1] == "error-supplier").unwrap();
    assert_eq!(bad.is_success(), boolf(r[2]));
    assert_eq!(opt_i32(bad.or_else(Some(&minus_one))), r[3]);
    assert_eq!(bad.get_error(), unesc(r[4]));
    assert_eq!(std::ptr::eq(bad.if_success(|v| seen.borrow_mut().push(format!("bad:{v}"))), &bad), boolf(r[5]));
    let bad_mapped: ChunkResult<String> = bad.map(|v| format!("x{v}"));
    assert_eq!(bad_mapped.or_else(None).map_or("null".to_string(), |s| s.clone()), r[6]);
    assert_eq!(opt_i32(chunk_result::or_else_of(&bad, Some(&minus_seven))), r[7]);
    let thrown = ChunkResult::<i32>::error(sup.clone()).or_else_throw(|| "java.lang.IllegalStateException: no chunk".to_string());
    assert_eq!(Some(thrown.unwrap_err()), unesc(r[8]));

    let r = &rows("CHUNKRESERR")[0];
    assert_eq!(bad.get_error(), unesc(r[1]), "the supplier runs again on every getError");
    assert_eq!(bad_mapped.get_error(), unesc(r[2]));
    assert_eq!(bad_str.get_error(), unesc(r[3]));
    assert_eq!(bad_str.get_error(), unesc(r[4]));
    assert_eq!(supplied.load(Ordering::SeqCst), i32f(r[5]));
    match &bad_mapped {
        ChunkResult::Fail(error) => assert_eq!(Arc::ptr_eq(error, &sup), boolf(r[6]), "map keeps the same supplier"),
        ChunkResult::Success(_) => panic!("map of a Fail is a Fail"),
    }
    assert_eq!(seen.borrow().join(","), r[7]);

    check_records(
        "ChunkResult$Success",
        &[ChunkResult::of(5), ChunkResult::of(5), ChunkResult::of(-5), ChunkResult::of(0)],
        &|x| x.java_hash_code_with(|v| *v).expect("Success hashes"),
    );
    let r = &rows("CHUNKRESFAILEQ")[0];
    let other: ErrorSupplier = Arc::new(|| "err1".to_string());
    assert_eq!(bad == bad, boolf(r[1]));
    assert_eq!(bad == ChunkResult::error(sup.clone()), boolf(r[2]), "Fail equals by supplier identity");
    assert_eq!(bad == ChunkResult::error(other), boolf(r[3]));
}

#[test]
fn static_cache_2d_matches_the_jar() {
    for r in rows("STATICCACHE") {
        let (cx, cz, range) = (i32f(r[1]), i32f(r[2]), i32f(r[3]));
        let inits = RefCell::new(Vec::new());
        let sc = StaticCache2D::create(cx, cz, range, |x: i32, z: i32| {
            inits.borrow_mut().push(format!("{x}:{z}"));
            format!("{x}/{z}")
        });
        assert_eq!(Some(sc.to_string()), unesc(r[4]), "toString");
        assert_eq!(inits.borrow().join(","), r[5], "initializer calls for create({cx}, {cz}, {range})");
        let mut each = Vec::new();
        sc.for_each(|v| each.push(v.map_or("null".to_string(), |s| s.clone())));
        assert_eq!(each.join(","), r[6], "forEach for create({cx}, {cz}, {range})");
        let mut grid = Vec::new();
        for dx in -3..=3 {
            for dz in -3..=3 {
                let (x, z) = (cx.wrapping_add(dx), cz.wrapping_add(dz));
                let contains = if sc.contains(x, z) { '1' } else { '0' };
                let got = if sc.contains(x, z) {
                    sc.get(x, z).map_or("null".to_string(), |s| s.clone())
                } else {
                    let text = panic_text(|| {
                        sc.get(x, z);
                    });
                    format!("EX:{}", text.strip_prefix("java.lang.IllegalArgumentException: ").expect("an IAE"))
                };
                grid.push(format!("{contains}{got}"));
            }
        }
        assert_eq!(Some(grid.join(";")), unesc(r[7]), "lookup grid for create({cx}, {cz}, {range})");
    }
    let text = panic_text(|| {
        StaticCache2D::create(0, 0, 23170, |_x: i32, _z: i32| String::new());
    });
    assert_eq!(Some(text), unesc(rows("STATICCACHENASE")[0][1]));
}

#[test]
fn carving_mask_replays_the_jar_scripts() {
    for r in rows("CARVING") {
        let mut cm: Option<CarvingMask> = None;
        let mut out = Vec::new();
        for op in r[1].split(' ') {
            let p: Vec<&str> = op.split(':').collect();
            let int = |s: &str| i32f(s);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> String {
                match p[0] {
                    "new" => {
                        cm = Some(CarvingMask::new(int(p[1]), int(p[2])));
                        "ok".to_string()
                    }
                    "arr" => {
                        let words: Vec<i64> = p[1].split(',').map(i64f).collect();
                        cm = Some(CarvingMask::from_array(&words, int(p[2])));
                        "ok".to_string()
                    }
                    "s" => {
                        cm.as_mut().unwrap().set(int(p[1]), int(p[2]), int(p[3]));
                        "ok".to_string()
                    }
                    "m" => {
                        cm.as_mut().unwrap().set_additional_mask(|x: i32, y: i32, z: i32| x == y && y == -z);
                        "ok".to_string()
                    }
                    _ => cm.as_ref().unwrap().get(int(p[1]), int(p[2]), int(p[3])).to_string(),
                }
            }));
            out.push(match result {
                Ok(s) => s,
                Err(e) => format!("EX:{}", e.downcast_ref::<String>().cloned().unwrap_or_default()),
            });
        }
        assert_eq!(Some(out.join(";")), unesc(r[2]), "script {}", r[1]);
        let tail = match &cm {
            None => "-".to_string(),
            Some(m) => {
                let arr: Vec<String> = m.to_array().iter().map(|w| w.to_string()).collect();
                let pos = ChunkPos::new(2, -3);
                let st: Vec<String> = m.stream(&pos).map(|b| format!("{}/{}/{}", b.get_x(), b.get_y(), b.get_z())).collect();
                format!("[{}]|{}", arr.join(","), st.join(","))
            }
        };
        assert_eq!(tail, r[3], "toArray/stream after {}", r[1]);
    }
}
