//! Port of: net/minecraft/client/gui/font/FontOption.java
//! Java class(es): net.minecraft.client.gui.font.FontOption, net.minecraft.client.gui.font.Filter
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, serialized names, and the nested `FontOption.Filter`
//! (`apply`, `merge`, `ALWAYS_PASS`). PORT-BLOCKED on DataFixerUpper: `CODEC` and `Filter.CODEC`.
//!
//! # `Filter.apply` IS AN EXACT-MATCH TEST ON THE LISTED OPTIONS ONLY
//!
//! Every entry `(option, wanted)` must satisfy `options.contains(option) == wanted`; options not
//! mentioned in the filter are ignored. So `{jp: false}` REJECTS a font when Japanese variants are
//! on, it does not merely "not require" them.
//!
//! # `merge` IS LEFT-BIASED
//!
//! `new HashMap(other.values)` then `putAll(this.values)`: on a conflicting key, THIS filter's value
//! wins over `other`'s.

/// Port of `net.minecraft.client.gui.font.FontOption`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontOption {
    /// `UNIFORM` -- ordinal 0.
    Uniform = 0,
    /// `JAPANESE_VARIANTS` -- ordinal 1.
    JapaneseVariants = 1,
}

/// Java: `FontOption.values()`, declaration order.
pub const VALUES: [FontOption; 2] = [
    FontOption::Uniform,
    FontOption::JapaneseVariants,
];

impl FontOption {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            FontOption::Uniform => "UNIFORM",
            FontOption::JapaneseVariants => "JAPANESE_VARIANTS",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `getSerializedName()`.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            FontOption::Uniform => "uniform",
            FontOption::JapaneseVariants => "jp",
        }
    }
}

impl std::fmt::Display for FontOption {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for FontOption {
    fn get_serialized_name(&self) -> &str {
        FontOption::get_serialized_name(self)
    }
}

/// Java: `public static final Codec<FontOption> CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Port of `net.minecraft.client.gui.font.FontOption.Filter`.
///
/// Java keeps an immutable `Map<FontOption, Boolean>`; map iteration order cannot change the
/// result of `apply` (it is an all-of test) or of `merge`, so a `HashMap` is equivalent.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Filter {
    values: std::collections::HashMap<FontOption, bool>,
}

impl Filter {
    /// Java: `public Filter(Map<FontOption, Boolean> values)`.
    pub fn new(values: std::collections::HashMap<FontOption, bool>) -> Filter {
        Filter { values }
    }

    /// Java: `public static final FontOption.Filter ALWAYS_PASS = new Filter(Map.of())`.
    pub fn always_pass() -> Filter {
        Filter::default()
    }

    /// Port of `Filter#apply(Set<FontOption>)`.
    pub fn apply(&self, options: &std::collections::HashSet<FontOption>) -> bool {
        for (key, wanted) in &self.values {
            if options.contains(key) != *wanted {
                return false;
            }
        }
        true
    }

    /// Port of `Filter#merge(Filter)`: `other`'s entries, overwritten by this filter's.
    pub fn merge(&self, other: &Filter) -> Filter {
        let mut options = other.values.clone();
        for (k, v) in &self.values {
            options.insert(*k, *v);
        }
        Filter { values: options }
    }

    /// The backing map (`Filter.CODEC` encodes it as `{name: bool}`).
    pub fn values(&self) -> &std::collections::HashMap<FontOption, bool> {
        &self.values
    }
}

/// Java: `public static final Codec<FontOption.Filter> Filter.CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn FILTER_CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
