//! Port of: net/minecraft/world/phys/shapes/BooleanOp.java
//! Java class(es): net.minecraft.world.phys.shapes.BooleanOp
//! Status: VERIFIED
//!
//! Session 22. `public interface BooleanOp`: a binary boolean operator for combining voxel shapes,
//! one method (`apply`) and all sixteen two-input truth tables as lambda constants.
//!
//! # Port shape
//!
//! The interface is a trait, and any `Fn(bool, bool) -> bool` closure is a `BooleanOp`. The sixteen
//! constants are the variants of [`Constant`], each its own truth table; they are distinct objects
//! in Java (callers compare them with `==`), and distinct variants here. The names keep Java's
//! declaration order, which is NOT truth-table order (`NOT_OR` is second, `TRUE` last).
//!
//! Verified by the `BOOLOP` group of `small_probe.txt` (`tests/session22.rs`), which lists the
//! interface's `BooleanOp` fields by reflection with all four inputs each -- so a renamed, added or
//! reordered constant fails the test.

/// Port of `net.minecraft.world.phys.shapes.BooleanOp`.
pub trait BooleanOp {
    /// Port of `boolean apply(boolean first, boolean second)`.
    fn apply(&self, first: bool, second: bool) -> bool;
}

impl<F: Fn(bool, bool) -> bool> BooleanOp for F {
    fn apply(&self, first: bool, second: bool) -> bool {
        self(first, second)
    }
}

/// The sixteen `BooleanOp` constants, in Java declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Constant {
    False,
    NotOr,
    OnlySecond,
    NotFirst,
    OnlyFirst,
    NotSecond,
    NotSame,
    NotAnd,
    And,
    Same,
    Second,
    Causes,
    First,
    CausedBy,
    Or,
    True,
}

impl BooleanOp for Constant {
    fn apply(&self, first: bool, second: bool) -> bool {
        match self {
            Constant::False => false,
            Constant::NotOr => !first && !second,
            Constant::OnlySecond => second && !first,
            Constant::NotFirst => !first,
            Constant::OnlyFirst => first && !second,
            Constant::NotSecond => !second,
            Constant::NotSame => first != second,
            Constant::NotAnd => !first || !second,
            Constant::And => first && second,
            Constant::Same => first == second,
            Constant::Second => second,
            Constant::Causes => !first || second,
            Constant::First => first,
            Constant::CausedBy => first || !second,
            Constant::Or => first || second,
            Constant::True => true,
        }
    }
}

/// Port of `BooleanOp.FALSE`.
pub const FALSE: Constant = Constant::False;
/// Port of `BooleanOp.NOT_OR`.
pub const NOT_OR: Constant = Constant::NotOr;
/// Port of `BooleanOp.ONLY_SECOND`.
pub const ONLY_SECOND: Constant = Constant::OnlySecond;
/// Port of `BooleanOp.NOT_FIRST`.
pub const NOT_FIRST: Constant = Constant::NotFirst;
/// Port of `BooleanOp.ONLY_FIRST`.
pub const ONLY_FIRST: Constant = Constant::OnlyFirst;
/// Port of `BooleanOp.NOT_SECOND`.
pub const NOT_SECOND: Constant = Constant::NotSecond;
/// Port of `BooleanOp.NOT_SAME`.
pub const NOT_SAME: Constant = Constant::NotSame;
/// Port of `BooleanOp.NOT_AND`.
pub const NOT_AND: Constant = Constant::NotAnd;
/// Port of `BooleanOp.AND`.
pub const AND: Constant = Constant::And;
/// Port of `BooleanOp.SAME`.
pub const SAME: Constant = Constant::Same;
/// Port of `BooleanOp.SECOND`.
pub const SECOND: Constant = Constant::Second;
/// Port of `BooleanOp.CAUSES`.
pub const CAUSES: Constant = Constant::Causes;
/// Port of `BooleanOp.FIRST`.
pub const FIRST: Constant = Constant::First;
/// Port of `BooleanOp.CAUSED_BY`.
pub const CAUSED_BY: Constant = Constant::CausedBy;
/// Port of `BooleanOp.OR`.
pub const OR: Constant = Constant::Or;
/// Port of `BooleanOp.TRUE`.
pub const TRUE: Constant = Constant::True;
