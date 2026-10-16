//! Session 19 batch c: levelgen, entity and world enums. Checks ports against the jar's constant
//! dump and the Java source's behaviour. Written by the coordinator after the batch agent stopped
//! before reaching its test file; every expected value below is read from the PORT and compared
//! with the jar's dump, never hand-copied.

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

/// Names, ordinals, `Display == name()`, plus one string field per constant compared with the
/// port's getter.
macro_rules! check_named_enum {
    ($module:path, $fq:expr, $field:expr, $getter:ident) => {{
        use $module as m;
        let fq = $fq;
        assert_enum_order(fq, &m::VALUES.map(|v| v.name()));
        for (i, v) in m::VALUES.iter().enumerate() {
            assert_eq!(v.ordinal(), i as i32);
            assert_eq!(v.to_string(), v.name());
            assert_enum_field(fq, v.name(), $field, &strv(v.$getter()));
        }
    }};
}

#[test]
fn string_enums_match_the_jar() {
    check_named_enum!(minecraft_rust::net::minecraft::world::level::gamerules::GameRuleType, "net.minecraft.world.level.gamerules.GameRuleType", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::level::levelgen::structure::templatesystem::LiquidSettings, "net.minecraft.world.level.levelgen.structure.templatesystem.LiquidSettings", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::item::enchantment::EnchantmentTarget, "net.minecraft.world.item.enchantment.EnchantmentTarget", "id", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::level::levelgen::structure::TerrainAdjustment, "net.minecraft.world.level.levelgen.structure.TerrainAdjustment", "id", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::level::levelgen::structure::placement::RandomSpreadType, "net.minecraft.world.level.levelgen.structure.placement.RandomSpreadType", "id", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::level::levelgen::placement::CaveSurface, "net.minecraft.world.level.levelgen.placement.CaveSurface", "id", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::entity::ai::gossip::GossipType, "net.minecraft.world.entity.ai.gossip.GossipType", "id", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::level::MoonPhase, "net.minecraft.world.level.MoonPhase", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::util::TriState, "net.minecraft.util.TriState", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::entity::MobCategory, "net.minecraft.world.entity.MobCategory", "name", get_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::entity::animal::equine::Variant, "net.minecraft.world.entity.animal.equine.Variant", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::entity::animal::golem::CopperGolemState, "net.minecraft.world.entity.animal.golem.CopperGolemState", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::damagesource::DamageEffects, "net.minecraft.world.damagesource.DamageEffects", "id", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::level::GameType, "net.minecraft.world.level.GameType", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::server::permissions::PermissionLevel, "net.minecraft.server.permissions.PermissionLevel", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::scores::DisplaySlot, "net.minecraft.world.scores.DisplaySlot", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::world::scores::TeamColor, "net.minecraft.world.scores.TeamColor", "name", get_serialized_name);
    check_named_enum!(minecraft_rust::net::minecraft::core::FrontAndTop, "net.minecraft.core.FrontAndTop", "name", get_serialized_name);
}

#[test]
fn int_id_fields_match_the_jar() {
    use minecraft_rust::net::minecraft::server::permissions::PermissionLevel as pl;
    use minecraft_rust::net::minecraft::world::entity::animal::equine::{Markings as mk, Variant as va};
    use minecraft_rust::net::minecraft::world::entity::animal::golem::CopperGolemState as cg;
    use minecraft_rust::net::minecraft::world::level::GameType as gt;
    use minecraft_rust::net::minecraft::world::scores::{DisplaySlot as ds, TeamColor as tc};
    let fq = "net.minecraft.world.entity.animal.equine.Markings";
    assert_enum_order(fq, &mk::VALUES.map(|v| v.name()));
    for v in mk::VALUES {
        assert_enum_field(fq, v.name(), "id", &i32v(v.get_id()));
    }
    for v in va::VALUES {
        assert_enum_field("net.minecraft.world.entity.animal.equine.Variant", v.name(), "id", &i32v(v.get_id()));
    }
    for v in cg::VALUES {
        assert_enum_field("net.minecraft.world.entity.animal.golem.CopperGolemState", v.name(), "id", &i32v(v.id()));
    }
    for v in gt::VALUES {
        assert_enum_field("net.minecraft.world.level.GameType", v.name(), "id", &i32v(v.get_id()));
    }
    for v in pl::VALUES {
        assert_enum_field("net.minecraft.server.permissions.PermissionLevel", v.name(), "id", &i32v(v.id()));
    }
    for v in ds::VALUES {
        assert_enum_field("net.minecraft.world.scores.DisplaySlot", v.name(), "id", &i32v(v.id()));
    }
    for v in tc::VALUES {
        let fq = "net.minecraft.world.scores.TeamColor";
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "displaySlot", &strv(v.display_slot().name()));
    }
}

