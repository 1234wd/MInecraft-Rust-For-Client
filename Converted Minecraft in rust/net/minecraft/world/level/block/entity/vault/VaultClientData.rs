//! Port of: net/minecraft/world/level/block/entity/vault/VaultClientData.java
//! Java class(es): net.minecraft.world.level.block.entity.vault.VaultClientData
//! Status: VERIFIED
//!
//! Client-side spin state of the item displayed inside a vault block. Every member is ported;
//! `ROTATION_SPEED` is checked against the constant dump and the spin sequence against a run of
//! the 26.2 jar (`_porting/tests/session19_e.rs`).
//!
//! `updateDisplayItemSpin` adds a literal `10.0F` (javac inlined `ROTATION_SPEED`) and wraps
//! with `Mth.wrapDegrees(float)`, whose `>= 180` test sends 180 to **-180**: the jar sequence
//! is `..., 160, 170, -180, -170, ...`, never `+180`.

use crate::net::minecraft::util::Mth::Mth;

/// Java: `VaultClientData.ROTATION_SPEED` = `10.0F`.
pub const ROTATION_SPEED: f32 = 10.0;

/// Port of `VaultClientData`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VaultClientData {
    current_spin: f32,
    previous_spin: f32,
}

impl VaultClientData {
    /// Port of the implicit no-arg constructor: both spins `0.0F`.
    pub fn new() -> Self {
        VaultClientData { current_spin: 0.0, previous_spin: 0.0 }
    }

    /// Port of `VaultClientData#currentSpin()`.
    pub fn current_spin(&self) -> f32 {
        self.current_spin
    }

    /// Port of `VaultClientData#previousSpin()`.
    pub fn previous_spin(&self) -> f32 {
        self.previous_spin
    }

    /// Port of `VaultClientData#updateDisplayItemSpin()`.
    pub fn update_display_item_spin(&mut self) {
        self.previous_spin = self.current_spin;
        self.current_spin = Mth::wrap_degrees_f32(self.current_spin + ROTATION_SPEED);
    }
}
