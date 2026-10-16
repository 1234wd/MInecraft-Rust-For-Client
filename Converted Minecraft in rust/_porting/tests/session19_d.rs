//! Session 19 batch d: item, equipment and player enums. Checks ports against the jar's constant dump and the
//! Java source's behaviour. See the file headers of the ported .rs files for status.

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

use std::collections::BTreeSet;

/// The panic message of `f`, which must panic with a `String` or `&str` payload.
fn panic_text(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let err = std::panic::catch_unwind(f).expect_err("expected a panic");
    if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        panic!("panic payload is not a string")
    }
}

/// Java's `AbstractCollection#toString()` for a list of enum constants: `[A, B, C]`.
fn java_list<T: std::fmt::Display>(items: &[T]) -> String {
    let parts: Vec<String> = items.iter().map(|i| i.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

/// `TranslatableContents#toString()` with no arguments, as the dump records a `Component` field.
fn translatable(key: &str) -> String {
    format!("translation{{key='{key}', args=[]}}")
}

// ============================================================================
// EquipmentSlot
// ============================================================================

#[test]
fn equipment_slot_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::VALUES;
    let fq = "net.minecraft.world.entity.EquipmentSlot";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "type", &strv(v.get_type().name()));
        assert_enum_field(fq, v.name(), "index", &i32v(v.get_index()));
        assert_enum_field(fq, v.name(), "countLimit", &i32v(v.count_limit()));
        assert_enum_field(fq, v.name(), "id", &i32v(v.get_id()));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_name()));
        assert_eq!(v.get_serialized_name(), v.get_name());
    }
}

#[test]
fn equipment_slot_type_nested_enum_follows_the_source() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::{Type, TYPE_VALUES};
    // Nested enums are not in the dump; the declaration order comes from the Java source.
    assert_eq!(TYPE_VALUES.map(|t| t.name()), ["HAND", "HUMANOID_ARMOR", "ANIMAL_ARMOR", "SADDLE"]);
    for (i, t) in TYPE_VALUES.iter().enumerate() {
        assert_eq!(t.ordinal(), i as i32);
        assert_eq!(*t as i32, i as i32);
        assert_eq!(t.to_string(), t.name());
    }
    assert_eq!(Type::Hand.name(), "HAND");
}

#[test]
fn equipment_slot_no_count_limit_and_count_limits() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::{EquipmentSlot, NO_COUNT_LIMIT, VALUES};
    assert_eq!(NO_COUNT_LIMIT, 0);
    // The 4-arg constructor (hands) passes countLimit = 0 = NO_COUNT_LIMIT; every other slot has 1.
    for v in VALUES {
        let expected = if matches!(v, EquipmentSlot::Mainhand | EquipmentSlot::Offhand) { NO_COUNT_LIMIT } else { 1 };
        assert_eq!(v.count_limit(), expected, "{v}");
    }
}

#[test]
fn equipment_slot_by_id_uses_ids_not_ordinals_and_zero_strategy() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::{EquipmentSlot, BY_ID, VALUES};
    for v in VALUES {
        assert_eq!(BY_ID(v.get_id()), v);
    }
    assert_eq!(BY_ID(1), EquipmentSlot::Feet, "id 1 is FEET even though OFFHAND has ordinal 1");
    assert_eq!(BY_ID(5), EquipmentSlot::Offhand);
    for oob in [-1, 8, 100, i32::MIN, i32::MAX] {
        assert_eq!(BY_ID(oob), EquipmentSlot::Mainhand, "ZERO strategy for {oob}");
    }
}

#[test]
fn equipment_slot_index_and_filter_bit_arithmetic() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::{EquipmentSlot, VALUES};
    for v in VALUES {
        assert_eq!(v.get_index_i32(0), v.get_index());
        assert_eq!(v.get_index_i32(36), 36 + v.get_index());
        assert_eq!(v.get_filter_bit(0), v.get_id());
        assert_eq!(v.get_filter_bit(1), v.get_id() + 1);
    }
    // Java int overflow wraps.
    assert_eq!(EquipmentSlot::Head.get_index_i32(i32::MAX), i32::MIN + 2);
    assert_eq!(EquipmentSlot::Saddle.get_filter_bit(i32::MAX), i32::MIN + 6);
    assert_eq!(EquipmentSlot::Offhand.get_filter_bit(-5), 0);
}

