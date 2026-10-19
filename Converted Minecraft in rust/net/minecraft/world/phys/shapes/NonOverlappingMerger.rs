//! Port of: net/minecraft/world/phys/shapes/NonOverlappingMerger.java
//! Java class(es): net.minecraft.world.phys.shapes.NonOverlappingMerger
//! Status: VERIFIED

use crate::javacompat::fastutil::DoubleList;
use crate::net::minecraft::world::phys::shapes::IndexMerger::{
    null_consumer_merge, IndexConsumer, IndexMerger,
};

/// Merges two non-overlapping coordinate lists that share their boundary coordinate.
pub struct NonOverlappingMerger<'a> {
    lower: Option<&'a dyn DoubleList>,
    upper: Option<&'a dyn DoubleList>,
    swap: bool,
}

impl<'a> NonOverlappingMerger<'a> {
    /// Java's protected constructor stores references without validating them.
    pub fn new(
        lower: Option<&'a dyn DoubleList>,
        upper: Option<&'a dyn DoubleList>,
        swap: bool,
    ) -> Self {
        Self { lower, upper, swap }
    }

    fn lower(&self) -> &dyn DoubleList {
        self.lower.unwrap_or_else(|| {
            panic!("java.lang.NullPointerException: Cannot invoke \"it.unimi.dsi.fastutil.doubles.DoubleList.size()\" because \"this.lower\" is null")
        })
    }

    fn upper(&self) -> &dyn DoubleList {
        self.upper.unwrap_or_else(|| {
            panic!("java.lang.NullPointerException: Cannot invoke \"it.unimi.dsi.fastutil.doubles.DoubleList.size()\" because \"this.upper\" is null")
        })
    }

    fn for_non_swapped_indexes(
        &self,
        mut consumer: Option<&mut dyn IndexConsumer>,
        swap: bool,
    ) -> bool {
        let lower_size = self.lower().size();
        for index in 0..lower_size {
            let (first, second) = if swap { (-1, index) } else { (index, -1) };
            match consumer.as_deref_mut() {
                Some(c) => {
                    if !c.merge(first, second, index) {
                        return false;
                    }
                }
                None => null_consumer_merge(),
            }
        }

        let upper_size = self.upper().size().wrapping_sub(1);
        for index in 0..upper_size {
            let (first, second) = if swap {
                (index, lower_size.wrapping_sub(1))
            } else {
                (lower_size.wrapping_sub(1), index)
            };
            match consumer.as_deref_mut() {
                Some(c) => {
                    if !c.merge(first, second, lower_size.wrapping_add(index)) {
                        return false;
                    }
                }
                None => null_consumer_merge(),
            }
        }

        true
    }
}

impl DoubleList for NonOverlappingMerger<'_> {
    fn get_double(&self, index: i32) -> f64 {
        let lower_size = self.lower().size();
        if index < lower_size {
            self.lower().get_double(index)
        } else {
            self.upper.unwrap_or_else(|| {
                panic!("java.lang.NullPointerException: Cannot invoke \"it.unimi.dsi.fastutil.doubles.DoubleList.getDouble(int)\" because \"this.upper\" is null")
            }).get_double(index.wrapping_sub(lower_size))
        }
    }

    fn size(&self) -> i32 {
        self.lower().size().wrapping_add(self.upper().size())
    }
}

impl IndexMerger for NonOverlappingMerger<'_> {
    fn get_list(&self) -> Option<&dyn DoubleList> {
        Some(self)
    }

    fn for_merged_indexes(&self, consumer: Option<&mut dyn IndexConsumer>) -> bool {
        self.for_non_swapped_indexes(consumer, self.swap)
    }

    fn size(&self) -> i32 {
        DoubleList::size(self)
    }
}
