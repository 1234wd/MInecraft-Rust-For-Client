//! Port of: net/minecraft/client/ClientBrandRetriever.java
//! Java class(es): net.minecraft.client.ClientBrandRetriever
//! Status: VERIFIED
//!
//! Session 19 batch h. `VANILLA_NAME` (checked against the jar's dump) and
//! `getClientModName()`. Nothing is PORT-BLOCKED.
//!
//! Note that `getClientModName()` returns the literal `"vanilla"`, not a read of `VANILLA_NAME`;
//! mod loaders patch the method body, which is why it is not written as a constant read. The
//! values are equal, which the tests pin.

/// Port of `ClientBrandRetriever#VANILLA_NAME`.
pub const VANILLA_NAME: &str = "vanilla";

/// Port of `ClientBrandRetriever#getClientModName()`.
pub fn get_client_mod_name() -> &'static str {
    "vanilla"
}
