//! Port of: com/mojang/blaze3d/systems/DeviceInfo.java
//! Java class(es): com.mojang.blaze3d.systems.DeviceInfo
//! Status: PARTIAL
//!
//! Session 24. `public record DeviceInfo(String name, String vendorName, String driverInfo, boolean
//! isZZeroToOne, String backendName, float timestampPeriod, DeviceLimits limits, DeviceFeatures
//! features, Set<String> underlyingExtensions, HintsAndWorkarounds hintsAndWorkarounds, DeviceType
//! type)`: what the GPU backend found out about the device. Components only, no body.
//!
//! # The record members
//!
//! * `equals` compares each component as the record does: the strings by `String.equals`, the
//!   `float` by `Float.compare` (NaN equals NaN, `0.0` differs from `-0.0` --
//!   `java_object::equals_f32`), the three records by their own `equals`, the enum by identity,
//!   and the extensions by `Set.equals` -- the same elements, ORDER IGNORED;
//! * `toString` is `DeviceInfo[name=.., ..., type=..]`: the float by `Float.toString`
//!   (`java_lang::float_to_string`), the records by their own `toString`, and the extension set by
//!   `AbstractCollection.toString` -- `[a, b]` in the SET'S iteration order;
//! * `hashCode` folds `DeviceType`'s identity hash (an enum), so it is not ported (OPEN_QUESTIONS
//!   #30).
//!
//! # The extension set
//!
//! The record keeps whatever `Set` its caller built (`GlHeuristics` passes an unmodifiable view of
//! the enabled-extension set, `VulkanDevice` its own), and its iteration order is that set's. The
//! port keeps the elements in the order the caller iterates them ([`DeviceInfo::new`] takes them
//! in that order) and compares them as a set. A repeated element is a set's single element: the
//! constructor keeps its first occurrence.
//!
//! The strings are non-null here; Java would accept (and print) `null`. Neither backend passes one.
//!
//! Verified by the `DEVINFO` group of `sweep_probe.txt` (`tests/session24.rs`).

use crate::com::mojang::blaze3d::systems::DeviceFeatures::DeviceFeatures;
use crate::com::mojang::blaze3d::systems::DeviceLimits::DeviceLimits;
use crate::com::mojang::blaze3d::systems::DeviceType::DeviceType;
use crate::com::mojang::blaze3d::systems::HintsAndWorkarounds::HintsAndWorkarounds;
use crate::javacompat::java_lang::float_to_string::float_to_string;
use crate::javacompat::java_object;

/// Port of `com.mojang.blaze3d.systems.DeviceInfo`.
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    name: String,
    vendor_name: String,
    driver_info: String,
    is_z_zero_to_one: bool,
    backend_name: String,
    timestamp_period: f32,
    limits: DeviceLimits,
    features: DeviceFeatures,
    underlying_extensions: Vec<String>,
    hints_and_workarounds: HintsAndWorkarounds,
    type_: DeviceType,
}

impl DeviceInfo {
    /// Port of the canonical constructor. `underlying_extensions` is the caller's `Set`, in its
    /// iteration order.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl Into<String>,
        vendor_name: impl Into<String>,
        driver_info: impl Into<String>,
        is_z_zero_to_one: bool,
        backend_name: impl Into<String>,
        timestamp_period: f32,
        limits: DeviceLimits,
        features: DeviceFeatures,
        underlying_extensions: impl IntoIterator<Item = String>,
        hints_and_workarounds: HintsAndWorkarounds,
        type_: DeviceType,
    ) -> Self {
        let mut extensions: Vec<String> = Vec::new();
        for e in underlying_extensions {
            if !extensions.contains(&e) {
                extensions.push(e);
            }
        }
        DeviceInfo {
            name: name.into(),
            vendor_name: vendor_name.into(),
            driver_info: driver_info.into(),
            is_z_zero_to_one,
            backend_name: backend_name.into(),
            timestamp_period,
            limits,
            features,
            underlying_extensions: extensions,
            hints_and_workarounds,
            type_,
        }
    }

    /// Port of the record accessor `String name()`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Port of the record accessor `String vendorName()`.
    pub fn vendor_name(&self) -> &str {
        &self.vendor_name
    }

    /// Port of the record accessor `String driverInfo()`.
    pub fn driver_info(&self) -> &str {
        &self.driver_info
    }

    /// Port of the record accessor `boolean isZZeroToOne()`.
    pub fn is_z_zero_to_one(&self) -> bool {
        self.is_z_zero_to_one
    }

    /// Port of the record accessor `String backendName()`.
    pub fn backend_name(&self) -> &str {
        &self.backend_name
    }

    /// Port of the record accessor `float timestampPeriod()`.
    pub fn timestamp_period(&self) -> f32 {
        self.timestamp_period
    }

    /// Port of the record accessor `DeviceLimits limits()`.
    pub fn limits(&self) -> &DeviceLimits {
        &self.limits
    }

    /// Port of the record accessor `DeviceFeatures features()`.
    pub fn features(&self) -> &DeviceFeatures {
        &self.features
    }

    /// Port of the record accessor `Set<String> underlyingExtensions()`, in the set's iteration
    /// order.
    pub fn underlying_extensions(&self) -> &[String] {
        &self.underlying_extensions
    }

    /// `underlyingExtensions().contains(extension)`.
    pub fn has_underlying_extension(&self, extension: &str) -> bool {
        self.underlying_extensions.iter().any(|e| e == extension)
    }

    /// Port of the record accessor `HintsAndWorkarounds hintsAndWorkarounds()`.
    pub fn hints_and_workarounds(&self) -> &HintsAndWorkarounds {
        &self.hints_and_workarounds
    }

    /// Port of the record accessor `DeviceType type()`.
    pub fn type_(&self) -> DeviceType {
        self.type_
    }
}

impl PartialEq for DeviceInfo {
    /// Port of the record's `equals`; the extensions compare as a `Set`.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.vendor_name == other.vendor_name
            && self.driver_info == other.driver_info
            && self.is_z_zero_to_one == other.is_z_zero_to_one
            && self.backend_name == other.backend_name
            && java_object::equals_f32(self.timestamp_period, other.timestamp_period)
            && self.limits == other.limits
            && self.features == other.features
            && self.underlying_extensions.len() == other.underlying_extensions.len()
            && self
                .underlying_extensions
                .iter()
                .all(|e| other.has_underlying_extension(e))
            && self.hints_and_workarounds == other.hints_and_workarounds
            && self.type_ == other.type_
    }
}

impl Eq for DeviceInfo {}

impl std::fmt::Display for DeviceInfo {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeviceInfo[name={}, vendorName={}, driverInfo={}, isZZeroToOne={}, backendName={}, \
             timestampPeriod={}, limits={}, features={}, underlyingExtensions=[{}], \
             hintsAndWorkarounds={}, type={}]",
            self.name,
            self.vendor_name,
            self.driver_info,
            self.is_z_zero_to_one,
            self.backend_name,
            float_to_string(self.timestamp_period),
            self.limits,
            self.features,
            self.underlying_extensions.join(", "),
            self.hints_and_workarounds,
            self.type_
        )
    }
}
