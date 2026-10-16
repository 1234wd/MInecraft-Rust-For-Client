//! Port of: net/minecraft/world/level/pathfinder/PathType.java
//! Java class(es): net.minecraft.world.level.pathfinder.PathType
//! Status: VERIFIED
//!
//! Session 19. Twenty-seven node types, each with its default path-finding `malus` (`float`).
//! Names, ordinals and the raw `f32` bits of every `malus` are checked against the jar in
//! `_porting/tests/session19_b.rs`.
//!
//! A `malus` of `-1.0` means "impassable" to the path finder; positive values are extra cost.
//! Several constants that look dangerous are *passable* by default (`WATER` 8, `FIRE` 16,
//! `DAMAGING_IN_NEIGHBOR` 8) while `DAMAGING`, `LAVA` and `LEAVES` are `-1`.

/// Port of `net.minecraft.world.level.pathfinder.PathType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PathType {
    /// Port of `PathType#BLOCKED` -- ordinal 0, malus `-1.0`.
    Blocked = 0,
    /// Port of `PathType#OPEN` -- ordinal 1, malus `0.0`.
    Open = 1,
    /// Port of `PathType#WALKABLE` -- ordinal 2, malus `0.0`.
    Walkable = 2,
    /// Port of `PathType#WALKABLE_DOOR` -- ordinal 3, malus `0.0`.
    WalkableDoor = 3,
    /// Port of `PathType#TRAPDOOR` -- ordinal 4, malus `0.0`.
    Trapdoor = 4,
    /// Port of `PathType#POWDER_SNOW` -- ordinal 5, malus `-1.0`.
    PowderSnow = 5,
    /// Port of `PathType#ON_TOP_OF_POWDER_SNOW` -- ordinal 6, malus `0.0`.
    OnTopOfPowderSnow = 6,
    /// Port of `PathType#FENCE` -- ordinal 7, malus `-1.0`.
    Fence = 7,
    /// Port of `PathType#LAVA` -- ordinal 8, malus `-1.0`.
    Lava = 8,
    /// Port of `PathType#WATER` -- ordinal 9, malus `8.0`.
    Water = 9,
    /// Port of `PathType#WATER_BORDER` -- ordinal 10, malus `8.0`.
    WaterBorder = 10,
    /// Port of `PathType#RAIL` -- ordinal 11, malus `0.0`.
    Rail = 11,
    /// Port of `PathType#UNPASSABLE_RAIL` -- ordinal 12, malus `-1.0`.
    UnpassableRail = 12,
    /// Port of `PathType#FIRE_IN_NEIGHBOR` -- ordinal 13, malus `8.0`.
    FireInNeighbor = 13,
    /// Port of `PathType#FIRE` -- ordinal 14, malus `16.0`.
    Fire = 14,
    /// Port of `PathType#DAMAGING_IN_NEIGHBOR` -- ordinal 15, malus `8.0`.
    DamagingInNeighbor = 15,
    /// Port of `PathType#DAMAGING` -- ordinal 16, malus `-1.0`.
    Damaging = 16,
    /// Port of `PathType#DOOR_OPEN` -- ordinal 17, malus `0.0`.
    DoorOpen = 17,
    /// Port of `PathType#DOOR_WOOD_CLOSED` -- ordinal 18, malus `-1.0`.
    DoorWoodClosed = 18,
    /// Port of `PathType#DOOR_IRON_CLOSED` -- ordinal 19, malus `-1.0`.
    DoorIronClosed = 19,
    /// Port of `PathType#BREACH` -- ordinal 20, malus `4.0`.
    Breach = 20,
    /// Port of `PathType#LEAVES` -- ordinal 21, malus `-1.0`.
    Leaves = 21,
    /// Port of `PathType#STICKY_HONEY` -- ordinal 22, malus `8.0`.
    StickyHoney = 22,
    /// Port of `PathType#COCOA` -- ordinal 23, malus `0.0`.
    Cocoa = 23,
    /// Port of `PathType#DAMAGE_CAUTIOUS` -- ordinal 24, malus `0.0`.
    DamageCautious = 24,
    /// Port of `PathType#ON_TOP_OF_TRAPDOOR` -- ordinal 25, malus `0.0`.
    OnTopOfTrapdoor = 25,
    /// Port of `PathType#BIG_MOBS_CLOSE_TO_DANGER` -- ordinal 26, malus `4.0`.
    BigMobsCloseToDanger = 26,
}

