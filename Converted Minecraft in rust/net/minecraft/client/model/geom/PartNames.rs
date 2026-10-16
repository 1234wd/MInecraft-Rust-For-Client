//! Port of: net/minecraft/client/model/geom/PartNames.java
//! Java class(es): net.minecraft.client.model.geom.PartNames
//! Status: VERIFIED
//!
//! Session 19 batch h. The 106 `public static final String` model-part names as free `pub const`s
//! (all checked against the jar's dump in `_porting/tests/session19_h.rs`), plus `tentacle(int)`.
//! Nothing is PORT-BLOCKED.
//!
//! Quirk worth pinning: `BLUE_CORAL_FIRST` is `"blue_first"` and `BLUE_CORAL_SECOND` is
//! `"blue_second"` -- unlike the yellow/red siblings, the `_coral` infix is missing.

pub const LEFT_FIN: &str = "left_fin";
pub const RIGHT_FIN: &str = "right_fin";
pub const TOP_FIN: &str = "top_fin";
pub const BOTTOM_FIN: &str = "bottom_fin";
pub const TAIL_FIN: &str = "tail_fin";
pub const LEFT_BLUE_FIN: &str = "left_blue_fin";
pub const RIGHT_BLUE_FIN: &str = "right_blue_fin";
pub const LEFT_ARM: &str = "left_arm";
pub const RIGHT_ARM: &str = "right_arm";
pub const LEFT_WING: &str = "left_wing";
pub const RIGHT_WING: &str = "right_wing";
pub const LEFT_WING_BASE: &str = "left_wing_base";
pub const RIGHT_WING_BASE: &str = "right_wing_base";
pub const LEFT_WING_TIP: &str = "left_wing_tip";
pub const RIGHT_WING_TIP: &str = "right_wing_tip";
pub const LEFT_EAR: &str = "left_ear";
pub const RIGHT_EAR: &str = "right_ear";
pub const LEFT_LEG: &str = "left_leg";
pub const RIGHT_LEG: &str = "right_leg";
pub const LEFT_HIND_LEG: &str = "left_hind_leg";
pub const RIGHT_HIND_LEG: &str = "right_hind_leg";
pub const LEFT_MID_LEG: &str = "left_mid_leg";
pub const RIGHT_MID_LEG: &str = "right_mid_leg";
pub const LEFT_FRONT_LEG: &str = "left_front_leg";
pub const RIGHT_FRONT_LEG: &str = "right_front_leg";
pub const LEFT_HIND_FOOT: &str = "left_hind_foot";
pub const RIGHT_HIND_FOOT: &str = "right_hind_foot";
pub const LEFT_FRONT_FOOT: &str = "left_front_foot";
pub const RIGHT_FRONT_FOOT: &str = "right_front_foot";
pub const FEET: &str = "feet";
pub const LEFT_HIND_LEG_TIP: &str = "left_hind_leg_tip";
pub const RIGHT_HIND_LEG_TIP: &str = "right_hind_leg_tip";
pub const LEFT_FRONT_LEG_TIP: &str = "left_front_leg_tip";
pub const RIGHT_FRONT_LEG_TIP: &str = "right_front_leg_tip";
pub const LEFT_LID: &str = "left_lid";
pub const RIGHT_LID: &str = "right_lid";
pub const LEFT_CHEST: &str = "left_chest";
pub const RIGHT_CHEST: &str = "right_chest";
pub const LEFT_HORN: &str = "left_horn";
pub const RIGHT_HORN: &str = "right_horn";
pub const LEFT_EYE: &str = "left_eye";
pub const RIGHT_EYE: &str = "right_eye";
pub const JAW: &str = "jaw";
pub const NOSE: &str = "nose";
pub const ARMS: &str = "arms";
pub const TAIL: &str = "tail";
pub const CUBE: &str = "cube";
pub const BEAK: &str = "beak";
pub const BACK_FIN: &str = "back_fin";
pub const MANE: &str = "mane";
pub const NECK: &str = "neck";
pub const MOUTH: &str = "mouth";
pub const UPPER_MOUTH: &str = "upper_mouth";
pub const INNER_MOUTH: &str = "inner_mouth";
pub const LOWER_MOUTH: &str = "lower_mouth";
pub const HEAD: &str = "head";
pub const HAT: &str = "hat";
pub const BODY: &str = "body";
pub const UPPER_BODY: &str = "upper_body";
pub const INNER_BODY: &str = "inner_body";
pub const WAIST: &str = "waist";
pub const HAT_RIM: &str = "hat_rim";
pub const GOGGLES: &str = "goggles";
pub const JACKET: &str = "jacket";
pub const HARNESS: &str = "harness";
pub const SHELL: &str = "shell";
pub const TOP_GILLS: &str = "top_gills";
pub const LEFT_GILLS: &str = "left_gills";
pub const RIGHT_GILLS: &str = "right_gills";
pub const ROOT: &str = "root";
pub const CROAKING_BODY: &str = "croaking_body";
pub const TONGUE: &str = "tongue";
pub const TONGUE_R1: &str = "tongue_r1";
pub const LEFT_HAND: &str = "left_hand";
pub const RIGHT_HAND: &str = "right_hand";
pub const LEFT_FOOT: &str = "left_foot";
pub const RIGHT_FOOT: &str = "right_foot";
pub const EYES: &str = "eyes";
pub const RIGHT_TENDRIL: &str = "right_tendril";
pub const LEFT_TENDRIL: &str = "left_tendril";
pub const RIGHT_RIBCAGE: &str = "right_ribcage";
pub const LEFT_RIBCAGE: &str = "left_ribcage";
pub const WIND_BODY: &str = "wind_body";
pub const WIND_TOP: &str = "wind_top";
pub const WIND_MID: &str = "wind_mid";
pub const WIND_BOTTOM: &str = "wind_bottom";
pub const RODS: &str = "rods";
pub const MUSHROOMS: &str = "mushrooms";
pub const BONE: &str = "bone";
pub const LEFT_PADDLE: &str = "left_paddle";
pub const RIGHT_PADDLE: &str = "right_paddle";
pub const BOTTOM: &str = "bottom";
pub const CHEST_BOTTOM: &str = "chest_bottom";
pub const CHEST_LID: &str = "chest_lid";
pub const CHEST_LOCK: &str = "chest_lock";
pub const YELLOW_CORAL: &str = "yellow_coral";
pub const YELLOW_CORAL_FIRST: &str = "yellow_coral_first";
pub const YELLOW_CORAL_SECOND: &str = "yellow_coral_second";
pub const PINK_CORAL: &str = "pink_coral";
pub const PINK_CORAL_SECOND: &str = "pink_coral_second";
pub const BLUE_CORAL: &str = "blue_coral";
pub const BLUE_CORAL_FIRST: &str = "blue_first";
pub const BLUE_CORAL_SECOND: &str = "blue_second";
pub const RED_CORAL: &str = "red_coral";
pub const RED_CORAL_FIRST: &str = "red_coral_first";
pub const RED_CORAL_SECOND: &str = "red_coral_second";

/// Port of `PartNames#tentacle(int)`: `"tentacle" + i` (Java `Integer.toString`, so negatives
/// keep their `-`).
pub fn tentacle(i: i32) -> String {
    format!("tentacle{i}")
}
