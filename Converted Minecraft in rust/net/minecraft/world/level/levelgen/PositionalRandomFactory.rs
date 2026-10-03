//! Port of: net/minecraft/world/level/levelgen/PositionalRandomFactory.java
//! Java class(es): net.minecraft.world.level.levelgen.PositionalRandomFactory
//! Status: VERIFIED

use crate::net::minecraft::util::RandomSource::RandomSource;

/// Port of `PositionalRandomFactory`.
///
/// "Same seed = same world": a factory turns a position (or a name) into an
/// independent `RandomSource`, so features can be generated in any order and still
/// agree.
pub trait PositionalRandomFactory {
    /// Port of `PositionalRandomFactory#fromHashOf(String)`.
    fn from_hash_of(&self, name: &str) -> Box<dyn RandomSource>;

    /// Port of `PositionalRandomFactory#fromSeed(long)`.
    fn from_seed(&self, seed: i64) -> Box<dyn RandomSource>;

    /// Port of `PositionalRandomFactory#at(int,int,int)`.
    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource>;

    /// Port of `PositionalRandomFactory#parityConfigString(StringBuilder)`.
    ///
    /// In Java this appends to a `StringBuilder`; that has no Rust analogue, so the
    /// two implementations RETURN the string. The content is identical, which is
    /// what `ConfigurableRandomProviderMode` compares in the tests, and the golden
    /// data pins it.
    fn parity_config_string(&self) -> String;
}