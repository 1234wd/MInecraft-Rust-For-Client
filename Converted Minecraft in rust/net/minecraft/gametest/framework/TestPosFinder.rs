//! Port of: net/minecraft/gametest/framework/TestPosFinder.java
//! Java class(es): net.minecraft.gametest.framework.TestPosFinder
//! Status: VERIFIED
//!
//! Session 22. `@FunctionalInterface public interface TestPosFinder` -- declaration only.
//!
//! The `Stream<BlockPos>` becomes a boxed iterator, consumed once as a Java stream is.
//!
//! It declares no behaviour of its own -- no default, static or constant -- so the port is the
//! trait's method set and signatures. Those are verified against the jar by the `API`/`APISUPER`
//! groups of `small_probe.txt` (`tests/session22.rs`): the probe lists the interface's declared
//! methods by reflection (name, generic parameter and return types, abstract/default/static), the
//! test compares them with the Java signature of every method below, and implements the trait with
//! exactly those methods so that a method missing from or added to this file fails to compile.

use crate::net::minecraft::core::BlockPos::BlockPos;

/// Port of `net.minecraft.gametest.framework.TestPosFinder`.
pub trait TestPosFinder {
    /// Port of `Stream<BlockPos> findTestPos()`.
    fn find_test_pos(&self) -> Box<dyn Iterator<Item = BlockPos> + '_>;
}
