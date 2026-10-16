//! Port of: net/minecraft/world/item/crafting/CraftingBookCategory.java
//! Java class(es): net.minecraft.world.item.crafting.CraftingBookCategory
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the four constants with `name` and `id`, `BY_ID`,
//! `getSerializedName`, and the private `id()` accessor. Constants and fields are checked
//! against the jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `CODEC` on DataFixerUpper, `STREAM_CODEC` on netty `ByteBuf`.
//!
//! Unlike most id enums here, the constructor takes `(name, id)` -- name first.

use crate::net::minecraft::util::ByIdMap;
use crate::net::minecraft::util::StringRepresentable;

/// Port of `net.minecraft.world.item.crafting.CraftingBookCategory`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CraftingBookCategory {
    /// Port of `CraftingBookCategory#BUILDING` -- ordinal 0, id 0, `"building"`.
    Building = 0,
    /// Port of `CraftingBookCategory#REDSTONE` -- ordinal 1, id 1, `"redstone"`.
    Redstone = 1,
    /// Port of `CraftingBookCategory#EQUIPMENT` -- ordinal 2, id 2, `"equipment"`.
    Equipment = 2,
    /// Port of `CraftingBookCategory#MISC` -- ordinal 3, id 3, `"misc"`.
    Misc = 3,
}

/// Java: `CraftingBookCategory.values()` -- declaration order.
pub const VALUES: [CraftingBookCategory; 4] = [
    CraftingBookCategory::Building,
    CraftingBookCategory::Redstone,
    CraftingBookCategory::Equipment,
    CraftingBookCategory::Misc,
];

impl CraftingBookCategory {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            CraftingBookCategory::Building => "BUILDING",
            CraftingBookCategory::Redstone => "REDSTONE",
            CraftingBookCategory::Equipment => "EQUIPMENT",
            CraftingBookCategory::Misc => "MISC",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `CraftingBookCategory#id()`.
    pub const fn id(&self) -> i32 {
        match self {
            CraftingBookCategory::Building => 0,
            CraftingBookCategory::Redstone => 1,
            CraftingBookCategory::Equipment => 2,
            CraftingBookCategory::Misc => 3,
        }
    }

    /// Port of `CraftingBookCategory#getSerializedName()` -- the `name` field.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            CraftingBookCategory::Building => "building",
            CraftingBookCategory::Redstone => "redstone",
            CraftingBookCategory::Equipment => "equipment",
            CraftingBookCategory::Misc => "misc",
        }
    }
}

impl std::fmt::Display for CraftingBookCategory {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl StringRepresentable::StringRepresentable for CraftingBookCategory {
    fn get_serialized_name(&self) -> &str {
        CraftingBookCategory::get_serialized_name(self)
    }
}

static BY_ID_MAP: std::sync::LazyLock<ByIdMap::Continuous<CraftingBookCategory>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(CraftingBookCategory::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Zero)
});

/// Java: `public static final IntFunction<CraftingBookCategory> BY_ID`, applied.
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..4` yields `BUILDING`.
pub fn BY_ID(id: i32) -> CraftingBookCategory {
    BY_ID_MAP.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<CraftingBookCategory> CODEC = StringRepresentable.fromEnum(...)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, CraftingBookCategory> STREAM_CODEC` --
/// `ByteBufCodecs.idMapper(BY_ID, CraftingBookCategory::id)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
