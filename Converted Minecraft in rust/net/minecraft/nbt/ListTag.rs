//! Port of: net/minecraft/nbt/ListTag.java
//! Java class(es): net.minecraft.nbt.ListTag, ListTag::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 23. `public final class ListTag extends AbstractList<Tag> implements CollectionTag`:
//! a list of tags of ANY kinds, with the anonymous `TYPE` (`load`/`parse`/`skip`). Every method
//! the class declares is ported; see "What is not" for the `AbstractList` methods it inherits.
//!
//! # A list is heterogeneous in memory, and homogeneous on disk
//!
//! There is no element-type field and no restriction on `add` -- `setTag`/`addTag` always
//! return `true`. The element type is worked out at write time by
//! [`identify_raw_element_type`](ListTag::identify_raw_element_type): the id shared by every
//! element, `10` (compound) as soon as two differ, and `0` for an empty list. A mixed list is
//! then written as a list of compounds by WRAPPING each element that is not already a plain
//! compound in `{"": element}` (`wrapIfNeeded`), and `load` UNWRAPS every one-entry compound
//! keyed `""` again (`addAndUnwrap`). Three consequences, all reproduced and probed:
//!
//! * a compound that happens to be `{"": x}` is unwrapped on the way back in -- it becomes `x`;
//! * a compound inside a mixed list is written bare unless it is itself a wrapper, in which
//!   case it is wrapped again so that it survives the round trip;
//! * `identifyRawElementType` only remembers the FIRST NON-ZERO id, so `[END, 5L]` reports
//!   `LONG` and is written as a long list whose first element writes nothing -- a stream that
//!   cannot be read back. (`IDENT end_long`.)
//!
//! # Index errors are `java.util.ArrayList`'s
//!
//! `get`/`set`/`remove`/`setTag` check with `Objects.checkIndex` ("Index 3 out of bounds for
//! length 3"); `add(int, Tag)`/`addTag` with `rangeCheckForAdd` ("Index: 4, Size: 3"), which
//! allows `index == size`. The port panics with those texts (`#runtime-exceptions`). The typed
//! getters (`getInt(i)`, `getStringOr(i, d)`, ...) never throw: out of range is "absent".
//!
//! # `TYPE`
//!
//! * `load`: `pushDepth`, 36 bytes, the element-type byte, a count that must not be negative
//!   (`NbtFormatException: ListTag length cannot be negative: <n>`), and -- only when the count
//!   is positive -- an element type that must not be `END` (`Missing type on ListTag`); then
//!   `4 * count` bytes and each element through `addAndUnwrap`. `popDepth` runs even when an
//!   element fails with an `IOException` (Java's `finally`).
//! * `parse`: the same header, then `visitList` (`BREAK` skips the whole body), then per element
//!   `visitElement` -- `SKIP` skips one, `BREAK` skips the rest, `HALT` stops -- and the
//!   element's own `parse`, whose `BREAK` also skips the rest.
//! * `skip`: reads the type and count WITHOUT the negative check and skips `count` elements.
//!
//! # What is not
//!
//! `ListTag` inherits the rest of `java.util.List` from `AbstractList` (`indexOf`, `contains`,
//! `subList`, `listIterator`, `removeIf`, `sort`, `addAll`, ...). They are not ported as named
//! methods: [`as_slice`](ListTag::as_slice) and [`iter`](ListTag::iter) give read access with
//! Rust's own slice methods, and nothing in this port's call graph mutates a list any other way.
//! `add(Tag)` (the inherited `AbstractList.add`, which appends) IS ported, as [`ListTag::add`]:
//! it is the common way to fill a list.
//!
//! Verified by the `LGET`, `IDENT`, `IDX` (`list_*`), `OTHER` (the empty list), `GEN`, `CASE`,
//! `CASEW` and `LTOPS`/`TLOAD`/`TEQ`/`TACC` groups of `_porting/test-data/nbt_probe.txt`
//! (`_porting/tests/session23.rs`).

use std::borrow::Cow;

