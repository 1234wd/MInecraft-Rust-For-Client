//! Port of: net/minecraft/network/ConnectionProtocol.java
//! Java class(es): net.minecraft.network.ConnectionProtocol
//! Status: VERIFIED
//!
//! Session 19. Five constants, each with a protocol `id` string. Names, ordinals and `id` are
//! checked against the jar in `_porting/tests/session19_b.rs`.
//!
//! `HANDSHAKING`'s id is `"handshake"`, not `"handshaking"`: the id and the constant name differ
//! for that one constant only.

/// Port of `net.minecraft.network.ConnectionProtocol`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    /// Port of `ConnectionProtocol#HANDSHAKING` -- ordinal 0, id `"handshake"`.
    Handshaking = 0,
    /// Port of `ConnectionProtocol#PLAY` -- ordinal 1, id `"play"`.
    Play = 1,
    /// Port of `ConnectionProtocol#STATUS` -- ordinal 2, id `"status"`.
    Status = 2,
    /// Port of `ConnectionProtocol#LOGIN` -- ordinal 3, id `"login"`.
    Login = 3,
    /// Port of `ConnectionProtocol#CONFIGURATION` -- ordinal 4, id `"configuration"`.
    Configuration = 4,
}

/// Java: `ConnectionProtocol.values()` -- declaration order.
pub const VALUES: [ConnectionProtocol; 5] = [
    ConnectionProtocol::Handshaking,
    ConnectionProtocol::Play,
    ConnectionProtocol::Status,
    ConnectionProtocol::Login,
    ConnectionProtocol::Configuration,
];

impl ConnectionProtocol {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ConnectionProtocol::Handshaking => "HANDSHAKING",
            ConnectionProtocol::Play => "PLAY",
            ConnectionProtocol::Status => "STATUS",
            ConnectionProtocol::Login => "LOGIN",
            ConnectionProtocol::Configuration => "CONFIGURATION",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ConnectionProtocol#id()`.
    #[inline]
    pub const fn id(&self) -> &'static str {
        match self {
            ConnectionProtocol::Handshaking => "handshake",
            ConnectionProtocol::Play => "play",
            ConnectionProtocol::Status => "status",
            ConnectionProtocol::Login => "login",
            ConnectionProtocol::Configuration => "configuration",
        }
    }
}

impl std::fmt::Display for ConnectionProtocol {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
