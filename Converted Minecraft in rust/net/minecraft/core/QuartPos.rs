//! Port of: net/minecraft/core/QuartPos.java
//! Java class(es): net.minecraft.core.QuartPos
//! Status: VERIFIED
//!
//! A **quart** is one quarter of a block: levelgen works at 1/4-block resolution so that
//! placement can land on quarter offsets without floating point.
//!
//! # EVERY METHOD IS A SHIFT, AND THE SHIFT DIRECTION IS THE WHOLE CLASS
//!
//! | direction | operation | constant |
//! |---|---|---|
//! | block -> quart | right, arithmetic | `>> 2` |
//! | quart -> block | left | `<< 2` |
//! | quart -> section | right, arithmetic | `>> 2` |
//! | section -> quart | left | `<< 2` |
//!
//! # `fromSection` IS *NOT* "SECTIONS ARE 64 QUARTS"
//!
//! Despite the name, `SECTION_TO_QUARTS_BITS` is **2**, so `fromSection(3)` is `3 << 2 == 12`,
//! not 192. Reading the method name and concluding that a section is 64 quarts gives a `<< 6`
//! here, and nothing in the corpus of small values would notice -- the mistake only shows up in
//! worldgen. The constant is 2 because `fromSection`/`toSection` move between a *section index*
//! and its *quart index*, i.e. they are the same "shift by 2" family as the block conversions,
//! and the name describes the axis rather than the scale factor.
//!
//! The two "narrowing" operations are **arithmetic** shifts, which is why the port uses Rust's
//! `>>` on `i32` directly rather than a masking form: Rust's signed `>>` is arithmetic, matching
//! Java's, so `-1 >> 2 == -1` and `-1 << 2 == -4` here. An unsigned shift would move coordinates
//! across the whole world for every negative input.
//!
//! `BITS`, `SIZE` and `MASK` are verified against the jar by `_porting/tests/consts.rs`.

/// Java: `QuartPos.BITS` = `2`.
pub const BITS: i32 = 2;

/// Java: `QuartPos.SIZE` = `4`.
pub const SIZE: i32 = 4;

/// Java: `QuartPos.MASK` = `3`.
pub const MASK: i32 = 3;

/// Java: `private static final int SECTION_TO_QUARTS_BITS` = `2`.
///
/// Despite the name this is **not** `log2(64)`. See the module docs.
pub const SECTION_TO_QUARTS_BITS: i32 = 2;

/// Java: `QuartPos` is `final` with a private constructor and is never instantiated, so this is
/// a namespace rather than a value.
pub struct QuartPos;

impl QuartPos {
    /// Port of `QuartPos#fromBlock(int)`.
    ///
    /// Arithmetic right shift by [`BITS`]. Floors towards negative infinity, so block `-1` maps
    /// to quart `-1`, not `0`.
    #[inline]
    pub fn from_block(block_coord: i32) -> i32 {
        block_coord >> BITS
    }

    /// Port of `QuartPos#quartLocal(int)`: the sub-block index within its block, `0..=3`.
    ///
    /// `& 3`, so the result is **never negative** -- unlike `fromBlock`, which is.
    #[inline]
    pub fn quart_local(block_coord: i32) -> i32 {
        block_coord & MASK
    }

    /// Port of `QuartPos#toBlock(int)`.
    ///
    /// A left shift, so it **wraps** at the top of the `int` range rather than saturating.
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // Java's int shift wraps.
    pub fn to_block(quart: i32) -> i32 {
        quart.wrapping_shl(BITS as u32)
    }

    /// Port of `QuartPos#fromSection(int)`.
    ///
    /// A section is 16 blocks = 64 quarts, and `16 << 2 == 64`, so this is a left shift by 2.
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // Java's int shift wraps.
    pub fn from_section(section: i32) -> i32 {
        section.wrapping_shl(SECTION_TO_QUARTS_BITS as u32)
    }

    /// Port of `QuartPos#toSection(int)`.
    ///
    /// Arithmetic right shift by [`SECTION_TO_QUARTS_BITS`].
    #[inline]
    pub fn to_section(quart: i32) -> i32 {
        quart >> SECTION_TO_QUARTS_BITS
    }
}