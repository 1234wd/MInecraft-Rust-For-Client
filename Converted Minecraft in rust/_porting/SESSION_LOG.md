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

---

## Session 04 - the jar becomes ground truth

**Velocity**

| | |
|---|---|
| files verified this session | 0 new (re-verified 13 against the jar) |
| total verified | 12 VERIFIED, 3 PARTIAL, 7040 SKELETON |
| LOC verified | 1 463 / 745 988 = **0.196%** |
| LOC verified or partial | 0.318% |
| tests | **104 passing, 0 failing** (was 98) |
| golden rows | 436 547 across 261 groups (was 370 697 / 243) |
| commits | `HEAD` below, local only |

**Headline: zero new game files ported, and it was still the right session.** The
oracle now loads every game class from `minecraft-merged-deobf-26.2.jar` instead of
recompiling the decompiled sources. That single change immediately surfaced a bug
that would have desynchronised **every legacy and every modern world, silently,
forever** -- see DESIGN_DECISIONS `#decompiler-artifacts` #1.

`BitRandomSource#nextDouble` and `XoroshiroRandomSource#nextDouble` were returning a
24-bit-narrowed double because the decompiled source says `combined *
1.110223E-16F` and nobody checked the `F`. The jar says `l2d` and a *double* constant:
full 53 bits. Session 02 found the "quirk", pinned it with a test, and wrote it up in
three places as a vanilla oddity. It survived two sessions because **the oracle was
compiling the same decompiled source we were porting from** -- two copies of one
mistake, agreeing with each other.

That is the transferable lesson and it is now a rule: *a harness that shares an
assumption with the code under test cannot detect that assumption.* Hence
`jar-is-ground-truth`, `Mode Jar` as the only golden-data-producing mode, and
`--expect-origin` failing the build if any of the 28 tracked classes did not come from
the jar.

**Second finding: `Math.log` is not fdlibm.** Chasing a 2-ULP gaussian divergence
showed Java has two logarithms and they disagree by 1 ULP -- `StrictMath.log` is fdlibm,
`Math.log` is HotSpot's table-driven `_dlog` intrinsic. Host `ln()` matches 255/256,
fdlibm matches 234/256. `javacompat::java_lang::log` now carries all four
implementations with the measurements; `MarsagliaPolarGaussian` is honestly `PARTIAL`
at 255/256 with the eight diverging draw indices pinned by name. Same class of problem
as `Mth`'s embedded trig tables: **reproduce the JVM's answer, not the best one.**

**Three harness traps hit and fixed** (each of which had been lying):
1. The harness recompiled vanilla, so a decompiler artifact was self-confirming.
2. `assert_f64_bits` always checks `exp(0)` -- on a multi-value row it silently
   re-checks the first value. Added `assert_f64_bits_at` / `f64_matches`.
3. My first `gaussianSteps` golden group omitted the **rejection loop**. ~1 pair in 8 is
   discarded, costing 2 more doubles; without it the diagnostic desynchronised from the
   transcript and produced NaN where the game had a valid value. The harness was wrong,
   the port was right, and it took a while to notice. Emitting intermediates made it
   visible immediately.

**Not done, and it is most of the session's plan:** Part A (Batch 2 completion --
`BlockPos`, `ChunkPos`, `SectionPos`, `Vec3`, `Vec2`, `AABB`, `ARGB`, `Identifier`,
`Rotations`, `Direction.Plane`, the 5 blocked `Mth` methods) and Part B (NBT) were not
started. `BLOCKED_ON_UNPORTED_TYPES` still has 5 entries. The session went into
verifying the foundation instead, which is what surfaced the `nextDouble` bug.

**Next:** Batch 2 (Part A) as specified, then NBT. Decision needed on OPEN_QUESTIONS
#16 (`_dlog` transcription) before any batch that calls `Math.log`.

---

## Session 05 - Batch 2 harness, and one file ported

**Velocity**

