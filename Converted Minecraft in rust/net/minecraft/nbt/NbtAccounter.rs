//! Port of: net/minecraft/nbt/NbtAccounter.java
//! Java class(es): net.minecraft.nbt.NbtAccounter
//! Status: VERIFIED
//!
//! Session 21. Every member ported: the two-arg constructor, the four factories, both
//! `accountBytes` overloads, `pushDepth`/`popDepth` and the two `@VisibleForTesting`
//! getters. Verified against the `ACC` rows of `_porting/test-data/nbt_probe.txt`
//! (JDK 25.0.4.1, 26.2 jar) by `_porting/tests/session19_g.rs`.
//!
//! This is the quota meter every `TagType.load` calls before it allocates, so it is the
//! thing standing between a malicious region file and the heap. Two of its three
//! arithmetic lines overflow on purpose, and a port that "fixes" either one changes which
//! files the game will load.
//!
//! # `usage + size > quota` IS A WRAPPING COMPARISON
//!
//! ```java
//! if (this.usage + size > this.quota) { throw ... }
//! this.usage += size;
//! ```
//!
//! `unlimitedHeap()` sets `quota = Long.MAX_VALUE`. Account `Long.MAX_VALUE` and the sum is
//! exactly the quota, so it passes and `usage == Long.MAX_VALUE`. Account **one more byte**
//! and `usage + size` wraps to `Long.MIN_VALUE`, which is not `> Long.MAX_VALUE` -- so it
//! passes too, and `usage` becomes negative. The accounter silently stops accounting.
//!
//! The oracle says so in as many words:
//!
//! ```text
//! ACC  overflow  ok:9223372036854775807;ok:-9223372036854775808;ok-wrap;
//! ```
//!
//! So the comparison is [`i64::wrapping_add`], not a checked add and not a widening to
//! `i128`. Either of those would reject the second call and refuse to read a file vanilla
//! reads. This is a real (if unreachable-in-practice) vanilla overflow, pinned rather than
//! repaired -- `known-divergences-are-pinned-not-deleted`.
//!
//! # `accountBytes(bytesPerEntry, count)` MULTIPLIES BEFORE IT CHECKS
//!
//! ```java
//! public void accountBytes(final long bytesPerEntry, final long count) {
//!     this.accountBytes(bytesPerEntry * count);
//! }
//! ```
//!
//! The product is a plain `long` multiply, so `accountBytes(4, 0x4000000000000000)` is
//! `4 * 2^62 == 2^64 == 0`, and a 4-exabyte array accounts for **nothing**. That is the
//! `ok-wrap` at the end of the row above. [`i64::wrapping_mul`].
//!
//! Note the asymmetry this creates with the negative check: `accountBytes(3, -2)` reaches
//! the single-argument form as `-6` and throws `IllegalArgumentException`, but a product
//! that wraps to a *positive* number is accepted. The guard sees only the product.
//!
//! # `MAX_STACK_DEPTH` is dead
//!
//! `private static final int MAX_STACK_DEPTH = 512` is never read: every factory passes a
//! literal `512` to the constructor instead. Kept as a private constant so the file matches
//! the Java field list, and marked `#[allow(dead_code)]` rather than deleted, because
//! deleting it would hide that vanilla has the same redundancy.

use crate::net::minecraft::nbt::NbtAccounterException;

/// Port of `NbtAccounter#DEFAULT_NBT_QUOTA`.
pub const DEFAULT_NBT_QUOTA: i32 = 2097152;

/// Port of `NbtAccounter#UNCOMPRESSED_NBT_QUOTA`.
pub const UNCOMPRESSED_NBT_QUOTA: i32 = 104857600;

/// Port of the private `NbtAccounter#MAX_STACK_DEPTH`.
///
/// Unread in vanilla too -- see the module docs.
#[allow(dead_code)]
const MAX_STACK_DEPTH: i32 = 512;

/// Port of `net.minecraft.nbt.NbtAccounter`.
#[derive(Clone, Debug)]
pub struct NbtAccounter {
    quota: i64,
    usage: i64,
    max_depth: i32,
    depth: i32,
}

impl NbtAccounter {
    /// Port of `NbtAccounter(long, int)`.
    ///
    /// `usage` and `depth` are Java field defaults (`0`), not constructor arguments.
    pub fn new(quota: i64, max_depth: i32) -> Self {
        Self { quota, usage: 0, max_depth, depth: 0 }
    }

    /// Port of `NbtAccounter#create(long)`.
    pub fn create(quota: i64) -> Self {
        Self::new(quota, 512)
    }

    /// Port of `NbtAccounter#defaultQuota()`.
    pub fn default_quota() -> Self {
        Self::new(2097152, 512)
    }

    /// Port of `NbtAccounter#uncompressedQuota()`.
    pub fn uncompressed_quota() -> Self {
        Self::new(104857600, 512)
    }

    /// Port of `NbtAccounter#unlimitedHeap()`.
    pub fn unlimited_heap() -> Self {
        Self::new(i64::MAX, 512)
    }

    /// Port of `NbtAccounter#accountBytes(long, long)`.
    ///
    /// Named for its first parameter because Java overloads on arity here, not on type, so
    /// `overload-naming`'s `<name>_<type>` rule has nothing to bite on.
    ///
    /// The product wraps -- see the module docs.
    pub fn account_bytes_per_entry(&mut self, bytes_per_entry: i64, count: i64) {
        self.account_bytes(bytes_per_entry.wrapping_mul(count));
    }

    /// Port of `NbtAccounter#accountBytes(long)`.
    ///
    /// # Panics
    ///
    /// `java.lang.IllegalArgumentException` for a negative size, and
    /// `net.minecraft.nbt.NbtAccounterException` when the quota is exceeded -- both with
    /// Java's exact message text.
    pub fn account_bytes(&mut self, size: i64) {
        if size < 0 {
            panic!("java.lang.IllegalArgumentException: Tried to account NBT tag with negative size: {size}");
        }

        // Wrapping, deliberately: see the module docs.
        if self.usage.wrapping_add(size) > self.quota {
            NbtAccounterException::throw(format!(
                "Tried to read NBT tag that was too big; tried to allocate: {} + {} bytes where max allowed: {}",
                self.usage, size, self.quota
            ));
        }

        self.usage = self.usage.wrapping_add(size);
    }

    /// Port of `NbtAccounter#pushDepth()`.
    ///
    /// # Panics
    ///
    /// `net.minecraft.nbt.NbtAccounterException` at the depth limit. Note the message
    /// quotes `maxDepth`, not the current depth.
    pub fn push_depth(&mut self) {
        if self.depth >= self.max_depth {
            NbtAccounterException::throw(format!(
                "Tried to read NBT tag with too high complexity, depth > {}",
                self.max_depth
            ));
        }

        self.depth += 1;
    }

    /// Port of `NbtAccounter#popDepth()`.
    ///
    /// # Panics
    ///
    /// `net.minecraft.nbt.NbtAccounterException` when already at the top level.
    pub fn pop_depth(&mut self) {
        if self.depth <= 0 {
            NbtAccounterException::throw("NBT-Accounter tried to pop stack-depth at top-level");
        }

        self.depth -= 1;
    }

    /// Port of `NbtAccounter#getUsage()` (`@VisibleForTesting`).
    pub fn get_usage(&self) -> i64 {
        self.usage
    }

    /// Port of `NbtAccounter#getDepth()` (`@VisibleForTesting`).
    pub fn get_depth(&self) -> i32 {
        self.depth
    }
}
