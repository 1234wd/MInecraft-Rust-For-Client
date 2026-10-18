//! Port of: net/minecraft/nbt/TagType.java
//! Java class(es): net.minecraft.nbt.TagType, net.minecraft.nbt.StaticSize, net.minecraft.nbt.VariableSize, TagType::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 21 ported the interface, both nested sub-interfaces, `createInvalid`, and the per-kind
//! `load`/`parse`/`skip`/`getName`/`getPrettyName` for eleven tag kinds; session 23 filled the
//! `LIST` and `COMPOUND` arms, which delegate to `ListTag`/`CompoundTag`'s ports of their
//! anonymous `TYPE`s. Verified against the `TT`, `TTEX` and `SKIP` rows of
//! `_porting/test-data/nbt_probe.txt` by `_porting/tests/session19_g.rs`, and through the
//! containers' `TYPE`s by the `GEN`/`CASE`/`TLOAD` rows in `_porting/tests/session23.rs`.
//!
//! # `TagType<T>` BECOMES A TAG-ID ENUM, NOT A TRAIT OBJECT
//!
//! Java has thirteen anonymous `TagType` instances, one per kind, reachable as
//! `ByteTag.TYPE` and from `TagTypes.getType(id)`. They are singletons distinguished only
//! by which tag kind they serve, so the port is [`TagTypeRef`]: `Known(id)` for the
//! thirteen, `Invalid(id)` for `createInvalid`.
//!
//! That choice buys two things a `&'static dyn TagType` would not. `TagTypeRef` is `Copy`
//! and `PartialEq`, so `visitRootEntry(this.getType())` passes it by value and a test can
//! compare types directly; and `createInvalid(id)` needs to carry its `id` into
//! `getName()` and into its error message, which a static cannot.
//!
//! The generic parameter `T extends Tag` is dropped. Every caller immediately widens to
//! `Tag` (`TagTypes.getType` already returns `TagType<?>`), and the enum
//! [`Tag`](crate::net::minecraft::nbt::Tag) is the only possible return type, so the
//! parameter carried no information the port can use.
//!
//! # `StaticSize` vs `VariableSize` IS THE SKIP STRATEGY, AND IT IS OBSERVABLE
//!
//! ```java
//! interface StaticSize<T> extends TagType<T> {
//!     default void skip(input, accounter)        { input.skipBytes(this.size()); }
//!     default void skip(input, count, accounter) { input.skipBytes(this.size() * count); }
//! }
//! interface VariableSize<T> extends TagType<T> {
//!     default void skip(input, count, accounter) { for (i < count) this.skip(input, accounter); }
//! }
//! ```
//!
//! So a static-size kind skips `size() * count` in **one** call with a wrapping `int`
//! multiply, while a variable-size kind loops and reads each element's header. The
//! difference shows up on a hostile stream: `skipBytes(2 * 0x40000000)` wraps to a
//! negative, and `DataInputStream.skipBytes` of a negative value skips nothing and returns
//! 0 rather than throwing. [`static_size`](TagTypeRef::static_size) is the discriminator,
//! and `EndTag`'s type is neither -- it implements `TagType` directly with two empty
//! `skip`s, which is why `static_size` returns `None` for it but
//! [`skip`](TagTypeRef::skip) still does nothing.
//!
//! # `createInvalid` THROWS FROM EVERY METHOD, INCLUDING `skip`
//!
//! Its four data methods all raise `new IOException("Invalid tag id: " + id)` -- `load`,
//! `parse` and **both** `skip`s. So an unknown tag id cannot be skipped over; the read
//! fails. `getName()` is `INVALID[<id>]` and `getPrettyName()` is `UNKNOWN_<id>`, and the
//! `TT` oracle rows cover ids `-2`, `-1`, `13` and `14`.
//!
//! The `IOException` is a *checked* exception that `NbtIo` propagates to its caller, so it
//! is a [`std::io::Error`] carrying the Java class name (via `javacompat::java_io`), not a
//! panic. That is the `panic-not-Result` rule's catch-and-continue half.

