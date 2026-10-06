//! Port of: net/minecraft/resources/Identifier.java
//! Java class(es): net.minecraft.resources.Identifier
//! Status: PORTED
//!
//! All 21 `identifier.*` golden groups in `batch2.txt` are bit-exact.
//!
//! # THE INVARIANT, AND WHY THE STRUCT FIELDS ARE PRIVATE
//!
//! Java's only real constructor is `private Identifier(String, String)`, guarded by two
//! `assert`s -- which the JVM does **not** run unless `-ea` is passed. Every public path in
//! the class goes through `createUntrusted` or `withDefaultNamespace`, both of which validate
//! with `assertValidNamespace`/`assertValidPath` and throw `IdentifierException`.
//!
//! In Rust the equivalent guarantee is the compiler's: the fields are private, and the only
//! ways to obtain an `Identifier` are the public constructors and accessors below, all of
//! which validate. There is deliberately no `Identifier { namespace, path }` literal anywhere,
//! so an invalid identifier is not a value this type can represent. That is stricter than
//! vanilla, and deliberately so -- it is the same choice the class already makes, just
//! enforced by types instead of by convention.
//!
//! # WHERE IT THROWS, AND WHY THE MESSAGE IS NOT WHAT THE SOURCE LOOKS LIKE
//!
//! `IdentifierException` is `net.minecraft`'s own unchecked exception. Per
//! `DESIGN_DECISIONS.md#runtime-exceptions`, this port **panics with Java's exact text**,
//! class name included: `net.minecraft.IdentifierException: Non [a-z0-9/._-] character in
//! path of location: minecraft:a:b`.
//!
//! Note the two messages are not interchangeable and the difference is load-bearing:
//!
//! | method   | message prefix                                            |
//! |----------|-----------------------------------------------------------|
//! | path     | `Non [a-z0-9/._-] character in path of location: `           |
//! | namespace| `Non [a-z0-9_.-] character in namespace of identifier: `  |
//!
//! The namespace message says "identifier" and the path message says "location" -- an
//! inconsistency in vanilla's own wording, reproduced exactly. The two character classes also
//! differ: a path may contain `/`, a namespace may not.
//!
//! ## And the message is escaped, which this file's source does not mention
//!
//! Every `throw` here hands plain concatenation to `IdentifierException`, whose constructor
//! runs `StringEscapeUtils.escapeJava` over it. So a newline in a path is reported as the two
//! characters `\n`, not as a newline. Nothing in this file hints at that, which is why it is
//! written here: see `net/minecraft/IdentifierException.rs`, where the escaping table was
//! measured against the jar rather than read off the commons-lang3 source.
//!
//! # `compareTo` COMPARES `path` FIRST
//!
//! ```java
//! int result = this.path.compareTo(o.path);
//! if (result == 0) { result = this.namespace.compareTo(o.namespace); }
//! ```
//!
//! Path first, namespace only as a tiebreak. That is the opposite of what the `toString()`
//! order suggests, and it is the reason `identifier.compareTo` is written with `(ns, path)`
//! argument pairs rather than whole `"ns:path"` strings.
//!
//! # UTF-16, NOT UTF-8, FOR `compareTo`
//!
//! `String#compareTo` compares UTF-16 code units. Rust's `str::cmp` compares UTF-8 bytes, and
//! the two disagree for anything above the BMP, where a non-BMP character is one Rust `char`
//! and two Java `char`s. [`java_string_compare`] compares code units explicitly.
//!
//! The golden corpus is pure ASCII, so no group here can detect the difference. That is
//! precisely why it is written down: an ASCII-only corpus cannot tell you a comparison is
//! right, only that it has not yet been wrong.

use crate::javacompat::java_lang;

/// Java: `public static final char NAMESPACE_SEPARATOR = ':'`.
pub const NAMESPACE_SEPARATOR: char = ':';

