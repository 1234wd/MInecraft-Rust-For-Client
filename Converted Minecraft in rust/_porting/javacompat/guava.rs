//! The parts of Guava 33.6.0-jre (`com.google.common`) that ported files need, type by type.
//! Session 24.
//!
//! | Java | here | used by |
//! |---|---|---|
//! | `ImmutableMap<String, String>` built from literals | [`ImmutableStrMap`] | the datafix rename tables (`BiomeFix`, `RecipesFix`, `CavesAndCliffsRenames`, ...) |
//! | `AbstractIterator<T>` (with `UnmodifiableIterator`) | [`AbstractIterator`] over a [`ComputeNext`] | `IntersectionIterator`, `MergingUniqueIterator` |
//! | `Iterators.peekingIterator(it)` (`Iterators.PeekingImpl`) | [`PeekingIterator`] | the same two |
//! | (`java.util.Iterator<E>`, which both take) | [`JavaIterator`], [`ExactIter`] | the same two |
//!
//! # `ImmutableMap` as a literal table
//!
//! `ImmutableMap.builder().put(k, v)....build()` keeps the PUT ORDER for every view (`entrySet`,
//! `keySet`, `values`, `toString`), rejects a repeated key at `build()` with
//! `IllegalArgumentException: Multiple entries with same key: ...`, and rejects nulls. The tables
//! ported here are compile-time literals, so [`ImmutableStrMap`] is a `const` slice of pairs in
//! put order; [`ImmutableStrMap::new`] refuses a repeated key at compile time (a `const` panic), so
//! a table that would throw in Java cannot be built here either.
//!
//! Observable members, as `java.util.Map` defines them and Guava implements them:
//!
//! * `get`/`containsKey` -- by `String.equals`;
//! * `size`, `isEmpty`, iteration in put order;
//! * `hashCode` -- the `Map` contract: the sum of `key.hashCode() ^ value.hashCode()`;
//! * `toString` -- `{k1=v1, k2=v2}` (`Maps.toStringImpl`);
//! * `equals` -- the same set of mappings, order ignored (the `Map` contract).
//!
//! Lookup is a linear scan. The largest table has 54 entries and none is on a hot path (each is
//! read while a datafixer rewrites one old value).

use crate::javacompat::java_lang::string_hash_code;

/// Guava 33.6.0-jre's `com.google.common.math.IntMath` methods used by small ports.
pub mod int_math {
    /// `IntMath.gcd(int, int)` for non-negative values.
    ///
    /// The Java method rejects negative operands before taking the GCD. Its messages name the
    /// first failing argument, in declaration order.
    pub fn gcd(a: i32, b: i32) -> i32 {
        if a < 0 {
            panic!("java.lang.IllegalArgumentException: a ({a}) must be >= 0");
        }
        if b < 0 {
            panic!("java.lang.IllegalArgumentException: b ({b}) must be >= 0");
        }

        let (mut x, mut y) = (a, b);
        while y != 0 {
            (x, y) = (y, x % y);
        }
        x
    }
}

/// Port of a Guava `ImmutableMap<String, String>` built from string literals.
#[derive(Debug, Clone, Copy)]
pub struct ImmutableStrMap {
    entries: &'static [(&'static str, &'static str)],
}

/// Whether two string literals are equal, usable in a `const fn`.
const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

impl ImmutableStrMap {
    /// `ImmutableMap.builder().put(k, v)...build()` over `entries`, in put order.
    ///
    /// # Panics
    ///
    /// At compile time, when a key repeats -- Guava's `build()` throws then.
    pub const fn new(entries: &'static [(&'static str, &'static str)]) -> Self {
        let mut i = 0;
        while i < entries.len() {
            let mut j = i + 1;
            while j < entries.len() {
                if str_eq(entries[i].0, entries[j].0) {
                    panic!("java.lang.IllegalArgumentException: Multiple entries with same key");
                }
                j += 1;
            }
            i += 1;
        }
        ImmutableStrMap { entries }
    }

