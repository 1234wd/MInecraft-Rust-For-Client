//! Session 19 batch g: NBT core (net/minecraft/nbt). Checks ports against probes of the real jar
//! and the Java source's behaviour.
//!
//! Session 21 filled this in. Every expectation here comes from
//! `_porting/test-data/nbt_probe.txt`, written by `_porting/java-oracle/src/p/NbtProbe.java`
//! against the real 26.2 jar on JDK 25.0.4.1 -- not from reading the Java source and
//! guessing. Where a test builds its own input (the `UTFW` corpus, the `COLL` edit script)
//! it mirrors the probe's generator line for line, which is the arrangement NbtProbe's
//! javadoc describes.
//!
//! # What is NOT covered here, and why
//!
//! `ListTag` and `CompoundTag` are still SKELETON, so the `LGET`, `CGET`, `READ`, `GEN`,
//! `CASE*`, `HM*`, `IDENT` and `ACC`-via-`NbtIo` groups have no port to check yet, and the
//! `list_*` rows of `IDX` and entries 7 and 8 of `OTHER` are skipped by name rather than
//! silently dropped -- see [`unstarted_groups_are_declared`], which fails if that list
//! drifts out of step with what the file contains.

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

use std::panic::{catch_unwind, AssertUnwindSafe};

use minecraft_rust::javacompat::java_io::{
    self, DataInput, DataInputStream, DataOutput, DataOutputStream,
};
use minecraft_rust::net::minecraft::nbt::ByteArrayTag::ByteArrayTag;
use minecraft_rust::net::minecraft::nbt::ByteTag::ByteTag;
use minecraft_rust::net::minecraft::nbt::CollectionTag::CollectionTag;
use minecraft_rust::net::minecraft::nbt::DoubleTag::DoubleTag;
use minecraft_rust::net::minecraft::nbt::FloatTag::FloatTag;
use minecraft_rust::net::minecraft::nbt::IntArrayTag::IntArrayTag;
use minecraft_rust::net::minecraft::nbt::IntTag::IntTag;
use minecraft_rust::net::minecraft::nbt::LongArrayTag::LongArrayTag;
use minecraft_rust::net::minecraft::nbt::LongTag::LongTag;
use minecraft_rust::net::minecraft::nbt::NbtAccounter::NbtAccounter;
use minecraft_rust::net::minecraft::nbt::NumericTag::NumericTag;
use minecraft_rust::net::minecraft::nbt::ShortTag::ShortTag;
use minecraft_rust::net::minecraft::nbt::StringTag;
use minecraft_rust::net::minecraft::nbt::StringTag::StringTag as StringTagValue;
use minecraft_rust::net::minecraft::nbt::Tag::Tag;
use minecraft_rust::net::minecraft::nbt::TagType::TagTypeRef;
use minecraft_rust::net::minecraft::nbt::TagTypes;

// =====================================================================================
// The probe reader
// =====================================================================================

mod probe {
    /// TAB-separated rows of `nbt_probe.txt` whose first field is `kind`.
    ///
    /// Returns the fields still ESCAPED -- callers unescape only the fields they compare,
    /// because `\N` (null) has to stay distinguishable from the two-character string `\N`.
    pub fn rows(kind: &str) -> Vec<Vec<String>> {
        let path =
            minecraft_rust::javacompat::golden::test_data_dir().join("nbt_probe.txt");
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        let rows: Vec<Vec<String>> = text
            .lines()
            .filter(|l| !l.starts_with('#'))
            .map(|l| l.split('\t').map(str::to_string).collect::<Vec<String>>())
            .filter(|f| f.first().map(|k| k == kind).unwrap_or(false))
            .collect();
        assert!(!rows.is_empty(), "nbt_probe.txt has no `{kind}` rows");
        rows
    }

    /// The probe's `esc` run backwards, to UTF-16 code units.
    ///
    /// `esc` doubles a backslash and writes every char outside `0x20..=0x7E` as `\uXXXX`
    /// UTF-16 code units, so unescaping to units is exact -- including the lone surrogates
    /// in the `UTFW`/`UTFR` corpus, which no Rust `String` can hold.
    pub fn unesc_units(field: &str) -> Vec<u16> {
        let chars: Vec<char> = field.chars().collect();
        let mut out = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '\\' {
                if i + 1 < chars.len() && chars[i + 1] == '\\' {
                    out.push('\\' as u16);
                    i += 2;
                    continue;
                }
                if i + 1 < chars.len() && chars[i + 1] == 'u' && i + 6 <= chars.len() {
                    let hex: String = chars[i + 2..i + 6].iter().collect();
                    if let Ok(unit) = u16::from_str_radix(&hex, 16) {
                        out.push(unit);
                        i += 6;
                        continue;
                    }
                }
                panic!("unrecognised escape at {i} in {field:?}");
            }
            let mut buf = [0u16; 2];
            for u in chars[i].encode_utf16(&mut buf) {
                out.push(*u);
            }
            i += 1;
        }
        out
    }

    /// [`unesc_units`] as a `String`.
    ///
    /// # Panics
    ///
    /// On an unpaired surrogate. Three `UTFW`/`UTFR` rows carry one; those tests use
    /// `unesc_units` and the `*_units` I/O entry points instead, so reaching this with a
    /// lone surrogate means a test picked the wrong helper.
    pub fn unesc(field: &str) -> String {
        let units = unesc_units(field);
        String::from_utf16(&units)
            .unwrap_or_else(|_| panic!("row field has an unpaired surrogate: {field:?}"))
    }

    /// The probe's `esc`, forwards, over UTF-16 code units.
    pub fn esc_units(units: &[u16]) -> String {
        let mut out = String::new();
        for &u in units {
            if u == '\\' as u16 {
                out.push_str("\\\\");
            } else if (0x20..=0x7e).contains(&u) {
                out.push(u as u8 as char);
            } else {
                out.push_str(&format!("\\u{u:04x}"));
            }
        }
        out
    }

    /// The probe's `esc`, forwards, over a `str`.
    pub fn esc(s: &str) -> String {
        let units: Vec<u16> = s.encode_utf16().collect();
        esc_units(&units)
    }

    /// The probe's `hex`: lower-case, no separator.
    pub fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// The probe's `unhex`.
    pub fn unhex(s: &str) -> Vec<u8> {
        assert!(s.len().is_multiple_of(2), "odd hex length: {s:?}");
        (0..s.len() / 2)
            .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).expect("hex"))
            .collect()
    }

    /// A `DataInputStream` over the given bytes, i.e. the probe's `in(byte[])`.
    pub fn input(bytes: &[u8]) -> super::DataInputStream<std::io::Cursor<Vec<u8>>> {
        super::DataInputStream::new(std::io::Cursor::new(bytes.to_vec()))
    }
}

