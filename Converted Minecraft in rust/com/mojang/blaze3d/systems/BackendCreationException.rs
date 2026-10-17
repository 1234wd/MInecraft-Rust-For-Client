//! Port of: com/mojang/blaze3d/systems/BackendCreationException.java
//! Java class(es): com.mojang.blaze3d.systems.BackendCreationException, com.mojang.blaze3d.systems.Reason
//! Status: VERIFIED
//!
//! Session 22. `public class BackendCreationException extends Exception` -- a message, a
//! [`Reason`] and an immutable list of missing capabilities -- with its nested `enum Reason`.
//!
//! CHECKED (`extends Exception`). `GlBackend` and `VulkanBackend` throw it when a graphics
//! backend cannot be created, and the caller catches it to try the next backend, so it is a value
//! for a `Result` far more often than something to [`throw`].
//!
//! # `List.copyOf`
//!
//! The three-argument constructor stores `List.copyOf(missingCapabilities)`, which matters twice:
//!
//! * it is a COPY, so a later change to the caller's list is not seen (the probe adds `"c"` to
//!   the source list after construction and the exception still reports `a,b`);
//! * it is IMMUTABLE, so `getMissingCapabilities().add(..)` throws -- here the accessor returns a
//!   slice, which cannot be pushed to at all;
//! * it rejects null elements with a `NullPointerException`. A Rust `String` cannot be null, so
//!   that failure has no Rust counterpart; the probe pins that Java does throw.
//!
//! The two-argument constructor passes `List.of()`: empty, immutable.
//!
//! Verified by the `EXC`, `BCE` and `BCREASON` groups of `small_probe.txt`
//! (`tests/session22.rs`): every `Reason` constant with its ordinal and display name, both
//! constructors (one with a null message), the copy, the immutability and the null-element NPE.

use crate::javacompat::throwable;

/// `BackendCreationException.class.getName()`.
pub const CLASS_NAME: &str = "com.mojang.blaze3d.systems.BackendCreationException";

/// The direct Java superclass. Checked: not a `RuntimeException`.
pub const SUPERCLASS: &str = "java.lang.Exception";

/// Port of `com.mojang.blaze3d.systems.BackendCreationException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendCreationException {
    message: Option<String>,
    reason: Reason,
    missing_capabilities: Vec<String>,
}

impl BackendCreationException {
    /// Port of `new BackendCreationException(String message, Reason reason, List<String> missingCapabilities)`.
    ///
    /// Copies the capabilities, as `List.copyOf` does.
    pub fn new(message: Option<&str>, reason: Reason, missing_capabilities: &[String]) -> Self {
        BackendCreationException {
            message: message.map(str::to_string),
            reason,
            missing_capabilities: missing_capabilities.to_vec(),
        }
    }

    /// Port of `new BackendCreationException(String message, Reason reason)`: no missing capabilities.
    pub fn new_without_capabilities(message: Option<&str>, reason: Reason) -> Self {
        BackendCreationException::new(message, reason, &[])
    }

    /// Port of `getReason()`.
    pub fn get_reason(&self) -> Reason {
        self.reason
    }

    /// Port of `getMissingCapabilities()`. Read-only, as Java's immutable list is.
    pub fn get_missing_capabilities(&self) -> &[String] {
        &self.missing_capabilities
    }

    /// Port of `Throwable#getMessage()`.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl std::fmt::Display for BackendCreationException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for BackendCreationException {}

/// `throw new BackendCreationException(message, reason, missingCapabilities)`.
#[cold]
#[inline(never)]
pub fn throw(message: &str, reason: Reason, missing_capabilities: &[String]) -> ! {
    panic!("{}", BackendCreationException::new(Some(message), reason, missing_capabilities))
}

/// Port of `BackendCreationException.Reason`. Declaration order is the ordinal order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reason {
    /// `GLFW_ERROR("glfw_error")` -- ordinal 0.
    GlfwError = 0,
    /// `VULKAN_LOADER_MISSING("vulkan_loader_missing")` -- ordinal 1.
    VulkanLoaderMissing = 1,
    /// `VULKAN_INSTANCE_CREATION_FAILED("vulkan_instance_creation_failed")` -- ordinal 2.
    VulkanInstanceCreationFailed = 2,
    /// `VULKAN_NO_DEVICE("vulkan_no_device")` -- ordinal 3.
    VulkanNoDevice = 3,
    /// `VULKAN_DEVICE_VERSION_TOO_LOW("vulkan_device_version_too_low")` -- ordinal 4.
    VulkanDeviceVersionTooLow = 4,
    /// `VULKAN_NO_GRAPHICS_QUEUE("vulkan_no_graphics_queue")` -- ordinal 5.
    VulkanNoGraphicsQueue = 5,
    /// `VULKAN_MISSING_EXTENSION("vulkan_missing_extension")` -- ordinal 6.
    VulkanMissingExtension = 6,
    /// `VULKAN_MISSING_FEATURE("vulkan_missing_feature")` -- ordinal 7.
    VulkanMissingFeature = 7,
    /// `OPENGL_MISSING("opengl_missing")` -- ordinal 8.
    OpenglMissing = 8,
    /// `OTHER("other")` -- ordinal 9.
    Other = 9,
}

impl Reason {
    /// Java: `Reason.values()`, in declaration order.
    pub const VALUES: [Reason; 10] = [
        Reason::GlfwError,
        Reason::VulkanLoaderMissing,
        Reason::VulkanInstanceCreationFailed,
        Reason::VulkanNoDevice,
        Reason::VulkanDeviceVersionTooLow,
        Reason::VulkanNoGraphicsQueue,
        Reason::VulkanMissingExtension,
        Reason::VulkanMissingFeature,
        Reason::OpenglMissing,
        Reason::Other,
    ];

    /// Java: `ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Reason::GlfwError => "GLFW_ERROR",
            Reason::VulkanLoaderMissing => "VULKAN_LOADER_MISSING",
            Reason::VulkanInstanceCreationFailed => "VULKAN_INSTANCE_CREATION_FAILED",
            Reason::VulkanNoDevice => "VULKAN_NO_DEVICE",
            Reason::VulkanDeviceVersionTooLow => "VULKAN_DEVICE_VERSION_TOO_LOW",
            Reason::VulkanNoGraphicsQueue => "VULKAN_NO_GRAPHICS_QUEUE",
            Reason::VulkanMissingExtension => "VULKAN_MISSING_EXTENSION",
            Reason::VulkanMissingFeature => "VULKAN_MISSING_FEATURE",
            Reason::OpenglMissing => "OPENGL_MISSING",
            Reason::Other => "OTHER",
        }
    }

    /// Port of `Reason#displayName()`: the constructor's `key`, stored as `displayName`.
    pub const fn display_name(&self) -> &'static str {
        match self {
            Reason::GlfwError => "glfw_error",
            Reason::VulkanLoaderMissing => "vulkan_loader_missing",
            Reason::VulkanInstanceCreationFailed => "vulkan_instance_creation_failed",
            Reason::VulkanNoDevice => "vulkan_no_device",
            Reason::VulkanDeviceVersionTooLow => "vulkan_device_version_too_low",
            Reason::VulkanNoGraphicsQueue => "vulkan_no_graphics_queue",
            Reason::VulkanMissingExtension => "vulkan_missing_extension",
            Reason::VulkanMissingFeature => "vulkan_missing_feature",
            Reason::OpenglMissing => "opengl_missing",
            Reason::Other => "other",
        }
    }
}

impl std::fmt::Display for Reason {
    /// Java: `Enum#toString()`, which `Reason` does not override, is `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
