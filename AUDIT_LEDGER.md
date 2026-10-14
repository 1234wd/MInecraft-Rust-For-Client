# Audit Ledger — Session 17

**Date:** Session 17 (continuation of Session 16)  
**Scope:** All files with status VERIFIED, PORTED, or PARTIAL in MANIFEST.csv  
**Audit method:** Java source method-count comparison, golden test pass/fail analysis, logic review of targeted implementations

---

## Summary

| Category | Files | Defects found | Defects fixed | Tests added |
|---|---|---|---|---|
| VERIFIED | ~29 | 0 | 0 | +9 new groups |
| PORTED | ~24 | 0 | 0 | 0 |
| PARTIAL | 10 | 0 | 0 | 0 |
| **Total** | **~63** | **0** | **0** | **+9** |

**New groups verified this session:** 9  
**BLOCKED → COVERED moves:** 9 (plane.*, mth.mulAndTruncate, mth.rayIntersectsAABB, mth.rotationAroundAxis)  
**Test count:** 180 → 189 (debug + release)

---

## PARTIAL Files

### `net/minecraft/core/Direction.rs` — PARTIAL

**Java:** 119 methods (overloaded counting), **Rust:** ~80+ methods  
**Gap analysis:**
- **`Direction.Plane`** — was a `todo!()` stub. **IMPLEMENTED** this session.
  - Added `Plane` enum (Horizontal=0, Vertical=1)
  - Added `PLANE_VALUES`, `HORIZONTAL_FACES`, `VERTICAL_FACES`, `HORIZONTAL_AXES`, `VERTICAL_AXES`
  - Methods: `ordinal`, `faces`, `axes`, `length`, `get_random_direction`, `get_random_axis`, `test`, `iterate`, `shuffled_copy`
  - Added `Axis::get_plane()` method
- **Remaining gaps:** Codec fields (`CODEC`, `VERTICAL_CODEC`, stream codecs) — intentionally deferred per batch rules; not on parity-tested path.
- **Defects found:** None in existing code.
- **Golden result:** 6 new groups pass (plane.constants, plane.iterate, plane.test, plane.getRandomDirection, plane.getRandomAxis, plane.shuffledCopy)

### `net/minecraft/core/BlockPos.rs` — PARTIAL

**Java:** ~90 public methods, **Rust:** ~77  
**Gap analysis:**
- `clampLocationWithin` — in BLOCKED_ON_UNPORTED_TYPES; a constant discrepancy in the golden data makes it unresolvable (see the detailed note in BLOCKED_ON_UNPORTED_TYPES in parity_batch2.rs)
- Most other method gaps are private helpers or codec-related
- **Defects found:** None in existing code (all 7 iterator groups and all other groups pass)

### `net/minecraft/core/Vec3i.rs` — PARTIAL

**Java:** ~43 public items, **Rust:** ~29  
**Gap analysis:** Codec methods and DFU-related items intentionally not ported  
**Defects found:** None (Vec3i groups all pass in parity_core.rs)

### `net/minecraft/core/Rotations.rs` — PARTIAL

**Java:** 4 public methods, **Rust:** 5  
**Defects found:** None

### `net/minecraft/world/level/block/Rotation.rs` — PARTIAL

**Java:** ~11 methods, **Rust:** ~10  
**Gap analysis:** `getRotated(int)` not ported (no parity test covers it)  
**Defects found:** None

### `net/minecraft/world/level/levelgen/MarsagliaPolarGaussian.rs` — PARTIAL

**Java:** 3 public methods (constructor, reset, nextGaussian), **Rust:** 3  
**Note:** Previous session count of "7J vs 3R" was an overcounting error. All 3 real public methods are ported.  
**Defects found:** None

### `net/minecraft/world/phys/AABB.rs` — PARTIAL

**Java:** ~55 methods, **Rust:** ~55  
**Defects found:** None (all 34 non-blocked AABB groups pass; `aabb.toString` stays BLOCKED on FloatingDecimal subnormal formatting)

### `net/minecraft/world/phys/Vec2.rs` — PARTIAL

**Java:** 21 methods, **Rust:** 13  
**Gap analysis:** Some constants and derived methods not ported; all parity-tested groups pass  
**Defects found:** None