/// The probe's `ACC` log spells an exception as `class + ":" + message` -- no space -- while
/// a panic from this port spells it `class + ": " + message`, which is what
/// `Throwable#toString` prints and therefore what a crash log shows. Same two fields, one
/// separator apart, so the comparison converts rather than weakening either side.
fn as_probe_log(panic_message: &str) -> String {
    match panic_message.split_once(": ") {
        Some((class, message)) => format!("{class}:{message}"),
        None => panic_message.to_string(),
    }
}

/// Runs `f`, expects a panic, returns its message. Same helper as `session19_f.rs`.
fn panic_msg<R>(f: impl FnOnce() -> R) -> String {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(_) => panic!("expected a panic, got none"),
        Err(e) => {
            if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "<non-string panic>".to_string()
            }
        }
    }
}

/// Renders a `std::io::Error` the way the probe's `exc` renders a `Throwable`:
/// `EX \t class \t message \t causeClass \t causeMessage`, as the trailing fields of a row.
fn exc_fields(e: &std::io::Error) -> Vec<String> {
    vec![
        "EX".to_string(),
        java_io::java_class_name(e).to_string(),
        match java_io::java_message(e) {
            Some(m) => probe::esc(&m),
            None => "\\N".to_string(),
        },
        // Nothing in `javacompat::java_io` chains a cause, and no row in the groups this
        // file checks has one either; the assertion below is what keeps that true.
        "\\N".to_string(),
        "\\N".to_string(),
    ]
}

// =====================================================================================
// TagType / TagTypes  --  `TT`, `TTEX`
// =====================================================================================

/// `TagTypes.getType(id).getName()` / `.getPrettyName()` / `.size()` for ids `-2 ..= 14`.
///
/// The `size` column is `-` for every kind that is not a `TagType.StaticSize`, which is how
/// the probe records "this type has no fixed element width".
#[test]
fn tag_types_match_the_jar() {
    let rows = probe::rows("TT");
    assert_eq!(rows.len(), 17, "expected ids -2..=14");

    for r in &rows {
        let id: i32 = r[1].parse().expect("tag id");
        let want_name = &r[2];
        let want_pretty = &r[3];
        let want_size = &r[4];

        let ty = TagTypes::get_type(id);
        assert_eq!(&ty.get_name(), want_name, "getName for id {id}");
        assert_eq!(&ty.get_pretty_name(), want_pretty, "getPrettyName for id {id}");

        let got_size = match ty.static_size() {
            Some(n) => n.to_string(),
            None => "-".to_string(),
        };
        assert_eq!(&got_size, want_size, "size() for id {id}");
    }
}

/// `TagTypes.getType(77).load(...)` throws `IOException("Invalid tag id: 77")`.
///
/// A checked exception in Java, so an `Err` here rather than a panic -- `NbtIo` propagates
/// it to its caller.
#[test]
fn invalid_tag_type_load_throws_like_the_jar() {
    let rows = probe::rows("TTEX");
    let r = rows.iter().find(|r| r[1] == "load77").expect("load77 row");

    let mut input = probe::input(&[]);
    let mut accounter = NbtAccounter::unlimited_heap();
    let err = TagTypes::get_type(77)
        .load(&mut input, &mut accounter)
        .expect_err("id 77 must not load");

    assert_eq!(exc_fields(&err), r[2..7].to_vec(), "load(77) exception");
}

// =====================================================================================
// NbtAccounter  --  `ACC`
// =====================================================================================

/// The quota meter, including both of its deliberate overflows.
///
/// Each row is a `;`-separated log the probe built by calling the accounter in a fixed
/// order and recording either `ok:<usage>` or `<exceptionClass>:<message>`. Reproducing the
/// log rather than asserting per-call keeps the ORDER of the state changes in the
/// comparison.
#[test]
fn nbt_accounter_matches_the_jar() {
    let rows = probe::rows("ACC");
    let row = |name: &str| -> String {
        let r = rows.iter().find(|r| r[1] == name).unwrap_or_else(|| panic!("no ACC {name}"));
        probe::unesc(&r[2])
    };

    // ---- "account": create(100), then 10, 50, 40, 1, 0, -1, then accountBytes(3, -2).
    let mut log = String::new();
    let mut a = NbtAccounter::create(100);
    for size in [10i64, 50, 40, 1, 0, -1] {
        match catch_unwind(AssertUnwindSafe(|| {
            let mut probe_acc = a.clone();
            probe_acc.account_bytes(size);
            probe_acc
        })) {
            Ok(next) => {
                a = next;
                log.push_str(&format!("ok:{};", a.get_usage()));
            }
            Err(e) => {
                let msg = if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    String::from("<non-string panic>")
                };
                log.push_str(&as_probe_log(&msg));
                log.push(';');
            }
        }
    }
    let msg = panic_msg(|| {
        let mut probe_acc = a.clone();
        probe_acc.account_bytes_per_entry(3, -2);
    });
    log.push_str(&as_probe_log(&msg));
    log.push(';');
    assert_eq!(log, row("account"), "ACC account");

    // ---- "depth": new NbtAccounter(Long.MAX_VALUE, 3), five pushes then five pops.
    let mut log = String::new();
    let mut d = NbtAccounter::new(i64::MAX, 3);
    for _ in 0..5 {
        match catch_unwind(AssertUnwindSafe(|| {
            let mut probe_acc = d.clone();
            probe_acc.push_depth();
            probe_acc
        })) {
            Ok(next) => {
                d = next;
                log.push_str(&format!("push:{};", d.get_depth()));
            }
            Err(e) => {
                log.push_str(&as_probe_log(
                    e.downcast_ref::<String>().expect("panic message"),
                ));
                log.push(';');
            }
        }
    }
    for _ in 0..5 {
        match catch_unwind(AssertUnwindSafe(|| {
            let mut probe_acc = d.clone();
            probe_acc.pop_depth();
            probe_acc
        })) {
            Ok(next) => {
                d = next;
                log.push_str(&format!("pop:{};", d.get_depth()));
            }
            Err(e) => {
                log.push_str(&as_probe_log(
                    e.downcast_ref::<String>().expect("panic message"),
                ));
                log.push(';');
            }
        }
    }
    assert_eq!(log, row("depth"), "ACC depth");

    // ---- "overflow": unlimitedHeap(), Long.MAX_VALUE then 1, then a wrapping product.
    //
    // This is the row that forces `wrapping_add` in the quota check: the second call pushes
    // usage past Long.MAX_VALUE and vanilla ACCEPTS it, leaving usage negative.
    let mut log = String::new();
    let mut ov = NbtAccounter::unlimited_heap();
    ov.account_bytes(i64::MAX);
    log.push_str(&format!("ok:{};", ov.get_usage()));
    ov.account_bytes(1);
    log.push_str(&format!("ok:{};", ov.get_usage()));
    // 4 * 0x4000000000000000 wraps to 0, so a 4-exabyte array accounts for nothing.
    NbtAccounter::unlimited_heap().account_bytes_per_entry(4, 0x4000_0000_0000_0000);
    log.push_str("ok-wrap;");
    assert_eq!(log, row("overflow"), "ACC overflow");
}

