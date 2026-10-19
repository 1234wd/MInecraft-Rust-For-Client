//! Session 25 sub-batch F: the fastutil `DoubleList` read surface, `CubePointRange`,
//! `OffsetDoubleList`, and the declaration-only `IndexMerger` interfaces.

use std::cell::RefCell;

use crate::sweep_probe::*;
use minecraft_rust::javacompat::fastutil::DoubleList;
use minecraft_rust::net::minecraft::world::phys::shapes::CubePointRange::CubePointRange;
use minecraft_rust::net::minecraft::world::phys::shapes::IdenticalMerger::IdenticalMerger;
use minecraft_rust::net::minecraft::world::phys::shapes::IndexMerger::{
    IndexConsumer, IndexMerger,
};
use minecraft_rust::net::minecraft::world::phys::shapes::NonOverlappingMerger::NonOverlappingMerger;
use minecraft_rust::net::minecraft::world::phys::shapes::OffsetDoubleList::OffsetDoubleList;

struct DoubleArrayList(RefCell<Vec<f64>>);

impl DoubleArrayList {
    fn new(values: Vec<f64>) -> Self {
        Self(RefCell::new(values))
    }
}

impl DoubleList for DoubleArrayList {
    fn get_double(&self, index: i32) -> f64 {
        let values = self.0.borrow();
        let len = values.len();
        if index >= len as i32 {
            panic!("java.lang.IndexOutOfBoundsException: Index ({index}) is greater than or equal to list size ({len})");
        }
        if index < 0 {
            panic!("java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {len}");
        }
        values[index as usize]
    }

    fn size(&self) -> i32 {
        self.0.borrow().len() as i32
    }
}

fn values(case: i32) -> Vec<f64> {
    match case {
        0 => Vec::new(),
        1 => vec![-0.0, 0.0, 1.25, -3.5],
        2 => vec![
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(1),
            f64::MAX,
        ],
        other => panic!("unknown values case {other}"),
    }
}

fn offset(case: i32) -> f64 {
    match case {
        0 => 0.0,
        1 => -0.0,
        2 => 0.25,
        3 => f64::NAN,
        4 => f64::INFINITY,
        5 => f64::NEG_INFINITY,
        other => panic!("unknown offset case {other}"),
    }
}

#[test]
fn cube_point_range_matches_jar_construction_size_and_unchecked_get() {
    for row in rows("S25_CUBERANGE") {
        let parts = i32f(row[1]);
        let got = or_ex(
            || CubePointRange::new(parts),
            |range| {
                let indexes = [i32::MIN, -1, 0, 1, parts.wrapping_sub(1), parts, i32::MAX];
                let mut fields = vec![range.size().to_string()];
                fields.extend(
                    indexes
                        .into_iter()
                        .map(|index| dbits(range.get_double(index))),
                );
                fields.join("|")
            },
        );
        assert_eq!(got, row[2], "CubePointRange({parts})");
    }
}

#[test]
fn offset_double_list_delegates_values_size_and_array_errors_to_its_source() {
    for row in rows("S25_OFFSET") {
        let (case, offset_case) = (i32f(row[1]), i32f(row[2]));
        let data = DoubleArrayList::new(values(case));
        let list = OffsetDoubleList::new(Some(&data), offset(offset_case));
        let got = if row[3] == "size" {
            list.size().to_string()
        } else {
            let index = i32f(row[3]);
            or_ex(|| list.get_double(index), dbits)
        };
        assert_eq!(
            nan_canon(&got),
            nan_canon(row[4]),
            "case {case}, offset {offset_case}, index {}",
            row[3]
        );
    }
}

