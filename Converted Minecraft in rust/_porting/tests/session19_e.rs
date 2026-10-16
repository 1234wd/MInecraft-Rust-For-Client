//! Session 19 batch e: constant holders and small classes. Checks ports against the jar's constant dump and the
//! Java source's behaviour. See the file headers of the ported .rs files for status.
//!
//! Where a class has no `consts.txt` group (or the behaviour is not a constant), the expected
//! values below were produced by a one-off run of the real 26.2 jar on JDK 25 (classpath =
//! `minecraft-merged-deobf-26.2.jar` + `_porting/java-oracle/lib/*`), not derived by hand.
//! Such values are marked `// jar:`.

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

use std::panic::{catch_unwind, AssertUnwindSafe};

/// Runs `f`, expects a panic, and returns its message. (The default hook's stderr line is
/// captured by the test harness; the hook is not swapped because tests run in parallel.)
fn panic_message<R>(f: impl FnOnce() -> R) -> String {
    let r = catch_unwind(AssertUnwindSafe(f));
    match r {
        Ok(_) => panic!("expected a panic, got a value"),
        Err(e) => {
            if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "<non-string panic>".to_string()
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Pure constant holders
// ---------------------------------------------------------------------------------------------

#[test]
fn exit_codes_match_the_jar() {
    use minecraft_rust::net::minecraft::ExitCodes as p;
    assert_all_consts(
        "net.minecraft.ExitCodes",
        &[
            ("EXIT_CODE_CRASH_DEFAULT", i32v(p::EXIT_CODE_CRASH_DEFAULT)),
            ("EXIT_CODE_CRASH_WITHOUT_REPORT", i32v(p::EXIT_CODE_CRASH_WITHOUT_REPORT)),
            ("EXIT_CODE_CRASH_EARLY_NO_MODULES", i32v(p::EXIT_CODE_CRASH_EARLY_NO_MODULES)),
            (
                "EXIT_CODE_CRASH_EARLY_ARGUMENT_LIBRARY_LOAD",
                i32v(p::EXIT_CODE_CRASH_EARLY_ARGUMENT_LIBRARY_LOAD),
            ),
            ("EXIT_CODE_CRASH_EARLY_ARGUMENT_PARSE", i32v(p::EXIT_CODE_CRASH_EARLY_ARGUMENT_PARSE)),
            ("EXIT_CODE_CRASH_SHUTDOWN", i32v(p::EXIT_CODE_CRASH_SHUTDOWN)),
            ("EXIT_CODE_VERSION_PARSING_FAIL", i32v(p::EXIT_CODE_VERSION_PARSING_FAIL)),
            ("EXIT_CODE_CLIENT_WATCHDOG", i32v(p::EXIT_CODE_CLIENT_WATCHDOG)),
        ],
    );
}

#[test]
fn redstone_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::level::redstone::Redstone as p;
    assert_all_consts(
        "net.minecraft.world.level.redstone.Redstone",
        &[
            ("BITS", i32v(p::BITS)),
            ("MASK", i32v(p::MASK)),
            ("SIGNAL_MIN", i32v(p::SIGNAL_MIN)),
            ("SIGNAL_MAX", i32v(p::SIGNAL_MAX)),
            ("SIGNAL_NONE", i32v(p::SIGNAL_NONE)),
        ],
    );
    assert_eq!(p::MASK, (1 << p::BITS) - 1);
}

#[test]
fn handler_names_match_the_jar() {
    use minecraft_rust::net::minecraft::network::HandlerNames as p;
    assert_all_consts(
        "net.minecraft.network.HandlerNames",
        &[
            ("DECOMPRESS", strv(p::DECOMPRESS)),
            ("COMPRESS", strv(p::COMPRESS)),
            ("DECODER", strv(p::DECODER)),
            ("ENCODER", strv(p::ENCODER)),
            ("INBOUND_CONFIG", strv(p::INBOUND_CONFIG)),
            ("OUTBOUND_CONFIG", strv(p::OUTBOUND_CONFIG)),
            ("SPLITTER", strv(p::SPLITTER)),
            ("PREPENDER", strv(p::PREPENDER)),
            ("DECRYPT", strv(p::DECRYPT)),
            ("ENCRYPT", strv(p::ENCRYPT)),
            ("UNBUNDLER", strv(p::UNBUNDLER)),
            ("BUNDLER", strv(p::BUNDLER)),
            ("PACKET_HANDLER", strv(p::PACKET_HANDLER)),
            ("TIMEOUT", strv(p::TIMEOUT)),
            ("LEGACY_QUERY", strv(p::LEGACY_QUERY)),
            ("LATENCY", strv(p::LATENCY)),
        ],
    );
}

#[test]
fn common_colors_match_the_jar() {
    use minecraft_rust::net::minecraft::util::CommonColors as p;
    assert_all_consts(
        "net.minecraft.util.CommonColors",
        &[
            ("WHITE", i32v(p::WHITE)),
            ("BLACK", i32v(p::BLACK)),
            ("GRAY", i32v(p::GRAY)),
            ("DARK_GRAY", i32v(p::DARK_GRAY)),
            ("LIGHT_GRAY", i32v(p::LIGHT_GRAY)),
            ("LIGHTER_GRAY", i32v(p::LIGHTER_GRAY)),
            ("RED", i32v(p::RED)),
            ("SOFT_RED", i32v(p::SOFT_RED)),
            ("GREEN", i32v(p::GREEN)),
            ("BLUE", i32v(p::BLUE)),
            ("YELLOW", i32v(p::YELLOW)),
            ("SOFT_YELLOW", i32v(p::SOFT_YELLOW)),
            ("DARK_PURPLE", i32v(p::DARK_PURPLE)),
            ("HIGH_CONTRAST_DIAMOND", i32v(p::HIGH_CONTRAST_DIAMOND)),
            ("COSMOS_PINK", i32v(p::COSMOS_PINK)),
            ("TEXT_GRAY", i32v(p::TEXT_GRAY)),
        ],
    );
    // The hex spellings in the port's doc comments.
    assert_eq!(p::HIGH_CONTRAST_DIAMOND as u32, 0xFF57FFE1);
    assert_eq!(p::SOFT_RED as u32, 0xFFDF5050);
    assert_eq!(p::DARK_PURPLE as u32, 0xFF500050);
}

#[test]
fn dimension_defaults_match_the_jar() {
    use minecraft_rust::net::minecraft::world::level::dimension::DimensionDefaults as p;
    assert_all_consts(
        "net.minecraft.world.level.dimension.DimensionDefaults",
        &[
            ("CLOUD_THICKNESS", i32v(p::CLOUD_THICKNESS)),
            ("BLOCK_LIGHT_TINT", i32v(p::BLOCK_LIGHT_TINT)),
            ("NIGHT_VISION_COLOR", i32v(p::NIGHT_VISION_COLOR)),
            ("TURTLE_EGG_HATCH_CHANCE", f32v(p::TURTLE_EGG_HATCH_CHANCE)),
            ("OVERWORLD_MIN_Y", i32v(p::OVERWORLD_MIN_Y)),
            ("OVERWORLD_LEVEL_HEIGHT", i32v(p::OVERWORLD_LEVEL_HEIGHT)),
            ("OVERWORLD_GENERATION_HEIGHT", i32v(p::OVERWORLD_GENERATION_HEIGHT)),
            ("OVERWORLD_LOGICAL_HEIGHT", i32v(p::OVERWORLD_LOGICAL_HEIGHT)),
            ("OVERWORLD_CLOUD_HEIGHT", f32v(p::OVERWORLD_CLOUD_HEIGHT)),
            ("NETHER_MIN_Y", i32v(p::NETHER_MIN_Y)),
            ("NETHER_LEVEL_HEIGHT", i32v(p::NETHER_LEVEL_HEIGHT)),
            ("NETHER_GENERATION_HEIGHT", i32v(p::NETHER_GENERATION_HEIGHT)),
            ("NETHER_LOGICAL_HEIGHT", i32v(p::NETHER_LOGICAL_HEIGHT)),
            ("END_MIN_Y", i32v(p::END_MIN_Y)),
            ("END_LEVEL_HEIGHT", i32v(p::END_LEVEL_HEIGHT)),
            ("END_GENERATION_HEIGHT", i32v(p::END_GENERATION_HEIGHT)),
            ("END_LOGICAL_HEIGHT", i32v(p::END_LOGICAL_HEIGHT)),
            ("END_ISLAND_BASE_Y", i32v(p::END_ISLAND_BASE_Y)),
        ],
    );
    assert_eq!(p::BLOCK_LIGHT_TINT as u32, 0xFFFFD88C);
    assert_eq!(p::NIGHT_VISION_COLOR as u32, 0xFF999999);
}

#[test]
fn food_constants_match_the_jar() {
    use minecraft_rust::net::minecraft::world::food::FoodConstants as p;
    assert_all_consts(
        "net.minecraft.world.food.FoodConstants",
        &[
            ("MAX_FOOD", i32v(p::MAX_FOOD)),
            ("MAX_SATURATION", f32v(p::MAX_SATURATION)),
            ("START_SATURATION", f32v(p::START_SATURATION)),
            ("SATURATION_FLOOR", f32v(p::SATURATION_FLOOR)),
            ("EXHAUSTION_DROP", f32v(p::EXHAUSTION_DROP)),
            ("HEALTH_TICK_COUNT", i32v(p::HEALTH_TICK_COUNT)),
            ("HEALTH_TICK_COUNT_SATURATED", i32v(p::HEALTH_TICK_COUNT_SATURATED)),
            ("HEAL_LEVEL", i32v(p::HEAL_LEVEL)),
            ("SPRINT_LEVEL", i32v(p::SPRINT_LEVEL)),
            ("STARVE_LEVEL", i32v(p::STARVE_LEVEL)),
            ("FOOD_SATURATION_POOR", f32v(p::FOOD_SATURATION_POOR)),
            ("FOOD_SATURATION_LOW", f32v(p::FOOD_SATURATION_LOW)),
            ("FOOD_SATURATION_NORMAL", f32v(p::FOOD_SATURATION_NORMAL)),
            ("FOOD_SATURATION_GOOD", f32v(p::FOOD_SATURATION_GOOD)),
            ("FOOD_SATURATION_MAX", f32v(p::FOOD_SATURATION_MAX)),
            ("FOOD_SATURATION_SUPERNATURAL", f32v(p::FOOD_SATURATION_SUPERNATURAL)),
            ("EXHAUSTION_HEAL", f32v(p::EXHAUSTION_HEAL)),
            ("EXHAUSTION_JUMP", f32v(p::EXHAUSTION_JUMP)),
            ("EXHAUSTION_SPRINT_JUMP", f32v(p::EXHAUSTION_SPRINT_JUMP)),
            ("EXHAUSTION_MINE", f32v(p::EXHAUSTION_MINE)),
            ("EXHAUSTION_ATTACK", f32v(p::EXHAUSTION_ATTACK)),
            ("EXHAUSTION_WALK", f32v(p::EXHAUSTION_WALK)),
            ("EXHAUSTION_CROUCH", f32v(p::EXHAUSTION_CROUCH)),
            ("EXHAUSTION_SPRINT", f32v(p::EXHAUSTION_SPRINT)),
            ("EXHAUSTION_SWIM", f32v(p::EXHAUSTION_SWIM)),
        ],
    );
}

#[test]
fn food_saturation_by_modifier_is_float_arithmetic() {
    use minecraft_rust::net::minecraft::world::food::FoodConstants as p;
    let bread = p::saturation_by_modifier(5, p::FOOD_SATURATION_NORMAL);
    assert_eq!(bread.to_bits(), (5.0_f32 * 0.6_f32 * 2.0_f32).to_bits());
    // 7 * 0.6F is not exact in float, so this pins the float-only evaluation order.
    assert_eq!(p::saturation_by_modifier(7, 0.6).to_bits(), (7.0_f32 * 0.6_f32 * 2.0_f32).to_bits());
    assert_eq!(p::saturation_by_modifier(0, p::FOOD_SATURATION_MAX), 0.0);
    assert_eq!(p::saturation_by_modifier(-3, 1.0), -6.0);
    assert_eq!(p::saturation_by_modifier(4, p::FOOD_SATURATION_SUPERNATURAL).to_bits(), (4.0_f32 * 1.2_f32 * 2.0_f32).to_bits());
    // int -> float widening rounds (16777217 is not representable).
    assert_eq!(p::saturation_by_modifier(16_777_217, 1.0), 33_554_432.0);
}

#[test]
fn entity_events_match_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::EntityEvent as p;
    // The dump records Java `byte` fields widened to int.
    let b = |v: i8| i32v(v as i32);
    assert_all_consts(
        "net.minecraft.world.entity.EntityEvent",
        &[
            ("JUMP", b(p::JUMP)),
            ("KINETIC_HIT", b(p::KINETIC_HIT)),
            ("DEATH", b(p::DEATH)),
            ("START_ATTACKING", b(p::START_ATTACKING)),
            ("STOP_ATTACKING", b(p::STOP_ATTACKING)),
            ("TAMING_FAILED", b(p::TAMING_FAILED)),
            ("TAMING_SUCCEEDED", b(p::TAMING_SUCCEEDED)),
            ("SHAKE_WETNESS", b(p::SHAKE_WETNESS)),
            ("USE_ITEM_COMPLETE", b(p::USE_ITEM_COMPLETE)),
            ("EAT_GRASS", b(p::EAT_GRASS)),
            ("OFFER_FLOWER", b(p::OFFER_FLOWER)),
            ("LOVE_HEARTS", b(p::LOVE_HEARTS)),
            ("VILLAGER_ANGRY", b(p::VILLAGER_ANGRY)),
            ("VILLAGER_HAPPY", b(p::VILLAGER_HAPPY)),
            ("WITCH_HAT_MAGIC", b(p::WITCH_HAT_MAGIC)),
            ("ZOMBIE_CONVERTING", b(p::ZOMBIE_CONVERTING)),
            ("FIREWORKS_EXPLODE", b(p::FIREWORKS_EXPLODE)),
            ("IN_LOVE_HEARTS", b(p::IN_LOVE_HEARTS)),
            ("SQUID_ANIM_SYNCH", b(p::SQUID_ANIM_SYNCH)),
            ("SILVERFISH_MERGE_ANIM", b(p::SILVERFISH_MERGE_ANIM)),
            ("GUARDIAN_ATTACK_SOUND", b(p::GUARDIAN_ATTACK_SOUND)),
            ("REDUCED_DEBUG_INFO", b(p::REDUCED_DEBUG_INFO)),
            ("FULL_DEBUG_INFO", b(p::FULL_DEBUG_INFO)),
            ("PERMISSION_LEVEL_ALL", b(p::PERMISSION_LEVEL_ALL)),
            ("PERMISSION_LEVEL_MODERATORS", b(p::PERMISSION_LEVEL_MODERATORS)),
            ("PERMISSION_LEVEL_GAMEMASTERS", b(p::PERMISSION_LEVEL_GAMEMASTERS)),
            ("PERMISSION_LEVEL_ADMINS", b(p::PERMISSION_LEVEL_ADMINS)),
            ("PERMISSION_LEVEL_OWNERS", b(p::PERMISSION_LEVEL_OWNERS)),
            ("FISHING_ROD_REEL_IN", b(p::FISHING_ROD_REEL_IN)),
            ("ARMORSTAND_WOBBLE", b(p::ARMORSTAND_WOBBLE)),
            ("STOP_OFFER_FLOWER", b(p::STOP_OFFER_FLOWER)),
            ("PROTECTED_FROM_DEATH", b(p::PROTECTED_FROM_DEATH)),
            ("DOLPHIN_LOOKING_FOR_TREASURE", b(p::DOLPHIN_LOOKING_FOR_TREASURE)),
            ("RAVAGER_STUNNED", b(p::RAVAGER_STUNNED)),
            ("TRUSTING_FAILED", b(p::TRUSTING_FAILED)),
            ("TRUSTING_SUCCEEDED", b(p::TRUSTING_SUCCEEDED)),
            ("VILLAGER_SWEAT", b(p::VILLAGER_SWEAT)),
            ("FOX_EAT", b(p::FOX_EAT)),
            ("TELEPORT", b(p::TELEPORT)),
            ("MAINHAND_BREAK", b(p::MAINHAND_BREAK)),
            ("OFFHAND_BREAK", b(p::OFFHAND_BREAK)),
            ("HEAD_BREAK", b(p::HEAD_BREAK)),
            ("CHEST_BREAK", b(p::CHEST_BREAK)),
            ("LEGS_BREAK", b(p::LEGS_BREAK)),
            ("FEET_BREAK", b(p::FEET_BREAK)),
            ("HONEY_SLIDE", b(p::HONEY_SLIDE)),
            ("HONEY_JUMP", b(p::HONEY_JUMP)),
            ("SWAP_HANDS", b(p::SWAP_HANDS)),
            ("CANCEL_SHAKE_WETNESS", b(p::CANCEL_SHAKE_WETNESS)),
            ("START_RAM", b(p::START_RAM)),
            ("END_RAM", b(p::END_RAM)),
            ("POOF", b(p::POOF)),
            ("TENDRILS_SHIVER", b(p::TENDRILS_SHIVER)),
            ("SONIC_CHARGE", b(p::SONIC_CHARGE)),
            ("SNIFFER_DIGGING_SOUND", b(p::SNIFFER_DIGGING_SOUND)),
            ("ARMADILLO_PEEK", b(p::ARMADILLO_PEEK)),
            ("BODY_BREAK", b(p::BODY_BREAK)),
            ("SHAKE", b(p::SHAKE)),
            ("DROWN_PARTICLES", b(p::DROWN_PARTICLES)),
            ("SADDLE_BREAK", b(p::SADDLE_BREAK)),
            ("RAVAGER_ROARED", b(p::RAVAGER_ROARED)),
            ("TNT_PRIME", b(p::TNT_PRIME)),
        ],
    );
}

#[test]
fn level_events_match_the_jar() {
    use minecraft_rust::net::minecraft::world::level::block::LevelEvent as p;
    assert_all_consts(
        "net.minecraft.world.level.block.LevelEvent",
        &[
            ("SOUND_DISPENSER_DISPENSE", i32v(p::SOUND_DISPENSER_DISPENSE)),
            ("SOUND_DISPENSER_FAIL", i32v(p::SOUND_DISPENSER_FAIL)),
            ("SOUND_DISPENSER_PROJECTILE_LAUNCH", i32v(p::SOUND_DISPENSER_PROJECTILE_LAUNCH)),
            ("SOUND_FIREWORK_SHOOT", i32v(p::SOUND_FIREWORK_SHOOT)),
            ("SOUND_EXTINGUISH_FIRE", i32v(p::SOUND_EXTINGUISH_FIRE)),
            ("SOUND_PLAY_JUKEBOX_SONG", i32v(p::SOUND_PLAY_JUKEBOX_SONG)),
            ("SOUND_STOP_JUKEBOX_SONG", i32v(p::SOUND_STOP_JUKEBOX_SONG)),
            ("SOUND_GHAST_WARNING", i32v(p::SOUND_GHAST_WARNING)),
            ("SOUND_GHAST_FIREBALL", i32v(p::SOUND_GHAST_FIREBALL)),
            ("SOUND_DRAGON_FIREBALL", i32v(p::SOUND_DRAGON_FIREBALL)),
            ("SOUND_BLAZE_FIREBALL", i32v(p::SOUND_BLAZE_FIREBALL)),
            ("SOUND_ZOMBIE_WOODEN_DOOR", i32v(p::SOUND_ZOMBIE_WOODEN_DOOR)),
            ("SOUND_ZOMBIE_IRON_DOOR", i32v(p::SOUND_ZOMBIE_IRON_DOOR)),
            ("SOUND_ZOMBIE_DOOR_CRASH", i32v(p::SOUND_ZOMBIE_DOOR_CRASH)),
            ("SOUND_WITHER_BLOCK_BREAK", i32v(p::SOUND_WITHER_BLOCK_BREAK)),
            ("SOUND_WITHER_BOSS_SPAWN", i32v(p::SOUND_WITHER_BOSS_SPAWN)),
            ("SOUND_WITHER_BOSS_SHOOT", i32v(p::SOUND_WITHER_BOSS_SHOOT)),
            ("SOUND_BAT_LIFTOFF", i32v(p::SOUND_BAT_LIFTOFF)),
            ("SOUND_ZOMBIE_INFECTED", i32v(p::SOUND_ZOMBIE_INFECTED)),
            ("SOUND_ZOMBIE_CONVERTED", i32v(p::SOUND_ZOMBIE_CONVERTED)),
            ("SOUND_DRAGON_DEATH", i32v(p::SOUND_DRAGON_DEATH)),
            ("SOUND_ANVIL_BROKEN", i32v(p::SOUND_ANVIL_BROKEN)),
            ("SOUND_ANVIL_USED", i32v(p::SOUND_ANVIL_USED)),
            ("SOUND_ANVIL_LAND", i32v(p::SOUND_ANVIL_LAND)),
            ("SOUND_PORTAL_TRAVEL", i32v(p::SOUND_PORTAL_TRAVEL)),
            ("SOUND_CHORUS_GROW", i32v(p::SOUND_CHORUS_GROW)),
            ("SOUND_CHORUS_DEATH", i32v(p::SOUND_CHORUS_DEATH)),
            ("SOUND_BREWING_STAND_BREW", i32v(p::SOUND_BREWING_STAND_BREW)),
            ("SOUND_END_PORTAL_SPAWN", i32v(p::SOUND_END_PORTAL_SPAWN)),
            ("SOUND_PHANTOM_BITE", i32v(p::SOUND_PHANTOM_BITE)),
            ("SOUND_ZOMBIE_TO_DROWNED", i32v(p::SOUND_ZOMBIE_TO_DROWNED)),
            ("SOUND_HUSK_TO_ZOMBIE", i32v(p::SOUND_HUSK_TO_ZOMBIE)),
            ("SOUND_GRINDSTONE_USED", i32v(p::SOUND_GRINDSTONE_USED)),
            ("SOUND_PAGE_TURN", i32v(p::SOUND_PAGE_TURN)),
            ("SOUND_SMITHING_TABLE_USED", i32v(p::SOUND_SMITHING_TABLE_USED)),
            ("SOUND_POINTED_DRIPSTONE_LAND", i32v(p::SOUND_POINTED_DRIPSTONE_LAND)),
            ("SOUND_DRIP_LAVA_INTO_CAULDRON", i32v(p::SOUND_DRIP_LAVA_INTO_CAULDRON)),
            ("SOUND_DRIP_WATER_INTO_CAULDRON", i32v(p::SOUND_DRIP_WATER_INTO_CAULDRON)),
            ("SOUND_SKELETON_TO_STRAY", i32v(p::SOUND_SKELETON_TO_STRAY)),
            ("SOUND_CRAFTER_CRAFT", i32v(p::SOUND_CRAFTER_CRAFT)),
            ("SOUND_CRAFTER_FAIL", i32v(p::SOUND_CRAFTER_FAIL)),
            ("SOUND_WIND_CHARGE_SHOOT", i32v(p::SOUND_WIND_CHARGE_SHOOT)),
            ("SOUND_SULFUR_SPIKE_LAND", i32v(p::SOUND_SULFUR_SPIKE_LAND)),
            ("COMPOSTER_FILL", i32v(p::COMPOSTER_FILL)),
            ("LAVA_FIZZ", i32v(p::LAVA_FIZZ)),
            ("REDSTONE_TORCH_BURNOUT", i32v(p::REDSTONE_TORCH_BURNOUT)),
            ("END_PORTAL_FRAME_FILL", i32v(p::END_PORTAL_FRAME_FILL)),
            ("DRIPSTONE_DRIP", i32v(p::DRIPSTONE_DRIP)),
            ("PARTICLES_AND_SOUND_PLANT_GROWTH", i32v(p::PARTICLES_AND_SOUND_PLANT_GROWTH)),
            ("PARTICLES_SHOOT_SMOKE", i32v(p::PARTICLES_SHOOT_SMOKE)),
            ("PARTICLES_DESTROY_BLOCK", i32v(p::PARTICLES_DESTROY_BLOCK)),
            ("PARTICLES_SPELL_POTION_SPLASH", i32v(p::PARTICLES_SPELL_POTION_SPLASH)),
            ("PARTICLES_EYE_OF_ENDER_DEATH", i32v(p::PARTICLES_EYE_OF_ENDER_DEATH)),
            ("PARTICLES_MOBBLOCK_SPAWN", i32v(p::PARTICLES_MOBBLOCK_SPAWN)),
            ("PARTICLES_DRAGON_FIREBALL_SPLASH", i32v(p::PARTICLES_DRAGON_FIREBALL_SPLASH)),
            ("PARTICLES_INSTANT_POTION_SPLASH", i32v(p::PARTICLES_INSTANT_POTION_SPLASH)),
            ("PARTICLES_DRAGON_BLOCK_BREAK", i32v(p::PARTICLES_DRAGON_BLOCK_BREAK)),
            ("PARTICLES_WATER_EVAPORATING", i32v(p::PARTICLES_WATER_EVAPORATING)),
            ("PARTICLES_SHOOT_WHITE_SMOKE", i32v(p::PARTICLES_SHOOT_WHITE_SMOKE)),
            ("PARTICLES_BEE_GROWTH", i32v(p::PARTICLES_BEE_GROWTH)),
            ("PARTICLES_TURTLE_EGG_PLACEMENT", i32v(p::PARTICLES_TURTLE_EGG_PLACEMENT)),
            ("PARTICLES_SMASH_ATTACK", i32v(p::PARTICLES_SMASH_ATTACK)),
            ("ANIMATION_END_GATEWAY_SPAWN", i32v(p::ANIMATION_END_GATEWAY_SPAWN)),
            ("ANIMATION_DRAGON_SUMMON_ROAR", i32v(p::ANIMATION_DRAGON_SUMMON_ROAR)),
            ("PARTICLES_ELECTRIC_SPARK", i32v(p::PARTICLES_ELECTRIC_SPARK)),
            ("PARTICLES_AND_SOUND_WAX_ON", i32v(p::PARTICLES_AND_SOUND_WAX_ON)),
            ("PARTICLES_WAX_OFF", i32v(p::PARTICLES_WAX_OFF)),
            ("PARTICLES_SCRAPE", i32v(p::PARTICLES_SCRAPE)),
            ("PARTICLES_SCULK_CHARGE", i32v(p::PARTICLES_SCULK_CHARGE)),
            ("PARTICLES_SCULK_SHRIEK", i32v(p::PARTICLES_SCULK_SHRIEK)),
            (
                "PARTICLES_AND_SOUND_BRUSH_BLOCK_COMPLETE",
                i32v(p::PARTICLES_AND_SOUND_BRUSH_BLOCK_COMPLETE),
            ),
            ("PARTICLES_EGG_CRACK", i32v(p::PARTICLES_EGG_CRACK)),
            ("PARTICLES_TRIAL_SPAWNER_SPAWN", i32v(p::PARTICLES_TRIAL_SPAWNER_SPAWN)),
            ("PARTICLES_TRIAL_SPAWNER_SPAWN_MOB_AT", i32v(p::PARTICLES_TRIAL_SPAWNER_SPAWN_MOB_AT)),
            ("PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER", i32v(p::PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER)),
            ("ANIMATION_TRIAL_SPAWNER_EJECT_ITEM", i32v(p::ANIMATION_TRIAL_SPAWNER_EJECT_ITEM)),
            ("ANIMATION_VAULT_ACTIVATE", i32v(p::ANIMATION_VAULT_ACTIVATE)),
            ("ANIMATION_VAULT_DEACTIVATE", i32v(p::ANIMATION_VAULT_DEACTIVATE)),
            ("ANIMATION_VAULT_EJECT_ITEM", i32v(p::ANIMATION_VAULT_EJECT_ITEM)),
            ("ANIMATION_SPAWN_COBWEB", i32v(p::ANIMATION_SPAWN_COBWEB)),
            (
                "PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER_OMINOUS",
                i32v(p::PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER_OMINOUS),
            ),
            (
                "PARTICLES_TRIAL_SPAWNER_BECOME_OMINOUS",
                i32v(p::PARTICLES_TRIAL_SPAWNER_BECOME_OMINOUS),
            ),
            ("PARTICLES_TRIAL_SPAWNER_SPAWN_ITEM", i32v(p::PARTICLES_TRIAL_SPAWNER_SPAWN_ITEM)),
        ],
    );
}

#[test]
fn enemy_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::monster::Enemy as p;
    assert_all_consts(
        "net.minecraft.world.entity.monster.Enemy",
        &[
            ("XP_REWARD_NONE", i32v(p::XP_REWARD_NONE)),
            ("XP_REWARD_SMALL", i32v(p::XP_REWARD_SMALL)),
            ("XP_REWARD_MEDIUM", i32v(p::XP_REWARD_MEDIUM)),
            ("XP_REWARD_LARGE", i32v(p::XP_REWARD_LARGE)),
            ("XP_REWARD_HUGE", i32v(p::XP_REWARD_HUGE)),
            ("XP_REWARD_BOSS", i32v(p::XP_REWARD_BOSS)),
        ],
    );
    // The marker trait is implementable with no members.
    struct Zombie;
    impl p::Enemy for Zombie {}
    fn is_enemy<T: p::Enemy>(_: &T) -> bool {
        true
    }
    assert!(is_enemy(&Zombie));
}

