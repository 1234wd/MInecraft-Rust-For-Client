//! Port of: net/minecraft/world/item/ItemDisplayContext.java
//! Java class(es): net.minecraft.world.item.ItemDisplayContext
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the ten constants with `id` and `name`, `BY_ID`, `getId`,
//! `getSerializedName`, `firstPerson`, `leftHand`. Constants and fields are checked against the
//! jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `CODEC` on DataFixerUpper.
//!
//! # `id` IS A `byte`
//!
//! The constructor stores `(byte) id` and `getId()` returns `byte`, so the port returns `i8`.
//! `BY_ID` is built from `ItemDisplayContext::getId`, i.e. the byte widened to `int`. Every id
//! is `0..=9`, so the narrowing never changes a value.
//!
//! # `leftHand` IS FALSE FOR `HEAD`, `GUI`, ...
//!
//! Only the two explicit left-hand contexts; `NONE` and the non-hand contexts are "not left".

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.item.ItemDisplayContext`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemDisplayContext {
    /// Port of `ItemDisplayContext#NONE` -- ordinal 0, id 0, `"none"`.
    None = 0,
    /// Port of `ItemDisplayContext#THIRD_PERSON_LEFT_HAND` -- ordinal 1, id 1, `"thirdperson_lefthand"`.
    ThirdPersonLeftHand = 1,
    /// Port of `ItemDisplayContext#THIRD_PERSON_RIGHT_HAND` -- ordinal 2, id 2, `"thirdperson_righthand"`.
    ThirdPersonRightHand = 2,
    /// Port of `ItemDisplayContext#FIRST_PERSON_LEFT_HAND` -- ordinal 3, id 3, `"firstperson_lefthand"`.
    FirstPersonLeftHand = 3,
    /// Port of `ItemDisplayContext#FIRST_PERSON_RIGHT_HAND` -- ordinal 4, id 4, `"firstperson_righthand"`.
    FirstPersonRightHand = 4,
    /// Port of `ItemDisplayContext#HEAD` -- ordinal 5, id 5, `"head"`.
    Head = 5,
    /// Port of `ItemDisplayContext#GUI` -- ordinal 6, id 6, `"gui"`.
    Gui = 6,
    /// Port of `ItemDisplayContext#GROUND` -- ordinal 7, id 7, `"ground"`.
    Ground = 7,
    /// Port of `ItemDisplayContext#FIXED` -- ordinal 8, id 8, `"fixed"`.
    Fixed = 8,
    /// Port of `ItemDisplayContext#ON_SHELF` -- ordinal 9, id 9, `"on_shelf"`.
    OnShelf = 9,
}

/// Java: `ItemDisplayContext.values()` -- declaration order.
pub const VALUES: [ItemDisplayContext; 10] = [
    ItemDisplayContext::None,
    ItemDisplayContext::ThirdPersonLeftHand,
    ItemDisplayContext::ThirdPersonRightHand,
    ItemDisplayContext::FirstPersonLeftHand,
    ItemDisplayContext::FirstPersonRightHand,
    ItemDisplayContext::Head,
    ItemDisplayContext::Gui,
    ItemDisplayContext::Ground,
    ItemDisplayContext::Fixed,
    ItemDisplayContext::OnShelf,
];

impl ItemDisplayContext {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ItemDisplayContext::None => "NONE",
            ItemDisplayContext::ThirdPersonLeftHand => "THIRD_PERSON_LEFT_HAND",
            ItemDisplayContext::ThirdPersonRightHand => "THIRD_PERSON_RIGHT_HAND",
            ItemDisplayContext::FirstPersonLeftHand => "FIRST_PERSON_LEFT_HAND",
            ItemDisplayContext::FirstPersonRightHand => "FIRST_PERSON_RIGHT_HAND",
            ItemDisplayContext::Head => "HEAD",
            ItemDisplayContext::Gui => "GUI",
            ItemDisplayContext::Ground => "GROUND",
            ItemDisplayContext::Fixed => "FIXED",
            ItemDisplayContext::OnShelf => "ON_SHELF",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ItemDisplayContext#getId()` -- a Java `byte`.
    pub const fn get_id(&self) -> i8 {
        match self {
            ItemDisplayContext::None => 0,
            ItemDisplayContext::ThirdPersonLeftHand => 1,
            ItemDisplayContext::ThirdPersonRightHand => 2,
            ItemDisplayContext::FirstPersonLeftHand => 3,
            ItemDisplayContext::FirstPersonRightHand => 4,
            ItemDisplayContext::Head => 5,
            ItemDisplayContext::Gui => 6,
            ItemDisplayContext::Ground => 7,
            ItemDisplayContext::Fixed => 8,
            ItemDisplayContext::OnShelf => 9,
        }
    }

    /// Port of `ItemDisplayContext#getSerializedName()` -- the `name` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            ItemDisplayContext::None => "none",
            ItemDisplayContext::ThirdPersonLeftHand => "thirdperson_lefthand",
            ItemDisplayContext::ThirdPersonRightHand => "thirdperson_righthand",
            ItemDisplayContext::FirstPersonLeftHand => "firstperson_lefthand",
            ItemDisplayContext::FirstPersonRightHand => "firstperson_righthand",
            ItemDisplayContext::Head => "head",
            ItemDisplayContext::Gui => "gui",
            ItemDisplayContext::Ground => "ground",
            ItemDisplayContext::Fixed => "fixed",
            ItemDisplayContext::OnShelf => "on_shelf",
        }
    }

    /// Port of `ItemDisplayContext#firstPerson()`: `FIRST_PERSON_LEFT_HAND || FIRST_PERSON_RIGHT_HAND`.
    pub const fn first_person(&self) -> bool {
        matches!(self, ItemDisplayContext::FirstPersonLeftHand | ItemDisplayContext::FirstPersonRightHand)
    }

    /// Port of `ItemDisplayContext#leftHand()`: `FIRST_PERSON_LEFT_HAND || THIRD_PERSON_LEFT_HAND`.
    pub const fn left_hand(&self) -> bool {
        matches!(self, ItemDisplayContext::FirstPersonLeftHand | ItemDisplayContext::ThirdPersonLeftHand)
    }
}

impl std::fmt::Display for ItemDisplayContext {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for ItemDisplayContext {
    fn get_serialized_name(&self) -> &str {
        ItemDisplayContext::get_serialized_name(self)
    }
}

static BY_ID_MAP: std::sync::LazyLock<ByIdMap::Continuous<ItemDisplayContext>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(|c: &ItemDisplayContext| c.get_id() as i32, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Java: `public static final IntFunction<ItemDisplayContext> BY_ID`, applied.
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..10` yields `NONE`.
pub fn BY_ID(id: i32) -> ItemDisplayContext {
    BY_ID_MAP.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<ItemDisplayContext> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
