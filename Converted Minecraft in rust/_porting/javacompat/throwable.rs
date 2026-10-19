//! Port of: (no Java counterpart -- see module docs)
//! Java class(es): `java.lang.Throwable` (`toString`, the `Throwable(Throwable)` message rule)
//! Status: VERIFIED
//!
//! Session 22. The two `Throwable` rules every ported exception class needs, kept in one place
//! so that nineteen small exception files cannot each get them slightly wrong.
//!
//! Checked against JDK 25 by the `EXC` group of `_porting/test-data/small_probe.txt`
//! (`_porting/tests/session22.rs`): every exception class ported in session 22 is constructed
//! in Java and in Rust with the same arguments, and `getMessage()` / `toString()` are compared
//! as strings.
//!
//! # `Throwable#toString`
//!
//! ```java
//! String s = getClass().getName();
//! String message = getLocalizedMessage();
//! return (message != null) ? (s + ": " + message) : s;
//! ```
//!
//! A `null` message prints the bare class name -- **no** trailing `": "` and no `"null"`. An
//! EMPTY message is not null: `new X("")` prints `"<class>: "`, with the separator and nothing
//! after it. The probe has a row for each, because both are easy to "tidy".
//!
//! # `Throwable(Throwable cause)` takes its message FROM the cause
//!
//! ```java
//! public Throwable(Throwable cause) {
//!     fillInStackTrace();
//!     detailMessage = (cause == null ? null : cause.toString());
//!     this.cause = cause;
//! }
//! ```
//!
//! So `new CryptException(new RuntimeException("x")).getMessage()` is
//! `"java.lang.RuntimeException: x"` -- the cause's whole `toString`, class name included --
//! and the outer `toString` therefore names two classes. [`message_from_cause`] is that rule.
//!
//! # What a cause is, in Rust
//!
//! [`Cause`] is a boxed [`std::error::Error`]. Its [`Display`](std::fmt::Display) must be the
//! cause's Java `toString`, which every exception struct ported in session 22 satisfies by
//! construction (their `Display` *is* [`to_string`]). A foreign Rust error used as a cause
//! keeps whatever `Display` it has; there is no Java class to be faithful to in that case.

use std::error::Error;
use std::sync::{Arc, Mutex};

/// A Java `Throwable` held as a cause. `Display` must print the cause's Java `toString`.
pub type Cause = Box<dyn Error + Send + Sync + 'static>;

/// A reference-counted Java throwable object for ports that retain, suppress, and rethrow the
/// same exception instance.
pub type ThrowableRef = Arc<JavaThrowable>;

/// The `Throwable` state required by `ExceptionCollector` and its suppression behavior.
pub struct JavaThrowable {
    class_name: String,
    message: Option<String>,
    cause: Option<ThrowableRef>,
    suppression_enabled: bool,
    suppressed: Mutex<Vec<ThrowableRef>>,
}

impl JavaThrowable {
    /// A Java throwable with the supplied runtime class name and nullable message.
    pub fn new(class_name: impl Into<String>, message: Option<String>) -> ThrowableRef {
        Self::with_options(class_name, message, None, true)
    }

    /// A Java throwable with an explicit cause and suppression setting.
    pub fn with_options(
        class_name: impl Into<String>,
        message: Option<String>,
        cause: Option<ThrowableRef>,
        suppression_enabled: bool,
    ) -> ThrowableRef {
        Arc::new(Self {
            class_name: class_name.into(),
            message,
            cause,
            suppression_enabled,
            suppressed: Mutex::new(Vec::new()),
        })
    }

    /// The class name returned by `getClass().getName()`.
    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    /// The nullable message returned by `getMessage()`.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Java `Throwable.toString()`.
    pub fn java_to_string(&self) -> String {
        to_string(&self.class_name, self.message.as_deref())
    }

    /// The cause reference, if any.
    pub fn cause(&self) -> Option<ThrowableRef> {
        self.cause.clone()
    }

    /// A defensive snapshot of Java `getSuppressed()`.
    pub fn suppressed(&self) -> Vec<ThrowableRef> {
        if !self.suppression_enabled {
            return Vec::new();
        }
        self.suppressed
            .lock()
            .expect("Throwable suppression lock poisoned")
            .clone()
    }

    /// Java `addSuppressed(Throwable)`, including self-suppression and null checks.
    pub fn add_suppressed(
        primary: &ThrowableRef,
        exception: Option<ThrowableRef>,
    ) -> Result<(), ThrowableRef> {
        let Some(exception) = exception else {
            return Err(Self::new(
                "java.lang.NullPointerException",
                Some("Cannot suppress a null exception.".to_string()),
            ));
        };

        if Arc::ptr_eq(primary, &exception) {
            return Err(Self::with_options(
                "java.lang.IllegalArgumentException",
                Some("Self-suppression not permitted".to_string()),
                Some(primary.clone()),
                true,
            ));
        }

        if primary.suppression_enabled {
            primary
                .suppressed
                .lock()
                .expect("Throwable suppression lock poisoned")
                .push(exception);
        }
        Ok(())
    }
}

/// Port of `Throwable#toString()` for a class name and a (possibly null) message.
pub fn to_string(class_name: &str, message: Option<&str>) -> String {
    match message {
        Some(m) => format!("{class_name}: {m}"),
        None => class_name.to_string(),
    }
}

/// The `Throwable(Throwable cause)` rule: the message is `cause.toString()`, or null.
pub fn message_from_cause(cause: Option<&Cause>) -> Option<String> {
    cause.map(|c| c.to_string())
}

/// `Throwable#getCause()` as a [`std::error::Error::source`] value.
pub fn source_of(cause: &Option<Cause>) -> Option<&(dyn Error + 'static)> {
    cause.as_ref().map(|c| c.as_ref() as &(dyn Error + 'static))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Fake(&'static str);
    impl std::fmt::Display for Fake {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0)
        }
    }
    impl Error for Fake {}

    #[test]
    fn null_message_is_the_bare_class_name_and_empty_message_keeps_the_separator() {
        assert_eq!(to_string("a.B", None), "a.B");
        assert_eq!(to_string("a.B", Some("")), "a.B: ");
        assert_eq!(to_string("a.B", Some("m")), "a.B: m");
    }

    #[test]
    fn cause_message_is_the_whole_cause_to_string() {
        let c: Cause = Box::new(Fake("java.lang.RuntimeException: x"));
        assert_eq!(
            message_from_cause(Some(&c)).as_deref(),
            Some("java.lang.RuntimeException: x")
        );
        assert_eq!(message_from_cause(None), None);
    }
}