#[test]
fn mojang_math_constants_match_the_jar() {
    use minecraft_rust::com::mojang::math::Constants as p;
    // jar: Float.floatToRawIntBits of each field.
    assert_eq!(p::PI.to_bits(), 0x40490fdb);
    assert_eq!(p::RAD_TO_DEG.to_bits(), 0x42652ee0);
    assert_eq!(p::DEG_TO_RAD.to_bits(), 0x3c8efa35);
    assert_eq!(p::EPSILON.to_bits(), 0x358637bd);
    // RAD_TO_DEG is a FLOAT division by the rounded pi; the f64 route lands one ulp higher.
    assert_eq!(((180.0_f64 / std::f64::consts::PI) as f32).to_bits(), 0x42652ee1);
    assert_ne!(p::RAD_TO_DEG.to_bits(), ((180.0_f64 / std::f64::consts::PI) as f32).to_bits());
}

// ---------------------------------------------------------------------------------------------
// VarInt / VarLong -- the wire format, so exact bytes.
// ---------------------------------------------------------------------------------------------

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

#[test]
fn varint_max_size_matches_the_jar() {
    use minecraft_rust::net::minecraft::network::VarInt as p;
    assert_all_consts("net.minecraft.network.VarInt", &[("MAX_VARINT_SIZE", i32v(p::MAX_VARINT_SIZE))]);
}

