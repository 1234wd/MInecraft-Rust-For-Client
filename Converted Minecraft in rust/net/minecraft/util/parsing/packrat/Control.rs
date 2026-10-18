//! Port of: net/minecraft/util/parsing/packrat/Control.java
//! Java class(es): net.minecraft.util.parsing.packrat.Control, Control::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 22. `public interface Control`: the packrat parser's "cut" flag -- `cut()` commits the
//! current alternative, `hasCut()` asks whether it was committed. One shared `UNBOUND` instance
//! ignores `cut()` and always answers `false`.
//!
//! `cut` takes `&mut self` because a real implementation sets a flag; `UNBOUND` is a stateless unit
//! struct, so it can be shared freely.
//!
//! Verified by the `PACKRATCTL` group of `small_probe.txt` (`tests/session22.rs`): `hasCut()` on
//! `UNBOUND` before and after `cut()`.

/// Port of `net.minecraft.util.parsing.packrat.Control`.
pub trait Control {
    /// Port of `void cut()`.
    fn cut(&mut self);

    /// Port of `boolean hasCut()`.
    fn has_cut(&self) -> bool;
}

/// The anonymous class behind `Control.UNBOUND`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Unbound;

impl Control for Unbound {
    /// Does nothing.
    fn cut(&mut self) {}

    /// Always `false`.
    fn has_cut(&self) -> bool {
        false
    }
}

/// Port of `Control.UNBOUND`.
pub const UNBOUND: Unbound = Unbound;