/// Java: `public static final String DEFAULT_NAMESPACE = "minecraft"`.
pub const DEFAULT_NAMESPACE: &str = "minecraft";

/// Java: `public static final String REALMS_NAMESPACE = "realms"`.
pub const REALMS_NAMESPACE: &str = "realms";

/// Java: `public static final String ALLOWED_NAMESPACE_CHARACTERS = "[a-z0-9_.-"`.
///
/// A description of the accepted set, not a pattern that is ever compiled -- and it is
/// missing its own closing bracket in vanilla. Reproduced, not corrected.
pub const ALLOWED_NAMESPACE_CHARACTERS: &str = "[a-z0-9_.-";

/// Java: `net.minecraft.IdentifierException`, message text reproduced exactly.
///
/// Delegates to [`IdentifierException::throw`], which is where the message is escaped -- see
/// that file for the measured `escapeJava` table. The escaping is NOT done here and must not be
/// duplicated: doing it in both places would double-escape every backslash.
///
/// # WHY THE CLASS NAME IS PART OF THE MESSAGE
///
/// The golden groups record `ex.getClass().getName() + ": " + ex.getMessage()`, so the string a
/// crash log would contain is `net.minecraft.IdentifierException: ...`. Panning with anything
/// shorter would make the port's diagnostics disagree with vanilla's for no gain.
#[cold]
#[inline(never)]
fn throw_identifier_exception(message: &str) -> ! {
    crate::net::minecraft::IdentifierException::throw(message)
}

/// Port of `Identifier#assertValidNamespace(String,String)`. Private in Java.
///
/// # WHY IT RETURNS `()` HERE AND A `String` IN JAVA
///
/// Java returns the validated string, which is pure ceremony -- the returned value is always
/// the argument that was passed in, and every caller immediately puts it in a field. Rust
/// cannot express "returns one of its two `&str` arguments" without a named lifetime, and
/// adding one to express something that carries no information would be noise. The validation
/// is the whole method.
///
/// The message quotes BOTH halves, `namespace + ":" + path`, even though only the namespace
/// is at fault. That is what makes the failure diagnosable from a log line alone, so the
/// trailing `path` is not decoration.
#[inline]
fn assert_valid_namespace(namespace: &str, path: &str) {
    if !is_valid_namespace(namespace) {
        throw_identifier_exception(&format!(
            "Non [a-z0-9_.-] character in namespace of identifier: {namespace}:{path}"
        ));
    }
}

/// Port of `Identifier#assertValidPath(String,String)`. Private in Java. See the note on
/// [`assert_valid_namespace`] for why this returns `()`.
#[inline]
fn assert_valid_path(namespace: &str, path: &str) {
    if !is_valid_path(path) {
        throw_identifier_exception(&format!(
            "Non [a-z0-9/._-] character in path of location: {namespace}:{path}"
        ));
    }
}

/// Port of `Identifier#validNamespaceChar(char)`. Private in Java.
///
/// No `/` -- a namespace is a single path segment, so a slash would let an identifier name a
/// directory it has no business naming.
#[inline]
fn valid_namespace_char(c: char) -> bool {
    c == '_' || c == '-' || c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.'
}

/// Port of `Identifier#validPathChar(char)`.
#[inline]
pub fn valid_path_char(c: char) -> bool {
    c == '_' || c == '-' || c.is_ascii_lowercase() || c.is_ascii_digit() || c == '/' || c == '.'
}

/// Port of `Identifier#isAllowedInIdentifier(char)`.
///
/// The **loosest** of the three character predicates, and the only one that admits `:` -- which
/// is what lets a command argument scanner find a whole `namespace:path` in one pass. It is not
/// a validity check and nothing may treat it as one.
#[inline]
pub fn is_allowed_in_identifier(c: char) -> bool {
    c.is_ascii_digit()
        || c.is_ascii_lowercase()
        || c == '_'
        || c == ':'
        || c == '/'
        || c == '.'
        || c == '-'
}

