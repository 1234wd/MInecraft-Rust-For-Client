//! Port of: net/minecraft/nbt/ShortTag.java
//! Java class(es): net.minecraft.nbt.ShortTag, ShortTag::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 21. Every member ported: the canonical constructor, `valueOf`, `write`,
//! `sizeInBytes`, `getId`, `getType`, `copy`, both `accept` overloads, the six
//! `NumericTag` conversions, `box()` and `toString()`. Verified against the `PRIM` rows of
//! `_porting/test-data/nbt_probe.txt` (JDK 25.0.4.1, 26.2 jar) by
//! `_porting/tests/session19_g.rs`.
//!
//!
//! # `valueOf`'s CACHE IS NOT OBSERVABLE HERE
//!
//! `valueOf` returns a shared instance for `-128..=1024` and allocates outside that range, so
//! two tags with the same out-of-range value are distinct objects in Java.
//!
//! Java can tell the difference with `==`; this port cannot, because [`ShortTag`] is a `Copy`
//! value type with no identity. Nothing in vanilla compares tags with `==` -- `CompoundTag`
//! and `ListTag` use `equals` -- so the cache is an allocation optimisation with no
//! behaviour behind it, and `value_of` is kept only because callers name it.

use crate::javacompat::java_io::DataOutput;
use crate::net::minecraft::nbt::NumericTag::NumericTag;
use crate::net::minecraft::nbt::PrimitiveTag::PrimitiveTag;
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;
use crate::net::minecraft::nbt::{StreamTagVisitor, TagType};

/// Port of the private `ShortTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 10;

/// Port of `ShortTag#TYPE`.
pub const TYPE: TagType::TagTypeRef = TagType::TagTypeRef::Known(2);

/// Port of the `record net.minecraft.nbt.ShortTag(short value)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShortTag {
    value: i16,
}

impl ShortTag {
    /// Port of the canonical constructor `ShortTag(short)`, which vanilla marks
    /// `@Deprecated(forRemoval = true)` in favour of [`value_of`](Self::value_of).
    pub const fn new(value: i16) -> Self {
        Self { value }
    }

    /// Port of the record accessor `ShortTag#value()`.
    pub const fn value(&self) -> i16 {
        self.value
    }

    /// Port of `ShortTag#valueOf(short)`.
    pub const fn value_of(i: i16) -> Self {
        Self::new(i)
    }


    /// Port of `ShortTag#write(DataOutput)`.
    pub fn write<O: DataOutput + ?Sized>(&self, output: &mut O) -> std::io::Result<()> {
        output.write_short(self.value as i32)
    }

    /// Port of `ShortTag#sizeInBytes()`.
    pub const fn size_in_bytes(&self) -> i32 {
        SELF_SIZE_IN_BYTES
    }

    /// Port of `ShortTag#getId()`.
    pub const fn get_id(&self) -> i8 {
        2
    }

    /// Port of `ShortTag#getType()`.
    pub const fn get_type(&self) -> TagType::TagTypeRef {
        TYPE
    }

    /// Port of `ShortTag#copy()`.
    ///
    /// Returns `this` in Java -- the record is immutable. A `Copy` of nine bytes here.
    pub const fn copy(&self) -> Self {
        *self
    }

    /// Port of `ShortTag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        visitor.visit_short(self);
    }

    /// Port of `ShortTag#accept(StreamTagVisitor)`.
    pub fn accept_stream(
        &self,
        visitor: &mut dyn StreamTagVisitor::StreamTagVisitor,
    ) -> StreamTagVisitor::ValueResult {
        visitor.visit_short(self.value)
    }

    /// `ShortTag#hashCode()`, the record's -- which for a single-component record is the
    /// component wrapper's own `hashCode`.
    ///
    /// `Short.hashCode(v)` is `(int) v`.
    pub fn java_hash_code(&self) -> i32 {
        self.value as i32
    }

    /// Wraps this tag as a [`Tag`].
    pub const fn into_tag(self) -> Tag {
        Tag::Short(self)
    }
}

impl PrimitiveTag for ShortTag {}

impl NumericTag for ShortTag {
    /// Port of `ShortTag#byteValue()`: `return (byte)(this.value & 0xFF);`.
    fn byte_value(&self) -> i8 {
        (self.value as i32 & 0xFF) as i8
    }

    /// Port of `ShortTag#shortValue()`: `return this.value;`.
    fn short_value(&self) -> i16 {
        self.value
    }

    /// Port of `ShortTag#intValue()`: `return this.value;`.
    fn int_value(&self) -> i32 {
        self.value as i32
    }

    /// Port of `ShortTag#longValue()`: `return this.value;`.
    fn long_value(&self) -> i64 {
        self.value as i64
    }

    /// Port of `ShortTag#floatValue()`: `return this.value;`.
    fn float_value(&self) -> f32 {
        self.value as f32
    }

    /// Port of `ShortTag#doubleValue()`: `return this.value;`.
    fn double_value(&self) -> f64 {
        self.value as f64
    }

    /// `box()` boxes to `java.lang.Short`.
    fn box_class_simple_name(&self) -> &'static str {
        "Short"
    }

    /// `String.valueOf(box())`.
    fn box_to_string(&self) -> String {
        self.value.to_string()
    }
}

/// Port of `ShortTag#toString()`.
impl std::fmt::Display for ShortTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor::new();
        visitor.visit_short(self);
        f.write_str(&visitor.build())
    }
}
