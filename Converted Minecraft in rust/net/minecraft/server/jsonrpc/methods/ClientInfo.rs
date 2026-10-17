//! Port of: net/minecraft/server/jsonrpc/methods/ClientInfo.java
//! Java class(es): net.minecraft.server.jsonrpc.methods.ClientInfo
//! Status: VERIFIED
//!
//! Session 22. `public record ClientInfo(Integer connectionId)` with an `of(Integer)` factory
//! that is the constructor under another name.
//!
//! The component is a BOXED `Integer`, so it can be null, and the port holds an `Option<i32>`.
//! A null id prints as `ClientInfo[connectionId=null]` and hashes to `0` (`Objects.hashCode`),
//! which is the same hash as an id of `0` -- the probe has both, and they are unequal.
//!
//! Verified by the `REC`, `RECEQ` and `CLIENTINFO` groups of `small_probe.txt`
//! (`tests/session22.rs`), with null, zero, negative and `MIN_VALUE` ids.

use crate::javacompat::java_object;

/// Port of `net.minecraft.server.jsonrpc.methods.ClientInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientInfo {
    connection_id: Option<i32>,
}

impl ClientInfo {
    /// Port of the canonical constructor. `None` is a Java `null`.
    pub const fn new(connection_id: Option<i32>) -> Self {
        ClientInfo { connection_id }
    }

    /// Port of `of(Integer connectionId)`: `new ClientInfo(connectionId)`.
    pub const fn of(connection_id: Option<i32>) -> ClientInfo {
        ClientInfo::new(connection_id)
    }

    /// Port of the accessor `connectionId()`.
    pub const fn connection_id(&self) -> Option<i32> {
        self.connection_id
    }

    /// Port of the record's generated `hashCode()`: `Objects.hashCode(connectionId)`.
    pub fn java_hash_code(&self) -> i32 {
        java_object::record_hash(&[java_object::hash_opt(self.connection_id.as_ref(), |v| *v)])
    }
}

impl std::fmt::Display for ClientInfo {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.connection_id {
            Some(id) => write!(f, "ClientInfo[connectionId={id}]"),
            None => f.write_str("ClientInfo[connectionId=null]"),
        }
    }
}