/// Port of `Identifier#isValidPath(String)`.
///
/// The empty string is **valid**: `isValidPath("")` is `true`, so `Identifier.parse("a:")`
/// yields the identifier `a:` with an empty path, and `Identifier.parse("")` yields
/// `minecraft:`. Both are accepted by vanilla and both are in the golden corpus.
///
/// Note `charAt` is UTF-16 in Java while this iterates `char`s, so a non-BMP character is one
/// `char` here and two there. Both are rejected either way, so the boolean is unchanged; the
/// difference is noted only because it would matter if the predicate ever became lenient.
#[inline]
pub fn is_valid_path(path: &str) -> bool {
    path.chars().all(valid_path_char)
}

/// Port of `Identifier#isValidNamespace(String)`.
///
/// The `..` check is **separate from the character loop** and comes first. A namespace of
/// exactly `..` consists entirely of legal characters, so without this it would validate --
/// and `Identifier.parse("..")` would then resolve to a parent directory. It does not, because
/// of this line. `Identifier.parse("..")` returns `minecraft:..`, i.e. a *path* of `..`, which
/// is legal: `isValidPath` has no such check.
#[inline]
pub fn is_valid_namespace(namespace: &str) -> bool {
    if namespace == ".." {
        return false;
    }
    namespace.chars().all(valid_namespace_char)
}

/// Port of `Identifier#createUntrusted(String,String)`. Private in Java.
#[inline]
fn create_untrusted(namespace: &str, path: &str) -> Identifier {
    assert_valid_namespace(namespace, path);
    assert_valid_path(namespace, path);
    Identifier {
        namespace: namespace.to_string(),
        path: path.to_string(),
    }
}

/// Port of `net.minecraft.resources.Identifier`.
///
/// `namespace` and `path` are private so that the validation in the constructors above cannot
/// be bypassed; use [`Identifier::get_namespace`] and [`Identifier::get_path`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    namespace: String,
    path: String,
}

/// Java: `public static Identifier fromNamespaceAndPath(String,String)`.
#[inline]
pub fn from_namespace_and_path(namespace: &str, path: &str) -> Identifier {
    create_untrusted(namespace, path)
}

/// Java: `public static Identifier withDefaultNamespace(String)`.
///
/// The namespace is the literal `"minecraft"`, not `DEFAULT_NAMESPACE` read at runtime -- the
/// constant is `final` so they cannot diverge, but the bytecode inlines the literal and that
/// is what is transcribed.
#[inline]
pub fn with_default_namespace(path: &str) -> Identifier {
    assert_valid_path(DEFAULT_NAMESPACE, path);
    Identifier {
        namespace: DEFAULT_NAMESPACE.to_string(),
        path: path.to_string(),
    }
}

/// Port of `Identifier#bySeparator(String,char)`.
///
/// Three shapes, and the middle one is the only one that treats the text before the separator
/// as a namespace:
///
/// * separator absent  -> the whole string is the path, namespace defaults
/// * separator at 0    -> the whole remainder is the path, namespace defaults
/// * separator at n>0  -> `[0,n)` is the namespace, the remainder is the path
///
/// A separator at index 0 means "default namespace", which is why `":stone"` and `"stone"`
/// produce the *same* identifier -- and why `":x:y"` does not, since its path is `"x:y"` and
/// `:` is not a valid path character.
#[inline]
pub fn by_separator(identifier: &str, separator: char) -> Identifier {
    match identifier.find(separator) {
        Some(separator_index) => {
            let path = &identifier[separator_index + separator.len_utf8()..];
            if separator_index != 0 {
                create_untrusted(&identifier[..separator_index], path)
            } else {
                with_default_namespace(path)
            }
        }
        None => with_default_namespace(identifier),
    }
}

