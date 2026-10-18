//! Port of: net/minecraft/gametest/framework/RetryOptions.java
//! Java class(es): net.minecraft.gametest.framework.RetryOptions
//! Status: VERIFIED
//!
//! Session 22. `public record RetryOptions(int numberOfTries, boolean haltOnFailure)` with a
//! shared `NO_RETRIES` and three predicates.
//!
//! # Any count below 1 means "unlimited"
//!
//! `unlimitedTries()` is `numberOfTries < 1`, so `0`, `-1` and `Integer.MIN_VALUE` all retry
//! forever, and `hasRetries()` is `numberOfTries != 1` -- true for those too.
//! `hasTriesLeft(attempts, successes)` treats ANY difference between the two counts as a failure:
//!
//! ```java
//! boolean hasFailures = attempts != successes;
//! boolean hasMoreAttempts = this.unlimitedTries() || attempts < this.numberOfTries;
//! return hasMoreAttempts && (!hasFailures || !this.haltOnFailure);
//! ```
//!
//! # Record members
//!
//! `equals` is derived, `hashCode` is [`RetryOptions::java_hash_code`]
//! (`31 * numberOfTries + Boolean.hashCode(haltOnFailure)`), `toString` is
//! `RetryOptions[numberOfTries=<n>, haltOnFailure=<b>]`.
//!
//! Verified by the `REC`, `RECEQ` and `RETRY` groups of `small_probe.txt` (`tests/session22.rs`):
//! every predicate for seven try counts (including `MIN_VALUE`, `0` and `MAX_VALUE`), both halt
//! settings, and a 6x4 grid of attempt/success counts.

use crate::javacompat::java_object;

/// Port of `net.minecraft.gametest.framework.RetryOptions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetryOptions {
    number_of_tries: i32,
    halt_on_failure: bool,
}

impl RetryOptions {
    /// Port of `private static final RetryOptions NO_RETRIES = new RetryOptions(1, true)`.
    const NO_RETRIES: RetryOptions = RetryOptions::new(1, true);

    /// Port of the canonical constructor.
    pub const fn new(number_of_tries: i32, halt_on_failure: bool) -> Self {
        RetryOptions {
            number_of_tries,
            halt_on_failure,
        }
    }

    /// Port of `noRetries()`.
    pub const fn no_retries() -> RetryOptions {
        Self::NO_RETRIES
    }

    /// Port of the accessor `numberOfTries()`.
    pub const fn number_of_tries(&self) -> i32 {
        self.number_of_tries
    }

    /// Port of the accessor `haltOnFailure()`.
    pub const fn halt_on_failure(&self) -> bool {
        self.halt_on_failure
    }

    /// Port of `unlimitedTries()`.
    pub const fn unlimited_tries(&self) -> bool {
        self.number_of_tries < 1
    }

    /// Port of `hasTriesLeft(int attempts, int successes)`.
    pub const fn has_tries_left(&self, attempts: i32, successes: i32) -> bool {
        let has_failures = attempts != successes;
        let has_more_attempts = self.unlimited_tries() || attempts < self.number_of_tries;
        has_more_attempts && (!has_failures || !self.halt_on_failure)
    }

    /// Port of `hasRetries()`.
    pub const fn has_retries(&self) -> bool {
        self.number_of_tries != 1
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            self.number_of_tries,
            java_object::hash_bool(self.halt_on_failure),
        ])
    }
}

impl std::fmt::Display for RetryOptions {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RetryOptions[numberOfTries={}, haltOnFailure={}]",
            self.number_of_tries, self.halt_on_failure
        )
    }
}
