//! Port of: net/minecraft/network/HandlerNames.java
//! Java class(es): net.minecraft.network.HandlerNames
//! Status: VERIFIED
//!
//! The names under which `Connection` registers its handlers in the netty channel pipeline.
//! A pure constant holder; all sixteen strings are checked against the jar's constant dump by
//! `_porting/tests/session19_e.rs`. The pipeline itself (netty) is not ported; these names only
//! matter once it is, because handlers are looked up and replaced by name.

/// Java: `HandlerNames.DECOMPRESS` = `"decompress"`.
pub const DECOMPRESS: &str = "decompress";
/// Java: `HandlerNames.COMPRESS` = `"compress"`.
pub const COMPRESS: &str = "compress";
/// Java: `HandlerNames.DECODER` = `"decoder"`.
pub const DECODER: &str = "decoder";
/// Java: `HandlerNames.ENCODER` = `"encoder"`.
pub const ENCODER: &str = "encoder";
/// Java: `HandlerNames.INBOUND_CONFIG` = `"inbound_config"`.
pub const INBOUND_CONFIG: &str = "inbound_config";
/// Java: `HandlerNames.OUTBOUND_CONFIG` = `"outbound_config"`.
pub const OUTBOUND_CONFIG: &str = "outbound_config";
/// Java: `HandlerNames.SPLITTER` = `"splitter"`.
pub const SPLITTER: &str = "splitter";
/// Java: `HandlerNames.PREPENDER` = `"prepender"`.
pub const PREPENDER: &str = "prepender";
/// Java: `HandlerNames.DECRYPT` = `"decrypt"`.
pub const DECRYPT: &str = "decrypt";
/// Java: `HandlerNames.ENCRYPT` = `"encrypt"`.
pub const ENCRYPT: &str = "encrypt";
/// Java: `HandlerNames.UNBUNDLER` = `"unbundler"`.
pub const UNBUNDLER: &str = "unbundler";
/// Java: `HandlerNames.BUNDLER` = `"bundler"`.
pub const BUNDLER: &str = "bundler";
/// Java: `HandlerNames.PACKET_HANDLER` = `"packet_handler"`.
pub const PACKET_HANDLER: &str = "packet_handler";
/// Java: `HandlerNames.TIMEOUT` = `"timeout"`.
pub const TIMEOUT: &str = "timeout";
/// Java: `HandlerNames.LEGACY_QUERY` = `"legacy_query"`.
pub const LEGACY_QUERY: &str = "legacy_query";
/// Java: `HandlerNames.LATENCY` = `"latency"`.
pub const LATENCY: &str = "latency";
