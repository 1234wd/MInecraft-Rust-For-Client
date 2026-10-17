//! Port of: net/minecraft/util/profiling/ResultField.java
//! Java class(es): net.minecraft.util.profiling.ResultField
//! Status: VERIFIED
//!
//! Session 22. `public final class ResultField implements Comparable<ResultField>`: one row of a
//! profiler report -- a name, two percentages and a call count, as public final fields.
//!
//! # `compareTo` returns a STRING difference, not just a sign
//!
//! ```java
//! if (resultField.percentage < this.percentage) return -1;
//! return resultField.percentage > this.percentage ? 1 : resultField.name.compareTo(this.name);
//! ```
//!
//! Highest percentage first; a tie falls through to `String#compareTo` of the OTHER name against
//! this one, whose value is the UTF-16 code-unit difference (e.g. `117` between `"\u{e9}"` and
//! `"tick"`), returned as is -- `javacompat::java_lang::string_compare_to`. A NaN percentage makes
//! both `<` and `>` false, so it ties with everything and the names decide.
//!
//! `getColor()` is `(name.hashCode() & 0xAAAAAA) + 0xFF444444` (written as `11184810` and
//! `-12303292` in the source): bright channels from the name's hash on a grey base.
//!
//! Because `compareTo` is not a total order over NaN and is not consistent with any `equals` (the
//! class has none), the port exposes [`ResultField::compare_to`] and implements no `Ord`.
//!
//! Verified by the `RESULTFIELD` group of `small_probe.txt` (`tests/session22.rs`): nine fields
//! (equal, NaN, `-0.0`, empty and non-ASCII names), the full `compareTo` matrix and `getColor`.

use crate::javacompat::java_lang;

/// Port of `net.minecraft.util.profiling.ResultField`.
#[derive(Debug, Clone)]
pub struct ResultField {
    /// Port of `public final double percentage`.
    pub percentage: f64,
    /// Port of `public final double globalPercentage`.
    pub global_percentage: f64,
    /// Port of `public final long count`.
    pub count: i64,
    /// Port of `public final String name`.
    pub name: String,
}

impl ResultField {
    /// Port of `new ResultField(String name, double percentage, double globalPercentage, long count)`.
    pub fn new(name: &str, percentage: f64, global_percentage: f64, count: i64) -> Self {
        ResultField {
            percentage,
            global_percentage,
            count,
            name: name.to_string(),
        }
    }

    /// Port of `compareTo(ResultField resultField)`.
    pub fn compare_to(&self, result_field: &ResultField) -> i32 {
        if result_field.percentage < self.percentage {
            -1
        } else if result_field.percentage > self.percentage {
            1
        } else {
            java_lang::string_compare_to(&result_field.name, &self.name)
        }
    }

    /// Port of `getColor()`: `(name.hashCode() & 11184810) + -12303292`.
    pub fn get_color(&self) -> i32 {
        (java_lang::string_hash_code(&self.name) & 11_184_810).wrapping_add(-12_303_292)
    }
}