// =====================================================================================
// The six numeric tags  --  `PRIM`
// =====================================================================================

/// Every conversion, `toString`, `hashCode`, `asBoolean`, the boxed class and
/// `sizeInBytes`, for all 92 probed values.
///
/// Row layout, from `NbtProbe.numRow`:
/// `PRIM <label> byte short int long floatBits doubleBits toString hashCode asBoolean
/// boxClass box sizeInBytes`.
#[test]
fn numeric_tags_match_the_jar() {
    let rows = probe::rows("PRIM");
    assert_eq!(rows.len(), 92, "PRIM row count");

    let mut seen = 0;
    for r in &rows {
        let label = &r[1];
        let (kind, raw) = label.split_once(':').expect("PRIM label is kind:value");

        // Build the same tag the probe built.
        let tag: Tag = match kind {
            "B" => ByteTag::value_of(raw.parse::<i8>().expect("i8")).into_tag(),
            "S" => ShortTag::value_of(raw.parse::<i16>().expect("i16")).into_tag(),
            "I" => IntTag::value_of(raw.parse::<i32>().expect("i32")).into_tag(),
            "L" => LongTag::value_of(raw.parse::<i64>().expect("i64")).into_tag(),
            // The float/double labels are RAW BITS in hex, so a NaN payload survives the
            // round trip through the file -- `golden-data-records-raw-bits`.
            "F" => {
                let bits = u32::from_str_radix(raw, 16).expect("f32 bits");
                FloatTag::new(f32::from_bits(bits)).into_tag()
            }
            "D" => {
                let bits = u64::from_str_radix(raw, 16).expect("f64 bits");
                DoubleTag::new(f64::from_bits(bits)).into_tag()
            }
            other => panic!("unknown PRIM kind {other:?}"),
        };

        let n = tag.as_numeric().expect("PRIM tags are numeric");
        let at = |i: usize| -> &str { r[i].as_str() };

        assert_eq!(n.byte_value().to_string(), at(2), "{label} byteValue");
        assert_eq!(n.short_value().to_string(), at(3), "{label} shortValue");
        assert_eq!(n.int_value().to_string(), at(4), "{label} intValue");
        assert_eq!(n.long_value().to_string(), at(5), "{label} longValue");
        // `Integer.toHexString` / `Long.toHexString`: no leading zeroes, lower case.
        assert_eq!(
            format!("{:x}", n.float_value().to_bits()),
            at(6),
            "{label} floatValue bits"
        );
        assert_eq!(
            format!("{:x}", n.double_value().to_bits()),
            at(7),
            "{label} doubleValue bits"
        );
        assert_eq!(probe::esc(&tag.to_string()), at(8), "{label} toString");

        let hash = match &tag {
            Tag::Byte(t) => t.java_hash_code(),
            Tag::Short(t) => t.java_hash_code(),
            Tag::Int(t) => t.java_hash_code(),
            Tag::Long(t) => t.java_hash_code(),
            Tag::Float(t) => t.java_hash_code(),
            Tag::Double(t) => t.java_hash_code(),
            other => panic!("not a numeric tag: {other:?}"),
        };
        assert_eq!(hash.to_string(), at(9), "{label} hashCode");
        assert_eq!(n.as_boolean().to_string(), at(10), "{label} asBoolean");
        assert_eq!(n.box_class_simple_name(), at(11), "{label} box class");
        assert_eq!(n.box_to_string(), at(12), "{label} box");
        assert_eq!(tag.size_in_bytes().to_string(), at(13), "{label} sizeInBytes");

        // `Tag`'s own `asX()` defaults must agree with the `NumericTag` overrides.
        assert_eq!(tag.as_byte(), Some(n.byte_value()), "{label} Tag::as_byte");
        assert_eq!(tag.as_int(), Some(n.int_value()), "{label} Tag::as_int");
        assert_eq!(tag.as_boolean(), Some(n.as_boolean()), "{label} Tag::as_boolean");
        assert_eq!(tag.as_string(), None, "{label} a numeric tag has no asString");

        seen += 1;
    }
    assert_eq!(seen, 92);
}

/// The two conversions that differ between `FloatTag` and `DoubleTag`.
///
/// Called out separately from the corpus because it is the one asymmetry most likely to be
/// "tidied" into a shared helper: `(long) f` truncates toward zero, `(long) Math.floor(d)`
/// floors. The corpus above covers it, but not legibly.
#[test]
fn float_long_value_truncates_and_double_long_value_floors() {
    assert_eq!(FloatTag::new(-1.5).long_value(), -1, "(long) -1.5F truncates");
    assert_eq!(DoubleTag::new(-1.5).long_value(), -2, "(long) Math.floor(-1.5) floors");
    assert_eq!(FloatTag::new(1.5).long_value(), 1);
    assert_eq!(DoubleTag::new(1.5).long_value(), 1);

    // `intValue` goes through `Mth.floor` for BOTH, so they agree there.
    assert_eq!(FloatTag::new(-1.5).int_value(), -2);
    assert_eq!(DoubleTag::new(-1.5).int_value(), -2);
}

/// `valueOf(-0.0)` returns `ZERO`, which holds `+0.0`, so the sign of zero is lost.
///
/// `new` keeps it. Vanilla behaviour, and the reason `TagType.load` for FLOAT/DOUBLE cannot
/// round-trip a negative zero out of a file.
#[test]
fn value_of_negative_zero_loses_the_sign() {
    assert!(FloatTag::value_of(-0.0).value().is_sign_positive(), "valueOf(-0.0F)");
    assert!(DoubleTag::value_of(-0.0).value().is_sign_positive(), "valueOf(-0.0)");
    assert!(FloatTag::new(-0.0).value().is_sign_negative(), "new FloatTag(-0.0F)");
    assert!(DoubleTag::new(-0.0).value().is_sign_negative(), "new DoubleTag(-0.0)");
}

// =====================================================================================
// StringTag  --  `QUOTE`
// =====================================================================================