#[test]
fn offset_double_list_keeps_null_and_delegate_aliasing_semantics() {
    let null_list = OffsetDoubleList::new(None, 0.5);
    for row in rows("S25_OFFSETNULL") {
        let got = match row[1] {
            "get" => or_ex(|| null_list.get_double(0), dbits),
            "size" => or_ex(|| null_list.size(), |v| v.to_string()),
            other => panic!("unknown null-list operation {other}"),
        };
        assert_eq!(got, row[2], "null delegate {}", row[1]);
    }

    let data = DoubleArrayList::new(vec![1.25, -0.0]);
    let list = OffsetDoubleList::new(Some(&data), 0.5);
    let before = format!(
        "{}|{}|{}",
        list.size(),
        dbits(list.get_double(0)),
        dbits(list.get_double(1))
    );
    data.0.borrow_mut()[0] = 4.0;
    data.0.borrow_mut().push(9.0);
    let after = format!(
        "{}|{}|{}|{}",
        list.size(),
        dbits(list.get_double(0)),
        dbits(list.get_double(1)),
        dbits(list.get_double(2))
    );
    let row = &rows("S25_OFFSETALIAS")[0];
    assert_eq!(before, row[1]);
    assert_eq!(after, row[2]);
}

fn merge_log(merger: &dyn IndexMerger, stop_after: i32, null_consumer: bool) -> String {
    if null_consumer {
        return or_ex(
            || merger.for_merged_indexes(None),
            |complete| format!("{complete}|"),
        );
    }

    let mut calls = Vec::new();
    let mut consumer = |first, second, result| {
        calls.push((first, second, result));
        stop_after < 0 || calls.len() < stop_after as usize
    };
    let complete = or_ex(
        || merger.for_merged_indexes(Some(&mut consumer)),
        |complete| complete.to_string(),
    );
    if complete.starts_with("EX:") {
        return complete;
    }
    let rendered = calls
        .iter()
        .map(|(first, second, result)| format!("{first},{second},{result}"))
        .collect::<Vec<_>>()
        .join(";");
    format!("{complete}|{rendered}")
}

fn merger_values(merger: &dyn DoubleList) -> String {
    let size = merger.size();
    (-1..=size + 1)
        .map(|index| or_ex(|| merger.get_double(index), dbits))
        .collect::<Vec<_>>()
        .join(";")
}

#[test]
fn identical_merger_matches_jar_iteration_early_stop_and_null_edges() {
    let cases = [Vec::new(), vec![7.5], vec![-0.0, 0.0, 1.25, -3.5]];
    for row in rows("S25_IDENTICAL") {
        let case = i32f(row[1]) as usize;
        let stop = i32f(row[2]);
        let list = DoubleArrayList::new(cases[case].clone());
        let merger = IdenticalMerger::new(Some(&list));
        assert_eq!(merger.size().to_string(), row[3]);
        assert_eq!(
            std::ptr::eq(merger.get_list().unwrap(), &list as &dyn DoubleList),
            boolf(row[4]),
            "delegate identity, case {case}"
        );
        assert_eq!(
            merge_log(&merger, stop, false),
            row[5],
            "case {case}, stop {stop}"
        );
    }

    for row in rows("S25_IDENTICALNULLCALL") {
        let case = i32f(row[1]) as usize;
        let list = DoubleArrayList::new(cases[case].clone());
        let merger = IdenticalMerger::new(Some(&list));
        assert_eq!(merge_log(&merger, -1, true), row[2], "case {case}");
    }

    let merger = IdenticalMerger::new(None);
    for row in rows("S25_IDENTICALNULL") {
        let got = match row[1] {
            "getList" => (merger.get_list().is_none()).to_string(),
            "size" => or_ex(|| merger.size(), |size| size.to_string()),
            "merge" => merge_log(&merger, -1, false),
            other => panic!("unknown null-identical operation {other}"),
        };
        assert_eq!(got, row[2], "null coordinates, {}", row[1]);
    }
}

