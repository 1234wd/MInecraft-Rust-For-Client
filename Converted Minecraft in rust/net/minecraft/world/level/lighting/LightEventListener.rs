//! Port of: net/minecraft/world/level/lighting/LightEventListener.java
//! Java class(es): net.minecraft.world.level.lighting.LightEventListener
//! Status: VERIFIED
//!
//! Session 22. `public interface LightEventListener`: what the lighting engine is told about block
//! and section changes. Six abstract methods and one default.
//!
//! `updateSectionStatus` is overloaded: the abstract one takes a `SectionPos`, and the default one
//! takes a `BlockPos` and forwards `SectionPos.of(pos)` -- each block coordinate shifted right by 4.
//! Per `DESIGN_DECISIONS.md#overload-naming` they are [`LightEventListener::update_section_status_section_pos`]
//! and [`LightEventListener::update_section_status_block_pos`].
//!
//! Verified by the `LIGHTEVT` group of `small_probe.txt` (`tests/session22.rs`): the default called
//! on four positions (negative, section-boundary and `int`-extreme coordinates), recording the
//! `SectionPos` that reached the abstract method.

use crate::net::minecraft::core::BlockPos::BlockPos;
use crate::net::minecraft::core::SectionPos::SectionPos;
use crate::net::minecraft::world::level::ChunkPos::ChunkPos;

/// Port of `net.minecraft.world.level.lighting.LightEventListener`.
pub trait LightEventListener {
    /// Port of `void checkBlock(BlockPos pos)`.
    fn check_block(&mut self, pos: &BlockPos);

    /// Port of `boolean hasLightWork()`.
    fn has_light_work(&self) -> bool;

    /// Port of `int runLightUpdates()`.
    fn run_light_updates(&mut self) -> i32;

    /// Port of `default void updateSectionStatus(BlockPos pos, boolean sectionEmpty)`.
    fn update_section_status_block_pos(&mut self, pos: &BlockPos, section_empty: bool) {
        self.update_section_status_section_pos(&SectionPos::of_block_pos(pos), section_empty);
    }

    /// Port of `void updateSectionStatus(SectionPos pos, boolean sectionEmpty)`.
    fn update_section_status_section_pos(&mut self, pos: &SectionPos, section_empty: bool);

    /// Port of `void setLightEnabled(ChunkPos pos, boolean enable)`.
    fn set_light_enabled(&mut self, pos: &ChunkPos, enable: bool);

    /// Port of `void propagateLightSources(ChunkPos pos)`.
    fn propagate_light_sources(&mut self, pos: &ChunkPos);
}