use crate::javacompat::java_io::{self, DataInput};
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
use crate::net::minecraft::nbt::NbtAccounter::NbtAccounter;
use crate::net::minecraft::nbt::ShortTag::ShortTag;
use crate::net::minecraft::nbt::StreamTagVisitor::{StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::StringTag::StringTag;
use crate::net::minecraft::nbt::Tag::Tag;

/// Port of `net.minecraft.nbt.TagType` and its two nested sub-interfaces.
///
/// `Known(id)` is one of the thirteen singletons; `Invalid(id)` is
/// `TagType.createInvalid(id)`. See the module docs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TagTypeRef {
    /// One of the thirteen per-kind singletons, by tag id (`0..=12`).
    Known(i8),
    /// Port of `TagType#createInvalid(int)`.
    Invalid(i32),
}

impl TagTypeRef {
    /// The `TagTypeRef` for a tag id, without `TagTypes`' bounds check.
    ///
    /// Callers that have a *known-valid* id (every `Tag::getType`) use this;
    /// [`crate::net::minecraft::nbt::TagTypes::get_type`] is the one that decides whether an
    /// id is valid at all.
    pub const fn of_id(type_id: i8) -> Self {
        TagTypeRef::Known(type_id)
    }

    /// Port of `TagType#getName()`.
    pub fn get_name(&self) -> String {
        match self {
            TagTypeRef::Known(0) => "END".to_string(),
            TagTypeRef::Known(1) => "BYTE".to_string(),
            TagTypeRef::Known(2) => "SHORT".to_string(),
            TagTypeRef::Known(3) => "INT".to_string(),
            TagTypeRef::Known(4) => "LONG".to_string(),
            TagTypeRef::Known(5) => "FLOAT".to_string(),
            TagTypeRef::Known(6) => "DOUBLE".to_string(),
            TagTypeRef::Known(7) => "BYTE[]".to_string(),
            TagTypeRef::Known(8) => "STRING".to_string(),
            TagTypeRef::Known(9) => "LIST".to_string(),
            TagTypeRef::Known(10) => "COMPOUND".to_string(),
            TagTypeRef::Known(11) => "INT[]".to_string(),
            TagTypeRef::Known(12) => "LONG[]".to_string(),
            // `Known` outside 0..=12 cannot be built by `TagTypes.getType`, which routes
            // those to `Invalid`. Reachable only by constructing the variant directly.
            TagTypeRef::Known(other) => format!("INVALID[{other}]"),
            TagTypeRef::Invalid(id) => format!("INVALID[{id}]"),
        }
    }

    /// Port of `TagType#getPrettyName()`.
    pub fn get_pretty_name(&self) -> String {
        match self {
            TagTypeRef::Known(0) => "TAG_End".to_string(),
            TagTypeRef::Known(1) => "TAG_Byte".to_string(),
            TagTypeRef::Known(2) => "TAG_Short".to_string(),
            TagTypeRef::Known(3) => "TAG_Int".to_string(),
            TagTypeRef::Known(4) => "TAG_Long".to_string(),
            TagTypeRef::Known(5) => "TAG_Float".to_string(),
            TagTypeRef::Known(6) => "TAG_Double".to_string(),
            TagTypeRef::Known(7) => "TAG_Byte_Array".to_string(),
            TagTypeRef::Known(8) => "TAG_String".to_string(),
            TagTypeRef::Known(9) => "TAG_List".to_string(),
            TagTypeRef::Known(10) => "TAG_Compound".to_string(),
            TagTypeRef::Known(11) => "TAG_Int_Array".to_string(),
            TagTypeRef::Known(12) => "TAG_Long_Array".to_string(),
            TagTypeRef::Known(other) => format!("UNKNOWN_{other}"),
            TagTypeRef::Invalid(id) => format!("UNKNOWN_{id}"),
        }
    }

    /// `TagType.StaticSize#size()`, or `None` when the kind is variable-size.
    ///
    /// `EndTag.TYPE` is neither sub-interface, so it answers `None` as well -- see the
    /// module docs.
    pub const fn static_size(&self) -> Option<i32> {
        match self {
            TagTypeRef::Known(1) => Some(1),
            TagTypeRef::Known(2) => Some(2),
            TagTypeRef::Known(3) => Some(4),
            TagTypeRef::Known(4) => Some(8),
            TagTypeRef::Known(5) => Some(4),
            TagTypeRef::Known(6) => Some(8),
            _ => None,
        }
    }

