//! Port of: net/minecraft/client/multiplayer/chat/report/ReportReason.java
//! Java class(es): net.minecraft.client.multiplayer.chat.report.ReportReason
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, `backendName` (upper-cased constructor argument), the keys behind
//! `title()`/`description()`, and `getIncompatibleCategories(ReportType)` (order preserved, it is a
//! `List.of`).
//! `Component` is not ported, so the getters that return the translated `Component` are
//! PORT-BLOCKED. The translation KEY each one is built from is ported as a `*_key()` getter and is
//! checked against the dump, which records the component as `translation{key='...', args=[]}`.

use crate::net::minecraft::client::multiplayer::chat::report::ReportType::ReportType;

/// Port of `net.minecraft.client.multiplayer.chat.report.ReportReason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReportReason {
    /// `I_WANT_TO_REPORT_THEM` -- ordinal 0.
    IWantToReportThem = 0,
    /// `HATE_SPEECH` -- ordinal 1.
    HateSpeech = 1,
    /// `HARASSMENT_OR_BULLYING` -- ordinal 2.
    HarassmentOrBullying = 2,
    /// `SELF_HARM_OR_SUICIDE` -- ordinal 3.
    SelfHarmOrSuicide = 3,
    /// `IMMINENT_HARM` -- ordinal 4.
    ImminentHarm = 4,
    /// `DEFAMATION_IMPERSONATION_FALSE_INFORMATION` -- ordinal 5.
    DefamationImpersonationFalseInformation = 5,
    /// `ALCOHOL_TOBACCO_DRUGS` -- ordinal 6.
    AlcoholTobaccoDrugs = 6,
    /// `CHILD_SEXUAL_EXPLOITATION_OR_ABUSE` -- ordinal 7.
    ChildSexualExploitationOrAbuse = 7,
    /// `TERRORISM_OR_VIOLENT_EXTREMISM` -- ordinal 8.
    TerrorismOrViolentExtremism = 8,
    /// `NON_CONSENSUAL_INTIMATE_IMAGERY` -- ordinal 9.
    NonConsensualIntimateImagery = 9,
    /// `SEXUALLY_INAPPROPRIATE` -- ordinal 10.
    SexuallyInappropriate = 10,
}

/// Java: `ReportReason.values()`, declaration order.
pub const VALUES: [ReportReason; 11] = [
    ReportReason::IWantToReportThem,
    ReportReason::HateSpeech,
    ReportReason::HarassmentOrBullying,
    ReportReason::SelfHarmOrSuicide,
    ReportReason::ImminentHarm,
    ReportReason::DefamationImpersonationFalseInformation,
    ReportReason::AlcoholTobaccoDrugs,
    ReportReason::ChildSexualExploitationOrAbuse,
    ReportReason::TerrorismOrViolentExtremism,
    ReportReason::NonConsensualIntimateImagery,
    ReportReason::SexuallyInappropriate,
];

