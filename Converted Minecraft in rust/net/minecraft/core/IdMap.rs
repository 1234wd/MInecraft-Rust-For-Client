//! Port of: net/minecraft/core/IdMap.java
//! Java class(es): net.minecraft.core.IdMap
//! Status: VERIFIED
//!
//! The int-id <-> object interface shared by registries, `IdMapper` and the palette bi-map.
//! `DEFAULT` is checked against the jar's constant dump.
//!
//! Mapping notes:
//! * `byId` returns `Option<&T>` (Java returns `@Nullable T`).
//! * `Iterable<T>` becomes [`IdMap::iter`], a boxed iterator over references.
//! * `getIdOrThrow`'s message embeds `value.toString()` and `this.toString()`, so that default
//!   method requires `T: Display` and `Self: Display`. `byIdOrThrow` needs neither.

use std::fmt::Display;

/// Port of `IdMap#DEFAULT`: the id `getId` returns for an unknown value.
pub const DEFAULT: i32 = -1;

/// Port of `net.minecraft.core.IdMap<T>`.
pub trait IdMap<T> {
    /// Port of `IdMap#getId(T)`; [`DEFAULT`] (`-1`) when absent.
    fn get_id(&self, thing: &T) -> i32;

    /// Port of `IdMap#byId(int)`.
    fn by_id(&self, id: i32) -> Option<&T>;

    /// Port of `IdMap#size()`.
    fn size(&self) -> i32;

    /// Port of `Iterable#iterator()`.
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a>;

    /// Port of `IdMap#byIdOrThrow(int)`.
    fn by_id_or_throw(&self, id: i32) -> &T {
        match self.by_id(id) {
            Some(result) => result,
            None => panic!("java.lang.IllegalArgumentException: No value with id {id}"),
        }
    }

    /// Port of `IdMap#getIdOrThrow(T)`.
    fn get_id_or_throw(&self, value: &T) -> i32
    where
        T: Display,
        Self: Display,
    {
        let id = self.get_id(value);
        if id == -1 {
            panic!("java.lang.IllegalArgumentException: Can't find id for '{value}' in map {self}");
        }
        id
    }
}