/// Port of `Identifier#tryBySeparator(String,char)`.
///
/// `None` where `by_separator` would throw, and it is **not** `try_by_separator` implemented as
/// `by_separator(...).ok()`. The validation order differs: this checks the *path* first and
/// returns `None` before it ever looks at the namespace, so it never constructs the invalid
/// intermediate a throwing implementation would. Same observable result, different work --
/// and worth stating because the two are easy to conflate and a naive port would be right
/// only by accident.
#[inline]
pub fn try_by_separator(identifier: &str, separator: char) -> Option<Identifier> {
    match identifier.find(separator) {
        Some(separator_index) => {
            let path = &identifier[separator_index + separator.len_utf8()..];
            if !is_valid_path(path) {
                None
            } else if separator_index != 0 {
                let namespace = &identifier[..separator_index];
                if is_valid_namespace(namespace) {
                    Some(Identifier {
                        namespace: namespace.to_string(),
                        path: path.to_string(),
                    })
                } else {
                    None
                }
            } else {
                Some(Identifier {
                    namespace: DEFAULT_NAMESPACE.to_string(),
                    path: path.to_string(),
                })
            }
        }
        None => {
            if is_valid_path(identifier) {
                Some(Identifier {
                    namespace: DEFAULT_NAMESPACE.to_string(),
                    path: identifier.to_string(),
                })
            } else {
                None
            }
        }
    }
}

/// Java: `public static Identifier parse(String)`.
#[inline]
pub fn parse(identifier: &str) -> Identifier {
    by_separator(identifier, NAMESPACE_SEPARATOR)
}

/// Java: `public static @Nullable Identifier tryParse(String)`.
#[inline]
pub fn try_parse(identifier: &str) -> Option<Identifier> {
    try_by_separator(identifier, NAMESPACE_SEPARATOR)
}

/// Java: `public static @Nullable Identifier tryBuild(String,String)`.
///
/// Unlike `try_by_separator`, this validates the namespace **before** the path, so for inputs
/// where both are invalid the two methods report differently. The golden corpus
/// (`identifier.tryBuild`) covers exactly this: namespace `"a/b"` with path `"A"` reports the
/// namespace as the problem.
#[inline]
pub fn try_build(namespace: &str, path: &str) -> Option<Identifier> {
    if is_valid_namespace(namespace) && is_valid_path(path) {
        Some(Identifier {
            namespace: namespace.to_string(),
            path: path.to_string(),
        })
    } else {
        None
    }
}

impl Identifier {
    /// Port of `Identifier#getPath()`.
    #[inline]
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Port of `Identifier#getNamespace()`.
    #[inline]
    pub fn get_namespace(&self) -> &str {
        &self.namespace
    }

    /// Port of `Identifier#hashCode()`: `31 * namespace.hashCode() + path.hashCode()`.
    ///
    /// Java's `String#hashCode` is `s[0]*31^(n-1) + ... + s[0]`, over UTF-16 code units --
    /// not Rust's `str` hash, which is neither 31-based nor stable across releases.
    #[inline]
    pub fn java_hash_code(&self) -> i32 {
        31i32.wrapping_mul(java_lang::string_hash_code(&self.namespace))
            .wrapping_add(java_lang::string_hash_code(&self.path))
    }

    /// Port of `Identifier#compareTo(Identifier)`. Path first, namespace as tiebreak.
    #[inline]
    pub fn compare_to(&self, other: &Identifier) -> std::cmp::Ordering {
        java_string_compare(&self.path, &other.path)
            .then_with(|| java_string_compare(&self.namespace, &other.namespace))
    }

    /// Port of `Identifier#withPath(String)`.
    ///
    /// Keeps this identifier's namespace and replaces the path. The namespace is passed to the
    /// validator, so an error message quotes the namespace the caller already had.
    #[inline]
    pub fn with_path(&self, new_path: &str) -> Identifier {
        assert_valid_path(&self.namespace, new_path);
        Identifier {
            namespace: self.namespace.clone(),
            path: new_path.to_string(),
        }
    }

