//! Port of: net/minecraft/network/protocol/handshake/ClientIntent.java
//! Java class(es): net.minecraft.network.protocol.handshake.ClientIntent
//! Status: VERIFIED
//!
//! Session 19. Three constants, the private wire ids, `byId` and `id`. Names and ordinals are
//! checked against the jar in `_porting/tests/session19_b.rs`; the id mapping and the exception
//! text are tested from the source.
//!
//! # THE WIRE ID IS ORDINAL + 1, AND UNKNOWN IDS THROW
//!
//! `STATUS` is ordinal 0 but id `1` (`LOGIN` 2, `TRANSFER` 3). `byId` does not fall back: any
//! other int (including `0`) throws `IllegalArgumentException("Unknown connection intent: " +
//! id)`, which the port raises as a panic with that exact text.

/// Java: `private static final int STATUS_ID = 1`.
const STATUS_ID: i32 = 1;
/// Java: `private static final int LOGIN_ID = 2`.
const LOGIN_ID: i32 = 2;
/// Java: `private static final int TRANSFER_ID = 3`.
const TRANSFER_ID: i32 = 3;

/// Port of `net.minecraft.network.protocol.handshake.ClientIntent`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClientIntent {
    /// Port of `ClientIntent#STATUS` -- ordinal 0, id `1`.
    Status = 0,
    /// Port of `ClientIntent#LOGIN` -- ordinal 1, id `2`.
    Login = 1,
    /// Port of `ClientIntent#TRANSFER` -- ordinal 2, id `3`.
    Transfer = 2,
}

/// Java: `ClientIntent.values()` -- declaration order.
pub const VALUES: [ClientIntent; 3] = [ClientIntent::Status, ClientIntent::Login, ClientIntent::Transfer];

impl ClientIntent {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ClientIntent::Status => "STATUS",
            ClientIntent::Login => "LOGIN",
            ClientIntent::Transfer => "TRANSFER",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `ClientIntent#id()` -- the wire id (`1`, `2`, `3`), not the ordinal.
    #[inline]
    pub const fn id(&self) -> i32 {
        match self {
            ClientIntent::Status => STATUS_ID,
            ClientIntent::Login => LOGIN_ID,
            ClientIntent::Transfer => TRANSFER_ID,
        }
    }
}

/// Port of `ClientIntent#byId(int)`.
///
/// Panics with `java.lang.IllegalArgumentException: Unknown connection intent: <id>` for any id
/// other than 1, 2 or 3.
pub fn by_id(id: i32) -> ClientIntent {
    match id {
        STATUS_ID => ClientIntent::Status,
        LOGIN_ID => ClientIntent::Login,
        TRANSFER_ID => ClientIntent::Transfer,
        _ => panic!("java.lang.IllegalArgumentException: Unknown connection intent: {id}"),
    }
}

impl std::fmt::Display for ClientIntent {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
