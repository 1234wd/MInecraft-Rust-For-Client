# Open questions — for the human, not for me

Flagged rather than guessed, per the no-guessing rule. Each says what I need and
what I did in the meantime.

---

## 1. `RandomSource#create()` uses `System.nanoTime()` — no portable Rust equivalent

`RandomSupport#generateUniqueSeed()` XORs a global uniquifier with
`System.nanoTime()`. Java's `nanoTime` is an arbitrary monotonic origin; Rust's
`Instant::now()` is also arbitrary but *not* the same arbitrary value, and
`std::time::SystemTime` is a wall clock that can jump.

**Current behaviour:** I use `SystemTime::now()` nanoseconds since the UNIX epoch.
**Consequence:** two Rust processes started at the same instant on the same tick
would produce the same uniquifier-derived seed. Vanilla has the same theoretical
window with its 1 ms-resolution counter, so this is not a practical regression —
but it is not bit-identical.

**Decision needed:** acceptable, or should `create_unseeded()` take an explicit seed
from the caller and drop `System.nanoTime` entirely? **This function is deliberately
NOT parity-tested** (it is non-deterministic by construction).

## 2. `nextInt(bound <= 0)` panics — do you want `Result`?

Java throws `IllegalArgumentException`. Nothing in vanilla catches it, so a panic is
behaviourally equivalent (both crash). But if you would rather have
`Result<T, RandomError>` throughout, say so now and I will start there rather than
retro-fitting 7000 files.

## 3. Five `Mth` methods are ported-but-untested, blocked on unported types

| Java method | blocked on | plan batch |
|---|---|---|
| `getSeed(Vec3i)` | `net/minecraft/core/Vec3i.rs` | core |
| `lerp(double, Vec3, Vec3)` | `net/minecraft/world/phys/Vec3.rs` | core |
| `rayIntersectsAABB(Vec3, Vec3, AABB)` | `Vec3.rs`, `AABB.rs` | core |
| `rotationAroundAxis(Vector3fc, Quaternionf, Quaternionf)` | JOML types | core |
| `mulAndTruncate(Fraction, int)` | commons-lang3 `Fraction` | util |

Their golden data **is** generated and sitting in `_porting/test-data/mth.txt`; only
the Rust side is missing. `_porting/tests/parity_mth.rs` lists them in
`BLOCKED_ON_UNPORTED_TYPES` and will fail if the list ever grows silently.

**Decision needed:** port `Fraction` into `javacompat` now (it is ~150 lines and
`NumberUtils.toInt` is already done), or leave `mulAndTruncate` for the util batch?

## 4. JOML in the client crate — replace or reimplement?

`Mth#rotationAroundAxis` takes `org.joml.Quaternionf` / `Vector3fc`, and the client
renderer needs far more of JOML (`Matrix4f`, `Matrix3f`, `Vector4f`, …). The oracle
compiles against the real jar because `invsqrt` is a version-specific bit trick.

**Current behaviour:** `Mth::inv_sqrt_f32/f64` reproduce JOML 1.10.8's definition
(`1.0f / (float)Math.sqrt((double)x)`) and are parity-tested; `rotationAroundAxis`
is omitted pending a decision.

**Decision needed:** (a) vendor a `javalike` module in `javacompat`, (b) pull in the
`joml` crate, or (c) reimplement the small subset the game uses?

## 5. Exact JOML / commons-lang3 versions used by Minecraft 26.2 are unverified

The oracle pins joml 1.10.8 and commons-lang3 3.17.0 (chosen as close-to-current
releases). Minecraft's own `build.gradle` was not in `minecraft-decompiled/`.

**Impact:** `Mth::inv_sqrt_*` depends on joml's `Math.invsqrt` definition. It has
been stable across the 1.10.x line, but if 26.2 pins a different major/minor this
needs rechecking. **Can you supply the version list from the official manifest?**

## 6. `PositionalRandomFactory#fromHashOf(Identifier)` is untested

The overload takes `net.minecraft.resources.Identifier`, which is still `SKEELTON`
(277 LOC, with its own parsing/validation rules). The oracle uses a stub
`Identifier` whose `toString()` is `namespace + ":" + path` — accurate, but testing
against my own stub would test the stub.

