//! Port of: net/minecraft/nbt/NbtFormatException.java
//! Java class(es): net.minecraft.nbt.NbtFormatException
//! Status: PORTED
//!
//! Session 21. `extends NbtException`, one message constructor, no body.
//!
//! Nothing in the ported batch throws it yet -- in vanilla it comes from `NbtUtils` and the
//! SNBT layer, both still SKELETON. It is ported now rather than later because it costs one
//! constant and because leaving it SKELETON while its two siblings are PORTED would make the
//! manifest imply a gap in `NbtAccounter` that does not exist.
//!
//! See [`NbtException`](crate::net::minecraft::nbt::NbtException) for why the Java subclass
//! relationship is not reproduced as a Rust type relationship.

/// Java's `NbtFormatException.class.getName()`.
pub const CLASS_NAME: &str = "net.minecraft.nbt.NbtFormatException";

/// Port of `new NbtFormatException(String)` followed by a `throw`.
#[cold]
#[inline(never)]
pub fn throw(message: impl AsRef<str>) -> ! {
    panic!("{}: {}", CLASS_NAME, message.as_ref())
}
