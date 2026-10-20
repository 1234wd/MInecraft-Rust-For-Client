//! Session 25 sub-batch O: four small JFR records and their typed event-field helper.

use crate::sweep_probe::*;
use minecraft_rust::javacompat::java_jfr::RecordedEvent;
use minecraft_rust::javacompat::java_time::Instant;
use minecraft_rust::net::minecraft::util::profiling::jfr::stats::ChunkIdentification::ChunkIdentification;
use minecraft_rust::net::minecraft::util::profiling::jfr::stats::CpuLoadStat::CpuLoadStat;
use minecraft_rust::net::minecraft::util::profiling::jfr::stats::FpsStat::FpsStat;
use minecraft_rust::net::minecraft::util::profiling::jfr::stats::PacketIdentification::PacketIdentification;

fn event() -> RecordedEvent {
    let mut event = RecordedEvent::new(Instant::of_epoch_second(0));
    event.insert_string("level", Some("minecraft:overworld".to_string()));
    event.insert_string("dimension", Some("minecraft:overworld".to_string()));
    event.insert_int("chunkPosX", -123);
    event.insert_int("chunkPosZ", 456);
    event.insert_float("jvmSystem", 0.5);
    event.insert_float("jvmUser", -0.0);
    event.insert_float("machineTotal", 0.125);
    event.insert_int("fps", 144);
    event.insert_string("packetDirection", Some("PLAY_TO_SERVER".to_string()));
    event.insert_string("protocolId", Some("minecraft:game".to_string()));
    event.insert_string("packetId", Some("minecraft:chat_command".to_string()));
    event
}

fn ex<T>(result: Result<T, minecraft_rust::javacompat::throwable::ThrowableRef>) -> String {
    result.map_or_else(
        |error| format!("EX:{}", esc(&error.java_to_string())),
        |_| "ok".to_string(),
    )
}

fn throwable_row(name: &str) -> String {
    rows("S25_JFRSTAT")
        .into_iter()
        .find(|row| row[1] == name)
        .unwrap_or_else(|| panic!("missing JFR oracle row {name}"))[2]
        .to_string()
}

#[test]
fn jfr_record_factories_fields_hashes_and_strings_match_a_real_recorded_event() {
    let event = event();
    let chunk = ChunkIdentification::from(Some(&event))
        .unwrap_or_else(|e| panic!("{}", e.java_to_string()));
    let row = rows("S25_JFRSTAT")
        .into_iter()
        .find(|row| row[1] == "chunk")
        .unwrap();
    assert_eq!(esc_opt(chunk.level()), row[2]);
    assert_eq!(esc_opt(chunk.dimension()), row[3]);
    assert_eq!(chunk.x().to_string(), row[4]);
    assert_eq!(chunk.z().to_string(), row[5]);
    assert_eq!(chunk.hash_code().to_string(), row[6]);
    assert_eq!(esc(&chunk.to_string()), row[7]);

    let cpu = CpuLoadStat::from(Some(&event)).unwrap_or_else(|e| panic!("{}", e.java_to_string()));
    let row = rows("S25_JFRSTAT")
        .into_iter()
        .find(|row| row[1] == "cpu")
        .unwrap();
    assert_eq!(dbits(cpu.jvm()), row[2]);
    assert_eq!(dbits(cpu.user_jvm()), row[3]);
    assert_eq!(dbits(cpu.system()), row[4]);
    assert_eq!(cpu.hash_code().to_string(), row[5]);
    assert_eq!(esc(&cpu.to_string()), row[6]);

    let fps = FpsStat::from(Some(&event), Some("fps"))
        .unwrap_or_else(|e| panic!("{}", e.java_to_string()));
    let row = rows("S25_JFRSTAT")
        .into_iter()
        .find(|row| row[1] == "fps")
        .unwrap();
    assert_eq!(fps.fps().to_string(), row[2]);
    assert_eq!(fps.hash_code().to_string(), row[3]);
    assert_eq!(esc(&fps.to_string()), row[4]);

    let packet = PacketIdentification::from(Some(&event))
        .unwrap_or_else(|e| panic!("{}", e.java_to_string()));
    let row = rows("S25_JFRSTAT")
        .into_iter()
        .find(|row| row[1] == "packet")
        .unwrap();
    assert_eq!(esc_opt(packet.direction()), row[2]);
    assert_eq!(esc_opt(packet.protocol_id()), row[3]);
    assert_eq!(esc_opt(packet.packet_id()), row[4]);
    assert_eq!(packet.hash_code().to_string(), row[5]);
    assert_eq!(esc(&packet.to_string()), row[6]);
}