#[test]
fn varint_encodings_match_the_jar() {
    use minecraft_rust::net::minecraft::network::VarInt as p;
    // jar: VarInt.write into Unpooled.buffer(), then getByteSize and read back.
    let cases: &[(i32, i32, &str)] = &[
        (0, 1, "00"),
        (1, 1, "01"),
        (127, 1, "7f"),
        (128, 2, "8001"),
        (255, 2, "ff01"),
        (25565, 3, "ddc701"),
        (2097151, 3, "ffff7f"),
        (2097152, 4, "80808001"),
        (268435455, 4, "ffffff7f"),
        (268435456, 5, "8080808001"),
        (i32::MAX, 5, "ffffffff07"),
        (-1, 5, "ffffffff0f"),
        (i32::MIN, 5, "8080808008"),
        (-128, 5, "80ffffff0f"),
    ];
    for &(v, size, bytes) in cases {
        let mut out = Vec::new();
        let returned = p::write(&mut out, v);
        assert_eq!(hex(returned), bytes, "write({v})");
        assert_eq!(p::get_byte_size(v), size, "getByteSize({v})");
        assert_eq!(out.len() as i32, size, "byte count of {v}");
        let mut ridx = 0;
        assert_eq!(p::read(&out, &mut ridx), v, "read back {v}");
        assert_eq!(ridx, out.len(), "readerIndex after {v}");
    }
}

