//! Session 25 sub-batch G: Guava IntMath.gcd and DiscreteCubeMerger.

use crate::sweep_probe::*;
use minecraft_rust::javacompat::fastutil::DoubleList;
use minecraft_rust::javacompat::guava::int_math::gcd;
use minecraft_rust::net::minecraft::world::phys::shapes::DiscreteCubeMerger::DiscreteCubeMerger;
use minecraft_rust::net::minecraft::world::phys::shapes::IndexMerger::IndexMerger;

#[test]
fn guava_int_math_gcd_matches_the_pinned_jar_and_errors() {
    for row in rows("S25_GCD") {
        let (a, b) = (i32f(row[1]), i32f(row[2]));
        assert_eq!(
            or_ex(|| gcd(a, b), |value| value.to_string()),
            row[3],
            "gcd({a}, {b})"
        );
    }
}

#[test]
fn discrete_cube_merger_matches_lcm_sizes_coordinates_indices_and_errors() {
    for row in rows("S25_DISCRETE") {
        let (first, second) = (i32f(row[1]), i32f(row[2]));
        let got = or_ex(
            || DiscreteCubeMerger::new(first, second),
            |merger| {
                let size = IndexMerger::size(&merger);
                if !(0..=512).contains(&size) {
                    return format!("{size}|large");
                }
                let list = IndexMerger::get_list(&merger).unwrap();
                let same_list = IndexMerger::get_list(&merger).unwrap();
                let address = list as *const dyn DoubleList as *const ();
                let same_address = same_list as *const dyn DoubleList as *const ();
                let values = (0..size)
                    .map(|index| dbits(list.get_double(index)))
                    .collect::<Vec<_>>()
                    .join(",");
                let mut calls = Vec::new();
                let mut consumer = |a, b, result| {
                    calls.push((a, b, result));
                    true
                };
                let complete = IndexMerger::for_merged_indexes(&merger, Some(&mut consumer));
                let all = calls
                    .iter()
                    .map(|(a, b, c)| format!("{a},{b},{c}"))
                    .collect::<Vec<_>>()
                    .join(";");
                let mut stopped = Vec::new();
                let mut stop = |a, b, result| {
                    stopped.push((a, b, result));
                    false
                };
                let stop_result = IndexMerger::for_merged_indexes(&merger, Some(&mut stop));
                let stop_log = stopped
                    .iter()
                    .map(|(a, b, c)| format!("{a},{b},{c}"))
                    .collect::<Vec<_>>()
                    .join(";");
                let null_call = or_ex(
                    || IndexMerger::for_merged_indexes(&merger, None),
                    |value| value.to_string(),
                );
                format!(
                    "{size}|{}|{values}|{complete}|{all}|{stop_result}|{stop_log}|{null_call}",
                    address == same_address
                )
            },
        );
        assert_eq!(got, row[3], "first={first}, second={second}");
    }
}

#[test]
fn discrete_cube_merger_declared_members_match_the_26_2_jar() {
    let mut want: Vec<String> = rows("S25_API")
        .iter()
        .filter(|row| row[1] == "DiscreteCubeMerger")
        .map(|r| format!("{}\t{}\t{}\t{}", r[1], r[2], r[3], r[4]))
        .collect();
    let mut have = MEMBERS
        .iter()
        .map(|m| format!("{}\t{}\t{}\t{}", m.0, m.1, m.2, m.3))
        .collect::<Vec<_>>();
    want.sort();
    have.sort();
    assert_eq!(
        have, want,
        "DiscreteCubeMerger declarations differ from the jar"
    );

    let got: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|row| row[1] == "DiscreteCubeMerger")
        .map(|r| (r[1].to_string(), r[2].to_string(), r[3].to_string()))
        .collect();
    assert_eq!(
        got,
        vec![(
            "DiscreteCubeMerger".to_string(),
            "net.minecraft.world.phys.shapes.IndexMerger".to_string(),
            "class".to_string()
        )]
    );
}

const MEMBERS: &[(&str, &str, &str, &str)] = &[
    (
        "DiscreteCubeMerger",
        "field",
        "firstDiv:int",
        "private final",
    ),
    (
        "DiscreteCubeMerger",
        "field",
        "result:net.minecraft.world.phys.shapes.CubePointRange",
        "private final",
    ),
    (
        "DiscreteCubeMerger",
        "field",
        "secondDiv:int",
        "private final",
    ),
    (
        "DiscreteCubeMerger",
        "method",
        "forMergedIndexes(net.minecraft.world.phys.shapes.IndexMerger$IndexConsumer)boolean",
        "public",
    ),
    (
        "DiscreteCubeMerger",
        "method",
        "getList()it.unimi.dsi.fastutil.doubles.DoubleList",
        "public",
    ),
    ("DiscreteCubeMerger", "method", "size()int", "public"),
    ("DiscreteCubeMerger", "ctor", "<init>(int,int)", "package"),
];
