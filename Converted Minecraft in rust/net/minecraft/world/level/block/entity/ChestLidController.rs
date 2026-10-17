//! Port of: net/minecraft/world/level/block/entity/ChestLidController.java
//! Java class(es): net.minecraft.world.level.block.entity.ChestLidController
//! Status: VERIFIED
//!
//! Session 22. `public class ChestLidController`: a chest lid's openness, moved 0.1 per tick toward
//! open or closed and interpolated between ticks.
//!
//! # `tickLid`
//!
//! ```java
//! this.oOpenness = this.openness;
//! if (!this.shouldBeOpen && this.openness > 0.0F) {
//!     this.openness = Math.max(this.openness - 0.1F, 0.0F);
//! } else if (this.shouldBeOpen && this.openness < 1.0F) {
//!     this.openness = Math.min(this.openness + 0.1F, 1.0F);
//! }
//! ```
//!
//! Each step is a `float` add of `0.1F`, so the intermediate values carry rounding -- nine steps
//! give `0.90000004` (`0x3f666668`), not `0.9` -- and the tenth rounds to exactly `1.0`. The probe
//! pins every intermediate bit pattern.
//!
//! `Math.max`/`Math.min` are Java's (`javacompat::java_lang`), whose `-0.0`/NaN rules differ from
//! Rust's `f32::max`/`min`. The local `speed = 0.1F` in the Java source is never read; the literal
//! `0.1F` is.
//!
//! `getOpenness(a)` is `Mth.lerp(a, oOpenness, openness)`.
//!
//! Verified by the `CHESTLID` group of `small_probe.txt` (`tests/session22.rs`): a 33-step
//! open/close script, openness at three partial ticks after every step.

use crate::javacompat::java_lang;
use crate::net::minecraft::util::Mth::Mth;

/// Port of `net.minecraft.world.level.block.entity.ChestLidController`.
#[derive(Debug, Clone, Copy, Default)]
pub struct ChestLidController {
    should_be_open: bool,
    openness: f32,
    o_openness: f32,
}

impl ChestLidController {
    /// Port of the implicit constructor: closed.
    pub const fn new() -> Self {
        ChestLidController {
            should_be_open: false,
            openness: 0.0,
            o_openness: 0.0,
        }
    }

    /// Port of `tickLid()`.
    pub fn tick_lid(&mut self) {
        self.o_openness = self.openness;
        if !self.should_be_open && self.openness > 0.0 {
            self.openness = java_lang::max_f32(self.openness - 0.1, 0.0);
        } else if self.should_be_open && self.openness < 1.0 {
            self.openness = java_lang::min_f32(self.openness + 0.1, 1.0);
        }
    }

    /// Port of `getOpenness(float a)`.
    pub fn get_openness(&self, a: f32) -> f32 {
        Mth::lerp_f32(a, self.o_openness, self.openness)
    }

    /// Port of `shouldBeOpen(boolean shouldBeOpen)`.
    pub fn should_be_open(&mut self, should_be_open: bool) {
        self.should_be_open = should_be_open;
    }
}
