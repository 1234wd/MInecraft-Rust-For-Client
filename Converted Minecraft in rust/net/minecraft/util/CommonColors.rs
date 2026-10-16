//! Port of: net/minecraft/util/CommonColors.java
//! Java class(es): net.minecraft.util.CommonColors
//! Status: VERIFIED
//!
//! Packed ARGB colours used by the GUI. A pure constant holder; all sixteen fields are checked
//! against the jar's constant dump by `_porting/tests/session19_e.rs`.
//!
//! The Java source spells them as signed decimal `int`s (`BLACK = -16777216`), which is the
//! same 32 bits as the `0xAARRGGBB` form. The Rust constants keep the decimal spelling so they
//! diff against the decompiled source by eye; every one has alpha `0xFF`, hence negative.

/// Java: `CommonColors.WHITE` = `-1` (`0xFFFFFFFF`).
pub const WHITE: i32 = -1;
/// Java: `CommonColors.BLACK` = `-16777216` (`0xFF000000`).
pub const BLACK: i32 = -16777216;
/// Java: `CommonColors.GRAY` = `-8355712` (`0xFF808080`).
pub const GRAY: i32 = -8355712;
/// Java: `CommonColors.DARK_GRAY` = `-12566464` (`0xFF404040`).
pub const DARK_GRAY: i32 = -12566464;
/// Java: `CommonColors.LIGHT_GRAY` = `-6250336` (`0xFFA0A0A0`).
pub const LIGHT_GRAY: i32 = -6250336;
/// Java: `CommonColors.LIGHTER_GRAY` = `-4539718` (`0xFFBABABA`).
pub const LIGHTER_GRAY: i32 = -4539718;
/// Java: `CommonColors.RED` = `-65536` (`0xFFFF0000`).
pub const RED: i32 = -65536;
/// Java: `CommonColors.SOFT_RED` = `-2142128` (`0xFFDF5050`).
pub const SOFT_RED: i32 = -2142128;
/// Java: `CommonColors.GREEN` = `-16711936` (`0xFF00FF00`).
pub const GREEN: i32 = -16711936;
/// Java: `CommonColors.BLUE` = `-16776961` (`0xFF0000FF`).
pub const BLUE: i32 = -16776961;
/// Java: `CommonColors.YELLOW` = `-256` (`0xFFFFFF00`).
pub const YELLOW: i32 = -256;
/// Java: `CommonColors.SOFT_YELLOW` = `-171` (`0xFFFFFF55`).
pub const SOFT_YELLOW: i32 = -171;
/// Java: `CommonColors.DARK_PURPLE` = `-11534256` (`0xFF500050`).
pub const DARK_PURPLE: i32 = -11534256;
/// Java: `CommonColors.HIGH_CONTRAST_DIAMOND` = `-11010079` (`0xFF57FFE1`).
pub const HIGH_CONTRAST_DIAMOND: i32 = -11010079;
/// Java: `CommonColors.COSMOS_PINK` = `-13108` (`0xFFFFCCCC`).
pub const COSMOS_PINK: i32 = -13108;
/// Java: `CommonColors.TEXT_GRAY` = `-2039584` (`0xFFE0E0E0`).
pub const TEXT_GRAY: i32 = -2039584;
