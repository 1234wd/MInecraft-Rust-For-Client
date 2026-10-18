//! Port of: net/minecraft/util/SingleKeyCache.java
//! Java class(es): net.minecraft.util.SingleKeyCache
//! Status: VERIFIED
//!
//! Session 22. `public class SingleKeyCache<K, V>`: memoises a function for the most recent key
//! only.
//!
//! ```java
//! if (this.cachedValue == null || !Objects.equals(this.cacheKey, cacheKey)) {
//!     this.cachedValue = this.computeValue.apply(cacheKey);
//!     this.cacheKey = cacheKey;
//! }
//! return this.cachedValue;
//! ```
//!
//! The key comparison is `Objects.equals` -- by VALUE, so an equal-but-distinct key is a hit
//! (probed with `new String("a")`). Only one key is remembered: going back to an older key
//! recomputes.
//!
//! # Null
//!
//! In Java a function that returns null leaves `cachedValue` null, so the NEXT lookup recomputes
//! even for the same key; and a null key compares equal to a null cached key. Rust keys and values
//! cannot be null, so neither case exists here.
//!
//! Verified by the `SINGLEKEY` group of `small_probe.txt` (`tests/session22.rs`): an eight-lookup
//! script, the value and the running compute count after each.

/// Port of `net.minecraft.util.SingleKeyCache<K, V>`.
pub struct SingleKeyCache<K, V> {
    compute_value: Box<dyn Fn(&K) -> V>,
    cache_key: Option<K>,
    cached_value: Option<V>,
}

impl<K: PartialEq + Clone, V> SingleKeyCache<K, V> {
    /// Port of `new SingleKeyCache(Function<K, V> computeValue)`: nothing cached yet.
    pub fn new(compute_value: impl Fn(&K) -> V + 'static) -> Self {
        SingleKeyCache {
            compute_value: Box::new(compute_value),
            cache_key: None,
            cached_value: None,
        }
    }

    /// Port of `getValue(K cacheKey)`.
    pub fn get_value(&mut self, cache_key: &K) -> &V {
        let hit = self.cached_value.is_some() && self.cache_key.as_ref() == Some(cache_key);
        if !hit {
            self.cached_value = Some((self.compute_value)(cache_key));
            self.cache_key = Some(cache_key.clone());
        }
        self.cached_value.as_ref().expect("just computed")
    }
}
