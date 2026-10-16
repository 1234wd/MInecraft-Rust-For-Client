//! Port of: net/minecraft/util/CrudeIncrementalIntIdentityHashBiMap.java
//! Java class(es): net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap
//! Status: VERIFIED
//!
//! Open-addressing (linear probing) identity bi-map behind `HashMapPalette` and the entity-data
//! serializer registry. Probing, growth (`size >= 0.8 * capacity` in `float`), the rehash order
//! during `grow`, `nextId` bookkeeping and `clear()` (which does NOT reset `values`) are ported
//! line for line and checked against the jar by replaying a jar run slot for slot.
//!
//! # The one unavoidable divergence: the hash source
//!
//! Java hashes keys with `System.identityHashCode`, which is per-JVM-run state and cannot be
//! reproduced. The port takes the identity hash from a caller-supplied `fn(&K) -> i32` given to
//! [`CrudeIncrementalIntIdentityHashBiMap::create`]; everything downstream of that number
//! (`murmurHash3Mixer`, `& 0x7FFFFFFF`, `% length`, probing) is exact. Likewise Java compares keys
//! with `==` (identity); the port compares with `K: PartialEq`. Given the same hash numbers and
//! distinct keys, slot layout, ids and iteration order are identical to the jar's (tested by
//! feeding in the identity hashes a jar run printed).
//!
//! # Vanilla quirks kept
//!
//! * `getId(null)` hashes to slot 0 (`identityHashCode(null) == 0`) and `indexOf` matches the
//!   first EMPTY slot (`null == null`), so it returns that slot's value -- `0` on a fresh map,
//!   a stale value after `clear()` -- not `-1`. Exposed as [`get_id_nullable`](CrudeIncrementalIntIdentityHashBiMap::get_id_nullable).
//! * `addMapping(k, id)` grows to the first power-of-two multiple `>= id`, so `id == newSize`
//!   still overflows `byId`: `java.lang.ArrayIndexOutOfBoundsException: Index 30 out of bounds for length 30`.
//! * `create(0)` builds a zero-length table and the first `add` divides by zero:
//!   `java.lang.ArithmeticException: / by zero`.
//! * `toString()` is `Object#toString` (identity hash); `Display` prints the class name and the
//!   object's address instead, both arbitrary per run as in Java.

use super::Mth::Mth;
use crate::net::minecraft::core::IdMap::IdMap;

/// Port of `CrudeIncrementalIntIdentityHashBiMap#NOT_FOUND` (private).
pub const NOT_FOUND: i32 = -1;

/// Port of `CrudeIncrementalIntIdentityHashBiMap#LOADFACTOR` (private).
pub const LOADFACTOR: f32 = 0.8;

/// Port of `net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap<K>`.
///
/// `EMPTY_SLOT` (Java `null`) is `None` in `keys` / `by_id`.
#[derive(Clone, Debug)]
pub struct CrudeIncrementalIntIdentityHashBiMap<K> {
    keys: Vec<Option<K>>,
    values: Vec<i32>,
    by_id: Vec<Option<K>>,
    next_id: i32,
    size: i32,
    /// Stand-in for `System.identityHashCode`; see the module docs.
    identity_hash: fn(&K) -> i32,
}

