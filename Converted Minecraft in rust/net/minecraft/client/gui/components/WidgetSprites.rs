//! Port of: net/minecraft/client/gui/components/WidgetSprites.java
//! Java class(es): net.minecraft.client.gui.components.WidgetSprites
//! Status: VERIFIED
//!
//! Session 22. `public record WidgetSprites(Identifier enabled, Identifier disabled,
//! Identifier enabledFocused, Identifier disabledFocused)`: three convenience constructors and one
//! lookup.
//!
//! # The constructors do not fill the slots the obvious way
//!
//! | Java constructor | enabled | disabled | enabledFocused | disabledFocused |
//! |---|---|---|---|---|
//! | `(sprite)` | sprite | sprite | sprite | sprite |
//! | `(sprite, focused)` | sprite | sprite | focused | focused |
//! | `(enabled, disabled, focused)` | enabled | disabled | focused | **disabled** |
//!
//! The three-argument form reuses `disabled` for the disabled-and-focused slot, NOT `focused`.
//! Those are [`WidgetSprites::from_sprite`], [`WidgetSprites::from_sprite_and_focused`] and
//! [`WidgetSprites::from_enabled_disabled_focused`].
//!
//! `get(enabled, focused)` picks the slot: `enabled ? (focused ? enabledFocused : enabled)
//! : (focused ? disabledFocused : disabled)`.
//!
//! # Record members
//!
//! `equals` is derived (four `Identifier#equals`), `hashCode` is the record fold over the four
//! `Identifier#hashCode`s, `toString` is the record form.
//!
//! Verified by the `REC`, `RECEQ` and `WIDGET` groups of `small_probe.txt` (`tests/session22.rs`):
//! all four constructors, and `get` for all four flag combinations on each.

use crate::javacompat::java_object;
use crate::net::minecraft::resources::Identifier::Identifier;

/// Port of `net.minecraft.client.gui.components.WidgetSprites`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WidgetSprites {
    enabled: Identifier,
    disabled: Identifier,
    enabled_focused: Identifier,
    disabled_focused: Identifier,
}

impl WidgetSprites {
    /// Port of the canonical constructor.
    pub fn new(
        enabled: Identifier,
        disabled: Identifier,
        enabled_focused: Identifier,
        disabled_focused: Identifier,
    ) -> Self {
        WidgetSprites {
            enabled,
            disabled,
            enabled_focused,
            disabled_focused,
        }
    }

    /// Port of `WidgetSprites(Identifier sprite)`: the same sprite in all four slots.
    pub fn from_sprite(sprite: &Identifier) -> Self {
        WidgetSprites::new(
            sprite.clone(),
            sprite.clone(),
            sprite.clone(),
            sprite.clone(),
        )
    }

    /// Port of `WidgetSprites(Identifier sprite, Identifier focused)`.
    pub fn from_sprite_and_focused(sprite: &Identifier, focused: &Identifier) -> Self {
        WidgetSprites::new(
            sprite.clone(),
            sprite.clone(),
            focused.clone(),
            focused.clone(),
        )
    }

    /// Port of `WidgetSprites(Identifier enabled, Identifier disabled, Identifier focused)`:
    /// note `disabledFocused` is `disabled`, not `focused`.
    pub fn from_enabled_disabled_focused(
        enabled: &Identifier,
        disabled: &Identifier,
        focused: &Identifier,
    ) -> Self {
        WidgetSprites::new(
            enabled.clone(),
            disabled.clone(),
            focused.clone(),
            disabled.clone(),
        )
    }

    /// Port of the accessor `enabled()`.
    pub fn enabled(&self) -> &Identifier {
        &self.enabled
    }

    /// Port of the accessor `disabled()`.
    pub fn disabled(&self) -> &Identifier {
        &self.disabled
    }

    /// Port of the accessor `enabledFocused()`.
    pub fn enabled_focused(&self) -> &Identifier {
        &self.enabled_focused
    }

    /// Port of the accessor `disabledFocused()`.
    pub fn disabled_focused(&self) -> &Identifier {
        &self.disabled_focused
    }

    /// Port of `get(boolean enabled, boolean focused)`.
    pub fn get(&self, enabled: bool, focused: bool) -> &Identifier {
        if enabled {
            if focused {
                &self.enabled_focused
            } else {
                &self.enabled
            }
        } else if focused {
            &self.disabled_focused
        } else {
            &self.disabled
        }
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            self.enabled.java_hash_code(),
            self.disabled.java_hash_code(),
            self.enabled_focused.java_hash_code(),
            self.disabled_focused.java_hash_code(),
        ])
    }
}

impl std::fmt::Display for WidgetSprites {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WidgetSprites[enabled={}, disabled={}, enabledFocused={}, disabledFocused={}]",
            self.enabled, self.disabled, self.enabled_focused, self.disabled_focused
        )
    }
}
