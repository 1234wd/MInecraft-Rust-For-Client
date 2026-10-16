//! Session 19 checks: ports verified against the jar's constant dump (`consts.txt`) and the
//! behaviour they unblock.

#[path = "support/consts_dump.rs"]
mod consts_dump;
use consts_dump::*;

// ===========================================================================
// Shared dependencies: StringRepresentable, ByIdMap
// ===========================================================================

#[test]
fn string_representable_threshold_matches_the_jar() {
    use minecraft_rust::net::minecraft::util::StringRepresentable::PRE_BUILT_MAP_THRESHOLD;
    assert_all_consts(
        "net.minecraft.util.StringRepresentable",
        &[("PRE_BUILT_MAP_THRESHOLD", i32v(PRE_BUILT_MAP_THRESHOLD))],
    );
}

#[test]
fn difficulty_by_id_wraps_and_by_name_is_exact() {
    use minecraft_rust::net::minecraft::world::Difficulty::{by_id, by_name, Difficulty};
    assert_eq!(by_id(0), Difficulty::Peaceful);
    assert_eq!(by_id(3), Difficulty::Hard);
    assert_eq!(by_id(4), Difficulty::Peaceful, "WRAP: 4 floorMod 4 == 0");
    assert_eq!(by_id(-1), Difficulty::Hard, "WRAP is floorMod: -1 -> 3");
    assert_eq!(by_id(i32::MIN), Difficulty::Peaceful);
    assert_eq!(by_name("normal"), Some(Difficulty::Normal));
    assert_eq!(by_name("NORMAL"), None);
    assert_eq!(by_name(""), None);
    assert_eq!(Difficulty::Hard.to_string(), "HARD");
}

#[test]
fn interaction_hand_and_dialog_action_by_id_use_zero_strategy() {
    use minecraft_rust::net::minecraft::server::dialog::DialogAction::{DialogAction, BY_ID};
    use minecraft_rust::net::minecraft::world::InteractionHand::{by_id, InteractionHand};
    assert_eq!(by_id(1), InteractionHand::OffHand);
    assert_eq!(by_id(2), InteractionHand::MainHand, "ZERO: out of range -> id 0");
    assert_eq!(by_id(-1), InteractionHand::MainHand);
    assert_eq!(BY_ID(2), DialogAction::WaitForResponse);
    assert_eq!(BY_ID(3), DialogAction::Close);
    assert_eq!(BY_ID(-7), DialogAction::Close);
}

// ===========================================================================
// Identifier: read(String) and resolveAgainst(Path), unblocked in session 19
// ===========================================================================

#[test]
fn identifier_read_string_matches_data_result_text() {
    use minecraft_rust::net::minecraft::resources::Identifier::read_string;
    assert_eq!(read_string("stone").unwrap().to_string(), "minecraft:stone");
    assert_eq!(read_string(":stone").unwrap().to_string(), "minecraft:stone");
    assert_eq!(read_string("mod:a/b").unwrap().to_string(), "mod:a/b");
    assert_eq!(
        read_string("Mod:x").unwrap_err(),
        "Not a valid resource location: Mod:x Non [a-z0-9_.-] character in namespace of identifier: Mod:x"
    );
    assert_eq!(
        read_string("a:B").unwrap_err(),
        "Not a valid resource location: a:B Non [a-z0-9/._-] character in path of location: a:B"
    );
    // namespace is checked first, like createUntrusted's argument order
    assert_eq!(
        read_string("A:B").unwrap_err(),
        "Not a valid resource location: A:B Non [a-z0-9_.-] character in namespace of identifier: A:B"
    );
    // the raw input is NOT escaped; only getMessage() is
    assert_eq!(
        read_string("a\nb").unwrap_err(),
        "Not a valid resource location: a\nb Non [a-z0-9/._-] character in path of location: minecraft:a\\nb"
    );
}

#[test]
fn identifier_resolve_against_returns_unnormalized_path_inside_root() {
    use minecraft_rust::net::minecraft::resources::Identifier::parse;
    use std::path::{Path, PathBuf};
    let root = Path::new("packs").join("data");
    let got = parse("mod:textures/a.png").resolve_against(&root);
    assert_eq!(got, root.join("mod").join("textures").join("a.png"));
    // `..` that stays inside the root is allowed and is NOT normalized away in the result
    let got = parse("mod:x/../y").resolve_against(&root);
    assert_eq!(got, PathBuf::from(format!("{}{s}mod{s}x{s}..{s}y", root.display(), s = std::path::MAIN_SEPARATOR)));
}

#[test]
fn identifier_resolve_against_rejects_escape_with_javas_message() {
    use minecraft_rust::net::minecraft::resources::Identifier::parse;
    use std::path::Path;
    let root = Path::new("root");
    // root/minecraft/../../etc normalizes to plain `etc`: both `..` cancel a name.
    let err = std::panic::catch_unwind(|| parse("minecraft:../../etc").resolve_against(root))
        .expect_err("escaping the root must throw");
    let msg = err.downcast_ref::<String>().unwrap();
    assert_eq!(
        msg,
        "java.lang.IllegalStateException: Identifier \"minecraft:../../etc\" tried to access path \"etc\" from root \"root\""
    );
    // One more `..` has nothing left to cancel on a relative path, and Path#normalize keeps it.
    let err = std::panic::catch_unwind(|| parse("minecraft:../../../etc").resolve_against(root))
        .expect_err("escaping the root must throw");
    let msg = err.downcast_ref::<String>().unwrap();
    let s = std::path::MAIN_SEPARATOR;
    assert_eq!(
        msg,
        &format!(
            "java.lang.IllegalStateException: Identifier \"minecraft:../../../etc\" tried to access path \"..{s}etc\" from root \"root\""
        )
    );
}

// ===========================================================================
// Client enums (session 19, coordinator batch)
// ===========================================================================