/// `quoteAndEscape` and `escapeWithoutQuotes` over the probe's 18-string corpus.
///
/// This is the test that pins the delimiter choice: the first `"` or `'` in the input picks
/// the OTHER character as the quote, and only that character then needs escaping.
#[test]
fn string_tag_quoting_matches_the_jar() {
    let rows = probe::rows("QUOTE");
    assert_eq!(rows.len(), 18, "QUOTE row count");

    for r in &rows {
        let input = probe::unesc(&r[1]);
        assert_eq!(
            probe::esc(&StringTag::quote_and_escape(&input)),
            r[2],
            "quoteAndEscape({:?})",
            input
        );
        assert_eq!(
            probe::esc(&StringTag::escape_without_quotes(&input)),
            r[3],
            "escapeWithoutQuotes({:?})",
            input
        );
    }
}

/// `sizeInBytes` and the accounter charge UTF-16 code units, not characters or UTF-8 bytes.
#[test]
fn string_tag_size_counts_utf16_code_units() {
    // An emoji is one `char` in Rust, four UTF-8 bytes, and TWO UTF-16 code units.
    assert_eq!(StringTag::java_length("\u{1F600}"), 2);
    assert_eq!(StringTag::java_length("abc"), 3);
    assert_eq!(StringTag::java_length("\u{4E2D}"), 1);

    // 36 + 2 * length()
    assert_eq!(StringTagValue::value_of("").size_in_bytes(), 36);
    assert_eq!(StringTagValue::value_of("abc").size_in_bytes(), 42);
    assert_eq!(StringTagValue::value_of("\u{1F600}").size_in_bytes(), 40);
}

// =====================================================================================
// The array tags  --  `COLL`, `IDX`, `OTHER`
// =====================================================================================

/// The probe's edit scripts, replayed.
///
/// Every `addTag`/`setTag` here is handed a tag of the WRONG numeric kind on purpose, so
/// the row content is the narrowing table: a `DoubleTag(300.7)` stored into a byte array
/// becomes `44`, a `FloatTag(1e30)` into a long array saturates to `Long.MAX_VALUE`.
#[test]
fn array_tag_collection_edits_match_the_jar() {
    let rows = probe::rows("COLL");
    let row = |name: &str| -> (String, String) {
        let r = rows.iter().find(|r| r[1] == name).unwrap_or_else(|| panic!("no COLL {name}"));
        (probe::unesc(&r[2]), r[3].clone())
    };

    // ---- bytearray
    let (want_str, want_rejected) = row("bytearray");
    let mut ba = ByteArrayTag::new(vec![1, 2, 3]);
    assert!(ba.add_tag(0, &FloatTag::value_of(-1.5).into_tag()));
    assert!(ba.add_tag(4, &LongTag::value_of(0x1ff).into_tag()));
    assert!(ba.set_tag(1, &DoubleTag::value_of(300.7).into_tag()));
    let rejected = ba.add_tag(0, &StringTagValue::value_of("x").into_tag());
    ba.remove(2);
    assert_eq!(ba.to_string(), want_str, "COLL bytearray");
    assert_eq!(rejected.to_string(), want_rejected, "COLL bytearray rejected");

    // ---- intarray
    let (want_str, want_rejected) = row("intarray");
    let mut ia = IntArrayTag::new(vec![1, 2, 3]);
    assert!(ia.add_tag(3, &DoubleTag::value_of(-1e20).into_tag()));
    assert!(ia.set_tag(0, &LongTag::value_of(0x1_2345_6789).into_tag()));
    ia.remove(1);
    assert_eq!(ia.to_string(), want_str, "COLL intarray");
    // The probe's last column is `ia.setTag(0, new ListTag())` -- a non-numeric tag, so
    // `false`. `ListTag` is not ported; `StringTag` exercises the same `else` branch, and
    // the assertion is that the column really is `false`.
    assert_eq!(want_rejected, "false", "COLL intarray rejected column");
    assert!(!ia.set_tag(0, &StringTagValue::value_of("x").into_tag()));

    // ---- longarray
    let (want_str, want_empty) = row("longarray");
    let mut la = LongArrayTag::new(vec![1, 2, 3]);
    assert!(la.add_tag(1, &FloatTag::value_of(1e30).into_tag()));
    assert!(la.set_tag(0, &DoubleTag::value_of(-2.5).into_tag()));
    la.clear();
    assert!(la.is_empty(), "clear() leaves an empty array");
    assert!(la.add_tag(0, &ByteTag::value_of(-1).into_tag()));
    assert_eq!(la.to_string(), want_str, "COLL longarray");
    assert_eq!(la.is_empty().to_string(), want_empty, "COLL longarray isEmpty");
}

/// The index-error messages for the three array tags.
///
/// `get`/`setTag`/`remove` raise the JDK's `ArrayIndexOutOfBoundsException` text, while
/// `addTag` goes through commons-lang3's `ArrayUtils.add` and raises
/// `IndexOutOfBoundsException` with a DIFFERENT message shape (`Index: n, Length: m`). Two
/// exception classes and two message formats in one class, which is exactly the kind of
/// thing that is only ever right by measurement.
#[test]
fn array_tag_index_errors_match_the_jar() {
    let rows = probe::rows("IDX");

    let expect = |name: &str| -> (String, String) {
        let r = rows.iter().find(|r| r[1] == name).unwrap_or_else(|| panic!("no IDX {name}"));
        assert_eq!(r[2], "EX", "IDX {name} was expected to throw");
        (r[3].clone(), probe::unesc(&r[4]))
    };

    let check = |name: &str, f: &mut dyn FnMut()| {
        let (class, message) = expect(name);
        let got = panic_msg(AssertUnwindSafe(&mut *f));
        assert_eq!(got, format!("{class}: {message}"), "IDX {name}");
    };

    check("ba_get_3", &mut || {
        ByteArrayTag::new(vec![1, 2, 3]).get(3);
    });
    check("ba_settag_3", &mut || {
        ByteArrayTag::new(vec![1, 2, 3]).set_tag(3, &IntTag::value_of(0).into_tag());
    });
    check("ba_addtag_4", &mut || {
        ByteArrayTag::new(vec![1, 2, 3]).add_tag(4, &IntTag::value_of(0).into_tag());
    });
    check("ba_addtag_neg", &mut || {
        ByteArrayTag::new(vec![1, 2, 3]).add_tag(-1, &IntTag::value_of(0).into_tag());
    });
    check("ba_remove_3", &mut || {
        ByteArrayTag::new(vec![1, 2, 3]).remove(3);
    });
    check("ia_get_3", &mut || {
        IntArrayTag::new(vec![1, 2, 3]).get(3);
    });
    check("ia_addtag_4", &mut || {
        IntArrayTag::new(vec![1, 2, 3]).add_tag(4, &IntTag::value_of(0).into_tag());
    });
    check("ia_remove_neg", &mut || {
        IntArrayTag::new(vec![1, 2, 3]).remove(-1);
    });
    check("la_get_3", &mut || {
        LongArrayTag::new(vec![1, 2, 3]).get(3);
    });
    check("la_addtag_5", &mut || {
        LongArrayTag::new(vec![1, 2, 3]).add_tag(5, &IntTag::value_of(0).into_tag());
    });
    check("la_remove_3", &mut || {
        LongArrayTag::new(vec![1, 2, 3]).remove(3);
    });
}

