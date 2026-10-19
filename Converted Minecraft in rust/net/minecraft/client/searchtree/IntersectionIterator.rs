//! Port of: net/minecraft/client/searchtree/IntersectionIterator.java
//! Java class(es): net.minecraft.client.searchtree.IntersectionIterator
//! Status: VERIFIED
//!
//! Session 24. `public class IntersectionIterator<T> extends AbstractIterator<T>`: the elements two
//! SORTED iterators have in common, in order -- the search tree's AND of two result lists.
//!
//! ```java
//! while (first.hasNext() && second.hasNext()) {
//!     int compare = comparator.compare(first.peek(), second.peek());
//!     if (compare == 0) { second.next(); return first.next(); }
//!     if (compare < 0) first.next(); else second.next();
//! }
//! return endOfData();
//! ```
//!
//! On a match the element from the FIRST iterator is returned (the second's equal one is
//! dropped), so when the comparator ties two different objects, which one comes out matters.
//!
//! # Shape of the port
//!
//! * `Iterators.peekingIterator(it)` is `javacompat::guava::PeekingIterator`, Guava's
//!   `PeekingImpl` transcribed: its `hasNext()` asks the source's `hasNext()` WITHOUT taking an
//!   element. So the sources are `javacompat::guava::JavaIterator`s (`ExactIter` over a `Vec`'s
//!   iterator is a Java `List.iterator()`), and the number of elements each source hands out is
//!   Java's: with an empty second source, nothing at all is taken from the first;
//! * the Guava `AbstractIterator` base -- `hasNext`/`next`/`peek` and its READY/NOT_READY/DONE/
//!   FAILED states -- is `javacompat::guava::AbstractIterator`, and [`IntersectionIterator`]
//!   is that type over this file's `computeNext` ([`Intersection`]). It is itself a
//!   `JavaIterator` (as `FullTextSearchTree` feeds a `MergingUniqueIterator` into one) and a Rust
//!   `Iterator`;
//! * `Comparator<T>` is a closure returning [`std::cmp::Ordering`]: the Java code only tests the
//!   sign of `compare`, which is exactly what an `Ordering` carries.
//!
//! Verified by the `SEARCHIT` group of `sweep_probe.txt` (`tests/session24.rs`): results, how
//! many elements each source handed out, how many comparisons ran, `peek`/`next` past the end,
//! and a comparator that throws (the iterator is then FAILED: `IllegalStateException`).

use std::cmp::Ordering;

use crate::javacompat::guava::{AbstractIterator, ComputeNext, JavaIterator, PeekingIterator};

/// The fields and `computeNext()` of `IntersectionIterator`.
pub struct Intersection<I1: JavaIterator, I2: JavaIterator<Item = I1::Item>, C> {
    first_iterator: PeekingIterator<I1>,
    second_iterator: PeekingIterator<I2>,
    comparator: C,
}

impl<I1, I2, C> ComputeNext for Intersection<I1, I2, C>
where
    I1: JavaIterator,
    I2: JavaIterator<Item = I1::Item>,
    C: FnMut(&I1::Item, &I1::Item) -> Ordering,
{
    type Item = I1::Item;

    /// Port of `protected T computeNext()`.
    fn compute_next(&mut self) -> Option<I1::Item> {
        while self.first_iterator.has_next() && self.second_iterator.has_next() {
            let compare = {
                let a = self.first_iterator.peek();
                let b = self.second_iterator.peek();
                (self.comparator)(a, b)
            };
            if compare == Ordering::Equal {
                self.second_iterator.next_elem();
                return Some(self.first_iterator.next_elem());
            }
            if compare == Ordering::Less {
                self.first_iterator.next_elem();
            } else {
                self.second_iterator.next_elem();
            }
        }
        None
    }
}

/// Port of `net.minecraft.client.searchtree.IntersectionIterator<T>`.
pub type IntersectionIterator<I1, I2, C> = AbstractIterator<Intersection<I1, I2, C>>;

/// Port of `public IntersectionIterator(Iterator<T> firstIterator, Iterator<T> secondIterator,
/// Comparator<T> comparator)`.
pub fn new<I1, I2, C>(
    first_iterator: I1,
    second_iterator: I2,
    comparator: C,
) -> IntersectionIterator<I1, I2, C>
where
    I1: JavaIterator,
    I2: JavaIterator<Item = I1::Item>,
    C: FnMut(&I1::Item, &I1::Item) -> Ordering,
{
    AbstractIterator::new(Intersection {
        first_iterator: PeekingIterator::new(first_iterator),
        second_iterator: PeekingIterator::new(second_iterator),
        comparator,
    })
}
