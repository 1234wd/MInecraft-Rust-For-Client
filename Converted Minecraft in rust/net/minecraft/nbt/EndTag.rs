//! Port of: net/minecraft/nbt/EndTag.java
//! Java class(es): net.minecraft.nbt.EndTag, EndTag::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 21. Every member: the singleton, `TYPE` (load/parse/both skips/both names),
//! `write`, `sizeInBytes`, `getId`, `getType`, `toString`, `copy` and both `accept`
//! overloads. Verified against the `TT` and `CASEW` rows of
//! `_porting/test-data/nbt_probe.txt` by `_porting/tests/session19_g.rs`.
//!
//! # THE SINGLETON HAS NO RUST TYPE
//!
//! `EndTag` is `final`, has a private no-arg constructor, one `public static final INSTANCE`
//! and no fields. So the type carries exactly zero bits of information, and in the port it
//! is the unit variant [`Tag::End`](crate::net::minecraft::nbt::Tag::Tag::End) rather than a
//! struct. [`INSTANCE`] is that variant, so Java's `EndTag.INSTANCE` transliterates.
//!
//! That is also why [`crate::net::minecraft::nbt::TagVisitor::TagVisitor::visit_end`] takes
//! no argument: Java's `visitEnd(EndTag)` parameter is the singleton and nothing else.
//!
//! # `write` IS EMPTY, AND `sizeInBytes` IS NOT
//!
//! `EndTag.write` writes nothing -- TAG_End is a one-byte type *id* in the enclosing
//! compound's stream, never a payload. But `sizeInBytes()` is `8`, and `TYPE.load` charges
//! the accounter `8` bytes for it. So an end tag costs heap budget while costing zero wire
//! bytes. That asymmetry is deliberate in vanilla (the 8 is `Tag.OBJECT_HEADER`, the JVM
//! object header it would occupy) and is what makes a stream of a billion end tags cost
//! quota.
//!
//! # BOTH `skip`s DO NOTHING
//!
//! `skip(input, accounter)` and `skip(input, count, accounter)` have empty bodies -- there
//! is nothing to skip. Note this is *not* `TagType.StaticSize`, which would skip
//! `size() * count` bytes; `EndTag.TYPE` implements `TagType` directly and overrides both.

use crate::javacompat::java_io::DataOutput;
use crate::net::minecraft::nbt::StreamTagVisitor::{StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagType::TagTypeRef;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;

/// Port of the private `EndTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 8;

/// Port of `EndTag#TYPE`.
pub const TYPE: TagTypeRef = TagTypeRef::Known(0);

/// Port of `EndTag#INSTANCE`.
///
/// The singleton is the unit variant -- see the module docs.
pub const INSTANCE: Tag = Tag::End;

/// Port of `EndTag#write(DataOutput)`.
///
/// Writes nothing. See the module docs for why that is not an oversight.
pub fn write<O: DataOutput + ?Sized>(_output: &mut O) -> std::io::Result<()> {
    Ok(())
}

/// Port of `EndTag#sizeInBytes()`.
pub const fn size_in_bytes() -> i32 {
    SELF_SIZE_IN_BYTES
}

/// Port of `EndTag#getId()`.
pub const fn get_id() -> i8 {
    0
}

/// Port of `EndTag#getType()`.
pub const fn get_type() -> TagTypeRef {
    TYPE
}

/// Port of `EndTag#copy()`.
pub const fn copy() -> Tag {
    Tag::End
}

/// Port of `EndTag#accept(TagVisitor)`.
pub fn accept(visitor: &mut dyn TagVisitor) {
    visitor.visit_end();
}

/// Port of `EndTag#accept(StreamTagVisitor)`.
pub fn accept_stream(visitor: &mut dyn StreamTagVisitor) -> ValueResult {
    visitor.visit_end()
}

/// Port of `EndTag#toString()`.
///
/// `StringTagVisitor.visitEnd` appends the literal `END`, so this is that string. Kept as a
/// function rather than a `Display` impl because there is no `EndTag` value to implement it
/// on -- `Tag::End`'s `Display` goes through the same visitor.
pub fn to_java_string() -> String {
    let mut visitor = crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor::new();
    visitor.visit_end();
    visitor.build()
}
