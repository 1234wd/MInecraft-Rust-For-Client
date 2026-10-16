//! Port of: net/minecraft/util/BitStorage.java
//! Java class(es): net.minecraft.util.BitStorage
//! Status: PORTED
//!
//! The interface behind chunk palettes' packed index arrays. Implemented by
//! [`SimpleBitStorage`](super::SimpleBitStorage::SimpleBitStorage) and
//! [`ZeroBitStorage`](super::ZeroBitStorage::ZeroBitStorage).
//!
//! Mapping notes:
//! * `getRaw()` returns the live backing array in Java; here it is a borrowed slice. Mutable
//!   access is an inherent `get_raw_mut` on the implementors.
//! * `getAll(IntConsumer)` takes `&mut dyn FnMut(i32)`; `unpack(int[])` takes `&mut [i32]`.
//! * `copy()` returns a boxed trait object. `ZeroBitStorage.copy()` returns `this` in Java;
//!   the Rust copy is a new box with the same (immutable) size, which no caller can tell apart.

/// Port of `net.minecraft.util.BitStorage`.
pub trait BitStorage {
    /// Port of `BitStorage#getAndSet(int, int)`: stores `value`, returns the previous value.
    fn get_and_set(&mut self, index: i32, value: i32) -> i32;

    /// Port of `BitStorage#set(int, int)`.
    fn set(&mut self, index: i32, value: i32);

    /// Port of `BitStorage#get(int)`.
    fn get(&self, index: i32) -> i32;

    /// Port of `BitStorage#getRaw()`.
    fn get_raw(&self) -> &[i64];

    /// Port of `BitStorage#getSize()`.
    fn get_size(&self) -> i32;

    /// Port of `BitStorage#getBits()`.
    fn get_bits(&self) -> i32;

    /// Port of `BitStorage#getAll(IntConsumer)`: every value in index order.
    fn get_all(&self, output: &mut dyn FnMut(i32));

    /// Port of `BitStorage#unpack(int[])`: writes the first `getSize()` values into `output`.
    fn unpack(&self, output: &mut [i32]);

    /// Port of `BitStorage#copy()`.
    fn copy(&self) -> Box<dyn BitStorage>;
}
