//! Port of: net/minecraft/world/level/chunk/PaletteResize.java
//! Java class(es): net.minecraft.world.level.chunk.PaletteResize
//! Status: VERIFIED
//!
//! Session 22. `public interface PaletteResize<T>`: the callback a chunk-section palette calls when
//! a new value no longer fits its bit width, returning the new value's id. One static factory.
//!
//! `noResizeExpected()` is for palettes that can never grow: it throws
//! `IllegalArgumentException("Unexpected palette resize, bits = " + bits + ", added value = " +
//! lastAddedValue)`. The value is string-concatenated, i.e. its `toString` -- so the port needs
//! `T: Display`, and panics with that exact text (`DESIGN_DECISIONS.md#runtime-exceptions`). Java
//! prints a null value as `null`; a Rust value cannot be null, so a caller with an optional value
//! displays it as Java would.
//!
//! Verified by the `PALRESIZE` group of `small_probe.txt` (`tests/session22.rs`): an `Integer`, a
//! `String`, a null and a `Double`, with four bit counts.

/// Port of `net.minecraft.world.level.chunk.PaletteResize<T>`.
pub trait PaletteResize<T> {
    /// Port of `int onResize(int bits, T lastAddedValue)`.
    fn on_resize(&mut self, bits: i32, last_added_value: T) -> i32;
}

/// The callback `PaletteResize.noResizeExpected()` returns: every resize is an error.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoResizeExpected;

impl<T: std::fmt::Display> PaletteResize<T> for NoResizeExpected {
    /// Throws `IllegalArgumentException`, whatever the arguments.
    fn on_resize(&mut self, bits: i32, last_added_value: T) -> i32 {
        panic!("java.lang.IllegalArgumentException: Unexpected palette resize, bits = {bits}, added value = {last_added_value}")
    }
}

/// Port of `static <T> PaletteResize<T> noResizeExpected()`.
pub const fn no_resize_expected() -> NoResizeExpected {
    NoResizeExpected
}
