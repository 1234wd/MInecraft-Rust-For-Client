//! Session 19 batch b: misc common enums and ChatFormatting. Checks ports against the jar's constant dump and the
//! Java source's behaviour. See the file headers of the ported .rs files for status.

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

/// The raw (unescaped, still `type:`-prefixed) value of one constant's dumped field, e.g. a
/// lambda identity string `str:...$$Lambda/...`.
fn raw_field(fq: &str, constant: &str, field: &str) -> String {
    let rows = const_rows(fq);
    let row = rows
        .iter()
        .find(|r| r.arg(0).as_opt_str() == Some(constant))
        .unwrap_or_else(|| panic!("{fq} has no constant `{constant}` in the dump"));
    let prefix = format!("{field}=");
    row.expect
        .iter()
        .filter_map(|v| v.as_opt_str())
        .find_map(|s| s.strip_prefix(prefix.as_str()).map(str::to_string))
        .unwrap_or_else(|| panic!("{fq}.{constant} has no `{field}=` entry"))
}

fn panic_text(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let err = std::panic::catch_unwind(f).expect_err("expected a panic");
    err.downcast_ref::<String>()
        .cloned()
        .or_else(|| err.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap()
}

/// Names + ordinals against the jar, discriminant == ordinal, and `Display` == `name()` (Java's
/// default `Enum#toString`).
macro_rules! check_plain_enum {
    ($fq:expr, $values:expr) => {{
        let values = $values;
        assert_enum_order($fq, &values.map(|v| v.name()));
        for (i, v) in values.iter().enumerate() {
            assert_eq!(v.ordinal(), i as i32, "{}", v.name());
            assert_eq!(v.ordinal(), *v as i32, "{}", v.name());
            assert_eq!(v.to_string(), v.name(), "Display must be Java's default toString");
        }
    }};
}

// ===========================================================================
// ChatFormatting
// ===========================================================================

#[test]
fn chat_formatting_matches_the_jar() {
    use minecraft_rust::net::minecraft::ChatFormatting::VALUES;
    let fq = "net.minecraft.ChatFormatting";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for (i, v) in VALUES.iter().enumerate() {
        assert_eq!(v.ordinal(), i as i32);
        assert_eq!(v.ordinal(), *v as i32);
        // `char code` is dumped as its int value.
        assert_enum_field(fq, v.name(), "code", &i32v(v.code() as i32));
        // The cached `toString` field, which is what `toString()` returns.
        assert_enum_field(fq, v.name(), "toString", &strv(&v.to_string()));
    }
}

#[test]
fn chat_formatting_to_string_is_the_section_sign_code_not_the_name() {
    use minecraft_rust::net::minecraft::ChatFormatting::{ChatFormatting, PREFIX_CODE, VALUES};
    assert_eq!(PREFIX_CODE, '\u{00A7}');
    for v in VALUES {
        assert_eq!(v.to_string(), format!("{PREFIX_CODE}{}", v.code()));
        assert_ne!(v.to_string(), v.name());
    }
    assert_eq!(ChatFormatting::Black.to_string(), "\u{00A7}0");
    assert_eq!(ChatFormatting::Reset.to_string(), "\u{00A7}r");
    assert_eq!(format!("{}x{}", ChatFormatting::Red, ChatFormatting::Reset), "\u{00A7}cx\u{00A7}r");
}

#[test]
fn chat_formatting_strip_formatting_follows_the_regex() {
    use minecraft_rust::net::minecraft::ChatFormatting::{strip_formatting, VALUES};
    let s = |x: &str| strip_formatting(Some(x)).unwrap();
    // `null` in, `null` out.
    assert_eq!(strip_formatting(None), None);
    assert_eq!(s(""), "");
    assert_eq!(s("plain"), "plain");
    assert_eq!(s("\u{00A7}aHello \u{00A7}lWorld\u{00A7}r"), "Hello World");
    // Every constant's own toString is stripped.
    for v in VALUES {
        assert_eq!(s(&v.to_string()), "", "{}", v.name());
    }
    // (?i): upper-case codes are stripped too.
    assert_eq!(s("\u{00A7}A\u{00A7}B\u{00A7}F\u{00A7}K\u{00A7}L\u{00A7}M\u{00A7}N\u{00A7}O\u{00A7}R"), "");
    // Not in [0-9A-FK-OR]: kept.
    let kept = "\u{00A7}g\u{00A7}j\u{00A7}p\u{00A7}q\u{00A7}s\u{00A7}x\u{00A7}z\u{00A7}G\u{00A7}X\u{00A7} ";
    assert_eq!(s(kept), kept);
    // One left-to-right pass, non-overlapping: the first `§` fails against `§`, the second matches.
    assert_eq!(s("\u{00A7}\u{00A7}aa"), "\u{00A7}a");
    assert_eq!(s("\u{00A7}\u{00A7}"), "\u{00A7}\u{00A7}");
    // A trailing prefix with nothing after it stays.
    assert_eq!(s("abc\u{00A7}"), "abc\u{00A7}");
    // Case folding is ASCII-only (no UNICODE_CASE): KELVIN SIGN after `§` is not a match.
    assert_eq!(s("\u{00A7}\u{212A}"), "\u{00A7}\u{212A}");
    // Non-ASCII and supplementary chars around matches survive intact.
    assert_eq!(s("\u{e9}\u{00A7}c\u{e9}"), "\u{e9}\u{e9}");
    assert_eq!(s("\u{00A7}\u{1F600}x\u{00A7}6"), "\u{00A7}\u{1F600}x");
}

#[test]
fn chat_formatting_strip_class_and_get_by_code_agree_on_ascii() {
    // For ASCII, `(?i)[0-9A-FK-OR]` and `getByCode(c) != null` describe the same set.
    use minecraft_rust::net::minecraft::ChatFormatting::{get_by_code, strip_formatting};
    for b in 0u8..128 {
        let c = b as char;
        let stripped = strip_formatting(Some(&format!("\u{00A7}{c}"))).unwrap().is_empty();
        assert_eq!(stripped, get_by_code(c).is_some(), "char {b}");
    }
}

#[test]
fn chat_formatting_get_by_code() {
    use minecraft_rust::net::minecraft::ChatFormatting::{get_by_code, ChatFormatting, VALUES};
    for v in VALUES {
        assert_eq!(get_by_code(v.code()), Some(v));
        assert_eq!(get_by_code(v.code().to_ascii_uppercase()), Some(v));
    }
    assert_eq!(get_by_code('A'), Some(ChatFormatting::Green));
    assert_eq!(get_by_code('R'), Some(ChatFormatting::Reset));
    assert_eq!(get_by_code('g'), None);
    assert_eq!(get_by_code('p'), None);
    assert_eq!(get_by_code('\u{00A7}'), None);
    assert_eq!(get_by_code(' '), None);
    // Character.toLowerCase is Unicode-aware: KELVIN SIGN lower-cases to 'k'.
    assert_eq!(get_by_code('\u{212A}'), Some(ChatFormatting::Obfuscated));
    // U+0130 lower-cases (simple mapping) to 'i', which is not a code.
    assert_eq!(get_by_code('\u{0130}'), None);
}

// ===========================================================================
// AdvancementType / MobEffectCategory (unblocked by ChatFormatting)
// ===========================================================================

#[test]
fn advancement_type_chat_color_matches_the_jar() {
    use minecraft_rust::net::minecraft::advancements::AdvancementType::AdvancementType;
    use minecraft_rust::net::minecraft::ChatFormatting::ChatFormatting;
    let fq = "net.minecraft.advancements.AdvancementType";
    for (v, name) in [
        (AdvancementType::Task, "TASK"),
        (AdvancementType::Challenge, "CHALLENGE"),
        (AdvancementType::Goal, "GOAL"),
    ] {
        // chatColor is dumped via name().
        assert_enum_field(fq, name, "chatColor", &strv(v.get_chat_color().name()));
    }
    assert_eq!(AdvancementType::Task.get_chat_color(), AdvancementType::Goal.get_chat_color());
    assert_eq!(AdvancementType::Challenge.get_chat_color(), ChatFormatting::DarkPurple);
}

#[test]
fn mob_effect_category_tooltip_formatting_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::effect::MobEffectCategory::{MobEffectCategory, VALUES};
    let fq = "net.minecraft.world.effect.MobEffectCategory";
    let names = ["BENEFICIAL", "HARMFUL", "NEUTRAL"];
    assert_enum_order(fq, &names);
    for (v, name) in VALUES.iter().zip(names) {
        assert_enum_field(fq, name, "tooltipFormatting", &strv(v.get_tooltip_formatting().name()));
    }
    assert_eq!(
        MobEffectCategory::Beneficial.get_tooltip_formatting(),
        MobEffectCategory::Neutral.get_tooltip_formatting()
    );
}

