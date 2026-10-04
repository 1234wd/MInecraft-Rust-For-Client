# Open questions â€” for the human, not for me

Flagged rather than guessed, per the no-guessing rule. Each says what I need and
what I did in the meantime.

---

## 1. `RandomSource#create()` uses `System.nanoTime()` â€” no portable Rust equivalent

`RandomSupport#generateUniqueSeed()` XORs a global uniquifier with
`System.nanoTime()`. Java's `nanoTime` is an arbitrary monotonic origin; Rust's
`Instant::now()` is also arbitrary but *not* the same arbitrary value, and
`std::time::SystemTime` is a wall clock that can jump.

**Current behaviour:** I use `SystemTime::now()` nanoseconds since the UNIX epoch.
**Consequence:** two Rust processes started at the same instant on the same tick
would produce the same uniquifier-derived seed. Vanilla has the same theoretical
window with its 1 ms-resolution counter, so this is not a practical regression â€”
but it is not bit-identical.

**Decision needed:** acceptable, or should `create_unseeded()` take an explicit seed
from the caller and drop `System.nanoTime` entirely? **This function is deliberately
NOT parity-tested** (it is non-deterministic by construction).

## 2. `nextInt(bound <= 0)` panics â€” do you want `Result`?

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

## 4. JOML in the client crate â€” replace or reimplement?

`Mth#rotationAroundAxis` takes `org.joml.Quaternionf` / `Vector3fc`, and the client
renderer needs far more of JOML (`Matrix4f`, `Matrix3f`, `Vector4f`, â€¦). The oracle
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
`Identifier` whose `toString()` is `namespace + ":" + path` â€” accurate, but testing
against my own stub would test the stub.

`fromHashOf(String)` â€” the seeded path that actually drives worldgen â€” **is**
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

## 8. `codec-not-ported` â€” the `CODEC` static fields

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

## 11. ANSWERED (session 03) — NaN policy

As directed: exact NaN sign and payload are matched only where game code can observe
them (`floatToRawIntBits`/`doubleToRawLongBits` results, hashing, serialisation,
network). Everywhere else the parity test asserts only that both sides are NaN.

`javacompat::nan_policy` implements the policy and documents that **the JVM's own NaN
sign is CPU-dependent** — x86-64 raises the negative real indefinite for an invalid
operation, AArch64 raises the positive default. **We target x86-64 HotSpot**, and the
oracle measures exactly that.

Closed because it was a policy question with a defensible answer, not a fork in the road.

## 12. ANSWERED (session 03) — entropy injection

Done: `javacompat::entropy` is the single funnel for `System.nanoTime`,
`currentTimeMillis` and thread-local entropy. `RandomSupport#generateUniqueSeed` now
calls `entropy::nano_time()` instead of `SystemTime::now()` directly.

This closes question 1 as far as it can be closed. The residual divergence is
**documented, not eliminated**: Java's `nanoTime` counts from an arbitrary JVM-chosen
origin and Rust cannot reproduce that number, so `generateUniqueSeed` returns a
different value than vanilla on the same call. It is unobservable by construction —
that is the point of the function. What is now guaranteed is that it is *injectable*,
so tests are deterministic and the remaining clock reads are auditable in one file.

If you want bit-identical `generateUniqueSeed` anyway, the only route is to drop
`nanoTime` and take an explicit seed from the caller. **Say so and I will do it.**

## 13. ANSWERED (session 03) — JOML

Decided: port the subset from JOML 1.10.8's own bytecode into
`_porting/javacompat/joml/`, do **not** use `glam`/`nalgebra`. The deciding factor is
concrete: `Vector3f#dot` and `Quaternionf#normalize` are built on `Math.fma`, an IEEE
fused multiply-add that rounds once; a general Rust math library would not reproduce
it, and its FMA-contraction behaviour is not something we can audit.

`invsqrt`, `fma`, `Vector3f::dot` and `Quaternionf::normalize/set` are ported and
unit-tested. The rationale is in DESIGN_DECISIONS.md (#joml-subset-ported-not-swapped).

## 14. STILL OPEN — the codec/serialisation decision needs a call from you

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

## 15. NEW (session 03) — `Util.java` does not compile under JDK 25

`Util#makeEnumMap` contains

```java
for (K key : (Enum[])keyType.getEnumConstants()) {   // K extends Enum<K>
```

which JDK 25's javac rejects: `incompatible types: Enum cannot be converted to K`.
This is a decompiler artefact — Mojang's real source has no such cast.

**Current handling:** `Util` is NOT in the oracle's compile list; the real
`minecraft-merged-deobf` jar supplies it, since it is only reached for trivial
delegation (`Util.make`) and is not a batch-2 port target.

**Question:** if a future batch needs to test something `Util` actually computes,
we will have to either compile a patched copy (breaking the "unmodified sources" claim
for that one file) or mirror it into `javacompat` as third-party-ish code. **I would
prefer mirroring into `javacompat` with a comment naming the artefact**, but that is a
rule change and should be your call.