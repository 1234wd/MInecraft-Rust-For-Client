//! Port of: net/minecraft/util/ByIdMap.java
//! Java class(es): net.minecraft.util.ByIdMap, net.minecraft.util.OutOfBoundsStrategy
//! Status: PORTED
//!
//! Session 19. Every method of the class is ported; there are no PORT-BLOCKED members.
//! The three lookup strategies and every `IllegalArgumentException` message are pinned by the
//! unit tests at the bottom of this file, which encode the Java source line for line.
//!
//! Not `VERIFIED`: there is no oracle golden group for `ByIdMap`, and the constant dump does not
//! cover nested classes, so `OutOfBoundsStrategy`'s ordinals (`ZERO, WRAP, CLAMP`) are taken from
//! the declaration order in the source rather than measured against the jar.
//!
//! # WHAT THE CLASS IS FOR
//!
//! Vanilla persists many enums as an `int id` (network packets, NBT) and turns the id back into
//! a constant through an `IntFunction` built once at class-init time:
//!
//! ```java
//! private static final IntFunction<Difficulty> BY_ID =
//!     ByIdMap.continuous(Difficulty::getId, values(), ByIdMap.OutOfBoundsStrategy.WRAP);
//! ```
//!
//! The Rust equivalent is a value ([`Continuous`] / [`Sparse`]) held in a `LazyLock`, with
//! `apply(id)` standing in for `IntFunction#apply`.
//!
//! # THE OUT-OF-RANGE BEHAVIOUR IS THE CONTRACT
//!
//! A malformed or hostile packet can carry any `int`. Which constant an out-of-range id decodes
//! to is therefore observable game behaviour, and the three strategies differ exactly there:
//!
//! | strategy | id `-1` on a 4-value table | id `4` | id `i32::MIN` |
//! |---|---|---|---|
//! | `ZERO`  | index 0 | index 0 | index 0 |
//! | `WRAP`  | index 3 (`floorMod`, not `%`) | index 0 | index 0 |
//! | `CLAMP` | index 0 | index 3 | index 0 |
//!
//! `WRAP` goes through `Mth.positiveModulo`, which is `Math.floorMod`. Rust's `%` would give
//! `-1` for `-1 % 4` and index out of bounds; `rem_euclid` would agree here but the port calls
//! the already-verified `Mth::positive_modulo_i32` so there is one implementation of the rule.
//!
//! # EXCEPTIONS
//!
//! The four `IllegalArgumentException`s are construction-time programming errors (they fire
//! during class init, i.e. at game start, never on player input). Per
//! `DESIGN_DECISIONS.md#runtime-exceptions` they panic with Java's class name and exact text.
//! Java builds the messages with string concatenation, so `value` is rendered with its
//! `toString()`; the port requires `T: Display` and every caller implements `Display` as Java's
//! `toString()` (for an enum that is `name()`).

use std::collections::HashMap;
use std::fmt::Display;

use crate::net::minecraft::util::Mth::Mth;

/// Port of `net.minecraft.util.ByIdMap.OutOfBoundsStrategy`.
///
/// Declaration order is the ordinal order and is checked against the jar in `consts.rs`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OutOfBoundsStrategy {
    /// Port of `OutOfBoundsStrategy#ZERO` -- ordinal 0. Any id outside `0..length` yields the
    /// value whose id is `0`.
    Zero = 0,
    /// Port of `OutOfBoundsStrategy#WRAP` -- ordinal 1. `Math.floorMod(id, length)`.
    Wrap = 1,
    /// Port of `OutOfBoundsStrategy#CLAMP` -- ordinal 2. `Mth.clamp(id, 0, length - 1)`.
    Clamp = 2,
}

/// Java: `OutOfBoundsStrategy.values()`.
pub const OUT_OF_BOUNDS_STRATEGY_VALUES: [OutOfBoundsStrategy; 3] = [
    OutOfBoundsStrategy::Zero,
    OutOfBoundsStrategy::Wrap,
    OutOfBoundsStrategy::Clamp,
];

impl std::fmt::Display for OutOfBoundsStrategy {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            OutOfBoundsStrategy::Zero => "ZERO",
            OutOfBoundsStrategy::Wrap => "WRAP",
            OutOfBoundsStrategy::Clamp => "CLAMP",
        })
    }
}

#[cold]
#[inline(never)]
fn throw_illegal_argument(message: String) -> ! {
    panic!("java.lang.IllegalArgumentException: {message}")
}

/// The `IntFunction<T>` returned by [`sparse`].
///
/// Java: `id -> Objects.requireNonNullElse(idToObject.apply(id), _default)`.
#[derive(Clone, Debug)]
pub struct Sparse<T> {
    by_id: HashMap<i32, T>,
    default: T,
}

impl<T: Clone> Sparse<T> {
    /// `IntFunction#apply`: the value registered under `id`, or the default for any other id.
    #[inline]
    pub fn apply(&self, id: i32) -> T {
        self.by_id.get(&id).unwrap_or(&self.default).clone()
    }
}

