# Porting plan — ordered bottom-up

Dependency order, not file-size order. Each batch lists the exact files, the golden
data that proves it, and what "done" means.

Legend for the **proof** column: the golden group name in `_porting/test-data/`.

---

## Batch 0 — infrastructure ✅ DONE (session 02)

- `_porting/tools/mirror.py`, `check_mirror.py`, `gen_tables.py`
- `_porting/java-oracle/` (compiles vanilla, 49 847 golden rows)
- `_porting/javacompat/` (`java_lang`, `java_random`, `md5`, `golden`)
- 7055 `.rs` skeletons + 551 `mod.rs`; `cargo build` clean in 3.6 s

---

## Batch 1 — util + RNG ✅ DONE (session 02)

Everything here is `VERIFIED`: bit-exact against the oracle.

| java_path | status | proof |
|---|---|---|
| `net/minecraft/util/Mth.java` | VERIFIED | `mth.txt`: 110 groups; `sinTableHash` covers all 65 536 LUT entries |
| `net/minecraft/util/RandomSource.java` | VERIFIED | `random.txt`: `legacy.*`, `xoroshiro.*`, … |
| `net/minecraft/util/LinearCongruentialGenerator.java` | VERIFIED | `lcg.next` |
| `net/minecraft/world/level/levelgen/BitRandomSource.java` | VERIFIED | `*.nextBits`, `bit.*` |
| `net/minecraft/world/level/levelgen/LegacyRandomSource.java` | VERIFIED | `legacy.*`, `legacyPos.*` |
| `net/minecraft/world/level/levelgen/SingleThreadedRandomSource.java` | VERIFIED | `single.*` |
| `net/minecraft/world/level/levelgen/ThreadSafeLegacyRandomSource.java` | VERIFIED | `threadsafe.*` |
| `net/minecraft/world/level/levelgen/Xoroshiro128PlusPlus.java` | VERIFIED | `xoroshiro128pp`, `xoroshiro128pp.zero` |
| `net/minecraft/world/level/levelgen/XoroshiroRandomSource.java` | VERIFIED | `xoroshiro.*`, `xoroshiroPos.*` |
| `net/minecraft/world/level/levelgen/RandomSupport.java` | VERIFIED | `mixStafford13`, `upgradeSeedTo128bit*`, `seedFromHashOf`, `seed128bit_*` |
| `net/minecraft/world/level/levelgen/MarsagliaPolarGaussian.java` | VERIFIED | exercised by `*.nextGaussian` |
| `net/minecraft/world/level/levelgen/PositionalRandomFactory.java` | VERIFIED | `*Pos.at/fromHashOf/fromSeed/parityConfigString` |
| `net/minecraft/world/level/levelgen/WorldgenRandom.java` | VERIFIED | `worldgen.*` |

**Still open in this batch** (see `OPEN_QUESTIONS.md`): the 5 `Mth` methods blocked
on `Vec3i`/`Vec3`/`AABB`/JOML/`Fraction`, and `Mth#mulAndTruncate`.

---

## Batch 2 — core value types (next session — recommended)

Unblocks batch 1's leftovers and is a prerequisite for almost everything else.
Small, closed, high fan-out.

**files**
```
net/minecraft/core/Vec3i.java          net/minecraft/world/phys/Vec3.java
net/minecraft/core/Direction.java     net/minecraft/world/phys/AABB.java
net/minecraft/core/BlockPos.java      net/minecraft/world/phys/Vec2.java
net/minecraft/resources/Identifier.java
net/minecraft/resources/ResourceKey.java
net/minecraft/util/ARGB.java          net/minecraft/util/ColorRGBA.java
net/minecraft/util/SimpleBitStorage.java
net/minecraft/util/ByIdMap.java        net/minecraft/core/IdMapper.java
```

**proof**
- new oracle groups: `Vec3.*`, `AABB.*`, `Identifier.parse/tryParse/toString`,
  `Direction.values/from2DDataValue/...`, `ARGB.*`
- close out `mth.txt` groups `getSeedVec3i`, `lerpVec3`, `rayIntersectsAABB`
- drop 3 entries from `BLOCKED_ON_UNPORTED_TYPES` in `parity_mth.rs`

**watch out**
- `Identifier` has its own validation grammar (`[a-z0-9._-]` namespace, `/` path,
  `:` separator). It is on a seeded path via `PositionalRandomFactory#fromHashOf`.
- `Direction` is an enum; its `values()` ORDER is persisted in saves.

---

## Batch 3 — NBT

```
net/minecraft/nbt/*.java                    (34 files)
net/minecraft/nbt/visitors/*.java           (7)
net/minecraft/nbt/SnbtGrammar.java          (909 LOC — the real parser)
net/minecraft/nbt/TagParser.java
```

