//! Port of: net/minecraft/nbt/StringTag.java
//! Java class(es): net.minecraft.nbt.StringTag, StringTag::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 21. Every member: `TYPE` (load/parse/skip/names), `skipString`, `valueOf`,
//! `write`, `sizeInBytes`, `getId`, `getType`, `toString`, `copy`, `asString`, both
//! `accept` overloads, and all four `quoteAndEscape`/`escapeWithoutQuotes` entry points.
//! Verified against the `QUOTE` rows of `_porting/test-data/nbt_probe.txt` (JDK 25.0.4.1,
//! 26.2 jar) by `_porting/tests/session19_g.rs`.
//!
//! # `quoteAndEscape` PICKS ITS QUOTE CHARACTER FROM THE CONTENT
//!
//! The algorithm is not "wrap in double quotes and escape". It reserves a slot, scans, and
//! back-patches:
//!
//! ```java
//! int quoteMarkIndex = result.length();
//! result.append(' ');            // placeholder, overwritten at the end
//! char quote = 0;
//! ...
//! result.setCharAt(quoteMarkIndex, quote);
//! ```
//!
//! The first `"` or `'` in the input *chooses the other one* as the delimiter
//! (`quote = c == '"' ? 39 : 34`), and only a later occurrence of the chosen delimiter gets
//! a backslash. So:
//!
//! | input | output |
//! |---|---|
//! | `it's` | `"it's"` -- apostrophe picks `"`, nothing escaped |
//! | `say "hi"` | `'say "hi"'` -- quote picks `'`, nothing escaped |
//! | `both ' and "` | `"both ' and \""` -- the `'` came first, so `"` is the delimiter and the `"` is escaped |
//!
//! A port that always used `"` would produce `"it's"` correctly and `"say \"hi\""` wrongly.
//! Every row above is an oracle row.
//!
//! Because the placeholder is written *into the caller's buffer* before the scan, the
//! two-argument form appends to a shared `StringBuilder` and records an absolute index into
//! it -- which is why [`quote_and_escape_into`] takes the buffer and
//! [`quote_and_escape`] is the thin wrapper, matching Java's pair exactly.
//!
//! # CONTROL CHARACTERS GO THROUGH `SnbtGrammar`
//!
//! `\b \t \n \f \r` become two-character escapes; anything else below `0x20` becomes
//! `\xNN` with **upper-case** hex. `0x7F` (DEL) is *not* escaped -- the test is `c < ' '`,
//! so the boundary is `0x20`, not `0x7E`. Same boundary trap as
//! `IdentifierException::escape_java`, and the `QUOTE` rows pin `\u007f` passing through raw.
//!
//! # `length()` IS UTF-16 CODE UNITS
//!
//! `sizeInBytes()` is `36 + 2 * this.value.length()`, and Java's `String.length()` counts
//! UTF-16 code units -- so an emoji costs 4, not 1 and not its UTF-8 length. `TYPE.load`
//! charges `accountBytes(2L, data.length())` for the same reason. [`java_length`] does the
//! counting; using Rust's `str::len()` (UTF-8 bytes) would under-charge ASCII-free strings
//! and over-charge nothing, and using `chars().count()` would halve the cost of every
//! supplementary character.

use crate::javacompat::java_io::{DataInput, DataOutput};
use crate::net::minecraft::nbt::PrimitiveTag::PrimitiveTag;
use crate::net::minecraft::nbt::SnbtGrammar;
use crate::net::minecraft::nbt::StreamTagVisitor::{StreamTagVisitor, ValueResult};
use crate::net::minecraft::nbt::Tag::Tag;
use crate::net::minecraft::nbt::TagType::TagTypeRef;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;

/// Port of the private `StringTag#SELF_SIZE_IN_BYTES`.
const SELF_SIZE_IN_BYTES: i32 = 36;

/// Port of `StringTag#TYPE`.
pub const TYPE: TagTypeRef = TagTypeRef::Known(8);

/// Port of the private `StringTag#DOUBLE_QUOTE`.
const DOUBLE_QUOTE: char = '"';
/// Port of the private `StringTag#SINGLE_QUOTE`.
const SINGLE_QUOTE: char = '\'';
/// Port of the private `StringTag#ESCAPE`.
const ESCAPE: char = '\\';
/// Port of the private `StringTag#NOT_SET`.
const NOT_SET: char = '\u{0000}';

/// `String.length()` -- the number of UTF-16 code units.
///
/// See the module docs: this is what `sizeInBytes` and the accounter charge for.
pub fn java_length(s: &str) -> i32 {
    s.chars().map(|c| c.len_utf16() as i32).sum()
}

/// Port of the `record net.minecraft.nbt.StringTag(String value)`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StringTag {
    value: String,
}