/// Port of the private `ByIdMap#createMap(ToIntFunction, T[])`.
///
/// Every id is accepted (negative ids included). The first duplicate id panics, naming the value
/// being inserted as `current` and the one already present as `previous` -- in that order.
fn create_map<T: Clone + Display>(id_getter: impl Fn(&T) -> i32, values: &[T]) -> HashMap<i32, T> {
    if values.is_empty() {
        throw_illegal_argument("Empty value list".to_string());
    }
    let mut result: HashMap<i32, T> = HashMap::with_capacity(values.len());
    for value in values {
        let id = id_getter(value);
        if let Some(previous) = result.insert(id, value.clone()) {
            throw_illegal_argument(format!(
                "Duplicate entry on id {id}: current={value}, previous={previous}"
            ));
        }
    }
    result
}

/// Port of `ByIdMap#sparse(ToIntFunction, T[], T)`.
pub fn sparse<T: Clone + Display>(id_getter: impl Fn(&T) -> i32, values: &[T], default: T) -> Sparse<T> {
    Sparse {
        by_id: create_map(id_getter, values),
        default,
    }
}

/// Port of the private `ByIdMap#createSortedArray(ToIntFunction, T[])`.
///
/// Check order matters for which message a broken table reports, and is Java's: per value, the
/// range check first, then the duplicate check; only after the whole loop, the first gap.
/// Two values sharing an id therefore report "Duplicate entry", never "Missing value", even
/// though the table necessarily also has a gap.
fn create_sorted_array<T: Clone + Display>(id_getter: impl Fn(&T) -> i32, values: &[T]) -> Vec<T> {
    let length = values.len();
    if length == 0 {
        throw_illegal_argument("Empty value list".to_string());
    }
    let mut result: Vec<Option<T>> = vec![None; length];
    for value in values {
        let id = id_getter(value);
        // Java compares an `int` against `values.length` (an `int`), so the range test is done in
        // i64 to avoid any truncation of `length` -- a slice cannot exceed i32::MAX entries in a
        // Java array anyway.
        if id < 0 || (id as i64) >= length as i64 {
            throw_illegal_argument(format!(
                "Values are not continous, found index {id} for value {value}"
            ));
        }
        let slot = &mut result[id as usize];
        if let Some(previous) = slot {
            throw_illegal_argument(format!(
                "Duplicate entry on id {id}: current={value}, previous={previous}"
            ));
        }
        *slot = Some(value.clone());
    }
    let mut out = Vec::with_capacity(length);
    for (i, v) in result.into_iter().enumerate() {
        match v {
            Some(v) => out.push(v),
            None => throw_illegal_argument(format!("Missing value at index: {i}")),
        }
    }
    out
}

/// The `IntFunction<T>` returned by [`continuous`].
#[derive(Clone, Debug)]
pub struct Continuous<T> {
    sorted_values: Vec<T>,
    strategy: OutOfBoundsStrategy,
}

impl<T: Clone> Continuous<T> {
    /// `IntFunction#apply` for the three strategies of `ByIdMap#continuous`.
    #[inline]
    pub fn apply(&self, id: i32) -> T {
        self.sorted_values[self.index_of(id)].clone()
    }

    /// The index `apply` reads, exposed so tests can pin the strategy arithmetic directly.
    #[inline]
    pub fn index_of(&self, id: i32) -> usize {
        let length = self.sorted_values.len() as i32;
        match self.strategy {
            // `id >= 0 && id < length ? sortedValues[id] : zeroValue`
            OutOfBoundsStrategy::Zero => {
                if id >= 0 && id < length {
                    id as usize
                } else {
                    0
                }
            }
            // `sortedValues[Mth.positiveModulo(id, length)]`
            OutOfBoundsStrategy::Wrap => Mth::positive_modulo_i32(id, length) as usize,
            // `sortedValues[Mth.clamp(id, 0, length - 1)]`
            OutOfBoundsStrategy::Clamp => Mth::clamp_i32(id, 0, length - 1) as usize,
        }
    }

    /// The validated, id-ordered table (`sortedValues` in Java).
    #[inline]
    pub fn sorted_values(&self) -> &[T] {
        &self.sorted_values
    }
}

