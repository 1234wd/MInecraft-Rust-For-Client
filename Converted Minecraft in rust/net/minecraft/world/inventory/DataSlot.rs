//! Port of: net/minecraft/world/inventory/DataSlot.java
//! Java class(es): net.minecraft.world.inventory.DataSlot, net.minecraft.world.inventory.DataSlot::<3 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 24. `public abstract class DataSlot`: one synchronised `int` of a container menu, with
//! a remembered previous value for change detection. Abstract `get`/`set`, the concrete
//! `checkAndClearUpdateFlag`, and three static factories, each returning an anonymous subclass.
//!
//! # The three anonymous subclasses are the only ones
//!
//! No other `DataSlot` subclass exists in the game (every `new DataSlot()` is in this file), so the
//! port is a closed set: [`DataSlot`] holds the base class's `prevValue` and an [`Access`] for
//! the subclass.
//!
//! # Aliasing is the point of the class
//!
//! `forContainer(container, i)` and `shared(storage, i)` read and write THROUGH a reference the
//! caller keeps: `AbstractContainerMenu.addDataSlots` wraps the block entity's `ContainerData`,
//! and `EnchantmentMenu` wraps its own `costs`/`enchantClue` arrays and keeps writing to them. A
//! copy would silently disconnect the menu from what it displays (OPEN_QUESTIONS #34). The port
//! therefore takes shared handles -- `Rc<RefCell<dyn ContainerData>>` and `Rc<RefCell<Vec<i32>>>`
//! (a Java `int[]`: the vector is never resized here) -- so a write through either side is seen by
//! the other, as in Java.
//!
//! # Errors
//!
//! `shared` does not check `index` when it is created; `storage[index]` throws on each `get`/`set`
//! instead, with Java's `ArrayIndexOutOfBoundsException` text. `forContainer` leaves the check to
//! the container.
//!
//! Verified by the `DSLOT` group of `sweep_probe.txt` (`tests/session24.rs`): scripts of `get`,
//! `set`, `checkAndClearUpdateFlag` and writes behind the slot's back, for all three factories.

use std::cell::RefCell;
use std::rc::Rc;

use crate::net::minecraft::world::inventory::ContainerData::ContainerData;

/// Which anonymous subclass of `DataSlot` this is, with its captured state.
pub enum Access {
    /// `forContainer(container, dataId)`: `container.get(dataId)` / `container.set(dataId, v)`.
    ForContainer {
        container: Rc<RefCell<dyn ContainerData>>,
        data_id: i32,
    },
    /// `shared(storage, index)`: `storage[index]`.
    Shared {
        storage: Rc<RefCell<Vec<i32>>>,
        index: i32,
    },
    /// `standalone()`: a private `int value` of the anonymous class, initially 0.
    Standalone { value: i32 },
}

/// Port of `net.minecraft.world.inventory.DataSlot`.
pub struct DataSlot {
    prev_value: i32,
    access: Access,
}

/// Port of `public static DataSlot forContainer(ContainerData container, int dataId)`.
pub fn for_container(container: Rc<RefCell<dyn ContainerData>>, data_id: i32) -> DataSlot {
    DataSlot {
        prev_value: 0,
        access: Access::ForContainer { container, data_id },
    }
}

/// Port of `public static DataSlot shared(int[] storage, int index)`.
pub fn shared(storage: Rc<RefCell<Vec<i32>>>, index: i32) -> DataSlot {
    DataSlot {
        prev_value: 0,
        access: Access::Shared { storage, index },
    }
}

/// Port of `public static DataSlot standalone()`.
pub fn standalone() -> DataSlot {
    DataSlot {
        prev_value: 0,
        access: Access::Standalone { value: 0 },
    }
}

/// Java's bounds check for `storage[index]`.
fn array_slot(len: usize, index: i32) -> usize {
    if index < 0 || index as usize >= len {
        panic!("java.lang.ArrayIndexOutOfBoundsException: Index {index} out of bounds for length {len}");
    }
    index as usize
}

impl DataSlot {
    /// Port of `public abstract int get()`, as each anonymous subclass implements it.
    pub fn get(&self) -> i32 {
        match &self.access {
            Access::ForContainer { container, data_id } => container.borrow().get(*data_id),
            Access::Shared { storage, index } => {
                let storage = storage.borrow();
                storage[array_slot(storage.len(), *index)]
            }
            Access::Standalone { value } => *value,
        }
    }

    /// Port of `public abstract void set(int value)`, as each anonymous subclass implements it.
    pub fn set(&mut self, value: i32) {
        match &mut self.access {
            Access::ForContainer { container, data_id } => {
                container.borrow_mut().set(*data_id, value)
            }
            Access::Shared { storage, index } => {
                let mut storage = storage.borrow_mut();
                let i = array_slot(storage.len(), *index);
                storage[i] = value;
            }
            Access::Standalone { value: v } => *v = value,
        }
    }

    /// Port of `public boolean checkAndClearUpdateFlag()`: whether `get()` changed since the last
    /// call (the remembered value starts at 0), remembering the new value either way.
    pub fn check_and_clear_update_flag(&mut self) -> bool {
        let current_value = self.get();
        let result = current_value != self.prev_value;
        self.prev_value = current_value;
        result
    }

    /// The subclass and its captured state.
    pub fn access(&self) -> &Access {
        &self.access
    }
}

impl std::fmt::Debug for DataSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match &self.access {
            Access::ForContainer { data_id, .. } => format!("ForContainer({data_id})"),
            Access::Shared { index, .. } => format!("Shared({index})"),
            Access::Standalone { value } => format!("Standalone({value})"),
        };
        f.debug_struct("DataSlot")
            .field("prev_value", &self.prev_value)
            .field("access", &kind)
            .finish()
    }
}
