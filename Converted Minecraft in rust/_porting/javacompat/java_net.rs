//! Narrow `java.net.URI` value support for the fixed public links in `CommonLinks`.

/// An immutable URI whose spelling is a Java `URI.toString()` value.
///
/// This helper covers the checked-in constant URIs only; it does not parse arbitrary URI input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uri(&'static str);

impl Uri {
    /// Builds one of the fixed, Java-validated URI constants.
    pub const fn from_static(value: &'static str) -> Self {
        Self(value)
    }

    /// The spelling returned by Java `URI.toString()` for these constants.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}
