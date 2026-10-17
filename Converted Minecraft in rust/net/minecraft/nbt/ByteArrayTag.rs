//! Port of: net/minecraft/nbt/ByteArrayTag.java
//! Java class(es): net.minecraft.nbt.ByteArrayTag, ByteArrayTag::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 21. Every member ported: the constructor, `TYPE` (load/parse/skip/names),
//! `write`, `sizeInBytes`, `getId`, `getType`, `toString`, `copy`, `equals`, `hashCode`,
//! both `accept` overloads, `getAsByteArray`, and the six `CollectionTag` operations.
//! Verified against the `COLL`, `OTHER` and `CASEW` rows of
//! `_porting/test-data/nbt_probe.txt` (JDK 25.0.4.1, 26.2 jar) by
//! `_porting/tests/session19_g.rs`.
//!
//! # `setTag` / `addTag` ACCEPT ANY NUMERIC TAG AND NARROW IT SILENTLY
//!
//! ```java
//! if (tag instanceof NumericTag numeric) {
//!     this.data[index] = numeric.byteValue();
//!     return true;
//! } else {
//!     return false;
//! }
//! ```
//!
//! So putting a `DoubleTag(3.9)` into a `ByteArrayTag` **succeeds** and stores
//! `byteValue()`'s answer -- which for the floating kinds means a `Mth.floor` and a mask, not
//! a rounding and not a rejection. Only a non-numeric tag (a string, a nested array)
//! returns `false`. That is how `NbtOps` writes through a collection without type-checking
//! first, and it means a data fixer can quietly truncate. The `COLL` oracle rows pin both
//! the `true` and the `false` branch.
//!
//! # `sizeInBytes` AND `skip` BOTH USE WRAPPING `int` ARITHMETIC
//!
//! `24 + 1 * this.data.length` and `input.skipBytes(input.readInt() * 1)` are plain
//! Java `int` expressions. A length near `Integer.MAX_VALUE` wraps rather than widening, so
//! `skip` can seek **backwards** on a hostile stream. Reproduced with `wrapping_mul` /
//! `wrapping_add`: the quota check in `load` is what actually stops such a stream, and
//! moving the overflow guard into `skip` would change which files the game rejects.
//!
//! # `load` CHARGES BEFORE IT ALLOCATES, AND A NEGATIVE LENGTH THROWS FIRST
//!
//! `accountBytes(24L)`, then `readInt()`, then `accountBytes(1L, length)`, then allocate.
//! With a negative length the second charge is negative, so
//! `IllegalArgumentException("Tried to account NBT tag with negative size: ...")` is raised
//! **before** Java would get to `new byte[length]` and its `NegativeArraySizeException`.
//! The exception a hostile file produces is therefore the accounter's, not the array's.
//!
//! # `copy()` IS A DEEP COPY; `equals` IS BY CONTENT
//!
//! `System.arraycopy` into a fresh array, `Arrays.equals`, and `Arrays.hashCode` -- which is
//! `h = 31*h + element` from `1`, **not** from `0`, and `1` for an empty array. Written out
//! in [`java_hash_code`](Self::java_hash_code) because the seed is the part that gets
//! guessed wrong.

