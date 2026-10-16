//! Port of: net/minecraft/ChatFormatting.java
//! Java class(es): net.minecraft.ChatFormatting
//! Status: VERIFIED
//!
//! Session 19. Every member of the 26.2 class is ported: the 22 constants with their `code`
//! and cached `toString` fields, `PREFIX_CODE`, `toString()`, `stripFormatting` (the
//! `STRIP_FORMATTING_PATTERN` regex, matched by hand) and `getByCode`. Ordinals and both fields
//! are checked against the jar in `_porting/tests/session19_b.rs`.
//!
//! The 26.2 class is much smaller than older ones: there is no `getByName`, `getById`,
//! `getNames`, colour value, `isFormat`/`isColor` or `cleanName` any more. Nothing is ported
//! that the source does not have.
//!
//! # `toString()` IS THE LEGACY CODE, NOT THE NAME
//!
//! `BLACK.toString()` is `"§0"` (U+00A7 then the code char), so [`std::fmt::Display`] prints
//! that. Java's `name()` (`"BLACK"`) is [`ChatFormatting::name`]. Anything that formats a
//! `ChatFormatting` by string concatenation in Java gets the section-sign form.
//!
//! # `stripFormatting` FOLDS CASE FOR ASCII ONLY
//!
//! The pattern is `(?i)§[0-9A-FK-OR]` without `UNICODE_CASE`, so `§A` and `§a` both match but
//! `§` followed by U+212A KELVIN SIGN does not. The scan is Java's `replaceAll`: left to right,
//! non-overlapping, one pass, so `"§§aa"` becomes `"§a"`.
//!
//! # `getByCode` FOLDS CASE WITH FULL UNICODE RULES
//!
//! `Character.toLowerCase(char)` is the Unicode simple lowercase mapping, so `getByCode('A')` is
//! `GREEN` and `getByCode('K')` (KELVIN SIGN, lowercase `k`) is `OBFUSCATED`, even though
//! `stripFormatting` would leave `§K` in place.

use std::fmt;

/// Port of `net.minecraft.ChatFormatting`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChatFormatting {
    /// Port of `ChatFormatting#BLACK` -- ordinal 0, code `'0'`.
    Black = 0,
    /// Port of `ChatFormatting#DARK_BLUE` -- ordinal 1, code `'1'`.
    DarkBlue = 1,
    /// Port of `ChatFormatting#DARK_GREEN` -- ordinal 2, code `'2'`.
    DarkGreen = 2,
    /// Port of `ChatFormatting#DARK_AQUA` -- ordinal 3, code `'3'`.
    DarkAqua = 3,
    /// Port of `ChatFormatting#DARK_RED` -- ordinal 4, code `'4'`.
    DarkRed = 4,
    /// Port of `ChatFormatting#DARK_PURPLE` -- ordinal 5, code `'5'`.
    DarkPurple = 5,
    /// Port of `ChatFormatting#GOLD` -- ordinal 6, code `'6'`.
    Gold = 6,
    /// Port of `ChatFormatting#GRAY` -- ordinal 7, code `'7'`.
    Gray = 7,
    /// Port of `ChatFormatting#DARK_GRAY` -- ordinal 8, code `'8'`.
    DarkGray = 8,
    /// Port of `ChatFormatting#BLUE` -- ordinal 9, code `'9'`.
    Blue = 9,
    /// Port of `ChatFormatting#GREEN` -- ordinal 10, code `'a'`.
    Green = 10,
    /// Port of `ChatFormatting#AQUA` -- ordinal 11, code `'b'`.
    Aqua = 11,
    /// Port of `ChatFormatting#RED` -- ordinal 12, code `'c'`.
    Red = 12,
    /// Port of `ChatFormatting#LIGHT_PURPLE` -- ordinal 13, code `'d'`.
    LightPurple = 13,
    /// Port of `ChatFormatting#YELLOW` -- ordinal 14, code `'e'`.
    Yellow = 14,
    /// Port of `ChatFormatting#WHITE` -- ordinal 15, code `'f'`.
    White = 15,
    /// Port of `ChatFormatting#OBFUSCATED` -- ordinal 16, code `'k'`.
    Obfuscated = 16,
    /// Port of `ChatFormatting#BOLD` -- ordinal 17, code `'l'`.
    Bold = 17,
    /// Port of `ChatFormatting#STRIKETHROUGH` -- ordinal 18, code `'m'`.
    Strikethrough = 18,
    /// Port of `ChatFormatting#UNDERLINE` -- ordinal 19, code `'n'`.
    Underline = 19,
    /// Port of `ChatFormatting#ITALIC` -- ordinal 20, code `'o'`.
    Italic = 20,
    /// Port of `ChatFormatting#RESET` -- ordinal 21, code `'r'`.
    Reset = 21,
}

