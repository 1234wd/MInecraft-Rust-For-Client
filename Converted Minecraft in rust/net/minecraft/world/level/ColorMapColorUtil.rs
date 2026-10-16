//! Port of: net/minecraft/world/level/ColorMapColorUtil.java
//! Java class(es): net.minecraft.world.level.ColorMapColorUtil
//! Status: PORTED
//!
//! The lookup behind `GrassColor`, `FoliageColor` and `DryFoliageColor`: maps a biome's
//! temperature and downfall onto a 256x256 colormap texture (`pixels`, row-major, row = `y`).
//! The interface has one static method and no instances, so it is a free function here. There
//! are no constants, hence no dump group; the results below were cross-checked against a run of
//! the 26.2 jar (`_porting/tests/session19_e.rs`).
//!
//! # Out-of-range input crashes, exactly as in vanilla
//!
//! The only guard is `index >= pixels.length -> default`. Nothing clamps `temp`/`rain`, so:
//!
//! - `temp > 1` makes `x` negative; `y << 8 | x` with a negative `x` is a negative index and
//!   Java throws `ArrayIndexOutOfBoundsException: Index -127 out of bounds for length 65536`
//!   (for `temp = 1.5, rain = 0.5`). `rain * temp > 1` does the same through `y`.
//! - `temp < 0` (or `rain * temp < 0`) makes `x`/`y` exceed 255; the index then overflows the
//!   table and the *default colour* comes back instead.
//! - `NaN` casts to `0` (Java `d2i`, same as Rust `as i32`).
//!
//! The port reproduces the throw rather than clamping (DESIGN_DECISIONS.md
//! `#runtime-exceptions`); the bounds check happens on the Java `int` before any `usize` cast.

/// Port of `ColorMapColorUtil#get(double, double, int[], int)`.
pub fn get(temp: f64, rain: f64, pixels: &[i32], default_map_color: i32) -> i32 {
    let rain = rain * temp;
    // Java `(int)` of a double: truncates toward zero, saturates, NaN -> 0 (same as `as i32`).
    let x = ((1.0 - temp) * 255.0) as i32;
    let y = ((1.0 - rain) * 255.0) as i32;
    let index = y.wrapping_shl(8) | x;
    if index as i64 >= pixels.len() as i64 {
        return default_map_color;
    }
    if index < 0 {
        panic!(
            "java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {}",
            pixels.len()
        );
    }
    pixels[index as usize]
}
