//! Port of: net/minecraft/nbt/Tag.java
//! Java class(es): net.minecraft.nbt.Tag
//! Status: PARTIAL
//!
//! Session 21. The constants, the tag-value representation and every `default` method of
//! the interface are ported. **Two of the thirteen tag kinds are not yet in the enum**:
//! `ListTag` and `CompoundTag`. See "What is missing" below.
//!
//! # A `sealed interface` BECOMES AN ENUM, NOT A TRAIT OBJECT
//!
//! ```java
//! public sealed interface Tag permits CompoundTag, CollectionTag, PrimitiveTag, EndTag {
//! ```
//!
//! `sealed` means the implementor set is closed at compile time, which is exactly what a
//! Rust `enum` expresses and exactly what `dyn Tag` does not. The choice is forced by the
//! containers rather than by taste: `CompoundTag` holds a `Map<String, Tag>` and `ListTag`
//! a `List<Tag>`, so a tag has to be an owned, sized, cloneable, comparable value.
//! `Box<dyn Tag>` would give up `Clone` (needed by `copy()`), `PartialEq` (needed by
//! `equals`, which the `OTHER` oracle rows check) and `Hash`, and would allocate for every
//! `ByteTag` in a byte-heavy chunk file.
//!
//! The per-kind data still lives in the per-class files the mirror requires -- `ByteTag.rs`
//! defines `ByteTag` -- and this enum only names them. So `mirror.py` stays 1:1 and the
//! match arms below read like Java's `permits` clause.
//!
//! # `Optional<T>` BECOMES `Option<T>`, AND THE DEFAULTS ARE REAL CODE
//!
//! The dozen `asX()` defaults are not accessors; they are a conversion table with teeth.
//! `asByte()` is `asNumber().map(Number::byteValue)`, so for a `DoubleTag` it runs
//! `Mth.floor` and then masks -- `DoubleTag(-1e30).asByte()` is `Some(0)`, not `None` and
//! not a saturated `-128`. Each numeric kind's narrowing lives in its own file; this one
//! just routes.
//!
//! `asBoolean()` is `asByte().map(b -> b != 0)` on `Tag`, but `NumericTag` overrides it to
//! `Optional.of(this.byteValue() != 0)`. Same answer for every numeric kind, and the
//! override only matters because `Tag`'s version would route through `asNumber`. Reproduced
//! as written rather than collapsed.
//!
//! # What is missing, and why
//!
//! `ListTag` and `CompoundTag` are still SKELETON, so [`Tag`] has no variant for them and
//! `asList()` / `asCompound()` are **not** ported. They could only ever return `None`
//! here, and a method that is hard-wired to one answer is worse than an absent one: a
//! caller writing `if let Some(c) = tag.as_compound()` would compile, never fire, and look
//! correct. Leaving them out makes that a compile error instead.
//!
//! `CompoundTag` is not merely "more of the same". It wraps a `java.util.HashMap`, and
//! `CompoundTag.write` serialises in **map iteration order**, so byte-exact NBT output
//! needs a map that reproduces JDK bucket order -- hash spreading, resize thresholds and
//! treeification included. `nbt_probe.txt` already carries the `HMOP`/`HMRES`/`HMCOPY`
//! rows for it. That is the next batch, deliberately not started here rather than
//! half-done.
//!
//! Consequently [`accept`](Tag::accept) covers eleven kinds and
//! [`crate::net::minecraft::nbt::TagVisitor`] declares eleven methods. When the containers
//! land, both grow and every `match` in the crate stops compiling until it handles them --
//! which is the point of using an enum.

use crate::net::minecraft::nbt::ByteArrayTag::ByteArrayTag;
use crate::net::minecraft::nbt::ByteTag::ByteTag;
use crate::net::minecraft::nbt::DoubleTag::DoubleTag;
use crate::net::minecraft::nbt::FloatTag::FloatTag;
use crate::net::minecraft::nbt::IntArrayTag::IntArrayTag;
use crate::net::minecraft::nbt::IntTag::IntTag;
use crate::net::minecraft::nbt::LongArrayTag::LongArrayTag;
use crate::net::minecraft::nbt::LongTag::LongTag;
use crate::net::minecraft::nbt::NumericTag::NumericTag;
use crate::net::minecraft::nbt::ShortTag::ShortTag;
use crate::net::minecraft::nbt::StringTag::StringTag;
use crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor;
use crate::net::minecraft::nbt::TagType::TagTypeRef;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;