impl<K: Clone + PartialEq> CrudeIncrementalIntIdentityHashBiMap<K> {
    /// Port of the private `CrudeIncrementalIntIdentityHashBiMap(int capacity)`.
    fn with_capacity(capacity: i32, identity_hash: fn(&K) -> i32) -> Self {
        if capacity < 0 {
            panic!("java.lang.NegativeArraySizeException: {capacity}");
        }
        let n = capacity as usize;
        CrudeIncrementalIntIdentityHashBiMap {
            keys: vec![None; n],
            values: vec![0; n],
            by_id: vec![None; n],
            next_id: 0,
            size: 0,
            identity_hash,
        }
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap.create(int initialCapacity)`: capacity is
    /// `(int)(initialCapacity / 0.8F)`. `identity_hash` replaces `System.identityHashCode`.
    pub fn create(initial_capacity: i32, identity_hash: fn(&K) -> i32) -> Self {
        Self::with_capacity((initial_capacity as f32 / 0.8f32) as i32, identity_hash)
    }

    /// Port of `getId(@Nullable K)` including the `null` case (see the module docs).
    pub fn get_id_nullable(&self, thing: Option<&K>) -> i32 {
        self.get_value(self.index_of(thing, self.hash(thing)))
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#getValue(int)` (private).
    fn get_value(&self, index: i32) -> i32 {
        if index == -1 {
            -1
        } else {
            self.values[index as usize]
        }
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#contains(K)`.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get_id(key) != -1
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#contains(int)`.
    pub fn contains(&self, id: i32) -> bool {
        self.by_id(id).is_some()
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#add(K)`: maps `key` at the next free id
    /// and returns that id.
    pub fn add(&mut self, key: K) -> i32 {
        let value = self.next_id();
        self.add_mapping(key, value);
        value
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#nextId()` (private).
    fn next_id(&mut self) -> i32 {
        while (self.next_id as i64) < self.by_id.len() as i64 && self.by_id[self.next_id as usize].is_some() {
            self.next_id += 1;
        }
        self.next_id
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#grow(int)` (private): re-adds every key in
    /// SLOT order into a fresh map, then adopts its arrays, `nextId` and `size`.
    fn grow(&mut self, new_size: i32) {
        let mut resized = Self::with_capacity(new_size, self.identity_hash);

        for (i, k) in self.keys.iter().enumerate() {
            if let Some(k) = k {
                resized.add_mapping(k.clone(), self.values[i]);
            }
        }

        self.keys = resized.keys;
        self.values = resized.values;
        self.by_id = resized.by_id;
        self.next_id = resized.next_id;
        self.size = resized.size;
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#addMapping(K, int)`.
    pub fn add_mapping(&mut self, key: K, id: i32) {
        let min_size = std::cmp::max(id, self.size.wrapping_add(1));
        if min_size as f32 >= self.keys.len() as f32 * 0.8f32 {
            let mut new_size = (self.keys.len() as i32).wrapping_shl(1);

            while new_size < id {
                new_size = new_size.wrapping_shl(1);
            }

            self.grow(new_size);
        }

        let index = self.find_empty(self.hash(Some(&key)));
        self.keys[index as usize] = Some(key.clone());
        self.values[index as usize] = id;
        if id < 0 || id as usize >= self.by_id.len() {
            panic!(
                "java.lang.ArrayIndexOutOfBoundsException: Index {} out of bounds for length {}",
                id,
                self.by_id.len()
            );
        }
        self.by_id[id as usize] = Some(key);
        self.size += 1;
        if id == self.next_id {
            self.next_id += 1;
        }
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#hash(K)` (private):
    /// `(murmurHash3Mixer(identityHashCode(key)) & 0x7FFFFFFF) % keys.length`.
    fn hash(&self, key: Option<&K>) -> i32 {
        let identity = match key {
            Some(k) => (self.identity_hash)(k),
            None => 0,
        };
        let len = self.keys.len() as i32;
        if len == 0 {
            panic!("java.lang.ArithmeticException: / by zero");
        }
        (Mth::murmur_hash3_mixer(identity) & 0x7FFF_FFFF) % len
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#indexOf(K, int)` (private).
    fn index_of(&self, key: Option<&K>, start_from: i32) -> i32 {
        let len = self.keys.len() as i32;
        for i in start_from..len {
            if self.keys[i as usize].as_ref() == key {
                return i;
            }
            if self.keys[i as usize].is_none() {
                return -1;
            }
        }

        for i in 0..start_from {
            if self.keys[i as usize].as_ref() == key {
                return i;
            }
            if self.keys[i as usize].is_none() {
                return -1;
            }
        }

        -1
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#findEmpty(int)` (private).
    fn find_empty(&self, start_from: i32) -> i32 {
        let len = self.keys.len() as i32;
        for i in start_from..len {
            if self.keys[i as usize].is_none() {
                return i;
            }
        }

        for i in 0..start_from {
            if self.keys[i as usize].is_none() {
                return i;
            }
        }

        panic!("java.lang.RuntimeException: Overflowed :(");
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#clear()`. `values` is left as is.
    pub fn clear(&mut self) {
        self.keys.iter_mut().for_each(|k| *k = None);
        self.by_id.iter_mut().for_each(|k| *k = None);
        self.next_id = 0;
        self.size = 0;
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#copy()`: clones all three arrays.
    pub fn copy(&self) -> Self {
        self.clone()
    }

    /// Test/diagnostic view of the slot table (Java's private `keys`).
    pub fn raw_keys(&self) -> &[Option<K>] {
        &self.keys
    }

    /// Test/diagnostic view of the slot values (Java's private `values`).
    pub fn raw_values(&self) -> &[i32] {
        &self.values
    }
}

impl<K: Clone + PartialEq> IdMap<K> for CrudeIncrementalIntIdentityHashBiMap<K> {
    /// Port of `CrudeIncrementalIntIdentityHashBiMap#getId(K)`.
    fn get_id(&self, thing: &K) -> i32 {
        self.get_id_nullable(Some(thing))
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#byId(int)`.
    fn by_id(&self, id: i32) -> Option<&K> {
        if id >= 0 && (id as usize) < self.by_id.len() {
            self.by_id[id as usize].as_ref()
        } else {
            None
        }
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#size()`.
    fn size(&self) -> i32 {
        self.size
    }

    /// Port of `CrudeIncrementalIntIdentityHashBiMap#iterator()`: non-null `byId` slots in id order.
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a K> + 'a> {
        Box::new(self.by_id.iter().filter_map(|k| k.as_ref()))
    }
}

impl<K> std::fmt::Display for CrudeIncrementalIntIdentityHashBiMap<K> {
    /// `Object#toString()`; see the module docs for the identity-hash stand-in.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap@{:x}",
            (self as *const Self as usize) as u32
        )
    }
}
