//! Port of: net/minecraft/nbt/NbtException.java
//! Java class(es): net.minecraft.nbt.NbtException
//! Status: PORTED
//!
//! Session 21. `public class NbtException extends RuntimeException` with a single
//! message constructor, and nothing else.
//!
//! # Why a class with no body still needs a file
//!
//! `DESIGN_DECISIONS.md#runtime-exceptions` says a port panics with Java's exception
//! **class name** and Java's exact message. So the only thing this class contributes to
//! the port is its name -- and that name is load-bearing: `NbtAccounter` throws the
//! subclass [`crate::net::minecraft::nbt::NbtAccounterException`], and the oracle rows in
//! `nbt_probe.txt` record `e.getClass().getName()`, so `net.minecraft.nbt.NbtException`
//! and `net.minecraft.nbt.NbtAccounterException` are distinguishable and a test will say so.
//!
//! The Java hierarchy (`NbtAccounterException` and `NbtFormatException` both extend this)
//! is NOT modelled as a Rust type hierarchy. Nothing in the game catches `NbtException`
//! and then re-dispatches on the subclass, so a hierarchy would be structure with no
//! observable behaviour behind it. If a `catch (NbtException)` site turns up in the I/O
//! layer, this is the comment to revisit -- it will need the `Result` treatment that
//! `panic-not-Result` reserves for catch-and-continue.

/// Java's `NbtException.class.getName()`.
pub const CLASS_NAME: &str = "net.minecraft.nbt.NbtException";

/// Port of `new NbtException(String)` followed by a `throw`.
///
/// Panics with `"<class name>: <message>"`, which is what `Throwable#toString` prints and
/// therefore what a crash log shows.
#[cold]
#[inline(never)]
pub fn throw(message: impl AsRef<str>) -> ! {
    panic!("{}: {}", CLASS_NAME, message.as_ref())
}
