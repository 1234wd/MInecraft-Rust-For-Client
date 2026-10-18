//! Port of: net/minecraft/client/model/geom/builders/MaterialDefinition.java
//! Java class(es): net.minecraft.client.model.geom.builders.MaterialDefinition
//! Status: VERIFIED
//!
//! Session 22. `public class MaterialDefinition`: a model's texture size, two `final int` fields
//! and a constructor, nothing else.
//!
//! The fields are PACKAGE-PRIVATE in Java (no modifier) and read directly by the model builders in
//! `model/geom/builders`, so they are `pub` fields here rather than accessors. The class does not
//! override `equals`, so Java compares instances by identity; no `PartialEq` is derived.
//!
//! Verified by the `MATDEF` group of `small_probe.txt` (`tests/session22.rs`), which reads the two
//! fields back by reflection.

/// Port of `net.minecraft.client.model.geom.builders.MaterialDefinition`.
#[derive(Debug, Clone, Copy)]
pub struct MaterialDefinition {
    /// Port of `final int xTexSize` (package-private).
    pub x_tex_size: i32,
    /// Port of `final int yTexSize` (package-private).
    pub y_tex_size: i32,
}

impl MaterialDefinition {
    /// Port of `new MaterialDefinition(int xTexSize, int yTexSize)`.
    pub const fn new(x_tex_size: i32, y_tex_size: i32) -> Self {
        MaterialDefinition {
            x_tex_size,
            y_tex_size,
        }
    }
}
