//! Port of: net/minecraft/core/HolderOwner.java
//! Java class(es): net.minecraft.core.HolderOwner
//! Status: VERIFIED
//!
//! Session 22. `public interface HolderOwner<T>`: the owner (a registry or registry lookup) that a
//! `Holder` belongs to. One default method.
//!
//! `canSerializeIn(context)` is `context == this` -- REFERENCE identity, so a holder serialises only
//! in its own owner, never in a different owner that happens to look the same. Here that is
//! `std::ptr::addr_eq` on the two references (the data address; a trait object's vtable is not
//! part of Java's identity).
//!
//! Verified by the `HOLDEROWNER` group of `small_probe.txt` (`tests/session22.rs`): an owner against
//! itself and against a second, distinct owner.

/// Port of `net.minecraft.core.HolderOwner<T>`.
pub trait HolderOwner<T> {
    /// Port of `default boolean canSerializeIn(HolderOwner<T> context)`: `context == this`.
    fn can_serialize_in(&self, context: &dyn HolderOwner<T>) -> bool {
        std::ptr::addr_eq(self as *const Self, context as *const dyn HolderOwner<T>)
    }
}
