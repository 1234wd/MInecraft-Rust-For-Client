//! Port of: net/minecraft/commands/arguments/selector/options/InvertableSetOptionState.java
//! Java class(es): net.minecraft.commands.arguments.selector.options.InvertableSetOptionState
//! Status: VERIFIED
//!
//! Session 22. `public class InvertableSetOptionState`: the parse state of an entity-selector
//! option that takes either ONE positive value or any number of negated values and tags.
//! `EntitySelectorParser` keeps one each for `name=`, `gamemode=`, `team=` and `type=` (so
//! `type=pig`, or `type=!pig,type=!cow`).
//!
//! # The state machine
//!
//! A private `enum Limitation { NONE, SINGLE, MULTIPLE }`, starting at `NONE`, plus the set of tags
//! already parsed:
//!
//! * a positive element moves to `SINGLE`, after which NOTHING more may be parsed;
//! * a negative element or a tag moves to `MULTIPLE`, after which more negatives and tags may be
//!   parsed but no positive element;
//! * `canParseTag(tag)` additionally refuses a tag already parsed.
//!
//! `markParsed*` do not check the predicates -- they just move the state -- so marking a positive
//! element after a negative one is accepted and lands in `SINGLE` (probed).
//!
//! The tag set is only ever tested for membership, so a Rust `HashSet` is a faithful replacement
//! for the Java `HashSet` (no iteration order is observable).
//!
//! Verified by the `INVSET` group of `small_probe.txt` (`tests/session22.rs`): seven scripts,
//! all eight predicates after every step.

use std::collections::HashSet;

use crate::net::minecraft::resources::Identifier::Identifier;

/// Port of the private `InvertableSetOptionState.Limitation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Limitation {
    None,
    Single,
    Multiple,
}

/// Port of `net.minecraft.commands.arguments.selector.options.InvertableSetOptionState`.
#[derive(Debug, Clone)]
pub struct InvertableSetOptionState {
    state: Limitation,
    tags: HashSet<Identifier>,
}

impl Default for InvertableSetOptionState {
    fn default() -> Self {
        InvertableSetOptionState::new()
    }
}

impl InvertableSetOptionState {
    /// Port of the implicit constructor: `Limitation.NONE`, no tags.
    pub fn new() -> Self {
        InvertableSetOptionState {
            state: Limitation::None,
            tags: HashSet::new(),
        }
    }

    fn can_limit_to_single(&self) -> bool {
        self.state == Limitation::None
    }

    fn can_limit_to_multiple(&self) -> bool {
        self.state != Limitation::Single
    }

    /// Port of `canParsePositiveElement()`.
    pub fn can_parse_positive_element(&self) -> bool {
        self.can_limit_to_single()
    }

    /// Port of `canParseNegativeElement()`.
    pub fn can_parse_negative_element(&self) -> bool {
        self.can_limit_to_multiple()
    }

    /// Port of `canParseAnyTag()`.
    pub fn can_parse_any_tag(&self) -> bool {
        self.can_limit_to_multiple()
    }

    /// Port of `canParseTag(Identifier tag)`: any tag allowed, and not this one already.
    pub fn can_parse_tag(&self, tag: &Identifier) -> bool {
        self.can_parse_any_tag() && !self.tags.contains(tag)
    }

    /// Port of `canParseAny()`.
    pub fn can_parse_any(&self) -> bool {
        self.state != Limitation::Single
    }

    /// Port of `canParseElement(boolean inverted)`.
    pub fn can_parse_element(&self, inverted: bool) -> bool {
        if inverted {
            self.can_parse_negative_element()
        } else {
            self.can_parse_positive_element()
        }
    }

    fn mark_parsed_single(&mut self) {
        self.state = Limitation::Single;
    }

    fn mark_parsed_multiple(&mut self) {
        self.state = Limitation::Multiple;
    }

    /// Port of `markParsedPositiveElement()`.
    pub fn mark_parsed_positive_element(&mut self) {
        self.mark_parsed_single();
    }

    /// Port of `markParsedNegativeElement()`.
    pub fn mark_parsed_negative_element(&mut self) {
        self.mark_parsed_multiple();
    }

    /// Port of `markParsedTag(Identifier tag)`.
    pub fn mark_parsed_tag(&mut self, tag: Identifier) {
        self.mark_parsed_multiple();
        self.tags.insert(tag);
    }

    /// Port of `markParsedElement(boolean inverted)`.
    pub fn mark_parsed_element(&mut self, inverted: bool) {
        if inverted {
            self.mark_parsed_negative_element();
        } else {
            self.mark_parsed_positive_element();
        }
    }
}