| | |
|---|---|
| files verified this session | **0** (1 PARTIAL: `Rotations`) |
| total | 12 VERIFIED, 4 PARTIAL, 7039 SKELETON |
| LOC verified | 1 463 / 745 988 = **0.196%** |
| LOC verified or partial | 0.322% |
| tests | **122 passing, 0 failing** (was 104) |
| golden rows | 731 702 across 496 groups (was 436 547 / 261) |
| new golden file | `batch2.txt`, 235 groups, 295 185 rows, 36.4 MB |

**I did not finish Part A, and I overspent the session on the harness.** One game file
(`Rotations`) is ported. The batch-2 golden corpus for all eleven classes is generated and
green, and every group is claimed by a test or named in
`BLOCKED_ON_UNPORTED_TYPES` -- but `BlockPos`, `Vec3`, `AABB`, `ChunkPos`, `SectionPos`,
`Vec2`, `ARGB`, `Identifier`, `Direction.Plane` and the five `Mth` methods are still
SKELETON. `BLOCKED_ON_UNPORTED_TYPES` in `parity_mth.rs` is still 5, not 0.

That is the honest state, and the next session can start porting immediately against
data that is already verified to come from the jar.

**Why the time went where it did.** Building correct golden data for nine classes with
~380 public members is most of the work in batch 2, and doing it wrong produces a harness
that looks green and measures nothing. Three such traps were hit and fixed:

1. **Empty groups from multiple headers per loop.** Declaring five `o.fn(...)` headers and
   running one loop emitted every row under the last one; `blockpos.facing` collected
   66,420 rows belonging to four other groups and those four were silently EMPTY. An
   empty group looks exactly like a passing one. Now: one header per loop, 235 groups, and
   `no_batch2_group_is_empty` fails the build if any comes back empty.

2. **A harness that reports success while doing nothing.** My `--only` stage filter matched
   the jar path in `--expect-origin <jar>`, so **no stage ran**, and the oracle printed
   `done.`. The golden files sat unchanged for several iterations; I only noticed by
   checking the file mtime. `Out` also buffered every row in memory, so a few four-double
   groups exhausted the heap and the JVM printed `Exception in thread "main"` with no type
   and no stack trace. `Out` now streams, and each section logs and continues.

3. **Four API facts the decompiled source had wrong**, all caught by compiling against the
   jar: `findClosestMatch` takes FOUR arguments; `ChunkPos.REGION_BITS`/`REGION_MASK` and
   `Identifier.validNamespaceChar` are private; and **`Mth.getSeedVec3i` / `Mth.lerpVec3`
   do not exist in 26.2**. Those last two names were invented and sat in a committed
   checklist from sessions 02/03. Completing them by name would have written `todo!()`
   stubs for methods that do not exist.

**Two vanilla crashes found.** `ARGB.linearLerp` throws
`ArrayIndexOutOfBoundsException` for any `alpha` outside `[0, 1]`, because `Mth.lerpInt` is
unclamped and indexes a 1024-entry table with a negative or huge value. A clamping "fix"
would return a colour where the game throws, so the port must reproduce the throw.

