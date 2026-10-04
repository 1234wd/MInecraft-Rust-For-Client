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