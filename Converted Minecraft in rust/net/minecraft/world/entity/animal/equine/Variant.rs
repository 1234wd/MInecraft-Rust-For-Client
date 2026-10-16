//! Port of: net/minecraft/world/entity/animal/equine/Variant.java
//! Java class(es): net.minecraft.world.entity.animal.equine.Variant
//! Status: PARTIAL
//!
//! Session 19. The seven horse coat colours with their `id` and `name` fields, `getId()`,
//! `getSerializedName()` and `byId(int)` (via the ported `ByIdMap`, `WRAP`) are ported.
//! Ordinals and both fields are checked against the jar in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec`) and `STREAM_CODEC` (netty `ByteBuf`).
//!
//! `byId` WRAPs (`Math.floorMod`): `7` is `WHITE`, `-1` is `DARK_BROWN`.

/// Port of `net.minecraft.world.entity.animal.equine.Variant`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Variant {
    /// Port of `Variant#WHITE` -- ordinal 0, id `0`, name `"white"`.
    White = 0,
    /// Port of `Variant#CREAMY` -- ordinal 1, id `1`, name `"creamy"`.
    Creamy = 1,
    /// Port of `Variant#CHESTNUT` -- ordinal 2, id `2`, name `"chestnut"`.
    Chestnut = 2,
    /// Port of `Variant#BROWN` -- ordinal 3, id `3`, name `"brown"`.
    Brown = 3,
    /// Port of `Variant#BLACK` -- ordinal 4, id `4`, name `"black"`.
    Black = 4,
    /// Port of `Variant#GRAY` -- ordinal 5, id `5`, name `"gray"`.
    Gray = 5,
    /// Port of `Variant#DARK_BROWN` -- ordinal 6, id `6`, name `"dark_brown"`.
    DarkBrown = 6,
}

/// Java: `Variant.values()` -- declaration order.
pub const VALUES: [Variant; 7] = [
    Variant::White,
    Variant::Creamy,
    Variant::Chestnut,
    Variant::Brown,
    Variant::Black,
    Variant::Gray,
    Variant::DarkBrown,
];

impl Variant {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Variant::White => "WHITE",
            Variant::Creamy => "CREAMY",
            Variant::Chestnut => "CHESTNUT",
            Variant::Brown => "BROWN",
            Variant::Black => "BLACK",
            Variant::Gray => "GRAY",
            Variant::DarkBrown => "DARK_BROWN",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Variant#getId()` -- the stored `id` field.
    #[inline]
    pub const fn get_id(&self) -> i32 {
        match self {
            Variant::White => 0,
            Variant::Creamy => 1,
            Variant::Chestnut => 2,
            Variant::Brown => 3,
            Variant::Black => 4,
            Variant::Gray => 5,
            Variant::DarkBrown => 6,
        }
    }

    /// Port of `Variant#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            Variant::White => "white",
            Variant::Creamy => "creamy",
            Variant::Chestnut => "chestnut",
            Variant::Brown => "brown",
            Variant::Black => "black",
            Variant::Gray => "gray",
            Variant::DarkBrown => "dark_brown",
        }
    }
}

impl std::fmt::Display for Variant {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for Variant {
    fn get_serialized_name(&self) -> &str {
        Variant::get_serialized_name(self)
    }
}

/// Java: `private static final IntFunction<Variant> BY_ID =
/// ByIdMap.continuous(Variant::getId, values(), WRAP)`.
static BY_ID: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<Variant>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            Variant::get_id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Wrap,
        )
    });

/// Port of `Variant#byId(int)`: `WRAP`, i.e. `values[floorMod(id, 7)]`.
pub fn by_id(id: i32) -> Variant {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<Variant> CODEC = StringRepresentable.fromEnum(values)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` (DataFixerUpper).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, Variant> STREAM_CODEC =
/// ByteBufCodecs.idMapper(BY_ID, Variant::getId)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`. The id mapping it would use is [`by_id`] /
/// [`Variant::get_id`].
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