macro_rules! check_enum_against_dump {
    ($module:path, $fq:expr) => {{
        use $module as m;
        assert_enum_order($fq, &m::VALUES.map(|v| v.name()));
        for (i, v) in m::VALUES.iter().enumerate() {
            assert_eq!(v.ordinal(), i as i32);
            assert_eq!(v.to_string(), v.name());
        }
    }};
}

#[test]
fn plain_client_enums_match_the_jar() {
    check_enum_against_dump!(minecraft_rust::net::minecraft::client::gui::components::Whence, "net.minecraft.client.gui.components.Whence");
    check_enum_against_dump!(minecraft_rust::net::minecraft::client::multiplayer::chat::GuiMessageSource, "net.minecraft.client.multiplayer.chat.GuiMessageSource");
    check_enum_against_dump!(minecraft_rust::net::minecraft::client::gui::narration::NarratedElementType, "net.minecraft.client.gui.narration.NarratedElementType");
    check_enum_against_dump!(minecraft_rust::net::minecraft::client::InputType, "net.minecraft.client.InputType");
    check_enum_against_dump!(minecraft_rust::net::minecraft::client::gui::navigation::ScreenAxis, "net.minecraft.client.gui.navigation.ScreenAxis");
    check_enum_against_dump!(minecraft_rust::net::minecraft::client::gui::navigation::ScreenDirection, "net.minecraft.client.gui.navigation.ScreenDirection");
}

#[test]
fn input_type_predicates() {
    use minecraft_rust::net::minecraft::client::InputType::InputType::*;
    assert_eq!([None.is_mouse(), Mouse.is_mouse(), KeyboardArrow.is_mouse(), KeyboardTab.is_mouse()], [false, true, false, false]);
    assert_eq!([None.is_keyboard(), Mouse.is_keyboard(), KeyboardArrow.is_keyboard(), KeyboardTab.is_keyboard()], [false, false, true, true]);
}

#[test]
fn report_type_backend_names_match_the_jar() {
    use minecraft_rust::net::minecraft::client::multiplayer::chat::report::ReportType::VALUES;
    let fq = "net.minecraft.client.multiplayer.chat.report.ReportType";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_enum_field(fq, v.name(), "backendName", &strv(v.backend_name()));
    }
}

#[test]
fn camera_type_fields_and_cycle() {
    use minecraft_rust::net::minecraft::client::CameraType::{CameraType, VALUES};
    let fq = "net.minecraft.client.CameraType";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_enum_field(fq, v.name(), "firstPerson", &boolv(v.is_first_person()));
        assert_enum_field(fq, v.name(), "mirrored", &boolv(v.is_mirrored()));
    }
    assert_eq!(CameraType::FirstPerson.cycle(), CameraType::ThirdPersonBack);
    assert_eq!(CameraType::ThirdPersonBack.cycle(), CameraType::ThirdPersonFront);
    assert_eq!(CameraType::ThirdPersonFront.cycle(), CameraType::FirstPerson);
}

#[test]
fn advancement_widget_type_sprites_match_the_jar() {
    use minecraft_rust::net::minecraft::advancements::AdvancementType::AdvancementType;
    use minecraft_rust::net::minecraft::client::gui::screens::advancements::AdvancementWidgetType::VALUES;
    let fq = "net.minecraft.client.gui.screens.advancements.AdvancementWidgetType";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_enum_field(fq, v.name(), "boxSprite", &strv(&v.box_sprite().to_string()));
        assert_enum_field(fq, v.name(), "taskFrameSprite", &strv(&v.frame_sprite(AdvancementType::Task).to_string()));
        assert_enum_field(fq, v.name(), "challengeFrameSprite", &strv(&v.frame_sprite(AdvancementType::Challenge).to_string()));
        assert_enum_field(fq, v.name(), "goalFrameSprite", &strv(&v.frame_sprite(AdvancementType::Goal).to_string()));
    }
}

#[test]
fn debug_and_mipmap_serialized_names_match_the_jar() {
    use minecraft_rust::net::minecraft::client::gui::components::debug::{DebugScreenEntryStatus as s, DebugScreenProfile as p};
    use minecraft_rust::net::minecraft::client::renderer::texture::MipmapStrategy as mm;
    let fq = "net.minecraft.client.gui.components.debug.DebugScreenEntryStatus";
    assert_enum_order(fq, &s::VALUES.map(|v| v.name()));
    for v in s::VALUES {
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
    }
    let fq = "net.minecraft.client.renderer.texture.MipmapStrategy";
    assert_enum_order(fq, &mm::VALUES.map(|v| v.name()));
    for v in mm::VALUES {
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
    }
    let fq = "net.minecraft.client.gui.components.debug.DebugScreenProfile";
    assert_enum_order(fq, &p::VALUES.map(|v| v.name()));
    for v in p::VALUES {
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_enum_field(fq, v.name(), "translationKey", &strv(v.translation_key()));
    }
    assert_eq!(p::CODEC().by_name("performance"), Some(p::DebugScreenProfile::Performance));
    assert_eq!(p::CODEC().by_name("PERFORMANCE"), None);
}

#[test]
fn screen_axis_and_direction_relations() {
    use minecraft_rust::net::minecraft::client::gui::navigation::ScreenAxis::{ScreenAxis, VALUES as AXES};
    use minecraft_rust::net::minecraft::client::gui::navigation::ScreenDirection::{ScreenDirection, VALUES as DIRS};
    for a in AXES {
        assert_eq!(a.orthogonal().orthogonal(), a);
        assert_eq!(a.get_positive().get_axis(), a);
        assert_eq!(a.get_negative().get_axis(), a);
        assert!(a.get_direction(true).is_positive());
        assert!(!a.get_direction(false).is_positive());
    }
    for d in DIRS {
        assert_eq!(d.get_opposite().get_opposite(), d);
        assert_ne!(d.is_positive(), d.get_opposite().is_positive());
        let cmp = d.coordinate_value_comparator();
        assert_eq!(cmp(5, 5), 0);
        assert_eq!(cmp(1, 2), if d.is_positive() { -1 } else { 1 });
        assert_eq!(d.is_after(3, 1), d.is_positive());
        assert_eq!(d.is_before(3, 1), !d.is_positive());
    }
    assert_eq!(ScreenDirection::Up.get_axis(), ScreenAxis::Vertical);
    assert_eq!(ScreenAxis::Vertical.get_positive(), ScreenDirection::Down);
}

