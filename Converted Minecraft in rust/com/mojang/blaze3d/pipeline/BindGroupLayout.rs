//! Port of: com/mojang/blaze3d/pipeline/BindGroupLayout.java
//! Java class(es): com.mojang.blaze3d.pipeline.BindGroupLayout, com.mojang.blaze3d.pipeline.BindGroupLayout.Builder, com.mojang.blaze3d.pipeline.BindGroupLayout.UniformDescription
//! Status: PARTIAL
//!
//! Session 24. `public class BindGroupLayout`: the named samplers and uniforms one bind group of a
//! render pipeline exposes, built with a [`Builder`], plus three static helpers over a list of
//! layouts.
//!
//! # Members
//!
//! | Java | here |
//! |---|---|
//! | `private BindGroupLayout(List, List)` | built only by [`Builder::build`] |
//! | `static Builder builder()` | [`builder`] |
//! | `getSamplers()`, `getUniforms()` | the same, as slices (the lists are `List.copyOf` copies, immutable) |
//! | `static flattenSamplers(List)`, `flattenUniforms(List)` | [`flatten_samplers`], [`flatten_uniforms`] |
//! | `static ensureCompatible(List)` | [`ensure_compatible`] |
//! | `Builder.withSampler`, `withUniform(name, type)`, `withUniform(name, type, format)`, `build` | [`Builder::with_sampler`], [`Builder::with_uniform`], [`Builder::with_uniform_format`], [`Builder::build`] |
//! | `record UniformDescription(String name, UniformType type, @Nullable GpuFormat gpuFormat)` and its two extra constructors | [`UniformDescription::new`], [`UniformDescription::of_type`], [`UniformDescription::of_format`] |
//!
//! # Checks, in Java's order and with Java's texts
//!
//! * `withUniform(name, TEXEL_BUFFER)` throws "Cannot use texel buffer without specifying texture
//!   format"; `withUniform(name, type, format)` with any other type throws "Only texel buffer can
//!   specify texture format". The builder is unchanged either way.
//! * `UniformDescription(name, type)` runs the canonical constructor first and only then throws
//!   "Texel buffer needs a texture format" for `TEXEL_BUFFER`. The canonical constructor checks
//!   nothing, so `(name, TEXEL_BUFFER, null)` and `(name, UNIFORM_BUFFER, format)` are accepted.
//! * `ensureCompatible` walks the layouts in order and, in each, the uniforms before the samplers,
//!   adding every name to one `HashSet`; the first name already present throws "Duplicate bind
//!   name '<name>' in bind group layout <index>". Only membership is asked of the set, so its
//!   iteration order never shows.
//!
//! # Not ported
//!
//! * `BindGroupLayout` overrides neither `equals` nor `toString`: they are `Object`'s identity
//!   versions, with an identity hash in the text (OPEN_QUESTIONS #30). No `PartialEq`/`Display`.
//! * `UniformDescription.hashCode` folds the identity hashes of two enums (#30). Its `equals` and
//!   `toString` are portable and ported.
//! * Strings are non-null here. In Java a `null` sampler name would make `build()` throw from
//!   `List.copyOf`; no vanilla caller passes one (every name is a literal).
//!
//! Verified by the `BINDGL` and `UNIDESC` groups of `sweep_probe.txt` (`tests/session24.rs`).

use std::collections::HashSet;

use crate::com::mojang::blaze3d::shaders::UniformType::UniformType;
use crate::com::mojang::blaze3d::GpuFormat::GpuFormat;

/// Port of `com.mojang.blaze3d.pipeline.BindGroupLayout.UniformDescription`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniformDescription {
    name: String,
    type_: UniformType,
    gpu_format: Option<GpuFormat>,
}

impl UniformDescription {
    /// Port of the canonical constructor `UniformDescription(String name, UniformType type,
    /// @Nullable GpuFormat gpuFormat)`. It checks nothing.
    pub fn new(name: impl Into<String>, type_: UniformType, gpu_format: Option<GpuFormat>) -> Self {
        UniformDescription {
            name: name.into(),
            type_,
            gpu_format,
        }
    }

    /// Port of `UniformDescription(String name, UniformType type)`: `this(name, type, null)`,
    /// then `IllegalArgumentException` for `TEXEL_BUFFER`.
    pub fn of_type(name: impl Into<String>, type_: UniformType) -> Self {
        let d = Self::new(name, type_, None);
        if type_ == UniformType::TexelBuffer {
            panic!("java.lang.IllegalArgumentException: Texel buffer needs a texture format");
        }
        d
    }

    /// Port of `UniformDescription(String name, GpuFormat gpuFormat)`: a `TEXEL_BUFFER` of that
    /// format.
    pub fn of_format(name: impl Into<String>, gpu_format: GpuFormat) -> Self {
        Self::new(name, UniformType::TexelBuffer, Some(gpu_format))
    }

