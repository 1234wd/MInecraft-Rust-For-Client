# Session log

Chronological, append-only. Newest session at the bottom. Every entry records what
was **verified**, not what was **attempted**.

Format per session: goal → what was built → what was proved → what is still wrong
or unproven → next.

---

## Session 01 — inventory and scaffolding

**Goal.** Establish what is being ported and set up the mirror.

**Built.**
- Inventoried `minecraft-decompiled/`: **7055 `.java` files, 745 988 LOC**.
- Identified the artifact: **Minecraft 26.2**, protocol 776, data version 4903,
  resource pack 88.0, data pack 107.1, Mojang official mappings (real class names
  like `net.minecraft.util.Mth`; no `class_XXXX` obfuscation).
- Determined the jar is a **Fabric Loom split-environment client jar**
  (`META-INF/MANIFEST.MF` → `Fabric-Loom-Split-Environment-Name: client`). No
  `fabric.mod.json`, so the exact Loader version is unknown (logged as an open
  question).
- Package roots are `net/` and `com/` (no `src/main/java`). `com/mojang/blaze3d`
  211 files / 22 973 LOC, `com/mojang/math` 10 / 862, `com/mojang/realmsclient`
  127 / 12 617, the rest `net/minecraft/**`.
- Largest subsystems: `world/level` 1312 files / 149 362 LOC, `world/entity`
  716 / 106 108, `client/renderer` 701 / 47 137, `client/gui` 444 / 55 744,
  `util/datafix` 396 / 34 194.

**Decisions.**
- `_porting/` lives entirely inside `Converted Minecraft in rust/_porting/` (Rule 3
  permits it there) so there is exactly one source of truth.
- `package-info.java` → `package-info.rs` with an explicit `#[path]` in `mod.rs`.
  The file keeps the Java base name; `package_info` is the identifier.
- Case-insensitive-FS collisions (`gui/Font.java` + `gui/font/`, and
  `server/dialog/Input.java` + `input/`) get explicit `#[path]` on BOTH sides,
  which bypasses rustc's E0761 ambiguity check.
- `mod.rs` declares child folders first, then sibling files.
- `.git/info/exclude` covers `target/`, `*.class`, `__pycache__/`, `Cargo.lock`,
  keeping the repo root at exactly `.git/`, `README.md`, `minecraft-decompiled/`,
  `Converted Minecraft in rust/`.

**Not done / open.**
- `gh` CLI not installed; STOP CHECK re-verified by hand (see session 02).
- JOML and commons-lang3 versions unverified (still open — question 5).

---

## Session 02 — first real ports: `Mth` + every random source

**Goal.** Turn the skeleton into a crate that builds, stand up a Java parity
oracle, and get the foundation batch (`Mth` and the RNG files) to VERIFIED.

### 2a. STOP CHECK — repo is PUBLIC

`https://api.github.com/repos/1234wd/Minecraft-Rust-For-Client` returns
`"private": false`, `"visibility": "public"`.

**Nothing was pushed.** All commits are local on `main`. **The owner must set the
repo to private before anything is pushed.**

### 2b. Infrastructure

- **`_porting/tools/mirror.py`** — generates the mirror from the Java tree. Idempotent;
  `--check` reports drift. Owns `Cargo.toml`, `lib.rs`, every `mod.rs`, and
  `MANIFEST.csv`. It reads each file's `//! Status:` line, so the manifest always
  reflects what is actually on disk.
- **`_porting/tools/check_mirror.py`** — the structural gate: every `.java` has a
  `.rs`, no orphan `.rs`, and `minecraft-decompiled/` is byte-identical to the
  recorded git baseline (`0f2f46b5aa0e`).
- **Mirror**: 7055 `.rs` skeletons + 552 `mod.rs`. Clean `cargo build` in **3.6 s**.

### 2c. The Java oracle

`_porting/java-oracle/` compiles the **original, unmodified** vanilla sources and
prints them out as golden rows.

- 13 vanilla files compiled with `-implicit:none` and an explicit file list, so a
  stray recompile cannot silently change behaviour.
- Real jars for `joml` 1.10.8 and `commons-lang3` 3.17.0 (committed under `lib/`
  so it runs offline).
- **17 hand-written stubs** for Guava `Hashing`/`Longs`, the DataFixerUpper
  `Codec`/`DataResult` API, jspecify `@Nullable`, netty's `ThreadLocalRandom`, and
  a handful of tiny MC value types (`Vec3i`, `BlockPos`, `AABB`, `Vec3`, `Util`,
  `ARGB`, `ThreadingDetector`, `Identifier`). Each stub file states at the top that
  it may only supply types the ported classes merely *touch*.
- Output: `_porting/test-data/{mth.txt (10 149 rows), mth_tables.txt (66 059),
  random.txt (39 698)}` ≈ **49 847 golden rows**, floats stored as raw IEEE bits.

Stub audit: of the stubs on parity-tested paths, `Hashing.md5()` ≡
`MessageDigest("MD5")` because the JCA pins MD5 to RFC 1321; `Longs.fromBytes` is
little-endian; `ARGB.color`, `Util.make`, and the `Vec3i`/`Vec3`/`AABB` accessors
are trivially checkable. `Codec`, `DataResult`, `Identifier`, and
`ThreadLocalRandom` sit on **no** tested path and are documented as such.

### 2d. Ports, and the bugs the oracle caught

13 files marked `VERIFIED`. The parity harness found **nine real bugs** that would
each have silently desynced worlds. Every one is now pinned by a test:

