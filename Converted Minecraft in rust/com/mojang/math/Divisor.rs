//! Port of: com.mojang.math.Divisor.java
//! Java class(es): com.mojang.math.Divisor
//! Status: VERIFIED

use crate::javacompat::fastutil::IntIterator;

/// Distributes an integer numerator over a fixed number of pieces using Java division/remainder.
pub struct Divisor {
    denominator: i32,
    quotient: i32,
    mod_value: i32,
    returned_parts: i32,
    remainder: i32,
}

impl Divisor {
    /// Stores the Java quotient and remainder when the denominator is positive.
    pub fn new(numerator: i32, denominator: i32) -> Self {
        let (quotient, mod_value) = if denominator > 0 {
            (numerator / denominator, numerator % denominator)
        } else {
            (0, 0)
        };
        Self {
            denominator,
            quotient,
            mod_value,
            returned_parts: 0,
            remainder: 0,
        }
    }

    /// `Divisor.asIterable(numerator, denominator)`; every iterator starts with fresh state.
    pub fn as_iterable(numerator: i32, denominator: i32) -> DivisorIterable {
        DivisorIterable {
            numerator,
            denominator,
        }
    }
}

impl IntIterator for Divisor {
    fn has_next(&self) -> bool {
        self.returned_parts < self.denominator
    }

    fn next_int(&mut self) -> i32 {
        if !self.has_next() {
            panic!("java.util.NoSuchElementException");
        }

        let mut next = self.quotient;
        self.remainder = self.remainder.wrapping_add(self.mod_value);
        if self.remainder >= self.denominator {
            self.remainder = self.remainder.wrapping_sub(self.denominator);
            next = next.wrapping_add(1);
        }
        self.returned_parts = self.returned_parts.wrapping_add(1);
        next
    }
}

/// Rust representation of Java's `Iterable<Integer>` returned by `asIterable`.
#[derive(Debug, Clone, Copy)]
pub struct DivisorIterable {
    numerator: i32,
    denominator: i32,
}

impl DivisorIterable {
    /// Java's `iterator()`: returns an independent `Divisor` each time.
    pub fn iterator(&self) -> Divisor {
        Divisor::new(self.numerator, self.denominator)
    }
}
