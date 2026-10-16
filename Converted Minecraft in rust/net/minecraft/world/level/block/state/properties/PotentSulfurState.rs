//! Port of: net/minecraft/world/level/block/state/properties/PotentSulfurState.java
//! Java class(es): net.minecraft.world.level.block.state.properties.PotentSulfurState
//! Status: VERIFIED
//!
//! Session 19. Five constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to return the `name` field, so `Display` prints `dry`.

/// Port of `net.minecraft.world.level.block.state.properties.PotentSulfurState`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PotentSulfurState {
    /// `DRY` -- ordinal 0, `"dry"`.
    Dry = 0,
    /// `WET` -- ordinal 1, `"wet"`.
    Wet = 1,
    /// `DORMANT` -- ordinal 2, `"dormant"`.
    Dormant = 2,
    /// `ERUPTING` -- ordinal 3, `"erupting"`.
    Erupting = 3,
    /// `CONTINUOUS` -- ordinal 4, `"continuous"`.
    Continuous = 4,
}

/// Java: `PotentSulfurState.values()`, declaration order.
pub const VALUES: [PotentSulfurState; 5] = [
    PotentSulfurState::Dry,
    PotentSulfurState::Wet,
    PotentSulfurState::Dormant,
    PotentSulfurState::Erupting,
    PotentSulfurState::Continuous,
];

impl PotentSulfurState {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            PotentSulfurState::Dry => "DRY",
            PotentSulfurState::Wet => "WET",
            PotentSulfurState::Dormant => "DORMANT",
            PotentSulfurState::Erupting => "ERUPTING",
            PotentSulfurState::Continuous => "CONTINUOUS",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `PotentSulfurState#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            PotentSulfurState::Dry => "dry",
            PotentSulfurState::Wet => "wet",
            PotentSulfurState::Dormant => "dormant",
            PotentSulfurState::Erupting => "erupting",
            PotentSulfurState::Continuous => "continuous",
        }
    }
}

impl std::fmt::Display for PotentSulfurState {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for PotentSulfurState {
    fn get_serialized_name(&self) -> &str {
        PotentSulfurState::get_serialized_name(self)
    }
}