impl ReportReason {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ReportReason::IWantToReportThem => "I_WANT_TO_REPORT_THEM",
            ReportReason::HateSpeech => "HATE_SPEECH",
            ReportReason::HarassmentOrBullying => "HARASSMENT_OR_BULLYING",
            ReportReason::SelfHarmOrSuicide => "SELF_HARM_OR_SUICIDE",
            ReportReason::ImminentHarm => "IMMINENT_HARM",
            ReportReason::DefamationImpersonationFalseInformation => "DEFAMATION_IMPERSONATION_FALSE_INFORMATION",
            ReportReason::AlcoholTobaccoDrugs => "ALCOHOL_TOBACCO_DRUGS",
            ReportReason::ChildSexualExploitationOrAbuse => "CHILD_SEXUAL_EXPLOITATION_OR_ABUSE",
            ReportReason::TerrorismOrViolentExtremism => "TERRORISM_OR_VIOLENT_EXTREMISM",
            ReportReason::NonConsensualIntimateImagery => "NON_CONSENSUAL_INTIMATE_IMAGERY",
            ReportReason::SexuallyInappropriate => "SEXUALLY_INAPPROPRIATE",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ReportReason#backendName()` (`name.toUpperCase(Locale.ROOT)`).
    pub const fn backend_name(&self) -> &'static str {
        match self {
            ReportReason::IWantToReportThem => "I_WANT_TO_REPORT_THEM",
            ReportReason::HateSpeech => "HATE_SPEECH",
            ReportReason::HarassmentOrBullying => "HARASSMENT_OR_BULLYING",
            ReportReason::SelfHarmOrSuicide => "SELF_HARM_OR_SUICIDE",
            ReportReason::ImminentHarm => "IMMINENT_HARM",
            ReportReason::DefamationImpersonationFalseInformation => "DEFAMATION_IMPERSONATION_FALSE_INFORMATION",
            ReportReason::AlcoholTobaccoDrugs => "ALCOHOL_TOBACCO_DRUGS",
            ReportReason::ChildSexualExploitationOrAbuse => "CHILD_SEXUAL_EXPLOITATION_OR_ABUSE",
            ReportReason::TerrorismOrViolentExtremism => "TERRORISM_OR_VIOLENT_EXTREMISM",
            ReportReason::NonConsensualIntimateImagery => "NON_CONSENSUAL_INTIMATE_IMAGERY",
            ReportReason::SexuallyInappropriate => "SEXUALLY_INAPPROPRIATE",
        }
    }

    /// The key `title` is built from.
    pub const fn title_key(&self) -> &'static str {
        match self {
            ReportReason::IWantToReportThem => "gui.abuseReport.reason.i_want_to_report_them",
            ReportReason::HateSpeech => "gui.abuseReport.reason.hate_speech",
            ReportReason::HarassmentOrBullying => "gui.abuseReport.reason.harassment_or_bullying",
            ReportReason::SelfHarmOrSuicide => "gui.abuseReport.reason.self_harm_or_suicide",
            ReportReason::ImminentHarm => "gui.abuseReport.reason.imminent_harm",
            ReportReason::DefamationImpersonationFalseInformation => "gui.abuseReport.reason.defamation_impersonation_false_information",
            ReportReason::AlcoholTobaccoDrugs => "gui.abuseReport.reason.alcohol_tobacco_drugs",
            ReportReason::ChildSexualExploitationOrAbuse => "gui.abuseReport.reason.child_sexual_exploitation_or_abuse",
            ReportReason::TerrorismOrViolentExtremism => "gui.abuseReport.reason.terrorism_or_violent_extremism",
            ReportReason::NonConsensualIntimateImagery => "gui.abuseReport.reason.non_consensual_intimate_imagery",
            ReportReason::SexuallyInappropriate => "gui.abuseReport.reason.sexually_inappropriate",
        }
    }

    /// The key `description` is built from.
    pub const fn description_key(&self) -> &'static str {
        match self {
            ReportReason::IWantToReportThem => "gui.abuseReport.reason.i_want_to_report_them.description",
            ReportReason::HateSpeech => "gui.abuseReport.reason.hate_speech.description",
            ReportReason::HarassmentOrBullying => "gui.abuseReport.reason.harassment_or_bullying.description",
            ReportReason::SelfHarmOrSuicide => "gui.abuseReport.reason.self_harm_or_suicide.description",
            ReportReason::ImminentHarm => "gui.abuseReport.reason.imminent_harm.description",
            ReportReason::DefamationImpersonationFalseInformation => "gui.abuseReport.reason.defamation_impersonation_false_information.description",
            ReportReason::AlcoholTobaccoDrugs => "gui.abuseReport.reason.alcohol_tobacco_drugs.description",
            ReportReason::ChildSexualExploitationOrAbuse => "gui.abuseReport.reason.child_sexual_exploitation_or_abuse.description",
            ReportReason::TerrorismOrViolentExtremism => "gui.abuseReport.reason.terrorism_or_violent_extremism.description",
            ReportReason::NonConsensualIntimateImagery => "gui.abuseReport.reason.non_consensual_intimate_imagery.description",
            ReportReason::SexuallyInappropriate => "gui.abuseReport.reason.sexually_inappropriate.description",
        }
    }

    /// Java: `public Component title()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn title(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Java: `public Component description()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn description(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Port of `ReportReason#getIncompatibleCategories(ReportType)`.
    pub fn get_incompatible_categories(report_type: ReportType) -> Vec<ReportReason> {
        match report_type {
            ReportType::Chat => vec![ReportReason::SexuallyInappropriate],
            ReportType::Skin => vec![ReportReason::ImminentHarm, ReportReason::DefamationImpersonationFalseInformation],
            _ => Vec::new(),
        }
    }
}

impl std::fmt::Display for ReportReason {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