/// Port of `Tag#OBJECT_HEADER`.
pub const OBJECT_HEADER: i32 = 8;
/// Port of `Tag#ARRAY_HEADER`.
pub const ARRAY_HEADER: i32 = 12;
/// Port of `Tag#OBJECT_REFERENCE`.
pub const OBJECT_REFERENCE: i32 = 4;
/// Port of `Tag#STRING_SIZE`.
pub const STRING_SIZE: i32 = 28;

/// Port of `Tag#TAG_END`.
pub const TAG_END: i8 = 0;
/// Port of `Tag#TAG_BYTE`.
pub const TAG_BYTE: i8 = 1;
/// Port of `Tag#TAG_SHORT`.
pub const TAG_SHORT: i8 = 2;
/// Port of `Tag#TAG_INT`.
pub const TAG_INT: i8 = 3;
/// Port of `Tag#TAG_LONG`.
pub const TAG_LONG: i8 = 4;
/// Port of `Tag#TAG_FLOAT`.
pub const TAG_FLOAT: i8 = 5;
/// Port of `Tag#TAG_DOUBLE`.
pub const TAG_DOUBLE: i8 = 6;
/// Port of `Tag#TAG_BYTE_ARRAY`.
pub const TAG_BYTE_ARRAY: i8 = 7;
/// Port of `Tag#TAG_STRING`.
pub const TAG_STRING: i8 = 8;
/// Port of `Tag#TAG_LIST`.
pub const TAG_LIST: i8 = 9;
/// Port of `Tag#TAG_COMPOUND`.
pub const TAG_COMPOUND: i8 = 10;
/// Port of `Tag#TAG_INT_ARRAY`.
pub const TAG_INT_ARRAY: i8 = 11;
/// Port of `Tag#TAG_LONG_ARRAY`.
pub const TAG_LONG_ARRAY: i8 = 12;

/// Port of `Tag#MAX_DEPTH`.
pub const MAX_DEPTH: i32 = 512;

/// Port of the `sealed interface net.minecraft.nbt.Tag`.
///
/// See the module docs for why this is an enum. `ListTag` and `CompoundTag` variants are
/// not here yet.
#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    /// `EndTag.INSTANCE`. A unit variant: `EndTag` has no state and a private constructor.
    End,
    /// `ByteTag`.
    Byte(ByteTag),
    /// `ShortTag`.
    Short(ShortTag),
    /// `IntTag`.
    Int(IntTag),
    /// `LongTag`.
    Long(LongTag),
    /// `FloatTag`.
    Float(FloatTag),
    /// `DoubleTag`.
    Double(DoubleTag),
    /// `ByteArrayTag`.
    ByteArray(ByteArrayTag),
    /// `StringTag`.
    String(StringTag),
    /// `IntArrayTag`.
    IntArray(IntArrayTag),
    /// `LongArrayTag`.
    LongArray(LongArrayTag),
}

impl Tag {
    /// Port of `Tag#getId()`.
    pub fn get_id(&self) -> i8 {
        match self {
            Tag::End => TAG_END,
            Tag::Byte(_) => TAG_BYTE,
            Tag::Short(_) => TAG_SHORT,
            Tag::Int(_) => TAG_INT,
            Tag::Long(_) => TAG_LONG,
            Tag::Float(_) => TAG_FLOAT,
            Tag::Double(_) => TAG_DOUBLE,
            Tag::ByteArray(_) => TAG_BYTE_ARRAY,
            Tag::String(_) => TAG_STRING,
            Tag::IntArray(_) => TAG_INT_ARRAY,
            Tag::LongArray(_) => TAG_LONG_ARRAY,
        }
    }

    /// Port of `Tag#getType()`.
    ///
    /// Every variant's id is valid by construction, so this does not go through
    /// [`crate::net::minecraft::nbt::TagTypes::get_type`]'s bounds check.
    pub fn get_type(&self) -> TagTypeRef {
        TagTypeRef::of_id(self.get_id())
    }

