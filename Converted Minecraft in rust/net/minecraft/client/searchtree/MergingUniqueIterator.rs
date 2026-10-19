//! Port of: net/minecraft/client/searchtree/MergingUniqueIterator.java
//! Java class(es): net.minecraft.client.searchtree.MergingUniqueIterator
//! Status: VERIFIED
//!
//! Session 24. `public class MergingUniqueIterator<T> extends AbstractIterator<T>`: the sorted
//! union of two SORTED iterators, an element the comparator ties across the two coming out once --
//! the search tree's OR of two result lists.
//!
//! ```java
//! boolean firstEmpty = !first.hasNext(), secondEmpty = !second.hasNext();
//! if (firstEmpty && secondEmpty) return endOfData();
//! if (firstEmpty) return second.next();
//! if (secondEmpty) return first.next();
//! int compare = comparator.compare(first.peek(), second.peek());
//! if (compare == 0) second.next();
//! return compare <= 0 ? first.next() : second.next();
//! ```
//!
//! * a tie consumes BOTH heads and returns the FIRST iterator's element;
//! * only ties ACROSS the two iterators are merged: a duplicate within one iterator comes out
//!   twice, and once one side is exhausted the other is passed through unchanged.
//!
//! The shape of the port is `IntersectionIterator`'s (this session): Guava's `PeekingImpl` as
//! `javacompat::guava::PeekingIterator` over `JavaIterator` sources (so `hasNext` takes nothing
//! from a source), `javacompat::guava::AbstractIterator` for the base class, and an
//! [`std::cmp::Ordering`] closure for the `Comparator`, whose sign is all the Java code reads.
//!
//! Verified by the `SEARCHIT` group of `sweep_probe.txt` (`tests/session24.rs`).

use std::cmp::Ordering;

use crate::javacompat::guava::{AbstractIterator, ComputeNext, JavaIterator, PeekingIterator};

/// The fields and `computeNext()` of `MergingUniqueIterator`.
pub struct MergingUnique<I1: JavaIterator, I2: JavaIterator<Item = I1::Item>, C> {
    first_iterator: PeekingIterator<I1>,
    second_iterator: PeekingIterator<I2>,
    comparator: C,
}

impl<I1, I2, C> ComputeNext for MergingUnique<I1, I2, C>
where
    I1: JavaIterator,
    I2: JavaIterator<Item = I1::Item>,
    C: FnMut(&I1::Item, &I1::Item) -> Ordering,
{
    type Item = I1::Item;

    /// Port of `protected T computeNext()`.
    fn compute_next(&mut self) -> Option<I1::Item> {
        let first_empty = !self.first_iterator.has_next();
        let second_empty = !self.second_iterator.has_next();
        if first_empty && second_empty {
            return None;
        }
        if first_empty {
            return Some(self.second_iterator.next_elem());
        }
        if second_empty {
            return Some(self.first_iterator.next_elem());
        }
        let compare = {
            let a = self.first_iterator.peek();
            let b = self.second_iterator.peek();
            (self.comparator)(a, b)
        };
        if compare == Ordering::Equal {
            self.second_iterator.next_elem();
        }
        Some(if compare != Ordering::Greater {
            self.first_iterator.next_elem()
        } else {
            self.second_iterator.next_elem()
        })
    }
}

/// Port of `net.minecraft.client.searchtree.MergingUniqueIterator<T>`.
pub type MergingUniqueIterator<I1, I2, C> = AbstractIterator<MergingUnique<I1, I2, C>>;

/// Port of `public MergingUniqueIterator(Iterator<T> firstIterator, Iterator<T> secondIterator,
/// Comparator<T> comparator)`.
pub fn new<I1, I2, C>(
    first_iterator: I1,
    second_iterator: I2,
    comparator: C,
) -> MergingUniqueIterator<I1, I2, C>
where
    I1: JavaIterator,
    I2: JavaIterator<Item = I1::Item>,
    C: FnMut(&I1::Item, &I1::Item) -> Ordering,
{
    AbstractIterator::new(MergingUnique {
        first_iterator: PeekingIterator::new(first_iterator),
        second_iterator: PeekingIterator::new(second_iterator),
        comparator,
    })
}
