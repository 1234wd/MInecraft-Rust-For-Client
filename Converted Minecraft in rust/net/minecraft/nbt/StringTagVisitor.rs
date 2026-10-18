//! Port of: net/minecraft/nbt/StringTagVisitor.java
//! Java class(es): net.minecraft.nbt.StringTagVisitor
//! Status: VERIFIED
//!
//! Session 21 ported eleven of the thirteen `visitX` methods plus `build()` and the private
//! `handleKeyEscape`; session 23 added `visitList` and `visitCompound` with `ListTag` and
//! `CompoundTag`. Verified against the `PRIM`, `QUOTE` and `OTHER` rows of
//! `_porting/test-data/nbt_probe.txt` by `_porting/tests/session19_g.rs`, and against the
//! `toString` of every `GEN`/`CASE`/`CASEW`/`CTOPS`/`LTOPS` tree by `_porting/tests/session23.rs`.
//!
//! # `visitCompound` SORTS THE KEYS -- BY `String.compareTo`
//!
//! `entries.sort(Entry.comparingByKey())`: unlike `CompoundTag.write`, the text form does NOT
//! follow `HashMap` order, so `toString` is the same for two compounds with the same entries
//! whatever their history. The comparison is `String.compareTo`, over UTF-16 code units --
//! which is not Rust's `str` ordering (UTF-8 bytes) once a key has a character at or above
//! U+E000 next to a supplementary one -- so the sort uses `java_lang::string_compare_to`.
//!
//! This is the SNBT writer behind every `Tag.toString()`, so it is the thing that decides
//! what a tag looks like in a crash log, in `/data get` output and in a command's error.
//!
//! # THE TYPE SUFFIXES ARE NOT UNIFORM
//!
//! `b s L f d` for byte/short/long/float/double, and **nothing** for int. Inside the array
//! forms the suffixes change again: `[B;1B,2B]` uses an upper-case `B` where the scalar
//! form uses lower-case `b`, `[L;1L]` keeps `L`, and `[I;1,2]` has no element suffix at
//! all. Transcribed per method rather than factored, because the asymmetry is the content.
//!
//! | tag | scalar | array element |
//! |---|---|---|
//! | byte | `1b` | `[B;1B,2B]` |
//! | short | `1s` | -- |
//! | int | `1` | `[I;1,2]` |
//! | long | `1L` | `[L;1L,2L]` |
//! | float | `1.0f` | -- |
//! | double | `1.0d` | -- |
//!
//! # `visitFloat` / `visitDouble` APPEND A RAW `float`, SO THEY NEED JAVA'S FORMATTER
//!
//! `this.builder.append(tag.value())` on a `float` is `StringBuilder.append(float)`, which
//! is `Float.toString`. That is the one place in this batch where the text depends on
//! `javacompat::java_lang::float_to_string` rather than on anything in `nbt/`.
//!
//! That dependency used to be a liability: the formatter missed Java's two-digit minimum
//! and its round-half-to-even tie rule, so `FloatTag.toString()` would have been wrong for
//! nine float subnormals and for 0.04% of all values. Both were fixed in session 21 (it was
//! never the `FloatingDecimal` port that OPEN_QUESTIONS #18 had assumed), verified against
//! JDK 21 over 11,223,706 values, so all thirteen visit methods are exact.
//!
//! # `visitEnd` WRITES `END`, WHICH IS NOT VALID SNBT
//!
//! `builder.append("END")` -- no quotes, no suffix. It cannot be parsed back, and that is
//! fine: an `EndTag` only reaches `toString` through a malformed tree or a direct call,
//! never through a well-formed compound. Pinned by the `CASEW end_value` oracle row rather
//! than assumed.

use crate::javacompat::java_lang;
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
use crate::net::minecraft::nbt::NumericTag::NumericTag;
use crate::net::minecraft::nbt::ShortTag::ShortTag;
use crate::net::minecraft::nbt::StringTag;
use crate::net::minecraft::nbt::StringTag::StringTag as StringTagValue;
use crate::net::minecraft::nbt::TagVisitor::TagVisitor;

/// Port of `net.minecraft.nbt.StringTagVisitor`.
#[derive(Clone, Debug, Default)]
pub struct StringTagVisitor {
    builder: String,
}

impl StringTagVisitor {
    /// Port of the implicit no-arg constructor.
    pub fn new() -> Self {
        Self {
            builder: String::new(),
        }
    }

    /// Port of `StringTagVisitor#build()`.
    pub fn build(&self) -> String {
        self.builder.clone()
    }

    /// Port of the private `StringTagVisitor#handleKeyEscape(String)`.
    ///
    /// A compound key is written bare when it matches `[A-Za-z._]+[A-Za-z0-9._+-]*` **and**
    /// is not `true`/`false` in any case mix; otherwise it is quoted. Ported now because it
    /// has no dependency on `CompoundTag`, and because the `UNQUOTED_KEY_MATCH` regex is the
    /// kind of thing that is cheaper to get right once than to revisit.
    ///
    /// The regex is reproduced by hand rather than with a regex engine (the crate has no
    /// dependencies): first character in `[A-Za-z._]`, every later character in
    /// `[A-Za-z0-9._+-]`, and at least one character. Note the first class excludes digits
    /// and `+`/`-`, so `1a` and `+a` are quoted while `a1` and `a+` are not.
    pub fn handle_key_escape(&mut self, input: &str) {
        if !input.eq_ignore_ascii_case("true")
            && !input.eq_ignore_ascii_case("false")
            && unquoted_key_match(input)
        {
            self.builder.push_str(input);
        } else {
            StringTag::quote_and_escape_into(input, &mut self.builder);
        }
    }
}

