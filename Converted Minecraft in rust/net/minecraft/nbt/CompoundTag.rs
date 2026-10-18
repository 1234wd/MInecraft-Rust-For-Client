//! Port of: net/minecraft/nbt/CompoundTag.java
//! Java class(es): net.minecraft.nbt.CompoundTag, CompoundTag::<1 anonymous class(es)>
//! Status: PARTIAL
//!
//! Session 23. `public final class CompoundTag implements Tag`: string-keyed tags in a
//! `java.util.HashMap`, with the anonymous `TYPE` (`load`/`parse`/`skip`). PARTIAL for two
//! reasons, both a missing DEPENDENCY rather than a missing method body:
//!
//! * **Blocked on DataFixerUpper** (`com.mojang.serialization`, `DESIGN_DECISIONS.md#dfu-proposal`):
//!   the `CODEC` field and the eight codec methods -- `store(String, Codec, T)`,
//!   `storeNullable(String, Codec, T)`, `store(String, Codec, DynamicOps, T)`,
//!   `storeNullable(String, Codec, DynamicOps, T)`, `store(MapCodec, T)`,
//!   `store(MapCodec, DynamicOps, T)`, `read(String, Codec)`, `read(String, Codec, DynamicOps)`,
//!   `read(MapCodec)`, `read(MapCodec, DynamicOps)`. They also need `NbtOps`. Not ported; no stub.
//! * **`readNamedTagData`'s crash report**: an `IOException` from a child's `load` becomes a
//!   `ReportedNbtException` built from a `CrashReport` with "Tag name"/"Tag type" details.
//!   `CrashReport` is SKELETON, so the port raises the exception with Java's observable text --
//!   class `net.minecraft.nbt.ReportedNbtException`, message `Loading NBT data` (the report's
//!   title, which `ReportedException.getMessage` returns) and the IOException as its cause -- but
//!   builds no report. Probed (`TLOAD compound_invalid_child`).
//!
//! Every other member is ported and verified.
//!
//! # The map decides the bytes
//!
//! `tags` is a [`JavaHashMap`], not a Rust map, because `write` emits entries in
//! `HashMap.keySet()` order and `copy()` re-inserts in it: the byte layout of a saved compound
//! depends on its insertion/removal history and table size, and that is reproduced exactly.
//! `toString` does NOT depend on it -- `StringTagVisitor` sorts the keys.
//!
//! * `copy()` is a deep copy into a NEW `HashMap` via `forEach(put)`: same entries, a table
//!   sized for them, so a compound that grew and shrank iterates differently after `copy()`.
//! * `shallowCopy()` is `new HashMap<>(this.tags)` -- the presizing copy constructor. Java shares
//!   the child tags with the original; Rust cannot, so the children are cloned (each a separate
//!   value, same content). Iteration order is the copy constructor's, which is what is observable.
//! * `equals` is the map's (`AbstractMap.equals`: same entries, any order) and `hashCode` the map's
//!   (sum of `key.hashCode() ^ value.hashCode()`), `None` where a value's hash is an identity hash.
//!
//! # Getters
//!
//! * `getX(name)` -> `Optional`: `getOptional(name).flatMap(Tag::asX)`, so a numeric tag of any
//!   kind answers with Java's narrowing (`getByte` of `DoubleTag(1e10)` floors then masks) and a
//!   non-numeric tag answers nothing.
//! * `getXOr(name, default)`: `instanceof NumericTag ? xValue() : default` -- the same narrowing.
//! * `getBooleanOr(name, d)` is `getByteOr(name, d ? 1 : 0) != 0`: the answer is the tag's
//!   `byteValue()`, the low eight bits for an integral tag, so a `LongTag` of `0x100` reads as
//!   `false` and one of `0x100000080` as `true`. (`CGET l`, `CTOPS`.)
//! * `getCompound`/`getList` return the live child; `getCompoundOrEmpty`/`getListOrEmpty` return
//!   it or a NEW empty tag that is not stored. Rust returns `Cow`, and `_mut` twins hand out the
//!   live child for editing (see `Tag`'s module docs on aliasing).
//!
//! # Null
//!
//! Java's map accepts a null key or value and `write` would then throw. Neither can be expressed
//! here: keys are `String`, values `Tag`. `put`/`remove` return the previous value as `Option`,
//! Java's `null` being `None`.
//!
//! # `TYPE`
//!
//! * `load`: `pushDepth`; 48 bytes; then entries until a `0` type byte: the key (`readUTF`, then
//!   28 + 2 * UTF-16 length bytes), the value through its type's `load`, and 36 bytes more only
//!   when the key is NEW -- a duplicate key replaces the value and is not charged again.
//!   `popDepth` runs even when an entry fails with an `IOException`.
//! * `parse`: as `load`, but driven by the visitor: `visitEntry(type)` (`SKIP` skips the entry,
//!   `BREAK` skips it and stops reading entries, `HALT` stops), then `visitEntry(type, key)` (the
//!   same three), then 36 bytes and the value's `parse` -- whose `HALT` stops but whose **`BREAK`
//!   is ignored**: the loop carries on with the next entry. That is what the bytecode does
//!   (checked with `javap`, not just the decompiled source), and it differs from `ListTag`, where
//!   an element's `BREAK` ends the list. After a `BREAK`, the remaining entries are skipped.
//! * `skip`: `pushDepth`, every entry's name and value skipped, `popDepth`.
//!
//! Verified by the `CGET`, `IDENT`, `OTHER` (the empty compound), `GEN`, `CASE`, `CASEW` and
//! `CTOPS`/`TLOAD`/`TEQ`/`TACC` groups of `_porting/test-data/nbt_probe.txt`
//! (`_porting/tests/session23.rs`).

