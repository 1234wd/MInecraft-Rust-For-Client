//! Port of: net/minecraft/nbt/CollectionTag.java
//! Java class(es): net.minecraft.nbt.CollectionTag, CollectionTag::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 21; `stream()` added and the file completed in session 24. Implemented by the three
//! array tags and (session 23) `ListTag`.
//!
//! # `iterator()` BECOMES `Iterator`, AND IT IS AN INDEX WALK, NOT A SLICE
//!
//! The anonymous `Iterator<Tag>` in the Java source holds an `index` and calls
//! `CollectionTag.this.get(this.index++)`, re-reading `size()` on every `hasNext()`. That
//! matters: `get` *constructs* a tag (`ByteArrayTag.get` returns `ByteTag.valueOf(data[i])`),
//! so iteration yields owned values rather than references into the backing array, and a
//! `size()` that shrinks mid-iteration is observed. [`CollectionTagIter`] reproduces both
//! properties.
//!
//! `next()` past the end throws `NoSuchElementException` in Java. Rust's `Iterator` returns
//! `None`, which is the same control flow at the one call site shape that exists (`for` /
//! `while let`), so the exception has nowhere to be observed from safe code and is not
//! raised. Noted rather than silently dropped.
//!
//! # `stream()` IS THE SAME INDEX WALK
//!
//! `StreamSupport.stream(this.spliterator(), false)` is a sequential, lazy, single-use
//! `java.util.stream.Stream` over the collection's spliterator, which for this interface's own
//! `iterator()` walks the indices in order. A Rust [`Iterator`] is that: lazy, single-use and
//! ordered, and every `Stream` operation the game applies to one (`map`, `filter`,
//! `forEach`, `collect`, `count`) has its `Iterator` counterpart. So [`CollectionTag::stream`]
//! returns the [`CollectionTagIter`] -- not a second traversal API. Verified by the `CSTREAM`
//! group of `sweep_probe.txt` (`tests/session24.rs`) for all four collection tags.
//!
//! # `setTag` / `addTag` RETURN A BOOLEAN INSTEAD OF THROWING
//!
//! Both answer `false` when handed a tag of the wrong kind rather than throwing -- that is
//! how `NbtOps` probes a collection without catching. The three array tags accept any
//! `NumericTag` and narrow it with that element type's `byteValue`/`intValue`/`longValue`,
//! so storing a `DoubleTag` into a `ByteArrayTag` *succeeds* and silently floors-and-masks.
//! The `COLL` oracle rows pin that.

use crate::net::minecraft::nbt::Tag::Tag;

/// Port of the `sealed interface net.minecraft.nbt.CollectionTag`.
pub trait CollectionTag {
    /// Port of `CollectionTag#clear()`.
    fn clear(&mut self);

    /// Port of `CollectionTag#setTag(int, Tag)`.
    fn set_tag(&mut self, index: i32, tag: &Tag) -> bool;

    /// Port of `CollectionTag#addTag(int, Tag)`.
    fn add_tag(&mut self, index: i32, tag: &Tag) -> bool;

    /// Port of `CollectionTag#remove(int)`.
    fn remove(&mut self, index: i32) -> Tag;

    /// Port of `CollectionTag#get(int)`.
    fn get(&self, index: i32) -> Tag;

    /// Port of `CollectionTag#size()`.
    fn size(&self) -> i32;

    /// Port of `CollectionTag#isEmpty()`.
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Port of `CollectionTag#stream()`: the elements in index order, each built by `get`.
    fn stream(&self) -> CollectionTagIter<'_, Self> {
        CollectionTagIter::new(self)
    }
}

/// Port of the anonymous `Iterator<Tag>` returned by `CollectionTag#iterator()`.
///
/// Re-reads `size()` each step and builds each element through `get` -- see the module docs.
pub struct CollectionTagIter<'a, C: CollectionTag + ?Sized> {
    collection: &'a C,
    index: i32,
}

impl<'a, C: CollectionTag + ?Sized> CollectionTagIter<'a, C> {
    /// Port of `CollectionTag#iterator()`.
    pub fn new(collection: &'a C) -> Self {
        Self {
            collection,
            index: 0,
        }
    }
}

impl<'a, C: CollectionTag + ?Sized> Iterator for CollectionTagIter<'a, C> {
    type Item = Tag;

    fn next(&mut self) -> Option<Tag> {
        // `hasNext()` is `index < size()`, evaluated fresh every time.
        if self.index < self.collection.size() {
            let tag = self.collection.get(self.index);
            self.index += 1;
            Some(tag)
        } else {
            None
        }
    }
}