#[test]
fn equipment_slot_is_armor_and_experience() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::{EquipmentSlot as S, VALUES};
    let armor: Vec<S> = VALUES.into_iter().filter(|s| s.is_armor()).collect();
    assert_eq!(armor, [S::Feet, S::Legs, S::Chest, S::Head, S::Body]);
    let xp: Vec<S> = VALUES.into_iter().filter(|s| s.can_increase_experience()).collect();
    assert_eq!(xp, [S::Mainhand, S::Offhand, S::Feet, S::Legs, S::Chest, S::Head, S::Body]);
}

#[test]
fn equipment_slot_by_name_and_its_exception() {
    use minecraft_rust::net::minecraft::util::StringRepresentable::StringRepresentable;
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::{by_name, VALUES};
    for v in VALUES {
        assert_eq!(by_name(v.get_serialized_name()), v);
        assert_eq!(StringRepresentable::get_serialized_name(&v), v.get_name());
    }
    assert_eq!(
        panic_text(|| {
            by_name("MAINHAND");
        }),
        "java.lang.IllegalArgumentException: Invalid slot 'MAINHAND'"
    );
    assert_eq!(
        panic_text(|| {
            by_name("");
        }),
        "java.lang.IllegalArgumentException: Invalid slot ''"
    );
}

#[test]
fn interaction_hand_as_equipment_slot() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::EquipmentSlot;
    use minecraft_rust::net::minecraft::world::InteractionHand::{as_equipment_slot, InteractionHand};
    assert_eq!(as_equipment_slot(InteractionHand::MainHand), EquipmentSlot::Mainhand);
    assert_eq!(as_equipment_slot(InteractionHand::OffHand), EquipmentSlot::Offhand);
}

// ============================================================================
// ArmorType
// ============================================================================

#[test]
fn armor_type_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::item::equipment::ArmorType::VALUES;
    let fq = "net.minecraft.world.item.equipment.ArmorType";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "slot", &strv(v.get_slot().name()));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_name()));
        assert_enum_field(fq, v.name(), "unitDurability", &i32v(v.unit_durability()));
        assert_eq!(v.get_serialized_name(), v.get_name());
    }
}

#[test]
fn armor_type_durability() {
    use minecraft_rust::net::minecraft::world::item::equipment::ArmorType::ArmorType as A;
    // Leather (multiplier 5) and diamond (33): the familiar vanilla durabilities.
    assert_eq!([A::Helmet, A::Chestplate, A::Leggings, A::Boots].map(|a| a.get_durability(5)), [55, 80, 75, 65]);
    assert_eq!([A::Helmet, A::Chestplate, A::Leggings, A::Boots].map(|a| a.get_durability(33)), [363, 528, 495, 429]);
    assert_eq!(A::Body.get_durability(0), 0);
    assert_eq!(A::Boots.get_durability(-1), -13);
    // Java `11 * Integer.MAX_VALUE` wraps to 2147483637.
    assert_eq!(A::Helmet.get_durability(i32::MAX), 2147483637);
}

// ============================================================================
// EquipmentSlotGroup
// ============================================================================

#[test]
fn equipment_slot_group_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlotGroup::VALUES;
    let fq = "net.minecraft.world.entity.EquipmentSlotGroup";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "key", &strv(v.get_serialized_name()));
        // `slots` is recorded by List#toString: the predicate's result over EquipmentSlot.VALUES.
        assert_enum_field(fq, v.name(), "slots", &strv(&java_list(v.slots())));
    }
}

#[test]
fn equipment_slot_group_test_agrees_with_slots_and_the_source_predicates() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::{self as es, EquipmentSlot as S};
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlotGroup::{EquipmentSlotGroup as G, VALUES};
    for g in VALUES {
        for s in es::VALUES {
            assert_eq!(g.test(s), g.slots().contains(&s), "{g} / {s}");
        }
    }
    for s in es::VALUES {
        assert!(G::Any.test(s));
        assert_eq!(G::Hand.test(s), s.get_type() == es::Type::Hand);
        assert_eq!(G::Armor.test(s), s.is_armor());
    }
    assert!(G::Armor.test(S::Body), "BODY is ANIMAL_ARMOR, which isArmor covers");
    assert!(!G::Armor.test(S::Saddle));
    assert!(!G::Hand.test(S::Head));
}