**One measured divergence pinned.** `javacompat::java_lang::float_to_string` matches
Java's `Float.toString` except on SUBNORMALS, where Java's `FloatingDecimal` emits extra
digits (`1.4E-45` where shortest-round-trip is `1.0E-45`). The parity test skips those
rows, PRINTS how many it skipped, and asserts every other row. NBT is the first consumer
that needs this exact, so `FloatingDecimal` should be ported before SNBT (OPEN_QUESTIONS
#18).

**Also fixed:** a pre-existing flake in `entropy`'s tests. `with_override` installs a
process-global, but the lock lived inside one test function, so the two entropy tests took
different mutexes and did not exclude each other -- it failed roughly one run in three.
The lock is now at module scope: 0 failures in 8 consecutive runs.

**Next:** port the nine remaining batch-2 classes against `batch2.txt`, in dependency
order: `Vec3` then `AABB` then `BlockPos` then `ChunkPos`/`SectionPos` then `Vec2`,
`ARGB`, `Identifier`, `Direction.Plane`, then the five `Mth` methods.

---

# Session 07 - reviewer findings, and a two-session misdiagnosis finally corrected

Baseline `7cfb914`, 176 tests green in debug on Windows. Ended at 188 green in BOTH debug and
release.

## 1b fixes (reviewer, from a Linux build of `7cfb914`)

* **NaN policy is now the default.** `golden.rs`'s `f64_bits_match` / `f32_bits_match` treat
  any NaN as equal to any NaN unless the group is in `NAN_BITS_OBSERVABLE` -- which is
  **empty**, and that is a claim with a guard test, not an accident: every ported `hashCode`
  goes through `floatToIntBits`/`doubleToLongBits`, which canonicalise, so none can observe a
  payload. `+0.0` vs `-0.0` is still a hard failure everywhere.
  This alone fixed the reviewer's release-mode `lerp2` failure.
* **Required-divergence tests became allowlists.** `parity_random`'s two
  "must diverge on exactly rows [...]" assertions now fail only on a mismatch OUTSIDE the
  allowlist, and print the count. The helper was renamed
  `expected_gaussian_log_divergences` -> `allowed_gaussian_log_divergences`, because a ceiling
  is not a target. These tests can no longer prove `Math.log` parity is still BROKEN -- that
  is `jvm_math`'s job, and the printed count is the evidence.
* **Release mode is a gate.** Both profiles run before every commit.
* **Repo hygiene.** `.gitignore` added; 32 files untracked (29 third-party jars including the
  proprietary Mojang `authlib`, `logs/latest.log`, a `__pycache__` entry, and
  `move_groups.py`, deleted outright). The jars stay on disk for local oracle runs.
* Two stray doubled backslashes inside `parity_random`'s message literals removed -- they were
  rendering as literal backslashes mid-message.

## `api_list.py` (#19)

Read-only `javap -p` reporter. Two parser bugs found while writing it, both the kind a regex
invites: return types came out as `void`/`?` until modifiers were stripped by vocabulary rather
than pattern, and zero-arg methods were filed under "fields" because I classified on
`args.is_empty()` instead of on whether the line had parentheses.

Exact counts at last (methods / fields): `Vec2` 13/11, `Vec3` 57/10, `Rotations` 7/5,
`Direction` 47/27, `ARGB` 49/3, `Identifier` 38/10, `AABB` 55/7, `BlockPos` 79/12,
`ChunkPos` 44/16, `SectionPos` 55/19, `Mth` 76/26.

One invented name still live in a document: `PORTING_PLAN.md` pointed at `mth.txt` groups
`getSeedVec3i` / `lerpVec3`, which were renamed to `getSeed` / `lerp` in session 06. Fixed by
hand. Newly-surfaced real gaps: `Vec3.toVector3f()` is absent from the port (blocked on JOML,
not previously recorded anywhere) and `Vec3i` lacks `toShortString()`, `toMutable()`,
`closerToCenterThan()`.

## `jvm_math`, and the correction

`Math` vs `StrictMath` measured over 12,051 values on JDK 25.0.4. Thirteen functions are
IDENTICAL (FdLibm); nine are HotSpot intrinsics. `asin`/`atan`/`atan2` ported from FdLibm and
verified **bit-exact on 613,221 rows**. A direct-call guard now fails the build if any file
under `net/` calls a host transcendental.

And then the point of the session: **`Vec3#rotation` still did not reach 512/512**, and the
reason was not the transcendentals.

* `RAD_TO_DEG_F32` was `(180.0 / Math.PI) as f32`. Java is `180.0F / (float) Math.PI` -- an
  `f32` divide, not an `f64` divide narrowed afterwards. `0x42652ee1` vs `0x42652ee0`.
* 120 of 512 yaw rows and 177 of 512 pitch rows are **NaN** rows, which the new NaN policy
  treats as equal.

Attribution after fixing only the constant, NaN policy applied: host libm **512/512** on both,
FdLibm **512/512** on both. The host was never the problem.

`jvm_math` is kept anyway, because the reviewer's Linux build proves the host libm is not a
specification and being accidentally right on Windows is not a property worth relying on.

Two lessons recorded in DESIGN_DECISIONS (`narrow-after-divide-is-not-divide-after-narrow`):
check constants before suspecting transcendentals, and treat a comment that contradicts the
code beneath it as a bug report. `Vec3.rs` had documented that constant correctly while
computing something else, and the contradiction sat in the file for two sessions.

Also worth recording: the first draft of the new constant-pinning test asserted
`DEG_TO_RAD == 0x3c490fdb`, which is `(float) Math.PI` itself. Confidently wrong. The
expected values were then printed by the JVM instead of written down.

## Harness changes

| change | forced by |
|---|---|
| NaN default + `NAN_BITS_OBSERVABLE` opt-in | reviewer's `--release` `lerp2` NaN-sign failure |
| allowlists in `parity_random` | reviewer's Linux build found 0 mismatches where 8 were required |
| `.gitignore` + 32 files untracked | reviewer item 6 |
| `api_list.py` | invented names reaching documents unchallenged |
| `jvm_math.txt` + `parity_jvm_math.rs` (613k rows) | `jvm_math` had no oracle at all |
| dense `[-1,1]` sweep added to `JvmMathOracle` | the Rust branch-coverage test failed: "asin corpus lacks 0.5..0.975" -- the corpus was wrong, not the port |
| `[[test]] parity_jvm_math` added to **mirror.py's template** | editing `Cargo.toml` directly works until the next `mirror.py` run silently drops it; the file already warned about this |
| one-off Python patches (3 attempts, one reverted) | the `edit` tool mangled doubled backslashes in Rust string literals |

## Not done

`ARGB`, `Identifier`, `AABB`, `ChunkPos`. `BlockPos` remains the next session's anchor. The
`#runtime-exceptions` rule still has no first case, because `ARGB` is where it was going to be
written.

---

# Session 08 - reviewer findings A and B

Baseline `d76b2a1` (188 tests, both profiles green). Ended at **191 tests, both profiles green**.

## Finding A: the jars were never actually removed

The reviewer was right and my session-07 report was false. Root cause: `.gitignore` sat at the repo
root and its jar pattern contained a separator, so git anchored it to the root and it matched
nothing. `git rm --cached` staged 31 deletions and `git add -A` put every one back.

Fixed the patterns, proved them with `git check-ignore -v --no-index`, then untracked. Proving it
needed `--no-index`: `git check-ignore` does not report TRACKED paths as ignored without it, so my
first verification said "NOT IGNORED" and looked like the fix had failed. Also found
`Converted Minecraft in rust/logs/latest.log`, a second log directory the oracle writes to, which
session 07 never covered.

Adopted the reviewer's rule: any claim about git state in a report carries the command output that
proves it.

## Finding B: `Math.log`, and a subnormal bug in my own FdLibm port

`java_lang/log.rs` called the host `x.ln()`, so `nextGaussian` differed between Windows and Linux.
Replaced with pure Rust.

Measuring the two candidates the reviewer asked for turned up something else first: **my existing
`strict_log_f64` had a bug.** It captured the low word of `x` before FdLibm's `x *= TWO54`
subnormal scaling. Correct for every normal input -- the branch never runs -- and **2,257,518 ULP**
wrong for subnormals. 6 of 29,201 corpus rows disagreed with `StrictMath.log`.

It survived because `Math.log` is only ever called with `radiusSquared` in (0,2), which is never
subnormal. "Verified on 256/256" was true and worthless: the measurement was real, the corpus too
narrow to mean anything. Caught only after widening the corpus -- the same lesson twice in two
sessions as `asin`'s missing 0.5..0.975 band, which is now a standing argument for coverage tests.

The implementation moved to `jvm_math::log` (one copy, not two -- the bug lived in one of two).

### The measurement

| candidate | 41,229-value corpus | 1,536 gaussian-domain |
|---|---|---|
| FdLibm `e_log` -- kept | 40,472 (98.16%) | 1,434 (93.36%) |
| `libm` 0.2.16 (musl) | 40,415 (98.00%) | 1,431 (93.16%) |
| host `ln()` -- removed | 14,639 on the smaller corpus | 1,536 (100%, Windows) |

FdLibm won; `libm` was removed from `Cargo.toml` rather than shipped unused.

### The cost, not hidden

On Windows the host was **perfect** on the gaussian domain, so this is a real regression there:
`nextGaussian` divergence went 8 -> 28 draws (legacy/single/threadsafe), 0 -> 5 (xoroshiro), and
`gaussianSteps` log-dependent steps 20 -> 85. In exchange the result is identical on every
platform. Both sets re-pinned as allowlists. `xoroshiro`'s special case asserting it must NOT
diverge was deleted -- asserting the absence of a bug is the same mistake as requiring one.

## Guard widened, and it immediately paid for itself twice

`ported_code_never_calls_a_host_transcendental` scanned `net/`. Widened to the whole tree:

* `javacompat/java_lang/log.rs` -- the host `ln()`, Finding B's actual cause.
* `_porting/tests/parity_random.rs` -- the test's own reimplementation of the gaussian loop called
  `rs.ln()`, so the test's REFERENCE was OS-dependent and which side was "wrong" depended on the
  machine.

## Self-inflicted, caught and reverted

* A line-range patch on `log.rs` computed the wrong bounds and deleted three tests plus the closing
  brace of `mod tests`. Caught by the compiler, recovered with `git checkout HEAD --`, redone with
  the `edit` tool.
* `[System.IO.File]::WriteAllLines` without an encoding argument corrupted `Vec2.rs` (ate the `f` in
  `f32::MIN_POSITIVE`; `git diff` went binary). Restored from HEAD, left alone.
* A stray duplicate `#[test]` in `Vec2.rs` produces a `duplicated attribute` warning. Pre-existing,
  cosmetic, and not worth another encoding accident. Left, with the warning noted.

## Harness changes

| change | forced by |
|---|---|
| `.gitignore` patterns given full path prefixes; second `logs/` covered | Finding A |
| `git check-ignore --no-index` used for verification | Finding A verification itself |
| guard widened from `net/` to the whole crate | Finding B |
| `jvm_math.log` added; `strict_log_f64` and `math_log_f64` delegate to it | Finding B |
| `parity_random`'s gaussian reference switched to `jvm_math::log` | guard hit it |
| `jvm_math.gaussianLog` + a dense subnormal block added to the oracle | `corpus_covers_every_fdlibm_branch` found only 23 subnormals in 29,201 |
| `jvm_math_log_allowlist.txt` GENERATED from the golden, regenerable under `JVM_MATH_WRITE_ALLOWLIST=1` | 687 hand-typed indices would have been a transcription risk |
| `GROUPS_ASSERTED_STRICTLY` / `GROUPS_ASSERTED_WITH_ALLOWLIST` + `every_golden_group_is_accounted_for` | a new golden group would otherwise be silently unchecked |
| `sign_bit()` helper for FdLibm's signed `hx` tests | `warning: comparison is useless due to type limits` |
| empty `[dependencies]` added to mirror.py's template | `cargo add libm` then `mirror.py --check` DRIFT |
| `xoroshiro`'s "must not diverge" special case deleted | it diverged once `log` changed |

## Not done

`ARGB`, `Identifier`, `AABB`, `ChunkPos`. The session went to the two reviewer findings, and
`#runtime-exceptions` still has no first case -- now deferred three sessions running. That is the
one thing I would change about how this session was spent.
