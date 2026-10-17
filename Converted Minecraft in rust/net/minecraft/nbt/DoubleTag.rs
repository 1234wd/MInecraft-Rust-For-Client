//! Port of: net/minecraft/nbt/DoubleTag.java
//! Java class(es): net.minecraft.nbt.DoubleTag, DoubleTag::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 21. Every member ported: the canonical constructor, `valueOf`, `write`,
//! `sizeInBytes`, `getId`, `getType`, `copy`, both `accept` overloads, the six
//! `NumericTag` conversions, `box()` and `toString()`. Verified against the `PRIM` rows of
//! `_porting/test-data/nbt_probe.txt` (JDK 25.0.4.1, 26.2 jar) by
//! `_porting/tests/session19_g.rs`.
//!
//! # `longValue()` FLOORS WHERE `FloatTag`'s TRUNCATES
//!
//! `DoubleTag#longValue()` is `(long) Math.floor(this.value)`; `FloatTag`'s is a bare
//! `(long) this.value`. `DoubleTag(-1.5).longValue()` is `-2`, `FloatTag(-1.5F)` gives
//! `-1`. See `FloatTag`'s module docs for the same note from the other side.
//!
//! `Math.floor` on a `double` returns a `double`, so NaN survives the floor and only the
//! `(long)` cast maps it to `0`; `+/-Infinity` floor to themselves and then saturate to
//! `Long.MAX_VALUE` / `Long.MIN_VALUE`. Rust's `f64::floor` then `as i64` does both the
//! same way.
//!
//! # `valueOf`'s CACHE IS NOT OBSERVABLE HERE
//!
//! `valueOf` has no table: it returns the shared `ZERO` when `data == 0` and allocates
//! otherwise. Note the comparison is `==`, so **`-0.0` also returns `ZERO`** -- and `ZERO`
//! holds `+0.0`, so `valueOf(-0.0)` loses the sign of zero. That is vanilla's behaviour and
//! it is pinned by a unit test; `new` keeps the sign, which is why `NbtIo` round-trips use it.
//!
//! Java can tell the difference with `==`; this port cannot, because [`DoubleTag`] is a `Copy`
//! value type with no identity. Nothing in vanilla compares tags with `==` -- `CompoundTag`
//! and `ListTag` use `equals` -- so the cache is an allocation optimisation with no
//! behaviour behind it, and `value_of` is kept only because callers name it.

use crate::javacompat::java_io::DataOutput;
use crate::net::minecraft::nbt::NumericTag::NumericTag;
use crate::net::minecraft::nbt::PrimitiveTag::PrimitiveTag;
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;
use crate::net::minecraft::nbt::{StreamTagVisitor, TagType};

/// Port of the private `DoubleTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 16;

/// Port of `DoubleTag#TYPE`.
pub const TYPE: TagType::TagTypeRef = TagType::TagTypeRef::Known(6);

/// Port of `DoubleTag#ZERO`.
pub const ZERO: DoubleTag = DoubleTag::new(0.0);

/// Port of the `record net.minecraft.nbt.DoubleTag(double value)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DoubleTag {
    value: f64,
}

impl DoubleTag {
    /// Port of the canonical constructor `DoubleTag(double)`, which vanilla marks
    /// `@Deprecated(forRemoval = true)` in favour of [`value_of`](Self::value_of).
    pub const fn new(value: f64) -> Self {
        Self { value }
    }

    /// Port of the record accessor `DoubleTag#value()`.
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Port of `DoubleTag#valueOf(double)`.
    ///
    /// `data == 0.0` is true for `-0.0`, so this returns `+0.0`'s `ZERO` for negative zero.
    /// See the module docs.
    pub fn value_of(data: f64) -> Self {
        if data == 0.0 {
            ZERO
        } else {
            Self::new(data)
        }
    }


    /// Port of `DoubleTag#write(DataOutput)`.
    pub fn write<O: DataOutput + ?Sized>(&self, output: &mut O) -> std::io::Result<()> {
        output.write_double(self.value)
    }

    /// Port of `DoubleTag#sizeInBytes()`.
    pub const fn size_in_bytes(&self) -> i32 {
        SELF_SIZE_IN_BYTES
    }

    /// Port of `DoubleTag#getId()`.
    pub const fn get_id(&self) -> i8 {
        6
    }

    /// Port of `DoubleTag#getType()`.
    pub const fn get_type(&self) -> TagType::TagTypeRef {
        TYPE
    }

    /// Port of `DoubleTag#copy()`.
    ///
    /// Returns `this` in Java -- the record is immutable. A `Copy` of nine bytes here.
    pub const fn copy(&self) -> Self {
        *self
    }

    /// Port of `DoubleTag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        visitor.visit_double(self);
    }

    /// Port of `DoubleTag#accept(StreamTagVisitor)`.
    pub fn accept_stream(
        &self,
        visitor: &mut dyn StreamTagVisitor::StreamTagVisitor,
    ) -> StreamTagVisitor::ValueResult {
        visitor.visit_double(self.value)
    }

    /// `DoubleTag#hashCode()`, the record's -- which for a single-component record is the
    /// component wrapper's own `hashCode`.
    ///
    /// `Double.hashCode(v)` is `(int)(bits ^ (bits >>> 32))` over `doubleToLongBits(v)`, which canonicalises every NaN to `0x7FF8000000000000`.
    pub fn java_hash_code(&self) -> i32 {
        {
            let bits = crate::javacompat::java_io::double_to_long_bits(self.value);
            (bits ^ ((bits as u64 >> 32) as i64)) as i32
        }
    }

    /// Wraps this tag as a [`Tag`].
    pub const fn into_tag(self) -> Tag {
        Tag::Double(self)
    }
}

impl PrimitiveTag for DoubleTag {}

impl NumericTag for DoubleTag {
    /// Port of `DoubleTag#byteValue()`: `return (byte)(Mth.floor(this.value) & 0xFF);`.
    fn byte_value(&self) -> i8 {
        (crate::net::minecraft::util::Mth::Mth::floor_f64(self.value) & 0xFF) as i8
    }

    /// Port of `DoubleTag#shortValue()`: `return (short)(Mth.floor(this.value) & 65535);`.
    fn short_value(&self) -> i16 {
        (crate::net::minecraft::util::Mth::Mth::floor_f64(self.value) & 65535) as i16
    }

    /// Port of `DoubleTag#intValue()`: `return Mth.floor(this.value);`.
    fn int_value(&self) -> i32 {
        crate::net::minecraft::util::Mth::Mth::floor_f64(self.value)
    }

    /// Port of `DoubleTag#longValue()`: `return (long)Math.floor(this.value);`.
    fn long_value(&self) -> i64 {
        self.value.floor() as i64
    }

    /// Port of `DoubleTag#floatValue()`: `return (float)this.value;`.
    fn float_value(&self) -> f32 {
        self.value as f32
    }

    /// Port of `DoubleTag#doubleValue()`: `return this.value;`.
    fn double_value(&self) -> f64 {
        self.value
    }

    /// `box()` boxes to `java.lang.Double`.
    fn box_class_simple_name(&self) -> &'static str {
        "Double"
    }

    /// `String.valueOf(box())`.
    fn box_to_string(&self) -> String {
        crate::javacompat::java_lang::float_to_string::double_to_string(self.value)
    }
}

/// Port of `DoubleTag#toString()`.
impl std::fmt::Display for DoubleTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor::new();
        visitor.visit_double(self);
        f.write_str(&visitor.build())
    }
}