#[test]
fn blaze3d_enums_keep_declaration_order() {
    use minecraft_rust::com::mojang::blaze3d::platform::{BlendFactor, BlendOp, CompareOp, PolygonMode};
    use minecraft_rust::com::mojang::blaze3d::shaders::UniformType;
    use minecraft_rust::com::mojang::blaze3d::systems::DeviceType;
    use minecraft_rust::com::mojang::blaze3d::textures::{AddressMode, FilterMode};
    assert_eq!(PolygonMode::VALUES.map(|v| v.name()), ["FILL", "WIREFRAME"]);
    assert_eq!(UniformType::VALUES.map(|v| v.name()), ["UNIFORM_BUFFER", "TEXEL_BUFFER"]);
    assert_eq!(AddressMode::VALUES.map(|v| v.name()), ["REPEAT", "CLAMP_TO_EDGE"]);
    assert_eq!(FilterMode::VALUES.map(|v| v.name()), ["NEAREST", "LINEAR"]);
    assert_eq!(BlendOp::VALUES.map(|v| v.name()), ["ADD", "SUBTRACT", "REVERSE_SUBTRACT", "MIN", "MAX"]);
    assert_eq!(DeviceType::VALUES.map(|v| v.name()), ["OTHER", "INTEGRATED", "DISCRETE", "VIRTUAL", "CPU"]);
    assert_eq!(CompareOp::VALUES.len(), 8);
    assert_eq!(CompareOp::VALUES[7].name(), "NEVER_PASS");
    assert_eq!(BlendFactor::VALUES.len(), 15);
    assert_eq!(BlendFactor::VALUES[14].name(), "ZERO");
    assert_eq!(BlendFactor::VALUES[12].name(), "SRC_ALPHA_SATURATE");
    for (i, v) in BlendFactor::VALUES.iter().enumerate() {
        assert_eq!(v.ordinal(), i as i32);
    }
}

#[test]
fn index_type_least_uses_the_high_bits() {
    use minecraft_rust::com::mojang::blaze3d::IndexType::IndexType;
    assert_eq!(IndexType::least(0), IndexType::Short);
    assert_eq!(IndexType::least(65535), IndexType::Short);
    assert_eq!(IndexType::least(65536), IndexType::Int);
    assert_eq!(IndexType::least(-1), IndexType::Int, "a negative length has high bits set");
    assert_eq!(IndexType::Short.bytes(), 2);
    assert_eq!(IndexType::Int.bytes(), 4);
}

#[test]
fn primitive_topology_index_count() {
    use minecraft_rust::com::mojang::blaze3d::PrimitiveTopology::{PrimitiveTopology, VALUES};
    assert_eq!(PrimitiveTopology::Quads.index_count(4), 6);
    assert_eq!(PrimitiveTopology::Quads.index_count(5), 6, "int division truncates first");
    assert_eq!(PrimitiveTopology::Lines.index_count(8), 12);
    assert_eq!(PrimitiveTopology::Quads.index_count(-5), -6, "truncates toward zero");
    assert_eq!(PrimitiveTopology::Quads.index_count(i32::MAX), (i32::MAX / 4).wrapping_mul(6), "wraps like Java int");
    assert_eq!(PrimitiveTopology::TriangleFan.index_count(7), 7);
    let strides: Vec<i32> = VALUES.iter().map(|v| v.primitive_stride()).collect();
    assert_eq!(strides, [2, 2, 1, 1, 3, 1, 1, 4]);
    let connected: Vec<bool> = VALUES.iter().map(|v| v.connected_primitives()).collect();
    assert_eq!(connected, [false, false, true, false, false, true, true, false]);
}

fn tr(key: &str) -> String {
    strv(&format!("translation{{key='{key}', args=[]}}"))
}

#[test]
fn option_enums_with_ids_and_captions_match_the_jar() {
    use minecraft_rust::net::minecraft::client::{AttackIndicatorStatus as a, NarratorStatus as n, TextureFilteringMethod as t};
    let fq = "net.minecraft.client.AttackIndicatorStatus";
    assert_enum_order(fq, &a::VALUES.map(|v| v.name()));
    for v in a::VALUES {
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "caption", &tr(v.caption_key()));
    }
    let fq = "net.minecraft.client.TextureFilteringMethod";
    assert_enum_order(fq, &t::VALUES.map(|v| v.name()));
    for v in t::VALUES {
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "caption", &tr(v.caption_key()));
    }
    let fq = "net.minecraft.client.NarratorStatus";
    assert_enum_order(fq, &n::VALUES.map(|v| v.name()));
    for v in n::VALUES {
        assert_enum_field(fq, v.name(), "id", &i32v(v.get_id()));
        assert_enum_field(fq, v.name(), "name", &tr(v.name_key()));
    }
}

#[test]
fn prioritize_chunk_updates_matches_the_jar() {
    use minecraft_rust::net::minecraft::client::PrioritizeChunkUpdates::{by_id, PrioritizeChunkUpdates, VALUES};
    let fq = "net.minecraft.client.PrioritizeChunkUpdates";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "caption", &tr(v.caption_key()));
    }
    assert_eq!(by_id(3), PrioritizeChunkUpdates::None);
    assert_eq!(by_id(-1), PrioritizeChunkUpdates::Nearby);
}

#[test]
fn legacy_int_options_wrap() {
    use minecraft_rust::net::minecraft::client::AttackIndicatorStatus::{by_id, AttackIndicatorStatus};
    use minecraft_rust::net::minecraft::client::NarratorStatus::{by_id as narrator, NarratorStatus};
    assert_eq!(by_id(-1), AttackIndicatorStatus::Hotbar);
    assert_eq!(by_id(3), AttackIndicatorStatus::Off);
    assert_eq!(narrator(4), NarratorStatus::Off);
    assert_eq!(narrator(-1), NarratorStatus::System);
    assert_eq!(
        [NarratorStatus::Off, NarratorStatus::All, NarratorStatus::Chat, NarratorStatus::System].map(|s| (s.should_narrate_chat(), s.should_narrate_system(), s.should_narrate_system_or_chat())),
        [(false, false, false), (true, true, true), (true, false, true), (false, true, true)]
    );
}

