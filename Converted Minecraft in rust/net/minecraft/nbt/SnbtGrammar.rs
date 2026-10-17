//! Port of: net/minecraft/nbt/SnbtGrammar.java
//! Java class(es): net.minecraft.nbt.SnbtGrammar
//! Status: PARTIAL
//!
//! Session 21. **One method of 909 lines of Java.** `escapeControlCharacters(char)` is
//! ported because `StringTag.quoteAndEscape` calls it on every character of every string
//! tag, so the whole SNBT text form of `StringTag` depends on it -- and it has no
//! dependencies of its own. Verified against the `QUOTE` rows of
//! `_porting/test-data/nbt_probe.txt` by `_porting/tests/session19_g.rs`.
//!
//! Everything else in this file -- the grammar itself, `SnbtOperations`, the Brigadier
//! `StringReader` plumbing, the number suffix rules -- is **not ported** and is PORT-BLOCKED
//! on `com.mojang.brigadier`. It is not stubbed either: there are no placeholder functions
//! here, so a caller that needs the parser gets a compile error naming the missing item
//! rather than a wrong answer at runtime.
//!
//! # THE ESCAPE TABLE, AND THE BOUNDARY THAT IS EASY TO GET WRONG
//!
//! ```java
//! public static @Nullable String escapeControlCharacters(final char c) {
//!     return switch (c) {
//!         case '\b' -> "b";
//!         case '\t' -> "t";
//!         case '\n' -> "n";
//!         default -> c < ' ' ? "x" + HEX_ESCAPE.toHexDigits((byte)c) : null;
//!         case '\f' -> "f";
//!         case '\r' -> "r";
//!     };
//! }
//! ```
//!
//! (The `default` arm really is written between the cases; that is a decompiler artifact of
//! the switch's table order and changes nothing -- `default` binds last regardless.)
//!
//! | input | result | note |
//! |---|---|---|
//! | `0x08 0x09 0x0A 0x0C 0x0D` | `b t n f r` | the five named escapes |
//! | other `0x00..=0x1F` | `xNN` | **upper-case** hex, two digits, e.g. `x00`, `x1F` |
//! | `0x20..` | `None` | **`0x7F` (DEL) is NOT escaped** |
//!
//! Two traps, both pinned by oracle rows:
//!
//! * **The boundary is `c < ' '`**, i.e. `0x20`. `0x7F` passes through raw, exactly as in
//!   `IdentifierException::escape_java` -- and for the same reason it is worth writing down:
//!   DEL is invisible in a terminal, so the difference between escaping it and not is
//!   invisible until two files fail to compare.
//! * **`HexFormat.of().withUpperCase()`**, so `\x1F`, never `\x1f`. The oracle row for
//!   `\u0001\u001f` is `\\x01\\x1F`.
//!
//! The returned `String` (not a `char`) is what the caller prefixes with a backslash, so
//! `None` means "no escape", not "empty escape". Modelled as `Option<String>` rather than
//! `Option<&'static str>` because the `xNN` arm is computed.

/// Port of `SnbtGrammar#HEX_ESCAPE`: `HexFormat.of().withUpperCase()`.
///
/// Only `toHexDigits((byte) c)` is used, which is two upper-case digits, so the
/// `HexFormat` object itself is not modelled.
const HEX_ESCAPE_UPPER_CASE: bool = true;

/// Port of `SnbtGrammar#escapeControlCharacters(char)`.
///
/// Returns the escape body *without* the leading backslash, or `None` when the character
/// needs no escape. See the module docs for the table.
pub fn escape_control_characters(c: char) -> Option<String> {
    match c {
        '\u{0008}' => Some("b".to_string()),
        '\u{0009}' => Some("t".to_string()),
        '\u{000A}' => Some("n".to_string()),
        '\u{000C}' => Some("f".to_string()),
        '\u{000D}' => Some("r".to_string()),
        _ if c < ' ' => {
            debug_assert!(HEX_ESCAPE_UPPER_CASE);
            // `(byte) c` for c < 0x20 is c itself, so the two digits never need masking.
            Some(format!("x{:02X}", c as u32))
        }
        _ => None,
    }
}
