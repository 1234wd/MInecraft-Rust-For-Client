//! Port of: com/mojang/blaze3d/vulkan/glsl/ShaderCompileException.java
//! Java class(es): com.mojang.blaze3d.vulkan.glsl.ShaderCompileException
//! Status: VERIFIED
//!
//! Session 22. `public class ShaderCompileException extends Exception`, with one `(String message)` constructor and no
//! other members.
//!
//! CHECKED (`extends Exception`, not `RuntimeException`): the GLSL-to-SPIR-V path declares it,
//! so its callers handle it rather than letting it unwind. That is the case this value type exists
//! for -- such a caller returns it in a `Result` instead of calling [`throw`].
//!
//! # Port shape
//!
//! A value type carrying the (nullable) message, so a caller can either throw it with [`throw`]
//! -- a panic whose text is Java's `toString`, per `DESIGN_DECISIONS.md#runtime-exceptions` --
//! or, where vanilla catches it and carries on, return it in a `Result`. `Display` is
//! `Throwable#toString`; see [`crate::javacompat::throwable`].
//!
//! Verified by the `EXC` group of `small_probe.txt` (`tests/session22.rs`): class name,
//! superclass, checked/unchecked, `getMessage()` and `toString()` for a plain, an empty, a null,
//! a control-character and a non-ASCII message.

use crate::javacompat::throwable;

/// `ShaderCompileException.class.getName()`.
pub const CLASS_NAME: &str = "com.mojang.blaze3d.vulkan.glsl.ShaderCompileException";

/// The direct Java superclass. Checked: not a `RuntimeException`.
pub const SUPERCLASS: &str = "java.lang.Exception";

/// Port of `com.mojang.blaze3d.vulkan.glsl.ShaderCompileException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShaderCompileException {
    message: Option<String>,
}

impl ShaderCompileException {
    /// Port of `new ShaderCompileException(String message)`. Java accepts a null message, and so does this.
    pub fn new(message: Option<&str>) -> Self {
        ShaderCompileException {
            message: message.map(str::to_string),
        }
    }

    /// Port of `Throwable#getMessage()`.
    pub fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl std::fmt::Display for ShaderCompileException {
    /// Port of `Throwable#toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&throwable::to_string(CLASS_NAME, self.get_message()))
    }
}

impl std::error::Error for ShaderCompileException {}

/// `throw new ShaderCompileException(message)`: panics with Java's `toString`.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("{}", ShaderCompileException::new(Some(message)))
}
