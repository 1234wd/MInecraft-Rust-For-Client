# Porting plan — ordered bottom-up
---

---


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

## Batch 2 — core value types (IN PROGRESS — 2 of 8 done)

Unblocks batch 1's leftovers and is a prerequisite for almost everything else.
Small, closed, high fan-out.

**status**

| java_path | rust_path | status | notes |
|---|---|---|---|
| `net/minecraft/core/Vec3i.java` | same | **PARTIAL** | everything but the codec fields |
| `net/minecraft/core/Direction.java` | same | **PARTIAL** | everything but the codec fields |
| `net/minecraft/core/BlockPos.java` | same | SKELETON | next — bit packing is the load-bearing part |
| `net/minecraft/core/Direction.java`'s `ChunkPos` | `world/level/ChunkPos.java` | SKELETON | |
| `net/minecraft/world/phys/Vec3.java` | same | SKELETON | `double` throughout |
| `net/minecraft/world/phys/AABB.java` | same | SKELETON | |
| `net/minecraft/resources/Identifier.java` | same | SKELETON | needs `Result` + message text |
| `net/minecraft/util/ARGB.java` | same | SKELETON | |
| `world/level/levelgen/SectionPos` / `world/phys/Vec2` / `util/Rotations` | — | SKELETON | deferred |

`Vec3i` and `Direction` are `PARTIAL` rather than `VERIFIED` because their
`Codec` / `StreamCodec` fields are still `todo!("PORT: codec — ...")`, per the batch
rule. Everything that *is* implemented is oracle-verified: 48 golden groups in
`core.txt`, 320 604 rows, driven by `_porting/tests/parity_core.rs`.

**files**
```
net/minecraft/core/Vec3i.java          net/minecraft/world/phys/Vec3.java
net/minecraft/core/Direction.java      net/minecraft/world/phys/AABB.java
net/minecraft/core/BlockPos.java       net/minecraft/world/phys/Vec2.java
net/minecraft/world/level/ChunkPos.java
net/minecraft/resources/Identifier.java
net/minecraft/resources/ResourceKey.java
net/minecraft/util/ARGB.java           net/minecraft/util/ColorRGBA.java
net/minecraft/util/SimpleBitStorage.java
net/minecraft/util/ByIdMap.java         net/minecraft/core/IdMapper.java
```

**proof**
- new oracle groups: `vec3i.*`, `direction.*`, `blockpos.*`, `chunkpos.*`, `vec3.*`,
  `aabb.*`, `argb.*`, `identifier.*` — all generated in `core.txt`
- close out `mth.txt` groups `getSeed`, `lerp`, `rayIntersectsAABB`,
  `rotationAroundAxis`, `mulAndTruncate`
  (group names corrected in session 06: they were `getSeedVec3i` / `lerpVec3`, which are
  names I invented and which **do not exist** in 26.2 -- verify with
  `python _porting/tools/api_list.py Mth`)
- empty `BLOCKED_ON_UNPORTED_TYPES` in `parity_core.rs`

**watch out — all four of these bit us in session 03**
- **`hashCode` is not arbitrary.** `Vec3i#hashCode` is `(y + z*31)*31 + x` — z and y
  weighted above x. Not `x*961 + y*31 + z`. It is the value Java's `HashMap` buckets on.
- **`Enum ordinals are persisted.** `Direction` order is `DOWN, UP, NORTH, SOUTH, WEST,
  EAST` and `data3d` equals it; `AxisDirection` is `POSITIVE, NEGATIVE` so POSITIVE is
  ordinal **0**. `Direction#from2DDataValue` filters to horizontal FIRST, so its modulus
  is 4, not 6.
- **Width changes hide in plain sight.** `Vec3i#distManhattan` takes `Math.abs` on the
  **int** and only then widens to `float`; `distChessboard` never leaves `int`. They
  disagree above 2^24, and at the world border.
- **`getApproximateNearest` seeds with `Float.MIN_VALUE`**, a tiny *positive* denormal,
  not `-Infinity` — so a zero dot never wins.

**Oracle harness traps, all of which fail SILENTLY rather than loudly**
- `Out` attributes rows to the most recent `fn(...)` header, so emitting two methods in
  one loop files everything under the last header: empty groups, and a parity test that
  passes by testing nothing. Every group is now checked non-empty.
- Rows with no arguments are ambiguous — record the discriminator (e.g. the ordinal)
  as an argument.
- `relative(Direction,int)` and `relative(Axis,int)` produce identical five-argument
  rows. Split them into separate groups.

---

## Batch 3 — NBT (next session — recommended)

Batch 2 is **half done**. `BlockPos`, `ChunkPos`, `Vec3`, `AABB`, `ARGB` and
`Identifier` are still `SKELETON` — **finish those first**, they are small, their
golden data already exists in `core.txt`, and `BlockPos` in particular is on a very hot
path. Then NBT.