#[test]
fn varint_write_appends_and_read_advances() {
    use minecraft_rust::net::minecraft::network::VarInt as p;
    let mut buf = vec![0xAA];
    p::write(p::write(&mut buf, 300), -1);
    assert_eq!(hex(&buf), "aaac02ffffffff0f");
    let mut ridx = 1;
    assert_eq!(p::read(&buf, &mut ridx), 300);
    assert_eq!(ridx, 3);
    assert_eq!(p::read(&buf, &mut ridx), -1);
    assert_eq!(ridx, 8);
}

#[test]
fn varint_read_quirks_match_the_jar() {
    use minecraft_rust::net::minecraft::network::VarInt as p;
    // jar: five bytes whose fifth has bits above bit 31 -> silently dropped, reads -1.
    let mut ridx = 0;
    assert_eq!(p::read(&unhex("ffffffff7f"), &mut ridx), -1);
    // jar: a sixth byte -> RuntimeException("VarInt too big").
    let six = unhex("808080808001");
    assert_eq!(
        panic_message(|| p::read(&six, &mut 0)),
        "java.lang.RuntimeException: VarInt too big"
    );
    // jar: five continuation bytes and nothing after -> netty underflow on the sixth read.
    let five = unhex("ffffffffff");
    assert_eq!(
        panic_message(|| p::read(&five, &mut 0)),
        "java.lang.IndexOutOfBoundsException: readerIndex(5) + length(1) exceeds writerIndex(5): \
         UnpooledHeapByteBuf(ridx: 5, widx: 5, cap: 5/5)"
    );
    let short = unhex("8080");
    assert_eq!(
        panic_message(|| p::read(&short, &mut 0)),
        "java.lang.IndexOutOfBoundsException: readerIndex(2) + length(1) exceeds writerIndex(2): \
         UnpooledHeapByteBuf(ridx: 2, widx: 2, cap: 2/2)"
    );
    // jar: same buffer state reached after a prior readByte.
    let short2 = unhex("0580");
    assert_eq!(
        panic_message(|| p::read(&short2, &mut 1)),
        "java.lang.IndexOutOfBoundsException: readerIndex(2) + length(1) exceeds writerIndex(2): \
         UnpooledHeapByteBuf(ridx: 2, widx: 2, cap: 2/2)"
    );
    // jar: Unpooled.wrappedBuffer(new byte[0]) is EMPTY_BUFFER -> null message.
    assert_eq!(panic_message(|| p::read(&[], &mut 0)), "java.lang.IndexOutOfBoundsException");
    // jar: hasContinuationBit((byte)0x80), (byte)0x7f, (byte)-1.
    assert!(p::has_continuation_bit(0x80_u8 as i8));
    assert!(!p::has_continuation_bit(0x7f));
    assert!(p::has_continuation_bit(-1));
}

