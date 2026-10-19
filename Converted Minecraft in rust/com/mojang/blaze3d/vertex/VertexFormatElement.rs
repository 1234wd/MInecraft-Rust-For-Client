//! Port of: com/mojang/blaze3d/vertex/VertexFormatElement.java
//! Java class(es): com.mojang.blaze3d.vertex.VertexFormatElement
//! Status: PARTIAL
//!
//! Session 24. `public record VertexFormatElement(String name, int offset, GpuFormat format)`: one
//! attribute of a vertex layout.
//!
//! * `toString` is overridden: `String.format(Locale.ROOT, "%s %s offset:%d", name, format,
//!   offset)` -- the format's enum name, and the offset as plain decimal (`Locale.ROOT` has no
//!   grouping, and `%d` prints a minus sign for a negative int);
//! * `equals` is the record's: `name` by `String.equals`, `offset` by value, `format` by identity
//!   (an enum constant), which is the derived `PartialEq`;
//! * `hashCode` folds in `format`'s IDENTITY hash (an enum), so it differs on every JVM run and is
//!   not ported (OPEN_QUESTIONS #30). The derived `Hash` is consistent with `equals`.
//!
//! `name` is a non-null `String` here. Java would accept `null` (and print `null`); no vanilla
//! caller passes one -- every element is built from a literal in `VertexFormat`'s builders.
//!
//! Verified by the `VFE` group of `sweep_probe.txt` (`tests/session24.rs`).

use crate::com::mojang::blaze3d::GpuFormat::GpuFormat;

/// Port of `com.mojang.blaze3d.vertex.VertexFormatElement`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VertexFormatElement {
    name: String,
    offset: i32,
    format: GpuFormat,
}

impl VertexFormatElement {
    /// Port of the canonical constructor.
    pub fn new(name: impl Into<String>, offset: i32, format: GpuFormat) -> Self {
        VertexFormatElement {
            name: name.into(),
            offset,
            format,
        }
    }

    /// Port of the record accessor `String name()`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Port of the record accessor `int offset()`.
    pub fn offset(&self) -> i32 {
        self.offset
    }

    /// Port of the record accessor `GpuFormat format()`.
    pub fn format(&self) -> GpuFormat {
        self.format
    }
}

impl std::fmt::Display for VertexFormatElement {
    /// Port of `toString()`: `String.format(Locale.ROOT, "%s %s offset:%d", name, format, offset)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} offset:{}", self.name, self.format, self.offset)
    }
}
