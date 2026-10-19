//! Session 25 sub-batch H: Divisor and fastutil's IntIterator subset.

use crate::sweep_probe::*;
use minecraft_rust::com::mojang::math::Divisor::{Divisor, DivisorIterable};
use minecraft_rust::javacompat::fastutil::IntIterator;

fn drain(divisor: &mut Divisor) -> String {
    let mut values = Vec::new();
    while divisor.has_next() {
        values.push(divisor.next_int());
    }
    let exhausted = or_ex(|| divisor.next_int(), |value| value.to_string());
    format!(
        "{}|{}|{exhausted}",
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(","),
        divisor.has_next()
    )
}

#[test]
fn divisor_java_quotient_remainder_and_exhaustion_match_the_jar() {
    for row in rows("S25_DIVISOR") {
        let (numerator, denominator) = (i32f(row[1]), i32f(row[2]));
        assert_eq!(
            drain(&mut Divisor::new(numerator, denominator)),
            row[3],
            "{numerator}/{denominator}"
        );
    }
}

#[test]
fn divisor_fastutil_skip_and_for_each_remaining_match_the_jar() {
    for row in rows("S25_DIVISORSKIP") {
        let (numerator, denominator, count) = (i32f(row[1]), i32f(row[2]), i32f(row[3]));
        let mut divisor = Divisor::new(numerator, denominator);
        let skipped = or_ex(
            || IntIterator::skip(&mut divisor, count),
            |value| value.to_string(),
        );
        let got = if skipped.starts_with("EX:") {
            skipped
        } else {
            format!("{skipped}|{}", drain(&mut divisor))
        };
        assert_eq!(
            got,
            if row.len() == 5 {
                row[4].to_string()
            } else {
                format!("{}|{}", row[4], row[5])
            },
            "skip({count}) for {numerator}/{denominator}"
        );
    }

    for row in rows("S25_DIVISORFOREACH") {
        let (numerator, denominator) = (i32f(row[1]), i32f(row[2]));
        let mut divisor = Divisor::new(numerator, denominator);
        let mut seen = Vec::new();
        let result = or_ex(
            || IntIterator::for_each_remaining(&mut divisor, Some(&mut |value| seen.push(value))),
            |_| "ok".to_string(),
        );
        let got = format!(
            "{result}|{}|{}",
            seen.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
            divisor.has_next()
        );
        assert_eq!(got, format!("{}|{}|{}", row[3], row[4], row[5]));
    }

    for row in rows("S25_DIVISORNULL") {
        let denominator = i32f(row[1]);
        let mut divisor = Divisor::new(4, denominator);
        let result = or_ex(
            || IntIterator::for_each_remaining(&mut divisor, None),
            |_| "ok".to_string(),
        );
        assert_eq!(result, row[2], "null action with denominator {denominator}");
        assert_eq!(divisor.has_next().to_string(), row[3]);
    }

    for row in rows("S25_DIVISORPREFIX") {
        let mut divisor = Divisor::new(i32f(row[1]), i32f(row[2]));
        let count = i32f(row[3]);
        let values = (0..count)
            .map(|_| divisor.next_int().to_string())
            .collect::<Vec<_>>()
            .join(",");
        assert_eq!(values, row[4], "overflow-sensitive numerator/denominator");
        assert_eq!(divisor.has_next().to_string(), row[5]);
    }
}

#[test]
fn divisor_as_iterable_returns_fresh_iterators_and_matches_jar_api() {
    for row in rows("S25_DIVISORITER") {
        let iterable: DivisorIterable = Divisor::as_iterable(i32f(row[1]), i32f(row[2]));
        let mut first = iterable.iterator();
        let mut second = iterable.iterator();
        let a = if first.has_next() {
            first.next_int().to_string()
        } else {
            "none".to_string()
        };
        let b = if second.has_next() {
            second.next_int().to_string()
        } else {
            "none".to_string()
        };
        assert_eq!(a, row[3]);
        assert_eq!(b, row[4]);
        assert_eq!(drain(&mut first), row[5]);
        assert_eq!(drain(&mut second), row[6]);
    }

    let mut want: Vec<String> = rows("S25_API")
        .iter()
        .filter(|row| row[1] == "Divisor")
        .map(|r| format!("{}\t{}\t{}\t{}", r[1], r[2], r[3], r[4]))
        .collect();
    let mut have = MEMBERS
        .iter()
        .map(|m| format!("{}\t{}\t{}\t{}", m.0, m.1, m.2, m.3))
        .collect::<Vec<_>>();
    want.sort();
    have.sort();
    assert_eq!(have, want, "Divisor declarations differ from the jar");
    let got: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|row| row[1] == "Divisor")
        .map(|r| (r[1].to_string(), r[2].to_string(), r[3].to_string()))
        .collect();
    assert_eq!(
        got,
        vec![(
            "Divisor".to_string(),
            "it.unimi.dsi.fastutil.ints.IntIterator".to_string(),
            "class".to_string()
        )]
    );
}

const MEMBERS: &[(&str, &str, &str, &str)] = &[
    ("Divisor", "ctor", "<init>(int,int)", "public"),
    ("Divisor", "field", "denominator:int", "private final"),
    ("Divisor", "field", "mod:int", "private final"),
    ("Divisor", "field", "quotient:int", "private final"),
    ("Divisor", "field", "remainder:int", "private"),
    ("Divisor", "field", "returnedParts:int", "private"),
    (
        "Divisor",
        "method",
        "asIterable(int,int)java.lang.Iterable<java.lang.Integer>",
        "public static",
    ),
    ("Divisor", "method", "hasNext()boolean", "public"),
    ("Divisor", "method", "nextInt()int", "public"),
];
