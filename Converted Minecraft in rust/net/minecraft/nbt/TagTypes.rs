//! Port of: net/minecraft/nbt/TagTypes.java
//! Java class(es): net.minecraft.nbt.TagTypes
//! Status: VERIFIED
//!
//! Session 21. The `TYPES` table and `getType(int)`. Verified against the `TT` and `TTEX`
//! rows of `_porting/test-data/nbt_probe.txt` by `_porting/tests/session19_g.rs`, which
//! checks `getName`, `getPrettyName` and `size()` for ids `-2 ..= 14`.
//!
//! # THE TABLE ORDER IS THE WIRE FORMAT
//!
//! ```java
//! private static final TagType<?>[] TYPES = new TagType[]{
//!     EndTag.TYPE, ByteTag.TYPE, ShortTag.TYPE, IntTag.TYPE, LongTag.TYPE,
//!     FloatTag.TYPE, DoubleTag.TYPE, ByteArrayTag.TYPE, StringTag.TYPE,
//!     ListTag.TYPE, CompoundTag.TYPE, IntArrayTag.TYPE, LongArrayTag.TYPE
//! };
//! ```
//!
//! The index **is** the tag id that appears in every NBT stream, so the order is not a
//! style choice -- swapping two entries would silently reinterpret every saved world.
//! `ByteArrayTag` at 7 and `StringTag` at 8 is the pair worth double-checking, because the
//! alphabetical instinct puts them the other way round.
//!
//! The port does not keep a table: [`TagTypeRef`] already encodes the id, so `getType` is
//! the bounds check and nothing else. The ordering above lives in `TagType`'s `get_name` /
//! `get_pretty_name` / `static_size` match arms, and the `TT` oracle rows are what hold it
//! in place.
//!
//! # `getType` IS THE ONLY PLACE AN ID IS VALIDATED
//!
//! `typeId >= 0 && typeId < TYPES.length ? TYPES[typeId] : TagType.createInvalid(typeId)`
//! -- so a negative or out-of-range id does not throw here. It returns a type whose every
//! data method throws `IOException("Invalid tag id: <id>")` when used. The `TT` rows cover
//! `-2`, `-1`, `13`, `14`, and the `TTEX` row covers id `77` actually being loaded.

use crate::net::minecraft::nbt::TagType::TagTypeRef;

/// Port of the private `TagTypes#TYPES`' length -- the number of valid tag ids.
const TYPES_LENGTH: i32 = 13;

/// Port of `TagTypes#getType(int)`.
///
/// Out-of-range ids yield `TagType.createInvalid(typeId)` rather than an error; see the
/// module docs.
pub fn get_type(type_id: i32) -> TagTypeRef {
    if type_id >= 0 && type_id < TYPES_LENGTH {
        TagTypeRef::Known(type_id as i8)
    } else {
        TagTypeRef::Invalid(type_id)
    }
}
