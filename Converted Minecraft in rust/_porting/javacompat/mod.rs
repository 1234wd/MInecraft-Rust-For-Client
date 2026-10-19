//! Java standard-library and third-party compatibility layer.
//!
//! NOT game code. These modules have no `.java` counterpart in
//! `minecraft-decompiled/`, so per Rule 3 they live in `_porting/javacompat/`
//! rather than in the mirror tree. `lib.rs` pulls them in with `#[path]`.
//!
//! See _porting/DESIGN_DECISIONS.md (#javacompat) for why each one exists and what
//! it would break if we used the Rust equivalent instead.
//!
//! | module | what it ports | pinned to |
//! |---|---|---|
//! | [`java_lang`] | `Math` floor/abs/min/max, narrowing casts, `String.hashCode`, NaN-sign-correct arithmetic | JDK 21/25 semantics |
//! | [`java_io`] | `DataInput`/`DataOutput`, big-endian primitives, Java's *modified* UTF-8 | JDK 25 |
//! | [`java_random`] | the 48-bit `java.util.Random` LCG | JDK reference |
//! | [`md5`] | RFC 1321 MD5 + `Longs.fromBytes` ordering | Guava 33.6.0-jre |
//! | [`fraction`] | `org.apache.commons.lang3.math.Fraction` (partial) | commons-lang3 3.20.0 |
//! | [`joml`] | the JOML subset `Mth` reaches, including `Math.fma` | joml 1.10.8 |
//! | [`entropy`] | `System.nanoTime`, `ThreadLocalRandom` seeding | n/a (injectable) |
//! | [`nan_policy`] | *which* NaN bits must match, and where | n/a (policy) |
//! | [`golden`] | the golden-data reader used by the parity tests | n/a |
//! | [`throwable`] | `Throwable#toString` and the `Throwable(Throwable)` message rule | JDK 25 |
//! | [`java_object`] | boxed `hashCode`s, `Objects.hash`, record `hashCode`/`equals` | JDK 25 |
//! | [`time_unit`] | `java.util.concurrent.TimeUnit` (conversion from nanoseconds) | JDK 25 |
//! | [`bit_set`] | `java.util.BitSet` (the subset `CarvingMask` uses) | JDK 25 |
//! | [`java_hash_map`] | `java.util.HashMap` with its exact iteration order, tree bins included (the subset `CompoundTag` uses) | JDK 25 |
//! | [`fastutil`] | `HashCommon.mix` (the fastutil methods vanilla calls, one by one) | fastutil 8.5.18 |
//!
//! Every "pinned to" version is read from the Fabric Loom / Gradle cache, i.e. what
//! Minecraft 26.2 actually resolves -- see `java-oracle/fetch_libs.ps1`.

pub mod bit_set;
pub mod entropy;
pub mod fastutil;
pub mod fraction;
pub mod golden;
pub mod java_hash_map;
pub mod java_io;
pub mod java_lang;
pub mod java_object;

#[path = "jvm_math.rs"]
pub mod jvm_math;
pub mod java_random;
pub mod joml;
pub mod md5;
pub mod nan_policy;
pub mod throwable;
pub mod time_unit;