// ===========================================================================
// Fieldless enums
// ===========================================================================

#[test]
fn fieldless_enums_match_the_jar() {
    use minecraft_rust::net::minecraft as mc;
    check_plain_enum!("net.minecraft.util.debugchart.TpsDebugDimensions", mc::util::debugchart::TpsDebugDimensions::VALUES);
    check_plain_enum!("net.minecraft.world.entity.MoverType", mc::world::entity::MoverType::VALUES);
    check_plain_enum!("net.minecraft.world.level.material.FogType", mc::world::level::material::FogType::VALUES);
    check_plain_enum!("net.minecraft.world.level.material.PushReaction", mc::world::level::material::PushReaction::VALUES);
    check_plain_enum!(
        "net.minecraft.world.entity.monster.piglin.PiglinArmPose",
        mc::world::entity::monster::piglin::PiglinArmPose::VALUES
    );
    check_plain_enum!("net.minecraft.server.level.FullChunkStatus", mc::server::level::FullChunkStatus::VALUES);
    check_plain_enum!(
        "net.minecraft.network.protocol.handshake.ClientIntent",
        mc::network::protocol::handshake::ClientIntent::VALUES
    );
    check_plain_enum!("net.minecraft.world.entity.EntitySpawnReason", mc::world::entity::EntitySpawnReason::VALUES);
    check_plain_enum!(
        "net.minecraft.util.debugchart.RemoteDebugSampleType",
        mc::util::debugchart::RemoteDebugSampleType::VALUES
    );
    check_plain_enum!(
        "net.minecraft.world.entity.InsideBlockEffectType",
        mc::world::entity::InsideBlockEffectType::VALUES
    );
    check_plain_enum!(
        "net.minecraft.server.packs.repository.PackCompatibility",
        mc::server::packs::repository::PackCompatibility::VALUES
    );
}