/// Java: `PathType.values()` -- declaration order.
pub const VALUES: [PathType; 27] = [
    PathType::Blocked,
    PathType::Open,
    PathType::Walkable,
    PathType::WalkableDoor,
    PathType::Trapdoor,
    PathType::PowderSnow,
    PathType::OnTopOfPowderSnow,
    PathType::Fence,
    PathType::Lava,
    PathType::Water,
    PathType::WaterBorder,
    PathType::Rail,
    PathType::UnpassableRail,
    PathType::FireInNeighbor,
    PathType::Fire,
    PathType::DamagingInNeighbor,
    PathType::Damaging,
    PathType::DoorOpen,
    PathType::DoorWoodClosed,
    PathType::DoorIronClosed,
    PathType::Breach,
    PathType::Leaves,
    PathType::StickyHoney,
    PathType::Cocoa,
    PathType::DamageCautious,
    PathType::OnTopOfTrapdoor,
    PathType::BigMobsCloseToDanger,
];

impl PathType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PathType::Blocked => "BLOCKED",
            PathType::Open => "OPEN",
            PathType::Walkable => "WALKABLE",
            PathType::WalkableDoor => "WALKABLE_DOOR",
            PathType::Trapdoor => "TRAPDOOR",
            PathType::PowderSnow => "POWDER_SNOW",
            PathType::OnTopOfPowderSnow => "ON_TOP_OF_POWDER_SNOW",
            PathType::Fence => "FENCE",
            PathType::Lava => "LAVA",
            PathType::Water => "WATER",
            PathType::WaterBorder => "WATER_BORDER",
            PathType::Rail => "RAIL",
            PathType::UnpassableRail => "UNPASSABLE_RAIL",
            PathType::FireInNeighbor => "FIRE_IN_NEIGHBOR",
            PathType::Fire => "FIRE",
            PathType::DamagingInNeighbor => "DAMAGING_IN_NEIGHBOR",
            PathType::Damaging => "DAMAGING",
            PathType::DoorOpen => "DOOR_OPEN",
            PathType::DoorWoodClosed => "DOOR_WOOD_CLOSED",
            PathType::DoorIronClosed => "DOOR_IRON_CLOSED",
            PathType::Breach => "BREACH",
            PathType::Leaves => "LEAVES",
            PathType::StickyHoney => "STICKY_HONEY",
            PathType::Cocoa => "COCOA",
            PathType::DamageCautious => "DAMAGE_CAUTIOUS",
            PathType::OnTopOfTrapdoor => "ON_TOP_OF_TRAPDOOR",
            PathType::BigMobsCloseToDanger => "BIG_MOBS_CLOSE_TO_DANGER",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `PathType#getMalus()` -- the `malus` field (constructor parameter `defaultCost`).
    pub const fn get_malus(&self) -> f32 {
        match self {
            PathType::Blocked => -1.0,
            PathType::Open => 0.0,
            PathType::Walkable => 0.0,
            PathType::WalkableDoor => 0.0,
            PathType::Trapdoor => 0.0,
            PathType::PowderSnow => -1.0,
            PathType::OnTopOfPowderSnow => 0.0,
            PathType::Fence => -1.0,
            PathType::Lava => -1.0,
            PathType::Water => 8.0,
            PathType::WaterBorder => 8.0,
            PathType::Rail => 0.0,
            PathType::UnpassableRail => -1.0,
            PathType::FireInNeighbor => 8.0,
            PathType::Fire => 16.0,
            PathType::DamagingInNeighbor => 8.0,
            PathType::Damaging => -1.0,
            PathType::DoorOpen => 0.0,
            PathType::DoorWoodClosed => -1.0,
            PathType::DoorIronClosed => -1.0,
            PathType::Breach => 4.0,
            PathType::Leaves => -1.0,
            PathType::StickyHoney => 8.0,
            PathType::Cocoa => 0.0,
            PathType::DamageCautious => 0.0,
            PathType::OnTopOfTrapdoor => 0.0,
            PathType::BigMobsCloseToDanger => 4.0,
        }
    }
}

impl std::fmt::Display for PathType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