`fromHashOf(String)` — the seeded path that actually drives worldgen — **is**
parity-tested and passing.

**Decision needed:** none, just flagging that this overload stays untested until
`Identifier.rs` is ported (batch 2).

## 7. Hash-map iteration order

The Prime Directive says to reproduce Java `HashMap`/`HashSet` iteration order where
it can affect behaviour, and to flag it. I have not hit a case yet in the util batch
(no hash-ordered iteration in `Mth` or the random sources).

**Note for future batches:** any registry, tag map, or `IdMap` iteration that feeds
into game state needs either a Java-order map (Java `HashMap` = `hash(key)` spread
with bucket chaining) or an explicit note that the order does not matter. Default
`std::collections::HashMap` uses SipHash and a random seed, so it is *not* a drop-in.

## 8. `codec-not-ported` — the `CODEC` static fields

`Xoroshiro128PlusPlus.CODEC` and `XoroshiroRandomSource.CODEC` are DataFixerUpper
codecs. They are needed for world serialization (`*_seed` fields in
`level.dat`) and are currently absent. DFU is a large dependency; this belongs to
the serialization batch.

## 9. Golden data is committed, not regenerated in CI

`_porting/test-data/*.txt` (2.4 MB) is committed so `cargo test` needs no JDK. If a
future contributor edits vanilla code without a JDK, `cargo test` will still pass
against stale goldens.

**Mitigation in place:** `_porting/tools/mirror.py --check` fails if the mirror
drifts, and `check_mirror.py` fails if `minecraft-decompiled/` changed. Neither
regenerates the goldens.

**Decision needed:** acceptable, or should CI run `java-oracle/run.ps1` and diff?

## 10. `TakeRandomSourceForTest` is a non-vanilla API on `WorldgenRandom`

To replay the oracle's nested loops, `WorldgenRandom` exposes
`take_random_source_for_test()`, which swaps the wrapped source out and leaves a
placeholder behind. It is `#[doc(hidden)]` and exists only for the parity tests.

**Decision needed:** fine to keep, or would you rather the tests construct a
`WorldgenRandom` per seed and lose the exact loop replication?
---

## 11. ANSWERED (session 03) � NaN policy

As directed: exact NaN sign and payload are matched only where game code can observe
them (`floatToRawIntBits`/`doubleToRawLongBits` results, hashing, serialisation,
network). Everywhere else the parity test asserts only that both sides are NaN.

`javacompat::nan_policy` implements the policy and documents that **the JVM's own NaN
sign is CPU-dependent** � x86-64 raises the negative real indefinite for an invalid
operation, AArch64 raises the positive default. **We target x86-64 HotSpot**, and the
oracle measures exactly that.

Closed because it was a policy question with a defensible answer, not a fork in the road.

## 12. ANSWERED (session 03) � entropy injection

Done: `javacompat::entropy` is the single funnel for `System.nanoTime`,
`currentTimeMillis` and thread-local entropy. `RandomSupport#generateUniqueSeed` now
calls `entropy::nano_time()` instead of `SystemTime::now()` directly.

This closes question 1 as far as it can be closed. The residual divergence is
**documented, not eliminated**: Java's `nanoTime` counts from an arbitrary JVM-chosen
origin and Rust cannot reproduce that number, so `generateUniqueSeed` returns a
different value than vanilla on the same call. It is unobservable by construction �
that is the point of the function. What is now guaranteed is that it is *injectable*,
so tests are deterministic and the remaining clock reads are auditable in one file.

If you want bit-identical `generateUniqueSeed` anyway, the only route is to drop
`nanoTime` and take an explicit seed from the caller. **Say so and I will do it.**

## 13. ANSWERED (session 03) � JOML

Decided: port the subset from JOML 1.10.8's own bytecode into
`_porting/javacompat/joml/`, do **not** use `glam`/`nalgebra`. The deciding factor is
concrete: `Vector3f#dot` and `Quaternionf#normalize` are built on `Math.fma`, an IEEE
fused multiply-add that rounds once; a general Rust math library would not reproduce
it, and its FMA-contraction behaviour is not something we can audit.