/// Java: `ChatFormatting.values()` -- declaration order, which is the ordinal order.
pub const VALUES: [ChatFormatting; 22] = [
    ChatFormatting::Black,
    ChatFormatting::DarkBlue,
    ChatFormatting::DarkGreen,
    ChatFormatting::DarkAqua,
    ChatFormatting::DarkRed,
    ChatFormatting::DarkPurple,
    ChatFormatting::Gold,
    ChatFormatting::Gray,
    ChatFormatting::DarkGray,
    ChatFormatting::Blue,
    ChatFormatting::Green,
    ChatFormatting::Aqua,
    ChatFormatting::Red,
    ChatFormatting::LightPurple,
    ChatFormatting::Yellow,
    ChatFormatting::White,
    ChatFormatting::Obfuscated,
    ChatFormatting::Bold,
    ChatFormatting::Strikethrough,
    ChatFormatting::Underline,
    ChatFormatting::Italic,
    ChatFormatting::Reset,
];

/// Java: `public static final char PREFIX_CODE = '§'` (U+00A7 SECTION SIGN).
pub const PREFIX_CODE: char = '\u{00A7}';

impl ChatFormatting {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ChatFormatting::Black => "BLACK",
            ChatFormatting::DarkBlue => "DARK_BLUE",
            ChatFormatting::DarkGreen => "DARK_GREEN",
            ChatFormatting::DarkAqua => "DARK_AQUA",
            ChatFormatting::DarkRed => "DARK_RED",
            ChatFormatting::DarkPurple => "DARK_PURPLE",
            ChatFormatting::Gold => "GOLD",
            ChatFormatting::Gray => "GRAY",
            ChatFormatting::DarkGray => "DARK_GRAY",
            ChatFormatting::Blue => "BLUE",
            ChatFormatting::Green => "GREEN",
            ChatFormatting::Aqua => "AQUA",
            ChatFormatting::Red => "RED",
            ChatFormatting::LightPurple => "LIGHT_PURPLE",
            ChatFormatting::Yellow => "YELLOW",
            ChatFormatting::White => "WHITE",
            ChatFormatting::Obfuscated => "OBFUSCATED",
            ChatFormatting::Bold => "BOLD",
            ChatFormatting::Strikethrough => "STRIKETHROUGH",
            ChatFormatting::Underline => "UNDERLINE",
            ChatFormatting::Italic => "ITALIC",
            ChatFormatting::Reset => "RESET",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// The private `code` field (Java has no getter; exposed read-only for parity tests).
    pub const fn code(&self) -> char {
        match self {
            ChatFormatting::Black => '0',
            ChatFormatting::DarkBlue => '1',
            ChatFormatting::DarkGreen => '2',
            ChatFormatting::DarkAqua => '3',
            ChatFormatting::DarkRed => '4',
            ChatFormatting::DarkPurple => '5',
            ChatFormatting::Gold => '6',
            ChatFormatting::Gray => '7',
            ChatFormatting::DarkGray => '8',
            ChatFormatting::Blue => '9',
            ChatFormatting::Green => 'a',
            ChatFormatting::Aqua => 'b',
            ChatFormatting::Red => 'c',
            ChatFormatting::LightPurple => 'd',
            ChatFormatting::Yellow => 'e',
            ChatFormatting::White => 'f',
            ChatFormatting::Obfuscated => 'k',
            ChatFormatting::Bold => 'l',
            ChatFormatting::Strikethrough => 'm',
            ChatFormatting::Underline => 'n',
            ChatFormatting::Italic => 'o',
            ChatFormatting::Reset => 'r',
        }
    }

