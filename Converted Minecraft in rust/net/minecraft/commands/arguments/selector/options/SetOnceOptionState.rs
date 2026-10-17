//! Port of: net/minecraft/commands/arguments/selector/options/SetOnceOptionState.java
//! Java class(es): net.minecraft.commands.arguments.selector.options.SetOnceOptionState
//! Status: VERIFIED
//!
//! Session 22. `public class SetOnceOptionState`: tracks whether an entity-selector option that may
//! appear only once (`limit=`, `sort=` ...) has been parsed. One `boolean`, starting `false`.
//!
//! `canParse()` is `!hasValue`; `markParsed()` sets it and is idempotent.
//!
//! Verified by the `SETONCE` group of `small_probe.txt` (`tests/session22.rs`): before, after one
//! mark, after a second mark.

/// Port of `net.minecraft.commands.arguments.selector.options.SetOnceOptionState`.
#[derive(Debug, Clone, Copy, Default)]
pub struct SetOnceOptionState {
    has_value: bool,
}

impl SetOnceOptionState {
    /// Port of the implicit constructor: not yet parsed.
    pub const fn new() -> Self {
        SetOnceOptionState { has_value: false }
    }

    /// Port of `canParse()`.
    pub const fn can_parse(&self) -> bool {
        !self.has_value
    }

    /// Port of `markParsed()`.
    pub fn mark_parsed(&mut self) {
        self.has_value = true;
    }
}
