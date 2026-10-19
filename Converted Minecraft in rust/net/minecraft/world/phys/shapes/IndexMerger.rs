//! Port of: net/minecraft/world/phys/shapes/IndexMerger.java
//! Java class(es): net.minecraft.world.phys.shapes.IndexMerger
//! Status: VERIFIED

use crate::javacompat::fastutil::DoubleList;

/// The read and merge contract shared by the voxel-shape index mergers.
pub trait IndexMerger {
    /// `getList()` may return Java `null`.
    fn get_list(&self) -> Option<&dyn DoubleList>;

    /// Calls the consumer for the merged indices and returns whether merging completed.
    /// Java permits a null callback; implementations panic with the corresponding NPE only
    /// if the callback is actually reached.
    fn for_merged_indexes(&self, consumer: Option<&mut dyn IndexConsumer>) -> bool;

    /// `size()`.
    fn size(&self) -> i32;
}

pub(crate) fn null_consumer_merge() -> ! {
    panic!("java.lang.NullPointerException: Cannot invoke \"net.minecraft.world.phys.shapes.IndexMerger$IndexConsumer.merge(int, int, int)\" because \"consumer\" is null");
}

/// Java's nested functional interface `IndexMerger.IndexConsumer`.
pub trait IndexConsumer {
    /// `merge(int firstIndex, int secondIndex, int resultIndex)`.
    fn merge(&mut self, first_index: i32, second_index: i32, result_index: i32) -> bool;
}

impl<F> IndexConsumer for F
where
    F: FnMut(i32, i32, i32) -> bool,
{
    fn merge(&mut self, first_index: i32, second_index: i32, result_index: i32) -> bool {
        self(first_index, second_index, result_index)
    }
}
