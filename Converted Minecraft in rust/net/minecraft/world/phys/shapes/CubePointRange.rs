//! Port of: net/minecraft/world/phys/shapes/CubePointRange.java
//! Java class(es): net.minecraft.world.phys.shapes.CubePointRange
//! Status: VERIFIED

use crate::javacompat::fastutil::DoubleList;

/// A list of `parts + 1` coordinates evenly spaced from 0 to 1.
pub struct CubePointRange {
    parts: i32,
}

impl CubePointRange {
    /// Port of `CubePointRange(int)`.
    pub fn new(parts: i32) -> Self {
        if parts <= 0 {
            panic!("java.lang.IllegalArgumentException: Need at least 1 part");
        }
        Self { parts }
    }

    /// Port of `getDouble(int)`. The Java override does not bounds-check the index.
    pub fn get_double(&self, index: i32) -> f64 {
        f64::from(index) / f64::from(self.parts)
    }

    /// Port of `size()`, including Java `int` overflow for `Integer.MAX_VALUE` parts.
    pub fn size(&self) -> i32 {
        self.parts.wrapping_add(1)
    }
}

impl DoubleList for CubePointRange {
    fn get_double(&self, index: i32) -> f64 {
        CubePointRange::get_double(self, index)
    }

    fn size(&self) -> i32 {
        CubePointRange::size(self)
    }
}
