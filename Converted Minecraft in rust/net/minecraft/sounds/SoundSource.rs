//! Port of: net/minecraft/sounds/SoundSource.java
//! Java class(es): net.minecraft.sounds.SoundSource
//! Status: VERIFIED
//!
//! Session 19. Eleven constants, each with the `name` used for the volume-slider option keys.
//! Names, ordinals and the `name` field are checked against the jar in
//! `_porting/tests/session19_b.rs`.
//!
//! # THE `name` FIELD IS SINGULAR WHERE THE CONSTANT IS PLURAL
//!
//! `RECORDS` -> `"record"`, `BLOCKS` -> `"block"`, `PLAYERS` -> `"player"`. Java's `name()`
//! (`"RECORDS"`) and `getName()` (`"record"`) are different strings; the port keeps them as
//! [`SoundSource::name`] and [`SoundSource::get_name`].

/// Port of `net.minecraft.sounds.SoundSource`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SoundSource {
    /// Port of `SoundSource#MASTER` -- ordinal 0, name `"master"`.
    Master = 0,
    /// Port of `SoundSource#MUSIC` -- ordinal 1, name `"music"`.
    Music = 1,
    /// Port of `SoundSource#RECORDS` -- ordinal 2, name `"record"`.
    Records = 2,
    /// Port of `SoundSource#WEATHER` -- ordinal 3, name `"weather"`.
    Weather = 3,
    /// Port of `SoundSource#BLOCKS` -- ordinal 4, name `"block"`.
    Blocks = 4,
    /// Port of `SoundSource#HOSTILE` -- ordinal 5, name `"hostile"`.
    Hostile = 5,
    /// Port of `SoundSource#NEUTRAL` -- ordinal 6, name `"neutral"`.
    Neutral = 6,
    /// Port of `SoundSource#PLAYERS` -- ordinal 7, name `"player"`.
    Players = 7,
    /// Port of `SoundSource#AMBIENT` -- ordinal 8, name `"ambient"`.
    Ambient = 8,
    /// Port of `SoundSource#VOICE` -- ordinal 9, name `"voice"`.
    Voice = 9,
    /// Port of `SoundSource#UI` -- ordinal 10, name `"ui"`.
    Ui = 10,
}

/// Java: `SoundSource.values()` -- declaration order.
pub const VALUES: [SoundSource; 11] = [
    SoundSource::Master,
    SoundSource::Music,
    SoundSource::Records,
    SoundSource::Weather,
    SoundSource::Blocks,
    SoundSource::Hostile,
    SoundSource::Neutral,
    SoundSource::Players,
    SoundSource::Ambient,
    SoundSource::Voice,
    SoundSource::Ui,
];

impl SoundSource {
    /// Java `Enum#name()` -- the constant name, e.g. `"RECORDS"`. Not the `name` field; see
    /// [`SoundSource::get_name`].
    pub const fn name(&self) -> &'static str {
        match self {
            SoundSource::Master => "MASTER",
            SoundSource::Music => "MUSIC",
            SoundSource::Records => "RECORDS",
            SoundSource::Weather => "WEATHER",
            SoundSource::Blocks => "BLOCKS",
            SoundSource::Hostile => "HOSTILE",
            SoundSource::Neutral => "NEUTRAL",
            SoundSource::Players => "PLAYERS",
            SoundSource::Ambient => "AMBIENT",
            SoundSource::Voice => "VOICE",
            SoundSource::Ui => "UI",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `SoundSource#getName()` -- the private `name` field, e.g. `"record"`.
    #[inline]
    pub const fn get_name(&self) -> &'static str {
        match self {
            SoundSource::Master => "master",
            SoundSource::Music => "music",
            SoundSource::Records => "record",
            SoundSource::Weather => "weather",
            SoundSource::Blocks => "block",
            SoundSource::Hostile => "hostile",
            SoundSource::Neutral => "neutral",
            SoundSource::Players => "player",
            SoundSource::Ambient => "ambient",
            SoundSource::Voice => "voice",
            SoundSource::Ui => "ui",
        }
    }
}

impl std::fmt::Display for SoundSource {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