`invsqrt`, `fma`, `Vector3f::dot` and `Quaternionf::normalize/set` are ported and
unit-tested. The rationale is in DESIGN_DECISIONS.md (#joml-subset-ported-not-swapped).

## 14. STILL OPEN � the codec/serialisation decision needs a call from you

A full proposal is in DESIGN_DECISIONS.md (#dfu-proposal): port DataFixerUpper into
`javacompat` (option A) versus write a minimal replacement (option B). **I recommend
A, after NBT.**

The three parity risks are the same either way and are worth your attention before I
start, because they are the ones that corrupt worlds *silently*:

1. **Field order** in a serialised map. `RecordCodecBuilder` preserves declaration
   order and that order is on the wire and in save files. A `HashMap` here corrupts
   worlds without erroring.
2. **`DataResult` error text** reaches the player, so a message change is user-visible.
3. **Numeric widening.** DFU refuses to silently narrow `int64 -> int32`; a
   replacement that permits it corrupts values at the world border instead of erroring.

Batch 3 (NBT) does not need this decision and can start immediately.

## 15. NEW (session 03) � `Util.java` does not compile under JDK 25

`Util#makeEnumMap` contains

```java
for (K key : (Enum[])keyType.getEnumConstants()) {   // K extends Enum<K>
```

which JDK 25's javac rejects: `incompatible types: Enum cannot be converted to K`.
This is a decompiler artefact � Mojang's real source has no such cast.

**Current handling:** `Util` is NOT in the oracle's compile list; the real
`minecraft-merged-deobf` jar supplies it, since it is only reached for trivial
delegation (`Util.make`) and is not a batch-2 port target.

**Question:** if a future batch needs to test something `Util` actually computes,
we will have to either compile a patched copy (breaking the "unmodified sources" claim
for that one file) or mirror it into `javacompat` as third-party-ish code. **I would
prefer mirroring into `javacompat` with a comment naming the artefact**, but that is a
rule change and should be your call.
---

## 16. NEW (session 04) - transcribe HotSpot's `_dlog`, or accept 255/256 `Math.log`?

**Status: open. Needs your call on effort, not on principle.** *(Session 07 confirmed the
premise and narrowed the options: `Math.log` IS a HotSpot intrinsic, so FdLibm will not do.
See #22 for the three options and a recommendation.)*

`MarsagliaPolarGaussian#nextGaussian` is the only ported method that calls
`Math.log`, via `multiplier = sqrt(-2.0 * log(rs) / rs)`. It is currently
**255/256-exact**. Measured over the 256 arbitrary doubles in the `random.txt`
`radiusSquared` corpus:

```
Math.log       == StrictMath.log (fdlibm)  : 234 / 256
Math.log       == host f64::ln()           : 255 / 256   <- what we use
StrictMath.log == host f64::ln()           : 235 / 256
```

All differences are 1 ULP. HotSpot's `Math.log` is the `_dlog` intrinsic, generated by
`generate_libmLog()` in `src/hotspot/cpu/x86/stubGenerator_x86_64_log.cpp`. It is a
**table-driven** algorithm, not fdlibm's polynomial: a 128-entry `_L_tbl` of doubles
indexed by the top mantissa bits, `_log2 = {ln2_hi, ln2_lo}`, 6 `_coeff` doubles, and a
packed `mulpd`/`addpd` polynomial tail.

**The choice:**

- **(a) Transcribe the stub** (~700 lines of MacroAssembler, mostly table loads and
  packed-double arithmetic). Gets `Math.log` to exact and `MarsagliaPolarGaussian` to
  VERIFIED. Risk: operation order in the packed polynomial is rounding-sensitive, so
  it needs the same oracle-then-diff discipline as everything else. It is also
  *bounded* work -- the table is a fixed constant array, not an algorithm to invent.
- **(b) Leave it at 255/256** and keep the pinned allowance. Honest, documented,
  testable, and the practical impact is tiny: 4 draw-pairs out of 256 seeds x16 draws
  are 1-2 ULP off in `nextGaussian` only.

**My recommendation: (a).** Not for the eight draws -- for what it implies. Minecraft
calls `Math.log`/`Math.exp`/`Math.pow` in worldgen noise and in several entity and
block-update paths. Every one of them will hit the same wall, and `javacompat` will
need `_dexp`, `_dpow`, `_dtanh`, `_dcbrt` and `_dlog10` (all present in the same
directory, all confirmed still wired up in JDK 25) to avoid the whole class of bug.
Porting one of them now proves the pattern; porting them one at a time later under
time pressure is how the `nextDouble` bug happened.

**Not a blocker for Batch 2 or NBT** -- neither uses `Math.log`. Block the decision
until a batch actually needs it, unless you want it done deliberately.

---

## 17. NEW (session 04) - `Math.sqrt` needs no special treatment, but verify per-function

Unlike `log`, `sqrt` **is** correctly rounded by IEEE-754, so `SQRTSD` gives the
correctly-rounded answer and Rust's `f64::sqrt` matches everywhere in the corpus. Noted
so nobody ports it "for consistency". Same question applies to every remaining transcendental; each needs measuring, not assuming.

**ANSWERED IN SESSION 07.** The measurement was run over a 12,051-value corpus against
`StrictMath` (i.e. FdLibm), and the split is clean rather than gradual:

* **13 functions are IDENTICAL** and can be ported from `FdLibm.java` today: `asin`, `acos`,
  `atan`, `atan2`, `sinh`, `cosh`, `hypot`, `log1p`, `expm1`, `sqrt`, `floor`, `ceil`, `rint`.
* **9 are HotSpot intrinsics** and need the platform stub: `log`, `log10`, `exp`, `sin`,
  `cos`, `tan`, `tanh`, `cbrt`, `pow`.

So the blanket `jvm_math` module is the right shape, but it does NOT need to cover the
intrinsics -- those are a separate, much larger job. `asin`, `atan` and `atan2` are ported and
verified on 613,221 rows; the remaining IDENTICAL ones are `acos`, `sinh`, `cosh`, `hypot`,
`log1p`, `expm1`, each the same job.

`floor`, `ceil` and `rint` need no port at all -- they are exact integer-like operations, and
`jvm_math`'s guard test treats them as non-violations for that reason.
---

## 18. NEW (session 05) - `Float.toString` on SUBNORMALS needs Java's `FloatingDecimal`

**Status: open. Small, well-scoped, not blocking Batch 2 or NBT.**

`javacompat::java_lang::float_to_string` re-lays Rust's shortest `{:e}` form into Java's
`Float.toString` layout (always a fractional part, scientific outside `[1e-3, 1e7)`).
That matches Java on every ordinary value, and disagrees on **SUBNORMALS**:

```text
input bits 0x00000001   (1.4012984643...e-45)
Java        "1.4E-45"    two significant digits
shortest    "1.0E-45"    one digit, and it round-trips too
```

Java's `FloatingDecimal.toJavaFormatString` is documented as producing "as many digits as
are needed to uniquely distinguish the argument value from adjacent values of type float"
-- but its implementation is not the shortest-decimal algorithm Rust uses, and for
subnormals it emits more digits than strictly necessary.

Measured impact: 1 of 23 rows in `rotations.toString`. `parity_batch2.rs` skips subnormal
rows, PRINTS how many it skipped, and asserts every other row.

Options:

- **(a) Port `java.lang.FloatingDecimal`'s digit generation.** Self-contained, ~300 lines
  of integer/float manipulation, no `Math` calls. Would make `float_to_string` exact
  everywhere, which matters because **SNBT** (`TagParser`, `SnbtPrinterTagVisitor`) prints
  floats and doubles and the text has to match character for character.
- **(b) Leave it.** Subnormal floats essentially never appear in SNBT output.

**Recommendation: (a), immediately before NBT.** NBT is the first consumer that needs
float text to be exact, and this is much cheaper to port than a transcendental because it
is pure integer arithmetic.

---

## 19. NEW (session 05) - `Mth.getSeedVec3i` and `Mth.lerpVec3` DO NOT EXIST in 26.2

Found by `javac` while writing the batch-2 oracle: the two names in
`BLOCKED_ON_UNPORTED_TYPES` from sessions 02/03 are stale. `javap -p` on the real jar
shows the actual methods are

```text
public static long getSeed(Vec3i)
public static Vec3 lerp(double, Vec3, Vec3)
```

A port that "completed" the blocked list by writing `getSeedVec3i` and `lerpVec3` would
have produced two `todo!()` stubs for methods that do not exist -- and would have left the
two methods vanilla actually uses untested. The manifest and `parity_mth.rs` now carry the
real names. Recording it because the stale names were in a committed checklist, and a
checklist is exactly where a wrong name survives longest.

---

## 20. NEW (session 06) - `+NaN + (-NaN)` has NO single answer; 39 golden rows depend on it

**Status: open, measured, pinned. Not blocking anything.**

`Vec3#subtract(s)` is `add(-s,-s,-s)`, so when a component is NaN and `s` is NaN, vanilla
evaluates `+NaN + (-NaN)`. What HotSpot returns depends on the COMPILED FORM, not on the
language. Two golden rows with identical operand bits disagree:

```text
Vec3( 0.0, 0.0, NaN).subtract(NaN)  ->  -NaN, -NaN, +NaN    first operand won
Vec3( 1.0, NaN, 1.0).subtract(NaN)  ->  -NaN, -NaN, -NaN    second operand won
```

The only difference is WHICH COMPONENT is NaN, which changes scalar-vs-vectorised codegen.
A standalone probe of `x + y` on HotSpot 25 says "first wins"; the game's own `Vec3.add`
produces both answers.

So `javacompat::java_lang::nan_result_*` implements the MEASURED MAJORITY -- **second operand
wins** -- which matches 295,146 of 295,185 rows overall and 4,057 of 4,096 in
`vec3.subtractScalar`. The **39** exceptions are compared first and only then tolerated, so
the other 4,057 rows still assert; anything that differs and is NOT a both-NaN add still
fails the build.

**Recommendation:** leave it. The divergence is only observable through NaN coordinates,
which do not occur in a running world (positions are finite; a NaN position means the game
has already gone wrong). Transcribing HotSpot's codegen choices is not tractable and not
worth it.

---

## 21. CLOSED (session 07) - `jvm_math`: the premise was right, the diagnosis was wrong

**Status: closed, and the answer is not the one the question expected.**

The question asked which of `asin`/`atan2` to port first, on the theory that the 136-yaw /
179-pitch divergence in `vec3.rotation` came from the host's `atan2`/`asin` differing from
HotSpot. Session 07 measured it, and the theory was wrong.

### What was measured

`Math.f` vs `StrictMath.f`, bit for bit, over a 12,051-value corpus on JDK 25.0.4:

* `asin`, `atan`, `atan2`: **0 differences** -- one-line delegations to `StrictMath`, which is
  `FdLibm.java`.
* `log`, `exp`, `pow`, `sin`, `cos`, `tan`, `tanh`, `log10`, `cbrt`: HotSpot **intrinsics**.

And then, after `asin`/`atan`/`atan2` were ported from FdLibm and verified on 613,221 rows,
`vec3.rotation` still did **not** reach 512/512 under strict comparison. Two further bugs:

1. **A 1-ULP-wrong constant.** Java multiplies by `180.0F / (float) Math.PI` (an `f32`
   divide); `Vec3.rs` had `(180.0 / Math.PI) as f32` (an `f64` divide, then narrowed).
   `0x42652ee0` vs `0x42652ee1`.
2. **The NaN comparison policy.** Of the 512 rows, **120 are NaN on yaw and 177 on pitch.**
   Those were the rows being "lost".

Attribution, measured after fixing only the constant, with the NaN policy applied:

| call style | yaw | pitch |
|---|---|---|
| host `atan2` + host `asin` | 512/512 | 512/512 |
| `jvm_math` (FdLibm) | 512/512 | 512/512 |

So on this corpus the host libm was never the problem. See DESIGN_DECISIONS
(`narrow-after-divide-is-not-divide-after-narrow`).

### What was kept, and why

`jvm_math` stays. Not because it was needed here -- it was not -- but because the reviewer's
Linux build is direct evidence that the host libm is not a specification, and being
accidentally right on Windows is not a property to rely on. The guard test in `jvm_math.rs`
now fails the build if any ported file calls a host transcendental.

`asin`, `atan` and `atan2` are PORTED and verified bit-exact against the JVM on 613,221 rows.
`acos`, `sinh`, `cosh`, `hypot`, `log1p`, `expm1` are measured IDENTICAL and are the obvious
next ports.

`Vec3#rotation` is now **512/512 on both pitch and yaw**, asserted strictly rather than
counted, and `Vec3` is PARTIAL for codec reasons only.

### The lesson worth keeping

"1 ULP on some rows" does not identify a cause. A wrong constant and an inexact
transcendental produce identical symptoms, and the constant is cheaper to check and less
often suspected. Check the constants first.

## 22. NEW (session 07) - do we still need `jvm_math` for the five remaining `Mth` methods?

**Status: open, but smaller than it was.**

`Mth.mulAndTruncate`, `Mth.rayIntersectsAABB` and `Mth.rotationAroundAxis` are blocked on
`AABB`/`Vec3i`/JOML, not on math, and were never a `jvm_math` question.

The real remaining question is narrower: **`Math.log`**. `MarsagliaPolarGaussian#nextGaussian`
is the only ported method that calls it, through
`multiplier = sqrt(-2.0 * log(rs) / rs)`, and it is 255/256 exact. `log` IS a HotSpot
intrinsic (7 of 12,051 sweep values differ from FdLibm, worst case 1 ULP), so the fix is the
x86-64 `_dlog` stub -- which is table-driven, so a polynomial will not do.

Options, in order of effort per unit of exactness:

1. **Accept 255/256 and pin it** with an allowlist. Cheap, and the single bad row is a draw
   from `radiusSquared` with probability ~0.4%, reached once every few thousand gaussian
   draws. But it is a real behavioural difference that can compound in entity AI.
2. **Transcribe `_dlog`** from HotSpot's x86-64 stub. Weeks, and it is x86-64 specific, so
   the ARM question then has to be answered too.
3. **Reduce to FdLibm's `log`** and accept 1 ULP on 7/12,051 inputs -- i.e. *worse* than the
   host, since the host currently matches HotSpot on 255 of 256. Not worth doing.

Recommendation: option 1 now, and revisit if a gaussian draw ever shows up in a
gameplay-visible divergence. This is your call on effort, not on principle.

**SUPERSEDED IN SESSION 08.** Option 1 as written was "accept 255/256", but the host libm was
not usable at all: the reviewer measured `nextGaussian` at 248/256 on Windows and 256/256 on
Linux, so the port's output depended on the OS. `Math.log` is now pure Rust (FdLibm `e_log` in
`jvm_math`), which is OS-independent and 1 ULP from `_dlog` on 2.6% of inputs. Measured cost and
the reasoning are in DESIGN_DECISIONS (`log is pure Rust now, and it is WORSE on Windows`).

## 23. NEW (session 09) - candidate (c) for `Math.log`: a correctly-rounded `log`

**Status: open, explicitly deferred until after Batch 2. Do not start before ARGB/Identifier.**

The evidence that suggests this is worth trying is short and specific:

* Linux glibc's `ln` matched HotSpot's `_dlog` on **256 of 256** gaussian draws (reviewer,
  session 08). glibc's `log` is correctly rounded for the overwhelming majority of inputs.
* So HotSpot's `_dlog` is *probably* correctly rounded over that range too -- i.e. it may be
  aiming at "the correctly rounded answer", not at FdLibm's slightly different one.
* FdLibm's `e_log` is a faithful port of fdlibm, which is **not** correctly rounded. That is the
  whole reason it misses on 2.6% of inputs.

So a correctly-rounded `log` could match `_dlog` far better than FdLibm does. Candidate (c): a
pure-Rust port of **CORE-MATH's `cr_log`** (MIT licence, so no GPLv2 + Classpath Exception
obligation, unlike the FdLibm work).

Things to measure, all of them in the existing harness:

1. Mismatches against `Math.log` on the 41,229-value `jvm_math.log` corpus. FdLibm gets 98.16%;
   the bar to beat is that, and the thing to beat is musl's 98.00%.
2. Mismatches on the 1,536 gaussian-domain rows. FdLibm gets 93.36%.
3. **How many inputs does it get right that FdLibm gets wrong?** That is the number that decides
   it. If it is a different set rather than a superset, both stay and it is a wash.
4. Subnormals and the exact powers of two, because a correctly-rounded implementation usually
   needs special handling there and that is where FdLibm's port already had one real bug.

If (c) turns out to match `_dlog` on 100% of the gaussian domain, then the 28/320 `nextGaussian`
divergence introduced in session 08 goes back to zero and the trade documented in DESIGN_DECISIONS
can be retired. That is the prize.

Also note the sequencing argument for doing this *after* Batch 2: `log` is reached from exactly
one ported method, `MarsagliaPolarGaussian#nextGaussian`. Two intrinsic ports (`_dlog`,
`_dpow`) and four game files are worth more attention than one.

## 24. NEW (session 09) - `Vec2.rs` is byte-CORRUPTED at HEAD and the edit tool refuses it

**Status: open, cosmetic, blocking only the cleanup the reviewer asked for.**

`net/minecraft/world/phys/Vec2.rs` contains **three NUL bytes**, so both `git diff` and the
`read`/`edit` tools classify it as binary:

| offset | line | column | renders as |
|---|---|---|---|
| 3458 | 80 | 70 | `(<NUL>x00000001)` should be `(0x00000001)` |
| 3561 | 83 | 5 | `(<NUL>x00800000)` should be `(0x00800000)` |
| 11849 | 323 | 34 | `(<NUL>x00800000, ...)` should be `(0x00800000, ...)` |

Separately, three occurrences of `f32::` are missing their `f` and read `32::` -- lines 82, 322
and 324. All six defects are in **doc comments only**, so nothing in the build is wrong; the
symptom is `warning: duplicated attribute` from a stray duplicate `#[test]` on lines 321 and 325,
which is what the reviewer noticed.

**Why it is not fixed here.** Session 09's rule is that `.rs` files are changed only with the edit
tool, and if the edit tool cannot express an edit then the edit is logged and skipped rather than
worked around with a script. The edit tool cannot read this file at all -- it refuses as binary.
Session 08 already tried the script route and made it worse: `[System.IO.File]::WriteAllLines`
with no encoding argument ate a byte and turned a clean file into a binary one.

What is needed is a decision, not an effort:

1. **Rewrite the file wholesale** with the `write` tool, hand-transcribing all ~14 KB. Works, but
   a 14 KB hand transcription is exactly the class of error this project forbids, and it would be
   done blind because the read tool also refuses.
2. **One deliberate, documented exception** to the edit-tool rule for byte-level repair, using
   the `write` tool to emit the corrected text and `git diff --stat` to confirm the change is
   confined to the six damaged spots. Lowest risk of the three.
3. **Leave it.** The build is clean apart from one cosmetic warning, and the file is not on the
   porting path.

Recommendation: option 2, one time, with the diff reviewed.

**RESOLVED (session 10) -- the reviewer found the cause, and it vindicates the edit-tool rule.**

All six defects are in doc comments, at exactly the positions where a **Markdown backtick**
opened a code span: `` `f32::MIN_POSITIVE` `` and `` `0x00800000` ``. In a **PowerShell
double-quoted string the backtick is the escape character**, so:

| source | PowerShell double-quoted string | result |
|---|---|---|
| `` `0 `` | escape `\0` | **NUL byte** |
| `` `f `` | escape `\f` | **form feed** `0x0C` |
| `` `l ``, `` `M ``, any other | not a recognised escape | backtick **silently dropped** |

That accounts for all six: three NULs (`0x00000001`, `0x00800000` twice), three form feeds, and
three vanished characters leaving `32::MIN_POSITIVE` instead of `` `f32::MIN_POSITIVE` ``. Every
one sits where a code span began.

So the mechanism was **a PowerShell write of Rust doc comments containing backticks** -- exactly
the operation session-09's rule 2 forbids. Three damages in three sessions, and the two most recent
share this root cause. The rule is not caution; it is the result.

Repaired under the reviewer's one-time exception: a byte-level pass restores `0` and `f`, then the
edit tool puts the backticks back and removes the duplicate `#[test]`. The NUL/form-feed count is
asserted in CI, so a recurrence fails the build instead of quietly turning a source file binary.

**How the corruption got in is still unexplained**, and that matters more than the fix. It is not
session 08's `WriteAllLines` -- that was reverted with `git checkout`, and these bytes are in the
committed tree at `ec2c610`. Candidates: an earlier session's PowerShell write with a non-UTF8
encoding, or `core.autocrlf` mangling a NUL during checkout. Worth knowing, because if it is
recurring then other files may be quietly damaged too. A cheap audit: `git grep -lP '\x00' -- '*.rs'`.