**proof** — round-trip oracle: write a NBT tree from Java, read it in Rust and
compare tag-for-tag, including `LongArrayTag`/`IntArrayTag` binary layout.
**watch out** — the binary format is a file format, so it must match byte for byte
(§"same file formats"). Endianness is big-endian; string lengths are UTF-16 code
units modified-UTF-8.

---

## Batch 4 — registries + `ResourceLocation`

```
net/minecraft/core/Holder*.java      net/minecraft/core/MappedRegistry.java
net/minecraft/core/Registry.java     net/minecraft/core/IdMap.java
net/minecraft/core/DefaultedRegistry.java  net/minecraft/resources/ResourceKey.java
```

**proof** — golden registry snapshots: registration order, assigned IDs, and
`IdMap` lookup round-trip.
**watch out** — **registry ID order is persisted in save data**. Any reordering
corrupts existing worlds. See `OPEN_QUESTIONS.md` #7 about iteration order.

---

## Batch 5 — codecs + data-fixer

```
net/minecraft/network/codec/*.java     net/minecraft/util/Codec*.java
net/minecraft/util/datafix/*.java      (396 files, large — consider splitting)
net/minecraft/util/ExtraCodecs.java
```

**proof** — golden JSON <-> value round-trips per codec.
**watch out** — this is where the `Codec` stubs in the oracle get replaced by real
DFU semantics (batch 1's `OPEN_QUESTIONS.md` #8).

---

## Batch 6 — block states + chunks

```
net/minecraft/world/level/block/state/**/*.java
net/minecraft/world/level/chunk/**/*.java
net/minecraft/world/level/block/Block.java  BlockBehaviour.java
net/minecraft/world/level/block/BlockState.java
```

**proof** — golden block-state property tables and a chunk serialise/deserialise
round-trip.
**watch out** — property iteration order affects model baking order (cosmetic) but
state ID order is persisted.

---

## Batch 7 — worldgen

```
net/minecraft/world/level/levelgen/**/*.java
net/minecraft/world/level/levelgen/feature/**, placement/**, carver/**
net/minecraft/world/level/biome/*.java
```

**proof** — **the real acceptance test: generate a world from a fixed seed in both
Java and Rust and compare the block hash.** That is the single test that proves
"same seed = same world" end to end. Everything above is a means to that end.
**watch out** — `DensityFunction` trees, `NoiseRouterData` constants, and the
decorator ordering are all order-sensitive.

---

## Batch 8 — entities

```
net/minecraft/world/entity/**/*.java    (716 files)
net/minecraft/world/entity/ai/**, item/**, player/**
```

**proof** — golden entity AI-goal ordering and a tick-count transcript (spawn →
N ticks → dump position/health/AI state).
**watch out** — `updateInterval`/`randomFrequency` stutter behaviour; attribute
modifiers.

---

## Batch 9 — server tick

```
net/minecraft/server/level/ServerLevel.java  net/minecraft/server/MinecraftServer.java
net/minecraft/server/players/**               net/minecraft/world/ticks/**
net/minecraft/world/level/Level.java
```

**proof** — a multi-tick transcript: run the server N ticks from a fixed seed and
compare the full observable state dump.
**watch out** — "keep the same threading model" (Prime Directive). Vanilla's
server thread must stay the owner of world state.

---

## Batch 10 — network

```
net/minecraft/network/**               (FriendlyByteBuf, codecs, protocols)
net/minecraft/server/network/**        net/minecraft/client/multiplayer/**
```

**proof** — **byte-level**: capture a real packet stream from the Java server and
assert the Rust client parses and re-serialises identical bytes.
**watch out** — "same network protocol as vanilla" is a hard requirement. VarInt,
VarLong, compression threshold and packet order must all match.

---

## Batch 11 — client

```
net/minecraft/client/**                (2 000+ files)
com/mojang/blaze3d/**                  net/minecraft/client/renderer/**
```

**proof** — screenshots and input-response traces compared against vanilla.
**watch out** — this is where `HashMap` ordering becomes visible in the frame
profile, and where JOML replacement is decided (`OPEN_QUESTIONS.md` #4).

---

## Cross-cutting rules (apply to every batch)

1. **Golden first.** Extend the oracle in the same commit as the port, so there is
   never a moment where "ported" and "verified" disagree.
2. **`cargo build`, `cargo test`, `check_mirror.py` must all pass before every commit.**
3. **Un-overloadable methods are the bug farm.** Operator precedence, mixed
   int/long widths, and float narrowing all bit us this session; the golden harness
   caught every one.
4. **When in doubt, measure the JVM at runtime** — not with `javac` constants, which
   fold and lie.
5. **Record it.** Every trap goes into the ported file's docs and into
   `DESIGN_DECISIONS.md` if it is a general rule.