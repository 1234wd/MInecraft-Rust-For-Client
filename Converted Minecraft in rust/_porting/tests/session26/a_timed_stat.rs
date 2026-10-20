//! Session 26 sub-batch A: TimedStat, TickTimeStat, and the java.time values they expose.

use crate::sweep_probe::{esc, rows};
use minecraft_rust::javacompat::java_jfr::RecordedEvent;
use minecraft_rust::javacompat::java_time::{Duration, Instant};
use minecraft_rust::javacompat::throwable::ThrowableRef;
use minecraft_rust::net::minecraft::util::profiling::jfr::stats::TickTimeStat::TickTimeStat;
use minecraft_rust::net::minecraft::util::profiling::jfr::stats::TimedStat::TimedStat;

fn event() -> RecordedEvent {
    let start = Instant::of_epoch_second_adjusted(1_700_000_000, 123_456_789);
    let mut event = RecordedEvent::new(start);
    event.insert_string("level", Some("minecraft:overworld".to_string()));
    event.insert_int("fps", 144);
    event.insert_duration_nanos("averageTickDuration", 9_876_543_210);
    event
}

fn ex<T>(result: Result<T, ThrowableRef>) -> String {
    result.map_or_else(
        |error| format!("EX:{}", esc(&error.java_to_string())),
        |_| "ok".to_string(),
    )
}

#[test]
fn tick_time_stat_uses_recorded_event_start_and_timespan_duration() {
    let event = event();
    let stat = TickTimeStat::from(Some(&event))
        .unwrap_or_else(|error| panic!("{}", error.java_to_string()));
    let row = rows("S26_TICK")
        .into_iter()
        .find(|row| row[1] == "from")
        .unwrap();
    assert_eq!(
        row[2], "true",
        "the JAR stat retains the exact RecordedEvent start instant"
    );
    assert_eq!(stat.timestamp(), Some(event.get_start_time()));
    let expected = Duration::of_nanos(9_876_543_210);
    assert_eq!(stat.current_average(), Some(expected));
    assert_eq!(expected.get_seconds().to_string(), row[3]);
    assert_eq!(expected.get_nano().to_string(), row[4]);

    let error_row = |name: &str| {
        rows("S26_TICKERROR")
            .into_iter()
            .find(|row| row[1] == name)
            .unwrap_or_else(|| panic!("missing TickTimeStat oracle case {name}"))[2]
    };
    assert_eq!(ex(TickTimeStat::from(None)), error_row("null-event"));
    assert_eq!(
        ex(event.get_duration(Some("missing"))),
        error_row("missing-duration")
    );
    assert_eq!(
        ex(event.get_duration(Some("level"))),
        error_row("wrong-duration-type")
    );
    assert_eq!(
        ex(event.get_duration(Some("fps"))),
        error_row("missing-timespan")
    );
    assert_eq!(
        ex(event.get_duration(None)),
        error_row("null-duration-name")
    );

    for row in rows("S26_JFRDURATION") {
        let raw = row[2].parse::<i64>().unwrap();
        let mut edge_event = RecordedEvent::new(event.get_start_time());
        edge_event.insert_duration_nanos("averageTickDuration", raw);
        let duration = edge_event
            .get_duration(Some("averageTickDuration"))
            .unwrap_or_else(|error| panic!("{}", error.java_to_string()));
        assert_eq!(duration.get_seconds().to_string(), row[3]);
        assert_eq!(duration.get_nano().to_string(), row[4]);
        assert_eq!(duration.java_hash_code().to_string(), row[5]);
        assert_eq!(esc(&duration.java_to_string()), row[6]);
    }
}

#[test]
fn java_duration_and_instant_values_match_jdk_25_edges() {
    for row in rows("S26_DURATION") {
        let nanos = row[2].parse::<i64>().unwrap();
        let duration = Duration::of_nanos(nanos);
        assert_eq!(
            duration.get_seconds().to_string(),
            row[3],
            "Duration.ofNanos({nanos}) seconds"
        );
        assert_eq!(
            duration.get_nano().to_string(),
            row[4],
            "Duration.ofNanos({nanos}) nanos"
        );
        assert_eq!(
            duration.java_hash_code().to_string(),
            row[5],
            "Duration.ofNanos({nanos}) hashCode"
        );
        assert_eq!(
            esc(&duration.java_to_string()),
            row[6],
            "Duration.ofNanos({nanos}) toString"
        );
    }

    for row in rows("S26_DURATION_ADJ") {
        if row.len() == 5 {
            assert_eq!(row[4], "EX:java.lang.ArithmeticException: long overflow");
            let seconds = row[2].parse::<i64>().unwrap();
            let nanos = row[3].parse::<i64>().unwrap();
            assert!(Duration::try_of_seconds_adjusted(seconds, nanos).is_none());
            continue;
        }
        let seconds = row[2].parse::<i64>().unwrap();
        let nanos = row[3].parse::<i64>().unwrap();
        let duration = Duration::of_seconds_adjusted(seconds, nanos);
        assert_eq!(duration.get_seconds().to_string(), row[4]);
        assert_eq!(duration.get_nano().to_string(), row[5]);
        assert_eq!(duration.java_hash_code().to_string(), row[6]);
        assert_eq!(esc(&duration.java_to_string()), row[7]);
    }

    for row in rows("S26_INSTANT") {
        let seconds = row[2].parse::<i64>().unwrap();
        let nanos = row[3].parse::<i64>().unwrap();
        let instant = Instant::of_epoch_second_adjusted(seconds, nanos);
        assert_eq!(instant.get_epoch_second().to_string(), row[2]);
        assert_eq!(instant.get_nano().to_string(), row[3]);
        assert_eq!(instant.java_hash_code().to_string(), row[4]);
        assert_eq!(
            esc(&instant.java_to_string()),
            row[5],
            "Instant at {seconds}s + {nanos}ns"
        );
    }
}

