//! Port of: net/minecraft/nbt/visitors/CollectToTag.java
//! Java class(es): net.minecraft.nbt.visitors.CollectToTag, CollectToTag.CompoundBuilder, CollectToTag.ContainerBuilder, CollectToTag.ListBuilder, CollectToTag.RootBuilder
//! Status: VERIFIED
//!
//! Session 23. `public class CollectToTag implements StreamTagVisitor`: rebuilds the tag a stream
//! describes. A stack of container builders, with a `RootBuilder` at the bottom; every value is
//! handed to the top builder, `visitRootEntry`/`visitEntry(type, id)`/`visitElement` push a
//! `ListBuilder` or `CompoundBuilder` when the type is `LIST` or `COMPOUND`, and
//! `visitContainerEnd` pops the top builder and hands what it built to the one below.
//!
//! What it rebuilds is not always what was written:
//!
//! * values go through `valueOf`, so a `-0.0` float or double comes back as `+0.0`;
//! * a `ListBuilder` adds each element with `addAndUnwrap`, so `{"": x}` elements unwrap;
//! * a compound entry is `put` under the LAST key seen (`lastId`), so the result has whatever
//!   iteration order the keys' insertion gives -- `CompoundTag.put` order, i.e. stream order.
//!
//! Every answer is `CONTINUE`/`ENTER`; the two subclasses (`CollectFields`, `SkipFields`) override
//! that. `depth()` is the stack size minus one: 0 before the root, 1 inside the root compound.
//!
//! # Port shape
//!
//! Java's builders hold tag OBJECTS and hand the reference up at `visitContainerEnd`; here each
//! builder owns its tag and moves it into the parent when it is popped -- the same tree. The
//! private `ContainerBuilder` interface and its three implementations are [`ContainerBuilder`].
//! The protected methods the subclasses call through `super.` are public inherent methods with
//! the trait methods' names, so `CollectFields` can call `self.base.visit_entry(..)` exactly
//! where Java calls `super.visitEntry(..)`.
//!
//! Popping past the root, or reading the top of an empty stack, throws
//! `NoSuchElementException` from `ArrayDeque` in Java; the port panics with that class. A
//! well-formed stream never does either.
//!
//! Verified by the `PW`/`PU` rows of every `GEN`/`CASE` tree in `_porting/test-data/nbt_probe.txt`
//! (`NbtIo.parse` into a `CollectToTag`, then the result's bytes and the quota used), in
//! `tests/session23.rs`.

use std::collections::VecDeque;