    /// Port of `Tag#sizeInBytes()`.
    pub fn size_in_bytes(&self) -> i32 {
        match self {
            Tag::End => 8,
            Tag::Byte(_) => 9,
            Tag::Short(_) => 10,
            Tag::Int(_) => 12,
            Tag::Long(_) => 16,
            Tag::Float(_) => 12,
            Tag::Double(_) => 16,
            Tag::ByteArray(t) => t.size_in_bytes(),
            Tag::String(t) => t.size_in_bytes(),
            Tag::IntArray(t) => t.size_in_bytes(),
            Tag::LongArray(t) => t.size_in_bytes(),
        }
    }

    /// Port of `Tag#copy()`.
    ///
    /// `PrimitiveTag` and `EndTag` return `this` because they are immutable; the array and
    /// container kinds deep-copy. Rust's `Clone` on this enum does the same thing by
    /// construction -- a `clone` of `Tag::Byte` copies nine bytes of stack and a `clone` of
    /// `Tag::ByteArray` copies the `Vec` -- so this is `clone()` with the Java name on it.
    pub fn copy(&self) -> Tag {
        self.clone()
    }

    /// Port of `Tag#write(DataOutput)`.
    pub fn write<O: crate::javacompat::java_io::DataOutput + ?Sized>(
        &self,
        output: &mut O,
    ) -> std::io::Result<()> {
        match self {
            // `EndTag.write` is empty.
            Tag::End => Ok(()),
            Tag::Byte(t) => t.write(output),
            Tag::Short(t) => t.write(output),
            Tag::Int(t) => t.write(output),
            Tag::Long(t) => t.write(output),
            Tag::Float(t) => t.write(output),
            Tag::Double(t) => t.write(output),
            Tag::ByteArray(t) => t.write(output),
            Tag::String(t) => t.write(output),
            Tag::IntArray(t) => t.write(output),
            Tag::LongArray(t) => t.write(output),
        }
    }

