//! Port of: net/minecraft/world/inventory/SimpleContainerData.java
//! Java class(es): net.minecraft.world.inventory.SimpleContainerData
//! Status: VERIFIED
//!
//! Session 24. `public class SimpleContainerData implements ContainerData`: a fixed-size `int[]`
//! behind the three [`ContainerData`] methods.
//!
//! Java's array checks are the behaviour, and the port reproduces their texts:
//!
//! * `new int[count]` with a negative `count` throws
//!   `java.lang.NegativeArraySizeException: <count>`;
//! * `get`/`set` outside `0..count` throw
//!   `java.lang.ArrayIndexOutOfBoundsException: Index <i> out of bounds for length <n>`.
//!
//! Verified by the `SCDATA` group of `sweep_probe.txt` (`tests/session24.rs`), which runs a
//! script of gets and sets, out-of-range indices included, over several sizes.

use crate::net::minecraft::world::inventory::ContainerData::ContainerData;

/// Port of `net.minecraft.world.inventory.SimpleContainerData`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleContainerData {
    ints: Vec<i32>,
}

impl SimpleContainerData {
    /// Port of `public SimpleContainerData(int count)`: `this.ints = new int[count]`.
    pub fn new(count: i32) -> Self {
        if count < 0 {
            panic!("java.lang.NegativeArraySizeException: {count}");
        }
        SimpleContainerData {
            ints: vec![0; count as usize],
        }
    }

    /// Java's array bounds check for `ints[index]`.
    fn slot(&self, index: i32) -> usize {
        if index < 0 || index as usize >= self.ints.len() {
            panic!(
                "java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {}",
                self.ints.len()
            );
        }
        index as usize
    }
}

impl ContainerData for SimpleContainerData {
    /// Port of `public int get(int dataId)`.
    fn get(&self, data_id: i32) -> i32 {
        self.ints[self.slot(data_id)]
    }

    /// Port of `public void set(int dataId, int value)`.
    fn set(&mut self, data_id: i32, value: i32) {
        let i = self.slot(data_id);
        self.ints[i] = value;
    }

    /// Port of `public int getCount()`: the array length.
    fn get_count(&self) -> i32 {
        self.ints.len() as i32
    }
}
