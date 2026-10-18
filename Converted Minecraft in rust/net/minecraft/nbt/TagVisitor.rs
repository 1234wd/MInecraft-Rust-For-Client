//! Port of: net/minecraft/nbt/TagVisitor.java
//! Java class(es): net.minecraft.nbt.TagVisitor
//! Status: VERIFIED
//!
//! Session 21 ported eleven of the thirteen `visitX` methods; session 23 added `visitList` and
//! `visitCompound` with `ListTag` and `CompoundTag`, so the interface is complete. It has no
//! behaviour of its own: its one implementor, `StringTagVisitor`, is verified against the
//! `PRIM`/`QUOTE`/`CASEW`/`GEN`/`CASE` rows of `nbt_probe.txt`.
//!
//! # `visitEnd` TAKES NO ARGUMENT HERE
//!
//! Java's is `void visitEnd(EndTag tag)`. `EndTag` is a singleton with no state and a
//! private constructor, so the parameter carries no information -- it exists because Java
//! has no way to write "the one instance" into a signature. The port drops it: there is no
//! `EndTag` *value* in the Rust model either, only [`Tag::End`](crate::net::minecraft::nbt::Tag::Tag::End).
//!
//! This is the one place in the batch where a signature deliberately differs from Java's,
//! so it is called out rather than left for the next reader to notice.

use crate::net::minecraft::nbt::ByteArrayTag::ByteArrayTag;
use crate::net::minecraft::nbt::ByteTag::ByteTag;
use crate::net::minecraft::nbt::CompoundTag::CompoundTag;
use crate::net::minecraft::nbt::DoubleTag::DoubleTag;
use crate::net::minecraft::nbt::FloatTag::FloatTag;
use crate::net::minecraft::nbt::IntArrayTag::IntArrayTag;
use crate::net::minecraft::nbt::IntTag::IntTag;
use crate::net::minecraft::nbt::ListTag::ListTag;
use crate::net::minecraft::nbt::LongArrayTag::LongArrayTag;
use crate::net::minecraft::nbt::LongTag::LongTag;
use crate::net::minecraft::nbt::ShortTag::ShortTag;
use crate::net::minecraft::nbt::StringTag::StringTag;

/// Port of `net.minecraft.nbt.TagVisitor`.
pub trait TagVisitor {
    /// Port of `TagVisitor#visitString(StringTag)`.
    fn visit_string(&mut self, tag: &StringTag);

    /// Port of `TagVisitor#visitByte(ByteTag)`.
    fn visit_byte(&mut self, tag: &ByteTag);

    /// Port of `TagVisitor#visitShort(ShortTag)`.
    fn visit_short(&mut self, tag: &ShortTag);

    /// Port of `TagVisitor#visitInt(IntTag)`.
    fn visit_int(&mut self, tag: &IntTag);

    /// Port of `TagVisitor#visitLong(LongTag)`.
    fn visit_long(&mut self, tag: &LongTag);

    /// Port of `TagVisitor#visitFloat(FloatTag)`.
    fn visit_float(&mut self, tag: &FloatTag);

    /// Port of `TagVisitor#visitDouble(DoubleTag)`.
    fn visit_double(&mut self, tag: &DoubleTag);

    /// Port of `TagVisitor#visitByteArray(ByteArrayTag)`.
    fn visit_byte_array(&mut self, tag: &ByteArrayTag);

    /// Port of `TagVisitor#visitIntArray(IntArrayTag)`.
    fn visit_int_array(&mut self, tag: &IntArrayTag);

    /// Port of `TagVisitor#visitLongArray(LongArrayTag)`.
    fn visit_long_array(&mut self, tag: &LongArrayTag);

    /// Port of `TagVisitor#visitList(ListTag)`.
    fn visit_list(&mut self, tag: &ListTag);

    /// Port of `TagVisitor#visitCompound(CompoundTag)`.
    fn visit_compound(&mut self, tag: &CompoundTag);

    /// Port of `TagVisitor#visitEnd(EndTag)`.
    ///
    /// The `EndTag` parameter is dropped -- see the module docs.
    fn visit_end(&mut self);
}