    /// Port of `Identifier#withPath(UnaryOperator<String>)`.
    ///
    /// Java takes a function object; this takes a closure. The modifier runs **before**
    /// validation and its result is what gets validated, so a modifier that returns something
    /// invalid throws from here rather than from the caller's own construction.
    #[inline]
    pub fn with_path_modifier<F>(&self, modifier: F) -> Identifier
    where
        F: FnOnce(&str) -> String,
    {
        self.with_path(&modifier(&self.path))
    }

    /// Port of `Identifier#withPrefix(String)`: prefix the PATH, not the namespace.
    ///
    /// The concatenation happens before validation, so `"minecraft" + "A/B"` is rejected even
    /// though `A/B` contains a legal path separator.
    #[inline]
    pub fn with_prefix(&self, prefix: &str) -> Identifier {
        self.with_path(&format!("{prefix}{}", self.path))
    }

    /// Port of `Identifier#withSuffix(String)`: suffix the PATH.
    #[inline]
    pub fn with_suffix(&self, suffix: &str) -> Identifier {
        self.with_path(&format!("{}{suffix}", self.path))
    }

    /// Port of `Identifier#toDebugFileName()`: `/` and `:` both become `_`.
    ///
    /// `String#replace` replaces **every** occurrence, and the order matters only in that the
    /// second pass sees the first pass's output -- which it cannot disturb, since `_` is not a
    /// separator.
    #[inline]
    pub fn to_debug_file_name(&self) -> String {
        self.to_string().replace('/', "_").replace(':', "_")
    }

    /// Port of `Identifier#toLanguageKey()`: `namespace + "." + path`.
    ///
    /// Note the path's own `/` survive, so `minecraft:a/b` keys as `minecraft.a/b` -- a dot
    /// separator with slashes inside, which is what translation keys actually look like.
    #[inline]
    pub fn to_language_key(&self) -> String {
        format!("{}.{}", self.namespace, self.path)
    }

    /// Port of `Identifier#toShortLanguageKey()`: drop the namespace when it is `minecraft`.
    #[inline]
    pub fn to_short_language_key(&self) -> String {
        if self.namespace == DEFAULT_NAMESPACE {
            self.path.clone()
        } else {
            self.to_language_key()
        }
    }

    /// Port of `Identifier#toShortString()`: drop the namespace when it is `minecraft`.
    ///
    /// Differs from [`Identifier::to_short_language_key`] in exactly one character: `.`
    /// versus `:`. Both drop the namespace; they are not interchangeable, and using one where
    /// the other is meant produces a string that looks right and parses as an identifier.
    #[inline]
    pub fn to_short_string(&self) -> String {
        if self.namespace == DEFAULT_NAMESPACE {
            self.path.clone()
        } else {
            self.to_string()
        }
    }

    /// Port of `Identifier#toLanguageKey(String prefix)`: `prefix + "." + toLanguageKey()`.
    #[inline]
    pub fn to_language_key_with_prefix(&self, prefix: &str) -> String {
        format!("{prefix}.{}", self.to_language_key())
    }

    /// Port of `Identifier#toLanguageKey(String prefix, String suffix)`.
    ///
    /// The separator between the language key and the suffix is a **dot**, with no dot after
    /// the prefix's own dot -- so `("pre", "suf")` on `minecraft:a/b/c` gives
    /// `pre.minecraft.a/b/c.suf`.
    #[inline]
    pub fn to_language_key_with_prefix_and_suffix(&self, prefix: &str, suffix: &str) -> String {
        format!("{prefix}.{}.{suffix}", self.to_language_key())
    }

