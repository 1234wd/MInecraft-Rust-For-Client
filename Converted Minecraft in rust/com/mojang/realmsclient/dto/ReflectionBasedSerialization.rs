//! Port of: com/mojang/realmsclient/dto/ReflectionBasedSerialization.java
//! Java class(es): com.mojang.realmsclient.dto.ReflectionBasedSerialization
//! Status: VERIFIED
//!
//! Session 26. The Java declaration is an empty marker interface. Its purpose is to mark Realms
//! DTOs accepted by `GuardedSerializer`; serializer behavior belongs to that consumer. This Rust
//! marker has no invented methods or default behavior. The jar probe confirms that it declares no
//! members and has no superinterfaces.

/// Marker for DTOs whose fields are handled by the Realms reflection serializer.
pub trait ReflectionBasedSerialization {}