#[test]
fn equipment_slot_group_by_slot() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot as es;
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlotGroup::{by_slot, EquipmentSlotGroup as G};
    let got: Vec<G> = es::VALUES.into_iter().map(by_slot).collect();
    assert_eq!(got, [G::Mainhand, G::Offhand, G::Feet, G::Legs, G::Chest, G::Head, G::Body, G::Saddle]);
    for s in es::VALUES {
        assert_eq!(by_slot(s).slots(), [s]);
        assert_eq!(by_slot(s).get_serialized_name(), s.get_serialized_name());
    }
}

#[test]
fn equipment_slot_group_by_id_and_iteration() {
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlot::EquipmentSlot as S;
    use minecraft_rust::net::minecraft::world::entity::EquipmentSlotGroup::{EquipmentSlotGroup as G, BY_ID, VALUES};
    for v in VALUES {
        assert_eq!(BY_ID(v.id()), v);
    }
    for oob in [-1, 11, i32::MIN, i32::MAX] {
        assert_eq!(BY_ID(oob), G::Any);
    }
    let mut seen = Vec::new();
    for s in G::Armor {
        seen.push(s);
    }
    assert_eq!(seen, [S::Feet, S::Legs, S::Chest, S::Head, S::Body]);
    assert_eq!(G::Hand.iterator().collect::<Vec<_>>(), [S::Mainhand, S::Offhand]);
    assert_eq!(G::Any.slots().len(), 8);
}

// ============================================================================
// ItemDisplayContext / ItemUseAnimation / SwingAnimationType
// ============================================================================

#[test]
fn item_display_context_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::item::ItemDisplayContext::VALUES;
    let fq = "net.minecraft.world.item.ItemDisplayContext";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.get_id() as i32));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
    }
}

#[test]
fn item_display_context_behaviour() {
    use minecraft_rust::net::minecraft::world::item::ItemDisplayContext::{ItemDisplayContext as C, BY_ID, VALUES};
    let first: Vec<C> = VALUES.into_iter().filter(|c| c.first_person()).collect();
    assert_eq!(first, [C::FirstPersonLeftHand, C::FirstPersonRightHand]);
    let left: Vec<C> = VALUES.into_iter().filter(|c| c.left_hand()).collect();
    assert_eq!(left, [C::ThirdPersonLeftHand, C::FirstPersonLeftHand]);
    for v in VALUES {
        assert_eq!(BY_ID(v.get_id() as i32), v);
    }
    for oob in [-1, 10, 127, -128, i32::MAX] {
        assert_eq!(BY_ID(oob), C::None);
    }
}

#[test]
fn item_use_animation_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::item::ItemUseAnimation::{by_id, ItemUseAnimation as A, VALUES};
    let fq = "net.minecraft.world.item.ItemUseAnimation";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.get_id()));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_enum_field(fq, v.name(), "customArmTransform", &boolv(v.has_custom_arm_transform()));
        assert_eq!(by_id(v.get_id()), v);
    }
    for oob in [-1, 12, i32::MIN] {
        assert_eq!(by_id(oob), A::None);
    }
}

#[test]
fn swing_animation_type_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::item::SwingAnimationType::{by_id, SwingAnimationType as T, VALUES};
    let fq = "net.minecraft.world.item.SwingAnimationType";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.get_id()));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_eq!(by_id(v.get_id()), v);
    }
    assert_eq!(by_id(3), T::None);
    assert_eq!(by_id(-1), T::None);
}

// ============================================================================
// Recipe book categories, MapPostProcessing
// ============================================================================

#[test]
fn cooking_book_category_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::item::crafting::CookingBookCategory::{by_id, CookingBookCategory as C, VALUES};
    let fq = "net.minecraft.world.item.crafting.CookingBookCategory";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_eq!(by_id(v.id()), v);
    }
    assert_eq!(by_id(3), C::Food);
    assert_eq!(by_id(-1), C::Food);
}

#[test]
fn crafting_book_category_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::item::crafting::CraftingBookCategory::{CraftingBookCategory as C, BY_ID, VALUES};
    let fq = "net.minecraft.world.item.crafting.CraftingBookCategory";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_eq!(BY_ID(v.id()), v);
    }
    assert_eq!(BY_ID(4), C::Building);
    assert_eq!(BY_ID(-1), C::Building);
}

