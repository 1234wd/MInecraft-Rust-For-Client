//! Port of: net/minecraft/client/gui/screens/advancements/AdvancementWidgetType.java
//! Java class(es): net.minecraft.client.gui.screens.advancements.AdvancementWidgetType
//! Status: VERIFIED
//!
//! Session 19. Every member is ported. Names and ordinals are checked against the jar's constant
//! dump in `_porting/tests/session19_consts.rs`. All four sprite fields are checked against their
//! `toString()` in the dump.
//!
//! The sprites are `Identifier`s built once in the Java constructor; the port builds an equal
//! `Identifier` per call (`Identifier` has value equality, and nothing compares these by identity).

use crate::net::minecraft::advancements::AdvancementType::AdvancementType;
use crate::net::minecraft::resources::Identifier::{with_default_namespace, Identifier};

/// Port of `net.minecraft.client.gui.screens.advancements.AdvancementWidgetType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AdvancementWidgetType {
    /// `OBTAINED` -- ordinal 0.
    Obtained = 0,
    /// `UNOBTAINED` -- ordinal 1.
    Unobtained = 1,
}

/// Java: `AdvancementWidgetType.values()`, declaration order.
pub const VALUES: [AdvancementWidgetType; 2] = [
    AdvancementWidgetType::Obtained,
    AdvancementWidgetType::Unobtained,
];

impl AdvancementWidgetType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            AdvancementWidgetType::Obtained => "OBTAINED",
            AdvancementWidgetType::Unobtained => "UNOBTAINED",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    fn sprite_path(&self, which: &str) -> String {
        let state = match self {
            AdvancementWidgetType::Obtained => "obtained",
            AdvancementWidgetType::Unobtained => "unobtained",
        };
        format!("advancements/{which}_{state}")
    }

    /// Port of `AdvancementWidgetType#boxSprite()`.
    pub fn box_sprite(&self) -> Identifier {
        with_default_namespace(&self.sprite_path("box"))
    }

    /// Port of `AdvancementWidgetType#frameSprite(AdvancementType)`.
    pub fn frame_sprite(&self, kind: AdvancementType) -> Identifier {
        match kind {
            AdvancementType::Task => with_default_namespace(&self.sprite_path("task_frame")),
            AdvancementType::Challenge => with_default_namespace(&self.sprite_path("challenge_frame")),
            AdvancementType::Goal => with_default_namespace(&self.sprite_path("goal_frame")),
        }
    }
}

impl std::fmt::Display for AdvancementWidgetType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
