//! Port of: net/minecraft/world/level/block/state/properties/SpeleothemThickness.java
//! Java class(es): net.minecraft.world.level.block.state.properties.SpeleothemThickness
//! Status: VERIFIED
//!
//! Session 19. Five constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to return the `name` field, so `Display` prints `tip_merge`.

/// Port of `net.minecraft.world.level.block.state.properties.SpeleothemThickness`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpeleothemThickness {
    /// `TIP_MERGE` -- ordinal 0, `"tip_merge"`.
    TipMerge = 0,
    /// `TIP` -- ordinal 1, `"tip"`.
    Tip = 1,
    /// `FRUSTUM` -- ordinal 2, `"frustum"`.
    Frustum = 2,
    /// `MIDDLE` -- ordinal 3, `"middle"`.
    Middle = 3,
    /// `BASE` -- ordinal 4, `"base"`.
    Base = 4,
}

/// Java: `SpeleothemThickness.values()`, declaration order.
pub const VALUES: [SpeleothemThickness; 5] = [
    SpeleothemThickness::TipMerge,
    SpeleothemThickness::Tip,
    SpeleothemThickness::Frustum,
    SpeleothemThickness::Middle,
    SpeleothemThickness::Base,
];

impl SpeleothemThickness {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            SpeleothemThickness::TipMerge => "TIP_MERGE",
            SpeleothemThickness::Tip => "TIP",
            SpeleothemThickness::Frustum => "FRUSTUM",
            SpeleothemThickness::Middle => "MIDDLE",
            SpeleothemThickness::Base => "BASE",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `SpeleothemThickness#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            SpeleothemThickness::TipMerge => "tip_merge",
            SpeleothemThickness::Tip => "tip",
            SpeleothemThickness::Frustum => "frustum",
            SpeleothemThickness::Middle => "middle",
            SpeleothemThickness::Base => "base",
        }
    }
}

impl std::fmt::Display for SpeleothemThickness {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for SpeleothemThickness {
    fn get_serialized_name(&self) -> &str {
        SpeleothemThickness::get_serialized_name(self)
    }
}
