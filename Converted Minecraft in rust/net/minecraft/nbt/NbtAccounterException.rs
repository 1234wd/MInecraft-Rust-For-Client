//! Port of: net/minecraft/nbt/NbtAccounterException.java
//! Java class(es): net.minecraft.nbt.NbtAccounterException
//! Status: PORTED
//!
//! Session 21. `extends NbtException`, one message constructor, no body.
//!
//! Thrown by all three of [`NbtAccounter`](crate::net::minecraft::nbt::NbtAccounter)'s
//! guards: the quota check, `pushDepth` past the limit, and `popDepth` at depth 0. Its
//! name is pinned by the `ACC` rows of `_porting/test-data/nbt_probe.txt`, which record
//! `e.getClass().getName() + ":" + e.getMessage()` -- so this file existing separately
//! from [`NbtException`](crate::net::minecraft::nbt::NbtException) is what lets a test
//! tell the two apart.
//!
//! See [`NbtException`](crate::net::minecraft::nbt::NbtException) for why the Java
//! subclass relationship is not reproduced as a Rust type relationship.

/// Java's `NbtAccounterException.class.getName()`.
pub const CLASS_NAME: &str = "net.minecraft.nbt.NbtAccounterException";

/// Port of `new NbtAccounterException(String)` followed by a `throw`.
#[cold]
#[inline(never)]
pub fn throw(message: impl AsRef<str>) -> ! {
    panic!("{}: {}", CLASS_NAME, message.as_ref())
}