#[test]
fn varint_byte_size_agrees_with_write_everywhere() {
    use minecraft_rust::net::minecraft::network::VarInt as p;
    let mut v: i64 = i32::MIN as i64;
    while v <= i32::MAX as i64 {
        let x = v as i32;
        for y in [x, x.wrapping_add(1), x.wrapping_sub(1)] {
            let mut out = Vec::new();
            p::write(&mut out, y);
            assert_eq!(p::get_byte_size(y), out.len() as i32, "{y}");
            assert_eq!(p::read(&out, &mut 0), y);
        }
        v += 7_654_321;
    }
    for shift in 0..32 {
        let y = 1_i32.wrapping_shl(shift);
        let mut out = Vec::new();
        p::write(&mut out, y);
        assert_eq!(p::get_byte_size(y), out.len() as i32, "1 << {shift}");
    }
}

#[test]
fn varlong_encodings_match_the_jar() {
    use minecraft_rust::net::minecraft::network::VarLong as p;
    // jar: VarLong.write into Unpooled.buffer(), then getByteSize and read back.
    let cases: &[(i64, i32, &str)] = &[
        (0, 1, "00"),
        (1, 1, "01"),
        (127, 1, "7f"),
        (128, 2, "8001"),
        (255, 2, "ff01"),
        (25565, 3, "ddc701"),
        (2097151, 3, "ffff7f"),
        (i32::MAX as i64, 5, "ffffffff07"),
        (-1, 10, "ffffffffffffffffff01"),
        (i32::MIN as i64, 10, "80808080f8ffffffff01"),
        (i64::MAX, 9, "ffffffffffffffff7f"),
        (i64::MIN, 10, "80808080808080808001"),
        (1_i64 << 56, 9, "808080808080808001"),
    ];
    for &(v, size, bytes) in cases {
        let mut out = Vec::new();
        assert_eq!(hex(p::write(&mut out, v)), bytes, "write({v})");
        assert_eq!(p::get_byte_size(v), size, "getByteSize({v})");
        assert_eq!(out.len() as i32, size);
        let mut ridx = 0;
        assert_eq!(p::read(&out, &mut ridx), v, "read back {v}");
        assert_eq!(ridx, out.len());
    }
}