#[test]
fn map_post_processing_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::item::component::MapPostProcessing::{MapPostProcessing as M, ID_MAP, VALUES};
    let fq = "net.minecraft.world.item.component.MapPostProcessing";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_eq!(ID_MAP(v.id()), v);
    }
    assert_eq!(ID_MAP(2), M::Lock);
    assert_eq!(ID_MAP(-1), M::Lock);
}

// ============================================================================
// HumanoidArm, Pose
// ============================================================================

#[test]
fn humanoid_arm_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::HumanoidArm::{by_id, HumanoidArm as H, VALUES};
    let fq = "net.minecraft.world.entity.HumanoidArm";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_enum_field(fq, v.name(), "caption", &strv(&translatable(v.caption_translation_key())));
        assert_ne!(v.get_opposite(), v);
        assert_eq!(v.get_opposite().get_opposite(), v);
        assert_eq!(by_id(v.id()), v);
    }
    assert_eq!(H::Left.get_opposite(), H::Right);
    assert_eq!(by_id(2), H::Left);
    assert_eq!(by_id(-1), H::Left);
}

#[test]
fn pose_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::Pose::{Pose as P, BY_ID, VALUES};
    let fq = "net.minecraft.world.entity.Pose";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "name", &strv(v.get_serialized_name()));
        assert_eq!(BY_ID(v.id()), v);
    }
    for oob in [-1, 18, i32::MIN, i32::MAX] {
        assert_eq!(BY_ID(oob), P::Standing);
    }
}

// ============================================================================
// Player options: ChatVisiblity, ParticleStatus, PlayerModelPart, PlayerModelType
// ============================================================================

#[test]
fn chat_visiblity_matches_the_jar_and_wraps() {
    use minecraft_rust::net::minecraft::world::entity::player::ChatVisiblity::{by_id, ChatVisiblity as C, VALUES};
    let fq = "net.minecraft.world.entity.player.ChatVisiblity";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "caption", &strv(&translatable(v.caption_translation_key())));
        assert_eq!(by_id(v.id()), v);
    }
    // WRAP = Math.floorMod(id, 3).
    assert_eq!(by_id(3), C::Full);
    assert_eq!(by_id(4), C::System);
    assert_eq!(by_id(-1), C::Hidden);
    assert_eq!(by_id(-3), C::Full);
    assert_eq!(by_id(i32::MIN), C::System); // floorMod(-2147483648, 3) == 1
    assert_eq!(by_id(i32::MAX), C::System); // floorMod(2147483647, 3) == 1
}

#[test]
fn particle_status_matches_the_jar_and_wraps() {
    use minecraft_rust::net::minecraft::server::level::ParticleStatus::{by_id, ParticleStatus as P, VALUES};
    let fq = "net.minecraft.server.level.ParticleStatus";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "caption", &strv(&translatable(v.caption_translation_key())));
        assert_eq!(by_id(v.id()), v);
    }
    assert_eq!(by_id(3), P::All);
    assert_eq!(by_id(-1), P::Minimal);
    assert_eq!(by_id(-2), P::Decreased);
    assert_eq!(by_id(i32::MIN), P::Decreased);
}

#[test]
fn player_model_part_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::player::PlayerModelPart::VALUES;
    let fq = "net.minecraft.world.entity.player.PlayerModelPart";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "bit", &i32v(v.get_bit()));
        assert_enum_field(fq, v.name(), "mask", &i32v(v.get_mask()));
        assert_enum_field(fq, v.name(), "id", &strv(v.get_id()));
        assert_enum_field(fq, v.name(), "name", &strv(&translatable(&v.name_translation_key())));
        assert_eq!(v.get_serialized_name(), v.get_id());
    }
}

#[test]
fn player_model_part_masks_form_a_disjoint_seven_bit_field() {
    use minecraft_rust::net::minecraft::world::entity::player::PlayerModelPart::{PlayerModelPart as P, VALUES};
    let mut all = 0;
    for v in VALUES {
        assert_eq!(v.get_mask().count_ones(), 1);
        assert_eq!(all & v.get_mask(), 0, "{v} overlaps another part");
        all |= v.get_mask();
    }
    assert_eq!(all, 0x7F);
    assert_eq!(P::Cape.get_mask(), 0x01);
    assert_eq!(P::Hat.get_mask(), 0x40);
}

