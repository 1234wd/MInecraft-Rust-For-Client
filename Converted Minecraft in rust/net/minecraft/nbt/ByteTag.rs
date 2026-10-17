//! Port of: net/minecraft/nbt/ByteTag.java
//! Java class(es): net.minecraft.nbt.ByteTag, ByteTag::<1 anonymous class(es)>
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
//! `valueOf` returns a shared instance from a 256-entry table indexed `128 + data`, so every
//! `byte` value is pre-made and the constructor is never reached.
//!
//! Java can tell the difference with `==`; this port cannot, because [`ByteTag`] is a `Copy`
//! value type with no identity. Nothing in vanilla compares tags with `==` -- `CompoundTag`
//! and `ListTag` use `equals` -- so the cache is an allocation optimisation with no
//! behaviour behind it, and `value_of` is kept only because callers name it.

use crate::javacompat::java_io::DataOutput;
use crate::net::minecraft::nbt::NumericTag::NumericTag;
use crate::net::minecraft::nbt::PrimitiveTag::PrimitiveTag;
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;
use crate::net::minecraft::nbt::{StreamTagVisitor, TagType};

/// Port of the private `ByteTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 9;

/// Port of `ByteTag#TYPE`.
pub const TYPE: TagType::TagTypeRef = TagType::TagTypeRef::Known(1);

/// Port of `ByteTag#ZERO`.
pub const ZERO: ByteTag = ByteTag::new(0);
/// Port of `ByteTag#ONE`.
pub const ONE: ByteTag = ByteTag::new(1);

/// Port of the `record net.minecraft.nbt.ByteTag(byte value)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ByteTag {
    value: i8,
}

impl ByteTag {
    /// Port of the canonical constructor `ByteTag(byte)`, which vanilla marks
    /// `@Deprecated(forRemoval = true)` in favour of [`value_of`](Self::value_of).
    pub const fn new(value: i8) -> Self {
        Self { value }
    }

    /// Port of the record accessor `ByteTag#value()`.
    pub const fn value(&self) -> i8 {
        self.value
    }

    /// Port of `ByteTag#valueOf(byte)`.
    pub const fn value_of(data: i8) -> Self {
        Self::new(data)
    }

    /// Port of `ByteTag#valueOf(boolean)`.
    pub const fn value_of_bool(data: bool) -> Self {
        if data {
            ONE
        } else {
            ZERO
        }
    }


    /// Port of `ByteTag#write(DataOutput)`.
    pub fn write<O: DataOutput + ?Sized>(&self, output: &mut O) -> std::io::Result<()> {
        output.write_byte(self.value as i32)
    }

    /// Port of `ByteTag#sizeInBytes()`.
    pub const fn size_in_bytes(&self) -> i32 {
        SELF_SIZE_IN_BYTES
    }

    /// Port of `ByteTag#getId()`.
    pub const fn get_id(&self) -> i8 {
        1
    }

    /// Port of `ByteTag#getType()`.
    pub const fn get_type(&self) -> TagType::TagTypeRef {
        TYPE
    }

    /// Port of `ByteTag#copy()`.
    ///
    /// Returns `this` in Java -- the record is immutable. A `Copy` of nine bytes here.
    pub const fn copy(&self) -> Self {
        *self
    }

    /// Port of `ByteTag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        visitor.visit_byte(self);
    }

    /// Port of `ByteTag#accept(StreamTagVisitor)`.
    pub fn accept_stream(
        &self,
        visitor: &mut dyn StreamTagVisitor::StreamTagVisitor,
    ) -> StreamTagVisitor::ValueResult {
        visitor.visit_byte(self.value)
    }

    /// `ByteTag#hashCode()`, the record's -- which for a single-component record is the
    /// component wrapper's own `hashCode`.
    ///
    /// `Byte.hashCode(v)` is `(int) v`.
    pub fn java_hash_code(&self) -> i32 {
        self.value as i32
    }

    /// Wraps this tag as a [`Tag`].
    pub const fn into_tag(self) -> Tag {
        Tag::Byte(self)
    }
}

impl PrimitiveTag for ByteTag {}

impl NumericTag for ByteTag {
    /// Port of `ByteTag#byteValue()`: `return this.value;`.
    fn byte_value(&self) -> i8 {
        self.value
    }

    /// Port of `ByteTag#shortValue()`: `return this.value;`.
    fn short_value(&self) -> i16 {
        self.value as i16
    }

    /// Port of `ByteTag#intValue()`: `return this.value;`.
    fn int_value(&self) -> i32 {
        self.value as i32
    }

    /// Port of `ByteTag#longValue()`: `return this.value;`.
    fn long_value(&self) -> i64 {
        self.value as i64
    }

    /// Port of `ByteTag#floatValue()`: `return this.value;`.
    fn float_value(&self) -> f32 {
        self.value as f32
    }

    /// Port of `ByteTag#doubleValue()`: `return this.value;`.
    fn double_value(&self) -> f64 {
        self.value as f64
    }

    /// `box()` boxes to `java.lang.Byte`.
    fn box_class_simple_name(&self) -> &'static str {
        "Byte"
    }

    /// `String.valueOf(box())`.
    fn box_to_string(&self) -> String {
        self.value.to_string()
    }
}

/// Port of `ByteTag#toString()`.
impl std::fmt::Display for ByteTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor::new();
        visitor.visit_byte(self);
        f.write_str(&visitor.build())
    }
}