use std::borrow::Cow;

use crate::javacompat::java_hash_map::JavaHashMap;
use crate::javacompat::java_io::{self, DataInput, DataOutput};
use crate::net::minecraft::nbt::ByteArrayTag::ByteArrayTag;
use crate::net::minecraft::nbt::ByteTag::ByteTag;
use crate::net::minecraft::nbt::DoubleTag::DoubleTag;
use crate::net::minecraft::nbt::FloatTag::FloatTag;
use crate::net::minecraft::nbt::IntArrayTag::IntArrayTag;
use crate::net::minecraft::nbt::IntTag::IntTag;
use crate::net::minecraft::nbt::ListTag::ListTag;
use crate::net::minecraft::nbt::LongArrayTag::LongArrayTag;
use crate::net::minecraft::nbt::LongTag::LongTag;
use crate::net::minecraft::nbt::NbtAccounter::NbtAccounter;
use crate::net::minecraft::nbt::ShortTag::ShortTag;
use crate::net::minecraft::nbt::StreamTagVisitor::{EntryResult, StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::StringTag::{self, StringTag as StringTagValue};
use crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor;
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagType::TagTypeRef;
use crate::net::minecraft::nbt::TagTypes;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;

/// Port of `CompoundTag#TYPE`.
pub const TYPE: TagTypeRef = TagTypeRef::Known(10);

/// Port of the private `CompoundTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 48;

/// Port of the private `CompoundTag#MAP_ENTRY_SIZE_IN_BYTES`. Declared and never read in Java
/// too: `sizeInBytes` and `load` count 36 bytes per entry, not 32.
#[allow(dead_code)]
const MAP_ENTRY_SIZE_IN_BYTES: i32 = 32;

/// Port of `net.minecraft.nbt.CompoundTag`.
///
/// `PartialEq` is `equals`: `Objects.equals(this.tags, other.tags)`, the map's content equality.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompoundTag {
    tags: JavaHashMap<String, Tag>,
}

impl CompoundTag {
    /// Port of `new CompoundTag()`: an empty `HashMap`.
    pub fn new() -> Self {
        CompoundTag {
            tags: JavaHashMap::new(),
        }
    }

    /// Port of the package-private `CompoundTag(Map<String, Tag>)`.
    pub(crate) fn from_map(tags: JavaHashMap<String, Tag>) -> Self {
        CompoundTag { tags }
    }

    // ------------------------------------------------------------ Tag

    /// Port of `CompoundTag#write(DataOutput)`: every entry in `keySet()` order through
    /// `writeNamedTag`, then a `0` byte.
    pub fn write<O: DataOutput + ?Sized>(&self, output: &mut O) -> std::io::Result<()> {
        for key in self.tags.keys() {
            let tag = self.tags.get(key.as_str()).expect("key from keySet");
            Self::write_named_tag(key, tag, output)?;
        }
        output.write_byte(0)
    }