#[test]
fn payload_fields_match_the_jar() {
    use minecraft_rust::net::minecraft::core::FrontAndTop as ft;
    use minecraft_rust::net::minecraft::world::entity::ai::gossip::GossipType as gs;
    use minecraft_rust::net::minecraft::world::entity::monster::warden::AngerLevel as al;
    use minecraft_rust::net::minecraft::world::entity::MobCategory as mc;
    use minecraft_rust::net::minecraft::world::level::levelgen::placement::CaveSurface as cs;
    use minecraft_rust::net::minecraft::world::level::MoonPhase as mp;
    for v in cs::VALUES {
        let fq = "net.minecraft.world.level.levelgen.placement.CaveSurface";
        assert_enum_field(fq, v.name(), "direction", &strv(&v.get_direction().name().to_uppercase()));
        assert_enum_field(fq, v.name(), "y", &i32v(v.get_y()));
    }
    for v in gs::VALUES {
        let fq = "net.minecraft.world.entity.ai.gossip.GossipType";
        assert_enum_field(fq, v.name(), "weight", &i32v(v.weight()));
        assert_enum_field(fq, v.name(), "max", &i32v(v.max()));
        assert_enum_field(fq, v.name(), "decayPerDay", &i32v(v.decay_per_day()));
        assert_enum_field(fq, v.name(), "decayPerTransfer", &i32v(v.decay_per_transfer()));
    }
    for v in mp::VALUES {
        assert_enum_field("net.minecraft.world.level.MoonPhase", v.name(), "index", &i32v(v.index()));
    }
    for v in mc::VALUES {
        let fq = "net.minecraft.world.entity.MobCategory";
        assert_enum_field(fq, v.name(), "max", &i32v(v.get_max_instances_per_chunk()));
        assert_enum_field(fq, v.name(), "isFriendly", &boolv(v.is_friendly()));
        assert_enum_field(fq, v.name(), "isPersistent", &boolv(v.is_persistent()));
        assert_enum_field(fq, v.name(), "debugAbbreviation", &strv(v.get_debug_abbreviation()));
        assert_enum_field(fq, v.name(), "noDespawnDistance", &i32v(v.get_no_despawn_distance()));
        assert_enum_field(fq, v.name(), "despawnDistance", &i32v(v.get_despawn_distance()));
    }
    let fq = "net.minecraft.world.entity.monster.warden.AngerLevel";
    assert_enum_order(fq, &al::VALUES.map(|v| v.name()));
    for v in al::VALUES {
        assert_enum_field(fq, v.name(), "minimumAnger", &i32v(v.get_minimum_anger()));
    }
    for v in ft::VALUES {
        let fq = "net.minecraft.core.FrontAndTop";
        assert_enum_field(fq, v.name(), "front", &strv(&v.front().name().to_uppercase()));
        assert_enum_field(fq, v.name(), "top", &strv(&v.top().name().to_uppercase()));
    }
}

#[test]
fn gossip_type_static_finals() {
    use minecraft_rust::net::minecraft::world::entity::ai::gossip::GossipType as gs;
    assert_eq!(gs::REPUTATION_CHANGE_PER_EVENT, 25);
    assert_eq!(gs::REPUTATION_CHANGE_PER_EVERLASTING_MEMORY, 20);
    assert_eq!(gs::REPUTATION_CHANGE_PER_TRADE, 2);
}

