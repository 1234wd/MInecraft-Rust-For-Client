//! Port of: net/minecraft/world/level/block/state/properties/SideChainPart.java
//! Java class(es): net.minecraft.world.level.block.state.properties.SideChainPart
//! Status: VERIFIED
//!
//! Session 19. Four constants carrying a `String name`, plus the chain-state transition methods.
//! Every member ported; ordinals and the `name` field of every constant are checked against the
//! jar (`session19_a.rs`), and every method is tested over all inputs.
//!
//! # QUIRKS, TRANSCRIBED AS WRITTEN
//!
//! * Declaration order is `UNCONNECTED, RIGHT, CENTER, LEFT` -- `RIGHT` before `LEFT`.
//! * `isConnectionTowards(endPart)` is `this == CENTER || this == endPart`, so
//!   `UNCONNECTED.isConnectionTowards(UNCONNECTED)` is `true`.
//! * `isChainEnd()` is `this != CENTER`, so `UNCONNECTED` counts as a chain end.
//!
//! `toString()` is overridden to `getSerializedName()`, so `Display` prints `unconnected`.

/// Port of `net.minecraft.world.level.block.state.properties.SideChainPart`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SideChainPart {
    /// `UNCONNECTED` -- ordinal 0, `"unconnected"`.
    Unconnected = 0,
    /// `RIGHT` -- ordinal 1, `"right"`.
    Right = 1,
    /// `CENTER` -- ordinal 2, `"center"`.
    Center = 2,
    /// `LEFT` -- ordinal 3, `"left"`.
    Left = 3,
}

/// Java: `SideChainPart.values()`, declaration order.
pub const VALUES: [SideChainPart; 4] = [
    SideChainPart::Unconnected,
    SideChainPart::Right,
    SideChainPart::Center,
    SideChainPart::Left,
];

impl SideChainPart {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            SideChainPart::Unconnected => "UNCONNECTED",
            SideChainPart::Right => "RIGHT",
            SideChainPart::Center => "CENTER",
            SideChainPart::Left => "LEFT",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `SideChainPart#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            SideChainPart::Unconnected => "unconnected",
            SideChainPart::Right => "right",
            SideChainPart::Center => "center",
            SideChainPart::Left => "left",
        }
    }

    /// Port of `SideChainPart#isConnected()`: `this != UNCONNECTED`.
    #[inline]
    pub const fn is_connected(&self) -> bool {
        !matches!(self, SideChainPart::Unconnected)
    }

    /// Port of `SideChainPart#isConnectionTowards(SideChainPart)`: `this == CENTER || this == endPart`.
    #[inline]
    pub const fn is_connection_towards(&self, end_part: SideChainPart) -> bool {
        matches!(self, SideChainPart::Center) || *self as i32 == end_part as i32
    }

    /// Port of `SideChainPart#isChainEnd()`: `this != CENTER`.
    #[inline]
    pub const fn is_chain_end(&self) -> bool {
        !matches!(self, SideChainPart::Center)
    }

    /// Port of `SideChainPart#whenConnectedToTheRight()`.
    #[inline]
    pub const fn when_connected_to_the_right(&self) -> SideChainPart {
        match self {
            SideChainPart::Unconnected | SideChainPart::Left => SideChainPart::Left,
            SideChainPart::Right | SideChainPart::Center => SideChainPart::Center,
        }
    }

    /// Port of `SideChainPart#whenConnectedToTheLeft()`.
    #[inline]
    pub const fn when_connected_to_the_left(&self) -> SideChainPart {
        match self {
            SideChainPart::Unconnected | SideChainPart::Right => SideChainPart::Right,
            SideChainPart::Center | SideChainPart::Left => SideChainPart::Center,
        }
    }

    /// Port of `SideChainPart#whenDisconnectedFromTheRight()`.
    #[inline]
    pub const fn when_disconnected_from_the_right(&self) -> SideChainPart {
        match self {
            SideChainPart::Unconnected | SideChainPart::Left => SideChainPart::Unconnected,
            SideChainPart::Right | SideChainPart::Center => SideChainPart::Right,
        }
    }

    /// Port of `SideChainPart#whenDisconnectedFromTheLeft()`.
    #[inline]
    pub const fn when_disconnected_from_the_left(&self) -> SideChainPart {
        match self {
            SideChainPart::Unconnected | SideChainPart::Right => SideChainPart::Unconnected,
            SideChainPart::Center | SideChainPart::Left => SideChainPart::Left,
        }
    }
}

impl std::fmt::Display for SideChainPart {
    /// Port of the `toString()` override: `getSerializedName()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_serialized_name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for SideChainPart {
    fn get_serialized_name(&self) -> &str {
        SideChainPart::get_serialized_name(self)
    }
}