    /// Port of the private `writeNamedTag(String, Tag, DataOutput)`: the id, and -- unless it is
    /// `END` -- the name and the payload.
    fn write_named_tag<O: DataOutput + ?Sized>(
        name: &str,
        tag: &Tag,
        output: &mut O,
    ) -> std::io::Result<()> {
        output.write_byte(tag.get_id() as i32)?;
        if tag.get_id() != 0 {
            output.write_utf(name)?;
            tag.write(output)?;
        }
        Ok(())
    }

    /// Port of `CompoundTag#sizeInBytes()`: 48, plus per entry `28 + 2 * key.length()` (UTF-16
    /// units), 36, and the value's size.
    pub fn size_in_bytes(&self) -> i32 {
        let mut size = SELF_SIZE_IN_BYTES;
        for (key, value) in self.tags.iter() {
            size = size
                .wrapping_add(28)
                .wrapping_add(2i32.wrapping_mul(StringTag::java_length(key)));
            size = size.wrapping_add(36);
            size = size.wrapping_add(value.size_in_bytes());
        }
        size
    }

    /// Port of `CompoundTag#keySet()`'s iteration, in `HashMap` order.
    pub fn key_set(&self) -> impl Iterator<Item = &String> {
        self.tags.keys()
    }

    /// Port of `CompoundTag#entrySet()`'s iteration, in `HashMap` order.
    pub fn entry_set(&self) -> impl Iterator<Item = (&String, &Tag)> {
        self.tags.iter()
    }

    /// Port of `CompoundTag#values()`'s iteration, in `HashMap` order.
    pub fn values(&self) -> impl Iterator<Item = &Tag> {
        self.tags.values()
    }

    /// Port of `CompoundTag#forEach(BiConsumer<String, Tag>)`, in `HashMap` order.
    pub fn for_each(&self, mut consumer: impl FnMut(&str, &Tag)) {
        for (key, tag) in self.tags.iter() {
            consumer(key, tag);
        }
    }

    /// Port of `CompoundTag#getId()`.
    pub const fn get_id(&self) -> i8 {
        10
    }

    /// Port of `CompoundTag#getType()`.
    pub const fn get_type(&self) -> TagTypeRef {
        TYPE
    }

    /// Port of `CompoundTag#size()`.
    pub fn size(&self) -> i32 {
        self.tags.size()
    }

    /// Port of `CompoundTag#isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }

    // ------------------------------------------------------------ put

    /// Port of `CompoundTag#put(String, Tag)`: returns the previous value, `None` for Java's null.
    pub fn put(&mut self, name: &str, tag: Tag) -> Option<Tag> {
        self.tags.put(name.to_string(), tag)
    }

    /// Port of `CompoundTag#putByte(String, byte)`.
    pub fn put_byte(&mut self, name: &str, value: i8) {
        self.put(name, ByteTag::value_of(value).into_tag());
    }

    /// Port of `CompoundTag#putShort(String, short)`.
    pub fn put_short(&mut self, name: &str, value: i16) {
        self.put(name, ShortTag::value_of(value).into_tag());
    }

    /// Port of `CompoundTag#putInt(String, int)`.
    pub fn put_int(&mut self, name: &str, value: i32) {
        self.put(name, IntTag::value_of(value).into_tag());
    }

    /// Port of `CompoundTag#putLong(String, long)`.
    pub fn put_long(&mut self, name: &str, value: i64) {
        self.put(name, LongTag::value_of(value).into_tag());
    }

    /// Port of `CompoundTag#putFloat(String, float)`: through `valueOf`, so `-0.0F` is stored as
    /// `FloatTag.ZERO` (`+0.0`).
    pub fn put_float(&mut self, name: &str, value: f32) {
        self.put(name, FloatTag::value_of(value).into_tag());
    }

    /// Port of `CompoundTag#putDouble(String, double)`: through `valueOf`, so `-0.0` becomes `+0.0`.
    pub fn put_double(&mut self, name: &str, value: f64) {
        self.put(name, DoubleTag::value_of(value).into_tag());
    }

    /// Port of `CompoundTag#putString(String, String)`.
    pub fn put_string(&mut self, name: &str, value: &str) {
        self.put(name, StringTagValue::value_of(value).into_tag());
    }

