//! Port of: com/mojang/blaze3d/platform/DisplayData.java
//! Java class(es): com.mojang.blaze3d.platform.DisplayData
//! Status: VERIFIED
//!
//! Session 22. `public record DisplayData(int width, int height, OptionalInt fullscreenWidth,
//! OptionalInt fullscreenHeight, boolean isFullscreen)` with two copy-with-one-change methods.
//!
//! # `OptionalInt`, ported as `Option<i32>`
//!
//! Three `OptionalInt` behaviours show up through the record, and all three are probed:
//!
//! * `toString` is `OptionalInt[5]` when present and `OptionalInt.empty` when not -- so the record
//!   prints `fullscreenWidth=OptionalInt.empty`, not `None` and not `null`;
//! * `hashCode` is the value when present and `0` when empty (so `of(0)` and `empty()` collide,
//!   while `equals` still tells them apart);
//! * `equals` is presence-then-value.
//!
//! The component accessor `isFullscreen()` keeps its Java name, as `is_fullscreen`.
//! `withSize(width, height)` and `withFullscreen(isFullscreen)` copy everything else unchanged.
//!
//! Verified by the `REC`, `RECEQ` and `DISPLAY` groups of `small_probe.txt` (`tests/session22.rs`).

use crate::javacompat::java_object;

/// Port of `com.mojang.blaze3d.platform.DisplayData`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DisplayData {
    width: i32,
    height: i32,
    fullscreen_width: Option<i32>,
    fullscreen_height: Option<i32>,
    is_fullscreen: bool,
}

/// `OptionalInt#toString()`.
fn optional_int_to_string(v: Option<i32>) -> String {
    match v {
        Some(x) => format!("OptionalInt[{x}]"),
        None => "OptionalInt.empty".to_string(),
    }
}

/// `OptionalInt#hashCode()`: the value, or `0` when empty.
fn optional_int_hash(v: Option<i32>) -> i32 {
    v.unwrap_or(0)
}

impl DisplayData {
    /// Port of the canonical constructor. `None` is `OptionalInt.empty()`.
    pub const fn new(
        width: i32,
        height: i32,
        fullscreen_width: Option<i32>,
        fullscreen_height: Option<i32>,
        is_fullscreen: bool,
    ) -> Self {
        DisplayData {
            width,
            height,
            fullscreen_width,
            fullscreen_height,
            is_fullscreen,
        }
    }

    /// Port of the accessor `width()`.
    pub const fn width(&self) -> i32 {
        self.width
    }

    /// Port of the accessor `height()`.
    pub const fn height(&self) -> i32 {
        self.height
    }

    /// Port of the accessor `fullscreenWidth()`.
    pub const fn fullscreen_width(&self) -> Option<i32> {
        self.fullscreen_width
    }

    /// Port of the accessor `fullscreenHeight()`.
    pub const fn fullscreen_height(&self) -> Option<i32> {
        self.fullscreen_height
    }

    /// Port of the accessor `isFullscreen()`.
    pub const fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    /// Port of `withSize(int width, int height)`.
    pub const fn with_size(&self, width: i32, height: i32) -> DisplayData {
        DisplayData::new(
            width,
            height,
            self.fullscreen_width,
            self.fullscreen_height,
            self.is_fullscreen,
        )
    }

    /// Port of `withFullscreen(boolean isFullscreen)`.
    pub const fn with_fullscreen(&self, is_fullscreen: bool) -> DisplayData {
        DisplayData::new(
            self.width,
            self.height,
            self.fullscreen_width,
            self.fullscreen_height,
            is_fullscreen,
        )
    }

    /// Port of the record's generated `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[
            self.width,
            self.height,
            optional_int_hash(self.fullscreen_width),
            optional_int_hash(self.fullscreen_height),
            java_object::hash_bool(self.is_fullscreen),
        ])
    }
}

impl std::fmt::Display for DisplayData {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DisplayData[width={}, height={}, fullscreenWidth={}, fullscreenHeight={}, isFullscreen={}]",
            self.width,
            self.height,
            optional_int_to_string(self.fullscreen_width),
            optional_int_to_string(self.fullscreen_height),
            self.is_fullscreen
        )
    }
}
