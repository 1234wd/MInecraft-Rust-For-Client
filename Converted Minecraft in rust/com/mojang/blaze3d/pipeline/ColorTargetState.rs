//! Port of: com/mojang/blaze3d/pipeline/ColorTargetState.java
//! Java class(es): com.mojang.blaze3d.pipeline.ColorTargetState, com.mojang.blaze3d.pipeline.ColorTargetState.WriteMask
//! Status: PARTIAL
//!
//! Session 24. `public record ColorTargetState(Optional<BlendFunction> blendFunction, GpuFormat
//! format, @WriteMask int writeMask)`: how a pipeline writes one colour attachment -- the optional
//! blending, the attachment's format, and which channels it writes.
//!
//! * the `WRITE_*` bits are red 1, green 2, blue 4, alpha 8; `WRITE_COLOR` is 7, `WRITE_ALL` 15,
//!   `WRITE_NONE` 0; [`DEFAULT`] is no blending, `RGBA8_UNORM`, all channels;
//! * `ColorTargetState(BlendFunction)` is `(Optional.of(blendFunction), RGBA8_UNORM, 15)`;
//! * `writeRed()`.. `writeAlpha()` test one bit each of the raw mask -- the canonical
//!   constructor checks nothing, so a mask with bits above 15, or a negative one, is kept as is;
//! * `@interface WriteMask` is a `@Retention(CLASS) @Target(TYPE_USE)` marker with no members:
//!   the type alias [`WriteMask`].
//!
//! `equals` and `toString` are the record's: `Optional.equals` (so [`BlendFunction`] equality),
//! the enum by identity, the int by value; `blendFunction=Optional[BlendFunction[...]]` or
//! `blendFunction=Optional.empty`. `hashCode` folds `BlendFunction`'s and the `GpuFormat`'s, both
//! built on enum identity hashes, so it is not ported (OPEN_QUESTIONS #30).
//!
//! `Optional<BlendFunction>` is `Option<BlendFunction>`. Java's canonical constructor would also
//! take a `null` Optional (printed `null`); no vanilla caller passes one.
//!
//! Verified by the `COLORTGT` group of `sweep_probe.txt` (`tests/session24.rs`).

use crate::com::mojang::blaze3d::pipeline::BlendFunction::BlendFunction;
use crate::com::mojang::blaze3d::GpuFormat::GpuFormat;

/// Port of the annotation type `ColorTargetState.WriteMask`: an `int` of `WRITE_*` bits.
pub type WriteMask = i32;

/// Port of `public static final int WRITE_RED = 1`.
pub const WRITE_RED: i32 = 1;
/// Port of `public static final int WRITE_GREEN = 2`.
pub const WRITE_GREEN: i32 = 2;
/// Port of `public static final int WRITE_BLUE = 4`.
pub const WRITE_BLUE: i32 = 4;
/// Port of `public static final int WRITE_ALPHA = 8`.
pub const WRITE_ALPHA: i32 = 8;
/// Port of `public static final @WriteMask int WRITE_COLOR = 7`.
pub const WRITE_COLOR: WriteMask = 7;
/// Port of `public static final @WriteMask int WRITE_ALL = 15`.
pub const WRITE_ALL: WriteMask = 15;
/// Port of `public static final int WRITE_NONE = 0`.
pub const WRITE_NONE: i32 = 0;
/// Port of `public static final int MAX_COLOR_TARGETS = 8`.
pub const MAX_COLOR_TARGETS: i32 = 8;

/// Port of `com.mojang.blaze3d.pipeline.ColorTargetState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorTargetState {
    blend_function: Option<BlendFunction>,
    format: GpuFormat,
    write_mask: WriteMask,
}

/// Port of `public static final ColorTargetState DEFAULT = new ColorTargetState(Optional.empty(),
/// GpuFormat.RGBA8_UNORM, 15)`.
pub const DEFAULT: ColorTargetState = ColorTargetState::new(None, GpuFormat::Rgba8Unorm, 15);

impl ColorTargetState {
    /// Port of the canonical constructor. Nothing is checked.
    pub const fn new(
        blend_function: Option<BlendFunction>,
        format: GpuFormat,
        write_mask: WriteMask,
    ) -> Self {
        ColorTargetState {
            blend_function,
            format,
            write_mask,
        }
    }

    /// Port of `public ColorTargetState(BlendFunction blendFunction)`:
    /// `this(Optional.of(blendFunction), GpuFormat.RGBA8_UNORM, 15)`.
    pub const fn of_blend_function(blend_function: BlendFunction) -> Self {
        Self::new(Some(blend_function), GpuFormat::Rgba8Unorm, 15)
    }

    /// Port of the record accessor `Optional<BlendFunction> blendFunction()`.
    pub const fn blend_function(&self) -> Option<BlendFunction> {
        self.blend_function
    }

    /// Port of the record accessor `GpuFormat format()`.
    pub const fn format(&self) -> GpuFormat {
        self.format
    }

    /// Port of the record accessor `@WriteMask int writeMask()`.
    pub const fn write_mask(&self) -> WriteMask {
        self.write_mask
    }

    /// Port of `public boolean writeRed()`: `(writeMask & 1) != 0`.
    pub const fn write_red(&self) -> bool {
        (self.write_mask & 1) != 0
    }

    /// Port of `public boolean writeGreen()`: `(writeMask & 2) != 0`.
    pub const fn write_green(&self) -> bool {
        (self.write_mask & 2) != 0
    }

    /// Port of `public boolean writeBlue()`: `(writeMask & 4) != 0`.
    pub const fn write_blue(&self) -> bool {
        (self.write_mask & 4) != 0
    }

    /// Port of `public boolean writeAlpha()`: `(writeMask & 8) != 0`.
    pub const fn write_alpha(&self) -> bool {
        (self.write_mask & 8) != 0
    }
}

impl std::fmt::Display for ColorTargetState {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.blend_function {
            Some(b) => write!(f, "ColorTargetState[blendFunction=Optional[{b}], ")?,
            None => f.write_str("ColorTargetState[blendFunction=Optional.empty, ")?,
        }
        write!(f, "format={}, writeMask={}]", self.format, self.write_mask)
    }
}
