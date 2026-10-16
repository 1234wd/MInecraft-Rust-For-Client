//! Port of: net/minecraft/world/level/block/state/properties/CreakingHeartState.java
//! Java class(es): net.minecraft.world.level.block.state.properties.CreakingHeartState
//! Status: VERIFIED
//!
//! Session 19. Three constants carrying a `String name`. Every member ported; ordinals and the
//! `name` field of every constant are checked against the jar (`session19_a.rs`).
//!
//! `toString()` is overridden to return the `name` field, so `Display` prints `uprooted`.

/// Port of `net.minecraft.world.level.block.state.properties.CreakingHeartState`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CreakingHeartState {
    /// `UPROOTED` -- ordinal 0, `"uprooted"`.
    Uprooted = 0,
    /// `DORMANT` -- ordinal 1, `"dormant"`.
    Dormant = 1,
    /// `AWAKE` -- ordinal 2, `"awake"`.
    Awake = 2,
}

/// Java: `CreakingHeartState.values()`, declaration order.
pub const VALUES: [CreakingHeartState; 3] = [
    CreakingHeartState::Uprooted,
    CreakingHeartState::Dormant,
    CreakingHeartState::Awake,
];

impl CreakingHeartState {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            CreakingHeartState::Uprooted => "UPROOTED",
            CreakingHeartState::Dormant => "DORMANT",
            CreakingHeartState::Awake => "AWAKE",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `CreakingHeartState#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            CreakingHeartState::Uprooted => "uprooted",
            CreakingHeartState::Dormant => "dormant",
            CreakingHeartState::Awake => "awake",
        }
    }
}

impl std::fmt::Display for CreakingHeartState {
    /// Port of the `toString()` override: the `name` field.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for CreakingHeartState {
    fn get_serialized_name(&self) -> &str {
        CreakingHeartState::get_serialized_name(self)
    }
}
