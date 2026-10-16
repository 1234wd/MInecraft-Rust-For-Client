//! Port of: net/minecraft/util/TriState.java
//! Java class(es): net.minecraft.util.TriState
//! Status: PARTIAL
//!
//! Session 19. The three constants, `from(boolean)`, `toBoolean(boolean)` and
//! `getSerializedName()` are ported; ordinals and the `name` field are checked against the jar
//! in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED: `CODEC` (DataFixerUpper `Codec.either` / `Either`).
//!
//! # THE CODEC WRITES BOOLEANS, NOT STRINGS, FOR TRUE/FALSE
//!
//! `CODEC` is `Codec.either(Codec.BOOL, fromEnum(values))` mapped so that `TRUE`/`FALSE`
//! encode as JSON booleans and only `DEFAULT` encodes as the string `"default"`. Decoding
//! accepts either form (so `"true"` the string also decodes, via the enum branch). Whoever
//! ports the codec must keep that asymmetry.

/// Port of `net.minecraft.util.TriState`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TriState {
    /// Port of `TriState#TRUE` -- ordinal 0, name `"true"`.
    True = 0,
    /// Port of `TriState#FALSE` -- ordinal 1, name `"false"`.
    False = 1,
    /// Port of `TriState#DEFAULT` -- ordinal 2, name `"default"`.
    Default = 2,
}

/// Java: `TriState.values()` -- declaration order.
pub const VALUES: [TriState; 3] = [TriState::True, TriState::False, TriState::Default];

impl TriState {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            TriState::True => "TRUE",
            TriState::False => "FALSE",
            TriState::Default => "DEFAULT",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `TriState#toBoolean(boolean)`: `TRUE -> true`, `FALSE -> false`, otherwise
    /// `defaultValue`.
    #[inline]
    pub const fn to_boolean(&self, default_value: bool) -> bool {
        match self {
            TriState::True => true,
            TriState::False => false,
            TriState::Default => default_value,
        }
    }

    /// Port of `TriState#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            TriState::True => "true",
            TriState::False => "false",
            TriState::Default => "default",
        }
    }
}

impl std::fmt::Display for TriState {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for TriState {
    fn get_serialized_name(&self) -> &str {
        TriState::get_serialized_name(self)
    }
}

/// Port of `TriState#from(boolean)`: `value ? TRUE : FALSE`. Never yields `DEFAULT`.
#[inline]
pub const fn from(value: bool) -> TriState {
    if value {
        TriState::True
    } else {
        TriState::False
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<TriState> CODEC` (`Codec.either(Codec.BOOL, ...).xmap(...)`).
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec` and `com.mojang.datafixers.util.Either`.
/// See the module docs for the boolean/string asymmetry it must keep.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