/// `toString`, `hashCode`, `sizeInBytes` and the `asX()` views for the non-numeric kinds.
///
/// Entries 7 and 8 of the probe's array are a `ListTag` and a `CompoundTag`; they are
/// skipped here and declared in [`unstarted_groups_are_declared`].
#[test]
fn non_numeric_tag_views_match_the_jar() {
    let rows = probe::rows("OTHER");
    assert_eq!(rows.len(), 9, "OTHER row count");

    // The probe's `others` array, entries 0..=6. 7 = new ListTag(), 8 = new CompoundTag().
    let tags: Vec<(usize, Tag)> = vec![
        (0, StringTagValue::value_of("").into_tag()),
        (1, StringTagValue::value_of("abc").into_tag()),
        (2, StringTagValue::value_of("\u{1F600}").into_tag()),
        (3, ByteArrayTag::new(vec![1, 2, -3]).into_tag()),
        (4, IntArrayTag::new(vec![1, 2, -3]).into_tag()),
        (5, LongArrayTag::new(vec![1, 2, -3, i64::MIN]).into_tag()),
        (6, ByteArrayTag::new(vec![]).into_tag()),
    ];

    for (i, tag) in &tags {
        let r = rows
            .iter()
            .find(|r| r[1] == i.to_string())
            .unwrap_or_else(|| panic!("no OTHER {i}"));

        assert_eq!(probe::esc(&tag.to_string()), r[2], "OTHER {i} toString");

        let hash = match tag {
            Tag::String(t) => t.java_hash_code(),
            Tag::ByteArray(t) => t.java_hash_code(),
            Tag::IntArray(t) => t.java_hash_code(),
            Tag::LongArray(t) => t.java_hash_code(),
            other => panic!("unexpected OTHER tag {other:?}"),
        };
        assert_eq!(hash.to_string(), r[3], "OTHER {i} hashCode");
        assert_eq!(tag.size_in_bytes().to_string(), r[4], "OTHER {i} sizeInBytes");

        // `asString()` -- `\N` when empty.
        let want_as_string = &r[5];
        match tag.as_string() {
            Some(s) => assert_eq!(&probe::esc(&s), want_as_string, "OTHER {i} asString"),
            None => assert_eq!(want_as_string, "\\N", "OTHER {i} asString"),
        }
        // Neither strings nor arrays are numeric, so both of these are `\N`.
        assert_eq!(r[6], "\\N", "OTHER {i} asNumber");
        assert!(tag.as_numeric().is_none(), "OTHER {i} as_numeric");
        assert_eq!(r[7], "\\N", "OTHER {i} asBoolean");
        assert!(tag.as_boolean().is_none(), "OTHER {i} as_boolean");
    }
}

/// `Arrays.hashCode` is seeded with `1`, so an empty array hashes to `1`, not `0`.
#[test]
fn empty_array_tag_hashes_to_one() {
    assert_eq!(ByteArrayTag::new(vec![]).java_hash_code(), 1);
    assert_eq!(IntArrayTag::new(vec![]).java_hash_code(), 1);
    assert_eq!(LongArrayTag::new(vec![]).java_hash_code(), 1);
}

/// `CollectionTag.iterator()` is an index walk that re-reads `size()`.
#[test]
fn collection_tag_iterator_walks_by_index() {
    let ia = IntArrayTag::new(vec![7, 8, 9]);
    let seen: Vec<Tag> = ia.iter().collect();
    assert_eq!(seen.len(), 3);
    assert_eq!(seen[0], IntTag::value_of(7).into_tag());
    assert_eq!(seen[2], IntTag::value_of(9).into_tag());

    assert_eq!(ByteArrayTag::new(vec![]).iter().count(), 0);
}

// =====================================================================================
// javacompat::java_io  --  `UTFW`, `UTFR`, `F32`, `F64`, `PRIMIO`, `SKIP`, `OUT`
// =====================================================================================
//
// `_porting/javacompat/java_io.rs` carried `Status: PORTED` and a header claiming it was
// "checked against JDK 25 by nbt_probe.txt ... tests/session19_g.rs" since session 19.
// It was never declared in `javacompat/mod.rs`, so it had never been COMPILED, let alone
// checked. Session 21 wired it in; these are the tests the header promised.

/// `DataOutputStream.writeUTF` -- Java's modified UTF-8, over the probe's 22-string corpus.
///
/// The corpus is rebuilt here as UTF-16 code units because two of its entries are lone
/// surrogates, which no Rust `String` can hold. `write_utf_units` is `java_io`'s exact
/// entry point for that case.
#[test]
fn modified_utf8_write_matches_the_jar() {
    fn units(s: &str) -> Vec<u16> {
        s.encode_utf16().collect()
    }
    fn repeat(s: &str, n: usize) -> Vec<u16> {
        let u = units(s);
        u.iter().cycle().take(u.len() * n).copied().collect()
    }

    // NbtProbe.ioSection's `utf` array, in order.
    let corpus: Vec<Vec<u16>> = vec![
        units(""),
        units("abc"),
        units("\u{0000}"),
        units("a\u{0000}b"),
        units("\u{007f}"),
        units("\u{0080}"),
        units("\u{07ff}"),
        units("\u{0800}"),
        units("\u{ffff}"),
        units("\u{1F600}"),
        units("\u{00e9}\u{4e2d}"),
        vec![0xD800],
        vec![0xDC00, 'x' as u16],
        repeat("a", 65535),
        repeat("a", 65536),
        repeat("\u{4e2d}", 21845),
        repeat("\u{4e2d}", 21846),
        repeat("\u{0000}", 32768),
        {
            let mut v = repeat("\u{0000}", 32767);
            v.push('a' as u16);
            v
        },
        repeat("\u{1F600}", 10923),
        {
            let mut v = repeat("\u{1F600}", 10922);
            v.extend(units("\u{00e9}"));
            v
        },
        {
            let mut v = units("\u{1F600}abcdefgh");
            v.extend(repeat("x", 65530));
            v.extend(units("123456\u{1F600}"));
            v
        },
    ];

    let rows = probe::rows("UTFW");
    assert_eq!(rows.len(), corpus.len(), "UTFW row count vs corpus");

    for (i, input) in corpus.iter().enumerate() {
        // The probe's label: `esc(s)` for short strings, `LONG:<length>:<hashCode>` else.
        let label = if input.len() > 64 {
            let mut hash: i32 = 0;
            for &u in input {
                hash = hash.wrapping_mul(31).wrapping_add(u as i32);
            }
            format!("LONG:{}:{}", input.len(), hash)
        } else {
            probe::esc_units(input)
        };
        let r = rows
            .iter()
            .find(|r| r[1] == label)
            .unwrap_or_else(|| panic!("no UTFW row labelled {label:?} (corpus index {i})"));

        let mut out = DataOutputStream::new(Vec::new());
        let result = java_io::write_utf_units(&mut out, input);

        match r[2].as_str() {
            "OK" => {
                result.unwrap_or_else(|e| panic!("{label}: unexpected error {e}"));
                let bytes = out.into_inner();
                assert_eq!(probe::hex(&bytes), r[3], "UTFW {label} bytes");
            }
            "OKLEN" => {
                result.unwrap_or_else(|e| panic!("{label}: unexpected error {e}"));
                let bytes = out.into_inner();
                assert_eq!(bytes.len().to_string(), r[3], "UTFW {label} length");
                assert_eq!(probe::hex(&bytes[..16]), r[4], "UTFW {label} first 16 bytes");
            }
            "EX" => {
                let err = result.expect_err(&format!("{label} was expected to throw"));
                assert_eq!(exc_fields(&err), r[2..7].to_vec(), "UTFW {label} exception");
            }
            other => panic!("unknown UTFW result {other:?}"),
        }
    }
}