#[test]
fn jfr_getter_errors_null_receivers_and_record_null_ieee_edges_match_the_jar() {
    let event = event();
    assert_eq!(
        ex(event.get_int(Some("missing"))),
        throwable_row("bad-int-field")
    );
    assert_eq!(
        ex(event.get_int(Some("level"))),
        throwable_row("wrong-int-type")
    );
    assert_eq!(
        ex(event.get_string(Some("fps"))),
        throwable_row("wrong-string-type")
    );
    assert_eq!(
        ex(ChunkIdentification::from(None)),
        throwable_row("null-event")
    );
    assert_eq!(
        ex(FpsStat::from(None, Some("fps"))),
        throwable_row("null-fps-event")
    );
    assert_eq!(
        ex(FpsStat::from(Some(&event), None)),
        throwable_row("null-fps-field")
    );
    assert_eq!(
        ex(FpsStat::from(Some(&event), Some("missing"))),
        throwable_row("missing-fps")
    );

    let null_chunk = ChunkIdentification::new(None, None, -1, 2);
    let row = rows("S25_JFRSTAT")
        .into_iter()
        .find(|row| row[1] == "null-chunk")
        .unwrap();
    assert_eq!(esc(&null_chunk.to_string()), row[2]);
    assert_eq!(null_chunk.hash_code().to_string(), row[3]);
    assert_eq!(null_chunk, ChunkIdentification::new(None, None, -1, 2));
    assert_eq!(
        (null_chunk == ChunkIdentification::new(None, None, -1, 2)).to_string(),
        row[4]
    );

    let null_packet = PacketIdentification::new(None, None, None);
    let row = rows("S25_JFRSTAT")
        .into_iter()
        .find(|row| row[1] == "null-packet")
        .unwrap();
    assert_eq!(esc(&null_packet.to_string()), row[2]);
    assert_eq!(null_packet.hash_code().to_string(), row[3]);
    assert_eq!(
        (null_packet == PacketIdentification::new(None, None, None)).to_string(),
        row[4]
    );

    let nan = CpuLoadStat::new(f64::NAN, -0.0, 1.0);
    let same_nan = CpuLoadStat::new(f64::from_bits(0x7ff0_0000_0000_0001), -0.0, 1.0);
    let signed_zero = CpuLoadStat::new(f64::NAN, 0.0, 1.0);
    let row = rows("S25_JFRSTAT")
        .into_iter()
        .find(|row| row[1] == "cpu-ieee")
        .unwrap();
    assert_eq!((nan == same_nan).to_string(), row[2]);
    assert_eq!((nan == signed_zero).to_string(), row[3]);
    assert_eq!(nan.hash_code().to_string(), row[4]);
}

#[test]
fn jfr_stat_record_declarations_match_the_26_2_jar() {
    let class_names = [
        "ChunkIdentification",
        "CpuLoadStat",
        "FpsStat",
        "PacketIdentification",
    ];
    let mut want: Vec<String> = rows("S25_API")
        .iter()
        .filter(|row| class_names.contains(&row[1]))
        .map(|row| format!("{}\t{}\t{}\t{}", row[1], row[2], row[3], row[4]))
        .collect();
    let mut have = MEMBERS
        .iter()
        .map(|m| format!("{}\t{}\t{}\t{}", m.0, m.1, m.2, m.3))
        .collect::<Vec<_>>();
    want.sort();
    have.sort();
    assert_eq!(have, want, "JFR record declarations differ from the jar");

    let mut got: Vec<(String, String, String)> = rows("S25_SUP")
        .iter()
        .filter(|row| class_names.contains(&row[1]))
        .map(|row| (row[1].to_string(), row[2].to_string(), row[3].to_string()))
        .collect();
    got.sort();
    let mut expected = class_names
        .iter()
        .map(|name| {
            (
                name.to_string(),
                "extends java.lang.Record".to_string(),
                "record".to_string(),
            )
        })
        .collect::<Vec<_>>();
    expected.sort();
    assert_eq!(got, expected);
}

