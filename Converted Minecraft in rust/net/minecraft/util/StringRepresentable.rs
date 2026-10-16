//! Port of: net/minecraft/util/StringRepresentable.java
//! Java class(es): net.minecraft.util.StringRepresentable, net.minecraft.util.EnumCodec, net.minecraft.util.StringRepresentableCodec, StringRepresentable::<1 anonymous class(es)>
//! Status: PARTIAL
//!
//! Session 19.
//!
//! Ported: the interface itself ([`StringRepresentable::get_serialized_name`]),
//! `PRE_BUILT_MAP_THRESHOLD` (checked against the jar in `session19_consts.rs`), both
//! `createNameLookup` overloads including the 16-entry switch between a linear scan and a map,
//! `fromEnum` / `fromEnumWithMapping`, and `EnumCodec#byName` in all three overloads.
//!
//! PORT-BLOCKED (DataFixerUpper is not ported, see `DESIGN_DECISIONS.md#dfu-proposal`): the
//! `Codec` half of `EnumCodec` / `StringRepresentableCodec` (`encode`/`decode`), `fromValues`
//! (returns a bare `Codec`), and `keys` (returns a DFU `Keyable`).
//!
//! # THE THRESHOLD CHANGES BEHAVIOUR, NOT JUST SPEED
//!
//! ```java
//! if (valueArray.length > 16) {
//!     Map<String, T> byName = Arrays.stream(valueArray).collect(Collectors.toMap(converter, d -> d));
//!     return byName::get;
//! } else {
//!     return id -> { for (T value : valueArray) if (converter.apply(value).equals(id)) return value; return null; };
//! }
//! ```
//!
//! The two branches disagree on duplicate serialized names. Up to 16 values, a duplicate is
//! silently tolerated and the *first* declared value wins. Above 16, `Collectors.toMap` throws
//! `IllegalStateException` at construction. The port keeps both branches rather than using a map
//! everywhere, because "first wins" is the observable rule for every small enum in the game.
//!
//! `> 16`, not `>= 16`: an enum with exactly 16 values takes the linear branch.
//!
//! # `byName` IS CASE-SENSITIVE AND EXACT
//!
//! `String#equals` / `HashMap#get`. No trimming, no lower-casing. `"STONE"` does not find a
//! value serialized as `"stone"`.

use std::collections::HashMap;
use std::fmt::Display;

/// Java: `int PRE_BUILT_MAP_THRESHOLD = 16`.
pub const PRE_BUILT_MAP_THRESHOLD: i32 = 16;

/// Port of the `net.minecraft.util.StringRepresentable` interface.
///
/// `getSerializedName()` is the persisted spelling of a value: it is what block-state
/// properties, commands, JSON and NBT write. Changing one is a save-format change.
pub trait StringRepresentable {
    /// Java: `String getSerializedName()`.
    fn get_serialized_name(&self) -> &str;
}

#[cold]
#[inline(never)]
fn throw_illegal_state(message: String) -> ! {
    panic!("java.lang.IllegalStateException: {message}")
}

/// The `Function<String, @Nullable T>` returned by `createNameLookup`.
///
/// `None` is Java's `null`.
#[derive(Clone, Debug)]
pub enum NameLookup<T> {
    /// `valueArray.length <= 16`: scan in declaration order, first match wins.
    Linear(Vec<(String, T)>),
    /// `valueArray.length > 16`: a `HashMap` built by `Collectors.toMap`.
    Map(HashMap<String, T>),
}

impl<T: Clone> NameLookup<T> {
    /// `Function#apply`.
    pub fn apply(&self, name: &str) -> Option<T> {
        match self {
            NameLookup::Linear(entries) => entries
                .iter()
                .find(|(n, _)| n == name)
                .map(|(_, v)| v.clone()),
            NameLookup::Map(map) => map.get(name).cloned(),
        }
    }
}