/// `DataInputStream.readUTF` -- including every malformed form the JDK rejects and the
/// overlong ones it accepts.
#[test]
fn modified_utf8_read_matches_the_jar() {
    for r in &probe::rows("UTFR") {
        let hex = &r[1];
        let bytes = probe::unhex(hex);
        let mut input = probe::input(&bytes);
        let result = java_io::read_utf_units(&mut input);

        match r[2].as_str() {
            "OK" => {
                let units = result.unwrap_or_else(|e| panic!("{hex}: unexpected error {e}"));
                assert_eq!(probe::esc_units(&units), r[3], "UTFR {hex}");
            }
            "EX" => {
                let err = result.expect_err(&format!("{hex} was expected to throw"));
                assert_eq!(exc_fields(&err), r[2..7].to_vec(), "UTFR {hex} exception");
            }
            other => panic!("unknown UTFR result {other:?}"),
        }
    }
}

/// `writeFloat` canonicalises NaN; `readFloat` keeps the raw bits.
///
/// So a NaN payload survives a read and not a write -- `writeFloat(0x7f800001)` puts
/// `7fc00000` on the wire. Both directions are in the row.
#[test]
fn float_and_double_io_matches_the_jar() {
    for r in &probe::rows("F32") {
        let bits = u32::from_str_radix(&r[1], 16).expect("f32 bits");
        let mut out = DataOutputStream::new(Vec::new());
        out.write_float(f32::from_bits(bits)).expect("writeFloat");
        assert_eq!(probe::hex(&out.into_inner()), r[2], "F32 {} write", r[1]);

        let mut input = probe::input(&probe::unhex(&r[1]));
        let back = input.read_float().expect("readFloat");
        assert_eq!(format!("{:x}", back.to_bits()), r[3], "F32 {} read", r[1]);
    }

    for r in &probe::rows("F64") {
        let bits = u64::from_str_radix(&r[1], 16).expect("f64 bits");
        let mut out = DataOutputStream::new(Vec::new());
        out.write_double(f64::from_bits(bits)).expect("writeDouble");
        assert_eq!(probe::hex(&out.into_inner()), r[2], "F64 {} write", r[1]);

        let mut input = probe::input(&probe::unhex(&r[1]));
        let back = input.read_double().expect("readDouble");
        assert_eq!(format!("{:x}", back.to_bits()), r[3], "F64 {} read", r[1]);
    }
}

/// A short stream raises `EOFException` from every primitive read.
#[test]
fn primitive_io_eof_matches_the_jar() {
    for r in &probe::rows("PRIMIO") {
        let label = &r[1];
        let (op, hex) = label.split_once(':').expect("PRIMIO label is op:hex");
        let bytes = probe::unhex(hex);
        let mut input = probe::input(&bytes);

        let result: std::io::Result<()> = match op {
            "readInt" => input.read_int().map(|_| ()),
            "readShort" => input.read_short().map(|_| ()),
            "readLong" => input.read_long().map(|_| ()),
            "readByte" => input.read_byte().map(|_| ()),
            "readUnsignedShort" => input.read_unsigned_short().map(|_| ()),
            "readFully3" => input.read_fully(&mut [0u8; 3]),
            other => panic!("unknown PRIMIO op {other:?}"),
        };

        match r[2].as_str() {
            "OK" => {
                result.unwrap_or_else(|e| panic!("{label}: unexpected error {e}"));
            }
            "EX" => {
                let err = result.expect_err(&format!("{label} was expected to throw"));
                assert_eq!(exc_fields(&err), r[2..7].to_vec(), "PRIMIO {label}");
            }
            other => panic!("unknown PRIMIO result {other:?}"),
        }
    }
}

/// `skipBytes` returns how many it actually skipped and never throws -- a negative or
/// oversized count is clamped, not rejected.
///
/// That is what makes `TagType.StaticSize.skip`'s wrapping multiply reachable: the skip
/// quietly does nothing and the next read is misaligned rather than failing.
#[test]
fn skip_bytes_matches_the_jar() {
    for r in &probe::rows("SKIP") {
        let label = &r[1];
        let (len, count) = label.split_once(':').expect("SKIP label is len:count");
        let len: usize = len.parse().expect("len");
        let count: i32 = count.parse().expect("count");

        let bytes: Vec<u8> = (0..len).map(|i| (i + 1) as u8).collect();
        let mut input = probe::input(&bytes);
        let skipped = input.skip_bytes(count).expect("skipBytes never throws");
        assert_eq!(skipped.to_string(), r[2], "SKIP {label} returned");

        match input.read_unsigned_byte() {
            Ok(next) => assert_eq!(next.to_string(), r[3], "SKIP {label} next byte"),
            Err(e) => assert_eq!(java_io::java_class_name(&e), r[3], "SKIP {label} next byte"),
        }
    }
}