#[test]
fn non_overlapping_merger_matches_jar_values_index_swaps_and_null_edges() {
    let cases = [
        Vec::new(),
        vec![7.5],
        vec![-0.0, 0.0, 1.25],
        vec![f64::NAN, f64::INFINITY, f64::from_bits(1), f64::MAX],
    ];
    for row in rows("S25_NONOVERLAP") {
        let (lower_case, upper_case, swap) =
            (i32f(row[1]) as usize, i32f(row[2]) as usize, boolf(row[3]));
        let lower = DoubleArrayList::new(cases[lower_case].clone());
        let upper = DoubleArrayList::new(cases[upper_case].clone());
        let merger = NonOverlappingMerger::new(Some(&lower), Some(&upper), swap);
        assert_eq!(IndexMerger::size(&merger).to_string(), row[4]);
        assert_eq!(
            (IndexMerger::get_list(&merger).unwrap() as *const dyn DoubleList as *const ())
                == (&merger as *const NonOverlappingMerger<'_> as *const ()),
            boolf(row[5]),
            "self list identity"
        );
        assert_eq!(
            nan_canon(&merger_values(&merger)),
            nan_canon(row[6]),
            "lower {lower_case}, upper {upper_case}, swap {swap}"
        );
        assert_eq!(
            merge_log(&merger, -1, false),
            row[7],
            "all indices, swap {swap}"
        );
        assert_eq!(
            merge_log(&merger, 1, false),
            row[8],
            "early stop, swap {swap}"
        );
        assert_eq!(
            merge_log(&merger, -1, true),
            row[9],
            "null callback, swap {swap}"
        );
    }

    let one = DoubleArrayList::new(vec![1.0]);
    for row in rows("S25_NONOVERLAPNULL") {
        let null_lower = row[1] == "lower";
        let merger = NonOverlappingMerger::new(
            if null_lower { None } else { Some(&one) },
            if null_lower { Some(&one) } else { None },
            false,
        );
        let got = match row[2] {
            "size" => or_ex(|| IndexMerger::size(&merger), |size| size.to_string()),
            "get" => or_ex(|| merger.get_double(if null_lower { 0 } else { 1 }), dbits),
            "merge" => merge_log(&merger, -1, false),
            other => panic!("unknown null-nonoverlap operation {other}"),
        };
        assert_eq!(got, row[3], "null {} operation {}", row[1], row[2]);
    }
}

#[test]
fn index_merger_and_fastutil_double_list_declarations_match_the_jar() {
    let mut want: Vec<String> = rows("S25_API")
        .iter()
        .filter(|r| {
            !matches!(
                r[1],
                "DiscreteCubeMerger"
                    | "Divisor"
                    | "CommonLinks"
                    | "UniquelyIdentifyable"
                    | "ExceptionCollector"
                    | "FileSystemCapabilities"
                    | "CopyOption"
                    | "LinkOption"
                    | "StandardCopyOption"
                    | "CrashReportDetail"
                    | "ChunkIdentification"
                    | "CpuLoadStat"
                    | "FpsStat"
                    | "PacketIdentification"
                    | "UUIDLookup"
            )
        })
        .map(|r| format!("{}\t{}\t{}\t{}", r[1], r[2], r[3], r[4]))
        .collect();
    let mut have = MEMBERS
        .iter()
        .map(|m| format!("{}\t{}\t{}\t{}", m.0, m.1, m.2, m.3))
        .collect::<Vec<_>>();
    want.sort();
    have.sort();
    assert_eq!(have, want, "Session 25 declarations and the jar disagree");

    let got_supers: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|r| {
            !matches!(
                r[1],
                "DiscreteCubeMerger"
                    | "Divisor"
                    | "CommonLinks"
                    | "UniquelyIdentifyable"
                    | "ExceptionCollector"
                    | "FileSystemCapabilities"
                    | "CopyOption"
                    | "LinkOption"
                    | "StandardCopyOption"
                    | "CrashReportDetail"
                    | "ChunkIdentification"
                    | "CpuLoadStat"
                    | "FpsStat"
                    | "PacketIdentification"
                    | "UUIDLookup"
            )
        })
        .map(|r| (r[1].to_string(), r[2].to_string(), r[3].to_string()))
        .collect();
    let have_supers: Vec<(String, String, String)> = SUPERS
        .iter()
        .map(|s| (s.0.to_string(), s.1.to_string(), s.2.to_string()))
        .collect();
    assert_eq!(
        have_supers, got_supers,
        "Session 25 supertypes and the jar disagree"
    );

    struct Contract {
        list: CubePointRange,
    }
    impl IndexMerger for Contract {
        fn get_list(&self) -> Option<&dyn DoubleList> {
            Some(&self.list)
        }
        fn for_merged_indexes(&self, consumer: Option<&mut dyn IndexConsumer>) -> bool {
            consumer.is_some_and(|consumer| consumer.merge(0, 1, 2))
        }
        fn size(&self) -> i32 {
            self.list.size()
        }
    }
    let contract = Contract {
        list: CubePointRange::new(2),
    };
    assert_eq!(contract.size(), 3);
    assert_eq!(contract.get_list().unwrap().get_double(1), 0.5);
    let mut call = |first, second, result| first == 0 && second == 1 && result == 2;
    assert!(contract.for_merged_indexes(Some(&mut call)));
}