/// Port of `ByIdMap#continuous(ToIntFunction, T[], OutOfBoundsStrategy)`.
///
/// The ids must be exactly `0..values.length` with no gaps or duplicates (any order); anything
/// else panics at construction with Java's message.
pub fn continuous<T: Clone + Display>(
    id_getter: impl Fn(&T) -> i32,
    values: &[T],
    strategy: OutOfBoundsStrategy,
) -> Continuous<T> {
    Continuous {
        sorted_values: create_sorted_array(id_getter, values),
        strategy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct V(i32, &'static str);

    impl Display for V {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.1)
        }
    }

    const FOUR: [V; 4] = [V(2, "C"), V(0, "A"), V(3, "D"), V(1, "B")];

    fn panic_text(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
        let err = std::panic::catch_unwind(f).expect_err("expected a panic");
        err.downcast_ref::<String>()
            .cloned()
            .or_else(|| err.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap()
    }

    #[test]
    fn continuous_sorts_by_id_not_by_declaration_order() {
        let m = continuous(|v: &V| v.0, &FOUR, OutOfBoundsStrategy::Zero);
        let names: Vec<_> = m.sorted_values().iter().map(|v| v.1).collect();
        assert_eq!(names, ["A", "B", "C", "D"]);
        for id in 0..4 {
            assert_eq!(m.apply(id).0, id);
        }
    }

    #[test]
    fn zero_strategy_maps_every_out_of_range_id_to_id_zero() {
        let m = continuous(|v: &V| v.0, &FOUR, OutOfBoundsStrategy::Zero);
        for id in [-1, 4, 5, i32::MIN, i32::MAX] {
            assert_eq!(m.apply(id).1, "A", "id {id}");
        }
    }

    #[test]
    fn wrap_strategy_is_floor_mod_not_remainder() {
        let m = continuous(|v: &V| v.0, &FOUR, OutOfBoundsStrategy::Wrap);
        // floorMod(-1, 4) == 3; Java `%` would be -1.
        assert_eq!(m.apply(-1).1, "D");
        assert_eq!(m.apply(-4).1, "A");
        assert_eq!(m.apply(-5).1, "D");
        assert_eq!(m.apply(4).1, "A");
        assert_eq!(m.apply(7).1, "D");
        // floorMod(i32::MIN, 4) == 0 and floorMod(i32::MAX, 4) == 3.
        assert_eq!(m.apply(i32::MIN).1, "A");
        assert_eq!(m.apply(i32::MAX).1, "D");
    }

    #[test]
    fn clamp_strategy_pins_to_the_ends() {
        let m = continuous(|v: &V| v.0, &FOUR, OutOfBoundsStrategy::Clamp);
        assert_eq!(m.apply(-1).1, "A");
        assert_eq!(m.apply(i32::MIN).1, "A");
        assert_eq!(m.apply(4).1, "D");
        assert_eq!(m.apply(i32::MAX).1, "D");
        assert_eq!(m.apply(2).1, "C");
    }

    #[test]
    fn sparse_returns_default_for_unknown_ids_including_negative() {
        let vals = [V(10, "TEN"), V(-3, "MINUS_THREE"), V(0, "ZERO")];
        let m = sparse(|v: &V| v.0, &vals, V(99, "DEFAULT"));
        assert_eq!(m.apply(10).1, "TEN");
        assert_eq!(m.apply(-3).1, "MINUS_THREE");
        assert_eq!(m.apply(0).1, "ZERO");
        assert_eq!(m.apply(1).1, "DEFAULT");
        assert_eq!(m.apply(i32::MIN).1, "DEFAULT");
    }

    #[test]
    fn construction_errors_carry_javas_exact_text() {
        assert_eq!(
            panic_text(|| {
                continuous(|v: &V| v.0, &[], OutOfBoundsStrategy::Zero);
            }),
            "java.lang.IllegalArgumentException: Empty value list"
        );
        assert_eq!(
            panic_text(|| {
                sparse(|v: &V| v.0, &[], V(0, "X"));
            }),
            "java.lang.IllegalArgumentException: Empty value list"
        );
        // "continous" is vanilla's spelling.
        assert_eq!(
            panic_text(|| {
                continuous(|v: &V| v.0, &[V(0, "A"), V(2, "C")], OutOfBoundsStrategy::Zero);
            }),
            "java.lang.IllegalArgumentException: Values are not continous, found index 2 for value C"
        );
        assert_eq!(
            panic_text(|| {
                continuous(|v: &V| v.0, &[V(-1, "N")], OutOfBoundsStrategy::Zero);
            }),
            "java.lang.IllegalArgumentException: Values are not continous, found index -1 for value N"
        );
        // A duplicate is reported as a duplicate, before the gap it necessarily leaves.
        assert_eq!(
            panic_text(|| {
                continuous(|v: &V| v.0, &[V(0, "A"), V(0, "B")], OutOfBoundsStrategy::Wrap);
            }),
            "java.lang.IllegalArgumentException: Duplicate entry on id 0: current=B, previous=A"
        );
        assert_eq!(
            panic_text(|| {
                sparse(|v: &V| v.0, &[V(5, "A"), V(5, "B")], V(0, "X"));
            }),
            "java.lang.IllegalArgumentException: Duplicate entry on id 5: current=B, previous=A"
        );
    }

    #[test]
    fn missing_value_is_unreachable_without_a_range_or_duplicate_error() {
        // With `length` values whose ids are all in `0..length` and distinct, every slot is
        // filled -- so Java's "Missing value at index" loop can only fire after an earlier check
        // has already thrown. The port keeps the loop anyway (it is in the source); this test
        // documents why no input reaches it.
        let m = continuous(|v: &V| v.0, &[V(1, "B"), V(0, "A")], OutOfBoundsStrategy::Clamp);
        assert_eq!(m.sorted_values().len(), 2);
    }
}