/// The whole `DataOutput` surface in one stream, plus `size()`.
///
/// `writeShort(0x12345)` keeps the low 16 bits, `writeChar(-2)` writes `fffe`,
/// `writeByte(0x1ff)` writes `ff`, and `writeBytes` truncates each char to one byte while
/// `writeChars` writes two -- five narrowing rules, none of them the same.
#[test]
fn data_output_stream_matches_the_jar() {
    let rows = probe::rows("OUT");
    let r = rows.iter().find(|r| r[1] == "mixed").expect("OUT mixed row");

    let mut d = DataOutputStream::new(Vec::new());
    d.write_short(0x12345).unwrap();
    d.write_char(-2).unwrap();
    d.write_byte(0x1ff).unwrap();
    d.write_boolean(true).unwrap();
    d.write_bytes("A\u{0141}\u{ffff}").unwrap();
    d.write_chars("A\u{0141}").unwrap();
    d.write_int(-2).unwrap();
    d.write_long(0x0102_0304_0506_0708).unwrap();

    let size = d.size();
    assert_eq!(size.to_string(), r[3], "OUT size()");
    assert_eq!(probe::hex(&d.into_inner()), r[2], "OUT bytes");
}

// =====================================================================================
// Round trips through the ported TagTypes
// =====================================================================================

/// Every ported tag kind survives `write` -> `TagType.load`.
///
/// Not an oracle test -- it is the invariant that makes the oracle tests above mean
/// something: `write` and `load` are separate transcriptions of the same wire format, and a
/// matching pair of mistakes would pass both of them individually.
#[test]
fn ported_tags_round_trip_through_their_tag_type() {
    let cases: Vec<Tag> = vec![
        ByteTag::value_of(-128).into_tag(),
        ByteTag::value_of(127).into_tag(),
        ShortTag::value_of(i16::MIN).into_tag(),
        IntTag::value_of(i32::MIN).into_tag(),
        LongTag::value_of(i64::MIN).into_tag(),
        FloatTag::new(f32::consts_pi()).into_tag(),
        DoubleTag::new(-1.0e300).into_tag(),
        StringTagValue::value_of("hello \u{1F600} \u{0000}").into_tag(),
        ByteArrayTag::new(vec![0, 1, -1, 127, -128]).into_tag(),
        IntArrayTag::new(vec![0, -1, i32::MAX, i32::MIN]).into_tag(),
        LongArrayTag::new(vec![0, -1, i64::MAX, i64::MIN]).into_tag(),
        ByteArrayTag::new(vec![]).into_tag(),
    ];

    for tag in &cases {
        let mut out = DataOutputStream::new(Vec::new());
        tag.write(&mut out).expect("write");
        let bytes = out.into_inner();

        let mut input = probe::input(&bytes);
        let mut accounter = NbtAccounter::unlimited_heap();
        let back = tag
            .get_type()
            .load(&mut input, &mut accounter)
            .unwrap_or_else(|e| panic!("load {tag} failed: {e}"));

        assert_eq!(&back, tag, "round trip of {tag}");
        assert_eq!(back.get_id(), tag.get_id(), "id of {tag}");
        // Everything written must be consumed.
        assert!(
            input.read_byte().is_err(),
            "load({tag}) left bytes on the stream"
        );
    }
}

/// `TagType.skip` advances exactly as far as `load` consumes.
#[test]
fn ported_tag_types_skip_exactly_what_they_load() {
    let cases: Vec<Tag> = vec![
        ByteTag::value_of(5).into_tag(),
        ShortTag::value_of(5).into_tag(),
        IntTag::value_of(5).into_tag(),
        LongTag::value_of(5).into_tag(),
        FloatTag::value_of(5.0).into_tag(),
        DoubleTag::value_of(5.0).into_tag(),
        StringTagValue::value_of("skip me \u{4e2d}").into_tag(),
        ByteArrayTag::new(vec![1, 2, 3]).into_tag(),
        IntArrayTag::new(vec![1, 2, 3]).into_tag(),
        LongArrayTag::new(vec![1, 2, 3]).into_tag(),
    ];

    for tag in &cases {
        let mut out = DataOutputStream::new(Vec::new());
        tag.write(&mut out).expect("write");
        // A sentinel after the payload: `skip` must land exactly on it.
        out.write_byte(0x5a).expect("sentinel");
        let bytes = out.into_inner();

        let mut input = probe::input(&bytes);
        let mut accounter = NbtAccounter::unlimited_heap();
        tag.get_type().skip(&mut input, &mut accounter).unwrap_or_else(|e| {
            panic!("skip {tag} failed: {e}")
        });
        assert_eq!(
            input.read_unsigned_byte().expect("sentinel"),
            0x5a,
            "skip({tag}) stopped in the wrong place"
        );
    }
}

/// `EndTag`: writes nothing, costs 8 bytes of quota, and both skips do nothing.
#[test]
fn end_tag_writes_nothing_but_costs_quota() {
    let mut out = DataOutputStream::new(Vec::new());
    Tag::End.write(&mut out).expect("write");
    assert!(out.into_inner().is_empty(), "EndTag.write writes nothing");

    assert_eq!(Tag::End.size_in_bytes(), 8);
    assert_eq!(Tag::End.get_id(), 0);
    assert_eq!(Tag::End.to_string(), "END");

    let mut input = probe::input(&[]);
    let mut accounter = NbtAccounter::unlimited_heap();
    let loaded = TagTypeRef::Known(0).load(&mut input, &mut accounter).expect("load");
    assert_eq!(loaded, Tag::End);
    assert_eq!(accounter.get_usage(), 8, "TYPE.load charges 8 for an end tag");

    // Both skips are empty, so neither consumes the sentinel.
    let mut input = probe::input(&[0x5a]);
    TagTypeRef::Known(0).skip(&mut input, &mut accounter).expect("skip");
    TagTypeRef::Known(0).skip_count(&mut input, 100, &mut accounter).expect("skip_count");
    assert_eq!(input.read_unsigned_byte().expect("sentinel"), 0x5a);
}

/// The accounter is charged the documented per-kind amounts.
#[test]
fn tag_type_load_charges_the_documented_quota() {
    for (tag, want) in [
        (ByteTag::value_of(1).into_tag(), 9i64),
        (ShortTag::value_of(1).into_tag(), 10),
        (IntTag::value_of(1).into_tag(), 12),
        (LongTag::value_of(1).into_tag(), 16),
        (FloatTag::value_of(1.0).into_tag(), 12),
        (DoubleTag::value_of(1.0).into_tag(), 16),
        // 24 + 1 * 3
        (ByteArrayTag::new(vec![1, 2, 3]).into_tag(), 27),
        // 24 + 4 * 3
        (IntArrayTag::new(vec![1, 2, 3]).into_tag(), 36),
        // 24 + 8 * 3
        (LongArrayTag::new(vec![1, 2, 3]).into_tag(), 48),
        // 36 + 2 * length("abc")
        (StringTagValue::value_of("abc").into_tag(), 42),
    ] {
        let mut out = DataOutputStream::new(Vec::new());
        tag.write(&mut out).expect("write");
        let bytes = out.into_inner();

        let mut input = probe::input(&bytes);
        let mut accounter = NbtAccounter::unlimited_heap();
        tag.get_type().load(&mut input, &mut accounter).expect("load");
        assert_eq!(accounter.get_usage(), want, "quota charged for {tag}");
    }
}

