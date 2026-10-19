//! Port of: net/minecraft/world/phys/shapes/OffsetDoubleList.java
//! Java class(es): net.minecraft.world.phys.shapes.OffsetDoubleList
//! Status: VERIFIED

use crate::javacompat::fastutil::DoubleList;

/// A view that adds a fixed offset to each value in a fastutil `DoubleList`.
pub struct OffsetDoubleList<'a> {
    delegate: Option<&'a dyn DoubleList>,
    offset: f64,
}

impl<'a> OffsetDoubleList<'a> {
    /// Port of `OffsetDoubleList(DoubleList, double)`. `None` preserves Java's accepted null
    /// reference so the later dereference raises the same `NullPointerException`.
    pub fn new(delegate: Option<&'a dyn DoubleList>, offset: f64) -> Self {
        Self { delegate, offset }
    }

    /// Port of `getDouble(int)`.
    pub fn get_double(&self, index: i32) -> f64 {
        let delegate = self.delegate.unwrap_or_else(|| {
            panic!("java.lang.NullPointerException: Cannot invoke \"it.unimi.dsi.fastutil.doubles.DoubleList.getDouble(int)\" because \"this.delegate\" is null")
        });
        delegate.get_double(index) + self.offset
    }

    /// Port of `size()`.
    pub fn size(&self) -> i32 {
        self.delegate.unwrap_or_else(|| {
            panic!("java.lang.NullPointerException: Cannot invoke \"it.unimi.dsi.fastutil.doubles.DoubleList.size()\" because \"this.delegate\" is null")
        }).size()
    }
}

impl DoubleList for OffsetDoubleList<'_> {
    fn get_double(&self, index: i32) -> f64 {
        OffsetDoubleList::get_double(self, index)
    }

    fn size(&self) -> i32 {
        OffsetDoubleList::size(self)
    }
}
