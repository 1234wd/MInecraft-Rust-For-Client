//! Port of: net/minecraft/core/NonNullList.java
//! Java class(es): net.minecraft.core.NonNullList
//! Status: PORTED
//!
//! A list that refuses `null` elements (inventories, `NonNullList<ItemStack>`). Rust values are
//! never null, so the `Objects.requireNonNull` checks have nothing left to reject; what remains
//! observable is the BACKING LIST each factory picks, and that is kept:
//!
//! | factory | Java backing | `add`/`remove` | bad index |
//! |---|---|---|---|
//! | `create`, `createWithCapacity` | `ArrayList` | allowed | `IndexOutOfBoundsException` |
//! | `withSize`, `of` | `Arrays.asList` (fixed size) | `UnsupportedOperationException` | `ArrayIndexOutOfBoundsException` |
//!
//! `clear()` refills with the default value when there is one (fixed-size lists never shrink),
//! otherwise empties the list. All messages were checked against the jar.
//!
//! `of(defaultValue, values...)` accepts a `null` default in Java; the port takes a value. Also
//! ported from the `AbstractList` superclass, because callers use them: `add(E)`, iteration,
//! `equals` (element-wise) and `toString` (`[a, b]`).

/// Which `java.util.List` the Java factory wrapped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Backing {
    /// `Lists.newArrayList()` / `newArrayListWithCapacity`.
    ArrayList,
    /// `Arrays.asList(...)`: fixed size, array-style index errors.
    FixedArray,
}

/// Port of `net.minecraft.core.NonNullList<E>`.
#[derive(Clone, Debug)]
pub struct NonNullList<E> {
    list: Vec<E>,
    backing: Backing,
    default_value: Option<E>,
}

impl<E: Clone> NonNullList<E> {
    /// Port of `NonNullList.create()`.
    pub fn create() -> NonNullList<E> {
        NonNullList { list: Vec::new(), backing: Backing::ArrayList, default_value: None }
    }

    /// Port of `NonNullList.createWithCapacity(int)`.
    pub fn create_with_capacity(capacity: i32) -> NonNullList<E> {
        if capacity < 0 {
            // Guava `Lists.newArrayListWithCapacity` -> `checkNonnegative`.
            panic!("java.lang.IllegalArgumentException: initialArraySize cannot be negative but was: {capacity}");
        }
        NonNullList { list: Vec::with_capacity(capacity as usize), backing: Backing::ArrayList, default_value: None }
    }

    /// Port of `NonNullList.withSize(int, E)`: fixed size, every slot `default_value`.
    pub fn with_size(size: i32, default_value: E) -> NonNullList<E> {
        if size < 0 {
            panic!("java.lang.NegativeArraySizeException: {size}");
        }
        NonNullList {
            list: vec![default_value.clone(); size as usize],
            backing: Backing::FixedArray,
            default_value: Some(default_value),
        }
    }

    /// Port of `NonNullList.of(E, E...)`: fixed size, holding `values`.
    pub fn of(default_value: E, values: Vec<E>) -> NonNullList<E> {
        NonNullList { list: values, backing: Backing::FixedArray, default_value: Some(default_value) }
    }

    fn check_access(&self, index: i32) {
        if index < 0 || index as usize >= self.list.len() {
            let class = match self.backing {
                Backing::ArrayList => "java.lang.IndexOutOfBoundsException",
                Backing::FixedArray => "java.lang.ArrayIndexOutOfBoundsException",
            };
            panic!("{class}: Index {index} out of bounds for length {}", self.list.len());
        }
    }

    fn check_resizable(&self) {
        if self.backing == Backing::FixedArray {
            panic!("java.lang.UnsupportedOperationException");
        }
    }

    /// Port of `NonNullList#get(int)`.
    pub fn get(&self, index: i32) -> &E {
        self.check_access(index);
        &self.list[index as usize]
    }

    /// Mutable access to the element `get(index)` returns (Java hands out the object itself).
    pub fn get_mut(&mut self, index: i32) -> &mut E {
        self.check_access(index);
        &mut self.list[index as usize]
    }

    /// Port of `NonNullList#set(int, E)`: returns the previous element.
    pub fn set(&mut self, index: i32, element: E) -> E {
        self.check_access(index);
        std::mem::replace(&mut self.list[index as usize], element)
    }

    /// Port of `NonNullList#add(int, E)`.
    pub fn add_at(&mut self, index: i32, element: E) {
        self.check_resizable();
        if index < 0 || index as usize > self.list.len() {
            panic!("java.lang.IndexOutOfBoundsException: Index: {index}, Size: {}", self.list.len());
        }
        self.list.insert(index as usize, element);
    }

    /// Port of the inherited `AbstractList#add(E)`: `add(size(), e)`, returns `true`.
    pub fn add(&mut self, element: E) -> bool {
        let size = self.size();
        self.add_at(size, element);
        true
    }

    /// Port of `NonNullList#remove(int)`: returns the removed element.
    pub fn remove(&mut self, index: i32) -> E {
        self.check_resizable();
        self.check_access(index);
        self.list.remove(index as usize)
    }

    /// Port of `NonNullList#size()`.
    pub fn size(&self) -> i32 {
        self.list.len() as i32
    }

    /// Port of `NonNullList#clear()`: refill with the default, or empty the list.
    pub fn clear(&mut self) {
        match self.default_value.clone() {
            None => {
                // `AbstractList#clear` removes element by element through `remove(int)`.
                while !self.list.is_empty() {
                    self.remove(0);
                }
            }
            Some(default_value) => {
                for i in 0..self.size() {
                    self.set(i, default_value.clone());
                }
            }
        }
    }

    /// Java `Collection#isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Java `Iterable#iterator()`, in index order.
    pub fn iter(&self) -> std::slice::Iter<'_, E> {
        self.list.iter()
    }

    /// Mutable iteration in index order.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, E> {
        self.list.iter_mut()
    }

    /// The elements as a slice.
    pub fn as_slice(&self) -> &[E] {
        &self.list
    }
}

impl<E: PartialEq> PartialEq for NonNullList<E> {
    /// `AbstractList#equals`: same elements in the same order (backing and default ignored).
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list
    }
}

impl<E: std::fmt::Display> std::fmt::Display for NonNullList<E> {
    /// `AbstractCollection#toString()`: `[a, b, c]`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (i, e) in self.list.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{e}")?;
        }
        f.write_str("]")
    }
}

impl<'a, E> IntoIterator for &'a NonNullList<E> {
    type Item = &'a E;
    type IntoIter = std::slice::Iter<'a, E>;
    fn into_iter(self) -> Self::IntoIter {
        self.list.iter()
    }
}
