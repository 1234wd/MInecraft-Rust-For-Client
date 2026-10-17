//! Port of: com/mojang/realmsclient/gui/screens/UploadResult.java
//! Java class(es): com.mojang.realmsclient.gui.screens.UploadResult, com.mojang.realmsclient.gui.screens.Builder
//! Status: VERIFIED
//!
//! Session 22. `public record UploadResult(int statusCode, @Nullable String errorMessage)` with
//! one reader and a nested mutable `Builder`.
//!
//! # `getSimplifiedErrorMessage`
//!
//! ```java
//! if (statusCode >= 200 && statusCode < 300) return null;
//! return statusCode == 400 && errorMessage != null ? errorMessage : String.valueOf(statusCode);
//! ```
//!
//! Only a 400 surfaces the server's text, and it surfaces it even when EMPTY; every other failure
//! is just the code as a string. A 2xx is success and has no message, whatever `errorMessage` says.
//!
//! # `Builder`
//!
//! Starts at `statusCode = -1`, `errorMessage = null`; each `with*` mutates and returns the SAME
//! builder (`&mut Self` here), and `build()` snapshots it, so one builder can build several results.
//!
//! # Record members
//!
//! `equals` is derived (an `Option<String>` component compares as `Objects.equals`), `hashCode`
//! is [`UploadResult::java_hash_code`] (null message hashes to `0`), `toString` prints a null
//! message as `null`.
//!
//! Verified by the `REC`, `RECEQ`, `UPLOAD` and `UPBUILD` groups of `small_probe.txt`
//! (`tests/session22.rs`): fourteen status codes around every boundary, each with a null, an empty
//! and a non-empty message, and the builder's default, chained and reset states.

use crate::javacompat::java_lang;
use crate::javacompat::java_object;

/// Port of `com.mojang.realmsclient.gui.screens.UploadResult`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UploadResult {
    status_code: i32,
    error_message: Option<String>,
}

impl UploadResult {
    /// Port of the canonical constructor. `None` is a Java `null` message.
    pub fn new(status_code: i32, error_message: Option<&str>) -> Self {
        UploadResult {
            status_code,
            error_message: error_message.map(str::to_string),
        }
    }

    /// Port of the accessor `statusCode()`.
    pub const fn status_code(&self) -> i32 {
        self.status_code
    }

    /// Port of the accessor `errorMessage()`.
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// Port of `getSimplifiedErrorMessage()`. `None` is a Java `null`.
    pub fn get_simplified_error_message(&self) -> Option<String> {
        if self.status_code >= 200 && self.status_code < 300 {
            return None;
        }
        match (&self.error_message, self.status_code) {
            (Some(message), 400) => Some(message.clone()),
            _ => Some(self.status_code.to_string()),
        }
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            self.status_code,
            java_object::hash_opt(self.error_message.as_ref(), |s| java_lang::string_hash_code(s)),
        ])
    }
}

impl std::fmt::Display for UploadResult {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UploadResult[statusCode={}, errorMessage={}]",
            self.status_code,
            java_object::str_or_null(self.error_message.as_deref())
        )
    }
}

/// Port of `UploadResult.Builder`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Builder {
    status_code: i32,
    error_message: Option<String>,
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

impl Builder {
    /// Port of `new UploadResult.Builder()`: `statusCode = -1`, `errorMessage = null`.
    pub const fn new() -> Self {
        Builder {
            status_code: -1,
            error_message: None,
        }
    }

    /// Port of `withStatusCode(int statusCode)`: mutates and returns the same builder.
    pub fn with_status_code(&mut self, status_code: i32) -> &mut Self {
        self.status_code = status_code;
        self
    }

    /// Port of `withErrorMessage(@Nullable String errorMessage)`: mutates and returns the same builder.
    pub fn with_error_message(&mut self, error_message: Option<&str>) -> &mut Self {
        self.error_message = error_message.map(str::to_string);
        self
    }

    /// Port of `build()`.
    pub fn build(&self) -> UploadResult {
        UploadResult::new(self.status_code, self.error_message.as_deref())
    }
}
