//! Port of: net/minecraft/world/level/GrassColor.java
//! Java class(es): net.minecraft.world.level.GrassColor
//! Status: PORTED
//!
//! The grass colormap: a process-wide `int[]` installed by `init` (from `grass.png`, by the
//! client's reload listener -- texture loading is not part of this class) and read through
//! `ColorMapColorUtil#get` with the magenta fallback `-65281` (`0xFFFF00FF`). No constants, so
//! no dump group; behaviour cross-checked against a run of the 26.2 jar
//! (`_porting/tests/session19_e.rs`).
//!
//! Java starts with `new int[65536]` -- all **zeros**, not the fallback -- so before `init`
//! every in-range lookup returns `0` (transparent black), and `getDefaultColor()` is `0`.
//!
//! `private static int[] pixels` is a mutable global; here it is a lock-guarded `Vec`. `init`
//! takes the vector by value: Java keeps the caller's array reference, so a caller mutating the
//! array after `init` would be visible there and not here (vanilla never does).

use crate::net::minecraft::world::level::ColorMapColorUtil;
use std::sync::{LazyLock, PoisonError, RwLock};

/// Java: `private static int[] pixels = new int[65536];`.
static PIXELS: LazyLock<RwLock<Vec<i32>>> = LazyLock::new(|| RwLock::new(vec![0; 65536]));

/// Port of `GrassColor#init(int[])`: replaces the colormap.
pub fn init(pixels: Vec<i32>) {
    *PIXELS.write().unwrap_or_else(PoisonError::into_inner) = pixels;
}

/// Port of `GrassColor#get(double, double)`.
pub fn get(temp: f64, rain: f64) -> i32 {
    let pixels = PIXELS.read().unwrap_or_else(PoisonError::into_inner);
    ColorMapColorUtil::get(temp, rain, &pixels, -65281)
}

/// Port of `GrassColor#getDefaultColor()`: `get(0.5, 1.0)`, i.e. pixel `(127, 127)`.
pub fn get_default_color() -> i32 {
    get(0.5, 1.0)
}