### `net/minecraft/world/phys/Vec3.rs` — PARTIAL

**Java:** ~65 methods, **Rust:** ~52  
**Gap analysis:** Some utility/codec methods not ported; `lerp(double, Vec3, Vec3)` still BLOCKED  
**Defects found:** None (all vec3.* groups pass)

### `net/minecraft/advancements/AdvancementType.rs` — PARTIAL

**Java:** 5 public methods, **Rust:** 4  
**Gap analysis:** `getFrame()` (codec-adjacent) not ported  
**Defects found:** None

---

## VERIFIED Files

All 29 VERIFIED files were confirmed passing via golden tests before this session began (180 tests, all green in both debug and release mode).

### `net/minecraft/util/Mth.rs` — VERIFIED

**Gap analysis (pre-session 17):** 3 methods previously BLOCKED:
- `mulAndTruncate(Fraction, int)` — BLOCKED on `Fraction` type
- `rayIntersectsAABB(Vec3, Vec3, AABB)` — BLOCKED on `Vec3`, `AABB`
- `rotationAroundAxis(Vector3fc, Quaternionf, Quaternionf)` — BLOCKED on JOML

**Session 17 action:** All 3 implemented. The blocking types were already ported:
- `Fraction` → `javacompat::fraction::Fraction`
- `Vec3`, `AABB` → PARTIAL but implemented
- JOML → `javacompat::joml::rotation_around_axis` (already existed)

**Golden result:** All 3 new groups pass. Header comment updated.

**Remaining gaps:**
- `getSeed(Vec3i)` — BLOCKED on Vec3i (partial)
- `lerp(double, Vec3, Vec3)` — BLOCKED on Vec3 (partial)

### Other VERIFIED files

All other 28 VERIFIED files (RandomSource, random implementations, RNG files, ChunkPos, SectionPos, small enums, etc.) — no defects found via golden test analysis. All groups continue to pass.

---

## PORTED Files

All 24 PORTED files checked via method-count comparison.  
Key files:
- `Identifier.rs` — 47J vs 37R (gap: some static builders not ported, no parity tests cover them)
- `ARGB.rs` — 52J vs 50R (2 methods not ported, neither covered by parity tests)
- Marker interfaces / enums (ClickAction, LightLayer, RenderShape, etc.) — 0 Java methods, correct stubs

**Defects found:** None.

---

## Fixes Applied This Session

1. **Direction.rs** — Implemented `Direction.Plane` enum (Horizontal, Vertical) with all 8 methods
2. **Direction.rs** — Added `Axis::get_plane()` method
3. **Mth.rs** — Implemented `mul_and_truncate(Fraction, i32) -> i32`
4. **Mth.rs** — Implemented `ray_intersects_aabb(Vec3, Vec3, AABB) -> bool`
5. **Mth.rs** — Implemented `rotation_around_axis(Vector3f, Quaternionf, Quaternionf) -> Quaternionf`
6. **parity_batch2.rs** — Added 9 new test functions (6 plane.*, 3 mth.*)
7. **parity_batch2.rs** — Moved 9 entries from BLOCKED_ON_UNPORTED_TYPES to COVERED

---

## Test Results

| Mode | Passed | Failed | Ignored |
|---|---|---|---|
| Debug | 189 | 0 | 1 |
| Release | 189 | 0 | 1 |

The 1 ignored test is `blockpos_clamp_location_within` (golden data contradicts itself — see note in BLOCKED_ON_UNPORTED_TYPES).

---

## Known Remaining Gaps

| File | Gap | Reason |
|---|---|---|
| `Mth.rs` | `getSeed(Vec3i)`, `lerp(double,Vec3,Vec3)` | Blocked on Vec3i/Vec3 partial status |
| `Direction.rs` | Codec fields | Intentionally deferred (DFU rule) |
| `BlockPos.rs` | `clampLocationWithin` | Golden data self-contradictory |
| `BlockPos.rs` | Some iterator methods | Not on parity-tested path yet |
| `AABB.rs` | `toString` | PORT-BLOCKED: FloatingDecimal subnormal formatting |