/// Port of `StringRepresentable#createNameLookup(T[], Function<T, String>)`.
///
/// The converter is evaluated once per value here. Java's linear branch re-evaluates it on every
/// lookup; every converter vanilla passes is a pure function of the value (`getSerializedName`,
/// optionally composed with a pure `String -> String` mapping), so the results are identical.
///
/// Panics with Java's `Collectors.toMap` text when more than 16 values share a key:
/// `java.lang.IllegalStateException: Duplicate key <k> (attempted merging values <u> and <v>)`,
/// where `u` is the value already in the map and `v` the later one.
pub fn create_name_lookup_with<T: Clone + Display>(
    values: &[T],
    converter: impl Fn(&T) -> String,
) -> NameLookup<T> {
    if values.len() as i32 > PRE_BUILT_MAP_THRESHOLD {
        let mut by_name: HashMap<String, T> = HashMap::with_capacity(values.len());
        for v in values {
            let key = converter(v);
            if let Some(existing) = by_name.get(&key) {
                throw_illegal_state(format!(
                    "Duplicate key {key} (attempted merging values {existing} and {v})"
                ));
            }
            by_name.insert(key, v.clone());
        }
        NameLookup::Map(by_name)
    } else {
        NameLookup::Linear(values.iter().map(|v| (converter(v), v.clone())).collect())
    }
}

/// Port of `StringRepresentable#createNameLookup(T[])`: keyed by `getSerializedName()`.
pub fn create_name_lookup<T: StringRepresentable + Clone + Display>(values: &[T]) -> NameLookup<T> {
    create_name_lookup_with(values, |v| v.get_serialized_name().to_string())
}

/// Port of `StringRepresentable.EnumCodec<E>`: the name-resolution half.
///
/// The `Codec` half (`decode`/`encode`, inherited from `StringRepresentableCodec`) is
/// PORT-BLOCKED on DataFixerUpper; see [`EnumCodec::decode`].
#[derive(Clone, Debug)]
pub struct EnumCodec<E> {
    values: Vec<E>,
    resolver: NameLookup<E>,
}

impl<E: Clone> EnumCodec<E> {
    /// Port of `EnumCodec#byName(String)`. `None` is Java's `null`.
    #[inline]
    pub fn by_name(&self, name: &str) -> Option<E> {
        self.resolver.apply(name)
    }

    /// Port of `EnumCodec#byName(String, E)`: `Objects.requireNonNullElse(byName(name), default)`.
    #[inline]
    pub fn by_name_or(&self, name: &str, default: E) -> E {
        self.by_name(name).unwrap_or(default)
    }

    /// Port of `EnumCodec#byName(String, Supplier<? extends E>)`.
    ///
    /// The supplier is only called when the name is unknown, as with
    /// `Objects.requireNonNullElseGet`.
    #[inline]
    pub fn by_name_or_else(&self, name: &str, default_supplier: impl FnOnce() -> E) -> E {
        self.by_name(name).unwrap_or_else(default_supplier)
    }

    /// The `valueArray` the codec was built from, in declaration order. Java keeps it for the
    /// compressed (ordinal) encoding.
    #[inline]
    pub fn values(&self) -> &[E] {
        &self.values
    }

    /// Java: `Codec#decode` via `StringRepresentableCodec`.
    ///
    /// PORT-BLOCKED: needs `DynamicOps`/`DataResult` and `ExtraCodecs.orCompressed`.
    pub fn decode(&self) -> ! {
        todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
    }

    /// Java: `Codec#encode` via `StringRepresentableCodec`.
    ///
    /// PORT-BLOCKED: needs `DynamicOps`/`DataResult` and `ExtraCodecs.orCompressed`.
    pub fn encode(&self) -> ! {
        todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
    }
}

/// Port of `StringRepresentable#fromEnum(Supplier<E[]>)`.
pub fn from_enum<E: StringRepresentable + Clone + Display>(values: &[E]) -> EnumCodec<E> {
    from_enum_with_mapping(values, |s| s.to_string())
}

/// Port of `StringRepresentable#fromEnumWithMapping(Supplier<E[]>, Function<String, String>)`.
///
/// The converter is applied to each value's serialized name to produce the lookup key. Note that
/// it only affects *lookup*: Java's codec still encodes with the unconverted
/// `getSerializedName()`.
pub fn from_enum_with_mapping<E: StringRepresentable + Clone + Display>(
    values: &[E],
    converter: impl Fn(&str) -> String,
) -> EnumCodec<E> {
    let resolver = create_name_lookup_with(values, |e| converter(e.get_serialized_name()));
    EnumCodec {
        values: values.to_vec(),
        resolver,
    }
}

