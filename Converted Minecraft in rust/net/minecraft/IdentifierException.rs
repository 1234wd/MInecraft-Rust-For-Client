//! Port of: net/minecraft/IdentifierException.java
//! Java class(es): net.minecraft.IdentifierException
//! Status: PORTED
//!
//! 13 lines of Java, and almost all of the porting difficulty is in one of them:
//!
//! ```java
//! public IdentifierException(final String message) {
//!     super(StringEscapeUtils.escapeJava(message));
//! }
//! ```
//!
//! # THE MESSAGE IS ESCAPED IN THE CONSTRUCTOR, NOT AT THE THROW SITE
//!
//! Every caller of `new IdentifierException(...)` passes plain concatenation. The escaping
//! happens here, in the exception, which means:
//!
//! * A caller that formats its own message differently still gets escaped output, and
//! * a port that panics with the concatenated string is **wrong**, not merely unpolished --
//!   `Identifier.parse("\n")` reports `location: minecraft:\n` with a literal backslash-n, and
//!   a newline in a crash log is both unreadable and ambiguous with the surrounding text.
//!
//! This is not a decompiler artifact. `minecraft-decompiled` shows the `escapeJava` call
//! correctly; it is simply easy to miss, because nothing in `Identifier.java` hints that the
//! message it builds is about to be rewritten. It was found by the golden: the port produced a
//! real newline where the jar produced `\n`, and `identifier.parseError` failed.
//!
//! # THE ESCAPING TABLE, MEASURED AGAINST THE JAR
//!
//! Not read off the commons-lang3 source -- read out of it. `p.EscapeProbe` fed every code
//! point through `Identifier.fromNamespaceAndPath` and dumped the bytes of the message the jar
//! produced. That matters because the table has two entries that a from-memory implementation
//! gets wrong:
//!
//! | input                | output        | note                                    |
//! |----------------------|---------------|-----------------------------------------|
//! | `0x08` `0x09` `0x0A` `0x0C` `0x0D` | `\b` `\t` `\n` `\f` `\r` | short forms, checked first |
//! | other `0x00..0x1F`   | `\u00XX`      | **uppercase** hex, e.g. `\u000B`, `\u001F` |
//! | `0x20..0x7F`         | unchanged     | **`0x7F` (DEL) is NOT escaped**         |
//! | `0x80..`             | `\uXXXX`      | uppercase hex, e.g. `\u0080`, `\u00FF`, `\u2028` |
//! | `"`                  | `\"`          | see the note below                      |
//! | `\`                  | `\\`          | see the note below                      |
//!
//! Two of those deserve emphasis because both are the kind of detail nobody would notice
//! missing:
//!
//! * **`0x7F` is not escaped.** The boundary is `>= 0x80`, not `> 0x7E`. DEL passes through raw.
//!   A port that wrote `c > 0x7E` would produce `\u007F` where the jar produces a raw DEL
//!   character -- invisible in a terminal, and byte-for-byte different in a log.
//! * **The hex digits are uppercase.** `\u000b` and `\u000B` are the same character to a human
//!   reading a log and different bytes on disk.
//!
//! ## The `"` and `\` rows are not covered by any golden group
//!
//! No `identifier.*` group feeds a quote or a backslash to `Identifier`, because neither is
//! reachable through any corpus entry. They are transcribed from `escapeJava`'s documented
//! table and are **unverified**. If a caller ever needs them, add a corpus entry rather than
//! trusting this comment.

/// Apache commons-lang3 `StringEscapeUtils.escapeJava`, as the jar applies it.
///
/// # WHY NOT `str::escape_debug`
///
/// Rust's own debug escaping is close but different where it matters: it escapes `\u{7f}` as
/// `\u{7f}` with braces, it lowercases nothing but formats differently, and its set of
/// "printable" is Unicode-wide rather than ASCII. None of those would match the jar byte for
/// byte.
pub fn escape_java(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '\u{8}' => out.push_str("\\b"),
            '\u{9}' => out.push_str("\\t"),
            '\u{a}' => out.push_str("\\n"),
            '\u{c}' => out.push_str("\\f"),
            '\u{d}' => out.push_str("\\r"),
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            // The short forms above must be matched BEFORE this arm, or `\n` would come out as
            // `\u000A`. Rust match arms are tried in order, which is what makes this correct.
            //
            // Note `>= 0x80` and not `> 0x7E`: DEL (0x7F) passes through unescaped, verified
            // against the jar by p.EscapeProbe.
            _ if (c as u32) < 0x20 || (c as u32) >= 0x80 => {
                out.push_str(&format!("\\u{:04X}", c as u32));
            }
            _ => out.push(c),
        }
    }
    out
}

/// Port of `new IdentifierException(String)`, and the throw site for every caller.
///
/// Panics with Java's class name and `escapeJava`'d message, per
/// `DESIGN_DECISIONS.md#runtime-exceptions`. The class name is part of the text because that
/// is what the golden groups recorded -- the oracle captured
/// `ex.getClass().getName() + ": " + ex.getMessage()`, and that concatenation is what a crash
/// log contains.
#[cold]
#[inline(never)]
pub fn throw(message: &str) -> ! {
    panic!("net.minecraft.IdentifierException: {}", escape_java(message))
}