#[test]
fn string_option_enums_match_the_jar() {
    use minecraft_rust::net::minecraft::client::{CloudStatus as c, InactivityFpsLimit as i, MusicToastDisplayState as m, PresenceSharing as p};
    let fq = "net.minecraft.client.CloudStatus";
    assert_enum_order(fq, &c::VALUES.map(|v| v.name()));
    for v in c::VALUES {
        assert_enum_field(fq, v.name(), "legacyName", &strv(v.get_serialized_name()));
        assert_enum_field(fq, v.name(), "caption", &tr(v.caption_key()));
        assert_eq!(c::by_serialized_name(v.get_serialized_name()), Some(v));
    }
    let fq = "net.minecraft.client.InactivityFpsLimit";
    assert_enum_order(fq, &i::VALUES.map(|v| v.name()));
    for v in i::VALUES {
        assert_enum_field(fq, v.name(), "serializedName", &strv(v.get_serialized_name()));
        assert_enum_field(fq, v.name(), "caption", &tr(v.caption_key()));
    }
    let fq = "net.minecraft.client.PresenceSharing";
    assert_enum_order(fq, &p::VALUES.map(|v| v.name()));
    // The dump records only the constants of an enum, not its other static finals, so
    // TRANSLATION_KEY_BASE is checked against the keys the constants were built from instead.
    for v in p::VALUES {
        assert!(v.translation_key().starts_with(&format!("{}.", p::TRANSLATION_KEY_BASE)));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_enum_field(fq, v.name(), "translatable", &tr(v.translation_key()));
        assert_enum_field(fq, v.name(), "tooltip", &tr(v.tooltip_key()));
    }
    let fq = "net.minecraft.client.MusicToastDisplayState";
    assert_enum_order(fq, &m::VALUES.map(|v| v.name()));
    for v in m::VALUES {
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_enum_field(fq, v.name(), "text", &tr(v.text_key()));
        assert_enum_field(fq, v.name(), "tooltip", &tr(v.tooltip_key()));
    }
    assert_eq!(m::VALUES.map(|v| (v.render_in_pause_screen(), v.render_toast())), [(false, false), (true, false), (true, true)]);
    assert_eq!(c::by_serialized_name("fancy"), None, "FANCY serializes as \"true\"");
}

#[test]
fn font_option_and_filter() {
    use minecraft_rust::net::minecraft::client::gui::font::FontOption::{FontOption, Filter, VALUES};
    use std::collections::{HashMap, HashSet};
    let fq = "net.minecraft.client.gui.font.FontOption";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
    }
    let jp_on: HashSet<_> = [FontOption::JapaneseVariants].into_iter().collect();
    let none: HashSet<FontOption> = HashSet::new();
    let want_jp_off = Filter::new(HashMap::from([(FontOption::JapaneseVariants, false)]));
    assert!(!want_jp_off.apply(&jp_on), "false means MUST be absent");
    assert!(want_jp_off.apply(&none));
    assert!(Filter::always_pass().apply(&jp_on));
    let want_jp_on = Filter::new(HashMap::from([(FontOption::JapaneseVariants, true), (FontOption::Uniform, false)]));
    let merged = want_jp_off.merge(&want_jp_on);
    assert_eq!(merged.values().get(&FontOption::JapaneseVariants), Some(&false), "this filter wins");
    assert_eq!(merged.values().get(&FontOption::Uniform), Some(&false), "other's extra keys are kept");
}

#[test]
fn ban_and_report_reasons_match_the_jar() {
    use minecraft_rust::net::minecraft::client::multiplayer::chat::report::BanReason::{BanReason, VALUES as BANS};
    use minecraft_rust::net::minecraft::client::multiplayer::chat::report::ReportReason::{ReportReason, VALUES as REASONS};
    use minecraft_rust::net::minecraft::client::multiplayer::chat::report::ReportType::ReportType;
    let fq = "net.minecraft.client.multiplayer.chat.report.BanReason";
    assert_enum_order(fq, &BANS.map(|v| v.name()));
    for v in BANS {
        assert_enum_field(fq, v.name(), "title", &tr(v.title_key()));
    }
    let mapped: Vec<(i32, &str)> = (-2..60).filter_map(|i| BanReason::by_id(i).map(|b| (i, b.name()))).collect();
    assert_eq!(
        mapped,
        [(2, "FALSE_REPORTING"), (5, "HATE_SPEECH"), (16, "HATE_TERRORISM_NOTORIOUS_FIGURE"), (17, "GENERIC_VIOLATION"),
         (19, "GENERIC_VIOLATION"), (21, "HARASSMENT_OR_BULLYING"), (23, "GENERIC_VIOLATION"), (25, "HATE_TERRORISM_NOTORIOUS_FIGURE"),
         (27, "DEFAMATION_IMPERSONATION_FALSE_INFORMATION"), (28, "DRUGS"), (29, "FRAUD"), (30, "SPAM_OR_ADVERTISING"),
         (31, "GENERIC_VIOLATION"), (32, "NUDITY_OR_PORNOGRAPHY"), (33, "SEXUALLY_INAPPROPRIATE"), (34, "EXTREME_VIOLENCE_OR_GORE"),
         (35, "SEXUALLY_INAPPROPRIATE"), (36, "SEXUALLY_INAPPROPRIATE"), (53, "IMMINENT_HARM_TO_PERSON_OR_PROPERTY")]
    );
    let fq = "net.minecraft.client.multiplayer.chat.report.ReportReason";
    assert_enum_order(fq, &REASONS.map(|v| v.name()));
    for v in REASONS {
        assert_enum_field(fq, v.name(), "backendName", &strv(v.backend_name()));
        assert_enum_field(fq, v.name(), "title", &tr(v.title_key()));
        assert_enum_field(fq, v.name(), "description", &tr(v.description_key()));
    }
    assert_eq!(ReportReason::get_incompatible_categories(ReportType::Chat), [ReportReason::SexuallyInappropriate]);
    assert_eq!(ReportReason::get_incompatible_categories(ReportType::Skin), [ReportReason::ImminentHarm, ReportReason::DefamationImpersonationFalseInformation]);
    assert!(ReportReason::get_incompatible_categories(ReportType::Username).is_empty());
}

