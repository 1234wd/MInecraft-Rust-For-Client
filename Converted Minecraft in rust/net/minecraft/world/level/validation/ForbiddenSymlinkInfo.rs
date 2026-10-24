//! Port of: net/minecraft/world/level/validation/ForbiddenSymlinkInfo.java
//! Java class(es): net.minecraft.world.level.validation.ForbiddenSymlinkInfo
//! Status: PORTED
//!
//! A record holding two `java.nio.file.Path` values (a symlink and its target).
//! `Path` is the narrow model in `javacompat::java_nio::Path`. The record's
//! `equals`/`hashCode`/`toString` are derived from the field values.

use crate::javacompat::java_nio::Path;

/// Port of `net.minecraft.world.level.validation.ForbiddenSymlinkInfo`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForbiddenSymlinkInfo {
    /// Port of the `link` component.
    pub link: Path,
    /// Port of the `target` component.
    pub target: Path,
}