#[test]
fn player_model_type_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::player::PlayerModelType::VALUES;
    let fq = "net.minecraft.world.entity.player.PlayerModelType";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &strv(v.get_serialized_name()));
        assert_enum_field(fq, v.name(), "legacyServicesId", &strv(v.legacy_services_id()));
    }
}

#[test]
fn player_model_type_by_legacy_services_name() {
    use minecraft_rust::net::minecraft::world::entity::player::PlayerModelType::{by_legacy_services_name as by, PlayerModelType as T};
    assert_eq!(by(Some("slim")), T::Slim);
    assert_eq!(by(Some("default")), T::Wide);
    // "wide" is the serialized name, not the legacy one: unknown here, so the WIDE fallback.
    assert_eq!(by(Some("wide")), T::Wide);
    assert_eq!(by(Some("SLIM")), T::Wide, "case-sensitive");
    assert_eq!(by(Some("")), T::Wide);
    assert_eq!(by(None), T::Wide, "null falls back to WIDE");
}

// ============================================================================
// ContainerInput, DebugEntityBlockIntersection
// ============================================================================

#[test]
fn container_input_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::inventory::ContainerInput::{by_id, ContainerInput as C, VALUES};
    let fq = "net.minecraft.world.inventory.ContainerInput";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_eq!(by_id(v.id()), v);
    }
    for oob in [-1, 7, i32::MIN, i32::MAX] {
        assert_eq!(by_id(oob), C::Pickup);
    }
}

#[test]
fn debug_entity_block_intersection_matches_the_jar() {
    use minecraft_rust::net::minecraft::util::debug::DebugEntityBlockIntersection::{
        by_id, DebugEntityBlockIntersection as D, VALUES,
    };
    let fq = "net.minecraft.util.debug.DebugEntityBlockIntersection";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "id", &i32v(v.id()));
        assert_enum_field(fq, v.name(), "color", &i32v(v.color()));
        assert_eq!(by_id(v.id()), v);
        assert_eq!((v.color() as u32) >> 24, 0x60, "alpha");
    }
    assert_eq!(D::InBlock.color() as u32, 0x6000FF00);
    assert_eq!(D::InFluid.color() as u32, 0x600000FF);
    assert_eq!(D::InAir.color() as u32, 0x60333333);
    assert_eq!(by_id(3), D::InBlock);
}

// ============================================================================
// SlotRange
// ============================================================================

#[test]
fn slot_range_declares_no_constants_of_its_own() {
    use minecraft_rust::net::minecraft::util::StringRepresentable::PRE_BUILT_MAP_THRESHOLD;
    // The only static final in the group is the one inherited from StringRepresentable.
    assert_all_consts(
        "net.minecraft.world.inventory.SlotRange",
        &[("PRE_BUILT_MAP_THRESHOLD", i32v(PRE_BUILT_MAP_THRESHOLD))],
    );
}

