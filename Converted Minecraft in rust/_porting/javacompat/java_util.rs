//! Narrow `java.util.UUID` support for Minecraft's authlib `UndashedUuid.toString` call.

/// A Java UUID represented by its two signed 64-bit fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid {
    most_significant_bits: i64,
    least_significant_bits: i64,
}

impl Uuid {
    /// `new UUID(mostSignificantBits, leastSignificantBits)`.
    pub const fn from_bits(most_significant_bits: i64, least_significant_bits: i64) -> Self {
        Self {
            most_significant_bits,
            least_significant_bits,
        }
    }

    /// `UndashedUuid.toString(UUID)`: Java's lowercase UUID hex with the hyphens removed.
    pub fn to_undashed_string(self) -> String {
        let bits = ((self.most_significant_bits as u64 as u128) << 64)
            | self.least_significant_bits as u64 as u128;
        format!("{bits:032x}")
    }
}