#[test]
fn mover_type_self_keeps_its_java_name() {
    use minecraft_rust::net::minecraft::world::entity::MoverType::MoverType;
    assert_eq!(MoverType::SelfMove.name(), "SELF");
    assert_eq!(MoverType::SelfMove.ordinal(), 0);
}

// ===========================================================================
// FullChunkStatus / Visibility
// ===========================================================================

#[test]
fn full_chunk_status_is_or_after_is_an_ordinal_comparison() {
    use minecraft_rust::net::minecraft::server::level::FullChunkStatus::{FullChunkStatus as S, VALUES};
    for v in VALUES {
        assert!(v.is_or_after(v), "{v} is_or_after itself");
        assert!(v.is_or_after(S::Inaccessible));
        assert!(S::EntityTicking.is_or_after(v));
    }
    assert!(S::Full.is_or_after(S::Inaccessible));
    assert!(!S::Inaccessible.is_or_after(S::Full));
    assert!(S::BlockTicking.is_or_after(S::Full));
    assert!(!S::BlockTicking.is_or_after(S::EntityTicking));
    assert!(!S::Full.is_or_after(S::BlockTicking));
}

#[test]
fn visibility_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::level::entity::Visibility::VALUES;
    let fq = "net.minecraft.world.level.entity.Visibility";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "accessible", &boolv(v.is_accessible()));
        assert_enum_field(fq, v.name(), "ticking", &boolv(v.is_ticking()));
    }
}

#[test]
fn visibility_from_full_chunk_status() {
    use minecraft_rust::net::minecraft::server::level::FullChunkStatus::FullChunkStatus as S;
    use minecraft_rust::net::minecraft::world::level::entity::Visibility::{from_full_chunk_status, Visibility};
    assert_eq!(from_full_chunk_status(S::Inaccessible), Visibility::Hidden);
    assert_eq!(from_full_chunk_status(S::Full), Visibility::Tracked);
    // Block-ticking chunks track their entities but do not tick them.
    assert_eq!(from_full_chunk_status(S::BlockTicking), Visibility::Tracked);
    assert_eq!(from_full_chunk_status(S::EntityTicking), Visibility::Ticking);
}

