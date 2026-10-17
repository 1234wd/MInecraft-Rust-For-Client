//! Port of: net/minecraft/nbt/LongTag.java
//! Java class(es): net.minecraft.nbt.LongTag, LongTag::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 21. Every member ported: the canonical constructor, `valueOf`, `write`,
//! `sizeInBytes`, `getId`, `getType`, `copy`, both `accept` overloads, the six
//! `NumericTag` conversions, `box()` and `toString()`. Verified against the `PRIM` rows of
//! `_porting/test-data/nbt_probe.txt` (JDK 25.0.4.1, 26.2 jar) by
//! `_porting/tests/session19_g.rs`.
//!
//! # `intValue()` MASKS WITH `-1L`, WHICH IS A NO-OP
//!
//! `return (int)(this.value & -1L);` -- `& -1L` sets no bits, so this is `(int) value`: the
//! low 32 bits, **truncated, not clamped**. `LongTag(Long.MIN_VALUE).intValue()` is `0`,
//! which the oracle confirms. The mask is reproduced verbatim so this file still diffs
//! against the Java source by eye.
//!
//! # `valueOf`'s CACHE IS NOT OBSERVABLE HERE
//!
//! `valueOf` returns a shared instance for `-128..=1024` and allocates outside that range, so
//! two tags with the same out-of-range value are distinct objects in Java.
//!
//! Java can tell the difference with `==`; this port cannot, because [`LongTag`] is a `Copy`
//! value type with no identity. Nothing in vanilla compares tags with `==` -- `CompoundTag`
//! and `ListTag` use `equals` -- so the cache is an allocation optimisation with no
//! behaviour behind it, and `value_of` is kept only because callers name it.

use crate::javacompat::java_io::DataOutput;
use crate::net::minecraft::nbt::NumericTag::NumericTag;
use crate::net::minecraft::nbt::PrimitiveTag::PrimitiveTag;
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;
use crate::net::minecraft::nbt::{StreamTagVisitor, TagType};

/// Port of the private `LongTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 16;

/// Port of `LongTag#TYPE`.
pub const TYPE: TagType::TagTypeRef = TagType::TagTypeRef::Known(4);

/// Port of the `record net.minecraft.nbt.LongTag(long value)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LongTag {
    value: i64,
}

impl LongTag {
    /// Port of the canonical constructor `LongTag(long)`, which vanilla marks
    /// `@Deprecated(forRemoval = true)` in favour of [`value_of`](Self::value_of).
    pub const fn new(value: i64) -> Self {
        Self { value }
    }

    /// Port of the record accessor `LongTag#value()`.
    pub const fn value(&self) -> i64 {
        self.value
    }

    /// Port of `LongTag#valueOf(long)`.
    pub const fn value_of(i: i64) -> Self {
        Self::new(i)
    }


    /// Port of `LongTag#write(DataOutput)`.
    pub fn write<O: DataOutput + ?Sized>(&self, output: &mut O) -> std::io::Result<()> {
        output.write_long(self.value)
    }

    /// Port of `LongTag#sizeInBytes()`.
    pub const fn size_in_bytes(&self) -> i32 {
        SELF_SIZE_IN_BYTES
    }

    /// Port of `LongTag#getId()`.
    pub const fn get_id(&self) -> i8 {
        4
    }

    /// Port of `LongTag#getType()`.
    pub const fn get_type(&self) -> TagType::TagTypeRef {
        TYPE
    }

    /// Port of `LongTag#copy()`.
    ///
    /// Returns `this` in Java -- the record is immutable. A `Copy` of nine bytes here.
    pub const fn copy(&self) -> Self {
        *self
    }

    /// Port of `LongTag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        visitor.visit_long(self);
    }

    /// Port of `LongTag#accept(StreamTagVisitor)`.
    pub fn accept_stream(
        &self,
        visitor: &mut dyn StreamTagVisitor::StreamTagVisitor,
    ) -> StreamTagVisitor::ValueResult {
        visitor.visit_long(self.value)
    }

    /// `LongTag#hashCode()`, the record's -- which for a single-component record is the
    /// component wrapper's own `hashCode`.
    ///
    /// `Long.hashCode(v)` is `(int)(v ^ (v >>> 32))`.
    pub fn java_hash_code(&self) -> i32 {
        (self.value ^ ((self.value as u64 >> 32) as i64)) as i32
    }

    /// Wraps this tag as a [`Tag`].
    pub const fn into_tag(self) -> Tag {
        Tag::Long(self)
    }
}

impl PrimitiveTag for LongTag {}

impl NumericTag for LongTag {
    /// Port of `LongTag#byteValue()`: `return (byte)(this.value & 255L);`.
    fn byte_value(&self) -> i8 {
        (self.value & 255) as i8
    }

    /// Port of `LongTag#shortValue()`: `return (short)(this.value & 65535L);`.
    fn short_value(&self) -> i16 {
        (self.value & 65535) as i16
    }

    /// Port of `LongTag#intValue()`: `return (int)(this.value & -1L);`.
    fn int_value(&self) -> i32 {
        (self.value & -1i64) as i32
    }

    /// Port of `LongTag#longValue()`: `return this.value;`.
    fn long_value(&self) -> i64 {
        self.value
    }

    /// Port of `LongTag#floatValue()`: `return (float)this.value;`.
    fn float_value(&self) -> f32 {
        self.value as f32
    }

    /// Port of `LongTag#doubleValue()`: `return this.value;`.
    fn double_value(&self) -> f64 {
        self.value as f64
    }

    /// `box()` boxes to `java.lang.Long`.
    fn box_class_simple_name(&self) -> &'static str {
        "Long"
    }

    /// `String.valueOf(box())`.
    fn box_to_string(&self) -> String {
        self.value.to_string()
    }
}

/// Port of `LongTag#toString()`.
impl std::fmt::Display for LongTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor::new();
        visitor.visit_long(self);
        f.write_str(&visitor.build())
    }
}
