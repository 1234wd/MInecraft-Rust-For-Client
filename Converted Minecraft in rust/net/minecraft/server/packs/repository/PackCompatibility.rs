//! Port of: net/minecraft/server/packs/repository/PackCompatibility.java
//! Java class(es): net.minecraft.server.packs.repository.PackCompatibility
//! Status: PARTIAL
//!
//! Session 19. Ported: the four constants, `UNKNOWN_VERSION` and `isCompatible`. Names and
//! ordinals are checked against the jar in `_porting/tests/session19_b.rs` (the dump does not
//! record static finals of enums, so `UNKNOWN_VERSION` is tested from the source).
//!
//! PORT-BLOCKED:
//! - `getDescription` / `getConfirmation`: `net.minecraft.network.chat.Component` is not
//!   ported. The jar's values are `Component.translatable("pack.incompatible." + key)` styled
//!   `GRAY`, and `Component.translatable("pack.incompatible.confirm." + key)`, with `key`
//!   `old`, `new`, `unknown`, `compatible` respectively (note: `TOO_OLD` -> `old`).
//! - `forVersion(InclusiveRange<PackFormat>, PackFormat)`: `net.minecraft.util.InclusiveRange`
//!   and `net.minecraft.server.packs.metadata.pack.PackFormat` are SKELETONs. Logic, for the
//!   porter: `UNKNOWN` if the range's min major is `Integer.MAX_VALUE`; else `TOO_OLD` if
//!   `max < game`; else `TOO_NEW` if `game < min`; else `COMPATIBLE`.

/// Java: `public static final int UNKNOWN_VERSION = Integer.MAX_VALUE`.
pub const UNKNOWN_VERSION: i32 = i32::MAX;

/// Port of `net.minecraft.server.packs.repository.PackCompatibility`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PackCompatibility {
    /// Port of `PackCompatibility#TOO_OLD` -- ordinal 0, key `"old"`.
    TooOld = 0,
    /// Port of `PackCompatibility#TOO_NEW` -- ordinal 1, key `"new"`.
    TooNew = 1,
    /// Port of `PackCompatibility#UNKNOWN` -- ordinal 2, key `"unknown"`.
    Unknown = 2,
    /// Port of `PackCompatibility#COMPATIBLE` -- ordinal 3, key `"compatible"`.
    Compatible = 3,
}

/// Java: `PackCompatibility.values()` -- declaration order.
pub const VALUES: [PackCompatibility; 4] = [
    PackCompatibility::TooOld,
    PackCompatibility::TooNew,
    PackCompatibility::Unknown,
    PackCompatibility::Compatible,
];

impl PackCompatibility {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PackCompatibility::TooOld => "TOO_OLD",
            PackCompatibility::TooNew => "TOO_NEW",
            PackCompatibility::Unknown => "UNKNOWN",
            PackCompatibility::Compatible => "COMPATIBLE",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `PackCompatibility#isCompatible()`: `this == COMPATIBLE` (so `UNKNOWN` is not).
    #[inline]
    pub const fn is_compatible(&self) -> bool {
        matches!(self, PackCompatibility::Compatible)
    }

    /// Java: `public Component getDescription()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_description(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Java: `public Component getConfirmation()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_confirmation(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }
}

impl std::fmt::Display for PackCompatibility {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Java: `public static PackCompatibility forVersion(InclusiveRange<PackFormat>
/// packDeclaredVersions, PackFormat gameSupportedVersion)`.
///
/// PORT-BLOCKED on `net.minecraft.util.InclusiveRange` and
/// `net.minecraft.server.packs.metadata.pack.PackFormat` (both SKELETON). See the module docs
/// for the decision order.
pub fn for_version() -> PackCompatibility {
    todo!("PORT-BLOCKED: net.minecraft.util.InclusiveRange and net.minecraft.server.packs.metadata.pack.PackFormat are not ported")
}