use crate::javacompat::java_io::{DataInput, DataOutput};
use crate::net::minecraft::nbt::CollectionTag::CollectionTag;
use crate::net::minecraft::nbt::CompoundTag::CompoundTag;
use crate::net::minecraft::nbt::NbtAccounter::NbtAccounter;
use crate::net::minecraft::nbt::NbtFormatException;
use crate::net::minecraft::nbt::StreamTagVisitor::{EntryResult, StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor;
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagType::TagTypeRef;
use crate::net::minecraft::nbt::TagTypes;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;

/// Port of `ListTag#TYPE`.
pub const TYPE: TagTypeRef = TagTypeRef::Known(9);

/// Port of the private `ListTag#WRAPPER_MARKER`.
const WRAPPER_MARKER: &str = "";

/// Port of the private `ListTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 36;

/// Port of `net.minecraft.nbt.ListTag`.
///
/// `PartialEq` is `List.equals`: same length, element-wise `equals`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListTag {
    list: Vec<Tag>,
}

impl ListTag {
    /// Port of `new ListTag()`.
    pub fn new() -> Self {
        ListTag { list: Vec::new() }
    }

    /// Port of the package-private `ListTag(List<Tag>)`.
    pub(crate) fn from_vec(list: Vec<Tag>) -> Self {
        ListTag { list }
    }

    // ------------------------------------------------------------ wrapping

    /// Port of the private `tryUnwrap(CompoundTag)`: the value of a one-entry compound keyed
    /// `""`, else the compound itself.
    fn try_unwrap(mut tag: CompoundTag) -> Tag {
        if tag.size() == 1 {
            if let Some(value) = tag.remove(WRAPPER_MARKER) {
                return value;
            }
        }
        Tag::Compound(tag)
    }

    /// Port of the private `isWrapper(CompoundTag)`.
    fn is_wrapper(tag: &CompoundTag) -> bool {
        tag.size() == 1 && tag.contains(WRAPPER_MARKER)
    }