impl StringTag {
    /// Port of the canonical constructor `StringTag(String)`, which vanilla marks
    /// `@Deprecated(forRemoval = true)` in favour of [`value_of`](Self::value_of).
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into() }
    }

    /// Port of the record accessor `StringTag#value()`.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Port of `StringTag#valueOf(String)`.
    ///
    /// Java returns the shared `EMPTY` instance for `""`. That is unobservable here (see
    /// `ByteTag`'s note on caches), so this is `new` with the branch kept for the reader.
    pub fn value_of(data: impl Into<String>) -> Self {
        let data = data.into();
        if data.is_empty() {
            Self::new("")
        } else {
            Self::new(data)
        }
    }

    /// Port of `StringTag#skipString(DataInput)`.
    ///
    /// `input.skipBytes(input.readUnsignedShort())` -- the modified-UTF-8 byte count, not a
    /// character count.
    pub fn skip_string<I: DataInput + ?Sized>(input: &mut I) -> std::io::Result<()> {
        let len = input.read_unsigned_short()?;
        input.skip_bytes(len)?;
        Ok(())
    }

    /// Port of `StringTag#write(DataOutput)`.
    pub fn write<O: DataOutput + ?Sized>(&self, output: &mut O) -> std::io::Result<()> {
        output.write_utf(&self.value)
    }

    /// Port of `StringTag#sizeInBytes()`.
    ///
    /// `36 + 2 * length()`, in Java `int` arithmetic -- so it wraps for a string longer
    /// than ~1G code units rather than widening.
    pub fn size_in_bytes(&self) -> i32 {
        SELF_SIZE_IN_BYTES.wrapping_add(2i32.wrapping_mul(java_length(&self.value)))
    }

    /// Port of `StringTag#getId()`.
    pub const fn get_id(&self) -> i8 {
        8
    }

    /// Port of `StringTag#getType()`.
    pub const fn get_type(&self) -> TagTypeRef {
        TYPE
    }

    /// Port of `StringTag#copy()`.
    ///
    /// Returns `this` in Java -- the record is immutable.
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// Port of `StringTag#asString()`.
    pub fn as_string(&self) -> Option<&str> {
        Some(&self.value)
    }

    /// Port of `StringTag#accept(TagVisitor)`.
    pub fn accept(&self, visitor: &mut dyn TagVisitor) {
        visitor.visit_string(self);
    }

    /// Port of `StringTag#accept(StreamTagVisitor)`.
    pub fn accept_stream(&self, visitor: &mut dyn StreamTagVisitor) -> ValueResult {
        visitor.visit_string(&self.value)
    }

    /// `StringTag#hashCode()` -- the record's, i.e. `String.hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_string_hash_code(&self.value)
    }

    /// Wraps this tag as a [`Tag`].
    pub fn into_tag(self) -> Tag {
        Tag::String(self)
    }
}

/// `String.hashCode()`: `s[0]*31^(n-1) + s[1]*31^(n-2) + ... + s[n-1]`, over UTF-16 code
/// units and in wrapping `int` arithmetic.
///
/// Not in `StringTag.java`, but it is what the record's `hashCode` resolves to, and
/// `CompoundTag`'s map will need it for bucket order.
pub fn java_string_hash_code(s: &str) -> i32 {
    let mut h: i32 = 0;
    let mut buf = [0u16; 2];
    for c in s.chars() {
        for unit in c.encode_utf16(&mut buf) {
            h = h.wrapping_mul(31).wrapping_add(*unit as i32);
        }
    }
    h
}

/// Port of `StringTag#quoteAndEscape(String)`.
pub fn quote_and_escape(input: &str) -> String {
    let mut result = String::new();
    quote_and_escape_into(input, &mut result);
    result
}

/// Port of `StringTag#quoteAndEscape(String, StringBuilder)`.
///
/// Appends to `result`, reserving one slot for the delimiter and back-patching it once the
/// scan has decided which quote to use. See the module docs.
pub fn quote_and_escape_into(input: &str, result: &mut String) {
    // Java records `result.length()` -- a UTF-16 index -- then overwrites that one char at
    // the end. Rust strings are UTF-8 and not randomly writable, so the port remembers the
    // BYTE offset of the placeholder instead. The placeholder is ASCII and so is every
    // possible delimiter, so the slot is exactly one byte wide either way.
    let quote_mark_index = result.len();
    result.push(' ');
    let mut quote = NOT_SET;

    for c in input.chars() {
        if c == ESCAPE {
            result.push_str("\\\\");
        } else if c != DOUBLE_QUOTE && c != SINGLE_QUOTE {
            match SnbtGrammar::escape_control_characters(c) {
                Some(escaped) => {
                    result.push(ESCAPE);
                    result.push_str(&escaped);
                }
                None => result.push(c),
            }
        } else {
            if quote == NOT_SET {
                // `quote = (char)(c == '"' ? 39 : 34)` -- the OTHER quote character.
                quote = if c == DOUBLE_QUOTE { SINGLE_QUOTE } else { DOUBLE_QUOTE };
            }

            if quote == c {
                result.push(ESCAPE);
            }

            result.push(c);
        }
    }

    if quote == NOT_SET {
        quote = DOUBLE_QUOTE;
    }

    // `result.setCharAt(quoteMarkIndex, quote)`.
    result.replace_range(quote_mark_index..quote_mark_index + 1, &quote.to_string());
    result.push(quote);
}

/// Port of `StringTag#escapeWithoutQuotes(String)`.
pub fn escape_without_quotes(input: &str) -> String {
    let mut result = String::new();
    escape_without_quotes_into(input, &mut result);
    result
}

/// Port of `StringTag#escapeWithoutQuotes(String, StringBuilder)`.
///
/// Unlike [`quote_and_escape_into`] this escapes **both** quote characters unconditionally,
/// because there is no delimiter to disambiguate them.
pub fn escape_without_quotes_into(input: &str, result: &mut String) {
    for c in input.chars() {
        match c {
            DOUBLE_QUOTE | SINGLE_QUOTE | ESCAPE => {
                result.push(ESCAPE);
                result.push(c);
            }
            _ => match SnbtGrammar::escape_control_characters(c) {
                Some(escaped) => {
                    result.push(ESCAPE);
                    result.push_str(&escaped);
                }
                None => result.push(c),
            },
        }
    }
}

impl PrimitiveTag for StringTag {}

/// Port of `StringTag#toString()`.
impl std::fmt::Display for StringTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut visitor = crate::net::minecraft::nbt::StringTagVisitor::StringTagVisitor::new();
        visitor.visit_string(self);
        f.write_str(&visitor.build())
    }
}
