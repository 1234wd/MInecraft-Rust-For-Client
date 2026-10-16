//! Port of: net/minecraft/world/level/block/state/properties/NoteBlockInstrument.java
//! Java class(es): net.minecraft.world.level.block.state.properties.NoteBlockInstrument
//! Status: PARTIAL
//!
//! Session 19. 27 constants carrying a `String name`, a `Holder<SoundEvent> soundEvent` and a
//! private nested `NoteBlockInstrument.Type` (`BASE_BLOCK`, `MOB_HEAD`, `CUSTOM`). Ordinals, the
//! `name` field and the `type` field of every constant are checked against the jar
//! (`session19_a.rs`).
//!
//! Ported: `getSerializedName`, `isTunable` (`type == BASE_BLOCK`), `hasCustomSound`
//! (`type == CUSTOM`), `worksAboveNoteBlock` (`type != BASE_BLOCK`), and the nested `Type` enum
//! ([`Type`], with the private field exposed read-only as [`NoteBlockInstrument::get_type`]).
//! No `toString()` override: `Display` is `name()` (`IRON_XYLOPHONE`).
//!
//! PORT-BLOCKED: `getSoundEvent()` / the `soundEvent` field. It returns a
//! `net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent>` taken from
//! `net.minecraft.sounds.SoundEvents`, all three SKELETON. The `SoundEvents` constant each value
//! uses is recorded on its variant below so the unblock is mechanical.
//!
//! # NAMES THAT DO NOT MATCH THEIR SOUND
//!
//! `DRAGON` holds `NOTE_BLOCK_IMITATE_ENDER_DRAGON`. `CUSTOM_HEAD` holds `UI_BUTTON_CLICK`, but
//! `NoteBlock.playNote` never uses it: `hasCustomSound()` makes it read the skull block entity's
//! `note_block_sound` instead and play nothing when that is absent. 27 values is above `StringRepresentable.PRE_BUILT_MAP_THRESHOLD` (16), so any
//! `createNameLookup` over this enum takes the `HashMap` branch.

