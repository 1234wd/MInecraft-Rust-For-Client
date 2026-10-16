//! Port of: net/minecraft/client/CameraType.java
//! Java class(es): net.minecraft.client.CameraType
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`. Both boolean fields are checked too.
//!
//! `cycle()` is `VALUES[(ordinal + 1) % 3]`: first person -> third person back -> third person front
//! -> first person, which is the F5 order.

/// Port of `net.minecraft.client.CameraType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CameraType {
    /// `FIRST_PERSON` -- ordinal 0.
    FirstPerson = 0,
    /// `THIRD_PERSON_BACK` -- ordinal 1.
    ThirdPersonBack = 1,
    /// `THIRD_PERSON_FRONT` -- ordinal 2.
    ThirdPersonFront = 2,
}

/// Java: `CameraType.values()`, declaration order.
pub const VALUES: [CameraType; 3] = [
    CameraType::FirstPerson,
    CameraType::ThirdPersonBack,
    CameraType::ThirdPersonFront,
];

impl CameraType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            CameraType::FirstPerson => "FIRST_PERSON",
            CameraType::ThirdPersonBack => "THIRD_PERSON_BACK",
            CameraType::ThirdPersonFront => "THIRD_PERSON_FRONT",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `CameraType#isFirstPerson()`.
    pub const fn is_first_person(&self) -> bool {
        match self {
            CameraType::FirstPerson => true,
            CameraType::ThirdPersonBack => false,
            CameraType::ThirdPersonFront => false,
        }
    }

    /// Port of `CameraType#isMirrored()`.
    pub const fn is_mirrored(&self) -> bool {
        match self {
            CameraType::FirstPerson => false,
            CameraType::ThirdPersonBack => false,
            CameraType::ThirdPersonFront => true,
        }
    }

    /// Port of `CameraType#cycle()`.
    pub const fn cycle(&self) -> CameraType {
        VALUES[((self.ordinal() + 1) % VALUES.len() as i32) as usize]
    }
}

impl std::fmt::Display for CameraType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
