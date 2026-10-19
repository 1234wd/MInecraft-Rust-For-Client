//! Port of: net/minecraft/util/CommonLinks.java
//! Java class(es): net.minecraft.util.CommonLinks
//! Status: VERIFIED

use crate::javacompat::java_net::Uri;
use crate::javacompat::java_util::Uuid;

/// The fixed URI and string constants used by Minecraft's help and account links.
pub struct CommonLinks;

impl CommonLinks {
    pub const GDPR: Uri = Uri::from_static("https://aka.ms/MinecraftGDPR");
    pub const EULA: Uri = Uri::from_static("https://aka.ms/MinecraftEULA");
    pub const PRIVACY_STATEMENT: Uri =
        Uri::from_static("http://go.microsoft.com/fwlink/?LinkId=521839");
    pub const ATTRIBUTION: Uri = Uri::from_static("https://aka.ms/MinecraftJavaAttribution");
    pub const LICENSES: Uri = Uri::from_static("https://aka.ms/MinecraftJavaLicenses");
    pub const BUY_MINECRAFT_JAVA: Uri = Uri::from_static("https://aka.ms/BuyMinecraftJava");
    pub const ACCOUNT_SETTINGS: Uri = Uri::from_static("https://aka.ms/JavaAccountSettings");
    pub const SNAPSHOT_FEEDBACK: Uri = Uri::from_static("https://aka.ms/snapshotfeedback?ref=game");
    pub const RELEASE_FEEDBACK: Uri = Uri::from_static("https://aka.ms/javafeedback?ref=game");
    pub const SNAPSHOT_BUGS_FEEDBACK: Uri =
        Uri::from_static("https://aka.ms/snapshotbugs?ref=game");
    pub const GENERAL_HELP: Uri = Uri::from_static("https://aka.ms/Minecraft-Support");
    pub const ACCESSIBILITY_HELP: Uri =
        Uri::from_static("https://aka.ms/MinecraftJavaAccessibility");
    pub const REPORTING_HELP: Uri = Uri::from_static("https://aka.ms/aboutjavareporting");
    pub const SUSPENSION_HELP: Uri = Uri::from_static("https://aka.ms/mcjavamoderation");
    pub const BLOCKING_HELP: Uri = Uri::from_static("https://aka.ms/javablocking");
    pub const SYMLINK_HELP: Uri = Uri::from_static("https://aka.ms/MinecraftSymLinks");
    pub const PRIVACY_AND_ONLINE_SETTINGS: Uri =
        Uri::from_static("https://aka.ms/MinecraftJavaXboxPrivacyAndSafety");
    pub const START_REALMS_TRIAL: Uri = Uri::from_static("https://aka.ms/startjavarealmstrial");
    pub const BUY_REALMS: Uri = Uri::from_static("https://aka.ms/BuyJavaRealms");
    pub const REALMS_TERMS: Uri = Uri::from_static("https://aka.ms/MinecraftRealmsTerms");
    pub const REALMS_CONTENT_CREATION: Uri =
        Uri::from_static("https://aka.ms/MinecraftRealmsContentCreator");
    pub const EXTEND_REALMS_LINK: &'static str = "https://aka.ms/ExtendJavaRealms";
    pub const INTENTIONAL_GAME_DESIGN_BUG_ID: &'static str = "MCPE-28723";
    pub const INTENTIONAL_GAME_DESIGN_BUG: Uri =
        Uri::from_static("https://bugs.mojang.com/browse/MCPE-28723");

    /// Java's implicit public no-argument constructor.
    pub const fn new() -> Self {
        Self
    }

    /// `extendRealms(String, UUID, boolean)`, preserving Java's literal concatenation.
    pub fn extend_realms(
        subscription_id: Option<&str>,
        profile_id: Option<Uuid>,
        trial: bool,
    ) -> String {
        let Some(subscription_id) = subscription_id else {
            return Self::EXTEND_REALMS_LINK.to_string();
        };
        format!(
            "{}&ref={}",
            Self::extend_realms_basic(Some(subscription_id), profile_id),
            if trial {
                "expiredTrial"
            } else {
                "expiredRealm"
            }
        )
    }

    /// `extendRealms(String, UUID)`.
    pub fn extend_realms_basic(subscription_id: Option<&str>, profile_id: Option<Uuid>) -> String {
        let Some(subscription_id) = subscription_id else {
            return Self::EXTEND_REALMS_LINK.to_string();
        };
        let profile_id = profile_id.unwrap_or_else(|| {
            panic!("java.lang.NullPointerException: Cannot invoke \"java.util.UUID.toString()\" because \"uuid\" is null")
        });
        format!(
            "{}?subscriptionId={subscription_id}&profileId={}",
            Self::EXTEND_REALMS_LINK,
            profile_id.to_undashed_string()
        )
    }
}
