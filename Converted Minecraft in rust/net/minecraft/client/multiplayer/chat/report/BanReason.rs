//! Port of: net/minecraft/client/multiplayer/chat/report/BanReason.java
//! Java class(es): net.minecraft.client.multiplayer.chat.report.BanReason
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, `byId`, and the key behind `title()`.
//! `Component` is not ported, so the getters that return the translated `Component` are
//! PORT-BLOCKED. The translation KEY each one is built from is ported as a `*_key()` getter and is
//! checked against the dump, which records the component as `translation{key='...', args=[]}`.
//!
//! # `byId` IS A SPARSE TABLE OF SERVICE CODES, NOT ORDINALS
//!
//! The ids come from the moderation service: several codes map to the same reason (`17, 19, 23, 31`
//! are all `GENERIC_VIOLATION`) and every unlisted code -- including `0`, `1` and every negative --
//! is `null` (`None`). Ordinals play no part.

/// Port of `net.minecraft.client.multiplayer.chat.report.BanReason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BanReason {
    /// `GENERIC_VIOLATION` -- ordinal 0.
    GenericViolation = 0,
    /// `FALSE_REPORTING` -- ordinal 1.
    FalseReporting = 1,
    /// `HATE_SPEECH` -- ordinal 2.
    HateSpeech = 2,
    /// `HATE_TERRORISM_NOTORIOUS_FIGURE` -- ordinal 3.
    HateTerrorismNotoriousFigure = 3,
    /// `HARASSMENT_OR_BULLYING` -- ordinal 4.
    HarassmentOrBullying = 4,
    /// `DEFAMATION_IMPERSONATION_FALSE_INFORMATION` -- ordinal 5.
    DefamationImpersonationFalseInformation = 5,
    /// `DRUGS` -- ordinal 6.
    Drugs = 6,
    /// `FRAUD` -- ordinal 7.
    Fraud = 7,
    /// `SPAM_OR_ADVERTISING` -- ordinal 8.
    SpamOrAdvertising = 8,
    /// `NUDITY_OR_PORNOGRAPHY` -- ordinal 9.
    NudityOrPornography = 9,
    /// `SEXUALLY_INAPPROPRIATE` -- ordinal 10.
    SexuallyInappropriate = 10,
    /// `EXTREME_VIOLENCE_OR_GORE` -- ordinal 11.
    ExtremeViolenceOrGore = 11,
    /// `IMMINENT_HARM_TO_PERSON_OR_PROPERTY` -- ordinal 12.
    ImminentHarmToPersonOrProperty = 12,
}

/// Java: `BanReason.values()`, declaration order.
pub const VALUES: [BanReason; 13] = [
    BanReason::GenericViolation,
    BanReason::FalseReporting,
    BanReason::HateSpeech,
    BanReason::HateTerrorismNotoriousFigure,
    BanReason::HarassmentOrBullying,
    BanReason::DefamationImpersonationFalseInformation,
    BanReason::Drugs,
    BanReason::Fraud,
    BanReason::SpamOrAdvertising,
    BanReason::NudityOrPornography,
    BanReason::SexuallyInappropriate,
    BanReason::ExtremeViolenceOrGore,
    BanReason::ImminentHarmToPersonOrProperty,
];

impl BanReason {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            BanReason::GenericViolation => "GENERIC_VIOLATION",
            BanReason::FalseReporting => "FALSE_REPORTING",
            BanReason::HateSpeech => "HATE_SPEECH",
            BanReason::HateTerrorismNotoriousFigure => "HATE_TERRORISM_NOTORIOUS_FIGURE",
            BanReason::HarassmentOrBullying => "HARASSMENT_OR_BULLYING",
            BanReason::DefamationImpersonationFalseInformation => "DEFAMATION_IMPERSONATION_FALSE_INFORMATION",
            BanReason::Drugs => "DRUGS",
            BanReason::Fraud => "FRAUD",
            BanReason::SpamOrAdvertising => "SPAM_OR_ADVERTISING",
            BanReason::NudityOrPornography => "NUDITY_OR_PORNOGRAPHY",
            BanReason::SexuallyInappropriate => "SEXUALLY_INAPPROPRIATE",
            BanReason::ExtremeViolenceOrGore => "EXTREME_VIOLENCE_OR_GORE",
            BanReason::ImminentHarmToPersonOrProperty => "IMMINENT_HARM_TO_PERSON_OR_PROPERTY",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// The translation key `title` is built from: `"gui.banned.reason." + name`.
    pub const fn title_key(&self) -> &'static str {
        match self {
            BanReason::GenericViolation => "gui.banned.reason.generic_violation",
            BanReason::FalseReporting => "gui.banned.reason.false_reporting",
            BanReason::HateSpeech => "gui.banned.reason.hate_speech",
            BanReason::HateTerrorismNotoriousFigure => "gui.banned.reason.hate_terrorism_notorious_figure",
            BanReason::HarassmentOrBullying => "gui.banned.reason.harassment_or_bullying",
            BanReason::DefamationImpersonationFalseInformation => "gui.banned.reason.defamation_impersonation_false_information",
            BanReason::Drugs => "gui.banned.reason.drugs",
            BanReason::Fraud => "gui.banned.reason.fraud",
            BanReason::SpamOrAdvertising => "gui.banned.reason.spam_or_advertising",
            BanReason::NudityOrPornography => "gui.banned.reason.nudity_or_pornography",
            BanReason::SexuallyInappropriate => "gui.banned.reason.sexually_inappropriate",
            BanReason::ExtremeViolenceOrGore => "gui.banned.reason.extreme_violence_or_gore",
            BanReason::ImminentHarmToPersonOrProperty => "gui.banned.reason.imminent_harm_to_person_or_property",
        }
    }

    /// Java: `public Component title()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn title(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Port of `BanReason#byId(int)`. `None` is Java's `null`.
    pub const fn by_id(id: i32) -> Option<BanReason> {
        match id {
            2 => Some(BanReason::FalseReporting),
            5 => Some(BanReason::HateSpeech),
            16 | 25 => Some(BanReason::HateTerrorismNotoriousFigure),
            17 | 19 | 23 | 31 => Some(BanReason::GenericViolation),
            21 => Some(BanReason::HarassmentOrBullying),
            27 => Some(BanReason::DefamationImpersonationFalseInformation),
            28 => Some(BanReason::Drugs),
            29 => Some(BanReason::Fraud),
            30 => Some(BanReason::SpamOrAdvertising),
            32 => Some(BanReason::NudityOrPornography),
            33 | 35 | 36 => Some(BanReason::SexuallyInappropriate),
            34 => Some(BanReason::ExtremeViolenceOrGore),
            53 => Some(BanReason::ImminentHarmToPersonOrProperty),
            _ => None,
        }
    }
}

impl std::fmt::Display for BanReason {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