/// Port of `net.minecraft.world.level.block.state.properties.NoteBlockInstrument`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NoteBlockInstrument {
    /// `HARP` -- ordinal 0, `"harp"`, `SoundEvents.NOTE_BLOCK_HARP`, `BASE_BLOCK`.
    Harp = 0,
    /// `BASEDRUM` -- ordinal 1, `"basedrum"`, `SoundEvents.NOTE_BLOCK_BASEDRUM`, `BASE_BLOCK`.
    Basedrum = 1,
    /// `SNARE` -- ordinal 2, `"snare"`, `SoundEvents.NOTE_BLOCK_SNARE`, `BASE_BLOCK`.
    Snare = 2,
    /// `HAT` -- ordinal 3, `"hat"`, `SoundEvents.NOTE_BLOCK_HAT`, `BASE_BLOCK`.
    Hat = 3,
    /// `BASS` -- ordinal 4, `"bass"`, `SoundEvents.NOTE_BLOCK_BASS`, `BASE_BLOCK`.
    Bass = 4,
    /// `FLUTE` -- ordinal 5, `"flute"`, `SoundEvents.NOTE_BLOCK_FLUTE`, `BASE_BLOCK`.
    Flute = 5,
    /// `BELL` -- ordinal 6, `"bell"`, `SoundEvents.NOTE_BLOCK_BELL`, `BASE_BLOCK`.
    Bell = 6,
    /// `GUITAR` -- ordinal 7, `"guitar"`, `SoundEvents.NOTE_BLOCK_GUITAR`, `BASE_BLOCK`.
    Guitar = 7,
    /// `CHIME` -- ordinal 8, `"chime"`, `SoundEvents.NOTE_BLOCK_CHIME`, `BASE_BLOCK`.
    Chime = 8,
    /// `XYLOPHONE` -- ordinal 9, `"xylophone"`, `SoundEvents.NOTE_BLOCK_XYLOPHONE`, `BASE_BLOCK`.
    Xylophone = 9,
    /// `IRON_XYLOPHONE` -- ordinal 10, `"iron_xylophone"`, `SoundEvents.NOTE_BLOCK_IRON_XYLOPHONE`,
    /// `BASE_BLOCK`.
    IronXylophone = 10,
    /// `COW_BELL` -- ordinal 11, `"cow_bell"`, `SoundEvents.NOTE_BLOCK_COW_BELL`, `BASE_BLOCK`.
    CowBell = 11,
    /// `DIDGERIDOO` -- ordinal 12, `"didgeridoo"`, `SoundEvents.NOTE_BLOCK_DIDGERIDOO`, `BASE_BLOCK`.
    Didgeridoo = 12,
    /// `BIT` -- ordinal 13, `"bit"`, `SoundEvents.NOTE_BLOCK_BIT`, `BASE_BLOCK`.
    Bit = 13,
    /// `BANJO` -- ordinal 14, `"banjo"`, `SoundEvents.NOTE_BLOCK_BANJO`, `BASE_BLOCK`.
    Banjo = 14,
    /// `PLING` -- ordinal 15, `"pling"`, `SoundEvents.NOTE_BLOCK_PLING`, `BASE_BLOCK`.
    Pling = 15,
    /// `TRUMPET` -- ordinal 16, `"trumpet"`, `SoundEvents.NOTE_BLOCK_TRUMPET`, `BASE_BLOCK`.
    Trumpet = 16,
    /// `TRUMPET_EXPOSED` -- ordinal 17, `"trumpet_exposed"`,
    /// `SoundEvents.NOTE_BLOCK_TRUMPET_EXPOSED`, `BASE_BLOCK`.
    TrumpetExposed = 17,
    /// `TRUMPET_OXIDIZED` -- ordinal 18, `"trumpet_oxidized"`,
    /// `SoundEvents.NOTE_BLOCK_TRUMPET_OXIDIZED`, `BASE_BLOCK`.
    TrumpetOxidized = 18,
    /// `TRUMPET_WEATHERED` -- ordinal 19, `"trumpet_weathered"`,
    /// `SoundEvents.NOTE_BLOCK_TRUMPET_WEATHERED`, `BASE_BLOCK`.
    TrumpetWeathered = 19,
    /// `ZOMBIE` -- ordinal 20, `"zombie"`, `SoundEvents.NOTE_BLOCK_IMITATE_ZOMBIE`, `MOB_HEAD`.
    Zombie = 20,
    /// `SKELETON` -- ordinal 21, `"skeleton"`, `SoundEvents.NOTE_BLOCK_IMITATE_SKELETON`, `MOB_HEAD`.
    Skeleton = 21,
    /// `CREEPER` -- ordinal 22, `"creeper"`, `SoundEvents.NOTE_BLOCK_IMITATE_CREEPER`, `MOB_HEAD`.
    Creeper = 22,
    /// `DRAGON` -- ordinal 23, `"dragon"`, `SoundEvents.NOTE_BLOCK_IMITATE_ENDER_DRAGON`, `MOB_HEAD`.
    Dragon = 23,
    /// `WITHER_SKELETON` -- ordinal 24, `"wither_skeleton"`,
    /// `SoundEvents.NOTE_BLOCK_IMITATE_WITHER_SKELETON`, `MOB_HEAD`.
    WitherSkeleton = 24,
    /// `PIGLIN` -- ordinal 25, `"piglin"`, `SoundEvents.NOTE_BLOCK_IMITATE_PIGLIN`, `MOB_HEAD`.
    Piglin = 25,
    /// `CUSTOM_HEAD` -- ordinal 26, `"custom_head"`, `SoundEvents.UI_BUTTON_CLICK`, `CUSTOM`.
    CustomHead = 26,
}

