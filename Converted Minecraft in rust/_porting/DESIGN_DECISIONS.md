# Design decisions — Rust port of Minecraft 26.2

Every entry records a decision that a future reader could plausibly "clean up" and
break world parity. If you change one of these, re-run `cargo test` **and** check
that the corresponding golden transcript still passes.

Format: **what** → **why** → **what would break if we did the obvious thing**.

---

## `mirror` — one `.java` file becomes exactly one `.rs` file

The whole crate is a filesystem mirror of `minecraft-decompiled/`: same folders,
same base names (`Mth.java` → `Mth.rs`), inner and anonymous classes kept inside
their outer file. `net/minecraft/util/Mth.java` → `net/minecraft/util/Mth.rs`.

**Why:** it makes a diff trivial to review line-by-line against vanilla, and it
makes `_porting/tools/check_mirror.py` a complete structural guarantee rather than
a convention.

**Cost:** Rust module paths get deeper than the Java ones — `Mth` lives at
`net::minecraft::util::Mth::Mth` (module `Mth`, struct `Mth`). Every ported file
therefore has a comment explaining the `use ...::Mth::Mth;` shape. This is
deliberate and is not worth "fixing".

---

## `javacompat` — Java stdlib semantics live outside the mirror

`Math.floorDiv`, `String.hashCode`, `java.util.Random`, MD5, and the
narrowing-conversion rules have no `.java` file in `minecraft-decompiled/`, so per
Rule 3 they live in `_porting/javacompat/` and are pulled in by `lib.rs` with
`#[path]`.

**Why:** the alternative — scattering hand-rolled `floor_mod` helpers through 7000
files — guarantees that at least one of them uses Rust semantics instead of Java's.

**What's in there** (each has a doc table of where Java and Rust disagree):

| module | why it exists |
|---|---|
| `java_lang` | `floor_div/floor_mod`, `abs` (incl. `MIN_VALUE`), NaN-propagating `min`/`max`, NaN-sign-correct arithmetic, `(byte)float` narrowing, `String.hashCode` over UTF-16 |
| `java_random` | the 48-bit LCG + `nextFloat`/`nextDouble` scaling |
| `md5` | RFC-1321; `RandomSupport#seedFromHashOf` hashes with it, and it has no crate deps |
| `golden` | reads `_porting/test-data/*.txt` and asserts on **raw IEEE bits** |

---

## `stubs` — the oracle compiles vanilla against minimal stubs

`_porting/java-oracle/` compiles the ORIGINAL, unmodified vanilla sources against
real `joml` + `commons-lang3` and 17 hand-written stubs for Guava, the DFU Codec
API, jspecify, netty's `ThreadLocalRandom`, and a few tiny value types.