    /// The `IOException` every `createInvalid` method raises.
    fn invalid_exception(id: i32) -> std::io::Error {
        java_io::io_exception(format!("Invalid tag id: {id}"))
    }

    /// Port of `TagType#load(DataInput, NbtAccounter)`.
    pub fn load<I: DataInput + ?Sized>(
        &self,
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<Tag> {
        match self {
            TagTypeRef::Known(0) => {
                accounter.account_bytes(8);
                Ok(Tag::End)
            }
            TagTypeRef::Known(1) => {
                accounter.account_bytes(9);
                Ok(ByteTag::value_of(input.read_byte()?).into_tag())
            }
            TagTypeRef::Known(2) => {
                accounter.account_bytes(10);
                Ok(ShortTag::value_of(input.read_short()?).into_tag())
            }
            TagTypeRef::Known(3) => {
                accounter.account_bytes(12);
                Ok(IntTag::value_of(input.read_int()?).into_tag())
            }
            TagTypeRef::Known(4) => {
                accounter.account_bytes(16);
                Ok(LongTag::value_of(input.read_long()?).into_tag())
            }
            TagTypeRef::Known(5) => {
                accounter.account_bytes(12);
                Ok(FloatTag::value_of(input.read_float()?).into_tag())
            }
            TagTypeRef::Known(6) => {
                accounter.account_bytes(16);
                Ok(DoubleTag::value_of(input.read_double()?).into_tag())
            }
            TagTypeRef::Known(7) => {
                Ok(ByteArrayTag::new(ByteArrayTag::read_accounted(input, accounter)?).into_tag())
            }
            TagTypeRef::Known(8) => {
                accounter.account_bytes(36);
                let data = input.read_utf()?;
                accounter.account_bytes_per_entry(
                    2,
                    crate::net::minecraft::nbt::StringTag::java_length(&data) as i64,
                );
                Ok(StringTag::value_of(data).into_tag())
            }
            TagTypeRef::Known(11) => {
                Ok(IntArrayTag::new(IntArrayTag::read_accounted(input, accounter)?).into_tag())
            }
            TagTypeRef::Known(12) => {
                Ok(LongArrayTag::new(LongArrayTag::read_accounted(input, accounter)?).into_tag())
            }
            TagTypeRef::Known(9) => ListTag::load_type(input, accounter),
            TagTypeRef::Known(10) => CompoundTag::load_type(input, accounter),
            TagTypeRef::Known(other) => Err(Self::invalid_exception(*other as i32)),
            TagTypeRef::Invalid(id) => Err(Self::invalid_exception(*id)),
        }
    }

    /// Port of `TagType#parse(DataInput, StreamTagVisitor, NbtAccounter)`.
    ///
    /// Each kind reads exactly what `load` reads -- through the same private
    /// `readAccounted` -- and hands the value to the visitor instead of building a tag.
    pub fn parse<I: DataInput + ?Sized>(
        &self,
        input: &mut I,
        output: &mut dyn StreamTagVisitor,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<ValueResult> {
        match self {
            TagTypeRef::Known(0) => {
                accounter.account_bytes(8);
                Ok(output.visit_end())
            }
            TagTypeRef::Known(1) => {
                accounter.account_bytes(9);
                Ok(output.visit_byte(input.read_byte()?))
            }
            TagTypeRef::Known(2) => {
                accounter.account_bytes(10);
                Ok(output.visit_short(input.read_short()?))
            }
            TagTypeRef::Known(3) => {
                accounter.account_bytes(12);
                Ok(output.visit_int(input.read_int()?))
            }
            TagTypeRef::Known(4) => {
                accounter.account_bytes(16);
                Ok(output.visit_long(input.read_long()?))
            }
            TagTypeRef::Known(5) => {
                accounter.account_bytes(12);
                Ok(output.visit_float(input.read_float()?))
            }
            TagTypeRef::Known(6) => {
                accounter.account_bytes(16);
                Ok(output.visit_double(input.read_double()?))
            }
            TagTypeRef::Known(7) => {
                let data = ByteArrayTag::read_accounted(input, accounter)?;
                Ok(output.visit_byte_array(&data))
            }
            TagTypeRef::Known(8) => {
                accounter.account_bytes(36);
                let data = input.read_utf()?;
                accounter.account_bytes_per_entry(
                    2,
                    crate::net::minecraft::nbt::StringTag::java_length(&data) as i64,
                );
                Ok(output.visit_string(&data))
            }
            TagTypeRef::Known(11) => {
                let data = IntArrayTag::read_accounted(input, accounter)?;
                Ok(output.visit_int_array(&data))
            }
            TagTypeRef::Known(12) => {
                let data = LongArrayTag::read_accounted(input, accounter)?;
                Ok(output.visit_long_array(&data))
            }
            TagTypeRef::Known(9) => ListTag::parse_type(input, output, accounter),
            TagTypeRef::Known(10) => CompoundTag::parse_type(input, output, accounter),
            TagTypeRef::Known(other) => Err(Self::invalid_exception(*other as i32)),
            TagTypeRef::Invalid(id) => Err(Self::invalid_exception(*id)),
        }
    }

    /// Port of `TagType#skip(DataInput, NbtAccounter)`.
    pub fn skip<I: DataInput + ?Sized>(
        &self,
        input: &mut I,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<()> {
        // `StaticSize.skip` is `input.skipBytes(this.size())`.
        if let Some(size) = self.static_size() {
            input.skip_bytes(size)?;
            return Ok(());
        }
        match self {
            // `EndTag.TYPE.skip` is empty -- not a `StaticSize`, so it falls through here.
            TagTypeRef::Known(0) => Ok(()),
            TagTypeRef::Known(7) => ByteArrayTag::skip(input),
            TagTypeRef::Known(8) => StringTag::skip_string(input),
            TagTypeRef::Known(11) => IntArrayTag::skip(input),
            TagTypeRef::Known(12) => LongArrayTag::skip(input),
            TagTypeRef::Known(9) => ListTag::skip_type(input, accounter),
            TagTypeRef::Known(10) => CompoundTag::skip_type(input, accounter),
            // Every static-size kind returned above, so a `Known` reaching here is an id
            // outside 0..=12 -- which `TagTypes::get_type` would have made `Invalid`.
            TagTypeRef::Known(other) => Err(Self::invalid_exception(*other as i32)),
            TagTypeRef::Invalid(id) => Err(Self::invalid_exception(*id)),
        }
    }

    /// Port of `TagType#skip(DataInput, int, NbtAccounter)`.
    ///
    /// `StaticSize` does one `skipBytes(size * count)` with a **wrapping** `int` multiply;
    /// `VariableSize` loops. See the module docs.
    pub fn skip_count<I: DataInput + ?Sized>(
        &self,
        input: &mut I,
        count: i32,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<()> {
        if let Some(size) = self.static_size() {
            input.skip_bytes(size.wrapping_mul(count))?;
            return Ok(());
        }
        match self {
            // Both of `EndTag.TYPE`'s skips are empty.
            TagTypeRef::Known(0) => Ok(()),
            TagTypeRef::Invalid(id) => Err(Self::invalid_exception(*id)),
            _ => {
                // `VariableSize.skip(input, count, accounter)`.
                for _ in 0..count {
                    self.skip(input, accounter)?;
                }
                Ok(())
            }
        }
    }

    /// Port of `TagType#parseRoot(DataInput, StreamTagVisitor, NbtAccounter)`.
    ///
    /// Note the Java `switch` **falls through** from `CONTINUE` into `HALT`/`default`:
    ///
    /// ```java
    /// switch (output.visitRootEntry(this)) {
    ///     case CONTINUE: this.parse(input, output, accounter);
    ///     case HALT: default: break;
    ///     case BREAK: this.skip(input, accounter);
    /// }
    /// ```
    ///
    /// The fall-through is harmless -- the next label is a `break` -- so this reads as three
    /// independent arms, which is how it is written here.
    pub fn parse_root<I: DataInput + ?Sized>(
        &self,
        input: &mut I,
        output: &mut dyn StreamTagVisitor,
        accounter: &mut NbtAccounter,
    ) -> std::io::Result<()> {
        match output.visit_root_entry(self) {
            ValueResult::Continue => {
                self.parse(input, output, accounter)?;
                Ok(())
            }
            ValueResult::Break => self.skip(input, accounter),
            ValueResult::Halt => Ok(()),
        }
    }
}
