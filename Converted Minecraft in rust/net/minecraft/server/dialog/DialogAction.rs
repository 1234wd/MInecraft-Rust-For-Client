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

impl std::fmt::Display for DialogAction {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            DialogAction::Close => "CLOSE",
            DialogAction::None => "NONE",
            DialogAction::WaitForResponse => "WAIT_FOR_RESPONSE",
        })
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for DialogAction {
    fn get_serialized_name(&self) -> &str {
        DialogAction::get_serialized_name(self)
    }
}

static BY_ID_MAP: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<DialogAction>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            DialogAction::id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Zero,
        )
    });

/// Java: `public static final IntFunction<DialogAction> BY_ID`, applied.
///
/// Session 19: unblocked by the `ByIdMap` port. `OutOfBoundsStrategy.ZERO`: an unknown id yields
/// `CLOSE` (the value whose id is 0).
pub fn BY_ID(id: i32) -> DialogAction {
    BY_ID_MAP.apply(id)
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