const MEMBERS: &[(&str, &str, &str, &str)] = &[
    ("ChunkIdentification", "ctor", "<init>(java.lang.String,java.lang.String,int,int)", "public"),
    ("ChunkIdentification", "field", "dimension:java.lang.String", "private final"),
    ("ChunkIdentification", "field", "level:java.lang.String", "private final"),
    ("ChunkIdentification", "field", "x:int", "private final"),
    ("ChunkIdentification", "field", "z:int", "private final"),
    ("ChunkIdentification", "method", "dimension()java.lang.String", "public"),
    ("ChunkIdentification", "method", "equals(java.lang.Object)boolean", "public"),
    ("ChunkIdentification", "method", "from(jdk.jfr.consumer.RecordedEvent)net.minecraft.util.profiling.jfr.stats.ChunkIdentification", "public static"),
    ("ChunkIdentification", "method", "hashCode()int", "public"),
    ("ChunkIdentification", "method", "level()java.lang.String", "public"),
    ("ChunkIdentification", "method", "toString()java.lang.String", "public"),
    ("ChunkIdentification", "method", "x()int", "public"),
    ("ChunkIdentification", "method", "z()int", "public"),
    ("CpuLoadStat", "ctor", "<init>(double,double,double)", "public"),
    ("CpuLoadStat", "field", "jvm:double", "private final"),
    ("CpuLoadStat", "field", "system:double", "private final"),
    ("CpuLoadStat", "field", "userJvm:double", "private final"),
    ("CpuLoadStat", "method", "equals(java.lang.Object)boolean", "public"),
    ("CpuLoadStat", "method", "from(jdk.jfr.consumer.RecordedEvent)net.minecraft.util.profiling.jfr.stats.CpuLoadStat", "public static"),
    ("CpuLoadStat", "method", "hashCode()int", "public"),
    ("CpuLoadStat", "method", "jvm()double", "public"),
    ("CpuLoadStat", "method", "system()double", "public"),
    ("CpuLoadStat", "method", "toString()java.lang.String", "public"),
    ("CpuLoadStat", "method", "userJvm()double", "public"),
    ("FpsStat", "ctor", "<init>(int)", "public"),
    ("FpsStat", "field", "fps:int", "private final"),
    ("FpsStat", "method", "equals(java.lang.Object)boolean", "public"),
    ("FpsStat", "method", "fps()int", "public"),
    ("FpsStat", "method", "from(jdk.jfr.consumer.RecordedEvent,java.lang.String)net.minecraft.util.profiling.jfr.stats.FpsStat", "public static"),
    ("FpsStat", "method", "hashCode()int", "public"),
    ("FpsStat", "method", "toString()java.lang.String", "public"),
    ("PacketIdentification", "ctor", "<init>(java.lang.String,java.lang.String,java.lang.String)", "public"),
    ("PacketIdentification", "field", "direction:java.lang.String", "private final"),
    ("PacketIdentification", "field", "packetId:java.lang.String", "private final"),
    ("PacketIdentification", "field", "protocolId:java.lang.String", "private final"),
    ("PacketIdentification", "method", "direction()java.lang.String", "public"),
    ("PacketIdentification", "method", "equals(java.lang.Object)boolean", "public"),
    ("PacketIdentification", "method", "from(jdk.jfr.consumer.RecordedEvent)net.minecraft.util.profiling.jfr.stats.PacketIdentification", "public static"),
    ("PacketIdentification", "method", "hashCode()int", "public"),
    ("PacketIdentification", "method", "packetId()java.lang.String", "public"),
    ("PacketIdentification", "method", "protocolId()java.lang.String", "public"),
    ("PacketIdentification", "method", "toString()java.lang.String", "public"),
];