const MEMBERS: &[(&str, &str, &str, &str)] = &[
    ("CubePointRange", "ctor", "<init>(int)", "public"),
    ("CubePointRange", "field", "parts:int", "private final"),
    ("CubePointRange", "method", "getDouble(int)double", "public"),
    ("CubePointRange", "method", "size()int", "public"),
    ("OffsetDoubleList", "ctor", "<init>(it.unimi.dsi.fastutil.doubles.DoubleList,double)", "public"),
    ("OffsetDoubleList", "field", "delegate:it.unimi.dsi.fastutil.doubles.DoubleList", "private final"),
    ("OffsetDoubleList", "field", "offset:double", "private final"),
    ("OffsetDoubleList", "method", "getDouble(int)double", "public"),
    ("OffsetDoubleList", "method", "size()int", "public"),
    ("IndexMerger", "method", "forMergedIndexes(net.minecraft.world.phys.shapes.IndexMerger$IndexConsumer)boolean", "public abstract"),
    ("IndexMerger", "method", "getList()it.unimi.dsi.fastutil.doubles.DoubleList", "public abstract"),
    ("IndexMerger", "method", "size()int", "public abstract"),
    ("IndexMerger$IndexConsumer", "method", "merge(int,int,int)boolean", "public abstract"),
    ("IdenticalMerger", "ctor", "<init>(it.unimi.dsi.fastutil.doubles.DoubleList)", "public"),
    ("IdenticalMerger", "field", "coords:it.unimi.dsi.fastutil.doubles.DoubleList", "private final"),
    ("IdenticalMerger", "method", "forMergedIndexes(net.minecraft.world.phys.shapes.IndexMerger$IndexConsumer)boolean", "public"),
    ("IdenticalMerger", "method", "getList()it.unimi.dsi.fastutil.doubles.DoubleList", "public"),
    ("IdenticalMerger", "method", "size()int", "public"),
    ("NonOverlappingMerger", "ctor", "<init>(it.unimi.dsi.fastutil.doubles.DoubleList,it.unimi.dsi.fastutil.doubles.DoubleList,boolean)", "protected"),
    ("NonOverlappingMerger", "field", "lower:it.unimi.dsi.fastutil.doubles.DoubleList", "private final"),
    ("NonOverlappingMerger", "field", "upper:it.unimi.dsi.fastutil.doubles.DoubleList", "private final"),
    ("NonOverlappingMerger", "field", "swap:boolean", "private final"),
    ("NonOverlappingMerger", "method", "forMergedIndexes(net.minecraft.world.phys.shapes.IndexMerger$IndexConsumer)boolean", "public"),
    ("NonOverlappingMerger", "method", "forNonSwappedIndexes(net.minecraft.world.phys.shapes.IndexMerger$IndexConsumer)boolean", "private"),
    ("NonOverlappingMerger", "method", "getDouble(int)double", "public"),
    ("NonOverlappingMerger", "method", "getList()it.unimi.dsi.fastutil.doubles.DoubleList", "public"),
    ("NonOverlappingMerger", "method", "size()int", "public"),
];

const SUPERS: &[(&str, &str, &str)] = &[
    ("CubePointRange", "extends it.unimi.dsi.fastutil.doubles.AbstractDoubleList", "class"),
    ("OffsetDoubleList", "extends it.unimi.dsi.fastutil.doubles.AbstractDoubleList", "class"),
    ("IndexMerger", "-", "interface"),
    ("IndexMerger$IndexConsumer", "-", "interface"),
    ("IdenticalMerger", "net.minecraft.world.phys.shapes.IndexMerger", "class"),
    ("NonOverlappingMerger", "net.minecraft.world.phys.shapes.IndexMerger,extends it.unimi.dsi.fastutil.doubles.AbstractDoubleList", "class"),
];
