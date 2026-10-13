//! Port of: net/minecraft/advancements/AdvancementType.java
//! Java class(es): net.minecraft.advancements.AdvancementType
//! Status: PARTIAL
//!
//! Three constants, each carrying a `String name`, a `ChatFormatting chatColor` and a
//! `Component displayName`. Ordinals and the `name` field are verified against the jar by
//! `_porting/tests/consts.rs`.
//!
//! # `TASK` AND `GOAL` SHARE A COLOUR, AND THAT IS NOT A TYPO
//!
//! ```java
//! TASK("task", ChatFormatting.GREEN),
//! CHALLENGE("challenge", ChatFormatting.DARK_PURPLE),
//! GOAL("goal", ChatFormatting.GREEN);
//! ```
//!
//! Both are `GREEN`. The temptation is to "fix" `GOAL` to `AQUA` or `GOLD`, which would look
//! more like the vanilla toast colours a player remembers and would be a gameplay change. The
//! jar says `GREEN` twice and that is what is ported.
//!
//! # THE `displayName` IS A **LAZY** TRANSLATION KEY, NOT A STRING
//!
//! ```java
//! this.displayName = Component.translatable("advancements.toast." + name);
//! ```
//!
//! Built once in the constructor from the translation key `advancements.toast.<name>`.
//! `net.minecraft.network.chat.Component` is not ported, so the field is a `todo!` rather than a
//! fabricated string: resolving the key here would bake one locale's English text into the port
//! and hide the key that actually needs translating.

use crate::net::minecraft::ChatFormatting as _ChatFormattingUnused;

/// Referenced only in doc text until `ChatFormatting` is ported; see `get_chat_color`.

/// Port of `net.minecraft.advancements.AdvancementType`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AdvancementType {
    /// Port of `AdvancementType#TASK` -- ordinal 0, name `"task"`, colour `GREEN`.
    Task = 0,
    /// Port of `AdvancementType#CHALLENGE` -- ordinal 1, name `"challenge"`, colour
    /// `DARK_PURPLE`.
    Challenge = 1,
    /// Port of `AdvancementType#GOAL` -- ordinal 2, name `"goal"`, colour `GREEN`.
    Goal = 2,
}

/// Java: `AdvancementType.values()`.
pub const VALUES: [AdvancementType; 3] = [
    AdvancementType::Task,
    AdvancementType::Challenge,
    AdvancementType::Goal,
];

impl AdvancementType {
    /// Port of the private `name` field, via `getSerializedName()`.
    ///
    /// These are the persisted advancement-type strings in datapacks.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            AdvancementType::Task => "task",
            AdvancementType::Challenge => "challenge",
            AdvancementType::Goal => "goal",
        }
    }

    /// Port of `AdvancementType#getChatColor()`.
    ///
    /// `TASK` and `GOAL` are **both** `GREEN`; see the module docs.
    ///
    /// The return type is `ChatFormatting`, which is still a SKELETON, so this is a `todo!`
    /// rather than an invented colour type. The values themselves are not in doubt: the constant
    /// dump records `chatColor` for all three constants, and `_porting/tests/consts.rs` asserts
    /// them. Whoever ports `ChatFormatting` only has to delete this `todo!` and give it a
    /// return type.
    pub fn get_chat_color(&self) -> () {
        let _ = self.get_serialized_name();
        todo!("PORT-BLOCKED: net.minecraft.ChatFormatting is a SKELETON (values verified in consts.txt)")
    }

    /// Port of the private `displayName` field.
    ///
    /// The value is a lazy translation of `advancements.toast.<name>`, constructed once in the
    /// Java constructor. `net.minecraft.network.chat.Component` is not ported.
    pub fn get_display_name(&self) -> () {
        let _ = self.get_serialized_name();
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Port of `AdvancementType#createAnnouncement(AdvancementHolder, ServerPlayer)`.
    ///
    /// Builds `chat.type.advancement.<name>` with the player and advancement names as arguments.
    /// All three inputs are unported types.
    #[allow(clippy::too_many_arguments)]
    pub fn create_announcement(&self, _holder: (), _player: ()) -> () {
        let _ = self.get_serialized_name();
        todo!("PORT-BLOCKED: Component, AdvancementHolder and ServerPlayer are not ported")
    }
}

/// Java: `public static final Codec<AdvancementType> CODEC`.
pub fn CODEC() -> () {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}