#[test]
fn moon_phase_ticks_and_count() {
    use minecraft_rust::net::minecraft::world::level::MoonPhase::{MoonPhase, COUNT, PHASE_LENGTH, VALUES};
    assert_eq!(COUNT, 8);
    assert_eq!(PHASE_LENGTH, 24000);
    for v in VALUES {
        assert_eq!(v.start_tick(), v.index() * 24000);
    }
    assert_eq!(MoonPhase::WaxingGibbous.start_tick(), 168000);
}

#[test]
fn tri_state_conversions() {
    use minecraft_rust::net::minecraft::util::TriState::{from, TriState};
    assert_eq!(from(true), TriState::True);
    assert_eq!(from(false), TriState::False);
    assert!(TriState::True.to_boolean(false));
    assert!(!TriState::False.to_boolean(true));
    assert!(TriState::Default.to_boolean(true));
    assert!(!TriState::Default.to_boolean(false));
}

#[test]
fn random_spread_type_consumes_ints_like_java() {
    use minecraft_rust::net::minecraft::util::RandomSource::RandomSource;
    use minecraft_rust::net::minecraft::world::level::levelgen::LegacyRandomSource::LegacyRandomSource;
    use minecraft_rust::net::minecraft::world::level::levelgen::structure::placement::RandomSpreadType::RandomSpreadType;
    // LINEAR is exactly one nextInt(limit).
    let mut a = LegacyRandomSource::new(12345);
    let mut b = LegacyRandomSource::new(12345);
    for _ in 0..50 {
        assert_eq!(RandomSpreadType::Linear.evaluate(&mut a, 24), b.next_int_bounded(24));
    }
    // TRIANGULAR is (nextInt + nextInt) / 2, left operand drawn first, truncating division.
    let mut a = LegacyRandomSource::new(-987654321);
    let mut b = LegacyRandomSource::new(-987654321);
    for _ in 0..50 {
        let x = b.next_int_bounded(9);
        let y = b.next_int_bounded(9);
        assert_eq!(RandomSpreadType::Triangular.evaluate(&mut a, 9), (x + y) / 2);
    }
}

#[test]
fn id_lookups_use_the_java_out_of_bounds_strategy() {
    use minecraft_rust::net::minecraft::server::permissions::PermissionLevel::{by_id as perm, PermissionLevel};
    use minecraft_rust::net::minecraft::world::entity::animal::equine::Markings::{by_id as markings, Markings};
    use minecraft_rust::net::minecraft::world::entity::animal::equine::Variant::{by_id as variant, Variant};
    use minecraft_rust::net::minecraft::world::entity::animal::golem::CopperGolemState::{CopperGolemState, BY_ID as golem};
    use minecraft_rust::net::minecraft::world::scores::DisplaySlot::{DisplaySlot, BY_ID as slot};
    use minecraft_rust::net::minecraft::world::scores::TeamColor::{TeamColor, BY_ID as team};
    // WRAP
    assert_eq!(markings(5), Markings::None);
    assert_eq!(markings(-1), Markings::BlackDots);
    assert_eq!(variant(7), Variant::White);
    assert_eq!(variant(-1), Variant::DarkBrown);
    // ZERO
    assert_eq!(golem(5), CopperGolemState::Idle);
    assert_eq!(golem(-1), CopperGolemState::Idle);
    assert_eq!(slot(19), DisplaySlot::List);
    assert_eq!(slot(18), DisplaySlot::TeamWhite);
    assert_eq!(team(16), TeamColor::Black);
    // CLAMP
    assert_eq!(perm(-3), PermissionLevel::All);
    assert_eq!(perm(9), PermissionLevel::Owners);
    assert_eq!(perm(2), PermissionLevel::Gamemasters);
}

#[test]
fn permission_level_ordering() {
    use minecraft_rust::net::minecraft::server::permissions::PermissionLevel::VALUES;
    for a in VALUES {
        for b in VALUES {
            assert_eq!(a.is_equal_or_higher_than(b), a.id() >= b.id());
        }
    }
}