    /// Port of the private `wrapIfNeeded(byte, Tag)`.
    fn wrap_if_needed(element_type: i8, tag: &Tag) -> Cow<'_, Tag> {
        if element_type != 10 {
            return Cow::Borrowed(tag);
        }
        match tag {
            Tag::Compound(compound) if !Self::is_wrapper(compound) => Cow::Borrowed(tag),
            _ => Cow::Owned(Tag::Compound(Self::wrap_element(tag))),
        }
    }

    /// Port of the private `wrapElement(Tag)`: `new CompoundTag(Map.of("", tag))`.
    ///
    /// Java's wrapper holds an immutable `Map.of` and lives only for one `write` or `accept`;
    /// one entry iterates the same in any map, so a fresh `CompoundTag` stands in for it.
    fn wrap_element(tag: &Tag) -> CompoundTag {
        let mut wrapper = CompoundTag::new();
        wrapper.put(WRAPPER_MARKER, tag.clone());
        wrapper
    }

    // ------------------------------------------------------------ Tag

    /// Port of `ListTag#write(DataOutput)`.
    pub fn write<O: DataOutput + ?Sized>(&self, output: &mut O) -> std::io::Result<()> {
        let element_type = self.identify_raw_element_type();
        output.write_byte(element_type as i32)?;
        output.write_int(self.list.len() as i32)?;
        for element in &self.list {
            Self::wrap_if_needed(element_type, element).write(output)?;
        }
        Ok(())
    }

    /// Port of the `@VisibleForTesting` `ListTag#identifyRawElementType()`.
    ///
    /// The first non-zero element id, unless a later element differs from it (then `10`).
    pub fn identify_raw_element_type(&self) -> i8 {
        let mut homogenous_type: i8 = 0;
        for element in &self.list {
            let element_type = element.get_id();
            if homogenous_type == 0 {
                homogenous_type = element_type;
            } else if homogenous_type != element_type {
                return 10;
            }
        }
        homogenous_type
    }

    /// Port of `ListTag#addAndUnwrap(Tag)`: a compound goes through `tryUnwrap` first.
    pub fn add_and_unwrap(&mut self, tag: Tag) {
        match tag {
            Tag::Compound(compound) => self.add(Self::try_unwrap(compound)),
            other => self.add(other),
        };
    }

    /// Port of `ListTag#sizeInBytes()`: 36, plus 4 per element, plus each element's size.
    pub fn size_in_bytes(&self) -> i32 {
        let mut size = SELF_SIZE_IN_BYTES;
        size = size.wrapping_add(4i32.wrapping_mul(self.list.len() as i32));
        for child in &self.list {
            size = size.wrapping_add(child.size_in_bytes());
        }
        size
    }

    /// Port of `ListTag#getId()`.
    pub const fn get_id(&self) -> i8 {
        9
    }

    /// Port of `ListTag#getType()`.
    pub const fn get_type(&self) -> TagTypeRef {
        TYPE
    }

    /// Port of `ListTag#copy()`: a new list of each element's `copy()`.
    pub fn copy(&self) -> ListTag {
        let mut copy = Vec::with_capacity(self.list.len());
        for tag in &self.list {
            copy.push(tag.copy());
        }
        ListTag::from_vec(copy)
    }

    /// Port of `ListTag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        visitor.visit_list(self);
    }

    /// Port of `ListTag#accept(StreamTagVisitor)`.
    ///
    /// `visitList` with the element type `write` would use; then per element (wrapped as `write`
    /// would wrap it) `visitElement` -- `SKIP` moves on, `BREAK` ends the list, `HALT` stops --
    /// and the element's own `accept`, whose `BREAK` also ends the list.
    pub fn accept_stream(&self, visitor: &mut dyn StreamTagVisitor) -> ValueResult {
        let element_type = self.identify_raw_element_type();
        match visitor.visit_list(&TagTypes::get_type(element_type as i32), self.size()) {
            ValueResult::Halt => return ValueResult::Halt,
            ValueResult::Break => return visitor.visit_container_end(),
            ValueResult::Continue => {}
        }
        let mut i = 0;
        while (i as usize) < self.list.len() {
            let tag = Self::wrap_if_needed(element_type, &self.list[i as usize]);
            match visitor.visit_element(&tag.get_type(), i) {
                EntryResult::Halt => return ValueResult::Halt,
                EntryResult::Break => return visitor.visit_container_end(),
                EntryResult::Enter => match tag.accept_stream(visitor) {
                    ValueResult::Halt => return ValueResult::Halt,
                    ValueResult::Break => return visitor.visit_container_end(),
                    // Falls through into `case SKIP`.
                    ValueResult::Continue => {}
                },
                EntryResult::Skip => {}
            }
            i += 1;
        }
        visitor.visit_container_end()
    }

    /// Port of `ListTag#hashCode()`: `List.hashCode`, `31 * h + e.hashCode()` from 1 -- or `None`
    /// if an element's hash is an identity hash (see `Tag`'s module docs).
    pub fn java_hash_code(&self) -> Option<i32> {
        let mut h: i32 = 1;
        for e in &self.list {
            h = h.wrapping_mul(31).wrapping_add(e.java_hash_code()?);
        }
        Some(h)
    }

    // ------------------------------------------------------------ the List methods

    /// Port of `ListTag#size()`.
    pub fn size(&self) -> i32 {
        self.list.len() as i32
    }

    /// Port of `ListTag#isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// `ArrayList`'s `Objects.checkIndex(index, size)`.
    fn check_index(&self, index: i32) -> usize {
        let len = self.list.len();
        if index < 0 || index as usize >= len {
            panic!(
                "java.lang.IndexOutOfBoundsException: Index {index} out of bounds for length {len}"
            );
        }
        index as usize
    }

    /// `ArrayList.rangeCheckForAdd(index)`: `0..=size`.
    fn check_index_for_add(&self, index: i32) -> usize {
        let len = self.list.len();
        if index < 0 || index as usize > len {
            panic!("java.lang.IndexOutOfBoundsException: Index: {index}, Size: {len}");
        }
        index as usize
    }

    /// Port of `ListTag#get(int)`: the element itself.
    pub fn get(&self, index: i32) -> &Tag {
        let i = self.check_index(index);
        &self.list[i]
    }

    /// `get(int)`, for editing the element in place -- Java hands out the live element.
    pub fn get_mut(&mut self, index: i32) -> &mut Tag {
        let i = self.check_index(index);
        &mut self.list[i]
    }

    /// Port of `ListTag#set(int, Tag)`: returns the element it replaced.
    pub fn set(&mut self, index: i32, tag: Tag) -> Tag {
        let i = self.check_index(index);
        std::mem::replace(&mut self.list[i], tag)
    }

    /// Port of `ListTag#add(int, Tag)`: inserts at `index`, which may equal `size()`.
    ///
    /// Named for its parameter types (`#overload-naming`): `add(Tag)` is [`ListTag::add`], and
    /// `add_tag` is `CollectionTag`'s `addTag`.
    pub fn add_i32_tag(&mut self, index: i32, tag: Tag) {
        let i = self.check_index_for_add(index);
        self.list.insert(i, tag);
    }

    /// Port of the inherited `AbstractList#add(Tag)`: `add(size(), tag)`, which appends and
    /// returns `true`.
    pub fn add(&mut self, tag: Tag) -> bool {
        self.list.push(tag);
        true
    }

    /// Port of `ListTag#remove(int)`: returns the removed element.
    pub fn remove(&mut self, index: i32) -> Tag {
        let i = self.check_index(index);
        self.list.remove(i)
    }

    /// Port of `ListTag#clear()`.
    pub fn clear(&mut self) {
        self.list.clear();
    }

    /// The elements, in order -- read access for the `List` methods not ported by name.
    pub fn as_slice(&self) -> &[Tag] {
        &self.list
    }

    /// Port of `iterator()` (and of `stream()`, which only ever walks it in order).
    pub fn iter(&self) -> std::slice::Iter<'_, Tag> {
        self.list.iter()
    }

    /// Port of `ListTag#compoundStream()`: the elements that are compounds, in order.
    pub fn compound_stream(&self) -> impl Iterator<Item = &CompoundTag> {
        self.list.iter().filter_map(Tag::as_compound)
    }

    // ------------------------------------------------------------ the typed getters

    /// Port of the private `getNullable(int)`: `None` outside `0..size()`, never a throw.
    fn get_nullable(&self, index: i32) -> Option<&Tag> {
        if index >= 0 && (index as usize) < self.list.len() {
            Some(&self.list[index as usize])
        } else {
            None
        }
    }

    /// Port of `ListTag#getCompound(int)`.
    pub fn get_compound(&self, index: i32) -> Option<&CompoundTag> {
        self.get_nullable(index).and_then(Tag::as_compound)
    }

    /// `getCompound(int)`, for editing the child in place.
    pub fn get_compound_mut(&mut self, index: i32) -> Option<&mut CompoundTag> {
        match self.list.get_mut(usize::try_from(index).ok()?) {
            Some(Tag::Compound(c)) => Some(c),
            _ => None,
        }
    }

    /// Port of `ListTag#getCompoundOrEmpty(int)`: the live child, or a NEW empty compound that
    /// is not added to the list.
    pub fn get_compound_or_empty(&self, index: i32) -> Cow<'_, CompoundTag> {
        match self.get_compound(index) {
            Some(c) => Cow::Borrowed(c),
            None => Cow::Owned(CompoundTag::new()),
        }
    }

    /// Port of `ListTag#getList(int)`.
    pub fn get_list(&self, index: i32) -> Option<&ListTag> {
        self.get_nullable(index).and_then(Tag::as_list)
    }

    /// `getList(int)`, for editing the child in place.
    pub fn get_list_mut(&mut self, index: i32) -> Option<&mut ListTag> {
        match self.list.get_mut(usize::try_from(index).ok()?) {
            Some(Tag::List(l)) => Some(l),
            _ => None,
        }
    }

    /// Port of `ListTag#getListOrEmpty(int)`: the live child, or a NEW empty list.
    pub fn get_list_or_empty(&self, index: i32) -> Cow<'_, ListTag> {
        match self.get_list(index) {
            Some(l) => Cow::Borrowed(l),
            None => Cow::Owned(ListTag::new()),
        }
    }

    /// Port of `ListTag#getShort(int)`: `getOptional(i).flatMap(Tag::asShort)`.
    pub fn get_short(&self, index: i32) -> Option<i16> {
        self.get_nullable(index).and_then(Tag::as_short)
    }

    /// Port of `ListTag#getShortOr(int, short)`.
    pub fn get_short_or(&self, index: i32, default_value: i16) -> i16 {
        self.get_nullable(index)
            .and_then(Tag::as_numeric)
            .map_or(default_value, |n| n.short_value())
    }

    /// Port of `ListTag#getInt(int)`.
    pub fn get_int(&self, index: i32) -> Option<i32> {
        self.get_nullable(index).and_then(Tag::as_int)
    }

    /// Port of `ListTag#getIntOr(int, int)`.
    pub fn get_int_or(&self, index: i32, default_value: i32) -> i32 {
        self.get_nullable(index)
            .and_then(Tag::as_numeric)
            .map_or(default_value, |n| n.int_value())
    }

    /// Port of `ListTag#getIntArray(int)`: the live array of an `IntArrayTag`.
    pub fn get_int_array(&self, index: i32) -> Option<&[i32]> {
        match self.get_nullable(index) {
            Some(Tag::IntArray(t)) => Some(t.get_as_int_array()),
            _ => None,
        }
    }

    /// Port of `ListTag#getLongArray(int)`.
    pub fn get_long_array(&self, index: i32) -> Option<&[i64]> {
        match self.get_nullable(index) {
            Some(Tag::LongArray(t)) => Some(t.get_as_long_array()),
            _ => None,
        }
    }

    /// Port of `ListTag#getDouble(int)`.
    pub fn get_double(&self, index: i32) -> Option<f64> {
        self.get_nullable(index).and_then(Tag::as_double)
    }

    /// Port of `ListTag#getDoubleOr(int, double)`.
    pub fn get_double_or(&self, index: i32, default_value: f64) -> f64 {
        self.get_nullable(index)
            .and_then(Tag::as_numeric)
            .map_or(default_value, |n| n.double_value())
    }

    /// Port of `ListTag#getFloat(int)`.
    pub fn get_float(&self, index: i32) -> Option<f32> {
        self.get_nullable(index).and_then(Tag::as_float)
    }

    /// Port of `ListTag#getFloatOr(int, float)`.
    pub fn get_float_or(&self, index: i32, default_value: f32) -> f32 {
        self.get_nullable(index)
            .and_then(Tag::as_numeric)
            .map_or(default_value, |n| n.float_value())
    }

    /// Port of `ListTag#getString(int)`: `asString()`, so only a `StringTag` answers.
    pub fn get_string(&self, index: i32) -> Option<&str> {
        match self.get_nullable(index) {
            Some(Tag::String(t)) => Some(t.value()),
            _ => None,
        }
    }

    /// Port of `ListTag#getStringOr(int, String)`.
    pub fn get_string_or<'a>(&'a self, index: i32, default_value: &'a str) -> &'a str {
        self.get_string(index).unwrap_or(default_value)
    }

    // ------------------------------------------------------------ TYPE

    /// Port of `ListTag.TYPE.load(DataInput, NbtAccounter)`.
    pub fn load_type<I: DataInput + ?Sized>(
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<Tag> {
        accounter.push_depth();
        let result = Self::load_list(input, accounter);
        accounter.pop_depth();
        result.map(Tag::List)
    }

    /// Port of `TYPE`'s private `loadList`.
    fn load_list<I: DataInput + ?Sized>(
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<ListTag> {
        accounter.account_bytes(36);
        let type_id = input.read_byte()?;
        let count = Self::read_list_count(input)?;
        if type_id == 0 && count > 0 {
            NbtFormatException::throw("Missing type on ListTag");
        }
        accounter.account_bytes_per_entry(4, count as i64);
        let element_type = TagTypes::get_type(type_id as i32);
        // `new ArrayList<>(count)`. The capacity is not observable; capping it keeps a hostile
        // count under an unlimited quota from reserving gigabytes before the first read fails.
        let mut list = ListTag::from_vec(Vec::with_capacity((count as usize).min(4096)));
        for _ in 0..count {
            list.add_and_unwrap(element_type.load(input, accounter)?);
        }
        Ok(list)
    }

    /// Port of `TYPE`'s private `readListCount`.
    fn read_list_count<I: DataInput + ?Sized>(input: &mut I) -> std::io::Result<i32> {
        let count = input.read_int()?;
        if count < 0 {
            NbtFormatException::throw(format!("ListTag length cannot be negative: {count}"));
        }
        Ok(count)
    }

    /// Port of `ListTag.TYPE.parse(DataInput, StreamTagVisitor, NbtAccounter)`.
    pub fn parse_type<I: DataInput + ?Sized>(
        input: &mut I,
        output: &mut dyn StreamTagVisitor,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<ValueResult> {
        accounter.push_depth();
        let result = Self::parse_list(input, output, accounter);
        accounter.pop_depth();
        result
    }

    /// Port of `TYPE`'s private `parseList`.
    fn parse_list<I: DataInput + ?Sized>(
        input: &mut I,
        output: &mut dyn StreamTagVisitor,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<ValueResult> {
        accounter.account_bytes(36);
        let element_type = TagTypes::get_type(input.read_byte()? as i32);
        let count = Self::read_list_count(input)?;
        match output.visit_list(&element_type, count) {
            ValueResult::Halt => return Ok(ValueResult::Halt),
            ValueResult::Break => {
                element_type.skip_count(input, count, accounter)?;
                return Ok(output.visit_container_end());
            }
            ValueResult::Continue => {}
        }
        accounter.account_bytes_per_entry(4, count as i64);
        let mut i = 0;
        loop {
            if i < count {
                match output.visit_element(&element_type, i) {
                    EntryResult::Halt => return Ok(ValueResult::Halt),
                    EntryResult::Break => element_type.skip(input, accounter)?,
                    EntryResult::Skip => {
                        element_type.skip(input, accounter)?;
                        i += 1;
                        continue;
                    }
                    EntryResult::Enter => match element_type.parse(input, output, accounter)? {
                        ValueResult::Halt => return Ok(ValueResult::Halt),
                        ValueResult::Break => {}
                        ValueResult::Continue => {
                            i += 1;
                            continue;
                        }
                    },
                }
            }
            let amount_to_skip = count - 1 - i;
            if amount_to_skip > 0 {
                element_type.skip_count(input, amount_to_skip, accounter)?;
            }
            return Ok(output.visit_container_end());
        }
    }

    /// Port of `ListTag.TYPE.skip(DataInput, NbtAccounter)`. No negative-count check here.
    pub fn skip_type<I: DataInput + ?Sized>(
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<()> {
        accounter.push_depth();
        let result = (|| {
            let element_type = TagTypes::get_type(input.read_byte()? as i32);
            let count = input.read_int()?;
            element_type.skip_count(input, count, accounter)
        })();
        accounter.pop_depth();
        result
    }

    /// Wraps this list as a [`Tag`].
    pub fn into_tag(self) -> Tag {
        Tag::List(self)
    }
}

impl CollectionTag for ListTag {
    /// Port of `ListTag#clear()`.
    fn clear(&mut self) {
        self.list.clear();
    }

    /// Port of `ListTag#setTag(int, Tag)`: always `true` -- any tag kind is accepted.
    fn set_tag(&mut self, index: i32, tag: &Tag) -> bool {
        self.set(index, tag.clone());
        true
    }

    /// Port of `ListTag#addTag(int, Tag)`: always `true`.
    fn add_tag(&mut self, index: i32, tag: &Tag) -> bool {
        self.add_i32_tag(index, tag.clone());
        true
    }

    /// Port of `ListTag#remove(int)`.
    fn remove(&mut self, index: i32) -> Tag {
        ListTag::remove(self, index)
    }

    /// Port of `ListTag#get(int)`, by value as the trait requires.
    fn get(&self, index: i32) -> Tag {
        ListTag::get(self, index).clone()
    }

    /// Port of `ListTag#size()`.
    fn size(&self) -> i32 {
        ListTag::size(self)
    }
}

impl<'a> IntoIterator for &'a ListTag {
    type Item = &'a Tag;
    type IntoIter = std::slice::Iter<'a, Tag>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.iter()
    }
}

/// Port of `ListTag#toString()`: `StringTagVisitor.visitList`.
impl std::fmt::Display for ListTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = StringTagVisitor::new();
        visitor.visit_list(self);
        f.write_str(&visitor.build())
    }
}
