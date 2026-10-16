//! Port of: net/minecraft/world/level/FoliageColor.java
//! Java class(es): net.minecraft.world.level.FoliageColor
//! Status: VERIFIED
//!
//! The foliage colormap plus the fixed tints for leaves that ignore biomes. Every member is
//! ported; the four constants are checked against the constant dump and the lookup against a
//! run of the 26.2 jar (`_porting/tests/session19_e.rs`).
//!
//! As with `GrassColor`, the initial table is 65536 **zeros**, so before `init` an in-range
//! lookup returns `0`, not `FOLIAGE_DEFAULT`; the default only comes back for an index past the
//! end of the table (e.g. after `init` with a short array). `init` takes the vector by value
//! (Java keeps the caller's reference; vanilla never mutates it afterwards).

use crate::net::minecraft::world::level::ColorMapColorUtil;
use std::sync::{LazyLock, PoisonError, RwLock};

/// Java: `FoliageColor.FOLIAGE_EVERGREEN` = `-10380959` (`0xFF619961`).
pub const FOLIAGE_EVERGREEN: i32 = -10380959;
/// Java: `FoliageColor.FOLIAGE_BIRCH` = `-8345771` (`0xFF80A755`).
pub const FOLIAGE_BIRCH: i32 = -8345771;
/// Java: `FoliageColor.FOLIAGE_DEFAULT` = `-12012264` (`0xFF48B518`).
pub const FOLIAGE_DEFAULT: i32 = -12012264;
/// Java: `FoliageColor.FOLIAGE_MANGROVE` = `-7158200` (`0xFF92C648`).
pub const FOLIAGE_MANGROVE: i32 = -7158200;

/// Java: `private static int[] pixels = new int[65536];`.
static PIXELS: LazyLock<RwLock<Vec<i32>>> = LazyLock::new(|| RwLock::new(vec![0; 65536]));

/// Port of `FoliageColor#init(int[])`: replaces the colormap.
pub fn init(pixels: Vec<i32>) {
    *PIXELS.write().unwrap_or_else(PoisonError::into_inner) = pixels;
}

/// Port of `FoliageColor#get(double, double)`, falling back to `FOLIAGE_DEFAULT`.
pub fn get(temp: f64, rain: f64) -> i32 {
    let pixels = PIXELS.read().unwrap_or_else(PoisonError::into_inner);
    ColorMapColorUtil::get(temp, rain, &pixels, FOLIAGE_DEFAULT)
}