#[test]
fn realms_service_quality() {
    use minecraft_rust::com::mojang::realmsclient::dto::ServiceQuality::{ServiceQuality, VALUES};
    assert_eq!(VALUES.map(|v| v.name()), ["GREAT", "GOOD", "OKAY", "POOR", "UNKNOWN"]);
    assert_eq!(ServiceQuality::by_value(0), None);
    assert_eq!(ServiceQuality::by_value(5), Some(ServiceQuality::Unknown));
    assert_eq!(ServiceQuality::by_value(1), Some(ServiceQuality::Great));
    assert_eq!(ServiceQuality::Poor.get_icon().to_string(), "minecraft:icon/ping_2");
}

// ===========================================================================
// BoundingBox (levelgen/structure)
// ===========================================================================

#[test]
fn bounding_box_construction_and_repair() {
    use minecraft_rust::net::minecraft::world::level::levelgen::structure::BoundingBox::BoundingBox;
    let b = BoundingBox::new(1, 2, 3, 4, 5, 6);
    assert_eq!(b.to_string(), "BoundingBox{minX=1, minY=2, minZ=3, maxX=4, maxY=5, maxZ=6}");
    // Objects.hash(1,2,3,4,5,6) computed by hand: 1 -> 32 -> 994 -> 30817 -> 955331 -> 29615266 -> 918073252
    assert_eq!(b.java_hash_code(), 918073252);
    // Inverted input is swapped per axis; the message is built from the RAW bounds.
    let inv = BoundingBox::new(4, 2, 9, 1, 5, 3);
    assert_eq!(inv, BoundingBox::new(1, 2, 3, 4, 5, 9));
    assert_eq!(
        BoundingBox::inverted_bounds_message(4, 2, 9, 1, 5, 3),
        "Invalid bounding box data, inverted bounds for: BoundingBox{minX=4, minY=2, minZ=9, maxX=1, maxY=5, maxZ=3}"
    );
    let inf = BoundingBox::infinite();
    assert_eq!(inf.get_x_span(), 0, "MAX - MIN + 1 wraps to 0 in Java int arithmetic");
    assert_eq!(inf.get_length().get_x(), -1);
    assert_eq!(inf.get_center().get_x(), i32::MIN, "span 0 / 2 == 0");
}

#[test]
fn bounding_box_geometry() {
    use minecraft_rust::net::minecraft::core::BlockPos::BlockPos;
    use minecraft_rust::net::minecraft::core::Direction::Direction;
    use minecraft_rust::net::minecraft::core::Vec3i::Vec3i;
    use minecraft_rust::net::minecraft::world::level::levelgen::structure::BoundingBox::BoundingBox;
    let b = BoundingBox::new(0, 0, 0, 9, 4, 2);
    assert_eq!((b.get_x_span(), b.get_y_span(), b.get_z_span()), (10, 5, 3));
    assert_eq!(b.get_center(), BlockPos::new(5, 2, 1));
    assert!(b.is_inside(9, 4, 2) && b.is_inside(0, 0, 0) && !b.is_inside(10, 0, 0));
    assert!(b.is_inside_vec(&Vec3i::new(3, 3, 1)));
    assert!(b.intersects(&BoundingBox::new(9, 4, 2, 20, 20, 20)), "touching corners intersect (inclusive)");
    assert!(!b.intersects(&BoundingBox::new(10, 0, 0, 20, 4, 2)));
    assert!(b.intersects_xz(9, 2, 30, 30));
    assert_eq!(BoundingBox::from_corners(&Vec3i::new(5, -1, 7), &Vec3i::new(-2, 3, 0)), BoundingBox::new(-2, -1, 0, 5, 3, 7));
    assert_eq!(b.moved(1, -1, 2), BoundingBox::new(1, -1, 2, 10, 3, 4));
    let mut m = b;
    m.move_by(1, 1, 1).move_by_vec(&Vec3i::new(-1, -1, -1));
    assert_eq!(m, b, "mutating move round-trips");
    assert_eq!(b.inflated_by(2), BoundingBox::new(-2, -2, -2, 11, 6, 4));
    assert_eq!(b.inflated_by_xyz(-6, 0, 0), BoundingBox::new(3, 0, 0, 6, 4, 2), "over-deflation inverts, constructor repairs");
    // orientBox: SOUTH is the default branch.
    assert_eq!(BoundingBox::orient_box(10, 64, 10, -1, 0, 2, 5, 3, 4, Direction::South), BoundingBox::new(9, 64, 12, 13, 66, 15));
    assert_eq!(BoundingBox::orient_box(10, 64, 10, -1, 0, 2, 5, 3, 4, Direction::North), BoundingBox::new(9, 64, 9, 13, 66, 12));
    assert_eq!(BoundingBox::orient_box(10, 64, 10, -1, 0, 2, 5, 3, 4, Direction::West), BoundingBox::new(9, 64, 9, 12, 66, 13));
    assert_eq!(BoundingBox::orient_box(10, 64, 10, -1, 0, 2, 5, 3, 4, Direction::East), BoundingBox::new(12, 64, 9, 15, 66, 13));
    assert_eq!(
        BoundingBox::orient_box(10, 64, 10, -1, 0, 2, 5, 3, 4, Direction::Up),
        BoundingBox::orient_box(10, 64, 10, -1, 0, 2, 5, 3, 4, Direction::South),
        "non-horizontal directions fall into the default (SOUTH) branch"
    );
}