/// Java: `NoteBlockInstrument.values()`, declaration order.
pub const VALUES: [NoteBlockInstrument; 27] = [
    NoteBlockInstrument::Harp,
    NoteBlockInstrument::Basedrum,
    NoteBlockInstrument::Snare,
    NoteBlockInstrument::Hat,
    NoteBlockInstrument::Bass,
    NoteBlockInstrument::Flute,
    NoteBlockInstrument::Bell,
    NoteBlockInstrument::Guitar,
    NoteBlockInstrument::Chime,
    NoteBlockInstrument::Xylophone,
    NoteBlockInstrument::IronXylophone,
    NoteBlockInstrument::CowBell,
    NoteBlockInstrument::Didgeridoo,
    NoteBlockInstrument::Bit,
    NoteBlockInstrument::Banjo,
    NoteBlockInstrument::Pling,
    NoteBlockInstrument::Trumpet,
    NoteBlockInstrument::TrumpetExposed,
    NoteBlockInstrument::TrumpetOxidized,
    NoteBlockInstrument::TrumpetWeathered,
    NoteBlockInstrument::Zombie,
    NoteBlockInstrument::Skeleton,
    NoteBlockInstrument::Creeper,
    NoteBlockInstrument::Dragon,
    NoteBlockInstrument::WitherSkeleton,
    NoteBlockInstrument::Piglin,
    NoteBlockInstrument::CustomHead,
];

impl NoteBlockInstrument {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            NoteBlockInstrument::Harp => "HARP",
            NoteBlockInstrument::Basedrum => "BASEDRUM",
            NoteBlockInstrument::Snare => "SNARE",
            NoteBlockInstrument::Hat => "HAT",
            NoteBlockInstrument::Bass => "BASS",
            NoteBlockInstrument::Flute => "FLUTE",
            NoteBlockInstrument::Bell => "BELL",
            NoteBlockInstrument::Guitar => "GUITAR",
            NoteBlockInstrument::Chime => "CHIME",
            NoteBlockInstrument::Xylophone => "XYLOPHONE",
            NoteBlockInstrument::IronXylophone => "IRON_XYLOPHONE",
            NoteBlockInstrument::CowBell => "COW_BELL",
            NoteBlockInstrument::Didgeridoo => "DIDGERIDOO",
            NoteBlockInstrument::Bit => "BIT",
            NoteBlockInstrument::Banjo => "BANJO",
            NoteBlockInstrument::Pling => "PLING",
            NoteBlockInstrument::Trumpet => "TRUMPET",
            NoteBlockInstrument::TrumpetExposed => "TRUMPET_EXPOSED",
            NoteBlockInstrument::TrumpetOxidized => "TRUMPET_OXIDIZED",
            NoteBlockInstrument::TrumpetWeathered => "TRUMPET_WEATHERED",
            NoteBlockInstrument::Zombie => "ZOMBIE",
            NoteBlockInstrument::Skeleton => "SKELETON",
            NoteBlockInstrument::Creeper => "CREEPER",
            NoteBlockInstrument::Dragon => "DRAGON",
            NoteBlockInstrument::WitherSkeleton => "WITHER_SKELETON",
            NoteBlockInstrument::Piglin => "PIGLIN",
            NoteBlockInstrument::CustomHead => "CUSTOM_HEAD",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `NoteBlockInstrument#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            NoteBlockInstrument::Harp => "harp",
            NoteBlockInstrument::Basedrum => "basedrum",
            NoteBlockInstrument::Snare => "snare",
            NoteBlockInstrument::Hat => "hat",
            NoteBlockInstrument::Bass => "bass",
            NoteBlockInstrument::Flute => "flute",
            NoteBlockInstrument::Bell => "bell",
            NoteBlockInstrument::Guitar => "guitar",
            NoteBlockInstrument::Chime => "chime",
            NoteBlockInstrument::Xylophone => "xylophone",
            NoteBlockInstrument::IronXylophone => "iron_xylophone",
            NoteBlockInstrument::CowBell => "cow_bell",
            NoteBlockInstrument::Didgeridoo => "didgeridoo",
            NoteBlockInstrument::Bit => "bit",
            NoteBlockInstrument::Banjo => "banjo",
            NoteBlockInstrument::Pling => "pling",
            NoteBlockInstrument::Trumpet => "trumpet",
            NoteBlockInstrument::TrumpetExposed => "trumpet_exposed",
            NoteBlockInstrument::TrumpetOxidized => "trumpet_oxidized",
            NoteBlockInstrument::TrumpetWeathered => "trumpet_weathered",
            NoteBlockInstrument::Zombie => "zombie",
            NoteBlockInstrument::Skeleton => "skeleton",
            NoteBlockInstrument::Creeper => "creeper",
            NoteBlockInstrument::Dragon => "dragon",
            NoteBlockInstrument::WitherSkeleton => "wither_skeleton",
            NoteBlockInstrument::Piglin => "piglin",
            NoteBlockInstrument::CustomHead => "custom_head",
        }
    }

    /// The private `type` field (Java has no getter; exposed read-only so the jar's `type`
    /// column can be checked and the three predicates below stay one-liners over it).
    #[inline]
    pub const fn get_type(&self) -> Type {
        match self {
            NoteBlockInstrument::Harp
            | NoteBlockInstrument::Basedrum
            | NoteBlockInstrument::Snare
            | NoteBlockInstrument::Hat
            | NoteBlockInstrument::Bass
            | NoteBlockInstrument::Flute
            | NoteBlockInstrument::Bell
            | NoteBlockInstrument::Guitar
            | NoteBlockInstrument::Chime
            | NoteBlockInstrument::Xylophone
            | NoteBlockInstrument::IronXylophone
            | NoteBlockInstrument::CowBell
            | NoteBlockInstrument::Didgeridoo
            | NoteBlockInstrument::Bit
            | NoteBlockInstrument::Banjo
            | NoteBlockInstrument::Pling
            | NoteBlockInstrument::Trumpet
            | NoteBlockInstrument::TrumpetExposed
            | NoteBlockInstrument::TrumpetOxidized
            | NoteBlockInstrument::TrumpetWeathered => Type::BaseBlock,
            NoteBlockInstrument::Zombie
            | NoteBlockInstrument::Skeleton
            | NoteBlockInstrument::Creeper
            | NoteBlockInstrument::Dragon
            | NoteBlockInstrument::WitherSkeleton
            | NoteBlockInstrument::Piglin => Type::MobHead,
            NoteBlockInstrument::CustomHead => Type::Custom,
        }
    }

    /// Port of `NoteBlockInstrument#isTunable()`: `type == Type.BASE_BLOCK`.
    #[inline]
    pub const fn is_tunable(&self) -> bool {
        matches!(self.get_type(), Type::BaseBlock)
    }

