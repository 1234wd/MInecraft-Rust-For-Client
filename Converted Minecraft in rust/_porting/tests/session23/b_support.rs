//! Shared helpers for the container tests: the probe's tag literal (`lit`) both ways, its
//! `TraceVisitor`, its exception spelling, and the three lines of `NbtIo` framing the tree rows
//! go through.
//!
//! `NbtIo` itself is not ported (and is not this session's work). Its `write(CompoundTag)`,
//! `read(DataInput, NbtAccounter)` and `parse(...)` are, for the inputs these rows use, a root
//! header around `CompoundTag`'s own `write`/`TYPE.load`/`TYPE.parse` -- that header is all that
//! is mirrored here, so every byte of payload is produced and consumed by the port.

#![allow(dead_code)]

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::nbt_probe::*;

use minecraft_rust::javacompat::java_io::{self, DataInput, DataInputStream, DataOutputStream};
use minecraft_rust::javacompat::java_lang;
use minecraft_rust::javacompat::java_random::JavaRandom;
use minecraft_rust::net::minecraft::nbt::ByteArrayTag::ByteArrayTag;
use minecraft_rust::net::minecraft::nbt::ByteTag::ByteTag;
use minecraft_rust::net::minecraft::nbt::CompoundTag::CompoundTag;
use minecraft_rust::net::minecraft::nbt::DoubleTag::DoubleTag;
use minecraft_rust::net::minecraft::nbt::FloatTag::FloatTag;
use minecraft_rust::net::minecraft::nbt::IntArrayTag::IntArrayTag;
use minecraft_rust::net::minecraft::nbt::IntTag::IntTag;
use minecraft_rust::net::minecraft::nbt::ListTag::ListTag;
use minecraft_rust::net::minecraft::nbt::LongArrayTag::LongArrayTag;
use minecraft_rust::net::minecraft::nbt::LongTag::LongTag;
use minecraft_rust::net::minecraft::nbt::NbtAccounter::NbtAccounter;
use minecraft_rust::net::minecraft::nbt::ShortTag::ShortTag;
use minecraft_rust::net::minecraft::nbt::StreamTagVisitor::{
    EntryResult, StreamTagVisitor, ValueResult,
};
use minecraft_rust::net::minecraft::nbt::StringTag::StringTag;
use minecraft_rust::net::minecraft::nbt::Tag::Tag;
use minecraft_rust::net::minecraft::nbt::TagType::TagTypeRef;
use minecraft_rust::net::minecraft::nbt::TagTypes;

// ------------------------------------------------------------------ literals

/// The probe's `qesc`: `esc`, a `"` written as `"`, in double quotes.
pub fn qesc(s: &str) -> String {
    format!("\"{}\"", esc(s).replace('"', "\\u0022"))
}

fn join<T: ToString>(xs: &[T]) -> String {
    xs.iter().map(T::to_string).collect::<Vec<_>>().join(",")
}