// ===========================================================================
// Enums with simple fields
// ===========================================================================

#[test]
fn pack_type_matches_the_jar() {
    use minecraft_rust::net::minecraft::server::packs::PackType::VALUES;
    let fq = "net.minecraft.server.packs.PackType";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "directory", &strv(v.get_directory()));
    }
}

#[test]
fn border_status_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::level::border::BorderStatus::{BorderStatus, VALUES};
    let fq = "net.minecraft.world.level.border.BorderStatus";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "color", &i32v(v.get_color()));
    }
    assert_eq!(BorderStatus::Growing.get_color(), 0x40FF80);
    assert_eq!(BorderStatus::Shrinking.get_color(), 0xFF3030);
    assert_eq!(BorderStatus::Stationary.get_color(), 0x20A0FF);
}

#[test]
fn connection_protocol_matches_the_jar() {
    use minecraft_rust::net::minecraft::network::ConnectionProtocol::VALUES;
    let fq = "net.minecraft.network.ConnectionProtocol";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "id", &strv(v.id()));
    }
}

#[test]
fn recipe_category_matches_the_jar() {
    use minecraft_rust::net::minecraft::data::recipes::RecipeCategory::VALUES;
    let fq = "net.minecraft.data.recipes.RecipeCategory";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "recipeFolderName", &strv(v.get_folder_name()));
    }
}

#[test]
fn metric_category_matches_the_jar() {
    use minecraft_rust::net::minecraft::util::profiling::metrics::MetricCategory::VALUES;
    let fq = "net.minecraft.util.profiling.metrics.MetricCategory";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "description", &strv(v.get_description()));
    }
}

#[test]
fn sound_source_matches_the_jar() {
    use minecraft_rust::net::minecraft::sounds::SoundSource::{SoundSource, VALUES};
    let fq = "net.minecraft.sounds.SoundSource";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "name", &strv(v.get_name()));
    }
    // name() and getName() are different strings.
    assert_eq!(SoundSource::Records.name(), "RECORDS");
    assert_eq!(SoundSource::Records.get_name(), "record");
}

#[test]
fn path_type_malus_bits_match_the_jar() {
    use minecraft_rust::net::minecraft::world::level::pathfinder::PathType::VALUES;
    let fq = "net.minecraft.world.level.pathfinder.PathType";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "malus", &f32v(v.get_malus()));
    }
}

// ===========================================================================
// ClientIntent
// ===========================================================================

#[test]
fn client_intent_ids_round_trip_and_are_ordinal_plus_one() {
    use minecraft_rust::net::minecraft::network::protocol::handshake::ClientIntent::{by_id, ClientIntent, VALUES};
    for v in VALUES {
        assert_eq!(v.id(), v.ordinal() + 1);
        assert_eq!(by_id(v.id()), v);
    }
    assert_eq!(by_id(1), ClientIntent::Status);
    assert_eq!(by_id(2), ClientIntent::Login);
    assert_eq!(by_id(3), ClientIntent::Transfer);
}

#[test]
fn client_intent_unknown_id_throws_illegal_argument() {
    use minecraft_rust::net::minecraft::network::protocol::handshake::ClientIntent::by_id;
    for id in [0, 4, -1, i32::MIN, i32::MAX] {
        assert_eq!(
            panic_text(move || {
                by_id(id);
            }),
            format!("java.lang.IllegalArgumentException: Unknown connection intent: {id}")
        );
    }
}

// ===========================================================================
// EntityAttachment
// ===========================================================================

#[test]
fn entity_attachment_matches_the_jar_including_shared_lambdas() {
    use minecraft_rust::net::minecraft::world::entity::EntityAttachment::VALUES;
    let fq = "net.minecraft.world.entity.EntityAttachment";
    check_plain_enum!(fq, VALUES);
    // The dump records each constant's lambda identity: two constants share a fallback in the
    // port exactly when they hold the same lambda object in the jar.
    for a in VALUES {
        let ja = raw_field(fq, a.name(), "fallback");
        assert!(ja.contains("EntityAttachment$Fallback$$Lambda"), "{ja}");
        for b in VALUES {
            let same_in_jar = ja == raw_field(fq, b.name(), "fallback");
            assert_eq!(a.fallback() == b.fallback(), same_in_jar, "{a} vs {b}");
        }
    }
}

