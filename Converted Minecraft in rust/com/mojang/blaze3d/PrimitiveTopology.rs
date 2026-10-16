//! Port of: com/mojang/blaze3d/PrimitiveTopology.java
//! Java class(es): com.mojang.blaze3d.PrimitiveTopology
//! Status: PORTED
//!
//! Session 19. Every member is ported. Not `VERIFIED`: the jar's constant dump has no group for
//! this class (the blaze3d classes did not load in ConstOracle), so names and ordinals are taken
//! from the declaration order in the source and pinned by `session19_consts.rs`.
//!
//! `indexCount` turns quads into triangle pairs: `vertexCount / 4 * 6`, Java `int` arithmetic, so
//! the division truncates toward zero FIRST (5 vertices -> 6 indices, not 7.5) and the multiply wraps
//! on overflow. `LINES` shares the quad formula because vanilla draws lines as screen-space quads.

/// Port of `com.mojang.blaze3d.PrimitiveTopology`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveTopology {
    /// `LINES` -- ordinal 0.
    Lines = 0,
    /// `DEBUG_LINES` -- ordinal 1.
    DebugLines = 1,
    /// `DEBUG_LINE_STRIP` -- ordinal 2.
    DebugLineStrip = 2,
    /// `POINTS` -- ordinal 3.
    Points = 3,
    /// `TRIANGLES` -- ordinal 4.
    Triangles = 4,
    /// `TRIANGLE_STRIP` -- ordinal 5.
    TriangleStrip = 5,
    /// `TRIANGLE_FAN` -- ordinal 6.
    TriangleFan = 6,
    /// `QUADS` -- ordinal 7.
    Quads = 7,
}

/// Java: `PrimitiveTopology.values()`, declaration order.
pub const VALUES: [PrimitiveTopology; 8] = [
    PrimitiveTopology::Lines,
    PrimitiveTopology::DebugLines,
    PrimitiveTopology::DebugLineStrip,
    PrimitiveTopology::Points,
    PrimitiveTopology::Triangles,
    PrimitiveTopology::TriangleStrip,
    PrimitiveTopology::TriangleFan,
    PrimitiveTopology::Quads,
];

impl PrimitiveTopology {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PrimitiveTopology::Lines => "LINES",
            PrimitiveTopology::DebugLines => "DEBUG_LINES",
            PrimitiveTopology::DebugLineStrip => "DEBUG_LINE_STRIP",
            PrimitiveTopology::Points => "POINTS",
            PrimitiveTopology::Triangles => "TRIANGLES",
            PrimitiveTopology::TriangleStrip => "TRIANGLE_STRIP",
            PrimitiveTopology::TriangleFan => "TRIANGLE_FAN",
            PrimitiveTopology::Quads => "QUADS",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Java: `public final int primitiveLength`.
    pub const fn primitive_length(&self) -> i32 {
        match self {
            PrimitiveTopology::Lines => 2,
            PrimitiveTopology::DebugLines => 2,
            PrimitiveTopology::DebugLineStrip => 2,
            PrimitiveTopology::Points => 1,
            PrimitiveTopology::Triangles => 3,
            PrimitiveTopology::TriangleStrip => 3,
            PrimitiveTopology::TriangleFan => 3,
            PrimitiveTopology::Quads => 4,
        }
    }

    /// Java: `public final int primitiveStride`.
    pub const fn primitive_stride(&self) -> i32 {
        match self {
            PrimitiveTopology::Lines => 2,
            PrimitiveTopology::DebugLines => 2,
            PrimitiveTopology::DebugLineStrip => 1,
            PrimitiveTopology::Points => 1,
            PrimitiveTopology::Triangles => 3,
            PrimitiveTopology::TriangleStrip => 1,
            PrimitiveTopology::TriangleFan => 1,
            PrimitiveTopology::Quads => 4,
        }
    }

    /// Java: `public final boolean connectedPrimitives`.
    pub const fn connected_primitives(&self) -> bool {
        match self {
            PrimitiveTopology::Lines => false,
            PrimitiveTopology::DebugLines => false,
            PrimitiveTopology::DebugLineStrip => true,
            PrimitiveTopology::Points => false,
            PrimitiveTopology::Triangles => false,
            PrimitiveTopology::TriangleStrip => true,
            PrimitiveTopology::TriangleFan => true,
            PrimitiveTopology::Quads => false,
        }
    }

    /// Port of `PrimitiveTopology#indexCount(int)`.
    pub const fn index_count(&self, vertex_count: i32) -> i32 {
        match self {
            PrimitiveTopology::Lines | PrimitiveTopology::Quads => (vertex_count / 4).wrapping_mul(6),
            PrimitiveTopology::DebugLines
            | PrimitiveTopology::DebugLineStrip
            | PrimitiveTopology::Points
            | PrimitiveTopology::Triangles
            | PrimitiveTopology::TriangleStrip
            | PrimitiveTopology::TriangleFan => vertex_count,
        }
    }
}

impl std::fmt::Display for PrimitiveTopology {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