    /// Port of `Tag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        match self {
            Tag::End => visitor.visit_end(),
            Tag::Byte(t) => visitor.visit_byte(t),
            Tag::Short(t) => visitor.visit_short(t),
            Tag::Int(t) => visitor.visit_int(t),
            Tag::Long(t) => visitor.visit_long(t),
            Tag::Float(t) => visitor.visit_float(t),
            Tag::Double(t) => visitor.visit_double(t),
            Tag::ByteArray(t) => visitor.visit_byte_array(t),
            Tag::String(t) => visitor.visit_string(t),
            Tag::IntArray(t) => visitor.visit_int_array(t),
            Tag::LongArray(t) => visitor.visit_long_array(t),
        }
    }

    /// Port of `Tag#asString()`.
    pub fn as_string(&self) -> Option<String> {
        match self {
            Tag::String(t) => Some(t.value().to_string()),
            _ => None,
        }
    }

    /// The `NumericTag` view of this tag, or `None`.
    ///
    /// Not a Java method. It stands in for `instanceof NumericTag`, the same way
    /// `RandomSource::as_bit_source` stands in for `instanceof LegacyRandomSource`
    /// (`DESIGN_DECISIONS.md#instanceof`), and it is what every `asX()` below routes
    /// through.
    pub fn as_numeric(&self) -> Option<&dyn NumericTag> {
        match self {
            Tag::Byte(t) => Some(t),
            Tag::Short(t) => Some(t),
            Tag::Int(t) => Some(t),
            Tag::Long(t) => Some(t),
            Tag::Float(t) => Some(t),
            Tag::Double(t) => Some(t),
            _ => None,
        }
    }

    /// Port of `Tag#asNumber()`.
    ///
    /// Java returns a boxed `Number`. There is no such type in Rust, and the only thing
    /// callers ever do with it is call one of the `xxxValue()` methods, so this returns the
    /// `NumericTag` itself -- `asNumber().map(Number::byteValue)` becomes
    /// `as_numeric().map(|n| n.byte_value())`. `box()`'s *boxed class* is still observable
    /// (`Byte` vs `Integer`), and that lives in [`NumericTag::box_class_simple_name`].
    pub fn as_number(&self) -> Option<&dyn NumericTag> {
        self.as_numeric()
    }

    /// Port of `Tag#asByte()`.
    pub fn as_byte(&self) -> Option<i8> {
        self.as_number().map(|n| n.byte_value())
    }

    /// Port of `Tag#asShort()`.
    pub fn as_short(&self) -> Option<i16> {
        self.as_number().map(|n| n.short_value())
    }

    /// Port of `Tag#asInt()`.
    pub fn as_int(&self) -> Option<i32> {
        self.as_number().map(|n| n.int_value())
    }

    /// Port of `Tag#asLong()`.
    pub fn as_long(&self) -> Option<i64> {
        self.as_number().map(|n| n.long_value())
    }

    /// Port of `Tag#asFloat()`.
    pub fn as_float(&self) -> Option<f32> {
        self.as_number().map(|n| n.float_value())
    }

    /// Port of `Tag#asDouble()`.
    pub fn as_double(&self) -> Option<f64> {
        self.as_number().map(|n| n.double_value())
    }

    /// Port of `Tag#asBoolean()`.
    ///
    /// `NumericTag` overrides this to go straight to `byteValue()`; for every other kind
    /// `asByte()` is `None`, so the two spellings agree and this one covers both.
    pub fn as_boolean(&self) -> Option<bool> {
        self.as_byte().map(|b| b != 0)
    }

    /// Port of `Tag#asByteArray()`.
    pub fn as_byte_array(&self) -> Option<&[i8]> {
        match self {
            Tag::ByteArray(t) => Some(t.get_as_byte_array()),
            _ => None,
        }
    }

    /// Port of `Tag#asIntArray()`.
    pub fn as_int_array(&self) -> Option<&[i32]> {
        match self {
            Tag::IntArray(t) => Some(t.get_as_int_array()),
            _ => None,
        }
    }

    /// Port of `Tag#asLongArray()`.
    pub fn as_long_array(&self) -> Option<&[i64]> {
        match self {
            Tag::LongArray(t) => Some(t.get_as_long_array()),
            _ => None,
        }
    }

    /// Port of `Tag#accept(StreamTagVisitor)`.
    pub fn accept_stream(
        &self,
        visitor: &mut dyn crate::net::minecraft::nbt::StreamTagVisitor::StreamTagVisitor,
    ) -> crate::net::minecraft::nbt::StreamTagVisitor::ValueResult {
        match self {
            Tag::End => visitor.visit_end(),
            Tag::Byte(t) => t.accept_stream(visitor),
            Tag::Short(t) => t.accept_stream(visitor),
            Tag::Int(t) => t.accept_stream(visitor),
            Tag::Long(t) => t.accept_stream(visitor),
            Tag::Float(t) => t.accept_stream(visitor),
            Tag::Double(t) => t.accept_stream(visitor),
            Tag::ByteArray(t) => t.accept_stream(visitor),
            Tag::String(t) => t.accept_stream(visitor),
            Tag::IntArray(t) => t.accept_stream(visitor),
            Tag::LongArray(t) => t.accept_stream(visitor),
        }
    }

    /// Port of `Tag#acceptAsRoot(StreamTagVisitor)`.
    ///
    /// `visitRootEntry` first, and only `CONTINUE` proceeds to `accept`. `BREAK` and `HALT`
    /// both stop -- unlike `TagType.parseRoot`, which skips the value on `BREAK`, because
    /// here the value is already in memory and there is nothing to skip over.
    pub fn accept_as_root(
        &self,
        output: &mut dyn crate::net::minecraft::nbt::StreamTagVisitor::StreamTagVisitor,
    ) {
        let entry_result = output.visit_root_entry(&self.get_type());
        if entry_result == crate::net::minecraft::nbt::StreamTagVisitor::ValueResult::Continue {
            self.accept_stream(output);
        }
    }
}

/// Port of `Tag#toString()`.
///
/// Every implementor's `toString` is the same three lines -- make a `StringTagVisitor`,
/// visit itself, return `build()` -- so it is written once here.
impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = StringTagVisitor::new();
        self.accept(&mut visitor);
        f.write_str(&visitor.build())
    }
}
