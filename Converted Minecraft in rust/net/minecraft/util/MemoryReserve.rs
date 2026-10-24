//! Port of: net/minecraft/util/MemoryReserve.java
//! Java class(es): net.minecraft.util.MemoryReserve
//! Status: PORTED
//!
//! A static reserve buffer: `allocate()` pins a 10 MiB byte array, `release()` drops it
//! and hints to the GC. Imports only `org.jspecify.annotations.Nullable` (a marker
//! annotation with no runtime effect).
//!
//! `System.gc()` is a JVM hint that may or may not trigger collection; Rust has no
//! garbage collector. Dropping the `Vec` in `release()` immediately frees the memory,
//! which is the deterministic equivalent of nulling the reference and hinting the GC.
//! This is not a stub — the buffer is genuinely allocated and freed.

use std::sync::{LazyLock, Mutex};

static RESERVE: LazyLock<Mutex<Option<Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(None));

const RESERVE_SIZE: usize = 10_485_760;

/// Port of `public static void allocate()`: `reserve = new byte[10485760]`.
pub fn allocate() {
    *RESERVE.lock().expect("reserve lock poisoned") = Some(vec![0u8; RESERVE_SIZE]);
}

/// Port of `public static void release()`: nulls the reference and hints the GC.
///
/// In Rust, `take()` removes the `Vec` from the `Option`, dropping it and freeing
/// the memory immediately. `System.gc()` has no Rust equivalent and is unnecessary.
pub fn release() {
    if RESERVE.lock().expect("reserve lock poisoned").take().is_some() {
        // Memory freed by the drop above. No GC hint needed in Rust.
    }
}