1. **`nextDouble` carries 24 bits, not 53.** `BitRandomSource#nextDouble` is
   `combined * 1.110223E-16F` — a `long` times a **float literal**. Java's binary
   numeric promotion widens `long → float` *first*, so the 53-bit value is squeezed
   into 24 mantissa bits before scaling. `XoroshiroRandomSource#nextDouble` has the
   same quirk. The "obvious" `(combined as f64) * 2f64.powi(-53)` gives a different
   stream and desyncs every legacy world, while still *looking* random.
2. **`WorldgenRandom#setDecorationSeed` is `(a + b) ^ seed`, not `a ^ b ^ seed`.**
   `*` and `+` bind tighter than `^`. `setLargeFeatureSeed` right next to it really
   *is* a three-way xor — the pair invites confusion.
3. **`WorldgenRandom#seedSlimeChunk` mixes int and long arithmetic term by term.**
   `z * z * 4392871L` is the only long multiply; the other three terms wrap at 32
   bits.
4. **`setDecorationSeed`/`setLargeFeatureSeed` must advance the call counter.**
   Java's `this.nextLong()` resolves to `WorldgenRandom`'s own inherited method,
   which goes through its `next(int)` override. Calling the wrapped source directly
   skips `count`, which feature code reads.
5. **`Mth#getSeed`** — `x * 3129871` is `int * int`, so it overflows in 32 bits and
   is only *then* widened for the xor. Computing it all in i64 gives a different
   seed.
6. **`Mth#clamp(long,long,long)`** was written `max(max(...))` instead of
   `min(max(...))`.
7. **The JVM's invalid-operation NaN is NEGATIVE** (`0xfff8…` / `0xffc0…`, the x86
   real-indefinite) where Rust returns the positive default. This applies to `+`,
   `−`, `*`, `/`, `%` and `sqrt`, so it is handled by a family of helpers in
   `javacompat`.
8. **`Math.min`/`Math.max` propagate the actual NaN operand** (payload and sign),
   and `Math.min(+0.0, −0.0)` is `−0.0` because HotSpot intrinsifies to `minss`,
   whose tie-break returns the second operand. Rust's `f32::min` does neither.
9. **`fastInvSqrt`, `fastInvCubeRoot`, `quantize`** need wrapping arithmetic or
   debug builds panic on overflow.

**The measurement trap.** Probing JVM behaviour with `javac` constants lies:
`Float.POSITIVE_INFINITY % 360.0F` is a compile-time constant, so javac folds it
with its own arithmetic and reports a *positive* NaN. HotSpot at runtime raises the
negative real-indefinite. The first probe led me to implement the wrong sign. Every
JVM fact recorded in `DESIGN_DECISIONS.md` was re-measured through a `volatile` or
an argument to defeat constant folding.

### 2e. Verification

```
javacompat     12 tests   floorDiv/floorMod, abs-on-NaN, NaN arithmetic,
                          signed-zero min/max, narrowing, UTF-16 hashCode, MD5
parity_mth     13 tests   110 golden groups, 10 149 rows
parity_random  15 tests   legacy / xoroshiro / single / threadsafe transcripts,
                          positional factories, RandomSupport, WorldgenRandom, LCG
                        ─────────
                          40 tests, 0 failures
```

The 65 536-entry `sin` lookup table is verified as a whole via an FNV-1a hash of
its raw bits, so a single wrong entry fails the build.

Also verified: `mirror.py --check` clean, `check_mirror.py` clean
(`.java` without `.rs`: **0**; orphan `.rs`: **0**; source tree vs git: **CLEAN**).

### 2f. Still open

Five `Mth` methods are ported-or-blocked on unported types
(`getSeed(Vec3i)`, `lerp(double,Vec3,Vec3)`, `rayIntersectsAABB`, `rotationAroundAxis`,
`mulAndTruncate(Fraction,int)`). Their golden rows already exist; only the Rust side
is missing. `parity_mth.rs` lists them in `BLOCKED_ON_UNPORTED_TYPES` and **fails if
that list ever grows silently**. Full list in `OPEN_QUESTIONS.md` (10 items).

Also deliberately untested because they are non-deterministic by construction:
`RandomSource.create()`, `createThreadSafe()`, `createThreadLocalInstance()` (unseeded),
and `RandomSupport.generateUniqueSeed()`.

`Xoroshiro128PlusPlus.CODEC` and `XoroshiroRandomSource.CODEC` are **not ported** —
they are DataFixerUpper codecs, needed for `level.dat` world serialization, and
belong to the serialization batch (question 8).

### 2g. Commits (local only — do not push, the repo is public)

| sha | what |
|---|---|
| `e842566` | mirror skeleton: 7055 `.rs` + 552 `mod.rs` + tools |
| `0e9c9e5` | Java oracle + 49 847 golden rows |
| `2a129b0` | `Mth` + all random sources VERIFIED, 40 tests |
| (this)   | docs: `DESIGN_DECISIONS`, `OPEN_QUESTIONS`, `PORTING_PLAN`, `SESSION_LOG`; manifest statuses |

### 2h. Next

Batch 2 — core value types (`Vec3i`, `Vec3`, `Direction`, `BlockPos`, `AABB`,
`Identifier`, `ARGB`, …). It closes out the five blocked `Mth` methods and is a
prerequisite for NBT, registries and worldgen. See `PORTING_PLAN.md`.
