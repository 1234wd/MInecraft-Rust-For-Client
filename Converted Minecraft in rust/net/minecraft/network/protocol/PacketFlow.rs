//! Port of: net/minecraft/network/protocol/PacketFlow.java
//! Java class(es): net.minecraft.network.protocol.PacketFlow
//! Status: VERIFIED
//!
//! Two constants and one derived value. Verified against the jar by `_porting/tests/consts.rs`:
//! the ordinals **and** the `id` field of each constant.
//!
//! # `SERVERBOUND` IS ORDINAL 0, NOT 1
//!
//! The declaration reads `SERVERBOUND("serverbound"), CLIENTBOUND("clientbound")`, and the
//! ordinals follow the declaration. It is tempting to read `CLIENTBOUND` as "the first one"
//! because it is the one players think about first; it is ordinal **1**. `PacketFlow` is not
//! written to the wire by ordinal today, but the ids are the persisted strings and those are
//! checked here.
//!
//! # `getOpposite` IS A COMPARISON, NOT AN INDEX
//!
//! ```java
//! return this == CLIENTBOUND ? SERVERBOUND : CLIENTBOUND;
//! ```
//!
//! With exactly two constants that is the same as `VALUES[1 - ordinal]`, but it is written as a
//! comparison because it reads that way in the source and because an index-based version would
//! silently change behaviour if a third constant were ever added.

/// Port of `net.minecraft.network.protocol.PacketFlow`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PacketFlow {
    /// Port of `PacketFlow#SERVERBOUND` -- ordinal 0, id `"serverbound"`.
    Serverbound = 0,
    /// Port of `PacketFlow#CLIENTBOUND` -- ordinal 1, id `"clientbound"`.
    Clientbound = 1,
}

/// Java: `PacketFlow.values()`.
pub const VALUES: [PacketFlow; 2] = [PacketFlow::Serverbound, PacketFlow::Clientbound];

impl PacketFlow {
    /// Port of `PacketFlow#getOpposite()`.
    #[inline]
    pub fn get_opposite(&self) -> PacketFlow {
        if *self == PacketFlow::Clientbound {
            PacketFlow::Serverbound
        } else {
            PacketFlow::Clientbound
        }
    }

    /// Port of `PacketFlow#id()`.
    #[inline]
    pub const fn id(&self) -> &'static str {
        match self {
            PacketFlow::Serverbound => "serverbound",
            PacketFlow::Clientbound => "clientbound",
        }
    }
}