    /// Port of `NoteBlockInstrument#hasCustomSound()`: `type == Type.CUSTOM`.
    #[inline]
    pub const fn has_custom_sound(&self) -> bool {
        matches!(self.get_type(), Type::Custom)
    }

    /// Port of `NoteBlockInstrument#worksAboveNoteBlock()`: `type != Type.BASE_BLOCK`.
    #[inline]
    pub const fn works_above_note_block(&self) -> bool {
        !matches!(self.get_type(), Type::BaseBlock)
    }

    /// Java: `public Holder<SoundEvent> getSoundEvent()` -- the `soundEvent` field (the
    /// `SoundEvents` constant named on each variant).
    ///
    /// PORT-BLOCKED on `net.minecraft.core.Holder` / `net.minecraft.sounds.SoundEvent` /
    /// `net.minecraft.sounds.SoundEvents`.
    pub fn get_sound_event(&self) {
        todo!("PORT-BLOCKED: net.minecraft.core.Holder<net.minecraft.sounds.SoundEvent> is not ported")
    }
}

impl std::fmt::Display for NoteBlockInstrument {
    /// Java's `Enum#toString()` (not overridden), i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for NoteBlockInstrument {
    fn get_serialized_name(&self) -> &str {
        NoteBlockInstrument::get_serialized_name(self)
    }
}

/// Port of the private nested enum `NoteBlockInstrument.Type`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    /// `BASE_BLOCK` -- ordinal 0: played by the block under the note block; tunable.
    BaseBlock = 0,
    /// `MOB_HEAD` -- ordinal 1: a mob head on top of the note block.
    MobHead = 1,
    /// `CUSTOM` -- ordinal 2: a player head with a custom note-block sound.
    Custom = 2,
}

/// Java: `NoteBlockInstrument.Type.values()`, declaration order.
pub const TYPE_VALUES: [Type; 3] = [Type::BaseBlock, Type::MobHead, Type::Custom];

impl Type {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Type::BaseBlock => "BASE_BLOCK",
            Type::MobHead => "MOB_HEAD",
            Type::Custom => "CUSTOM",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }
}

impl std::fmt::Display for Type {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
