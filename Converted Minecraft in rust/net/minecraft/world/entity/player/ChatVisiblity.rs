//! Port of: net/minecraft/world/entity/player/ChatVisiblity.java
//! Java class(es): net.minecraft.world.entity.player.ChatVisiblity
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the three constants with `id`, the private `BY_ID` (as
//! [`by_id`]) and the caption's translation key. Constants, `id` and the `caption`'s key are
//! checked against the jar in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `caption()` on `net.minecraft.network.chat.Component`; `LEGACY_CODEC`
//! (`Codec.INT.xmap(BY_ID::apply, v -> v.id)`) on DataFixerUpper.
//!
//! The class name's misspelling ("Visiblity") is vanilla's and is kept.
//!
//! # `BY_ID` WRAPS
//!
//! Unlike most id enums, `OutOfBoundsStrategy.WRAP`: the options file stores this as an int, and
//! an out-of-range value decodes by `Math.floorMod(id, 3)` -- `3` is `FULL`, `-1` is `HIDDEN`.

use crate::net::minecraft::util::ByIdMap;

/// Port of `net.minecraft.world.entity.player.ChatVisiblity`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChatVisiblity {
    /// Port of `ChatVisiblity#FULL` -- ordinal 0, id 0.
    Full = 0,
    /// Port of `ChatVisiblity#SYSTEM` -- ordinal 1, id 1.
    System = 1,
    /// Port of `ChatVisiblity#HIDDEN` -- ordinal 2, id 2.
    Hidden = 2,
}

/// Java: `ChatVisiblity.values()` -- declaration order.
pub const VALUES: [ChatVisiblity; 3] = [ChatVisiblity::Full, ChatVisiblity::System, ChatVisiblity::Hidden];

impl ChatVisiblity {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            ChatVisiblity::Full => "FULL",
            ChatVisiblity::System => "SYSTEM",
            ChatVisiblity::Hidden => "HIDDEN",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `id` field (what `LEGACY_CODEC` encodes).
    pub const fn id(&self) -> i32 {
        match self {
            ChatVisiblity::Full => 0,
            ChatVisiblity::System => 1,
            ChatVisiblity::Hidden => 2,
        }
    }

    /// The constructor's `key` argument. Java does not keep it as a field; it goes straight into
    /// `caption = Component.translatable(key)`. Exposed so `caption()` can be built once
    /// `Component` is ported.
    pub const fn caption_translation_key(&self) -> &'static str {
        match self {
            ChatVisiblity::Full => "options.chat.visibility.full",
            ChatVisiblity::System => "options.chat.visibility.system",
            ChatVisiblity::Hidden => "options.chat.visibility.hidden",
        }
    }
}

impl std::fmt::Display for ChatVisiblity {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

static BY_ID: std::sync::LazyLock<ByIdMap::Continuous<ChatVisiblity>> = std::sync::LazyLock::new(|| {
    ByIdMap::continuous(ChatVisiblity::id, &VALUES, ByIdMap::OutOfBoundsStrategy::Wrap)
});

/// Java: `private static final IntFunction<ChatVisiblity> BY_ID`, applied (the decoding half of
/// `LEGACY_CODEC`).
///
/// `OutOfBoundsStrategy.WRAP`: `Math.floorMod(id, 3)`.
pub fn by_id(id: i32) -> ChatVisiblity {
    BY_ID.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public Component caption()`.
///
/// PORT-BLOCKED on `net.minecraft.network.chat.Component`. The key is
/// [`ChatVisiblity::caption_translation_key`].
pub fn caption(_v: ChatVisiblity) {
    todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
}

/// Java: `public static final Codec<ChatVisiblity> LEGACY_CODEC = Codec.INT.xmap(BY_ID::apply, v -> v.id)`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`. Decoding is [`by_id`], encoding is
/// [`ChatVisiblity::id`].
pub fn LEGACY_CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
