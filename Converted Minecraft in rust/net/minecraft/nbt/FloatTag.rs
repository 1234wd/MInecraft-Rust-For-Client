//! Port of: net/minecraft/nbt/FloatTag.java
//! Java class(es): net.minecraft.nbt.FloatTag, FloatTag::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 21. Every member ported: the canonical constructor, `valueOf`, `write`,
//! `sizeInBytes`, `getId`, `getType`, `copy`, both `accept` overloads, the six
//! `NumericTag` conversions, `box()` and `toString()`. Verified against the `PRIM` rows of
//! `_porting/test-data/nbt_probe.txt` (JDK 25.0.4.1, 26.2 jar) by
//! `_porting/tests/session19_g.rs`.
//!
//! # `longValue()` TRUNCATES WHERE `DoubleTag`'s FLOORS
//!
//! `FloatTag#longValue()` is `(long) this.value` -- truncation toward zero.
//! `DoubleTag#longValue()` is `(long) Math.floor(this.value)`. So
//! `FloatTag(-1.5F).longValue()` is `-1` and `DoubleTag(-1.5).longValue()` is `-2`: same
//! method name, sibling classes, opposite answers on negatives. Do not route the two
//! through a shared helper.
//!
//! Rust's `as` conversion from float to integer saturates and maps NaN to `0`, which is
//! exactly Java's narrowing primitive conversion, so `as i64` is the faithful spelling of
//! `(long)` here. `intValue()` goes through `Mth.floor` instead, which also saturates --
//! `Mth::floor_f32(f32::NEG_INFINITY)` is `i32::MIN`.
//!
//! # `valueOf`'s CACHE IS NOT OBSERVABLE HERE
//!
//! `valueOf` has no table: it returns the shared `ZERO` when `data == 0` and allocates
//! otherwise. Note the comparison is `==`, so **`-0.0` also returns `ZERO`** -- and `ZERO`
//! holds `+0.0`, so `valueOf(-0.0)` loses the sign of zero. That is vanilla's behaviour and
//! it is pinned by a unit test; `new` keeps the sign, which is why `NbtIo` round-trips use it.
//!
//! Java can tell the difference with `==`; this port cannot, because [`FloatTag`] is a `Copy`
//! value type with no identity. Nothing in vanilla compares tags with `==` -- `CompoundTag`
//! and `ListTag` use `equals` -- so the cache is an allocation optimisation with no
//! behaviour behind it, and `value_of` is kept only because callers name it.

use crate::javacompat::java_io::DataOutput;
use crate::net::minecraft::nbt::NumericTag::NumericTag;
use crate::net::minecraft::nbt::PrimitiveTag::PrimitiveTag;
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;
use crate::net::minecraft::nbt::{StreamTagVisitor, TagType};

/// Port of the private `FloatTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 12;

/// Port of `FloatTag#TYPE`.
pub const TYPE: TagType::TagTypeRef = TagType::TagTypeRef::Known(5);

/// Port of `FloatTag#ZERO`.
pub const ZERO: FloatTag = FloatTag::new(0.0);

/// Port of the `record net.minecraft.nbt.FloatTag(float value)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloatTag {
    value: f32,
}

impl FloatTag {
    /// Port of the canonical constructor `FloatTag(float)`, which vanilla marks
    /// `@Deprecated(forRemoval = true)` in favour of [`value_of`](Self::value_of).
    pub const fn new(value: f32) -> Self {
        Self { value }
    }

    /// Port of the record accessor `FloatTag#value()`.
    pub const fn value(&self) -> f32 {
        self.value
    }

    /// Port of `FloatTag#valueOf(float)`.
    ///
    /// `data == 0.0F` is true for `-0.0`, so this returns `+0.0`'s `ZERO` for negative
    /// zero. See the module docs.
    pub fn value_of(data: f32) -> Self {
        if data == 0.0 {
            ZERO
        } else {
            Self::new(data)
        }
    }


    /// Port of `FloatTag#write(DataOutput)`.
    pub fn write<O: DataOutput + ?Sized>(&self, output: &mut O) -> std::io::Result<()> {
        output.write_float(self.value)
    }

    /// Port of `FloatTag#sizeInBytes()`.
    pub const fn size_in_bytes(&self) -> i32 {
        SELF_SIZE_IN_BYTES
    }

    /// Port of `FloatTag#getId()`.
    pub const fn get_id(&self) -> i8 {
        5
    }

    /// Port of `FloatTag#getType()`.
    pub const fn get_type(&self) -> TagType::TagTypeRef {
        TYPE
    }

    /// Port of `FloatTag#copy()`.
    ///
    /// Returns `this` in Java -- the record is immutable. A `Copy` of nine bytes here.
    pub const fn copy(&self) -> Self {
        *self
    }

    /// Port of `FloatTag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        visitor.visit_float(self);
    }

    /// Port of `FloatTag#accept(StreamTagVisitor)`.
    pub fn accept_stream(
        &self,
        visitor: &mut dyn StreamTagVisitor::StreamTagVisitor,
    ) -> StreamTagVisitor::ValueResult {
        visitor.visit_float(self.value)
    }

    /// `FloatTag#hashCode()`, the record's -- which for a single-component record is the
    /// component wrapper's own `hashCode`.
    ///
    /// `Float.hashCode(v)` is `floatToIntBits(v)`, which canonicalises every NaN to `0x7FC00000` -- so a signalling NaN hashes like a quiet one while `floatValue()` still returns its raw bits.
    pub fn java_hash_code(&self) -> i32 {
        crate::javacompat::java_io::float_to_int_bits(self.value)
    }

    /// Wraps this tag as a [`Tag`].
    pub const fn into_tag(self) -> Tag {
        Tag::Float(self)
    }
}

impl PrimitiveTag for FloatTag {}

impl NumericTag for FloatTag {
    /// Port of `FloatTag#byteValue()`: `return (byte)(Mth.floor(this.value) & 0xFF);`.
    fn byte_value(&self) -> i8 {
        (crate::net::minecraft::util::Mth::Mth::floor_f32(self.value) & 0xFF) as i8
    }

    /// Port of `FloatTag#shortValue()`: `return (short)(Mth.floor(this.value) & 65535);`.
    fn short_value(&self) -> i16 {
        (crate::net::minecraft::util::Mth::Mth::floor_f32(self.value) & 65535) as i16
    }

    /// Port of `FloatTag#intValue()`: `return Mth.floor(this.value);`.
    fn int_value(&self) -> i32 {
        crate::net::minecraft::util::Mth::Mth::floor_f32(self.value)
    }

    /// Port of `FloatTag#longValue()`: `return (long)this.value;`.
    fn long_value(&self) -> i64 {
        self.value as i64
    }

    /// Port of `FloatTag#floatValue()`: `return this.value;`.
    fn float_value(&self) -> f32 {
        self.value
    }

    /// Port of `FloatTag#doubleValue()`: `return this.value;`.
    fn double_value(&self) -> f64 {
        self.value as f64
    }

    /// `box()` boxes to `java.lang.Float`.
    fn box_class_simple_name(&self) -> &'static str {
        "Float"
    }

    /// `String.valueOf(box())`.
    fn box_to_string(&self) -> String {
        crate::javacompat::java_lang::float_to_string::float_to_string(self.value)
    }
}

/// Port of `FloatTag#toString()`.
impl std::fmt::Display for FloatTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor::new();
        visitor.visit_float(self);
        f.write_str(&visitor.build())
    }
}