**Why:** a stub may only supply types the ported classes merely *touch*. Every stub
file says so at the top, and each one either is bit-identical by specification
(`MessageDigest("MD5")` is pinned to RFC 1321 by the JCA, so it equals Guava's) or
sits on a path the oracle never exercises (the DFU `Codec` static initialisers).

`run.ps1` prints the `CodeSource` of every ported class so you can confirm the
oracle linked against vanilla rather than a reimplementation.

---

## `embedded-trig-tables` — Mth's sin/asin/cos tables are embedded, not computed

`Mth::SIN` (65 536 `f32`), `Mth::ASIN_TAB` and `Mth::COS_TAB` (257 `f64` each) are
spliced in verbatim from the oracle by `_porting/tools/gen_tables.py`.

**Why not call `f64::sin`:** Java's `Math.sin` is allowed a 1-ulp error and the exact
value depends on JVM internals (fdlibm vs. an x86 intrinsic). Rust's `f64::sin`
links whatever libm the OS has. Across 65 536 entries a 1-ulp double difference can
still flip the rounded `f32` result for a handful of indices — and one flipped entry
changes every `sin`/`cos` call that lands on it, i.e. block rotation, entity yaw,
camera angles.

**Enforcement:** `sin_lookup_table_matches_bit_for_bit` hashes all 65 536 raw bits
with FNV-1a and compares against a golden hash, so a single wrong entry fails.

---

## `panic-not-Result` — where Java throws, we panic

Vanilla never *catches* the exceptions these paths would raise
(`IllegalArgumentException` for `nextInt(bound <= 0)`, `Math.pow`, …), so a throw is
a crash in Java too. Panicking keeps the signatures clean; converting them all to
`Result` would infect the whole call graph for no behavioural gain.

**But:** where Java *does* catch-and-continue, the Rust port MUST use `Result` and
reproduce the same recovery. That rule applies per-method and is flagged in each
port once we reach the I/O and network layers.

---

## `no-self-referential-structs` — `MarsagliaPolarGaussian` takes its source as a parameter

Java's `MarsagliaPolarGaussian` holds `public final RandomSource randomSource`, and
every construction site passes `this` — a self-reference Rust cannot express.

**What we did:** `next_gaussian(&mut self, random_source: &mut dyn RandomSource)`.
`LegacyRandomSource` moves the cached pair into a local so `self` can be borrowed
at the same time.

**Observable difference:** the field is no longer publicly readable. Nothing in
vanilla reads it, so nothing can observe the change.

---

## `threading-detector` — AtomicLong + CAS becomes `&mut self`

`LegacyRandomSource` and `ThreadSafeLegacyRandomSource` guard their 48-bit state with
`AtomicLong` and throw via `ThreadingDetector` if two threads touch it. Rust's
exclusive `&mut` borrow gives the same guarantee with no runtime cost, so the CAS
loop and the exception have no counterpart.

**Consequence:** these types are no longer `Sync`, so they cannot be shared across
threads without a lock. Vanilla's chunk generator threads already take ownership per
chunk, so this matches — but it is a real constraint on the future port.

---

## `instanceof` — `as_bit_source()`

`WorldgenRandom#next(int)` does
`this.randomSource instanceof LegacyRandomSource l ? l.next(bits) : (int)(this.randomSource.nextLong() >>> 64 - bits)`.

Rust has no `instanceof`, so `RandomSource` has one extra method,
`as_bit_source() -> Option<&mut dyn BitRandomSource>`, defaulting to `None` and
overridden by the three LCG types. It mirrors the Java dispatch literally — do not
collapse it into a "always use nextLong" shortcut, which would silently change
legacy worldgen.

---

## `overload-naming` — `<name>_<type>` only when Java overloads

Java overloads become distinct Rust names: `Mth.floor(float)` → `floor_f32`,
`Mth.floor(double)` → `floor_f64`, `Mth.abs(int)` → `abs_i32`. Un-overloaded methods
keep the plain snake_case name. Every item carries a
`/// Port of Class#method(paramTypes)` line so the files diff by eye.

---

## Golden data records RAW BITS, never decimals

`f32`/`f64` are stored as `0x%08x` / `0x%016x` of `floatToRawIntBits` /
`doubleToRawLongBits`. Tests compare with `to_bits()`.

**Why:** a decimal round-trip would erase `-0.0` vs `+0.0`, subnormals, and NaN
payloads — all of which turned out to be real divergences here.

**The trap we hit:** probing JVM behaviour with `javac` constants is unreliable,
because `Float.POSITIVE_INFINITY % 360.0F` is a *compile-time constant* and javac
folds it with its own arithmetic. It reported a positive NaN; HotSpot at runtime
raises the negative "real indefinite". Every JVM fact recorded here was measured
through a `volatile` or an argument to defeat constant folding.

---

## Transcript semantics in the tests

For stateful objects, a golden row's expected value depends on all draws before it.
The oracle's loop nesting is reproduced in the tests (restart the source when the
seed changes, otherwise keep advancing it), so a row is reproducible from its
position. Creating a fresh source per row would pass some values and fail others
for reasons unrelated to the port.

---

## `stubs`/`jar` versions

The oracle pins `joml-1.10.8.jar` and `commons-lang3-3.17.0.jar`, fetched by
`fetch_libs.ps1` and committed under `_porting/java-oracle/lib/` so it runs offline.
joml's `Math.invsqrt` is version-specific, so pinning matters. The exact versions
Minecraft 26.2 ships are **unverified** — see `OPEN_QUESTIONS.md` #5.