use crate::net::minecraft::nbt::ByteArrayTag::ByteArrayTag;
use crate::net::minecraft::nbt::ByteTag::ByteTag;
use crate::net::minecraft::nbt::CompoundTag::{self, CompoundTag as CompoundTagValue};
use crate::net::minecraft::nbt::DoubleTag::DoubleTag;
use crate::net::minecraft::nbt::FloatTag::FloatTag;
use crate::net::minecraft::nbt::IntArrayTag::IntArrayTag;
use crate::net::minecraft::nbt::IntTag::IntTag;
use crate::net::minecraft::nbt::ListTag::{self, ListTag as ListTagValue};
use crate::net::minecraft::nbt::LongArrayTag::LongArrayTag;
use crate::net::minecraft::nbt::LongTag::LongTag;
use crate::net::minecraft::nbt::ShortTag::ShortTag;
use crate::net::minecraft::nbt::StreamTagVisitor::{EntryResult, StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::StringTag::StringTag;
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagType::TagTypeRef;

/// Port of the private `CollectToTag.ContainerBuilder` and its three implementations.
#[derive(Debug, Clone)]
enum ContainerBuilder {
    /// `RootBuilder`: keeps the last value it was given (`@Nullable`).
    Root(Option<Tag>),
    /// `CompoundBuilder`: `put(lastId, value)`; `lastId` starts as `""`.
    Compound {
        compound: CompoundTagValue,
        last_id: String,
    },
    /// `ListBuilder`: `addAndUnwrap(value)`.
    List(ListTagValue),
}

impl ContainerBuilder {
    /// `acceptKey(String)`: only `CompoundBuilder` overrides the empty default.
    fn accept_key(&mut self, id: &str) {
        if let ContainerBuilder::Compound { last_id, .. } = self {
            *last_id = id.to_string();
        }
    }

    /// `acceptValue(Tag)`.
    fn accept_value(&mut self, tag: Tag) {
        match self {
            ContainerBuilder::Root(result) => *result = Some(tag),
            ContainerBuilder::Compound { compound, last_id } => {
                compound.put(last_id, tag);
            }
            ContainerBuilder::List(list) => list.add_and_unwrap(tag),
        }
    }

    /// `build()`: the root's value (which may be null), or the container itself.
    fn build(self) -> Option<Tag> {
        match self {
            ContainerBuilder::Root(result) => result,
            ContainerBuilder::Compound { compound, .. } => Some(compound.into_tag()),
            ContainerBuilder::List(list) => Some(list.into_tag()),
        }
    }

    /// `build()` without giving the builder up, for `getResult()` on the bottom one.
    fn build_ref(&self) -> Option<Tag> {
        self.clone().build()
    }
}

fn no_such_element() -> ! {
    panic!("java.util.NoSuchElementException")
}

/// Port of `net.minecraft.nbt.visitors.CollectToTag`.
#[derive(Debug, Clone)]
pub struct CollectToTag {
    container_stack: VecDeque<ContainerBuilder>,
}

impl Default for CollectToTag {
    fn default() -> Self {
        Self::new()
    }
}

impl CollectToTag {
    /// Port of `new CollectToTag()`: a `RootBuilder` on an otherwise empty stack.
    pub fn new() -> Self {
        let mut container_stack = VecDeque::new();
        container_stack.push_back(ContainerBuilder::Root(None));
        CollectToTag { container_stack }
    }

    /// Port of `getResult()`: `containerStack.getFirst().build()` -- the root's value once the
    /// stream is complete, `None` (Java `null`) before.
    pub fn get_result(&self) -> Option<Tag> {
        self.container_stack
            .front()
            .unwrap_or_else(|| no_such_element())
            .build_ref()
    }

    /// Port of the protected `depth()`: `containerStack.size() - 1`.
    pub fn depth(&self) -> i32 {
        self.container_stack.len() as i32 - 1
    }

    /// Port of the private `appendEntry(Tag)`: to the top builder.
    fn append_entry(&mut self, instance: Tag) {
        self.container_stack
            .back_mut()
            .unwrap_or_else(|| no_such_element())
            .accept_value(instance);
    }

    /// Port of the private `enterContainerIfNeeded(TagType)`.
    fn enter_container_if_needed(&mut self, tag_type: &TagTypeRef) {
        if *tag_type == ListTag::TYPE {
            self.container_stack
                .push_back(ContainerBuilder::List(ListTagValue::new()));
        } else if *tag_type == CompoundTag::TYPE {
            self.container_stack.push_back(ContainerBuilder::Compound {
                compound: CompoundTagValue::new(),
                last_id: String::new(),
            });
        }
    }

    /// Port of `visitEnd()`: appends `EndTag.INSTANCE`.
    pub fn visit_end(&mut self) -> ValueResult {
        self.append_entry(Tag::End);
        ValueResult::Continue
    }

    /// Port of `visit(String)`.
    pub fn visit_string(&mut self, value: &str) -> ValueResult {
        self.append_entry(StringTag::value_of(value).into_tag());
        ValueResult::Continue
    }

    /// Port of `visit(byte)`.
    pub fn visit_byte(&mut self, value: i8) -> ValueResult {
        self.append_entry(ByteTag::value_of(value).into_tag());
        ValueResult::Continue
    }

    /// Port of `visit(short)`.
    pub fn visit_short(&mut self, value: i16) -> ValueResult {
        self.append_entry(ShortTag::value_of(value).into_tag());
        ValueResult::Continue
    }

    /// Port of `visit(int)`.
    pub fn visit_int(&mut self, value: i32) -> ValueResult {
        self.append_entry(IntTag::value_of(value).into_tag());
        ValueResult::Continue
    }

    /// Port of `visit(long)`.
    pub fn visit_long(&mut self, value: i64) -> ValueResult {
        self.append_entry(LongTag::value_of(value).into_tag());
        ValueResult::Continue
    }

    /// Port of `visit(float)`: through `FloatTag.valueOf`, so `-0.0F` becomes `+0.0F`.
    pub fn visit_float(&mut self, value: f32) -> ValueResult {
        self.append_entry(FloatTag::value_of(value).into_tag());
        ValueResult::Continue
    }

    /// Port of `visit(double)`: through `DoubleTag.valueOf`, so `-0.0` becomes `+0.0`.
    pub fn visit_double(&mut self, value: f64) -> ValueResult {
        self.append_entry(DoubleTag::value_of(value).into_tag());
        ValueResult::Continue
    }

    /// Port of `visit(byte[])`.
    pub fn visit_byte_array(&mut self, value: &[i8]) -> ValueResult {
        self.append_entry(ByteArrayTag::new(value.to_vec()).into_tag());
        ValueResult::Continue
    }

    /// Port of `visit(int[])`.
    pub fn visit_int_array(&mut self, value: &[i32]) -> ValueResult {
        self.append_entry(IntArrayTag::new(value.to_vec()).into_tag());
        ValueResult::Continue
    }

    /// Port of `visit(long[])`.
    pub fn visit_long_array(&mut self, value: &[i64]) -> ValueResult {
        self.append_entry(LongArrayTag::new(value.to_vec()).into_tag());
        ValueResult::Continue
    }

    /// Port of `visitList(TagType, int)`: nothing to do -- the builder was pushed by the entry.
    pub fn visit_list(&mut self, _element_type: &TagTypeRef, _size: i32) -> ValueResult {
        ValueResult::Continue
    }

    /// Port of `visitElement(TagType, int)`.
    pub fn visit_element(&mut self, tag_type: &TagTypeRef, _index: i32) -> EntryResult {
        self.enter_container_if_needed(tag_type);
        EntryResult::Enter
    }

    /// Port of `visitEntry(TagType)`.
    pub fn visit_entry(&mut self, _tag_type: &TagTypeRef) -> EntryResult {
        EntryResult::Enter
    }

    /// Port of `visitEntry(TagType, String)`: the key goes to the top builder BEFORE a new
    /// container is pushed for the value.
    pub fn visit_entry_named(&mut self, tag_type: &TagTypeRef, id: &str) -> EntryResult {
        self.container_stack
            .back_mut()
            .unwrap_or_else(|| no_such_element())
            .accept_key(id);
        self.enter_container_if_needed(tag_type);
        EntryResult::Enter
    }

    /// Port of `visitContainerEnd()`: pop, build, hand to the new top (unless the build is null).
    pub fn visit_container_end(&mut self) -> ValueResult {
        let container = self
            .container_stack
            .pop_back()
            .unwrap_or_else(|| no_such_element());
        if let Some(tag) = container.build() {
            self.container_stack
                .back_mut()
                .unwrap_or_else(|| no_such_element())
                .accept_value(tag);
        }
        ValueResult::Continue
    }

    /// Port of `visitRootEntry(TagType)`.
    pub fn visit_root_entry(&mut self, tag_type: &TagTypeRef) -> ValueResult {
        self.enter_container_if_needed(tag_type);
        ValueResult::Continue
    }
}

impl StreamTagVisitor for CollectToTag {
    fn visit_end(&mut self) -> ValueResult {
        CollectToTag::visit_end(self)
    }
    fn visit_string(&mut self, value: &str) -> ValueResult {
        CollectToTag::visit_string(self, value)
    }
    fn visit_byte(&mut self, value: i8) -> ValueResult {
        CollectToTag::visit_byte(self, value)
    }
    fn visit_short(&mut self, value: i16) -> ValueResult {
        CollectToTag::visit_short(self, value)
    }
    fn visit_int(&mut self, value: i32) -> ValueResult {
        CollectToTag::visit_int(self, value)
    }
    fn visit_long(&mut self, value: i64) -> ValueResult {
        CollectToTag::visit_long(self, value)
    }
    fn visit_float(&mut self, value: f32) -> ValueResult {
        CollectToTag::visit_float(self, value)
    }
    fn visit_double(&mut self, value: f64) -> ValueResult {
        CollectToTag::visit_double(self, value)
    }
    fn visit_byte_array(&mut self, value: &[i8]) -> ValueResult {
        CollectToTag::visit_byte_array(self, value)
    }
    fn visit_int_array(&mut self, value: &[i32]) -> ValueResult {
        CollectToTag::visit_int_array(self, value)
    }
    fn visit_long_array(&mut self, value: &[i64]) -> ValueResult {
        CollectToTag::visit_long_array(self, value)
    }
    fn visit_list(&mut self, element_type: &TagTypeRef, size: i32) -> ValueResult {
        CollectToTag::visit_list(self, element_type, size)
    }
    fn visit_entry(&mut self, tag_type: &TagTypeRef) -> EntryResult {
        CollectToTag::visit_entry(self, tag_type)
    }
    fn visit_entry_named(&mut self, tag_type: &TagTypeRef, id: &str) -> EntryResult {
        CollectToTag::visit_entry_named(self, tag_type, id)
    }
    fn visit_element(&mut self, tag_type: &TagTypeRef, index: i32) -> EntryResult {
        CollectToTag::visit_element(self, tag_type, index)
    }
    fn visit_container_end(&mut self) -> ValueResult {
        CollectToTag::visit_container_end(self)
    }
    fn visit_root_entry(&mut self, tag_type: &TagTypeRef) -> ValueResult {
        CollectToTag::visit_root_entry(self, tag_type)
    }
}

/// Implements every `StreamTagVisitor` method that a `CollectToTag` subclass does NOT override
/// by forwarding to its `base: CollectToTag` -- Java's inheritance, spelled out.
macro_rules! forward_collect_to_tag {
    ($($name:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {
        $(fn $name(&mut self, $($arg: $ty),*) -> $ret {
            self.base.$name($($arg),*)
        })*
    };
}
pub(crate) use forward_collect_to_tag;