    /// Port of the record accessor `String name()`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Port of the record accessor `UniformType type()`.
    pub fn type_(&self) -> UniformType {
        self.type_
    }

    /// Port of the record accessor `@Nullable GpuFormat gpuFormat()`.
    pub fn gpu_format(&self) -> Option<GpuFormat> {
        self.gpu_format
    }
}

impl std::fmt::Display for UniformDescription {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UniformDescription[name={}, type={}, gpuFormat=",
            self.name, self.type_
        )?;
        match self.gpu_format {
            Some(g) => write!(f, "{g}]"),
            None => f.write_str("null]"),
        }
    }
}

/// Port of `com.mojang.blaze3d.pipeline.BindGroupLayout`.
#[derive(Debug, Clone)]
pub struct BindGroupLayout {
    samplers: Vec<String>,
    uniforms: Vec<UniformDescription>,
}

/// Port of `public static BindGroupLayout.Builder builder()`.
pub fn builder() -> Builder {
    Builder {
        samplers: Vec::new(),
        uniforms: Vec::new(),
    }
}

impl BindGroupLayout {
    /// Port of `public List<String> getSamplers()`.
    pub fn get_samplers(&self) -> &[String] {
        &self.samplers
    }

    /// Port of `public List<BindGroupLayout.UniformDescription> getUniforms()`.
    pub fn get_uniforms(&self) -> &[UniformDescription] {
        &self.uniforms
    }
}

/// Port of `public static List<String> flattenSamplers(List<BindGroupLayout> bindGroupLayouts)`.
pub fn flatten_samplers(bind_group_layouts: &[&BindGroupLayout]) -> Vec<String> {
    let mut flattened = Vec::new();
    for layout in bind_group_layouts {
        flattened.extend(layout.get_samplers().iter().cloned());
    }
    flattened
}

/// Port of `public static List<UniformDescription> flattenUniforms(List<BindGroupLayout> bindGroupLayouts)`.
pub fn flatten_uniforms(bind_group_layouts: &[&BindGroupLayout]) -> Vec<UniformDescription> {
    let mut flattened = Vec::new();
    for layout in bind_group_layouts {
        flattened.extend(layout.get_uniforms().iter().cloned());
    }
    flattened
}

/// Port of `public static void ensureCompatible(List<BindGroupLayout> bindGroupLayouts)`.
pub fn ensure_compatible(bind_group_layouts: &[&BindGroupLayout]) {
    let mut names: HashSet<&str> = HashSet::new();
    for (layout_index, layout) in bind_group_layouts.iter().enumerate() {
        for uniform in layout.get_uniforms() {
            if !names.insert(uniform.name()) {
                panic!(
                    "java.lang.IllegalArgumentException: Duplicate bind name '{}' in bind group layout {layout_index}",
                    uniform.name()
                );
            }
        }
        for sampler in layout.get_samplers() {
            if !names.insert(sampler) {
                panic!(
                    "java.lang.IllegalArgumentException: Duplicate bind name '{sampler}' in bind group layout {layout_index}"
                );
            }
        }
    }
}

/// Port of `com.mojang.blaze3d.pipeline.BindGroupLayout.Builder`.
#[derive(Debug, Clone)]
pub struct Builder {
    samplers: Vec<String>,
    uniforms: Vec<UniformDescription>,
}

impl Builder {
    /// Port of `public BindGroupLayout.Builder withSampler(String sampler)`.
    pub fn with_sampler(&mut self, sampler: impl Into<String>) -> &mut Self {
        self.samplers.push(sampler.into());
        self
    }

    /// Port of `public BindGroupLayout.Builder withUniform(String name, UniformType type)`.
    pub fn with_uniform(&mut self, name: impl Into<String>, type_: UniformType) -> &mut Self {
        if type_ == UniformType::TexelBuffer {
            panic!(
                "java.lang.IllegalArgumentException: Cannot use texel buffer without specifying texture format"
            );
        }
        self.uniforms.push(UniformDescription::of_type(name, type_));
        self
    }

    /// Port of `public BindGroupLayout.Builder withUniform(String name, UniformType type, GpuFormat format)`.
    pub fn with_uniform_format(
        &mut self,
        name: impl Into<String>,
        type_: UniformType,
        format: GpuFormat,
    ) -> &mut Self {
        if type_ != UniformType::TexelBuffer {
            panic!(
                "java.lang.IllegalArgumentException: Only texel buffer can specify texture format"
            );
        }
        self.uniforms
            .push(UniformDescription::of_format(name, format));
        self
    }

    /// Port of `public BindGroupLayout build()`: copies of both lists, so later builder calls do
    /// not reach a built layout.
    pub fn build(&self) -> BindGroupLayout {
        BindGroupLayout {
            samplers: self.samplers.clone(),
            uniforms: self.uniforms.clone(),
        }
    }
}