    /// `Map.get(key)`: the value, or `None` (Java's `null`).
    pub fn get(&self, key: &str) -> Option<&'static str> {
        self.entries
            .iter()
            .find(|(k, _)| *k == key)
            .map(|&(_, v)| v)
    }

    /// `Map.getOrDefault(key, defaultValue)`.
    pub fn get_or_default<'a>(&self, key: &str, default_value: &'a str) -> &'a str {
        self.get(key).unwrap_or(default_value)
    }

    /// `Map.containsKey(key)`.
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// `Map.containsValue(value)`.
    pub fn contains_value(&self, value: &str) -> bool {
        self.entries.iter().any(|&(_, v)| v == value)
    }

    /// `Map.size()`.
    pub fn size(&self) -> i32 {
        self.entries.len() as i32
    }

    /// `Map.isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// `entrySet()` in put order, as `(key, value)` pairs.
    pub fn entries(&self) -> &'static [(&'static str, &'static str)] {
        self.entries
    }

    /// `keySet()` in put order.
    pub fn keys(&self) -> impl Iterator<Item = &'static str> {
        self.entries.iter().map(|&(k, _)| k)
    }

    /// `values()` in put order.
    pub fn values(&self) -> impl Iterator<Item = &'static str> {
        self.entries.iter().map(|&(_, v)| v)
    }

    /// `Map.hashCode()`: the sum, wrapping, of `key.hashCode() ^ value.hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        self.entries.iter().fold(0i32, |acc, &(k, v)| {
            acc.wrapping_add(string_hash_code(k) ^ string_hash_code(v))
        })
    }
}

impl PartialEq for ImmutableStrMap {
    /// `Map.equals`: the same mappings, whatever their order.
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size() && self.entries.iter().all(|&(k, v)| other.get(k) == Some(v))
    }
}

impl Eq for ImmutableStrMap {}

impl std::fmt::Display for ImmutableStrMap {
    /// `toString()`: `{k1=v1, k2=v2}` in put order.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        for (i, (k, v)) in self.entries.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{k}={v}")?;
        }
        f.write_str("}")
    }
}

/// The `computeNext()` of a Guava `AbstractIterator` subclass: the next element, or `None` where
/// Java calls `endOfData()`.
pub trait ComputeNext {
    /// The element type `T`.
    type Item;

    /// Port of `protected abstract T computeNext()`; `None` is `return endOfData()`.
    fn compute_next(&mut self) -> Option<Self::Item>;
}

/// `AbstractIterator.State`, in Java's declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Ready,
    NotReady,
    Done,
    Failed,
}

/// Port of `com.google.common.collect.AbstractIterator<T>`, read from Guava 33.6.0-jre's bytecode.
///
/// ```java
/// public final boolean hasNext() {
///     checkState(state != FAILED);              // IllegalStateException, no message
///     switch (state) { case DONE: return false; case READY: return true; default: }
///     return tryToComputeNext();
/// }
/// private boolean tryToComputeNext() {
///     state = FAILED;                           // stays FAILED if computeNext throws
///     next = computeNext();
///     if (state != DONE) { state = READY; return true; }
///     return false;
/// }
/// public final T next() {                       // NoSuchElementException when exhausted
///     if (!hasNext()) throw new NoSuchElementException();
///     state = NOT_READY; T result = next; next = null; return result;
/// }
/// public final T peek() {                       // NoSuchElementException when exhausted
///     if (!hasNext()) throw new NoSuchElementException();
///     return next;
/// }
/// ```
///
/// `endOfData()` sets `DONE`, after which `computeNext` is never called again. The inherited
/// `UnmodifiableIterator.remove()` always throws `UnsupportedOperationException`; Rust iterators
/// have no `remove`, so there is nothing to port. As a Rust [`Iterator`], `next()` returns `None`
/// where Java's `next()` would throw, and the iterator is fused.
#[derive(Debug)]
pub struct AbstractIterator<C: ComputeNext> {
    inner: C,
    state: State,
    next: Option<C::Item>,
}

impl<C: ComputeNext> AbstractIterator<C> {
    /// `protected AbstractIterator()`: `state = NOT_READY`.
    pub fn new(inner: C) -> Self {
        AbstractIterator {
            inner,
            state: State::NotReady,
            next: None,
        }
    }

    /// The subclass state (the fields of the Java subclass).
    pub fn inner(&self) -> &C {
        &self.inner
    }

    /// Port of `public final boolean hasNext()`.
    pub fn has_next(&mut self) -> bool {
        if self.state == State::Failed {
            panic!("java.lang.IllegalStateException");
        }
        match self.state {
            State::Done => false,
            State::Ready => true,
            _ => self.try_to_compute_next(),
        }
    }

    /// Port of `private boolean tryToComputeNext()`.
    fn try_to_compute_next(&mut self) -> bool {
        self.state = State::Failed;
        match self.inner.compute_next() {
            Some(v) => {
                self.next = Some(v);
                self.state = State::Ready;
                true
            }
            None => {
                self.next = None;
                self.state = State::Done;
                false
            }
        }
    }