/// The probe's `lit(Tag)`. A compound prints in ITERATION order, so this checks order too.
pub fn lit(t: &Tag) -> String {
    match t {
        Tag::End => "E".to_string(),
        Tag::Byte(b) => format!("b{}", b.value()),
        Tag::Short(s) => format!("s{}", s.value()),
        Tag::Int(i) => format!("i{}", i.value()),
        Tag::Long(l) => format!("l{}", l.value()),
        Tag::Float(f) => format!("f{:x}", f.value().to_bits()),
        Tag::Double(d) => format!("d{:x}", d.value().to_bits()),
        Tag::String(s) => format!("S{}", qesc(s.value())),
        Tag::ByteArray(a) => format!("B({})", join(a.get_as_byte_array())),
        Tag::IntArray(a) => format!("I({})", join(a.get_as_int_array())),
        Tag::LongArray(a) => format!("L({})", join(a.get_as_long_array())),
        Tag::List(l) => format!("[{}]", l.iter().map(lit).collect::<Vec<_>>().join(",")),
        Tag::Compound(c) => format!(
            "{{{}}}",
            c.entry_set()
                .map(|(k, v)| format!("{}:{}", qesc(k), lit(v)))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

/// Parses one literal. Compound entries are `put` in the order written, which for these small
/// maps reproduces the order they were printed in; lists use `add`.
pub fn parse_lit(s: &str) -> Tag {
    let mut p = LitParser {
        chars: s.chars().collect(),
        at: 0,
    };
    let t = p.tag();
    assert_eq!(p.at, p.chars.len(), "trailing text in literal {s:?}");
    t
}

/// A quoted string from a literal: `"..."` in the probe's `esc`.
pub fn parse_qstring(s: &str) -> String {
    let mut p = LitParser {
        chars: s.chars().collect(),
        at: 0,
    };
    let q = p.qstring();
    assert_eq!(p.at, p.chars.len(), "trailing text after {s:?}");
    q
}

struct LitParser {
    chars: Vec<char>,
    at: usize,
}

impl LitParser {
    fn peek(&self) -> char {
        self.chars[self.at]
    }

    fn bump(&mut self) -> char {
        let c = self.chars[self.at];
        self.at += 1;
        c
    }

    fn expect(&mut self, c: char) {
        assert_eq!(self.bump(), c, "literal: expected {c:?} at {}", self.at - 1);
    }

    /// Up to (not including) the next `,` `)` `]` `}` or end.
    fn word(&mut self) -> String {
        let start = self.at;
        while self.at < self.chars.len() && !matches!(self.chars[self.at], ',' | ')' | ']' | '}') {
            self.at += 1;
        }
        self.chars[start..self.at].iter().collect()
    }

    fn qstring(&mut self) -> String {
        self.expect('"');
        let start = self.at;
        while self.peek() != '"' {
            self.at += 1;
        }
        let raw: String = self.chars[start..self.at].iter().collect();
        self.expect('"');
        s(&raw)
    }

    fn numbers<T: std::str::FromStr>(&mut self) -> Vec<T> {
        self.expect('(');
        let mut out = Vec::new();
        while self.peek() != ')' {
            let w = self.word();
            out.push(
                w.parse()
                    .unwrap_or_else(|_| panic!("literal: bad number {w:?}")),
            );
            if self.peek() == ',' {
                self.bump();
            }
        }
        self.expect(')');
        out
    }

    fn tag(&mut self) -> Tag {
        match self.bump() {
            'E' => Tag::End,
            'b' => ByteTag::value_of(self.word().parse().unwrap()).into_tag(),
            's' => ShortTag::value_of(self.word().parse().unwrap()).into_tag(),
            'i' => IntTag::value_of(self.word().parse().unwrap()).into_tag(),
            'l' => LongTag::value_of(self.word().parse().unwrap()).into_tag(),
            // `new`, not `valueOf`: the literal records the stored bits, -0.0 and NaN payloads too.
            'f' => FloatTag::new(f32::from_bits(
                u32::from_str_radix(&self.word(), 16).unwrap(),
            ))
            .into_tag(),
            'd' => DoubleTag::new(f64::from_bits(
                u64::from_str_radix(&self.word(), 16).unwrap(),
            ))
            .into_tag(),
            'S' => StringTag::value_of(self.qstring()).into_tag(),
            'B' => ByteArrayTag::new(self.numbers()).into_tag(),
            'I' => IntArrayTag::new(self.numbers()).into_tag(),
            'L' => LongArrayTag::new(self.numbers()).into_tag(),
            '[' => {
                let mut l = ListTag::new();
                while self.peek() != ']' {
                    l.add(self.tag());
                    if self.peek() == ',' {
                        self.bump();
                    }
                }
                self.expect(']');
                l.into_tag()
            }
            '{' => {
                let mut c = CompoundTag::new();
                while self.peek() != '}' {
                    let k = self.qstring();
                    self.expect(':');
                    let v = self.tag();
                    c.put(&k, v);
                    if self.peek() == ',' {
                        self.bump();
                    }
                }
                self.expect('}');
                c.into_tag()
            }
            other => panic!("literal: unexpected {other:?} at {}", self.at - 1),
        }
    }
}

// ------------------------------------------------------------------ bytes

/// `tag.write(new DataOutputStream(bos))`, as hex.
pub fn write_hex(t: &Tag) -> String {
    let mut out = DataOutputStream::new(Vec::new());
    t.write(&mut out).expect("write");
    hex(&out.into_inner())
}

/// `NbtIo.write(CompoundTag, DataOutput)`: `writeUnnamedTag` -- the id, an empty name, the
/// payload. (Its string fallback only matters for strings over 65535 bytes; see the callers.)
pub fn nbt_write(c: &CompoundTag) -> Vec<u8> {
    let mut out = DataOutputStream::new(Vec::new());
    use minecraft_rust::javacompat::java_io::DataOutput;
    out.write_byte(10).unwrap();
    out.write_utf("").unwrap();
    c.write(&mut out).expect("write");
    out.into_inner()
}

/// `NbtIo.read(DataInput, NbtAccounter)` for a compound root: `readUnnamedTag` reads the id,
/// skips the name and loads the payload through `TagTypes.getType(id).load`.
pub fn nbt_read(bytes: &[u8], acc: &mut NbtAccounter) -> Tag {
    let mut input = DataInputStream::new(bytes);
    let id = input.read_byte().unwrap();
    StringTag::skip_string(&mut input).unwrap();
    TagTypes::get_type(id as i32)
        .load(&mut input, acc)
        .expect("load")
}

/// `NbtIo.parse(DataInput, StreamTagVisitor, NbtAccounter)` for a non-`END` root.
pub fn nbt_parse(
    bytes: &[u8],
    visitor: &mut dyn StreamTagVisitor,
    acc: &mut NbtAccounter,
) -> std::io::Result<()> {
    let mut input = DataInputStream::new(bytes);
    let ty = TagTypes::get_type(input.read_byte()? as i32);
    match visitor.visit_root_entry(&ty) {
        ValueResult::Halt => Ok(()),
        ValueResult::Break => {
            StringTag::skip_string(&mut input)?;
            ty.skip(&mut input, acc)
        }
        ValueResult::Continue => {
            StringTag::skip_string(&mut input)?;
            ty.parse(&mut input, visitor, acc).map(|_| ())
        }
    }
}

// ------------------------------------------------------------------ exceptions

/// A panic payload as text.
pub fn panic_text(e: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = e.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = e.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "<non-string panic>".to_string()
    }
}

/// Runs `f`; `Ok`, or the panic's text when the port raised a Java exception.
pub fn catch<R>(f: impl FnOnce() -> R) -> Result<R, String> {
    catch_unwind(AssertUnwindSafe(f)).map_err(panic_text)
}

/// The probe's `exc(Throwable)` -- `EX`, class, message, cause class, cause message -- for a
/// panic this port raised (`class: message`, optionally `\nCaused by: class: message`).
pub fn exc_from_panic(text: &str) -> Vec<String> {
    let (head, cause) = match text.split_once("\nCaused by: ") {
        Some((h, c)) => (h, Some(c)),
        None => (text, None),
    };
    let split = |t: &str| -> (String, String) {
        match t.split_once(": ") {
            Some((class, msg)) => (class.to_string(), esc(msg)),
            None => (t.to_string(), "\\N".to_string()),
        }
    };
    let (class, msg) = split(head);
    let (cc, cm) = match cause {
        Some(c) => split(c),
        None => ("\\N".to_string(), "\\N".to_string()),
    };
    vec!["EX".to_string(), class, msg, cc, cm]
}

/// The probe's `exc(Throwable)` for an `IOException` this port returned.
pub fn exc_from_io(e: &std::io::Error) -> Vec<String> {
    vec![
        "EX".to_string(),
        java_io::java_class_name(e).to_string(),
        java_io::java_message(e).map_or("\\N".to_string(), |m| esc(&m)),
        "\\N".to_string(),
        "\\N".to_string(),
    ]
}

/// The probe's `exText`: `EX:` + class + `:` + message, for an edit that panicked.
pub fn ex_text(panic: &str) -> String {
    let f = exc_from_panic(panic);
    format!("EX:{}:{}", f[1], f[2])
}

// ------------------------------------------------------------------ the trace visitor

/// The probe's `TraceVisitor`: logs every call; answers `CONTINUE`/`ENTER`, or pseudo-randomly
/// from a `java.util.Random` when one is given.
pub struct Trace {
    pub log: String,
    r: Option<JavaRandom>,
}

impl Trace {
    pub fn new(r: Option<JavaRandom>) -> Self {
        Trace {
            log: String::new(),
            r,
        }
    }

    /// `new Random(1000L * policy + id.hashCode())`, or none for policy 0.
    pub fn for_policy(policy: i32, id: &str) -> Self {
        let r = (policy != 0).then(|| {
            JavaRandom::new(1000i64 * policy as i64 + java_lang::string_hash_code(id) as i64)
        });
        Trace::new(r)
    }

    fn vr(&mut self, s: &str) -> ValueResult {
        let res = match self.r.as_mut() {
            None => ValueResult::Continue,
            Some(r) => match r.next_int_bounded(20) {
                0 => ValueResult::Halt,
                1 | 2 => ValueResult::Break,
                _ => ValueResult::Continue,
            },
        };
        let name = match res {
            ValueResult::Continue => "CONTINUE",
            ValueResult::Break => "BREAK",
            ValueResult::Halt => "HALT",
        };
        self.log.push_str(&format!("{s}={name};"));
        res
    }

    fn er(&mut self, s: &str) -> EntryResult {
        let res = match self.r.as_mut() {
            None => EntryResult::Enter,
            Some(r) => match r.next_int_bounded(20) {
                0 => EntryResult::Halt,
                1 | 2 => EntryResult::Break,
                3..=5 => EntryResult::Skip,
                _ => EntryResult::Enter,
            },
        };
        let name = match res {
            EntryResult::Enter => "ENTER",
            EntryResult::Skip => "SKIP",
            EntryResult::Break => "BREAK",
            EntryResult::Halt => "HALT",
        };
        self.log.push_str(&format!("{s}={name};"));
        res
    }
}

impl StreamTagVisitor for Trace {
    fn visit_end(&mut self) -> ValueResult {
        self.vr("E")
    }
    fn visit_string(&mut self, v: &str) -> ValueResult {
        self.vr(&format!("s:{}", esc(v)))
    }
    fn visit_byte(&mut self, v: i8) -> ValueResult {
        self.vr(&format!("b:{v}"))
    }
    fn visit_short(&mut self, v: i16) -> ValueResult {
        self.vr(&format!("h:{v}"))
    }
    fn visit_int(&mut self, v: i32) -> ValueResult {
        self.vr(&format!("i:{v}"))
    }
    fn visit_long(&mut self, v: i64) -> ValueResult {
        self.vr(&format!("l:{v}"))
    }
    fn visit_float(&mut self, v: f32) -> ValueResult {
        // `Integer.toHexString(Float.floatToIntBits(v))`: NaN canonicalised.
        self.vr(&format!("f:{:x}", java_io::float_to_int_bits(v) as u32))
    }
    fn visit_double(&mut self, v: f64) -> ValueResult {
        self.vr(&format!("d:{:x}", java_io::double_to_long_bits(v) as u64))
    }
    fn visit_byte_array(&mut self, v: &[i8]) -> ValueResult {
        self.vr(&format!("B:{}", join(v)))
    }
    fn visit_int_array(&mut self, v: &[i32]) -> ValueResult {
        self.vr(&format!("I:{}", join(v)))
    }
    fn visit_long_array(&mut self, v: &[i64]) -> ValueResult {
        self.vr(&format!("L:{}", join(v)))
    }
    fn visit_list(&mut self, t: &TagTypeRef, size: i32) -> ValueResult {
        self.vr(&format!("LIST:{},{size}", t.get_name()))
    }
    fn visit_entry(&mut self, t: &TagTypeRef) -> EntryResult {
        self.er(&format!("ENT:{}", t.get_name()))
    }
    fn visit_entry_named(&mut self, t: &TagTypeRef, id: &str) -> EntryResult {
        self.er(&format!("ENTN:{},{}", t.get_name(), esc(id)))
    }
    fn visit_element(&mut self, t: &TagTypeRef, index: i32) -> EntryResult {
        self.er(&format!("ELEM:{},{index}", t.get_name()))
    }
    fn visit_container_end(&mut self) -> ValueResult {
        self.vr("CEND")
    }
    fn visit_root_entry(&mut self, t: &TagTypeRef) -> ValueResult {
        self.vr(&format!("ROOT:{}", t.get_name()))
    }
}
