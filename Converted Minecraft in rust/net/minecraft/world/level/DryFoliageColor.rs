//! Port of: net/minecraft/world/level/DryFoliageColor.java
//! Java class(es): net.minecraft.world.level.DryFoliageColor
//! Status: VERIFIED
//!
//! The dry-foliage colormap (leaf litter etc.). Every member is ported; `FOLIAGE_DRY_DEFAULT`
//! is checked against the constant dump and the lookup against a run of the 26.2 jar
//! (`_porting/tests/session19_e.rs`).
//!
//! The initial table is 65536 **zeros** (so pre-`init` lookups return `0`); the default only
//! comes back for an index past the end of the table. `init` takes the vector by value (Java
//! keeps the caller's reference; vanilla never mutates it afterwards).

use crate::net::minecraft::world::level::ColorMapColorUtil;
use std::sync::{LazyLock, PoisonError, RwLock};

/// Java: `DryFoliageColor.FOLIAGE_DRY_DEFAULT` = `-10732494` (`0xFF5C3B32`).
pub const FOLIAGE_DRY_DEFAULT: i32 = -10732494;

/// Java: `private static int[] pixels = new int[65536];`.
static PIXELS: LazyLock<RwLock<Vec<i32>>> = LazyLock::new(|| RwLock::new(vec![0; 65536]));

/// Port of `DryFoliageColor#init(int[])`: replaces the colormap.
pub fn init(pixels: Vec<i32>) {
    *PIXELS.write().unwrap_or_else(PoisonError::into_inner) = pixels;
}

/// Port of `DryFoliageColor#get(double, double)`, falling back to `FOLIAGE_DRY_DEFAULT` (the
/// Java source passes the literal `-10732494`, the same value).
pub fn get(temp: f64, rain: f64) -> i32 {
    let pixels = PIXELS.read().unwrap_or_else(PoisonError::into_inner);
    ColorMapColorUtil::get(temp, rain, &pixels, FOLIAGE_DRY_DEFAULT)
}