    /// Port of `public final T next()`, throwing as Java does when there is no element.
    pub fn next_or_throw(&mut self) -> C::Item {
        if !self.has_next() {
            panic!("java.util.NoSuchElementException");
        }
        self.state = State::NotReady;
        self.next.take().expect("READY holds an element")
    }

    /// Port of `public final T peek()`: the next element without consuming it; `None` where Java
    /// throws `NoSuchElementException`.
    pub fn peek(&mut self) -> Option<&C::Item> {
        if !self.has_next() {
            return None;
        }
        self.next.as_ref()
    }
}

impl<C: ComputeNext> Iterator for AbstractIterator<C> {
    type Item = C::Item;

    /// `hasNext() ? next() : <end>`.
    fn next(&mut self) -> Option<C::Item> {
        if !self.has_next() {
            return None;
        }
        self.state = State::NotReady;
        self.next.take()
    }
}

impl<C: ComputeNext> std::iter::FusedIterator for AbstractIterator<C> {}

/// `java.util.Iterator<E>`, whose `hasNext()` answers WITHOUT consuming an element.
///
/// A Rust [`Iterator`] can only find out whether there is a next element by taking it, which
/// changes how many elements a source hands out. Guava's peeking iterator relies on the
/// difference (its `hasNext()` asks the source's `hasNext()`), so the iterators ported on top of
/// it take this trait.
pub trait JavaIterator {
    /// `E`.
    type Item;

    /// `boolean hasNext()`: no element is consumed.
    fn has_next(&mut self) -> bool;

    /// `E next()`, throwing `NoSuchElementException` (a panic with that text) when exhausted.
    fn next_elem(&mut self) -> Self::Item;
}

/// A [`JavaIterator`] over a Rust iterator that knows its remaining length -- a `Vec`'s or a
/// slice's, i.e. a Java `List.iterator()` -- so `has_next` need not consume anything.
#[derive(Debug, Clone)]
pub struct ExactIter<I: ExactSizeIterator>(pub I);

impl<I: ExactSizeIterator> JavaIterator for ExactIter<I> {
    type Item = I::Item;

    fn has_next(&mut self) -> bool {
        self.0.len() > 0
    }

    fn next_elem(&mut self) -> I::Item {
        self.0
            .next()
            .unwrap_or_else(|| panic!("java.util.NoSuchElementException"))
    }
}

impl<C: ComputeNext> JavaIterator for AbstractIterator<C> {
    type Item = C::Item;

    fn has_next(&mut self) -> bool {
        AbstractIterator::has_next(self)
    }

    fn next_elem(&mut self) -> C::Item {
        self.next_or_throw()
    }
}

/// Port of Guava's `Iterators.PeekingImpl<E>`, what `Iterators.peekingIterator(it)` returns
/// (read from the 33.6.0-jre bytecode):
///
/// ```java
/// public boolean hasNext() { return hasPeeked || iterator.hasNext(); }
/// public E next() {
///     if (!hasPeeked) return iterator.next();
///     E result = peekedElement; hasPeeked = false; peekedElement = null; return result;
/// }
/// public E peek() {
///     if (!hasPeeked) { peekedElement = iterator.next(); hasPeeked = true; }
///     return peekedElement;
/// }
/// ```
///
/// `remove()` is not ported: nothing calls it, and Rust iterators have no `remove`.
#[derive(Debug)]
pub struct PeekingIterator<I: JavaIterator> {
    iterator: I,
    has_peeked: bool,
    peeked_element: Option<I::Item>,
}

impl<I: JavaIterator> PeekingIterator<I> {
    /// Port of `Iterators.peekingIterator(Iterator<? extends T> iterator)`.
    pub fn new(iterator: I) -> Self {
        PeekingIterator {
            iterator,
            has_peeked: false,
            peeked_element: None,
        }
    }

    /// Port of `hasNext()`.
    pub fn has_next(&mut self) -> bool {
        self.has_peeked || self.iterator.has_next()
    }

    /// Port of `next()`; throws as the source does when it is exhausted.
    pub fn next_elem(&mut self) -> I::Item {
        if !self.has_peeked {
            return self.iterator.next_elem();
        }
        self.has_peeked = false;
        self.peeked_element
            .take()
            .expect("hasPeeked holds an element")
    }

    /// Port of `peek()`; throws as the source does when it is exhausted.
    pub fn peek(&mut self) -> &I::Item {
        if !self.has_peeked {
            self.peeked_element = Some(self.iterator.next_elem());
            self.has_peeked = true;
        }
        self.peeked_element
            .as_ref()
            .expect("hasPeeked holds an element")
    }
}
