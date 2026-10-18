//! Port of: net/minecraft/client/renderer/MultiblockChestResources.java
//! Java class(es): net.minecraft.client.renderer.MultiblockChestResources
//! Status: VERIFIED
//!
//! Session 22. `public record MultiblockChestResources<T>(T single, T left, T right)`: one resource
//! per chest shape, a selector by [`ChestType`] and a `map`.
//!
//! * `select(type)` picks the component for `SINGLE`/`LEFT`/`RIGHT`. The Java switch's `default`
//!   arm throws `MatchException` and is unreachable for a non-null enum; a Rust `match` is exhaustive.
//! * `map(f)` builds a new record from `f(single)`, `f(left)`, `f(right)` -- evaluated IN THAT
//!   ORDER (constructor-argument order), which the probe pins with a logging function.
//!
//! # Record members over a generic `T`
//!
//! `equals` is component-wise (`T: PartialEq`), `toString` is the record form with each
//! component's `toString` (`T: Display`), and `hashCode` is the record fold over the components'
//! `hashCode`s. Rust has no universal Java `hashCode`, so [`MultiblockChestResources::java_hash_code_with`]
//! takes the component hash as a function; for `Integer` components it is the value.
//!
//! Verified by the `MULTICHEST`, `REC` and `RECEQ` groups of `small_probe.txt` (`tests/session22.rs`).

use crate::javacompat::java_object;
use crate::net::minecraft::world::level::block::state::properties::ChestType::ChestType;

/// Port of `net.minecraft.client.renderer.MultiblockChestResources<T>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MultiblockChestResources<T> {
    single: T,
    left: T,
    right: T,
}

impl<T> MultiblockChestResources<T> {
    /// Port of the canonical constructor.
    pub const fn new(single: T, left: T, right: T) -> Self {
        MultiblockChestResources {
            single,
            left,
            right,
        }
    }

    /// Port of the accessor `single()`.
    pub fn single(&self) -> &T {
        &self.single
    }

    /// Port of the accessor `left()`.
    pub fn left(&self) -> &T {
        &self.left
    }

    /// Port of the accessor `right()`.
    pub fn right(&self) -> &T {
        &self.right
    }

    /// Port of `select(ChestType chestType)`.
    pub fn select(&self, chest_type: ChestType) -> &T {
        match chest_type {
            ChestType::Single => &self.single,
            ChestType::Left => &self.left,
            ChestType::Right => &self.right,
        }
    }

    /// Port of `map(Function<T, S> mapper)`: `single`, then `left`, then `right`.
    pub fn map<S>(&self, mut mapper: impl FnMut(&T) -> S) -> MultiblockChestResources<S> {
        let single = mapper(&self.single);
        let left = mapper(&self.left);
        let right = mapper(&self.right);
        MultiblockChestResources::new(single, left, right)
    }

    /// Port of the record's generated `hashCode()`, given each component's Java `hashCode`.
    pub fn java_hash_code_with(&self, hash: impl Fn(&T) -> i32) -> i32 {
        java_object::record_hash(&[hash(&self.single), hash(&self.left), hash(&self.right)])
    }
}

impl<T: std::fmt::Display> std::fmt::Display for MultiblockChestResources<T> {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MultiblockChestResources[single={}, left={}, right={}]",
            self.single, self.left, self.right
        )
    }
}