#[test]
fn tick_time_record_methods_and_nullable_components_match_the_jar() {
    let fixed = TickTimeStat::new(
        Some(Instant::of_epoch_second_adjusted(-1, 500_000_000)),
        Some(Duration::of_seconds_adjusted(-1, 500_000_000)),
    );
    let same = TickTimeStat::new(
        Some(Instant::of_epoch_second_adjusted(-1, 500_000_000)),
        Some(Duration::of_nanos(-500_000_000)),
    );
    let different = TickTimeStat::new(fixed.timestamp(), Some(Duration::of_nanos(0)));
    let row = rows("S26_TICKRECORD")
        .into_iter()
        .find(|row| row[1] == "fixed")
        .unwrap();
    assert_eq!(esc(&fixed.timestamp().unwrap().java_to_string()), row[2]);
    assert_eq!(
        esc(&fixed.current_average().unwrap().java_to_string()),
        row[3]
    );
    assert_eq!(fixed.hash_code().to_string(), row[4]);
    assert_eq!(esc(&fixed.java_to_string()), row[5]);
    assert_eq!(fixed, same);
    assert_eq!(fixed.equals(&same).to_string(), row[6]);
    assert_ne!(fixed, different);
    assert_eq!(fixed.equals(&different).to_string(), row[7]);

    let nulls = TickTimeStat::new(None, None);
    let row = rows("S26_TICKRECORD")
        .into_iter()
        .find(|row| row[1] == "nulls")
        .unwrap();
    assert_eq!(nulls.hash_code().to_string(), row[2]);
    assert_eq!(esc(&nulls.java_to_string()), row[3]);
    assert_eq!(nulls, TickTimeStat::new(None, None));
    assert_eq!(
        nulls.equals(&TickTimeStat::new(None, None)).to_string(),
        row[4]
    );

    struct DurationView(Duration);
    impl TimedStat for DurationView {
        fn duration(&self) -> &Duration {
            &self.0
        }
    }
    let view = DurationView(Duration::of_nanos(-1));
    assert_eq!(view.duration(), &Duration::of_nanos(-1));
}

#[test]
fn timed_stat_declarations_match_the_26_2_jar() {
    let class_names = ["TimedStat", "TickTimeStat"];
    let mut want: Vec<String> = rows("S26_API")
        .iter()
        .filter(|row| class_names.contains(&row[1]))
        .map(|row| format!("{}\t{}\t{}\t{}", row[1], row[2], row[3], row[4]))
        .collect();
    let mut have = MEMBERS
        .iter()
        .map(|member| format!("{}\t{}\t{}\t{}", member.0, member.1, member.2, member.3))
        .collect::<Vec<_>>();
    want.sort();
    have.sort();
    assert_eq!(
        have, want,
        "TimedStat and TickTimeStat declarations differ from the jar"
    );

    let mut got: Vec<(String, String, String)> = rows("S26_SUP")
        .iter()
        .filter(|row| class_names.contains(&row[1]))
        .map(|row| (row[1].to_string(), row[2].to_string(), row[3].to_string()))
        .collect();
    got.sort();
    let mut expected = vec![
        (
            "TimedStat".to_string(),
            "-".to_string(),
            "interface".to_string(),
        ),
        (
            "TickTimeStat".to_string(),
            "extends java.lang.Record".to_string(),
            "record".to_string(),
        ),
    ];
    expected.sort();
    assert_eq!(got, expected);
}

const MEMBERS: &[(&str, &str, &str, &str)] = &[
    (
        "TimedStat",
        "method",
        "duration()java.time.Duration",
        "public abstract",
    ),
    (
        "TickTimeStat",
        "ctor",
        "<init>(java.time.Instant,java.time.Duration)",
        "public",
    ),
    (
        "TickTimeStat",
        "field",
        "currentAverage:java.time.Duration",
        "private final",
    ),
    (
        "TickTimeStat",
        "field",
        "timestamp:java.time.Instant",
        "private final",
    ),
    (
        "TickTimeStat",
        "method",
        "currentAverage()java.time.Duration",
        "public",
    ),
    (
        "TickTimeStat",
        "method",
        "equals(java.lang.Object)boolean",
        "public",
    ),
    (
        "TickTimeStat",
        "method",
        "from(jdk.jfr.consumer.RecordedEvent)net.minecraft.util.profiling.jfr.stats.TickTimeStat",
        "public static",
    ),
    ("TickTimeStat", "method", "hashCode()int", "public"),
    (
        "TickTimeStat",
        "method",
        "timestamp()java.time.Instant",
        "public",
    ),
    (
        "TickTimeStat",
        "method",
        "toString()java.lang.String",
        "public",
    ),
];
