//! Port of: net.minecraft.world.phys.shapes.IdenticalMerger.java
//! Java class(es): net.minecraft.world.phys.shapes.IdenticalMerger
//! Status: VERIFIED

use crate::javacompat::fastutil::DoubleList;
use crate::net::minecraft::world::phys::shapes::IndexMerger::{
    null_consumer_merge, IndexConsumer, IndexMerger,
};

/// Iterates two identical coordinate lists as paired indices.
pub struct IdenticalMerger<'a> {
    coords: Option<&'a dyn DoubleList>,
}

impl<'a> IdenticalMerger<'a> {
    /// Java's constructor stores the reference without validating it.
    pub fn new(coords: Option<&'a dyn DoubleList>) -> Self {
        Self { coords }
    }

    fn coords(&self) -> &dyn DoubleList {
        self.coords.unwrap_or_else(|| {
            panic!("java.lang.NullPointerException: Cannot invoke \"it.unimi.dsi.fastutil.doubles.DoubleList.size()\" because \"this.coords\" is null")
        })
    }
}

impl IndexMerger for IdenticalMerger<'_> {
    fn get_list(&self) -> Option<&dyn DoubleList> {
        self.coords
    }

    fn for_merged_indexes(&self, mut consumer: Option<&mut dyn IndexConsumer>) -> bool {
        let last = self.coords().size().wrapping_sub(1);
        for index in 0..last {
            match consumer.as_deref_mut() {
                Some(c) => {
                    if !c.merge(index, index, index) {
                        return false;
                    }
                }
                None => null_consumer_merge(),
            }
        }
        true
    }

    fn size(&self) -> i32 {
        self.coords().size()
    }
}