#[test]
fn anger_level_thresholds() {
    use minecraft_rust::net::minecraft::world::entity::monster::warden::AngerLevel::{by_anger, sorted_levels, AngerLevel};
    assert_eq!(sorted_levels(), [AngerLevel::Angry, AngerLevel::Agitated, AngerLevel::Calm]);
    assert_eq!(by_anger(i32::MIN), AngerLevel::Calm, "below every threshold falls through to CALM");
    assert_eq!(by_anger(0), AngerLevel::Calm);
    assert_eq!(by_anger(39), AngerLevel::Calm);
    assert_eq!(by_anger(40), AngerLevel::Agitated);
    assert_eq!(by_anger(79), AngerLevel::Agitated);
    assert_eq!(by_anger(80), AngerLevel::Angry);
    assert_eq!(by_anger(i32::MAX), AngerLevel::Angry);
    assert!(AngerLevel::Angry.is_angry());
    assert!(!AngerLevel::Agitated.is_angry());
}

#[test]
fn game_type_lookups_and_predicates() {
    use minecraft_rust::net::minecraft::world::level::GameType::*;
    assert_eq!(DEFAULT_MODE, GameType::Survival);
    assert_eq!(by_id(3), GameType::Spectator);
    assert_eq!(by_id(4), GameType::Survival, "ZERO strategy");
    assert_eq!(by_name("creative"), GameType::Creative);
    assert_eq!(by_name("CREATIVE"), GameType::Survival, "unknown names fall back to SURVIVAL");
    assert_eq!(by_name_or("nope", None), None);
    assert_eq!(by_name_or("adventure", None), Some(GameType::Adventure));
    assert_eq!(get_nullable_id(None), -1);
    assert_eq!(get_nullable_id(Some(GameType::Adventure)), 2);
    assert_eq!(by_nullable_id(-1), None);
    assert_eq!(by_nullable_id(-2), Some(GameType::Survival), "only -1 means unset; other ids go through byId");
    assert!(is_valid_id(0) && is_valid_id(3) && !is_valid_id(4) && !is_valid_id(-1));
    assert_eq!(VALUES.map(|g| g.is_block_placing_restricted()), [false, false, true, true]);
    assert_eq!(VALUES.map(|g| g.is_creative()), [false, true, false, false]);
    assert_eq!(VALUES.map(|g| g.is_survival()), [true, false, true, false]);
    assert_eq!(CODEC().by_name("spectator"), Some(GameType::Spectator));
}

#[test]
fn team_color_and_display_slot_lookups() {
    use minecraft_rust::net::minecraft::world::scores::DisplaySlot::{DisplaySlot, CODEC as slot_codec};
    use minecraft_rust::net::minecraft::world::scores::TeamColor::{by_name, TeamColor, VALUES};
    assert_eq!(by_name("light_purple"), Some(TeamColor::LightPurple));
    assert_eq!(by_name("LIGHT_PURPLE"), None);
    for c in VALUES {
        assert_eq!(by_name(c.get_serialized_name()), Some(c));
    }
    // DisplaySlot has 19 constants: above the 16-entry threshold, so its CODEC uses a map.
    assert_eq!(slot_codec().by_name("sidebar.team.gold"), Some(DisplaySlot::TeamGold));
    assert_eq!(slot_codec().by_name("gold"), None);
}

#[test]
fn front_and_top_lookup_table() {
    use minecraft_rust::net::minecraft::core::Direction::VALUES as DIRS;
    use minecraft_rust::net::minecraft::core::FrontAndTop::{from_front_and_top, VALUES};
    let mut hits = 0;
    for front in DIRS {
        for top in DIRS {
            match from_front_and_top(front, top) {
                Some(v) => {
                    assert_eq!((v.front(), v.top()), (front, top));
                    hits += 1;
                }
                None => assert!(!VALUES.iter().any(|v| v.front() == front && v.top() == top)),
            }
        }
    }
    assert_eq!(hits, 12, "exactly the 12 declared pairs resolve; the other 24 are Java null");
}
