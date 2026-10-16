//! Port of: net/minecraft/server/packs/PackType.java
//! Java class(es): net.minecraft.server.packs.PackType
//! Status: VERIFIED
//!
//! Session 19. Two constants, each with the top-level pack `directory`. Names, ordinals and
//! `directory` are checked against the jar in `_porting/tests/session19_b.rs`.

/// Port of `net.minecraft.server.packs.PackType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PackType {
    /// Port of `PackType#CLIENT_RESOURCES` -- ordinal 0, directory `"assets"`.
    ClientResources = 0,
    /// Port of `PackType#SERVER_DATA` -- ordinal 1, directory `"data"`.
    ServerData = 1,
}

/// Java: `PackType.values()` -- declaration order.
pub const VALUES: [PackType; 2] = [PackType::ClientResources, PackType::ServerData];

impl PackType {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            PackType::ClientResources => "CLIENT_RESOURCES",
            PackType::ServerData => "SERVER_DATA",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `PackType#getDirectory()`.
    #[inline]
    pub const fn get_directory(&self) -> &'static str {
        match self {
            PackType::ClientResources => "assets",
            PackType::ServerData => "data",
        }
    }
}

impl std::fmt::Display for PackType {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