**files (exact)**
```
net/minecraft/nbt/Tag.java                      net/minecraft/nbt/CompoundTag.java
net/minecraft/nbt/ListTag.java                  net/minecraft/nbt/ByteTag.java
net/minecraft/nbt/ShortTag.java                 net/minecraft/nbt/IntTag.java
net/minecraft/nbt/LongTag.java                  net/minecraft/nbt/FloatTag.java
net/minecraft/nbt/DoubleTag.java                net/minecraft/nbt/ByteArrayTag.java
net/minecraft/nbt/IntArrayTag.java              net/minecraft/nbt/LongArrayTag.java
net/minecraft/nbt/StringTag.java                net/minecraft/nbt/IntHolder.java
net/minecraft/nbt/CompoundTagUtil.java          net/minecraft/nbt/NumericTag.java
net/minecraft/nbt/NbtIo.java                    net/minecraft/nbt/NbtAccounter.java
net/minecraft/nbt/SnbtGrammar.java              net/minecraft/nbt/TagParser.java
net/minecraft/nbt/visitors/*.java               (7 files)
net/minecraft/core/Holder.java                  net/minecraft/core/HolderOwner.java
net/minecraft/core/HolderLookup.java            net/minecraft/resources/ResourceKey.java
net/minecraft/core/MappedRegistry.java          net/minecraft/core/Registry.java
```
Excluded deliberately: `net/minecraft/util/datafix/` (396 files) and anything under
`net/minecraft/network/codec/` — both need the DFU decision in
`DESIGN_DECISIONS.md` (#dfu-proposal) first.

**proof**
- new golden groups: `nbt.roundtrip` (write a tree from Java, read it in Rust,
  compare tag-for-tag), `nbt.io` (raw byte-for-byte `NbtIo` output, both compressed and
  not), `snbt.parse` / `snbt.error` (the parser *and its error messages*).
- the minimum registry surface so `CompoundTag` can hold a `Holder`.

**watch out**
- **Big-endian on the wire.** `NbtIo` writes lengths and string bytes big-endian, and
  string lengths are UTF-16 code-unit counts over modified-UTF-8. This is a *file
  format*, so it must match byte for byte, not just semantically.
- **`CompoundTag` iteration order.** The last-inserted key wins on duplicate keys, and
  vanilla relies on `LinkedHashMap` ordering in places. Do NOT use `HashMap` (see
  `OPEN_QUESTIONS.md` #7).
- **Tag ids are persisted.** The `TAG_*` constants are a wire format, exactly like
  `Direction`'s ordinals.
- `SnbtGrammar` is 909 LOC and is the real parser; `TagParser` is the lenient
  user-input variant. They disagree on several inputs on purpose.
- gzip: use the same DEFLATE settings or `level.dat` will differ in bytes (not in
  content). Check whether vanilla pins the compression level.

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
   DESIGN_DECISIONS.md if it is a general rule.

---

   `DESIGN_DECISIONS.md` if it is a general rule.## Batch 3 � NBT (next session � recommended)

Batch 2 is half done; `BlockPos`, `ChunkPos`, `Vec3`, `AABB`, `ARGB` and `Identifier`
are still `SKELETON`. **Finish those first** � they are small, their golden data already
exists in `core.txt`, and BlockPos in particular is on a very hot path. Then:

**files (exact)**
```
net/minecraft/nbt/Tag.java                      net/minecraft/nbt/CompoundTag.java
net/minecraft/nbt/ListTag.java                  net/minecraft/nbt/ByteTag.java
net/minecraft/nbt/ShortTag.java                 net/minecraft/nbt/IntTag.java
net/minecraft/nbt/LongTag.java                  net/minecraft/nbt/FloatTag.java
net/minecraft/nbt/DoubleTag.java                net/minecraft/nbt/ByteArrayTag.java
net/minecraft/nbt/IntArrayTag.java              net/minecraft/nbt/LongArrayTag.java
net/minecraft/nbt/StringTag.java                net/minecraft/nbt/IntHolder.java
net/minecraft/nbt/CompoundTagUtil.java          net/minecraft/nbt/NumericTag.java
net/minecraft/nbt/NbtIo.java                    net/minecraft/nbt/NbtAccounter.java
net/minecraft/nbt/NbtAccounter.java
net/minecraft/nbt/SnbtGrammar.java              net/minecraft/nbt/TagParser.java
net/minecraft/nbt/visitors/*.java               (7 files)
net/minecraft/core/Holder.java                  net/minecraft/core/HolderOwner.java
net/minecraft/core/HolderLookup.java            net/minecraft/resources/ResourceKey.java
net/minecraft/core/MappedRegistry.java          net/minecraft/core/Registry.java
```

**proof**
- new golden groups: `nbt.roundtrip` (write a tree from Java, read it in Rust,
  compare tag-for-tag), `nbt.io` (raw byte-for-byte `NbtIo` output, both compressed and
  not), `snbt.parse` / `snbt.error` (the parser *and its error messages*).
- the minimum registry surface so `CompoundTag` can hold a `Holder`.

**watch out**
- **Big-endian on the wire.** `NbtIo` writes lengths and string bytes big-endian, and
  string lengths are UTF-16 code-unit counts over modified-UTF-8. This is a *file
  format*, so it must match byte for byte, not just semantically.
- **`CompoundTag` iteration order.** The last-inserted key wins on duplicate keys, and
  vanilla relies on `LinkedHashMap` ordering in places. Do NOT use `HashMap` (see
  OPEN_QUESTIONS #7).
- **Tag ids are persisted.** The `TAG_*` constants are a wire format, like
  `Direction`'s ordinals.
- `SnbtGrammar` is 909 LOC and is the real parser; `TagParser` is the lenient
  user-input variant. They disagree on several inputs on purpose.
- gzip: use the same DEFLATE settings or `level.dat` will differ in bytes (not in
  content). Check whether vanilla fixes the compression level.
---
# Porting plan — ordered bottom-up

---