#[test]
fn varlong_read_quirks_match_the_jar() {
    use minecraft_rust::net::minecraft::network::VarLong as p;
    // jar: ten bytes, the tenth contributes only its lowest bit (shift 63) -> -1.
    assert_eq!(p::read(&unhex("ffffffffffffffffff7f"), &mut 0), -1);
    // jar: an eleventh byte -> RuntimeException("VarLong too big").
    let eleven = unhex("8080808080808080808001");
    assert_eq!(
        panic_message(|| p::read(&eleven, &mut 0)),
        "java.lang.RuntimeException: VarLong too big"
    );
    assert_eq!(panic_message(|| p::read(&[], &mut 0)), "java.lang.IndexOutOfBoundsException");
    let short = unhex("ff");
    assert_eq!(
        panic_message(|| p::read(&short, &mut 0)),
        "java.lang.IndexOutOfBoundsException: readerIndex(1) + length(1) exceeds writerIndex(1): \
         UnpooledHeapByteBuf(ridx: 1, widx: 1, cap: 1/1)"
    );
    assert!(p::has_continuation_bit(-128));
    assert!(!p::has_continuation_bit(127));
    for shift in 0..64 {
        let y = 1_i64.wrapping_shl(shift);
        for z in [y, y.wrapping_sub(1), y.wrapping_neg()] {
            let mut out = Vec::new();
            p::write(&mut out, z);
            assert_eq!(p::get_byte_size(z), out.len() as i32, "{z}");
            assert_eq!(p::read(&out, &mut 0), z);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// PktUtils (rcon)
// ---------------------------------------------------------------------------------------------

#[test]
fn pkt_utils_constants_match_the_jar() {
    use minecraft_rust::net::minecraft::server::rcon::PktUtils as p;
    assert_all_consts("net.minecraft.server.rcon.PktUtils", &[("MAX_PACKET_SIZE", i32v(p::MAX_PACKET_SIZE))]);
    // jar: new String(PktUtils.HEX_CHAR) (arrays are not in the dump).
    assert_eq!(p::HEX_CHAR.iter().collect::<String>(), "0123456789abcdef");
}

#[test]
fn pkt_utils_string_from_byte_array_matches_the_jar() {
    use minecraft_rust::net::minecraft::server::rcon::PktUtils as p;
    let s = b"hello\0world"; // 11 bytes
    // jar: each call below on "hello\0world".getBytes(UTF_8).
    assert_eq!(p::string_from_byte_array(s, 0, 11), "hello");
    assert_eq!(p::string_from_byte_array(s, 6, 11), "worl", "b[length - 1] is never included");
    assert_eq!(p::string_from_byte_array(s, 2, 4), "l");
    assert_eq!(p::string_from_byte_array(s, 0, 20), "hello", "NUL stops the scan before the overrun");
    assert_eq!(p::string_from_byte_array(s, 5, 11), "");
    assert_eq!(
        panic_message(|| p::string_from_byte_array(s, 20, 11)),
        "java.lang.StringIndexOutOfBoundsException: Range [20, 20 + -10) out of bounds for length 11"
    );
    assert_eq!(
        panic_message(|| p::string_from_byte_array(s, 0, 0)),
        "java.lang.ArrayIndexOutOfBoundsException: Index -1 out of bounds for length 11"
    );
    assert_eq!(p::string_from_byte_array(b"ab", 0, 2), "a");
    assert_eq!(p::string_from_byte_array(b"abc\0", 3, 4), "");
    assert_eq!(
        panic_message(|| p::string_from_byte_array(b"abc\0", -1, 4)),
        "java.lang.ArrayIndexOutOfBoundsException: Index -1 out of bounds for length 4"
    );
    assert_eq!(
        panic_message(|| p::string_from_byte_array(&[], 0, 0)),
        "java.lang.ArrayIndexOutOfBoundsException: Index -1 out of bounds for length 0"
    );
}

/// Code units of `s` as `"%04x"` joined by spaces (the oracle's format).
fn utf16_units(s: &str) -> String {
    s.encode_utf16().map(|u| format!("{u:04x}")).collect::<Vec<_>>().join(" ")
}

#[test]
fn pkt_utils_decodes_utf8_exactly_like_the_jdk() {
    use minecraft_rust::net::minecraft::server::rcon::PktUtils as p;
    // jar (JDK 25): PktUtils.stringFromByteArray(bytes + {0}, 0, bytes.length + 1), as UTF-16 units.
    let cases: &[(&str, &str)] = &[
        ("eda080", "fffd"),
        ("eda0", "fffd"),
        ("ed9fbf", "d7ff"),
        ("ee8080", "e000"),
        ("e0a080", "0800"),
        ("e09f80", "fffd fffd fffd"),
        ("f0908080", "d800 dc00"),
        ("f08f8080", "fffd fffd fffd fffd"),
        ("f48fbfbf", "dbff dfff"),
        ("f4908080", "fffd fffd fffd fffd"),
        ("f09f98", "fffd"),
        ("f09f", "fffd"),
        ("f09f41", "fffd 0041"),
        ("f09f9841", "fffd 0041"),
        ("e282", "fffd"),
        ("e241", "fffd 0041"),
        ("e2", "fffd"),
        ("c3", "fffd"),
        ("c341", "fffd 0041"),
        ("c3a9", "00e9"),
        ("c2a0", "00a0"),
        ("c080", "fffd fffd"),
        ("c1bf", "fffd fffd"),
        ("f5808080", "fffd fffd fffd fffd"),
        ("f8", "fffd"),
        ("ff41", "fffd 0041"),
        ("80", "fffd"),
        ("bf41", "fffd 0041"),
        ("efbfbd", "fffd"),
        ("efbfbe", "fffe"),
        ("f09f9880", "d83d de00"),
        ("e282ac", "20ac"),
        ("41e2", "0041 fffd"),
        ("edbfbf", "fffd"),
        ("edb080", "fffd"),
        ("f0", "fffd"),
        ("f041", "fffd 0041"),
        ("f4", "fffd"),
        ("f490", "fffd fffd"),
        ("f48f", "fffd"),
        ("f48f41", "fffd 0041"),
        ("e0", "fffd"),
        ("e0a0", "fffd"),
        ("e080", "fffd fffd"),
        ("e0a041", "fffd 0041"),
        ("c2c3a9", "fffd 00e9"),
        ("ed", "fffd"),
        ("eda041", "fffd 0041"),
    ];
    for &(input, want) in cases {
        let mut b = unhex(input);
        b.push(0);
        let got = p::string_from_byte_array(&b, 0, b.len() as i32);
        assert_eq!(utf16_units(&got), want, "decode {input}");
    }
    // The case the Rust std lossy decoder gets wrong: an encoded surrogate is ONE replacement.
    assert_eq!(String::from_utf8_lossy(&unhex("eda080")).chars().count(), 3);
}

#[test]
fn pkt_utils_utf8_fuzz_matches_the_jar() {
    use minecraft_rust::javacompat::java_lang::string_hash_code;
    use minecraft_rust::javacompat::java_random::JavaRandom;
    use minecraft_rust::net::minecraft::server::rcon::PktUtils as p;
    // jar: 200000 arrays from java.util.Random(19) (generator mirrored below), each decoded by
    // PktUtils.stringFromByteArray; acc = acc*31 + s.hashCode(); acc = acc*31 + s.length().
    const SPECIAL: [i32; 16] =
        [0xE0, 0xED, 0xF0, 0xF4, 0xF5, 0xF8, 0x41, 0xEF, 0xBF, 0xC0, 0xC1, 0xC2, 0xA0, 0x9F, 0x90, 0x8F];
    let mut r = JavaRandom::new(19);
    let mut acc: i32 = 0;
    let mut total: i32 = 0;
    for _ in 0..200_000 {
        let len = r.next_int_bounded(7) + 1;
        let mut arr = vec![0u8; len as usize + 1];
        for slot in arr.iter_mut().take(len as usize) {
            let k = r.next_int_bounded(4);
            let v = match k {
                0 => 1 + r.next_int_bounded(255),
                1 => 0x80 + r.next_int_bounded(64),
                2 => 0xC0 + r.next_int_bounded(64),
                _ => SPECIAL[r.next_int_bounded(SPECIAL.len() as i32) as usize],
            };
            *slot = v as u8;
        }
        let s = p::string_from_byte_array(&arr, 0, arr.len() as i32);
        let units = s.encode_utf16().count() as i32;
        acc = acc.wrapping_mul(31).wrapping_add(string_hash_code(&s));
        acc = acc.wrapping_mul(31).wrapping_add(units);
        total += units;
    }
    assert_eq!((acc, total), (-793884959, 717482));
}

#[test]
fn pkt_utils_int_readers_match_the_jar() {
    use minecraft_rust::net::minecraft::server::rcon::PktUtils as p;
    let le: [u8; 8] = [0x78, 0x56, 0x34, 0x12, 0xff, 0xfe, 0xfd, 0xfc];
    // jar: little-endian.
    assert_eq!(p::int_from_byte_array_2(&le, 0), 0x12345678);
    assert_eq!(p::int_from_byte_array_2(&le, 4), -50462977); // 0xfcfdfeff
    assert_eq!(p::int_from_byte_array_2(&le, 5), 0, "fewer than 4 bytes left -> 0, no throw");
    assert_eq!(p::int_from_byte_array_2(&le, 2), -16838092); // 0xfeff1234
    assert_eq!(
        panic_message(|| p::int_from_byte_array_2(&le, -1)),
        "java.lang.ArrayIndexOutOfBoundsException: Index -1 out of bounds for length 8"
    );
    assert_eq!(
        panic_message(|| p::int_from_byte_array_3(&le, 6, 12)),
        "java.lang.ArrayIndexOutOfBoundsException: Index 9 out of bounds for length 8",
        "offset + 3 is evaluated first"
    );
    assert_eq!(p::int_from_byte_array_3(&le, 0, 3), 0);
    assert_eq!(p::int_from_byte_array_3(&le, i32::MIN, i32::MAX), 0, "length - offset - 4 wraps");
    // jar: network (big-endian) order.
    assert_eq!(p::int_from_network_byte_array(&le, 0, 8), 0x78563412);
    assert_eq!(p::int_from_network_byte_array(&le, 4, 8), -66052); // 0xfffefdfc
    assert_eq!(
        panic_message(|| p::int_from_network_byte_array(&le, 6, 12)),
        "java.lang.ArrayIndexOutOfBoundsException: Index 8 out of bounds for length 8",
        "offset is evaluated first, so offset + 2 is the first bad index"
    );
    assert_eq!(
        panic_message(|| p::int_from_network_byte_array(&le, -1, 8)),
        "java.lang.ArrayIndexOutOfBoundsException: Index -1 out of bounds for length 8"
    );
    assert_eq!(p::int_from_network_byte_array(&le, 5, 8), 0);
}

#[test]
fn pkt_utils_to_hex_string_matches_the_jar() {
    use minecraft_rust::net::minecraft::server::rcon::PktUtils as p;
    // jar: toHexString((byte) i) for i = -128, -91, ..., 94 (step 37), then -1 and 0.
    let want = [
        (-128, "80"),
        (-91, "a5"),
        (-54, "ca"),
        (-17, "ef"),
        (20, "14"),
        (57, "39"),
        (94, "5e"),
        (-1, "ff"),
        (0, "00"),
    ];
    for (b, s) in want {
        assert_eq!(p::to_hex_string(b as i8), s);
    }
    for b in i8::MIN..=i8::MAX {
        assert_eq!(p::to_hex_string(b), format!("{:02x}", b as u8));
    }
}