#[test]
fn bounding_box_encapsulation_and_corners() {
    use minecraft_rust::net::minecraft::core::BlockPos::BlockPos;
    use minecraft_rust::net::minecraft::world::level::levelgen::structure::BoundingBox::BoundingBox;
    assert_eq!(BoundingBox::encapsulating_positions(std::iter::empty()), None);
    let ps = [BlockPos::new(1, 5, 1), BlockPos::new(-3, 2, 8), BlockPos::new(0, 9, 0)];
    assert_eq!(BoundingBox::encapsulating_positions(ps.iter()), Some(BoundingBox::new(-3, 2, 0, 1, 9, 8)));
    let bs = [BoundingBox::new(0, 0, 0, 1, 1, 1), BoundingBox::new(5, -5, 2, 6, 0, 3)];
    assert_eq!(BoundingBox::encapsulating_boxes(bs.iter()), Some(BoundingBox::new(0, -5, 0, 6, 1, 3)));
    assert_eq!(BoundingBox::encapsulating(&bs[0], &bs[1]), BoundingBox::new(0, -5, 0, 6, 1, 3));
    let mut corners = Vec::new();
    BoundingBox::new(0, 10, 20, 1, 11, 21).for_all_corners(|p| corners.push((p.get_x(), p.get_y(), p.get_z())));
    assert_eq!(
        corners,
        [(1, 11, 21), (0, 11, 21), (1, 10, 21), (0, 10, 21), (1, 11, 20), (0, 11, 20), (1, 10, 20), (0, 10, 20)]
    );
    // Chunks: blocks -1..16 in X span chunks -1, 0, 1; Z 0..15 is chunk 0 only.
    let chunks = BoundingBox::new(-1, 0, 0, 16, 0, 15).intersecting_chunks();
    let xs: Vec<i32> = chunks.iter().map(|c| c.x).collect();
    assert_eq!(xs, [-1, 0, 1]);
}

// ===========================================================================
// DynamicGraphMinFixedPoint / LeveledPriorityQueue / ChunkTracker / SectionTracker /
// ChunkTrackingView / RegionBitmap -- replayed against `graph_probe.txt`, produced by
// `_porting/java-oracle/src/p/GraphProbe.java` on the real jar. The scripts below MUST stay
// identical to the ones in GraphProbe.java.
// ===========================================================================

mod graph_twin {
    pub use minecraft_rust::net::minecraft::core::SectionPos::SectionPos;
    pub use minecraft_rust::net::minecraft::server::level::ChunkTracker::ChunkTracker;
    pub use minecraft_rust::net::minecraft::server::level::SectionTracker::SectionTracker;
    pub use minecraft_rust::net::minecraft::world::level::lighting::DynamicGraphMinFixedPoint::{DynamicGraphMinFixedPoint, GraphState};
    pub use minecraft_rust::net::minecraft::world::level::ChunkPos::ChunkPos;
    pub use std::collections::{BTreeMap, HashMap};

    pub fn probe_rows(prefix: &str) -> Vec<Vec<String>> {
        let path = minecraft_rust::javacompat::golden::test_data_dir().join("graph_probe.txt");
        let text = std::fs::read_to_string(&path).unwrap();
        let rows: Vec<Vec<String>> = text
            .lines()
            .filter(|l| l.starts_with(prefix))
            .map(|l| l.split('|').map(str::to_string).collect())
            .collect();
        assert!(!rows.is_empty(), "graph_probe.txt has no `{prefix}` rows");
        rows
    }

    pub struct Ct {
        pub g: GraphState,
        pub levels: HashMap<i64, i32>,
        pub sources: HashMap<i64, i32>,
        pub lc: i32,
    }

    impl DynamicGraphMinFixedPoint for Ct {
        fn graph(&mut self) -> &mut GraphState {
            &mut self.g
        }
        fn graph_ref(&self) -> &GraphState {
            &self.g
        }
        fn get_computed_level(&mut self, node: i64, parent: i64, level: i32) -> i32 {
            self.chunk_get_computed_level(node, parent, level)
        }
        fn check_neighbors_after_update(&mut self, node: i64, level: i32, only: bool) {
            self.chunk_check_neighbors_after_update(node, level, only)
        }
        fn get_level(&mut self, node: i64) -> i32 {
            *self.levels.get(&node).unwrap_or(&(self.lc - 1))
        }
        fn set_level(&mut self, node: i64, level: i32) {
            if level >= self.lc - 1 {
                self.levels.remove(&node);
            } else {
                self.levels.insert(node, level);
            }
        }
        fn compute_level_from_neighbor(&mut self, from: i64, to: i64, level: i32) -> i32 {
            self.chunk_compute_level_from_neighbor(from, to, level)
        }
        fn is_source(&self, node: i64) -> bool {
            self.chunk_is_source(node)
        }
    }

    impl ChunkTracker for Ct {
        fn get_level_from_source(&mut self, to: i64) -> i32 {
            *self.sources.get(&to).unwrap_or(&(self.lc - 1))
        }
    }

    impl Ct {
        pub fn new(lc: i32) -> Ct {
            Ct { g: GraphState::new(lc, 16, 256), levels: HashMap::new(), sources: HashMap::new(), lc }
        }
        pub fn snapshot(&self) -> String {
            let mut sorted = BTreeMap::new();
            for (k, v) in &self.levels {
                let p = ChunkPos::unpack(*k);
                sorted.insert(format!("{:+06},{:+06}", p.x, p.z), *v);
            }
            sorted.iter().map(|(k, v)| format!("{k}={v};")).collect()
        }
    }

    pub struct St {
        pub g: GraphState,
        pub levels: HashMap<i64, i32>,
        pub sources: HashMap<i64, i32>,
        pub lc: i32,
    }

