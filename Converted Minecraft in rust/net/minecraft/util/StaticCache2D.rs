//! Port of: net/minecraft/util/StaticCache2D.java
//! Java class(es): net.minecraft.util.StaticCache2D, net.minecraft.util.Initializer
//! Status: VERIFIED
//!
//! Session 22. `public class StaticCache2D<T>`: a square of values around a centre, computed once at
//! construction by an `Initializer` (`T get(int x, int z)`) and then looked up by coordinate.
//!
//! # Construction, in `int`
//!
//! `create(centerX, centerZ, range, init)` sets `min = center - range`, `size = 2 * range + 1`, and
//! allocates `sizeX * sizeZ` slots; then, x-major, z-minor, it calls `init(x, z)` for
//! `x in [minX, minX + sizeX)` and `z in [minZ, minZ + sizeZ)`. Every one of those is plain `int`
//! arithmetic that WRAPS, and the port keeps each:
//!
//! * a NEGATIVE range gives a negative size whose square is a positive slot count, so the cache has
//!   slots but the loops never run: every slot stays null, `forEach` hands the consumer nulls, and
//!   `contains` is always false (probed for `-1` and `-2`);
//! * a centre near `MAX_VALUE` makes `minX + sizeX` wrap negative, so again no slot is filled --
//!   but `contains` compares DIFFERENCES, so it is true for the square's coordinates, and `get`
//!   returns the null in the slot rather than throwing (probed);
//! * a range whose square overflows to a negative count throws
//!   `NegativeArraySizeException(<count>)` (probed: range `23170`, count `-2147479015`).
//!
//! A slot is therefore an `Option<T>` here, and both readers say so: `forEach` passes `None` and
//! `get` returns `None` where Java hands out null. Only those degenerate caches hold a null; a cache
//! built with a sane range and centre has every slot filled.
//!
//! # Lookup
//!
//! `get(x, z)` outside the square throws `IllegalArgumentException("Requested out of range value
//! (x,z) from " + this)`, and `toString()` is `String.format(Locale.ROOT,
//! "StaticCache2D[%d, %d, %d, %d]", minX, minZ, minX + sizeX, minZ + sizeZ)` -- the exclusive maxima,
//! wrapped. The port panics with that text.
//!
//! Verified by the `STATICCACHE` and `STATICCACHENASE` groups of `small_probe.txt`
//! (`tests/session22.rs`): six caches, the initializer's call order, `forEach` order, and `contains`
//! / `get` (or the exception text) over a 7 x 7 grid around each centre.

/// Port of `StaticCache2D.Initializer<T>`.
pub trait Initializer<T> {
    /// Port of `T get(int x, int z)`.
    fn get(&mut self, x: i32, z: i32) -> T;
}

impl<T, F: FnMut(i32, i32) -> T> Initializer<T> for F {
    fn get(&mut self, x: i32, z: i32) -> T {
        self(x, z)
    }
}

/// Port of `net.minecraft.util.StaticCache2D<T>`.
#[derive(Debug, Clone)]
pub struct StaticCache2D<T> {
    min_x: i32,
    min_z: i32,
    size_x: i32,
    size_z: i32,
    cache: Vec<Option<T>>,
}

impl<T> StaticCache2D<T> {
    /// Port of `static <T> StaticCache2D<T> create(int centerX, int centerZ, int range, Initializer<T> initializer)`.
    pub fn create(center_x: i32, center_z: i32, range: i32, initializer: impl Initializer<T>) -> StaticCache2D<T> {
        let min_x = center_x.wrapping_sub(range);
        let min_z = center_z.wrapping_sub(range);
        let size = 2i32.wrapping_mul(range).wrapping_add(1);
        StaticCache2D::new(min_x, min_z, size, size, initializer)
    }

    /// Port of the private constructor.
    fn new(min_x: i32, min_z: i32, size_x: i32, size_z: i32, mut initializer: impl Initializer<T>) -> StaticCache2D<T> {
        let len = size_x.wrapping_mul(size_z);
        if len < 0 {
            panic!("java.lang.NegativeArraySizeException: {len}");
        }
        let mut cache = StaticCache2D {
            min_x,
            min_z,
            size_x,
            size_z,
            cache: (0..len).map(|_| None).collect(),
        };
        let mut x = min_x;
        while x < min_x.wrapping_add(size_x) {
            let mut z = min_z;
            while z < min_z.wrapping_add(size_z) {
                let index = cache.get_index(x, z) as usize;
                cache.cache[index] = Some(initializer.get(x, z));
                z = z.wrapping_add(1);
            }
            x = x.wrapping_add(1);
        }
        cache
    }

    /// Port of `forEach(Consumer<T> consumer)`: every slot in index order; `None` is a Java null.
    pub fn for_each(&self, mut consumer: impl FnMut(Option<&T>)) {
        for o in &self.cache {
            consumer(o.as_ref());
        }
    }

    /// Port of `get(int x, int z)`. `None` is the Java null of a slot construction never filled.
    ///
    /// # Panics
    ///
    /// With Java's `IllegalArgumentException` text when `(x, z)` is outside the square.
    pub fn get(&self, x: i32, z: i32) -> Option<&T> {
        if !self.contains(x, z) {
            panic!("java.lang.IllegalArgumentException: Requested out of range value ({x},{z}) from {self}");
        }
        self.cache[self.get_index(x, z) as usize].as_ref()
    }

    /// Port of `contains(int x, int z)`.
    pub fn contains(&self, x: i32, z: i32) -> bool {
        let delta_x = x.wrapping_sub(self.min_x);
        let delta_z = z.wrapping_sub(self.min_z);
        delta_x >= 0 && delta_x < self.size_x && delta_z >= 0 && delta_z < self.size_z
    }

    /// Port of `private int getIndex(int x, int z)`.
    fn get_index(&self, x: i32, z: i32) -> i32 {
        let delta_x = x.wrapping_sub(self.min_x);
        let delta_z = z.wrapping_sub(self.min_z);
        delta_x.wrapping_mul(self.size_z).wrapping_add(delta_z)
    }
}

impl<T> std::fmt::Display for StaticCache2D<T> {
    /// Port of `toString()`: `StaticCache2D[minX, minZ, minX + sizeX, minZ + sizeZ]`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StaticCache2D[{}, {}, {}, {}]",
            self.min_x,
            self.min_z,
            self.min_x.wrapping_add(self.size_x),
            self.min_z.wrapping_add(self.size_z)
        )
    }
}