#[test]
fn entity_attachment_fallback_points() {
    use minecraft_rust::net::minecraft::world::entity::EntityAttachment::{
        EntityAttachment as A, Fallback, FallbackConstant, ZERO,
    };
    let pts = |a: A, w: f32, h: f32| -> Vec<(u64, u64, u64)> {
        a.create_fallback_points(w, h)
            .iter()
            .map(|p| (p.x().to_bits(), p.y().to_bits(), p.z().to_bits()))
            .collect()
    };
    let h = 1.8f32;
    // AT_FEET returns ZERO: one point at the origin.
    assert_eq!(pts(A::Vehicle, 0.6, h), vec![(0, 0, 0)]);
    assert_eq!(ZERO().len(), 1);
    assert_eq!((ZERO()[0].x(), ZERO()[0].y(), ZERO()[0].z()), (0.0, 0.0, 0.0));
    // AT_HEIGHT widens the float height to double.
    assert_eq!(pts(A::Passenger, 0.6, h), vec![(0, (h as f64).to_bits(), 0)]);
    assert_eq!(pts(A::NameTag, 0.6, h), vec![(0, (h as f64).to_bits(), 0)]);
    assert_eq!(A::Passenger.create_fallback_points(0.6, h)[0].y(), 1.7999999523162842);
    // AT_CENTER: height / 2.0 in double.
    assert_eq!(A::WardenChest.create_fallback_points(0.6, h)[0].y(), 0.8999999761581421);
    // Width is ignored by every lambda.
    for a in [A::Passenger, A::Vehicle, A::NameTag, A::WardenChest] {
        assert_eq!(pts(a, 0.0, h), pts(a, 123.5, h));
    }
    // `float / double`: the smallest subnormal halves to a non-zero double (f32 halving would
    // round it to 0).
    let tiny = f32::from_bits(1);
    let y = FallbackConstant::AtCenter.create(1.0, tiny)[0].y();
    assert_eq!(y, tiny as f64 / 2.0);
    assert_ne!(y, 0.0);
    assert_eq!((tiny / 2.0) as f64, 0.0);
}

// ===========================================================================
// EntitySpawnReason
// ===========================================================================

#[test]
fn entity_spawn_reason_predicates() {
    use minecraft_rust::net::minecraft::world::entity::EntitySpawnReason::{
        ignores_light_requirements, is_spawner, VALUES,
    };
    let spawners: Vec<&str> = VALUES.iter().filter(|v| is_spawner(**v)).map(|v| v.name()).collect();
    assert_eq!(spawners, ["SPAWNER", "TRIAL_SPAWNER"]);
    let no_light: Vec<&str> = VALUES
        .iter()
        .filter(|v| ignores_light_requirements(**v))
        .map(|v| v.name())
        .collect();
    assert_eq!(no_light, ["TRIAL_SPAWNER"]);
}

// ===========================================================================
// TickPriority
// ===========================================================================

#[test]
fn tick_priority_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::ticks::TickPriority::VALUES;
    let fq = "net.minecraft.world.ticks.TickPriority";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "value", &i32v(v.get_value()));
    }
}

#[test]
fn tick_priority_by_value_clamps_out_of_range() {
    use minecraft_rust::net::minecraft::world::ticks::TickPriority::{by_value, TickPriority, VALUES};
    for v in VALUES {
        assert_eq!(by_value(v.get_value()), v);
    }
    assert_eq!(by_value(-4), TickPriority::ExtremelyHigh);
    assert_eq!(by_value(i32::MIN), TickPriority::ExtremelyHigh);
    assert_eq!(by_value(4), TickPriority::ExtremelyLow);
    assert_eq!(by_value(i32::MAX), TickPriority::ExtremelyLow);
}

// ===========================================================================
// Direction8
// ===========================================================================

#[test]
fn direction8_matches_the_jar() {
    use minecraft_rust::net::minecraft::core::Direction8::VALUES;
    let fq = "net.minecraft.core.Direction8";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        // Set#toString of the immutable enum set: "[north, east]".
        let set: Vec<String> = v.get_directions().iter().map(|d| d.to_string()).collect();
        assert_enum_field(fq, v.name(), "directions", &strv(&format!("[{}]", set.join(", "))));
        // Vec3i#toString: "Vec3i{x=1, y=0, z=-1}".
        assert_enum_field(fq, v.name(), "step", &strv(&v.step().to_string()));
        assert_eq!(v.get_step_x(), v.step().get_x());
        assert_eq!(v.get_step_z(), v.step().get_z());
        assert_eq!(v.step().get_y(), 0);
    }
}