/// Java: `static <T extends StringRepresentable> Codec<T> fromValues(Supplier<T[]>)`.
///
/// PORT-BLOCKED: the return value is a bare DFU `Codec`, and it also needs
/// `Util.createIndexLookup`. The name lookup it would use is [`create_name_lookup`].
pub fn from_values() -> ! {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `static Keyable keys(StringRepresentable[])`.
///
/// PORT-BLOCKED: `com.mojang.serialization.Keyable` / `DynamicOps` are not ported. The keys it
/// would stream are each value's `getSerializedName()`, in array order.
pub fn keys() -> ! {
    todo!("PORT-BLOCKED: com.mojang.serialization.Keyable is not ported")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct N(&'static str, &'static str);

    impl StringRepresentable for N {
        fn get_serialized_name(&self) -> &str {
            self.1
        }
    }

    impl Display for N {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0)
        }
    }

    fn many(n: usize) -> Vec<N> {
        // Leak once per test: a handful of short strings, kept 'static for the N type.
        (0..n)
            .map(|i| {
                let name: &'static str = Box::leak(format!("C{i}").into_boxed_str());
                let ser: &'static str = Box::leak(format!("c{i}").into_boxed_str());
                N(name, ser)
            })
            .collect()
    }

    #[test]
    fn sixteen_values_use_the_linear_branch_and_seventeen_the_map() {
        assert!(matches!(create_name_lookup(&many(16)), NameLookup::Linear(_)));
        assert!(matches!(create_name_lookup(&many(17)), NameLookup::Map(_)));
        for n in [1usize, 16, 17, 40] {
            let vals = many(n);
            let lookup = create_name_lookup(&vals);
            for v in &vals {
                assert_eq!(lookup.apply(v.1), Some(*v));
            }
            assert_eq!(lookup.apply("missing"), None);
        }
    }

    #[test]
    fn small_tables_tolerate_duplicates_and_the_first_wins() {
        let vals = [N("A", "same"), N("B", "same"), N("C", "other")];
        let lookup = create_name_lookup(&vals);
        assert_eq!(lookup.apply("same"), Some(N("A", "same")));
    }

    #[test]
    fn large_tables_reject_duplicates_with_collectors_to_map_text() {
        let mut vals = many(17);
        vals[16] = N("LATE", "c3");
        let err = std::panic::catch_unwind(|| {
            create_name_lookup(&vals);
        })
        .expect_err("a duplicate key above the threshold must throw");
        let msg = err.downcast_ref::<String>().unwrap();
        assert_eq!(
            msg,
            "java.lang.IllegalStateException: Duplicate key c3 (attempted merging values C3 and LATE)"
        );
    }

    #[test]
    fn lookup_is_exact_and_case_sensitive() {
        let vals = [N("STONE", "stone")];
        let codec = from_enum(&vals);
        assert_eq!(codec.by_name("stone"), Some(vals[0]));
        assert_eq!(codec.by_name("STONE"), None);
        assert_eq!(codec.by_name(" stone"), None);
        assert_eq!(codec.by_name(""), None);
    }

    #[test]
    fn by_name_defaults_and_supplier_is_lazy() {
        let vals = [N("A", "a"), N("B", "b")];
        let codec = from_enum(&vals);
        assert_eq!(codec.by_name_or("b", vals[0]), vals[1]);
        assert_eq!(codec.by_name_or("zzz", vals[0]), vals[0]);
        let mut called = false;
        assert_eq!(
            codec.by_name_or_else("a", || {
                called = true;
                vals[1]
            }),
            vals[0]
        );
        assert!(!called, "the supplier must not run when the name resolves");
        assert_eq!(codec.by_name_or_else("q", || vals[1]), vals[1]);
    }

    #[test]
    fn with_mapping_changes_the_lookup_key_only() {
        let vals = [N("A", "a"), N("B", "b")];
        let codec = from_enum_with_mapping(&vals, |s| format!("x_{s}"));
        assert_eq!(codec.by_name("x_a"), Some(vals[0]));
        assert_eq!(codec.by_name("a"), None);
        assert_eq!(codec.values(), &vals);
    }
}