/// `StringTagVisitor#UNQUOTED_KEY_MATCH`: `[A-Za-z._]+[A-Za-z0-9._+-]*`, anchored by
/// `Matcher#matches`.
fn unquoted_key_match(input: &str) -> bool {
    let mut chars = input.chars();
    // `[A-Za-z._]+` needs at least one.
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '.' || first == '_') {
        return false;
    }
    // The `+` is greedy and the second class is a superset of the first, so a hand-rolled
    // matcher only has to check that every remaining character is in the second class.
    chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '+' | '-'))
}

impl TagVisitor for StringTagVisitor {
    /// Port of `StringTagVisitor#visitString(StringTag)`.
    fn visit_string(&mut self, tag: &StringTagValue) {
        self.builder
            .push_str(&StringTag::quote_and_escape(tag.value()));
    }

    /// Port of `StringTagVisitor#visitByte(ByteTag)`.
    fn visit_byte(&mut self, tag: &ByteTag) {
        self.builder.push_str(&tag.value().to_string());
        self.builder.push('b');
    }

    /// Port of `StringTagVisitor#visitShort(ShortTag)`.
    fn visit_short(&mut self, tag: &ShortTag) {
        self.builder.push_str(&tag.value().to_string());
        self.builder.push('s');
    }

    /// Port of `StringTagVisitor#visitInt(IntTag)`.
    ///
    /// No suffix -- int is SNBT's default integral type.
    fn visit_int(&mut self, tag: &IntTag) {
        self.builder.push_str(&tag.value().to_string());
    }

    /// Port of `StringTagVisitor#visitLong(LongTag)`.
    fn visit_long(&mut self, tag: &LongTag) {
        self.builder.push_str(&tag.value().to_string());
        self.builder.push('L');
    }

    /// Port of `StringTagVisitor#visitFloat(FloatTag)`.
    ///
    /// `StringBuilder.append(float)` is `Float.toString` -- see the module docs.
    fn visit_float(&mut self, tag: &FloatTag) {
        self.builder.push_str(&tag.box_to_string());
        self.builder.push('f');
    }

    /// Port of `StringTagVisitor#visitDouble(DoubleTag)`.
    fn visit_double(&mut self, tag: &DoubleTag) {
        self.builder.push_str(&tag.box_to_string());
        self.builder.push('d');
    }

    /// Port of `StringTagVisitor#visitByteArray(ByteArrayTag)`.
    fn visit_byte_array(&mut self, tag: &ByteArrayTag) {
        self.builder.push_str("[B;");
        let data = tag.get_as_byte_array();
        for (i, b) in data.iter().enumerate() {
            if i != 0 {
                self.builder.push(',');
            }
            self.builder.push_str(&b.to_string());
            self.builder.push('B');
        }
        self.builder.push(']');
    }

    /// Port of `StringTagVisitor#visitIntArray(IntArrayTag)`.
    fn visit_int_array(&mut self, tag: &IntArrayTag) {
        self.builder.push_str("[I;");
        let data = tag.get_as_int_array();
        for (i, v) in data.iter().enumerate() {
            if i != 0 {
                self.builder.push(',');
            }
            self.builder.push_str(&v.to_string());
        }
        self.builder.push(']');
    }

    /// Port of `StringTagVisitor#visitLongArray(LongArrayTag)`.
    fn visit_long_array(&mut self, tag: &LongArrayTag) {
        self.builder.push_str("[L;");
        let data = tag.get_as_long_array();
        for (i, v) in data.iter().enumerate() {
            if i != 0 {
                self.builder.push(',');
            }
            self.builder.push_str(&v.to_string());
            self.builder.push('L');
        }
        self.builder.push(']');
    }

    /// Port of `StringTagVisitor#visitList(ListTag)`: `[a,b,c]`, elements in list order.
    fn visit_list(&mut self, tag: &ListTag) {
        self.builder.push('[');
        for i in 0..tag.size() {
            if i != 0 {
                self.builder.push(',');
            }
            tag.get(i).accept(self);
        }
        self.builder.push(']');
    }

    /// Port of `StringTagVisitor#visitCompound(CompoundTag)`: `{k:v,...}` with the entries
    /// sorted by key (`Entry.comparingByKey()`, i.e. `String.compareTo`) and each key through
    /// [`handle_key_escape`](StringTagVisitor::handle_key_escape).
    fn visit_compound(&mut self, tag: &CompoundTag) {
        self.builder.push('{');
        let mut entries: Vec<(&String, &crate::net::minecraft::nbt::Tag::Tag)> =
            tag.entry_set().collect();
        // A stable sort, as `List.sort` is; the keys are distinct, so stability cannot show.
        entries.sort_by(|a, b| java_lang::string_compare_to(a.0, b.0).cmp(&0));
        for (i, (key, value)) in entries.into_iter().enumerate() {
            if i != 0 {
                self.builder.push(',');
            }
            self.handle_key_escape(key);
            self.builder.push(':');
            value.accept(self);
        }
        self.builder.push('}');
    }

    /// Port of `StringTagVisitor#visitEnd(EndTag)`.
    fn visit_end(&mut self) {
        self.builder.push_str("END");
    }
}
