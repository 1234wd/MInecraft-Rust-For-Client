//! Port of: com/mojang/realmsclient/dto/ServiceQuality.java
//! Java class(es): com.mojang.realmsclient.dto.ServiceQuality, com.mojang.realmsclient.dto.RealmsServiceQualityJsonAdapter
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, `getValue`, `getIcon`, `byValue`. PORT-BLOCKED on Gson: the nested
//! `RealmsServiceQualityJsonAdapter` (its decode rule is "`byValue`, else log a warning and use
//! `UNKNOWN`"). The constant dump has no group for this class, so names/ordinals come from the
//! source.
//!
//! `value` starts at 1, not 0, and `UNKNOWN` (5) is a real value the service can send; `byValue(0)`
//! is `null`.

use crate::net::minecraft::resources::Identifier::{with_default_namespace, Identifier};

/// Port of `com.mojang.realmsclient.dto.ServiceQuality`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ServiceQuality {
    /// `GREAT` -- ordinal 0.
    Great = 0,
    /// `GOOD` -- ordinal 1.
    Good = 1,
    /// `OKAY` -- ordinal 2.
    Okay = 2,
    /// `POOR` -- ordinal 3.
    Poor = 3,
    /// `UNKNOWN` -- ordinal 4.
    Unknown = 4,
}

/// Java: `ServiceQuality.values()`, declaration order.
pub const VALUES: [ServiceQuality; 5] = [
    ServiceQuality::Great,
    ServiceQuality::Good,
    ServiceQuality::Okay,
    ServiceQuality::Poor,
    ServiceQuality::Unknown,
];

impl ServiceQuality {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ServiceQuality::Great => "GREAT",
            ServiceQuality::Good => "GOOD",
            ServiceQuality::Okay => "OKAY",
            ServiceQuality::Poor => "POOR",
            ServiceQuality::Unknown => "UNKNOWN",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ServiceQuality#getValue()`.
    pub const fn get_value(&self) -> i32 {
        match self {
            ServiceQuality::Great => 1,
            ServiceQuality::Good => 2,
            ServiceQuality::Okay => 3,
            ServiceQuality::Poor => 4,
            ServiceQuality::Unknown => 5,
        }
    }

    /// The path `icon` is built from with `Identifier.withDefaultNamespace`.
    pub const fn icon_path(&self) -> &'static str {
        match self {
            ServiceQuality::Great => "icon/ping_5",
            ServiceQuality::Good => "icon/ping_4",
            ServiceQuality::Okay => "icon/ping_3",
            ServiceQuality::Poor => "icon/ping_2",
            ServiceQuality::Unknown => "icon/ping_unknown",
        }
    }

    /// Port of `ServiceQuality#getIcon()`.
    pub fn get_icon(&self) -> Identifier {
        with_default_namespace(self.icon_path())
    }

    /// Port of `ServiceQuality#byValue(int)`: first constant in declaration order whose value
    /// matches, `None` (Java `null`) otherwise.
    pub fn by_value(value: i32) -> Option<ServiceQuality> {
        VALUES.iter().copied().find(|q| q.get_value() == value)
    }
}

impl std::fmt::Display for ServiceQuality {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Java: `public static class RealmsServiceQualityJsonAdapter extends TypeAdapter<ServiceQuality>`.
///
/// PORT-BLOCKED on `com.google.gson.TypeAdapter` / `JsonReader` / `JsonWriter`.
pub fn RealmsServiceQualityJsonAdapter() {
    todo!("PORT-BLOCKED: com.google.gson.TypeAdapter is not ported")
}
