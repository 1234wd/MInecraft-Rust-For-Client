//! Port of: net/minecraft/world/level/block/state/properties/BedPart.java
//! Java class(es): net.minecraft.world.level.block.state.properties.BedPart
//! Status: PARTIAL
//!
//! Session 19. Two constants carrying a `String name`. Ordinals and the `name` field of every
//! constant are checked against the jar (`session19_a.rs`). `toString()` is overridden to the
//! `name` field, so `Display` prints `head`.
//!
//! PORT-BLOCKED: `CODEC` (`StringRepresentable.fromEnum(BedPart::values)`) needs DFU's
//! `com.mojang.serialization.Codec`. The field is declared as a plain `Codec<BedPart>`, not an
//! `EnumCodec`, so Java callers cannot reach `byName` through it; no `by_name` is exposed here.

/// Port of `net.minecraft.world.level.block.state.properties.BedPart`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BedPart {
    /// `HEAD` -- ordinal 0, `"head"`.
    Head = 0,
    /// `FOOT` -- ordinal 1, `"foot"`.
    Foot = 1,
}

/// Java: `BedPart.values()`, declaration order.
pub const VALUES: [BedPart; 2] = [BedPart::Head, BedPart::Foot];

impl BedPart {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            BedPart::Head => "HEAD",
            BedPart::Foot => "FOOT",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `BedPart#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            BedPart::Head => "head",
            BedPart::Foot => "foot",
        }
    }
}

impl std::fmt::Display for BedPart {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for BedPart {
    fn get_serialized_name(&self) -> &str {
        BedPart::get_serialized_name(self)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<BedPart> CODEC = StringRepresentable.fromEnum(BedPart::values)`.
///
/// `com.mojang.serialization.Codec` is not ported. See DESIGN_DECISIONS.md (#dfu-proposal).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