    /// Port of `Identifier#resolveAgainst(Path)`.
    ///
    /// `todo!("PORT-BLOCKED: java.nio.file.Path is not ported")`. Needs a real path type with
    /// `resolve(String,String)`, `normalize()` and `startsWith(Path)`, and this method's whole
    /// job is the containment check -- returning a `String` and calling it a port would skip
    /// the check that makes the method safe.
    pub fn resolve_against(&self, root: &str) -> String {
        let _ = root;
        todo!("PORT-BLOCKED: java.nio.file.Path is not ported")
    }
}

impl std::fmt::Display for Identifier {
    /// Port of `Identifier#toString()`: `namespace + ":" + path`.
    ///
    /// This is **not** [`Identifier::to_short_string`], and the difference is visible in the
    /// golden: `minecraft:stone` prints as `minecraft:stone` here and as `stone` there.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

/// `String#compareTo`: lexicographic over **UTF-16 code units**, not UTF-8 bytes.
///
/// Returns only the ordering, because `String#compareTo`'s exact return value is
/// implementation-defined beyond its sign, and every caller in `Identifier` (and in
/// `identifier.compareTo`'s golden group) goes through `Integer.signum`.
fn java_string_compare(a: &str, b: &str) -> std::cmp::Ordering {
    // Fast path: for ASCII-only strings -- which is every identifier vanilla accepts -- UTF-8
    // byte order and UTF-16 code-unit order agree, and no allocation is needed.
    if a.is_ascii() && b.is_ascii() {
        return a.as_bytes().cmp(b.as_bytes());
    }
    a.encode_utf16().cmp(b.encode_utf16())
}

// ============================================================================
// Blocked on types that are not ported
// ============================================================================
//
// These are the honest gaps. Each names its dependency rather than returning a placeholder,
// because a placeholder here would be silently wrong: every one of them either validates input
// or drives a network/disk format, and both are places a stub would hide a real desync.

/// Java: `public static DataResult<Identifier> read(String)`.
///
/// `todo!("PORT-BLOCKED: com.mojang.serialization.DataResult is not ported")`. This is a
/// `Codec` entry point: it converts a `String` into an identifier and, on failure, into a
/// `DataResult` carrying the message `"Not a valid resource location: " + input + " " +
/// ex.getMessage()`. That message composition is the part worth preserving when DFU lands,
/// and it is why this cannot be faked with an `Option`.
pub fn read_string(input: &str) -> Result<Identifier, String> {
    let _ = input;
    todo!("PORT-BLOCKED: com.mojang.serialization.DataResult is not ported")
}

/// Java: `private static String readGreedy(StringReader)`.
///
/// `todo!("PORT-BLOCKED: com.mojang.brigadier.StringReader is not ported")`. Consumes the
/// longest run of `isAllowedInIdentifier` characters from the reader's cursor. The predicate is
/// ported ([`is_allowed_in_identifier`]); only the cursor is missing.
pub fn read_greedy(_reader: &mut ()) -> String {
    todo!("PORT-BLOCKED: com.mojang.brigadier.StringReader is not ported")
}

/// Java: `public static Identifier read(StringReader) throws CommandSyntaxException`.
///
/// `todo!("PORT-BLOCKED: com.mojang.brigadier.StringReader is not ported")`. Worth noting for
/// whoever unblocks it: on failure this **resets the cursor to where it started** before
/// throwing `ERROR_INVALID`, so the caller can report the error at the token's beginning rather
/// than wherever the greedy scan stopped.
pub fn read(reader: &mut ()) -> Identifier {
    let _ = reader;
    todo!("PORT-BLOCKED: com.mojang.brigadier.StringReader is not ported")
}

/// Java: `public static Identifier readNonEmpty(StringReader) throws CommandSyntaxException`.
///
/// As `read`, plus a rejection of an empty result -- checked **before** `parse`, so an empty
/// argument fails with `ERROR_INVALID` and not with an identifier exception.
pub fn read_non_empty(reader: &mut ()) -> Identifier {
    let _ = reader;
    todo!("PORT-BLOCKED: com.mojang.brigadier.StringReader is not ported")
}