    impl DynamicGraphMinFixedPoint for St {
        fn graph(&mut self) -> &mut GraphState {
            &mut self.g
        }
        fn graph_ref(&self) -> &GraphState {
            &self.g
        }
        fn get_computed_level(&mut self, node: i64, parent: i64, level: i32) -> i32 {
            self.section_get_computed_level(node, parent, level)
        }
        fn check_neighbors_after_update(&mut self, node: i64, level: i32, only: bool) {
            self.section_check_neighbors_after_update(node, level, only)
        }
        fn get_level(&mut self, node: i64) -> i32 {
            *self.levels.get(&node).unwrap_or(&(self.lc - 1))
        }
        fn set_level(&mut self, node: i64, level: i32) {
            if level >= self.lc - 1 {
                self.levels.remove(&node);
            } else {
                self.levels.insert(node, level);
            }
        }
        fn compute_level_from_neighbor(&mut self, from: i64, to: i64, level: i32) -> i32 {
            self.section_compute_level_from_neighbor(from, to, level)
        }
    }

    impl SectionTracker for St {
        fn get_level_from_source(&mut self, to: i64) -> i32 {
            *self.sources.get(&to).unwrap_or(&(self.lc - 1))
        }
    }

    impl St {
        pub fn new(lc: i32) -> St {
            St { g: GraphState::new(lc, 16, 256), levels: HashMap::new(), sources: HashMap::new(), lc }
        }
        pub fn snapshot(&self) -> String {
            let mut sorted = BTreeMap::new();
            for (k, v) in &self.levels {
                sorted.insert(format!("{:+06},{:+06},{:+06}", SectionPos::x(*k), SectionPos::y(*k), SectionPos::z(*k)), *v);
            }
            sorted.iter().map(|(k, v)| format!("{k}={v};")).collect()
        }
    }
}

#[test]
fn chunk_tracker_matches_the_jar_step_by_step() {
    use graph_twin::*;
    let mut got: Vec<(String, i32, i32, String)> = Vec::new();
    let mut rec = |step: &str, t: &Ct, ret: i32| got.push((step.to_string(), ret, t.get_queue_size(), t.snapshot()));
    let mut c = Ct::new(12);
    let (a, b, d) = (ChunkPos::pack(0, 0), ChunkPos::pack(4, 1), ChunkPos::pack(-3, -2));
    c.sources.insert(a, 3);
    c.update(a, 3, true);
    c.sources.insert(b, 5);
    c.update(b, 5, true);
    rec("s1.enqueued", &c, -1);
    let r = c.run_updates(7);
    rec("s2.run7", &c, r);
    let r = c.run_updates(13);
    rec("s3.run13", &c, r);
    let r = c.run_updates(100000);
    rec("s4.runAll", &c, r);
    c.sources.remove(&a);
    c.update(a, 11, false);
    rec("s5.raised", &c, -1);
    let r = c.run_updates(5);
    rec("s6.run5", &c, r);
    let r = c.run_updates(17);
    rec("s7.run17", &c, r);
    let r = c.run_updates(100000);
    rec("s8.runAll", &c, r);
    c.sources.insert(d, 0);
    c.update(d, 0, true);
    let r = c.run_updates(3);
    rec("s9.run3", &c, r);
    c.remove_if(|node| ChunkPos::unpack(node).x < -4);
    rec("s10.removeIf", &c, -1);
    let r = c.run_updates(100000);
    rec("s11.runAll", &c, r);
    c.sources.insert(b, 9);
    c.update(b, 9, false);
    let r = c.run_updates(11);
    rec("s12.run11", &c, r);
    let r = c.run_updates(100000);
    rec("s13.runAll", &c, r);
    c.sources.insert(a, 7);
    c.update(a, 7, true);
    c.sources.insert(ChunkPos::pack(2, 0), 2);
    c.update(ChunkPos::pack(2, 0), 2, true);
    let r = c.run_updates(9);
    rec("s14.run9", &c, r);
    let r = c.run_updates(100000);
    rec("s15.runAll", &c, r);

    let want = probe_rows("graph.chunk|");
    assert_eq!(want.len(), got.len());
    for (w, g) in want.iter().zip(got.iter()) {
        assert_eq!(w[1], g.0);
        assert_eq!(w[2].parse::<i32>().unwrap(), g.1, "{}: runUpdates budget", g.0);
        assert_eq!(w[3].parse::<i32>().unwrap(), g.2, "{}: queue size", g.0);
        assert_eq!(w[4], g.3, "{}: levels", g.0);
    }
}

#[test]
fn section_tracker_matches_the_jar_step_by_step() {
    use graph_twin::*;
    let mut got: Vec<(String, i32, i32, String)> = Vec::new();
    let mut rec = |step: &str, t: &St, ret: i32| got.push((step.to_string(), ret, t.get_queue_size(), t.snapshot()));
    let mut s = St::new(6);
    let (p, q, r0) = (SectionPos::as_long_xyz(0, 0, 0), SectionPos::as_long_xyz(3, -1, 2), SectionPos::as_long_xyz(-2, 4, -1));
    s.sources.insert(p, 1);
    s.update(p, 1, true);
    s.sources.insert(q, 2);
    s.update(q, 2, true);
    let r = s.run_updates(10);
    rec("t1.run10", &s, r);
    let r = s.run_updates(31);
    rec("t2.run31", &s, r);
    let r = s.run_updates(100000);
    rec("t3.runAll", &s, r);
    s.sources.remove(&p);
    s.update(p, 5, false);
    let r = s.run_updates(20);
    rec("t4.run20", &s, r);
    let r = s.run_updates(100000);
    rec("t5.runAll", &s, r);
    s.sources.insert(r0, 0);
    s.update(r0, 0, true);
    let r = s.run_updates(7);
    rec("t6.run7", &s, r);
    s.remove_if(|node| SectionPos::y(node) > 4);
    rec("t7.removeIf", &s, -1);
    let r = s.run_updates(100000);
    rec("t8.runAll", &s, r);

    let want = probe_rows("graph.section|");
    assert_eq!(want.len(), got.len());
    for (w, g) in want.iter().zip(got.iter()) {
        assert_eq!(w[1], g.0);
        assert_eq!(w[2].parse::<i32>().unwrap(), g.1, "{}: runUpdates budget", g.0);
        assert_eq!(w[3].parse::<i32>().unwrap(), g.2, "{}: queue size", g.0);
        assert_eq!(w[4], g.3, "{}: levels", g.0);
    }
}

