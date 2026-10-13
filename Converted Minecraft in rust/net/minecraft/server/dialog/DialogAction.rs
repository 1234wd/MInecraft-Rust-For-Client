//! Port of: net/minecraft/server/dialog/DialogAction.java
//! Java class(es): net.minecraft.server.dialog.DialogAction
//! Status: VERIFIED
//!
//! Three constants, each carrying an `int id` **and** a `String name`. Verified against the jar
//! by `_porting/tests/consts.rs`: ordinals plus both fields for every constant.
//!
//! # TWO INDEPENDENT NUMBERINGS, AND THEY AGREE BY COINCIDENCE
//!
//! `id` is `0, 1, 2` and the ordinals are `0, 1, 2`, so they are the same today. They are
//! separate things: the ordinal is Java's enum ordinal, and `id` is what `BY_ID` maps from and
//! what structure data stores. A constant inserted at the front would shift the ordinals and
//! leave the ids alone. They are therefore ported as two separate things, with `id()` reading
//! the `id` field rather than the discriminant.
//!
//! # `willUnpause` IS AN EXPLICIT TWO-VALUE TEST, NOT AN ORDINAL RANGE
//!
//! ```java
//! return this == CLOSE || this == WAIT_FOR_RESPONSE;
//! ```
//!
//! Written as written. An ordinal test such as `!= NONE` would be shorter and would silently
//! change meaning if a fourth constant arrived.

/// Port of `net.minecraft.server.dialog.DialogAction`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DialogAction {
    /// Port of `DialogAction#CLOSE` -- ordinal 0, id `0`, name `"close"`.
    Close = 0,
    /// Port of `DialogAction#NONE` -- ordinal 1, id `1`, name `"none"`.
    None = 1,
    /// Port of `DialogAction#WAIT_FOR_RESPONSE` -- ordinal 2, id `2`, name `"wait_for_response"`.
    WaitForResponse = 2,
}

/// Java: `DialogAction.values()`.
pub const VALUES: [DialogAction; 3] = [
    DialogAction::Close,
    DialogAction::None,
    DialogAction::WaitForResponse,
];

impl DialogAction {
    /// Port of the private `id` field, read through `BY_ID`.
    ///
    /// Returns the stored `id`, NOT the discriminant -- they coincide today and the difference is
    /// invisible until someone inserts a constant.
    #[inline]
    pub const fn id(&self) -> i32 {
        match self {
            DialogAction::Close => 0,
            DialogAction::None => 1,
            DialogAction::WaitForResponse => 2,
        }
    }

    /// Port of `DialogAction#getSerializedName()` -- the `name` field.
    ///
    /// These are the command and structure-data spellings: `"close"`, `"none"`,
    /// `"wait_for_response"`.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            DialogAction::Close => "close",
            DialogAction::None => "none",
            DialogAction::WaitForResponse => "wait_for_response",
        }
    }

    /// Port of `DialogAction#willUnpause()`.
    #[inline]
    pub const fn will_unpause(&self) -> bool {
        matches!(self, DialogAction::Close | DialogAction::WaitForResponse)
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final IntFunction<DialogAction> BY_ID`.
///
/// `net.minecraft.util.ByIdMap` is not ported. The mapping itself is a pure function of `id()`,
/// so a caller needing behaviour can use [`DialogAction::id`] plus a local match; what cannot be
/// reproduced is `ByIdMap.OutOfBoundsStrategy.ZERO`, i.e. that an unknown id yields `CLOSE`.
pub fn BY_ID(_id: i32) -> DialogAction {
    todo!("PORT-BLOCKED: net.minecraft.util.ByIdMap is not ported")
}

/// Java: `public static final Codec<DialogAction> CODEC`.
///
/// `com.mojang.serialization.Codec` is not ported. See DESIGN_DECISIONS.md (#dfu-proposal).
pub fn CODEC() -> () {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, DialogAction> STREAM_CODEC`.
///
/// `netty::ByteBuf` is not ported, so there is no buffer type to write into.
pub fn STREAM_CODEC() -> () {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}