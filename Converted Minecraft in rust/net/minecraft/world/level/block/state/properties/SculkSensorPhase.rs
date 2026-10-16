//! Port of: net/minecraft/world/level/block/state/properties/SculkSensorPhase.java
//! Java class(es): net.minecraft.world.level.block.state.properties.SculkSensorPhase
//! Status: VERIFIED
//!
//! Session 19. Three constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to return the `name` field, so `Display` prints `inactive`.

/// Port of `net.minecraft.world.level.block.state.properties.SculkSensorPhase`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SculkSensorPhase {
    /// `INACTIVE` -- ordinal 0, `"inactive"`.
    Inactive = 0,
    /// `ACTIVE` -- ordinal 1, `"active"`.
    Active = 1,
    /// `COOLDOWN` -- ordinal 2, `"cooldown"`.
    Cooldown = 2,
}

/// Java: `SculkSensorPhase.values()`, declaration order.
pub const VALUES: [SculkSensorPhase; 3] = [
    SculkSensorPhase::Inactive,
    SculkSensorPhase::Active,
    SculkSensorPhase::Cooldown,
];

impl SculkSensorPhase {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            SculkSensorPhase::Inactive => "INACTIVE",
            SculkSensorPhase::Active => "ACTIVE",
            SculkSensorPhase::Cooldown => "COOLDOWN",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `SculkSensorPhase#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            SculkSensorPhase::Inactive => "inactive",
            SculkSensorPhase::Active => "active",
            SculkSensorPhase::Cooldown => "cooldown",
        }
    }
}

impl std::fmt::Display for SculkSensorPhase {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for SculkSensorPhase {
    fn get_serialized_name(&self) -> &str {
        SculkSensorPhase::get_serialized_name(self)
    }
}
