//! Port of: net/minecraft/world/level/block/entity/BeaconBeamOwner.java
//! Java class(es): net.minecraft.world.level.block.entity.BeaconBeamOwner, net.minecraft.world.level.block.entity.Section
//! Status: VERIFIED
//!
//! Session 22. `public interface BeaconBeamOwner` -- one method, `getBeamSections()` -- and its
//! nested `class Section`: one coloured run of a beacon beam, with a colour fixed at construction
//! and a height that starts at 1 and grows one block at a time.
//!
//! `Section.increaseHeight()` is `height++` on an `int`; the port wraps it the same way.
//!
//! Java returns a `List<Section>`; the trait returns a borrowed slice of the implementor's sections.
//!
//! Verified by the `BEAMSEC` group of `small_probe.txt` (`tests/session22.rs`).

/// Port of `net.minecraft.world.level.block.entity.BeaconBeamOwner`.
pub trait BeaconBeamOwner {
    /// Port of `BeaconBeamOwner#getBeamSections()`.
    fn get_beam_sections(&self) -> &[Section];
}

/// Port of `BeaconBeamOwner.Section`.
#[derive(Debug, Clone, Copy)]
pub struct Section {
    color: i32,
    height: i32,
}

impl Section {
    /// Port of `new BeaconBeamOwner.Section(int color)`: height 1.
    pub const fn new(color: i32) -> Self {
        Section { color, height: 1 }
    }

    /// Port of `increaseHeight()`: `height++`, wrapping.
    pub fn increase_height(&mut self) {
        self.height = self.height.wrapping_add(1);
    }

    /// Port of `getColor()`.
    pub const fn get_color(&self) -> i32 {
        self.color
    }

    /// Port of `getHeight()`.
    pub const fn get_height(&self) -> i32 {
        self.height
    }
}