#[test]
fn dynamic_graph_rejects_too_many_levels() {
    use minecraft_rust::net::minecraft::world::level::lighting::DynamicGraphMinFixedPoint::GraphState;
    let e = std::panic::catch_unwind(|| GraphState::new(254, 1, 1)).expect_err("254 levels must throw");
    assert_eq!(e.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| e.downcast_ref::<String>().cloned()).unwrap(),
        "java.lang.IllegalArgumentException: Level count must be < 254.");
    let _ = GraphState::new(253, 1, 1);
}

#[test]
fn leveled_priority_queue_is_insertion_ordered_per_level() {
    use minecraft_rust::net::minecraft::world::level::lighting::LeveledPriorityQueue::LeveledPriorityQueue;
    let mut q = LeveledPriorityQueue::new(4, 2);
    assert!(q.is_empty());
    q.enqueue(30, 2);
    q.enqueue(10, 2);
    q.enqueue(20, 1);
    q.enqueue(30, 2); // already present: does not move
    assert_eq!(q.first_queued_level(), 1);
    assert_eq!(q.remove_first_long(), 20);
    assert_eq!(q.first_queued_level(), 2, "emptied level rescans upward");
    assert_eq!(q.queued_at(2), [30, 10]);
    q.dequeue(30, 2, 4);
    assert_eq!(q.remove_first_long(), 10);
    assert!(q.is_empty());
}

#[test]
fn chunk_tracking_view_matches_the_jar() {
    use minecraft_rust::net::minecraft::server::level::ChunkTrackingView::{difference, of, ChunkTrackingView, EMPTY};
    use minecraft_rust::net::minecraft::world::level::ChunkPos::ChunkPos;
    let parse = |s: &str| -> (i32, i32, i32) {
        let v: Vec<i32> = s.split(',').map(|x| x.parse().unwrap()).collect();
        (v[0], v[1], v[2])
    };
    for r in graph_twin::probe_rows("view.forEach|") {
        let (cx, cz, rad) = parse(&r[1]);
        let mut s = String::new();
        of(ChunkPos::new(cx, cz), rad).for_each(&mut |p: ChunkPos| s.push_str(&format!("{},{};", p.x, p.z)));
        assert_eq!(s, r[2], "forEach order for {}", r[1]);
    }
    let mut n = 0;
    for r in graph_twin::probe_rows("view.contains|") {
        let (cx, cz, rad) = parse(&r[1]);
        let xz: Vec<i32> = r[2].split(',').map(|x| x.parse().unwrap()).collect();
        let v = of(ChunkPos::new(cx, cz), rad);
        assert_eq!(v.contains_with(xz[0], xz[1], true).to_string(), r[3], "{r:?}");
        assert_eq!(v.contains_with(xz[0], xz[1], false).to_string(), r[4], "{r:?}");
        assert_eq!(v.is_in_view_distance(xz[0], xz[1]).to_string(), r[5], "{r:?}");
        n += 1;
    }
    assert!(n > 500);
    let mut all = vec![EMPTY];
    for (cx, cz, rad) in [(0, 0, 2), (1, 0, 2), (0, 3, 3), (10, 10, 1), (0, 0, 0), (-5, 7, 4)] {
        all.push(of(ChunkPos::new(cx, cz), rad));
    }
    for r in graph_twin::probe_rows("view.difference|") {
        let (i, j): (usize, usize) = (r[1].parse().unwrap(), r[2].parse().unwrap());
        let (mut enter, mut leave) = (String::new(), String::new());
        difference(
            &all[i],
            &all[j],
            |p| enter.push_str(&format!("{},{};", p.x, p.z)),
            |p| leave.push_str(&format!("{},{};", p.x, p.z)),
        );
        assert_eq!((enter.as_str(), leave.as_str()), (r[3].as_str(), r[4].as_str()), "difference({i}, {j})");
    }
    assert!(!ChunkTrackingView::Empty.contains(0, 0));
}

#[test]
fn region_bitmap_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::level::chunk::storage::RegionBitmap::RegionBitmap;
    let fmt = |v: Vec<i32>| format!("{{{}}}", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
    let rows = graph_twin::probe_rows("bitmap|");
    let mut bm = RegionBitmap::new();
    let mut i = 0;
    let mut check = |bm: &RegionBitmap, result: String, rows: &[Vec<String>], i: &mut usize| {
        assert_eq!((result.as_str(), fmt(bm.get_used()).as_str()), (rows[*i][2].as_str(), rows[*i][3].as_str()), "step {}", rows[*i][1]);
        *i += 1;
    };
    bm.force(0, 2);
    check(&bm, "-".into(), &rows, &mut i);
    let r = bm.allocate(1);
    check(&bm, r.to_string(), &rows, &mut i);
    let r = bm.allocate(3);
    check(&bm, r.to_string(), &rows, &mut i);
    bm.free(2, 1);
    check(&bm, "-".into(), &rows, &mut i);
    let r = bm.allocate(2);
    check(&bm, r.to_string(), &rows, &mut i);
    let r = bm.allocate(1);
    check(&bm, r.to_string(), &rows, &mut i);
    bm.free(3, 3);
    check(&bm, "-".into(), &rows, &mut i);
    let r = bm.allocate(2);
    check(&bm, r.to_string(), &rows, &mut i);
    let r = bm.allocate(0);
    check(&bm, r.to_string(), &rows, &mut i);
    let mut bm2 = bm.clone();
    let e = std::panic::catch_unwind(move || bm2.force(5, -1)).expect_err("negative size throws");
    let msg = e.downcast_ref::<String>().unwrap().clone();
    check(&bm, msg, &rows, &mut i);
    assert_eq!(i, rows.len());
}
