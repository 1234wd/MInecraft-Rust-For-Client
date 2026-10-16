//! Port of: net/minecraft/core/IdMapper.java
//! Java class(es): net.minecraft.core.IdMapper
//! Status: PORTED
//!
//! A growable id <-> object table (`Block.BLOCK_STATE_REGISTRY`, `Fluid.FLUID_STATE_REGISTRY`).
//! Ids are assigned densely by `add`; `addMapping` may leave holes, which `byId` reports as
//! `None` and iteration skips. Iteration is in id order and yields an object once per id it is
//! mapped at, while `size()` counts distinct objects (`tToId.size()`): both quirks checked
//! against the jar.
//!
//! # Divergences, stated
//!
//! * Java keys `tToId` on IDENTITY (`Reference2IntOpenHashMap`). The port keys on `T: Eq + Hash`.
//!   For the real users (block/fluid states, which are interned singletons) the two coincide;
//!   a caller that needs identity for a type with structural equality must wrap it.
//! * `toString()` is `Object#toString`, i.e. `net.minecraft.core.IdMapper@<identity hash>`. The
//!   identity hash cannot be reproduced; `Display` prints the class name and the object's address
//!   in its place (both arbitrary per run, like Java's).

use std::collections::HashMap;
use std::hash::Hash;

use super::IdMap::IdMap;

/// Port of `net.minecraft.core.IdMapper<T>`.
#[derive(Clone, Debug)]
pub struct IdMapper<T> {
    next_id: i32,
    t_to_id: HashMap<T, i32>,
    id_to_t: Vec<Option<T>>,
}

impl<T: Eq + Hash + Clone> IdMapper<T> {
    /// Port of `IdMapper()`: expected size 512.
    pub fn new() -> IdMapper<T> {
        IdMapper::with_expected_size(512)
    }

    /// Port of `IdMapper(int expectedSize)`.
    pub fn with_expected_size(expected_size: i32) -> IdMapper<T> {
        if expected_size < 0 {
            // `Lists.newArrayListWithExpectedSize` runs first and rejects it.
            panic!("java.lang.IllegalArgumentException: arraySize cannot be negative but was: {expected_size}");
        }
        IdMapper {
            next_id: 0,
            t_to_id: HashMap::with_capacity(expected_size as usize),
            id_to_t: Vec::with_capacity(expected_size as usize),
        }
    }

    /// Port of `IdMapper#addMapping(T, int)`.
    ///
    /// The map entry is written before the list is touched, so a negative id records the mapping
    /// and then panics with `ArrayList.set`'s `IndexOutOfBoundsException`, as in Java.
    pub fn add_mapping(&mut self, thing: T, id: i32) {
        self.t_to_id.insert(thing.clone(), id);

        while (self.id_to_t.len() as i64) <= id as i64 {
            self.id_to_t.push(None);
        }

        if id < 0 {
            panic!(
                "java.lang.IndexOutOfBoundsException: Index {} out of bounds for length {}",
                id,
                self.id_to_t.len()
            );
        }
        self.id_to_t[id as usize] = Some(thing);
        if self.next_id <= id {
            self.next_id = id.wrapping_add(1);
        }
    }

    /// Port of `IdMapper#add(T)`: maps `thing` at the next free id.
    pub fn add(&mut self, thing: T) {
        let id = self.next_id;
        self.add_mapping(thing, id);
    }

    /// Port of `IdMapper#contains(int)`.
    pub fn contains(&self, id: i32) -> bool {
        self.by_id(id).is_some()
    }
}

impl<T: Eq + Hash + Clone> Default for IdMapper<T> {
    fn default() -> Self {
        IdMapper::new()
    }
}

impl<T: Eq + Hash + Clone> IdMap<T> for IdMapper<T> {
    /// Port of `IdMapper#getId(T)`: `-1` when absent (`defaultReturnValue(-1)`).
    fn get_id(&self, thing: &T) -> i32 {
        *self.t_to_id.get(thing).unwrap_or(&-1)
    }

    /// Port of `IdMapper#byId(int)`.
    fn by_id(&self, id: i32) -> Option<&T> {
        if id >= 0 && (id as usize) < self.id_to_t.len() {
            self.id_to_t[id as usize].as_ref()
        } else {
            None
        }
    }

    /// Port of `IdMapper#size()`: the number of distinct mapped objects.
    fn size(&self) -> i32 {
        self.t_to_id.len() as i32
    }

    /// Port of `IdMapper#iterator()`: non-null slots in id order.
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a T> + 'a> {
        Box::new(self.id_to_t.iter().filter_map(|t| t.as_ref()))
    }
}

impl<T> std::fmt::Display for IdMapper<T> {
    /// `Object#toString()`; see the module docs for the identity-hash stand-in.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "net.minecraft.core.IdMapper@{:x}", (self as *const Self as usize) as u32)
    }
}
