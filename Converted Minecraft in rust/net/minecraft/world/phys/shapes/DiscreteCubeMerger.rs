//! Port of: net.minecraft.world.phys.shapes.DiscreteCubeMerger.java
//! Java class(es): net.minecraft.world.phys.shapes.DiscreteCubeMerger
//! Status: VERIFIED

use crate::javacompat::fastutil::DoubleList;
use crate::javacompat::guava::int_math::gcd;
use crate::net::minecraft::world::phys::shapes::CubePointRange::CubePointRange;
use crate::net::minecraft::world::phys::shapes::IndexMerger::{
    null_consumer_merge, IndexConsumer, IndexMerger,
};

/// The same `Shapes.lcm(first, second)` expression used by the Java constructor.
fn shapes_lcm(first: i32, second: i32) -> i64 {
    let divisor = gcd(first, second);
    if divisor == 0 {
        panic!("java.lang.ArithmeticException: / by zero");
    }
    (first as i64).wrapping_mul((second / divisor) as i64)
}

/// Merges two discrete cube grids by mapping indices through their greatest common divisor.
pub struct DiscreteCubeMerger {
    result: CubePointRange,
    first_div: i32,
    second_div: i32,
}

impl DiscreteCubeMerger {
    /// The Java constructor is package-private and stores the lcm and reduced divisors.
    pub fn new(first_size: i32, second_size: i32) -> Self {
        let result = CubePointRange::new(shapes_lcm(first_size, second_size) as i32);
        let divisor = gcd(first_size, second_size);
        Self {
            result,
            first_div: first_size / divisor,
            second_div: second_size / divisor,
        }
    }
}

impl IndexMerger for DiscreteCubeMerger {
    fn get_list(&self) -> Option<&dyn DoubleList> {
        Some(&self.result)
    }

    fn for_merged_indexes(&self, mut consumer: Option<&mut dyn IndexConsumer>) -> bool {
        let last = self.result.size().wrapping_sub(1);
        for index in 0..last {
            let first_index = index / self.second_div;
            let second_index = index / self.first_div;
            match consumer.as_deref_mut() {
                Some(c) => {
                    if !c.merge(first_index, second_index, index) {
                        return false;
                    }
                }
                None => null_consumer_merge(),
            }
        }
        true
    }

    fn size(&self) -> i32 {
        self.result.size()
    }
}