use crate::javacompat::java_io::{DataInput, DataOutput};
use crate::net::minecraft::nbt::CollectionTag::{CollectionTag, CollectionTagIter};
use crate::net::minecraft::nbt::NbtAccounter::NbtAccounter;
use crate::net::minecraft::nbt::StreamTagVisitor::{StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagType::TagTypeRef;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;
use crate::net::minecraft::nbt::ByteTag::ByteTag;

/// Port of the private `ByteArrayTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 24;

/// Port of the per-element byte width used by `sizeInBytes`, `skip` and the accounter.
const ELEMENT_BYTES: i32 = 1;

/// Port of `ByteArrayTag#TYPE`.
pub const TYPE: TagTypeRef = TagTypeRef::Known(7);

/// Port of `net.minecraft.nbt.ByteArrayTag`.
#[derive(Clone, Debug)]
pub struct ByteArrayTag {
    data: Vec<i8>,
}

impl ByteArrayTag {
    /// Port of `ByteArrayTag(byte[])`.
    pub fn new(data: Vec<i8>) -> Self {
        Self { data }
    }

    /// Port of the anonymous `TYPE.load`'s private `readAccounted(DataInput, NbtAccounter)`.
    ///
    /// Charges, reads the length, charges per element, then reads. See the module docs for
    /// why a negative length throws from the accounter.
    pub fn read_accounted<I: DataInput + ?Sized>(
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<Vec<i8>> {
        accounter.account_bytes(SELF_SIZE_IN_BYTES as i64);
        let length = input.read_int()?;
        accounter.account_bytes_per_entry(ELEMENT_BYTES as i64, length as i64);
        let mut data = vec![0i8; length as usize];
        // `input.readFully(data)` -- a bulk read, so a short stream raises EOFException
        // part-way rather than leaving zeroes behind.
        {
            let mut raw = vec![0u8; data.len()];
            input.read_fully(&mut raw)?;
            for (slot, b) in data.iter_mut().zip(raw) {
                *slot = b as i8;
            }
        }
        Ok(data)
    }

    /// Port of the anonymous `TYPE.skip(DataInput, NbtAccounter)`.
    pub fn skip<I: DataInput + ?Sized>(input: &mut I) -> std::io::Result<()> {
        let length = input.read_int()?;
        input.skip_bytes(length.wrapping_mul(ELEMENT_BYTES))?;
        Ok(())
    }

    /// Port of `ByteArrayTag#write(DataOutput)`.
    pub fn write<O: DataOutput + ?Sized>(&self, output: &mut O) -> std::io::Result<()> {
        output.write_int(self.data.len() as i32)?;
        // `output.write(this.data)` -- the whole array in one call.
        {
            let raw: Vec<u8> = self.data.iter().map(|&b| b as u8).collect();
            output.write(&raw)?;
        }
        Ok(())
    }

    /// Port of `ByteArrayTag#sizeInBytes()`.
    pub fn size_in_bytes(&self) -> i32 {
        SELF_SIZE_IN_BYTES.wrapping_add(ELEMENT_BYTES.wrapping_mul(self.data.len() as i32))
    }

    /// Port of `ByteArrayTag#getId()`.
    pub const fn get_id(&self) -> i8 {
        7
    }

    /// Port of `ByteArrayTag#getType()`.
    pub const fn get_type(&self) -> TagTypeRef {
        TYPE
    }

    /// Port of `ByteArrayTag#copy()`.
    ///
    /// `System.arraycopy` into a fresh array -- a deep copy, unlike the primitive tags.
    pub fn copy(&self) -> Self {
        Self { data: self.data.clone() }
    }

    /// Port of `ByteArrayTag#getAsByteArray()`.
    ///
    /// Java hands back the **live** array, so a caller can mutate the tag through it. The
    /// port returns a shared slice; [`get_as_byte_array_mut`](Self::get_as_byte_array_mut)
    /// is the aliasing-free way to get the mutable view Java's callers use.
    pub fn get_as_byte_array(&self) -> &[i8] {
        &self.data
    }

    /// The mutable view of `getAsByteArray()`'s live array.
    pub fn get_as_byte_array_mut(&mut self) -> &mut [i8] {
        &mut self.data
    }

    /// Port of `ByteArrayTag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        visitor.visit_byte_array(self);
    }

    /// Port of `ByteArrayTag#accept(StreamTagVisitor)`.
    pub fn accept_stream(&self, visitor: &mut dyn StreamTagVisitor) -> ValueResult {
        visitor.visit_byte_array(&self.data)
    }

    /// Port of `ByteArrayTag#hashCode()`: `Arrays.hashCode(byte[])`.
    ///
    /// Seeded with `1`, not `0`, so an empty array hashes to `1`.
    pub fn java_hash_code(&self) -> i32 {
        let mut h: i32 = 1;
        for v in &self.data {
            h = h.wrapping_mul(31).wrapping_add(*v as i32);
        }
        h
    }

    /// Port of `ByteArrayTag#iterator()`, inherited from `CollectionTag`.
    pub fn iter(&self) -> CollectionTagIter<'_, Self> {
        CollectionTagIter::new(self)
    }

    /// Wraps this tag as a [`Tag`].
    pub fn into_tag(self) -> Tag {
        Tag::ByteArray(self)
    }
}

/// Port of `ByteArrayTag#equals(Object)`: `Arrays.equals`.
impl PartialEq for ByteArrayTag {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl CollectionTag for ByteArrayTag {
    /// Port of `ByteArrayTag#clear()`.
    ///
    /// `this.data = new byte[0]` -- a fresh empty array, not a truncation in place.
    fn clear(&mut self) {
        self.data = Vec::new();
    }

    /// Port of `ByteArrayTag#setTag(int, Tag)`.
    ///
    /// Narrows any numeric tag. See the module docs.
    fn set_tag(&mut self, index: i32, tag: &Tag) -> bool {
        match tag.as_numeric() {
            Some(numeric) => {
                let value = numeric.byte_value();
                let len = self.data.len();
                if index < 0 || index as usize >= len {
                    panic!(
                        "java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {len}"
                    );
                }
                self.data[index as usize] = value;
                true
            }
            None => false,
        }
    }

    /// Port of `ByteArrayTag#addTag(int, Tag)`.
    ///
    /// `ArrayUtils.add(array, index, value)` from commons-lang3, which throws
    /// `IndexOutOfBoundsException` with its own message -- not the JDK's -- for an index
    /// outside `0..=length`. Note the upper bound is *inclusive*: appending at `length` is
    /// legal.
    fn add_tag(&mut self, index: i32, tag: &Tag) -> bool {
        match tag.as_numeric() {
            Some(numeric) => {
                let value = numeric.byte_value();
                let len = self.data.len();
                if index < 0 || index as usize > len {
                    panic!(
                        "java.lang.IndexOutOfBoundsException: Index: {index}, Length: {len}"
                    );
                }
                self.data.insert(index as usize, value);
                true
            }
            None => false,
        }
    }

    /// Port of `ByteArrayTag#remove(int)`.
    ///
    /// Returns the removed element as a tag, after shrinking the array.
    fn remove(&mut self, index: i32) -> Tag {
        let len = self.data.len();
        if index < 0 || index as usize >= len {
            panic!(
                "java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {len}"
            );
        }
        let prev = self.data.remove(index as usize);
        ByteTag::value_of(prev).into_tag()
    }

    /// Port of `ByteArrayTag#get(int)`.
    fn get(&self, index: i32) -> Tag {
        let len = self.data.len();
        if index < 0 || index as usize >= len {
            panic!(
                "java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {len}"
            );
        }
        ByteTag::value_of(self.data[index as usize]).into_tag()
    }

    /// Port of `ByteArrayTag#size()`.
    fn size(&self) -> i32 {
        self.data.len() as i32
    }
}

/// Port of `ByteArrayTag#toString()`.
impl std::fmt::Display for ByteArrayTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor::new();
        visitor.visit_byte_array(self);
        f.write_str(&visitor.build())
    }
}