/// A hostile array length is stopped by the accounter, with the accounter's message --
/// not by the allocation.
#[test]
fn hostile_array_length_is_rejected_by_the_accounter() {
    // length = -1: `accountBytes(1, -1)` is negative, so IllegalArgumentException comes
    // first -- before Java would reach `new byte[-1]`.
    let bytes = vec![0xff, 0xff, 0xff, 0xff];
    let msg = panic_msg(|| {
        let mut input = probe::input(&bytes);
        let mut accounter = NbtAccounter::unlimited_heap();
        let _ = TagTypeRef::Known(7).load(&mut input, &mut accounter);
    });
    assert_eq!(
        msg,
        "java.lang.IllegalArgumentException: Tried to account NBT tag with negative size: -1"
    );

    // A huge positive length is stopped by the quota, not by the allocator.
    let bytes = vec![0x7f, 0xff, 0xff, 0xff];
    let msg = panic_msg(|| {
        let mut input = probe::input(&bytes);
        let mut accounter = NbtAccounter::default_quota();
        let _ = TagTypeRef::Known(7).load(&mut input, &mut accounter);
    });
    assert!(
        msg.starts_with("net.minecraft.nbt.NbtAccounterException: Tried to read NBT tag that was too big"),
        "unexpected message: {msg}"
    );
}

// =====================================================================================
// Coverage declaration
// =====================================================================================

/// The groups of `nbt_probe.txt` this file does NOT claim, and why.
///
/// A list in a comment goes stale; this one fails. The assertion is that every group in the
/// file is either checked by a test above or named here, so adding a group to the probe
/// without a test -- or porting `CompoundTag` and forgetting to delete its entry -- breaks
/// the build instead of quietly reducing coverage. Same discipline as `parity_batch2.rs`'s
/// `BLOCKED_ON_UNPORTED_TYPES`.
#[test]
fn unstarted_groups_are_declared() {
    /// Checked by the tests in this file.
    const COVERED: &[&str] = &[
        "TT", "TTEX", "ACC", "PRIM", "QUOTE", "COLL", "IDX", "OTHER", "UTFW", "UTFR", "F32",
        "F64", "PRIMIO", "SKIP", "OUT",
    ];

    /// Checked by `tests/session23.rs` (session 23: the JDK-order map, `CompoundTag`, `ListTag`).
    /// Each must be read there -- `rows("<group>")` in one of [`SESSION23_SOURCES`] -- or this
    /// test fails, so moving a group here is not a way to stop checking it.
    const COVERED_IN_SESSION23: &[&str] =
        &["HMOP", "HMRES", "HMCOPY", "HMINT", "HMINTRES", "HMSTEP", "HMEQ"];

    /// Blocked on `ListTag` / `CompoundTag` / `NbtIo` / the `visitors` package, none of
    /// which is ported. Delete an entry when its port lands, and the `COVERED` check will
    /// then demand a test for it.
    const BLOCKED: &[(&str, &str)] = &[
        ("GEN", "the shared tree generator: needs CompoundTag and ListTag"),
        ("READ", "NbtIo.read over crafted streams: needs NbtIo"),
        ("CASE", "tree round trips: needs CompoundTag and ListTag"),
        ("CASED", "direct writeUTF of over-long strings: needs NbtIo"),
        ("CASEW", "tree write/toString: needs CompoundTag and ListTag"),
        ("IDENT", "CompoundTag's identify() fast path: needs CompoundTag"),
        ("LGET", "ListTag getters: needs ListTag"),
        ("CGET", "CompoundTag getters: needs CompoundTag"),
    ];

    for group in COVERED_IN_SESSION23 {
        let claim = format!("rows(\"{group}\")");
        assert!(
            SESSION23_SOURCES.iter().any(|src| src.contains(&claim)),
            "`{group}` is declared COVERED_IN_SESSION23 but no session23 module reads it"
        );
    }

    // Every group actually present in the file.
    let path = minecraft_rust::javacompat::golden::test_data_dir().join("nbt_probe.txt");
    let text = std::fs::read_to_string(&path).expect("nbt_probe.txt");
    let mut present: Vec<String> = text
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .map(|l| l.split('\t').next().unwrap_or("").to_string())
        .collect();
    present.sort();
    present.dedup();

    let blocked_names: Vec<&str> = BLOCKED.iter().map(|(n, _)| *n).collect();

    for group in &present {
        let covered = COVERED.contains(&group.as_str())
            || COVERED_IN_SESSION23.contains(&group.as_str());
        let blocked = blocked_names.contains(&group.as_str());
        assert!(
            covered || blocked,
            "nbt_probe.txt group `{group}` is neither covered by a test nor declared BLOCKED"
        );
        assert!(
            !(covered && blocked),
            "group `{group}` is listed both as COVERED and as BLOCKED"
        );
    }

    // And nothing is declared that the file does not contain.
    for name in COVERED
        .iter()
        .chain(COVERED_IN_SESSION23.iter())
        .chain(blocked_names.iter())
    {
        assert!(
            present.iter().any(|p| p == name),
            "`{name}` is declared but nbt_probe.txt has no such group"
        );
    }

    assert_eq!(
        present.len(),
        COVERED.len() + COVERED_IN_SESSION23.len() + BLOCKED.len(),
        "group bookkeeping drifted: {} in the file, {} covered + {} covered in session 23 + {} blocked",
        present.len(),
        COVERED.len(),
        COVERED_IN_SESSION23.len(),
        BLOCKED.len()
    );
}

/// The sources of `tests/session23.rs` and its modules, for the `COVERED_IN_SESSION23` check.
const SESSION23_SOURCES: &[&str] = &[
    include_str!("session23.rs"),
    include_str!("session23/a_hashmap.rs"),
];

/// Helper: `(float) Math.PI`, spelled out so the round-trip corpus has a value with a long
/// mantissa without depending on `Mth`.
trait F32Consts {
    fn consts_pi() -> f32;
}
impl F32Consts for f32 {
    fn consts_pi() -> f32 {
        std::f32::consts::PI
    }
}