    /// Port of `CompoundTag#putByteArray(String, byte[])`. Java stores the caller's array itself;
    /// the port takes it by value.
    pub fn put_byte_array(&mut self, name: &str, value: Vec<i8>) {
        self.put(name, ByteArrayTag::new(value).into_tag());
    }

    /// Port of `CompoundTag#putIntArray(String, int[])`.
    pub fn put_int_array(&mut self, name: &str, value: Vec<i32>) {
        self.put(name, IntArrayTag::new(value).into_tag());
    }

    /// Port of `CompoundTag#putLongArray(String, long[])`.
    pub fn put_long_array(&mut self, name: &str, value: Vec<i64>) {
        self.put(name, LongArrayTag::new(value).into_tag());
    }

    /// Port of `CompoundTag#putBoolean(String, boolean)`: `ByteTag.valueOf(boolean)`, 1 or 0.
    pub fn put_boolean(&mut self, name: &str, value: bool) {
        self.put(name, ByteTag::value_of_bool(value).into_tag());
    }

    // ------------------------------------------------------------ get

    /// Port of `CompoundTag#get(String)`: the live value, `None` when absent.
    pub fn get(&self, name: &str) -> Option<&Tag> {
        self.tags.get(name)
    }

    /// `get(String)`, for editing the value in place.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Tag> {
        self.tags.get_mut(name)
    }

    /// Port of `CompoundTag#contains(String)`: `containsKey`.
    pub fn contains(&self, name: &str) -> bool {
        self.tags.contains_key(name)
    }

    /// The `NumericTag` under `name`, for the `getXOr` family (`instanceof NumericTag`).
    fn numeric(
        &self,
        name: &str,
    ) -> Option<&dyn crate::net::minecraft::nbt::NumericTag::NumericTag> {
        self.get(name).and_then(Tag::as_numeric)
    }

    /// Port of `CompoundTag#getByte(String)`: `getOptional(name).flatMap(Tag::asByte)`.
    pub fn get_byte(&self, name: &str) -> Option<i8> {
        self.get(name).and_then(Tag::as_byte)
    }

    /// Port of `CompoundTag#getByteOr(String, byte)`.
    pub fn get_byte_or(&self, name: &str, default_value: i8) -> i8 {
        self.numeric(name).map_or(default_value, |n| n.byte_value())
    }

    /// Port of `CompoundTag#getShort(String)`.
    pub fn get_short(&self, name: &str) -> Option<i16> {
        self.get(name).and_then(Tag::as_short)
    }

    /// Port of `CompoundTag#getShortOr(String, short)`.
    pub fn get_short_or(&self, name: &str, default_value: i16) -> i16 {
        self.numeric(name)
            .map_or(default_value, |n| n.short_value())
    }

    /// Port of `CompoundTag#getInt(String)`.
    pub fn get_int(&self, name: &str) -> Option<i32> {
        self.get(name).and_then(Tag::as_int)
    }

    /// Port of `CompoundTag#getIntOr(String, int)`.
    pub fn get_int_or(&self, name: &str, default_value: i32) -> i32 {
        self.numeric(name).map_or(default_value, |n| n.int_value())
    }

    /// Port of `CompoundTag#getLong(String)`.
    pub fn get_long(&self, name: &str) -> Option<i64> {
        self.get(name).and_then(Tag::as_long)
    }

    /// Port of `CompoundTag#getLongOr(String, long)`.
    pub fn get_long_or(&self, name: &str, default_value: i64) -> i64 {
        self.numeric(name).map_or(default_value, |n| n.long_value())
    }

    /// Port of `CompoundTag#getFloat(String)`.
    pub fn get_float(&self, name: &str) -> Option<f32> {
        self.get(name).and_then(Tag::as_float)
    }

    /// Port of `CompoundTag#getFloatOr(String, float)`.
    pub fn get_float_or(&self, name: &str, default_value: f32) -> f32 {
        self.numeric(name)
            .map_or(default_value, |n| n.float_value())
    }

    /// Port of `CompoundTag#getDouble(String)`.
    pub fn get_double(&self, name: &str) -> Option<f64> {
        self.get(name).and_then(Tag::as_double)
    }

    /// Port of `CompoundTag#getDoubleOr(String, double)`.
    pub fn get_double_or(&self, name: &str, default_value: f64) -> f64 {
        self.numeric(name)
            .map_or(default_value, |n| n.double_value())
    }

    /// Port of `CompoundTag#getString(String)`: `asString()`, so only a `StringTag` answers.
    pub fn get_string(&self, name: &str) -> Option<&str> {
        match self.get(name) {
            Some(Tag::String(t)) => Some(t.value()),
            _ => None,
        }
    }

    /// Port of `CompoundTag#getStringOr(String, String)`.
    pub fn get_string_or<'a>(&'a self, name: &str, default_value: &'a str) -> &'a str {
        self.get_string(name).unwrap_or(default_value)
    }

    /// Port of `CompoundTag#getByteArray(String)`: the live array of a `ByteArrayTag`.
    pub fn get_byte_array(&self, name: &str) -> Option<&[i8]> {
        match self.get(name) {
            Some(Tag::ByteArray(t)) => Some(t.get_as_byte_array()),
            _ => None,
        }
    }

    /// Port of `CompoundTag#getIntArray(String)`.
    pub fn get_int_array(&self, name: &str) -> Option<&[i32]> {
        match self.get(name) {
            Some(Tag::IntArray(t)) => Some(t.get_as_int_array()),
            _ => None,
        }
    }

    /// Port of `CompoundTag#getLongArray(String)`.
    pub fn get_long_array(&self, name: &str) -> Option<&[i64]> {
        match self.get(name) {
            Some(Tag::LongArray(t)) => Some(t.get_as_long_array()),
            _ => None,
        }
    }

    /// Port of `CompoundTag#getCompound(String)`: the live child.
    pub fn get_compound(&self, name: &str) -> Option<&CompoundTag> {
        self.get(name).and_then(Tag::as_compound)
    }

    /// `getCompound(String)`, for editing the child in place.
    pub fn get_compound_mut(&mut self, name: &str) -> Option<&mut CompoundTag> {
        match self.tags.get_mut(name) {
            Some(Tag::Compound(c)) => Some(c),
            _ => None,
        }
    }

    /// Port of `CompoundTag#getCompoundOrEmpty(String)`: the live child, or a NEW empty compound
    /// that is not stored.
    pub fn get_compound_or_empty(&self, name: &str) -> Cow<'_, CompoundTag> {
        match self.get_compound(name) {
            Some(c) => Cow::Borrowed(c),
            None => Cow::Owned(CompoundTag::new()),
        }
    }

    /// Port of `CompoundTag#getList(String)`: the live child.
    pub fn get_list(&self, name: &str) -> Option<&ListTag> {
        self.get(name).and_then(Tag::as_list)
    }

    /// `getList(String)`, for editing the child in place.
    pub fn get_list_mut(&mut self, name: &str) -> Option<&mut ListTag> {
        match self.tags.get_mut(name) {
            Some(Tag::List(l)) => Some(l),
            _ => None,
        }
    }

    /// Port of `CompoundTag#getListOrEmpty(String)`: the live child, or a NEW empty list.
    pub fn get_list_or_empty(&self, name: &str) -> Cow<'_, ListTag> {
        match self.get_list(name) {
            Some(l) => Cow::Borrowed(l),
            None => Cow::Owned(ListTag::new()),
        }
    }

    /// Port of `CompoundTag#getBoolean(String)`: `asBoolean()`, `byteValue() != 0`.
    pub fn get_boolean(&self, name: &str) -> Option<bool> {
        self.get(name).and_then(Tag::as_boolean)
    }

    /// Port of `CompoundTag#getBooleanOr(String, boolean)`: `getByteOr(name, d ? 1 : 0) != 0`.
    pub fn get_boolean_or(&self, name: &str, default_value: bool) -> bool {
        self.get_byte_or(name, if default_value { 1 } else { 0 }) != 0
    }

    // ------------------------------------------------------------ remove, copies, merge

    /// Port of `CompoundTag#remove(String)`: returns the removed value.
    pub fn remove(&mut self, name: &str) -> Option<Tag> {
        self.tags.remove(name)
    }

    /// Port of the package-private `CompoundTag#shallowCopy()`: `new HashMap<>(this.tags)`.
    ///
    /// The copy constructor's presizing decides the iteration order. Java shares the children;
    /// these are clones (see the module docs).
    pub fn shallow_copy(&self) -> CompoundTag {
        CompoundTag::from_map(JavaHashMap::copy_of(&self.tags))
    }

    /// Port of `CompoundTag#copy()`: a NEW `HashMap`, filled by `forEach((k, v) -> put(k, v.copy()))`.
    pub fn copy(&self) -> CompoundTag {
        let mut new_tags = JavaHashMap::new();
        for (key, tag) in self.tags.iter() {
            new_tags.put(key.clone(), tag.copy());
        }
        CompoundTag::from_map(new_tags)
    }

    /// Port of `CompoundTag#merge(CompoundTag)`: for each entry of `other` (in its order), a
    /// compound merges into a compound already here, and anything else is `put` as a `copy()`.
    pub fn merge(&mut self, other: &CompoundTag) -> &mut CompoundTag {
        for tag_name in other.tags.keys() {
            let other_tag = other.tags.get(tag_name.as_str()).expect("key from keySet");
            if let Tag::Compound(other_compound) = other_tag {
                if let Some(self_compound) = self.get_compound_mut(tag_name) {
                    self_compound.merge(other_compound);
                    continue;
                }
            }
            self.put(tag_name, other_tag.copy());
        }
        self
    }

    /// Port of `CompoundTag#hashCode()`: the map's hash, or `None` if a value's hash is an
    /// identity hash.
    pub fn java_hash_code(&self) -> Option<i32> {
        let mut h: i32 = 0;
        for (key, value) in self.tags.iter() {
            h = h.wrapping_add(
                crate::javacompat::java_hash_map::JavaHashKey::java_hash_code(key.as_str())
                    ^ value.java_hash_code()?,
            );
        }
        Some(h)
    }

    // ------------------------------------------------------------ visitors

    /// Port of `CompoundTag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        visitor.visit_compound(self);
    }

    /// Port of `CompoundTag#accept(StreamTagVisitor)`: per entry in map order, `visitEntry(type)`
    /// then `visitEntry(type, key)` -- `SKIP` moves on, `BREAK` ends the compound, `HALT` stops --
    /// then the value's own `accept`, whose `BREAK` also ends the compound.
    pub fn accept_stream(&self, visitor: &mut dyn StreamTagVisitor) -> ValueResult {
        for (key, value) in self.tags.iter() {
            let tag_type = value.get_type();
            match visitor.visit_entry(&tag_type) {
                EntryResult::Halt => return ValueResult::Halt,
                EntryResult::Break => return visitor.visit_container_end(),
                EntryResult::Skip => continue,
                EntryResult::Enter => {}
            }
            match visitor.visit_entry_named(&tag_type, key) {
                EntryResult::Halt => return ValueResult::Halt,
                EntryResult::Break => return visitor.visit_container_end(),
                EntryResult::Skip => continue,
                EntryResult::Enter => {}
            }
            match value.accept_stream(visitor) {
                ValueResult::Halt => return ValueResult::Halt,
                ValueResult::Break => return visitor.visit_container_end(),
                ValueResult::Continue => {}
            }
        }
        visitor.visit_container_end()
    }

    // ------------------------------------------------------------ TYPE

    /// Port of `CompoundTag.TYPE.load(DataInput, NbtAccounter)`.
    pub fn load_type<I: DataInput + ?Sized>(
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<Tag> {
        accounter.push_depth();
        let result = Self::load_compound(input, accounter);
        accounter.pop_depth();
        result.map(Tag::Compound)
    }

    /// Port of `TYPE`'s private `loadCompound`.
    fn load_compound<I: DataInput + ?Sized>(
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<CompoundTag> {
        accounter.account_bytes(48);
        // `Maps.newHashMap()` is `new HashMap<>()`.
        let mut values: JavaHashMap<String, Tag> = JavaHashMap::new();
        loop {
            let tag_type = input.read_byte()?;
            if tag_type == 0 {
                break;
            }
            let key = Self::read_string(input, accounter)?;
            let tag = Self::read_named_tag_data(
                TagTypes::get_type(tag_type as i32),
                &key,
                input,
                accounter,
            );
            if values.put(key, tag).is_none() {
                accounter.account_bytes(36);
            }
        }
        Ok(CompoundTag::from_map(values))
    }

    /// Port of `TYPE`'s private `readString`: `readUTF`, then 28 + 2 * length bytes.
    fn read_string<I: DataInput + ?Sized>(
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<String> {
        let key = input.read_utf()?;
        accounter.account_bytes(28);
        accounter.account_bytes_per_entry(2, StringTag::java_length(&key) as i64);
        Ok(key)
    }

    /// Port of the private `readNamedTagData(TagType, String, DataInput, NbtAccounter)`.
    ///
    /// # Panics
    ///
    /// `net.minecraft.nbt.ReportedNbtException: Loading NBT data`, caused by the child's
    /// `IOException`, where Java throws the same unchecked exception. The `CrashReport` behind it
    /// (with its "Tag name" and "Tag type" details) is not built: `CrashReport` is not ported.
    fn read_named_tag_data<I: DataInput + ?Sized>(
        tag_type: TagTypeRef,
        _name: &str,
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> Tag {
        match tag_type.load(input, accounter) {
            Ok(tag) => tag,
            Err(e) => panic!(
                "net.minecraft.nbt.ReportedNbtException: Loading NBT data\nCaused by: {}",
                java_io::java_to_string(&e)
            ),
        }
    }

    /// Port of `CompoundTag.TYPE.parse(DataInput, StreamTagVisitor, NbtAccounter)`.
    pub fn parse_type<I: DataInput + ?Sized>(
        input: &mut I,
        output: &mut dyn StreamTagVisitor,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<ValueResult> {
        accounter.push_depth();
        let result = Self::parse_compound(input, output, accounter);
        accounter.pop_depth();
        result
    }

    /// Port of `TYPE`'s private `parseCompound` -- see the module docs for its `BREAK` quirk.
    fn parse_compound<I: DataInput + ?Sized>(
        input: &mut I,
        output: &mut dyn StreamTagVisitor,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<ValueResult> {
        accounter.account_bytes(48);
        let mut tag_type_id;
        loop {
            tag_type_id = input.read_byte()?;
            if tag_type_id == 0 {
                break;
            }
            let tag_type = TagTypes::get_type(tag_type_id as i32);
            match output.visit_entry(&tag_type) {
                EntryResult::Halt => return Ok(ValueResult::Halt),
                EntryResult::Break => {
                    StringTagValue::skip_string(input)?;
                    tag_type.skip(input, accounter)?;
                    break;
                }
                EntryResult::Skip => {
                    StringTagValue::skip_string(input)?;
                    tag_type.skip(input, accounter)?;
                    continue;
                }
                EntryResult::Enter => {}
            }
            let key = Self::read_string(input, accounter)?;
            match output.visit_entry_named(&tag_type, &key) {
                EntryResult::Halt => return Ok(ValueResult::Halt),
                EntryResult::Break => {
                    tag_type.skip(input, accounter)?;
                    break;
                }
                EntryResult::Skip => {
                    tag_type.skip(input, accounter)?;
                    continue;
                }
                EntryResult::Enter => {}
            }
            accounter.account_bytes(36);
            match tag_type.parse(input, output, accounter)? {
                ValueResult::Halt => return Ok(ValueResult::Halt),
                // Not a stop: the bytecode jumps straight back to the next entry.
                ValueResult::Break | ValueResult::Continue => {}
            }
        }
        if tag_type_id != 0 {
            loop {
                tag_type_id = input.read_byte()?;
                if tag_type_id == 0 {
                    break;
                }
                StringTagValue::skip_string(input)?;
                TagTypes::get_type(tag_type_id as i32).skip(input, accounter)?;
            }
        }
        Ok(output.visit_container_end())
    }

    /// Port of `CompoundTag.TYPE.skip(DataInput, NbtAccounter)`.
    pub fn skip_type<I: DataInput + ?Sized>(
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<()> {
        accounter.push_depth();
        let result = (|| loop {
            let tag_type_id = input.read_byte()?;
            if tag_type_id == 0 {
                return Ok(());
            }
            StringTagValue::skip_string(input)?;
            TagTypes::get_type(tag_type_id as i32).skip(input, accounter)?;
        })();
        accounter.pop_depth();
        result
    }

    /// Wraps this compound as a [`Tag`].
    pub fn into_tag(self) -> Tag {
        Tag::Compound(self)
    }
}

/// Port of `CompoundTag#toString()`: `StringTagVisitor.visitCompound` (keys sorted).
impl std::fmt::Display for CompoundTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = StringTagVisitor::new();
        visitor.visit_compound(self);
        f.write_str(&visitor.build())
    }
}
