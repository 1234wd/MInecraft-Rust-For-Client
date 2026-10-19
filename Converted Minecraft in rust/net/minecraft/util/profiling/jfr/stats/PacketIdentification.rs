//! Port of: net/minecraft/util/profiling/jfr/stats/PacketIdentification.java
//! Java class(es): net.minecraft.util.profiling.jfr.stats.PacketIdentification
//! Status: VERIFIED

use crate::javacompat::java_jfr::{null_event, RecordedEvent};
use crate::javacompat::java_lang::string_hash_code;
use crate::javacompat::java_object::{record_hash, str_or_null};
use crate::javacompat::throwable::ThrowableRef;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PacketIdentification {
    direction: Option<String>,
    protocol_id: Option<String>,
    packet_id: Option<String>,
}

impl PacketIdentification {
    pub fn new(
        direction: Option<String>,
        protocol_id: Option<String>,
        packet_id: Option<String>,
    ) -> Self {
        Self {
            direction,
            protocol_id,
            packet_id,
        }
    }

    pub fn from(event: Option<&RecordedEvent>) -> Result<Self, ThrowableRef> {
        let event = event.ok_or_else(|| null_event("getString"))?;
        Ok(Self::new(
            event.get_string(Some("packetDirection"))?,
            event.get_string(Some("protocolId"))?,
            event.get_string(Some("packetId"))?,
        ))
    }

    pub fn direction(&self) -> Option<&str> {
        self.direction.as_deref()
    }
    pub fn protocol_id(&self) -> Option<&str> {
        self.protocol_id.as_deref()
    }
    pub fn packet_id(&self) -> Option<&str> {
        self.packet_id.as_deref()
    }

    pub fn hash_code(&self) -> i32 {
        record_hash(&[
            self.direction.as_deref().map_or(0, string_hash_code),
            self.protocol_id.as_deref().map_or(0, string_hash_code),
            self.packet_id.as_deref().map_or(0, string_hash_code),
        ])
    }
}

impl std::fmt::Display for PacketIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PacketIdentification[direction={}, protocolId={}, packetId={}]",
            str_or_null(self.direction.as_deref()),
            str_or_null(self.protocol_id.as_deref()),
            str_or_null(self.packet_id.as_deref())
        )
    }
}
