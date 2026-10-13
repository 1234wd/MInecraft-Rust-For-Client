//! Port of: net/minecraft/world/level/LightLayer.java
//! Java class(es): net.minecraft.world.level.LightLayer
//! Status: PORTED
//!
//! Tier 2, not yet oracle-verified.
//!
//! A two-constant enum indexing the two light-value arrays in `LevelChunk` (sky light and block
//! light), and serialized by ordinal in the light-engine packet. **The declaration order IS the
//! persisted order**, so `Sky = 0` and `Block = 1` are load-bearing.

/// Port of `net.minecraft.world.level.LightLayer`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LightLayer {
    /// Port of `LightLayer#SKY` -- ordinal 0.
    Sky = 0,
    /// Port of `LightLayer#BLOCK` -- ordinal 1.
    Block = 1,
}

/// Java: `LightLayer.values()`.
///
/// Declaration order, which is also the persisted ordinal order.
pub const VALUES: [LightLayer; 2] = [LightLayer::Sky, LightLayer::Block];