    /// The private `toString` field, `"§" + code`, cached in the constructor.
    const fn to_string_field(&self) -> &'static str {
        match self {
            ChatFormatting::Black => "\u{00A7}0",
            ChatFormatting::DarkBlue => "\u{00A7}1",
            ChatFormatting::DarkGreen => "\u{00A7}2",
            ChatFormatting::DarkAqua => "\u{00A7}3",
            ChatFormatting::DarkRed => "\u{00A7}4",
            ChatFormatting::DarkPurple => "\u{00A7}5",
            ChatFormatting::Gold => "\u{00A7}6",
            ChatFormatting::Gray => "\u{00A7}7",
            ChatFormatting::DarkGray => "\u{00A7}8",
            ChatFormatting::Blue => "\u{00A7}9",
            ChatFormatting::Green => "\u{00A7}a",
            ChatFormatting::Aqua => "\u{00A7}b",
            ChatFormatting::Red => "\u{00A7}c",
            ChatFormatting::LightPurple => "\u{00A7}d",
            ChatFormatting::Yellow => "\u{00A7}e",
            ChatFormatting::White => "\u{00A7}f",
            ChatFormatting::Obfuscated => "\u{00A7}k",
            ChatFormatting::Bold => "\u{00A7}l",
            ChatFormatting::Strikethrough => "\u{00A7}m",
            ChatFormatting::Underline => "\u{00A7}n",
            ChatFormatting::Italic => "\u{00A7}o",
            ChatFormatting::Reset => "\u{00A7}r",
        }
    }
}

impl fmt::Display for ChatFormatting {
    /// Port of the `ChatFormatting#toString()` override: the cached `"§" + code`, not `name()`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_string_field())
    }
}

/// The character class of the private `STRIP_FORMATTING_PATTERN`, `(?i)§[0-9A-FK-OR]`.
///
/// `CASE_INSENSITIVE` without `UNICODE_CASE` folds ASCII letters only, so this is exactly
/// `0-9`, `A-F`/`a-f`, `K-O`/`k-o` and `R`/`r`.
#[inline]
const fn is_strip_formatting_code(c: char) -> bool {
    matches!(c, '0'..='9' | 'A'..='F' | 'a'..='f' | 'K'..='O' | 'k'..='o' | 'R' | 'r')
}

/// Port of `ChatFormatting#stripFormatting(String)`:
/// `STRIP_FORMATTING_PATTERN.matcher(input).replaceAll("")`, `null` in, `null` out.
///
/// Implemented as the regex engine would scan: at each position try `§` + class char, and on a
/// match drop both and continue after them; otherwise keep one char and move on. `'§'` itself
/// matches only literally (it is not ASCII, so `(?i)` does not touch it).
pub fn strip_formatting(input: Option<&str>) -> Option<String> {
    let input = input?;
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == PREFIX_CODE {
            if let Some(&next) = chars.peek() {
                if is_strip_formatting_code(next) {
                    chars.next();
                    continue;
                }
            }
        }
        out.push(c);
    }
    Some(out)
}

/// Java `Character.toLowerCase(char)`: the Unicode simple (1:1) lowercase mapping.
///
/// Rust's `char::to_lowercase` is the full mapping; it differs from the simple one only for
/// U+0130 (`"i\u{307}"` vs `'i'`), and taking the first char gives the simple result there.
#[inline]
fn java_char_to_lower_case(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

/// Port of `ChatFormatting#getByCode(char)`: lower-case the char, then the first constant (in
/// declaration order) with that code, or `None` (Java `null`).
pub fn get_by_code(code: char) -> Option<ChatFormatting> {
    let sanitized = java_char_to_lower_case(code);
    VALUES.iter().copied().find(|format| format.code() == sanitized)
}
