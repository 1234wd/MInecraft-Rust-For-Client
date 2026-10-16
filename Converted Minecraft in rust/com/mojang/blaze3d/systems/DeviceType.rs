//! Port of: com/mojang/blaze3d/systems/DeviceType.java
//! Java class(es): com.mojang.blaze3d.systems.DeviceType
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.

/// Port of `com.mojang.blaze3d.systems.DeviceType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceType {
    /// `OTHER` -- ordinal 0.
    Other = 0,
    /// `INTEGRATED` -- ordinal 1.
    Integrated = 1,
    /// `DISCRETE` -- ordinal 2.
    Discrete = 2,
    /// `VIRTUAL` -- ordinal 3.
    Virtual = 3,
    /// `CPU` -- ordinal 4.
    Cpu = 4,
}

/// Java: `DeviceType.values()`, declaration order.
pub const VALUES: [DeviceType; 5] = [
    DeviceType::Other,
    DeviceType::Integrated,
    DeviceType::Discrete,
    DeviceType::Virtual,
    DeviceType::Cpu,
];

impl DeviceType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            DeviceType::Other => "OTHER",
            DeviceType::Integrated => "INTEGRATED",
            DeviceType::Discrete => "DISCRETE",
            DeviceType::Virtual => "VIRTUAL",
            DeviceType::Cpu => "CPU",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for DeviceType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