#[test]
fn direction8_sets_iterate_in_direction_ordinal_order() {
    use minecraft_rust::net::minecraft::core::Direction8::VALUES;
    for v in VALUES {
        let ords: Vec<i32> = v.get_directions().iter().map(|d| d.ordinal()).collect();
        let mut sorted = ords.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(ords, sorted, "{v}");
    }
}

// ===========================================================================
// Rarity
// ===========================================================================

#[test]
fn rarity_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::item::Rarity::VALUES;
    let fq = "net.minecraft.world.item.Rarity";
    check_plain_enum!(fq, VALUES);
    for v in VALUES {
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_enum_field(fq, v.name(), "color", &strv(v.color().name()));
    }
}

#[test]
fn rarity_by_id_uses_the_zero_strategy() {
    use minecraft_rust::net::minecraft::world::item::Rarity::{Rarity, BY_ID, VALUES};
    for v in VALUES {
        assert_eq!(BY_ID(v.id()), v);
    }
    for id in [-1, 4, 5, i32::MIN, i32::MAX] {
        assert_eq!(BY_ID(id), Rarity::Common, "id {id}");
    }
}

#[test]
fn rarity_is_string_representable() {
    use minecraft_rust::net::minecraft::util::StringRepresentable::{from_enum, StringRepresentable};
    use minecraft_rust::net::minecraft::world::item::Rarity::{Rarity, VALUES};
    let codec = from_enum(&VALUES);
    for v in VALUES {
        assert_eq!(codec.by_name(StringRepresentable::get_serialized_name(&v)), Some(v));
    }
    assert_eq!(codec.by_name("EPIC"), None);
    assert_eq!(codec.by_name("epic"), Some(Rarity::Epic));
}

// ===========================================================================
// PackCompatibility / RemoteDebugSampleType / InsideBlockEffectType
// ===========================================================================

#[test]
fn pack_compatibility_is_compatible_and_unknown_version() {
    use minecraft_rust::net::minecraft::server::packs::repository::PackCompatibility::{
        PackCompatibility, UNKNOWN_VERSION, VALUES,
    };
    assert_eq!(UNKNOWN_VERSION, i32::MAX);
    let ok: Vec<&str> = VALUES.iter().filter(|v| v.is_compatible()).map(|v| v.name()).collect();
    assert_eq!(ok, ["COMPATIBLE"]);
    assert!(!PackCompatibility::Unknown.is_compatible());
}

#[test]
fn pack_compatibility_translation_keys_documented_in_the_port_match_the_jar() {
    // Pins the module docs (the Component fields themselves are PORT-BLOCKED).
    let fq = "net.minecraft.server.packs.repository.PackCompatibility";
    for (c, key) in [("TOO_OLD", "old"), ("TOO_NEW", "new"), ("UNKNOWN", "unknown"), ("COMPATIBLE", "compatible")] {
        assert_enum_field(
            fq,
            c,
            "description",
            &strv(&format!("translation{{key='pack.incompatible.{key}', args=[]}}[style={{color=gray}}]")),
        );
        assert_enum_field(
            fq,
            c,
            "confirmation",
            &strv(&format!("translation{{key='pack.incompatible.confirm.{key}', args=[]}}")),
        );
    }
}

#[test]
fn remote_debug_sample_type_subscription_documented_in_the_port_matches_the_jar() {
    assert_enum_field(
        "net.minecraft.util.debugchart.RemoteDebugSampleType",
        "TICK_TIME",
        "subscription",
        &strv("minecraft:dedicated_server_tick_time"),
    );
}

#[test]
fn inside_block_effect_types_each_hold_a_distinct_lambda_in_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::InsideBlockEffectType::VALUES;
    let fq = "net.minecraft.world.entity.InsideBlockEffectType";
    let mut effects: Vec<String> = VALUES.iter().map(|v| raw_field(fq, v.name(), "effect")).collect();
    effects.sort();
    effects.dedup();
    assert_eq!(effects.len(), VALUES.len());
}