#[test]
fn slot_range_of() {
    use minecraft_rust::net::minecraft::util::StringRepresentable::StringRepresentable;
    use minecraft_rust::net::minecraft::world::inventory::SlotRange::{of, SlotRange};
    let r = of("hotbar.*", vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(r.size(), 9);
    assert_eq!(r.slots(), [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(r.get_serialized_name(), "hotbar.*");
    assert_eq!(r.to_string(), "hotbar.*");

    // Through the interface, as callers hold it.
    let dyn_range: &dyn SlotRange = &r;
    assert_eq!(dyn_range.size(), 9);

    let empty = of("", Vec::new());
    assert_eq!(empty.size(), 0);
    assert_eq!(empty.to_string(), "");

    // The list is kept as given: order and duplicates are not normalised.
    let odd = of("x", vec![5, 5, -1]);
    assert_eq!(odd.slots(), [5, 5, -1]);
    assert_eq!(odd.size(), 3);
}

// ============================================================================
// Relative
// ============================================================================

#[test]
fn relative_matches_the_jar() {
    use minecraft_rust::net::minecraft::world::entity::Relative::VALUES;
    let fq = "net.minecraft.world.entity.Relative";
    assert_enum_order(fq, &VALUES.map(|v| v.name()));
    for v in VALUES {
        assert_eq!(v.ordinal(), v as i32);
        assert_eq!(v.to_string(), v.name());
        assert_enum_field(fq, v.name(), "bit", &i32v(v.bit()));
    }
}

#[test]
fn relative_wire_bit_layout_is_exact() {
    use minecraft_rust::net::minecraft::world::entity::Relative::{pack, unpack, Relative as R};
    // The teleport packets' int: bit n <-> constant. Hard-coded from the Java source.
    let layout = [
        (R::X, 0x001),
        (R::Y, 0x002),
        (R::Z, 0x004),
        (R::YRot, 0x008),
        (R::XRot, 0x010),
        (R::DeltaX, 0x020),
        (R::DeltaY, 0x040),
        (R::DeltaZ, 0x080),
        (R::RotateDelta, 0x100),
    ];
    for (r, mask) in layout {
        assert_eq!(r.get_mask(), mask, "{r}");
        assert_eq!(pack(&BTreeSet::from([r])), mask, "{r}");
        assert_eq!(unpack(mask), BTreeSet::from([r]), "{r}");
        assert!(r.is_set(mask));
        assert!(r.is_set(-1));
        assert!(!r.is_set(!mask));
    }
}

#[test]
fn relative_pack_unpack_round_trip() {
    use minecraft_rust::net::minecraft::world::entity::Relative::{pack, unpack, Relative as R, ALL, DELTA, ROTATION};
    assert_eq!(pack(&ALL), 0x1FF);
    assert_eq!(pack(&ROTATION), 0x018);
    assert_eq!(pack(&DELTA), 0x1E0);
    assert_eq!(pack(&BTreeSet::new()), 0);

    assert_eq!(unpack(0), BTreeSet::new());
    assert_eq!(unpack(-1), *ALL);
    assert_eq!(unpack(0x200), BTreeSet::new(), "bit 9 and above are ignored");
    assert_eq!(unpack(i32::MIN), BTreeSet::new());
    for v in 0..=0x3FF {
        assert_eq!(pack(&unpack(v)), v & 0x1FF, "value {v:#x}");
    }
    for v in [i32::MAX, i32::MIN, -1, 0x7FFF_FE00, 0x1234_5678] {
        assert_eq!(pack(&unpack(v)), v & 0x1FF, "value {v:#x}");
    }
    // EnumSet order: unpack iterates by ordinal, so Y_ROT comes before X_ROT.
    assert_eq!(unpack(0x018).into_iter().collect::<Vec<_>>(), [R::YRot, R::XRot]);
}

#[test]
fn relative_set_constants_and_builders() {
    use minecraft_rust::net::minecraft::world::entity::Relative::{
        direction, position, rotation, union, Relative as R, ALL, DELTA, ROTATION, VALUES,
    };
    assert_eq!(*ALL, VALUES.into_iter().collect::<BTreeSet<_>>());
    assert_eq!(*ROTATION, BTreeSet::from([R::XRot, R::YRot]));
    assert_eq!(*DELTA, BTreeSet::from([R::DeltaX, R::DeltaY, R::DeltaZ, R::RotateDelta]));

    // rotation(relativeYRot, relativeXRot): yaw first.
    assert_eq!(rotation(true, false), BTreeSet::from([R::YRot]));
    assert_eq!(rotation(false, true), BTreeSet::from([R::XRot]));
    assert_eq!(rotation(true, true), *ROTATION);
    assert_eq!(rotation(false, false), BTreeSet::new());

    assert_eq!(position(true, false, true), BTreeSet::from([R::X, R::Z]));
    assert_eq!(position(false, true, false), BTreeSet::from([R::Y]));
    assert_eq!(position(false, false, false), BTreeSet::new());

    // direction never includes ROTATE_DELTA, unlike DELTA.
    assert_eq!(direction(true, true, true), BTreeSet::from([R::DeltaX, R::DeltaY, R::DeltaZ]));
    assert_ne!(direction(true, true, true), *DELTA);
    assert_eq!(direction(false, false, true), BTreeSet::from([R::DeltaZ]));

    assert_eq!(union(&[]), BTreeSet::new());
    assert_eq!(union(&[&position(true, true, true), &ROTATION, &DELTA]), *ALL);
    assert_eq!(union(&[&ROTATION, &rotation(true, false)]), *ROTATION);
}
